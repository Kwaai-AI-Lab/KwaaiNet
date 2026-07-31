//! [`NetworkHandle`] — the clonable facade over the swarm task.
//!
//! Every method sends a [`Command`] carrying a `oneshot::Sender` down an
//! `mpsc` channel to [`crate::service::NetworkService`]'s event loop and awaits
//! the reply. The handle never touches the `Swarm` itself, which is what makes
//! it `Clone + Send + Sync` and safe to hand to unrelated tasks.
//!
//! Method names deliberately mirror `kwaai_p2p_daemon::client::P2PClient`
//! (`connect_peer`, `disconnect_peer`, `list_peers`, …) so that later phases
//! can swap the p2pd client for this handle with minimal churn at call sites.
//!
//! **Event-loop discipline:** no command handler may block. Commands that
//! cannot be answered synchronously from swarm state (dials, DHT queries) are
//! parked in a pending map keyed by `ConnectionId`/`QueryId` and resolved when
//! the corresponding swarm event arrives. Every pending entry must be removed
//! on *both* the success and failure event, or the caller waits forever — see
//! the error arms in `service.rs`.
//!
//! Unary RPC is the exception that proves the rule: [`Command::CallUnary`]
//! hands its `oneshot` straight to `unary::Behaviour::send_request`, which owns
//! it until the call resolves on *some* path (dial failure, negotiation
//! refusal, timeout, remote error, success). No pending map is needed here
//! because the behaviour itself never drops a reply channel silently.

use std::future::Future;
use std::pin::Pin;

use libp2p::{Multiaddr, PeerId};
use tokio::sync::{mpsc, oneshot};

use crate::error::{P2PError, P2PResult};
use crate::unary::UnaryResult;

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
    /// Stop the event loop.
    Shutdown { reply: oneshot::Sender<()> },
}

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

    /// Ask the event loop to stop. Returns once it has acknowledged; await the
    /// `JoinHandle` from `NetworkService::spawn` to know it has fully exited.
    pub async fn shutdown(&self) -> P2PResult<()> {
        self.call(|reply| Command::Shutdown { reply }).await
    }
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
