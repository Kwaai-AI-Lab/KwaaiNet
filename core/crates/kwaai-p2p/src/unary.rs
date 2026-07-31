//! Hivemind unary RPC as a libp2p `NetworkBehaviour`.
//!
//! One request and one response per stream, framed and enveloped by
//! `kwaai_hivemind_dht::wire`, on a protocol ID that is the **bare hivemind
//! handler name with no leading slash** (`DHTProtocol.rpc_store`, `hello`, …).
//! Phase 0 proved this wire shape and the slash-less negotiation against a
//! real go-libp2p-daemon (`kwaai-network-tests/tests/07_wire_interop.rs`).
//!
//! ## Why not `libp2p::request_response`
//!
//! Two of its design constants do not fit hivemind:
//!
//! - **Protocol-per-request.** `request_response::Behaviour` negotiates from
//!   one protocol list fixed at construction; `send_request` cannot say which
//!   protocol a given request is for. A hivemind call *is* its protocol — the
//!   handler name — so every outbound request here negotiates exactly one
//!   protocol, the one the caller asked for.
//! - **Dynamic inbound registration.** External processes register and remove
//!   unary handlers at runtime (`add_unary_handler` over the IPC socket,
//!   Phase 3). The inbound protocol set therefore lives behind an
//!   `Arc<RwLock<…>>` shared with every connection handler, and
//!   `listen_protocol()` reads it per inbound stream — a registration made
//!   after a connection was established is still visible to that connection's
//!   next negotiation.
//!
//! `libp2p_stream` is also out: `StreamProtocol` rejects names without a
//! leading `/`, and hivemind names have none.
//!
//! ## Shape
//!
//! [`Behaviour`] mirrors `request_response`'s structure (pending-request
//! queues, preloading handlers on connection establishment, dial-on-demand)
//! but resolves outcomes through the `oneshot` each request carries instead of
//! surfacing request-ID events — the caller side of [`Behaviour::send_request`]
//! awaits its own reply channel, so no request-ID bookkeeping leaks into the
//! service loop. Inbound requests surface as [`Event::InboundRequest`] with a
//! `responder` oneshot; the stream worker writes whatever arrives on it (or an
//! error frame on timeout/drop) and closes the stream.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll};
use std::time::Duration;

use futures::channel::mpsc;
use futures::future::BoxFuture;
use futures::io::AsyncWriteExt as _;
use futures::stream::FuturesUnordered;
use futures::{FutureExt, SinkExt, StreamExt};
use libp2p::core::upgrade::{InboundUpgrade, OutboundUpgrade, UpgradeInfo};
use libp2p::core::{Endpoint, Multiaddr};
use libp2p::swarm::handler::{
    ConnectionEvent, DialUpgradeError, FullyNegotiatedInbound, FullyNegotiatedOutbound,
};
use libp2p::swarm::{
    behaviour::PeerAddresses, dial_opts::DialOpts, ConnectionDenied, ConnectionHandler,
    ConnectionHandlerEvent, ConnectionId, FromSwarm, NetworkBehaviour, NotifyHandler,
    StreamUpgradeError, SubstreamProtocol, THandler, THandlerInEvent, THandlerOutEvent, ToSwarm,
};
use libp2p::swarm::{DialFailure, Stream};
use libp2p::PeerId;
use tokio::sync::oneshot;
use tracing::{debug, trace, warn};

use kwaai_hivemind_dht::wire;

/// A hivemind unary protocol ID: the bare handler name, no leading slash.
///
/// This type exists because `libp2p::StreamProtocol` refuses slash-less names;
/// multistream-select itself does not care (Phase 0,
/// `slashless_protocol_negotiates`). `Arc<str>` because one registration is
/// cloned into every connection handler and every negotiation.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct UnaryProtocol(Arc<str>);

impl UnaryProtocol {
    /// Wrap a handler name. Empty names are a caller bug — multistream-select
    /// cannot negotiate them — hence the panic, mirroring `StreamProtocol::new`.
    pub fn new(name: impl Into<Arc<str>>) -> Self {
        let name = name.into();
        assert!(!name.is_empty(), "unary protocol name must not be empty");
        Self(name)
    }
}

impl AsRef<str> for UnaryProtocol {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for UnaryProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnaryProtocol({})", self.0)
    }
}

