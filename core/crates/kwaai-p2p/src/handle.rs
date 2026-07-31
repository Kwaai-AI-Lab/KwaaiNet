//! [`NetworkHandle`] — the clonable facade over the swarm task.
//!
//! Every method sends a [`Command`] carrying a `oneshot::Sender` down an
//! `mpsc` channel to [`crate::service::NetworkService`]'s event loop and awaits
//! the reply. The handle never touches the `Swarm` itself, which is what makes
//! it `Clone + Send + Sync` and safe to hand to unrelated tasks.
//!
//! Method names mirror `kwaai_p2p_daemon::client::P2PClient` (`connect_peer`,
//! `disconnect_peer`, `list_peers`, …).
//!
//! **Event-loop discipline:** no command handler may block. Commands that
//! cannot be answered synchronously from swarm state (dials, DHT queries) are
//! parked in a pending map keyed by `ConnectionId`/`QueryId` and resolved when
//! the corresponding swarm event arrives. Every pending entry must be removed
//! on *both* the success and failure event, or the caller waits forever — see
//! the error arms in `service.rs`.

use std::future::Future;
use std::pin::Pin;

use libp2p::{Multiaddr, PeerId};
use tokio::sync::{mpsc, oneshot};

use crate::error::{P2PError, P2PResult};
use crate::raw_stream::{InboundStream, OpenResult, RawStream, RawStreamError};
use crate::unary::{UnaryProtocol, UnaryResult};

/// One inbound unary call handed to a registered handler task.
///
/// The `responder` is `unary::InboundRequest::responder`: sending on it writes
/// the success or error arm back on the caller's stream. Dropping it makes the
/// stream worker synthesise an error arm, so a panicking handler still resolves
/// the remote caller rather than stalling it until its own timeout.
#[derive(Debug)]
pub struct InboundUnaryCall {
    /// The caller, derived from the connection — never from the frame's `peer`
    /// field, which arrives unrewritten on this path.
    pub peer: PeerId,
    /// The raw application payload.
    pub data: Vec<u8>,
    /// Where this call's result goes.
    pub responder: oneshot::Sender<Result<Vec<u8>, String>>,
}

/// The channel the service uses to hand inbound calls to a handler task.
///
/// Unbounded, and therefore never blocking the swarm event loop — the bound
/// that matters is `unary::Config::max_concurrent_streams`, applied per
/// connection before a request is ever decoded.
pub type InboundUnarySender = mpsc::UnboundedSender<InboundUnaryCall>;

/// A registered unary handler: request bytes in, response-or-error-arm out.
///
/// Boxed rather than generic because handlers for different protocols live in
/// one map. The shape mirrors `kwaai_p2p_daemon::P2PClient::add_unary_handler`'s
/// `F: Fn(Vec<u8>) -> Fut` so call sites migrate without restructuring.
pub type UnaryHandler = Box<
    dyn Fn(Vec<u8>) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, String>> + Send>>
        + Send
        + Sync
        + 'static,
>;

/// A connected peer, as reported by [`NetworkHandle::list_peers`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerInfo {
    /// The remote peer's ID.
    pub peer_id: PeerId,
    /// The multiaddr of the connection (remote address for outbound, the
    /// observed send-back address for inbound).
    pub addr: Multiaddr,
    /// Which side opened the connection.
    pub direction: Direction,
}

/// Which end dialed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// We dialed them.
    Outbound,
    /// They dialed us.
    Inbound,
}

impl Direction {
    /// Lowercase label, matching the p2pd control-protocol vocabulary.
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Outbound => "outbound",
            Direction::Inbound => "inbound",
        }
    }
}