impl fmt::Display for UnaryProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Why an outbound unary call failed.
#[derive(Debug, thiserror::Error)]
pub enum UnaryError {
    /// No response within the configured timeout.
    #[error("unary call timed out")]
    Timeout,
    /// The remote refused the protocol during negotiation — it serves no such
    /// handler. This is the "clean protocol refusal" the health probe relies on.
    #[error("remote does not support protocol {0}")]
    UnsupportedProtocol(String),
    /// We could not establish any connection to the peer.
    #[error("dial failed: {0}")]
    DialFailure(String),
    /// The remote handler ran and returned the error arm.
    #[error("remote handler error: {0}")]
    Remote(String),
    /// The exchange broke below the application layer (stream reset, bad
    /// frame, callId mismatch, …).
    #[error("unary wire failure: {0}")]
    Wire(String),
}

/// An application-level result: the responder's payload or its error arm.
pub type UnaryResult = Result<Vec<u8>, UnaryError>;

/// One outbound call, from `send_request` until its stream worker resolves it.
pub struct OutboundMessage {
    proto: UnaryProtocol,
    data: Vec<u8>,
    reply: oneshot::Sender<UnaryResult>,
}

impl fmt::Debug for OutboundMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OutboundMessage")
            .field("proto", &self.proto)
            .field("data_len", &self.data.len())
            .finish_non_exhaustive()
    }
}

/// An inbound call decoded off a stream, en route to whoever serves `proto`.
///
/// Send the handler's outcome on `responder`; the stream worker encodes it
/// (success or error arm) with the original callId and closes the stream.
/// Dropping `responder` sends the error arm instead — the caller is never left
/// hanging until its own timeout.
#[derive(Debug)]
pub struct InboundRequest {
    /// The negotiated protocol (authoritative — the frame's `proto` field is
    /// logged on mismatch but never trusted for dispatch).
    pub proto: UnaryProtocol,
    /// The raw application payload (`CallUnaryRequest.data`).
    pub data: Vec<u8>,
    /// Where the handler's result goes.
    pub responder: oneshot::Sender<Result<Vec<u8>, String>>,
}

/// Events surfaced to the swarm owner.
#[derive(Debug)]
pub enum Event {
    /// A remote peer called one of our registered unary handlers.
    InboundRequest {
        /// The caller, taken from the connection — NEVER from
        /// `callUnary.peer`, which arrives unrewritten on this path (Phase 0
        /// finding, `raw_wire_responder_is_accepted_by_daemon_caller`).
        peer: PeerId,
        /// The decoded request.
        request: InboundRequest,
    },
}

/// Tuning knobs, deliberately few.
#[derive(Debug, Clone)]
pub struct Config {
    /// Per-call budget, covering the whole exchange after negotiation
    /// (write + remote handler + read). Also bounds how long an inbound
    /// stream waits for its local handler.
    pub request_timeout: Duration,
    /// Cap on concurrent stream workers per connection; streams beyond it are
    /// dropped at negotiation.
    pub max_concurrent_streams: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            // Matches the p2pd client's `send_to_bootstrap` budget.
            request_timeout: Duration::from_secs(30),
            max_concurrent_streams: 100,
        }
    }
}

// ============================================================================
// Substream upgrade
// ============================================================================

/// A one-shot upgrade that hands back the stream plus whichever protocol
/// multistream-select agreed on. Like `libp2p::core::upgrade::ReadyUpgrade`,
/// except it advertises many protocols and reports the selected one.
pub struct Protocols {
    protocols: Vec<UnaryProtocol>,
}

impl UpgradeInfo for Protocols {
    type Info = UnaryProtocol;
    type InfoIter = std::vec::IntoIter<UnaryProtocol>;

    fn protocol_info(&self) -> Self::InfoIter {
        self.protocols.clone().into_iter()
    }
}

impl InboundUpgrade<Stream> for Protocols {
    type Output = (Stream, UnaryProtocol);
    type Error = std::convert::Infallible;
    type Future = futures::future::Ready<Result<Self::Output, Self::Error>>;

    fn upgrade_inbound(self, io: Stream, protocol: Self::Info) -> Self::Future {
        futures::future::ready(Ok((io, protocol)))
    }
}

impl OutboundUpgrade<Stream> for Protocols {
    type Output = (Stream, UnaryProtocol);
    type Error = std::convert::Infallible;
    type Future = futures::future::Ready<Result<Self::Output, Self::Error>>;

    fn upgrade_outbound(self, io: Stream, protocol: Self::Info) -> Self::Future {
        futures::future::ready(Ok((io, protocol)))
    }
}

// ============================================================================
// Connection handler
// ============================================================================

/// What an inbound stream worker hands the connection handler once it has a
/// decoded request.
struct InboundStreamRequest {
    proto: UnaryProtocol,
    data: Vec<u8>,
    responder: oneshot::Sender<Result<Vec<u8>, String>>,
}

/// Per-connection handler: negotiates streams and runs one worker future per
/// stream. Workers resolve their own oneshots, so `ToBehaviour` only ever
/// carries inbound requests.
pub struct Handler {
    /// The peer on the other end — `CallUnaryRequest.peer` on outbound frames,
    /// matching what Go's caller writes (the dial target).
    remote: PeerId,
    /// Shared with [`Behaviour`]; read per `listen_protocol()` call so runtime
    /// registrations reach existing connections.
    inbound_protocols: Arc<RwLock<HashSet<UnaryProtocol>>>,
    config: Config,

    /// Outbound requests not yet emitted as substream requests.
    pending_outbound: VecDeque<OutboundMessage>,
    /// Emitted substream requests awaiting negotiation, in emission order —
    /// `FullyNegotiatedOutbound`/`DialUpgradeError` arrive in the same order.
    requested_outbound: VecDeque<OutboundMessage>,

    /// Stream workers. Timeouts live inside each future so every worker
    /// resolves its oneshot on every path.
    workers: FuturesUnordered<BoxFuture<'static, ()>>,

    /// Inbound requests decoded by workers, drained in `poll`. Zero-capacity:
    /// a worker parks until the handler actually forwards its request.
    inbound_rx: mpsc::Receiver<InboundStreamRequest>,
    inbound_tx: mpsc::Sender<InboundStreamRequest>,
}

impl Handler {
    fn new(
        remote: PeerId,
        inbound_protocols: Arc<RwLock<HashSet<UnaryProtocol>>>,
        config: Config,
    ) -> Self {
        let (inbound_tx, inbound_rx) = mpsc::channel(0);
        Self {
            remote,
            inbound_protocols,
            config,
            pending_outbound: VecDeque::new(),
            requested_outbound: VecDeque::new(),
            workers: FuturesUnordered::new(),
            inbound_rx,
            inbound_tx,
        }
    }

    fn on_fully_negotiated_inbound(&mut self, mut stream: Stream, proto: UnaryProtocol) {
        if self.workers.len() >= self.config.max_concurrent_streams {
            warn!(%proto, "dropping inbound unary stream: at capacity");
            return;
        }

        let mut to_handler = self.inbound_tx.clone();
        let timeout = self.config.request_timeout;

        self.workers.push(
            async move {
                let outcome: Result<(), wire::WireError> = async {
                    let frame = wire::read_framed_futures(&mut stream).await?;
                    let (call_id, _peer, frame_proto, data) = wire::decode_unary_request(&frame)?;
                    if frame_proto != proto.as_ref() {
                        // Dispatch is by negotiated protocol; the frame field is
                        // informational (hivemind's Python client fills it, Go
                        // ignores it).
                        debug!(negotiated = %proto, in_frame = %frame_proto,
                            "unary frame proto differs from negotiated protocol");
                    }

                    let (result_tx, result_rx) = oneshot::channel();
                    let request = InboundStreamRequest {
                        proto,
                        data,
                        responder: result_tx,
                    };
                    let result = if to_handler.send(request).await.is_err() {
                        // Handler shutting down with the stream mid-flight.
                        Err("node shutting down".to_string())
                    } else {
                        match tokio::time::timeout(timeout, result_rx).await {
                            Ok(Ok(result)) => result,
                            Ok(Err(_)) => Err("handler dropped the request".to_string()),
                            Err(_) => Err("handler timed out".to_string()),
                        }
                    };

                    stream
                        .write_all(&wire::encode_unary_response(&call_id, result))
                        .await?;
                    stream.flush().await?;
                    stream.close().await?;
                    Ok(())
                }
                .await;

                if let Err(e) = outcome {
                    trace!(error = %e, "inbound unary stream failed");
                }
            }
            .boxed(),
        );
    }