/// Commands accepted by the swarm event loop.
///
/// Every variant carries a `oneshot::Sender` so that a caller always learns the
/// outcome — including "the service is shutting down", which surfaces as a
/// dropped sender and therefore a `RecvError` on the handle side.
#[derive(Debug)]
pub enum Command {
    /// Dial a multiaddr (which must carry `/p2p/<peer-id>`), resolving when the
    /// connection is established or the dial fails.
    ConnectPeer {
        addr: Multiaddr,
        reply: oneshot::Sender<P2PResult<PeerId>>,
    },
    /// Close all connections to a peer.
    DisconnectPeer {
        peer: PeerId,
        reply: oneshot::Sender<P2PResult<()>>,
    },
    /// Snapshot of currently connected peers.
    ListPeers {
        reply: oneshot::Sender<Vec<PeerInfo>>,
    },
    /// Addresses other peers reported observing us at, with a count of how many
    /// distinct peers reported each.
    ObservedAddrs {
        reply: oneshot::Sender<Vec<(Multiaddr, usize)>>,
    },
    /// Addresses the swarm is actually listening on.
    ListenAddrs {
        reply: oneshot::Sender<Vec<Multiaddr>>,
    },
    /// The protocol list a connected peer advertised over identify.
    PeerProtocols {
        peer: PeerId,
        reply: oneshot::Sender<Option<Vec<String>>>,
    },
    /// Every peer currently in the Kademlia routing table.
    RoutingPeers { reply: oneshot::Sender<Vec<PeerId>> },
    /// Kademlia lookup for a peer's addresses.
    DhtFindPeer {
        peer: PeerId,
        reply: oneshot::Sender<P2PResult<Vec<Multiaddr>>>,
    },
    /// Insert an address into the Kademlia routing table.
    AddKadAddress {
        peer: PeerId,
        addr: Multiaddr,
        reply: oneshot::Sender<()>,
    },
    /// Dial every initial peer, then run `kad.bootstrap()`.
    Bootstrap {
        peers: Vec<Multiaddr>,
        reply: oneshot::Sender<P2PResult<()>>,
    },
    /// Call a hivemind unary handler on a remote peer.
    ///
    /// Unlike the other slow commands this carries no pending-map entry: the
    /// `reply` is handed to `unary::Behaviour::send_request`, which resolves it
    /// on every outcome including dial failure and timeout.
    CallUnary {
        peer: PeerId,
        proto: String,
        data: Vec<u8>,
        reply: oneshot::Sender<UnaryResult>,
    },
    /// Start serving `proto`, routing inbound calls to `sender`.
    AddUnaryHandler {
        proto: String,
        sender: InboundUnarySender,
        reply: oneshot::Sender<()>,
    },
    /// Stop serving `proto`. Subsequent calls to it get a clean negotiation
    /// refusal. Reports whether a handler was actually registered.
    RemoveUnaryHandler {
        proto: String,
        reply: oneshot::Sender<bool>,
    },

    /// Open a **raw** libp2p stream to `peer`, negotiating the first of
    /// `protos` the remote accepts.
    ///
    /// Like `CallUnary` this carries no pending-map entry: the reply channel is
    /// handed to `raw_stream::Behaviour::open_stream`, which resolves it on
    /// every path including dial failure and negotiation refusal.
    OpenRawStream {
        peer: PeerId,
        protos: Vec<String>,
        reply: oneshot::Sender<OpenResult>,
    },
    /// Start accepting inbound raw streams on each of `protos`, routing them to
    /// `sender`.
    ///
    /// Reports the protocols that were **not** registered because another
    /// handler already owns them, so the caller can refuse with the Go daemon's
    /// "already set" wording without a second round trip.
    AddStreamHandler {
        protos: Vec<String>,
        sender: InboundStreamSender,
        reply: oneshot::Sender<Vec<String>>,
    },
    /// Stop accepting inbound raw streams on `protos`. Reports which were
    /// actually registered.
    RemoveStreamHandler {
        protos: Vec<String>,
        reply: oneshot::Sender<Vec<String>>,
    },

    /// Stop the event loop.
    Shutdown { reply: oneshot::Sender<()> },
}