    fn on_fully_negotiated_outbound(&mut self, mut stream: Stream, proto: UnaryProtocol) {
        let message = self
            .requested_outbound
            .pop_front()
            .expect("negotiated an outbound stream without a pending message");
        debug_assert_eq!(message.proto, proto, "outbound negotiation out of order");

        let callee = self.remote.to_bytes();
        let timeout = self.config.request_timeout;

        self.workers.push(
            async move {
                let exchange = async {
                    // 16 raw UUID bytes: Go does `uuid.FromBytes` and drops the
                    // message if that fails.
                    let call_id = uuid::Uuid::new_v4().into_bytes();
                    stream
                        .write_all(&wire::encode_unary_request(
                            &call_id,
                            &callee,
                            proto.as_ref(),
                            &message.data,
                        ))
                        .await
                        .map_err(|e| UnaryError::Wire(e.to_string()))?;
                    stream
                        .flush()
                        .await
                        .map_err(|e| UnaryError::Wire(e.to_string()))?;

                    let frame = wire::read_framed_futures(&mut stream)
                        .await
                        .map_err(|e| UnaryError::Wire(e.to_string()))?;
                    let (echoed_id, result) = wire::decode_unary_response(&frame)
                        .map_err(|e| UnaryError::Wire(e.to_string()))?;
                    if echoed_id != call_id {
                        return Err(UnaryError::Wire("callId mismatch in reply".to_string()));
                    }
                    let _ = stream.close().await;

                    result.map_err(UnaryError::Remote)
                };

                let result = match tokio::time::timeout(timeout, exchange).await {
                    Ok(result) => result,
                    Err(_) => Err(UnaryError::Timeout),
                };
                let _ = message.reply.send(result);
            }
            .boxed(),
        );
    }

    fn on_dial_upgrade_error(&mut self, error: StreamUpgradeError<std::convert::Infallible>) {
        let message = self
            .requested_outbound
            .pop_front()
            .expect("upgrade error for an outbound stream without a pending message");

        let error = match error {
            StreamUpgradeError::Timeout => UnaryError::Timeout,
            StreamUpgradeError::NegotiationFailed => {
                UnaryError::UnsupportedProtocol(message.proto.to_string())
            }
            StreamUpgradeError::Io(e) => UnaryError::Wire(e.to_string()),
            StreamUpgradeError::Apply(infallible) => match infallible {},
        };
        let _ = message.reply.send(Err(error));
    }
}

impl ConnectionHandler for Handler {
    type FromBehaviour = OutboundMessage;
    type ToBehaviour = InboundStreamRequestEvent;
    type InboundProtocol = Protocols;
    type OutboundProtocol = Protocols;
    type InboundOpenInfo = ();
    type OutboundOpenInfo = ();

    fn listen_protocol(&self) -> SubstreamProtocol<Self::InboundProtocol, ()> {
        let protocols = self
            .inbound_protocols
            .read()
            .expect("inbound protocol set lock poisoned")
            .iter()
            .cloned()
            .collect();
        SubstreamProtocol::new(Protocols { protocols }, ())
    }

    fn on_behaviour_event(&mut self, message: Self::FromBehaviour) {
        self.pending_outbound.push_back(message);
    }