/// The channel the service uses to hand inbound raw streams to their handler.
///
/// Unbounded for the same reason as [`InboundUnarySender`]: an unbounded send
/// never blocks, which is what makes it legal inside the swarm select loop. The
/// real bound is the receiver's own accept loop, and a stream sitting in this
/// queue is not consuming a remote's window — libp2p's flow control has not
/// been released yet because nobody has read from the stream.
pub type InboundStreamSender = mpsc::UnboundedSender<InboundStream>;

/// Clonable control handle for a running [`crate::service::NetworkService`].
#[derive(Debug, Clone)]
pub struct NetworkHandle {
    local_peer_id: PeerId,
    commands: mpsc::Sender<Command>,
    /// The configured per-call unary budget, kept here purely so a timeout can
    /// be reported as [`P2PError::Timeout`] with the real number of
    /// milliseconds rather than a placeholder.
    request_timeout: std::time::Duration,
}

impl NetworkHandle {
    pub(crate) fn new(
        local_peer_id: PeerId,
        commands: mpsc::Sender<Command>,
        request_timeout: std::time::Duration,
    ) -> Self {
        Self {
            local_peer_id,
            commands,
            request_timeout,
        }
    }

    /// This node's peer ID. Available without touching the event loop.
    pub fn peer_id(&self) -> PeerId {
        self.local_peer_id
    }

    /// This node's peer ID in base58 (`12D3Koo…` / `Qm…`).
    pub fn local_peer_id(&self) -> String {
        self.local_peer_id.to_base58()
    }

    /// Send a command and await its reply, mapping a dead event loop onto
    /// [`P2PError::NotInitialized`].
    async fn call<T>(&self, make: impl FnOnce(oneshot::Sender<T>) -> Command) -> P2PResult<T> {
        let (tx, rx) = oneshot::channel();
        self.commands
            .send(make(tx))
            .await
            .map_err(|_| P2PError::NotInitialized)?;
        rx.await.map_err(|_| P2PError::NotInitialized)
    }

    /// Dial `multiaddr_str`, which must include a `/p2p/<peer-id>` component.
    /// Resolves once the connection is established.
    pub async fn connect_peer(&self, multiaddr_str: &str) -> P2PResult<PeerId> {
        let addr: Multiaddr = multiaddr_str
            .parse()
            .map_err(|e| P2PError::InvalidAddress(format!("{multiaddr_str}: {e}")))?;
        self.call(|reply| Command::ConnectPeer { addr, reply })
            .await?
    }

    /// Close all connections to `peer`.
    pub async fn disconnect_peer(&self, peer: PeerId) -> P2PResult<()> {
        self.call(|reply| Command::DisconnectPeer { peer, reply })
            .await?
    }

    /// Currently connected peers.
    pub async fn list_peers(&self) -> P2PResult<Vec<PeerInfo>> {
        self.call(|reply| Command::ListPeers { reply }).await
    }

    /// Addresses peers have observed us at, paired with the number of distinct
    /// peers that reported each. A high count is the signal later phases use to
    /// promote an address to a confirmed external address.
    pub async fn observed_addrs(&self) -> P2PResult<Vec<(Multiaddr, usize)>> {
        self.call(|reply| Command::ObservedAddrs { reply }).await
    }

    /// Addresses the swarm is listening on (post-resolution, so an ephemeral
    /// `/tcp/0` shows the real port).
    pub async fn listen_addrs(&self) -> P2PResult<Vec<Multiaddr>> {
        self.call(|reply| Command::ListenAddrs { reply }).await
    }

    /// The protocols `peer` advertised in its most recent identify response, or
    /// `None` if identify has not completed with that peer yet.
    ///
    /// This is what the identify-driven capability checks read: relay-hop
    /// support (`/libp2p/circuit/relay/0.2.0/hop`), AutoNAT
    /// (`/libp2p/autonat/1.0.0`) and dcutr all announce themselves here.
    pub async fn peer_protocols(&self, peer: PeerId) -> P2PResult<Option<Vec<String>>> {
        self.call(|reply| Command::PeerProtocols { peer, reply })
            .await
    }

    /// Every peer in the Kademlia routing table, nearest bucket first.
    ///
    /// This is the routing snapshot a hivemind DHT server needs: hivemind
    /// answers each `rpc_find` with the `k` peers nearest the queried key
    /// (`protocol.py:362-364`), drawn from its own routing table. We have no
    /// second routing table — kad's k-buckets *are* it — so the DHT service
    /// periodically pulls this and feeds it to
    /// `kwaai_hivemind_dht::DHTStorage::update_peer_ids`.
    ///
    /// Note these are **connected-or-known kad peers**, not the hivemind DHT
    /// node set: a peer here may speak `/ipfs/kad/1.0.0` without serving
    /// `DHTProtocol.rpc_*`. That only costs a caller one wasted hop during an
    /// iterative lookup, which is why it is acceptable to return them
    /// unfiltered; hivemind's own routing table has the same property.
    ///
    /// Reading k-buckets is a synchronous walk over in-memory state, so it does
    /// not block the event loop.
    pub async fn routing_peers(&self) -> P2PResult<Vec<PeerId>> {
        self.call(|reply| Command::RoutingPeers { reply }).await
    }

    /// Resolve a peer's addresses through Kademlia.
    ///
    /// Runs `get_closest_peers(peer)` and, when the query completes, reads the
    /// addresses that the walk deposited in the local routing table. Returns an
    /// empty vec if the peer was not found.
    pub async fn dht_find_peer(&self, peer: PeerId) -> P2PResult<Vec<Multiaddr>> {
        self.call(|reply| Command::DhtFindPeer { peer, reply })
            .await?
    }

    /// Add a known address for `peer` to the Kademlia routing table.
    pub async fn add_kad_address(&self, peer: PeerId, addr: Multiaddr) -> P2PResult<()> {
        self.call(|reply| Command::AddKadAddress { peer, addr, reply })
            .await
    }

    /// Dial each address in `initial_peers` and then run a Kademlia bootstrap.
    ///
    /// Succeeds if *at least one* peer was reachable; fails only when every
    /// dial failed (or the list was empty), because a partially-reachable
    /// bootstrap set is the normal case on a live network.
    pub async fn bootstrap(&self, initial_peers: Vec<Multiaddr>) -> P2PResult<()> {
        self.call(|reply| Command::Bootstrap {
            peers: initial_peers,
            reply,
        })
        .await?
    }

    /// Call the unary handler `proto` on `peer` with `data`.
    ///
    /// Mirrors `kwaai_p2p_daemon::P2PClient::call_unary_handler` (which takes
    /// peer-ID *bytes*; here the peer is already typed). Dials on demand, so a
    /// call to a peer we are not connected to works as long as some behaviour —
    /// Kademlia in practice — can supply an address, matching Go's
    /// `host.NewStream` semantics.
    ///
    /// # Errors
    ///
    /// [`crate::unary::UnaryError`] maps onto [`P2PError`] as follows, chosen so
    /// call sites can keep the coarse distinctions they already make:
    ///
    /// | `UnaryError` | `P2PError` | why |
    /// | --- | --- | --- |
    /// | `Timeout` | [`P2PError::Timeout`] | carries the configured budget in ms |
    /// | `UnsupportedProtocol` | [`P2PError::Protocol`] | a clean refusal is a protocol-level answer, not a transport fault |
    /// | `DialFailure` | [`P2PError::DialFailed`] | we never reached the peer |
    /// | `Remote` | [`P2PError::Protocol`] | the remote handler ran and returned its error arm; its text is preserved verbatim |
    /// | `Wire` | [`P2PError::Transport`] | the exchange broke below the application layer |
    pub async fn call_unary_handler(
        &self,
        peer: PeerId,
        proto: &str,
        data: &[u8],
    ) -> P2PResult<Vec<u8>> {
        let result = self
            .call(|reply| Command::CallUnary {
                peer,
                proto: proto.to_string(),
                data: data.to_vec(),
                reply,
            })
            .await?;
        result.map_err(|e| unary_error(e, proto, self.request_timeout))
    }