    fn poll(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<ConnectionHandlerEvent<Protocols, (), Self::ToBehaviour>> {
        // Drive stream workers; their outcomes travel through oneshots, so
        // completion here is just cleanup.
        while let Poll::Ready(Some(())) = self.workers.poll_next_unpin(cx) {}

        if let Poll::Ready(Some(request)) = self.inbound_rx.poll_next_unpin(cx) {
            return Poll::Ready(ConnectionHandlerEvent::NotifyBehaviour(
                InboundStreamRequestEvent(request),
            ));
        }

        if let Some(message) = self.pending_outbound.pop_front() {
            let protocols = Protocols {
                protocols: vec![message.proto.clone()],
            };
            self.requested_outbound.push_back(message);
            return Poll::Ready(ConnectionHandlerEvent::OutboundSubstreamRequest {
                protocol: SubstreamProtocol::new(protocols, ()),
            });
        }

        Poll::Pending
    }

    fn on_connection_event(
        &mut self,
        event: ConnectionEvent<Self::InboundProtocol, Self::OutboundProtocol, (), ()>,
    ) {
        match event {
            ConnectionEvent::FullyNegotiatedInbound(FullyNegotiatedInbound {
                protocol: (stream, proto),
                ..
            }) => self.on_fully_negotiated_inbound(stream, proto),
            ConnectionEvent::FullyNegotiatedOutbound(FullyNegotiatedOutbound {
                protocol: (stream, proto),
                ..
            }) => self.on_fully_negotiated_outbound(stream, proto),
            ConnectionEvent::DialUpgradeError(DialUpgradeError { error, .. }) => {
                self.on_dial_upgrade_error(error)
            }
            _ => {}
        }
    }
}

/// Newtype so the handler's `ToBehaviour` has a `Debug` impl without exposing
/// the internal request struct.
pub struct InboundStreamRequestEvent(InboundStreamRequest);

impl fmt::Debug for InboundStreamRequestEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InboundStreamRequestEvent")
            .field("proto", &self.0.proto)
            .field("data_len", &self.0.data.len())
            .finish()
    }
}

// ============================================================================
// Behaviour
// ============================================================================

/// The hivemind unary RPC behaviour. See the module docs for the design.
pub struct Behaviour {
    config: Config,
    /// Inbound protocol registrations, shared with every connection handler.
    inbound_protocols: Arc<RwLock<HashSet<UnaryProtocol>>>,
    /// Live connections per peer, most-recent last.
    connected: HashMap<PeerId, Vec<ConnectionId>>,
    /// Requests to not-yet-connected peers, flushed on establishment, failed
    /// on dial failure.
    pending_outbound_requests: HashMap<PeerId, Vec<OutboundMessage>>,
    /// Events for the swarm, drained in `poll`.
    pending_events: VecDeque<ToSwarm<Event, OutboundMessage>>,
    /// Peer addresses learned from the swarm (`NewExternalAddrOfPeer`), served
    /// back on dial-on-demand. In production Kademlia's routing table supplies
    /// most dial addresses; this cache covers hints injected via
    /// `Swarm::add_peer_address`.
    addresses: PeerAddresses,
}

impl Behaviour {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            inbound_protocols: Arc::new(RwLock::new(HashSet::new())),
            connected: HashMap::new(),
            pending_outbound_requests: HashMap::new(),
            pending_events: VecDeque::new(),
            addresses: PeerAddresses::default(),
        }
    }

    /// Start advertising `proto` on inbound negotiation. Idempotent. Takes
    /// effect for every subsequent inbound stream, including on connections
    /// that already exist.
    pub fn register_protocol(&mut self, proto: UnaryProtocol) {
        self.inbound_protocols
            .write()
            .expect("inbound protocol set lock poisoned")
            .insert(proto);
    }

    /// Stop advertising `proto`. Streams already negotiated are unaffected.
    pub fn unregister_protocol(&mut self, proto: &UnaryProtocol) {
        self.inbound_protocols
            .write()
            .expect("inbound protocol set lock poisoned")
            .remove(proto);
    }

    /// Call `proto` on `peer` with `data`, resolving `reply` with the outcome.
    ///
    /// Dials on demand: if no connection exists the request is parked, a dial
    /// is issued (addresses come from the composed behaviours — Kademlia's
    /// routing table in practice, matching Go's `host.NewStream` semantics),
    /// and the request is preloaded onto the first connection that
    /// establishes. If the dial fails, `reply` resolves with
    /// [`UnaryError::DialFailure`].
    pub fn send_request(
        &mut self,
        peer: PeerId,
        proto: UnaryProtocol,
        data: Vec<u8>,
        reply: oneshot::Sender<UnaryResult>,
    ) {
        let message = OutboundMessage { proto, data, reply };

        match self.connected.get(&peer) {
            Some(connections) if !connections.is_empty() => {
                // Streams are cheap; the first connection is fine.
                self.pending_events.push_back(ToSwarm::NotifyHandler {
                    peer_id: peer,
                    handler: NotifyHandler::One(connections[0]),
                    event: message,
                });
            }
            _ => {
                let parked = self.pending_outbound_requests.entry(peer).or_default();
                // One dial per burst: requests parked while a dial is in
                // flight ride along on the connection it establishes.
                if parked.is_empty() {
                    self.pending_events.push_back(ToSwarm::Dial {
                        opts: DialOpts::peer_id(peer).build(),
                    });
                }
                parked.push(message);
            }
        }
    }

    /// Hand queued requests for `peer` to a freshly built connection handler.
    fn preload_new_handler(&mut self, handler: &mut Handler, peer: PeerId) {
        if let Some(pending) = self.pending_outbound_requests.remove(&peer) {
            for message in pending {
                handler.on_behaviour_event(message);
            }
        }
    }
}