    /// Serve `proto`, dispatching each inbound call to `handler`.
    ///
    /// Mirrors `kwaai_p2p_daemon::P2PClient::add_unary_handler` minus its
    /// `balanced` flag, which is a p2pd-side load-balancing knob with no
    /// meaning for an in-process handler.
    ///
    /// Spawns one long-lived dispatch task that owns the receiving end of the
    /// service's dispatch channel and spawns a task **per call**, so a slow
    /// handler delays only its own caller. Registering a protocol that is
    /// already served replaces the previous handler; the old dispatch task ends
    /// when the service drops its sender.
    pub async fn add_unary_handler<F, Fut>(&self, proto: &str, handler: F) -> P2PResult<()>
    where
        F: Fn(Vec<u8>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Vec<u8>, String>> + Send + 'static,
    {
        let handler: UnaryHandler = Box::new(move |data| Box::pin(handler(data)));
        self.add_unary_handler_boxed(proto, handler).await
    }

    /// [`NetworkHandle::add_unary_handler`] with the handler already boxed, for
    /// call sites that store handlers in a collection (Phase 3's IPC server
    /// dispatches by protocol name at runtime).
    pub async fn add_unary_handler_boxed(
        &self,
        proto: &str,
        handler: UnaryHandler,
    ) -> P2PResult<()> {
        let (tx, mut rx) = mpsc::unbounded_channel::<InboundUnaryCall>();

        self.call(|reply| Command::AddUnaryHandler {
            proto: proto.to_string(),
            sender: tx,
            reply,
        })
        .await?;

        let handler = std::sync::Arc::new(handler);
        // Ends when the service drops its sender — on shutdown, on
        // `remove_unary_handler`, or when this protocol is re-registered.
        tokio::spawn(async move {
            while let Some(call) = rx.recv().await {
                let handler = std::sync::Arc::clone(&handler);
                tokio::spawn(async move {
                    let result = handler(call.data).await;
                    // The receiver is gone only if the stream worker already
                    // gave up (its own timeout); nothing left to report.
                    let _ = call.responder.send(result);
                });
            }
        });

        Ok(())
    }

    /// Stop serving `proto`. Returns `true` if a handler was registered.
    ///
    /// After this resolves, calls to `proto` are refused during negotiation —
    /// the remote sees [`crate::unary::UnaryError::UnsupportedProtocol`], the
    /// same clean refusal a never-registered protocol produces.
    pub async fn remove_unary_handler(&self, proto: &str) -> P2PResult<bool> {
        self.call(|reply| Command::RemoveUnaryHandler {
            proto: proto.to_string(),
            reply,
        })
        .await
    }

    // ------------------------------------------------------------------
    // Raw streams (pipe mode)
    // ------------------------------------------------------------------

    /// Open a raw libp2p stream to `peer` on the first of `protos` it accepts.
    ///
    /// Mirrors `kwaai_p2p_daemon::P2PClient::stream_open`'s semantics and Go's
    /// `doStreamOpen`: the protocol list is a preference order handed to
    /// multistream-select, and the peer is dialled on demand if there is no
    /// connection. Returns the protocol that won, plus the stream.
    ///
    /// The stream is `futures::io::AsyncRead + AsyncWrite`. Nothing is written
    /// to or read from it here — framing, if any, belongs to the protocol the
    /// caller just negotiated.
    ///
    /// # Errors
    ///
    /// [`RawStreamError`] maps onto [`P2PError`] on the same principle as
    /// [`NetworkHandle::call_unary_handler`]: a clean refusal is a
    /// protocol-level answer ([`P2PError::Protocol`]), an unreachable peer is
    /// [`P2PError::DialFailed`], and anything below the application layer is
    /// [`P2PError::Transport`].
    pub async fn open_raw_stream(
        &self,
        peer: PeerId,
        protos: Vec<String>,
    ) -> P2PResult<(String, RawStream)> {
        let result = self
            .call(|reply| Command::OpenRawStream {
                peer,
                protos,
                reply,
            })
            .await?;
        match result {
            Ok((proto, stream)) => Ok((proto.as_ref().to_string(), stream)),
            Err(e) => Err(raw_stream_error(e)),
        }
    }

    /// Accept inbound raw streams on each of `protos`.
    ///
    /// Returns an [`mpsc::UnboundedReceiver`] of [`InboundStream`]s, plus the
    /// protocols that could **not** be registered because another handler
    /// already serves them. That split mirrors Go's `doStreamHandler`, which
    /// refuses a protocol already in its handler map (with `balanced` false —
    /// the only mode this codebase uses) but registers the rest of the list.
    ///
    /// Dropping the receiver does *not* unregister: call
    /// [`NetworkHandle::remove_stream_handler`] for that, so that ownership is
    /// explicit rather than tied to a channel's lifetime.
    pub async fn accept_streams(
        &self,
        protos: Vec<String>,
    ) -> P2PResult<(mpsc::UnboundedReceiver<InboundStream>, Vec<String>)> {
        let (tx, rx) = mpsc::unbounded_channel::<InboundStream>();
        let refused = self
            .call(|reply| Command::AddStreamHandler {
                protos,
                sender: tx,
                reply,
            })
            .await?;
        Ok((rx, refused))
    }

    /// Stop accepting inbound raw streams on `protos`.
    ///
    /// Returns the subset that was actually registered. Streams already open
    /// keep running — this governs negotiation only, matching Go's
    /// `host.RemoveStreamHandler`.
    pub async fn remove_stream_handler(&self, protos: Vec<String>) -> P2PResult<Vec<String>> {
        self.call(|reply| Command::RemoveStreamHandler { protos, reply })
            .await
    }

    /// Ask the event loop to stop. Returns once it has acknowledged; await the
    /// `JoinHandle` from `NetworkService::spawn` to know it has fully exited.
    pub async fn shutdown(&self) -> P2PResult<()> {
        self.call(|reply| Command::Shutdown { reply }).await
    }
}

/// Map a raw-stream failure onto the crate error type. See
/// [`NetworkHandle::open_raw_stream`] for the rationale behind each arm.
fn raw_stream_error(error: RawStreamError) -> P2PError {
    match error {
        RawStreamError::UnsupportedProtocol(p) => {
            P2PError::Protocol(format!("remote does not support protocol {p}"))
        }
        RawStreamError::DialFailure(e) => P2PError::DialFailed(e),
        RawStreamError::Io(e) => P2PError::Transport(e),
    }
}

/// Parse protocol names for the raw-stream behaviour, dropping empties.
///
/// [`UnaryProtocol::new`] panics on an empty name (multistream-select cannot
/// negotiate one), and the protocol list on a `STREAM_HANDLER` request comes
/// straight off the wire from an external process — so it is filtered here
/// rather than trusted.
pub(crate) fn parse_protocols(protos: &[String]) -> Vec<UnaryProtocol> {
    protos
        .iter()
        .filter(|p| !p.is_empty())
        .map(|p| UnaryProtocol::new(p.as_str()))
        .collect()
}

/// Map a unary failure onto the crate error type. See
/// [`NetworkHandle::call_unary_handler`] for the rationale behind each arm.
fn unary_error(
    error: crate::unary::UnaryError,
    proto: &str,
    request_timeout: std::time::Duration,
) -> P2PError {
    use crate::unary::UnaryError;
    match error {
        UnaryError::Timeout => P2PError::Timeout(request_timeout.as_millis() as u64),
        UnaryError::UnsupportedProtocol(p) => {
            P2PError::Protocol(format!("remote does not support protocol {p}"))
        }
        UnaryError::DialFailure(e) => P2PError::DialFailed(e),
        UnaryError::Remote(e) => P2PError::Protocol(format!("remote handler error ({proto}): {e}")),
        UnaryError::Wire(e) => P2PError::Transport(e),
    }
}