impl NetworkBehaviour for Behaviour {
    type ConnectionHandler = Handler;
    type ToSwarm = Event;

    fn handle_established_inbound_connection(
        &mut self,
        connection_id: ConnectionId,
        peer: PeerId,
        _local_addr: &Multiaddr,
        _remote_addr: &Multiaddr,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        let mut handler = Handler::new(
            peer,
            Arc::clone(&self.inbound_protocols),
            self.config.clone(),
        );
        self.preload_new_handler(&mut handler, peer);
        self.connected.entry(peer).or_default().push(connection_id);
        Ok(handler)
    }

    fn handle_established_outbound_connection(
        &mut self,
        connection_id: ConnectionId,
        peer: PeerId,
        _addr: &Multiaddr,
        _role_override: Endpoint,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        let mut handler = Handler::new(
            peer,
            Arc::clone(&self.inbound_protocols),
            self.config.clone(),
        );
        self.preload_new_handler(&mut handler, peer);
        self.connected.entry(peer).or_default().push(connection_id);
        Ok(handler)
    }

    fn handle_pending_outbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        maybe_peer: Option<PeerId>,
        _addresses: &[Multiaddr],
        _effective_role: Endpoint,
    ) -> Result<Vec<Multiaddr>, ConnectionDenied> {
        Ok(match maybe_peer {
            Some(peer) => self.addresses.get(&peer).collect(),
            None => Vec::new(),
        })
    }

    fn on_swarm_event(&mut self, event: FromSwarm) {
        self.addresses.on_swarm_event(&event);
        match event {
            FromSwarm::ConnectionClosed(closed) => {
                if let Some(connections) = self.connected.get_mut(&closed.peer_id) {
                    connections.retain(|&id| id != closed.connection_id);
                    if connections.is_empty() {
                        self.connected.remove(&closed.peer_id);
                    }
                }
                // In-flight exchanges on the closed connection resolve
                // themselves: each worker owns its stream and oneshot, and the
                // stream erroring out resolves the caller with a wire error.
            }
            FromSwarm::DialFailure(DialFailure { peer_id, error, .. }) => {
                // A dial cancelled by its own peer condition means another
                // dial to the same peer is already in flight — the parked
                // requests are still going to get their connection.
                if matches!(error, libp2p::swarm::DialError::DialPeerConditionFalse(_)) {
                    return;
                }
                if let Some(peer) = peer_id {
                    // Parked requests exist only while disconnected, so a
                    // failed dial to this peer fails them all (mirrors
                    // request_response's reasoning).
                    if let Some(pending) = self.pending_outbound_requests.remove(&peer) {
                        let text = error.to_string();
                        for message in pending {
                            let _ = message
                                .reply
                                .send(Err(UnaryError::DialFailure(text.clone())));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn on_connection_handler_event(
        &mut self,
        peer: PeerId,
        _connection: ConnectionId,
        event: THandlerOutEvent<Self>,
    ) {
        let InboundStreamRequestEvent(request) = event;
        self.pending_events
            .push_back(ToSwarm::GenerateEvent(Event::InboundRequest {
                peer,
                request: InboundRequest {
                    proto: request.proto,
                    data: request.data,
                    responder: request.responder,
                },
            }));
    }

    fn poll(
        &mut self,
        _cx: &mut Context<'_>,
    ) -> Poll<ToSwarm<Self::ToSwarm, THandlerInEvent<Self>>> {
        if let Some(event) = self.pending_events.pop_front() {
            return Poll::Ready(event);
        }
        Poll::Pending
    }
}
