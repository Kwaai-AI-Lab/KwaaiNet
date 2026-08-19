//! Raw libp2p streams as a libp2p `NetworkBehaviour` — the substrate for the
//! control socket's **pipe mode** (Phase 3 slice 2).
//!
//! Where [`crate::unary`] negotiates a protocol and then runs a one-request /
//! one-response state machine over the stream, this behaviour negotiates a
//! protocol and then **hands the stream out untouched**. Everything after
//! negotiation is the caller's business: `stream_open` yields the libp2p
//! `Stream` to whoever asked for it, and `accept_streams` yields each inbound
//! one. The p2pd control protocol's `STREAM_OPEN` / `STREAM_HANDLER` verbs are
//! exactly this — the Go daemon calls `host.NewStream` / `SetStreamHandler` and
//! then `io.Copy`s in both directions (`stream.go::doStreamPipe`).
//!
//! ## Why a sibling behaviour rather than a mode on `unary::Behaviour`
//!
//! The two share their *negotiation* requirements and nothing else. Both must
//! speak protocol IDs with no leading slash (hivemind's `DHTProtocol.rpc_store`
//! and, for symmetry, whatever an external process registers), which rules out
//! `libp2p_stream` — `StreamProtocol::new` refuses slash-less names. So this
//! module reuses [`crate::unary::Protocols`] and [`UnaryProtocol`] verbatim for
//! the upgrade, and diverges immediately after: no framing, no timeout, no
//! request/response bookkeeping, no worker futures. A raw stream may live for
//! hours (inference-mux holds one open for a node's whole session), which is
//! precisely the lifetime model the unary path is built to *prevent*.
//!
//! ## Shape
//!
//! - **Outbound**: [`Behaviour::open_stream`] parks a request, dials on demand
//!   (same discipline as `unary`, for the same reason: Go's `host.NewStream`
//!   dials), and resolves a `oneshot` with the negotiated protocol and the
//!   stream. Failure resolves it too — no path drops the channel silently.
//! - **Inbound**: protocols are registered in a shared set read per
//!   `listen_protocol()`, so a registration made now reaches connections
//!   established earlier. Each negotiated inbound stream surfaces as
//!   [`Event::InboundStream`].
//!
//! The streams handed out are `futures::io::{AsyncRead, AsyncWrite}`. The relay
//! loop in `kwaai-p2p-daemon`'s control server adapts them to tokio's traits;
//! see [`crate::raw_stream::RawStream`].

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll};

use libp2p::core::transport::PortUse;
use libp2p::core::{Endpoint, Multiaddr};
use libp2p::swarm::handler::{
    ConnectionEvent, DialUpgradeError, FullyNegotiatedInbound, FullyNegotiatedOutbound,
};
use libp2p::swarm::{
    behaviour::PeerAddresses, dial_opts::DialOpts, ConnectionDenied, ConnectionHandler,
    ConnectionHandlerEvent, ConnectionId, FromSwarm, NetworkBehaviour, NotifyHandler, Stream,
    StreamUpgradeError, SubstreamProtocol, THandler, THandlerInEvent, THandlerOutEvent, ToSwarm,
};
use libp2p::PeerId;
use tokio::sync::oneshot;
use tracing::{debug, trace};

use crate::unary::{Protocols, UnaryProtocol};

/// A negotiated raw libp2p stream.
///
/// This is `libp2p::swarm::Stream` — `futures::io::AsyncRead + AsyncWrite`,
/// full-duplex, with `close()` performing a half-close (FIN) that the remote
/// observes as EOF. The alias exists to give the pipe-mode call sites a name
/// that says what the value is for.
pub type RawStream = Stream;

/// Why opening a raw stream failed.
///
/// Deliberately parallel to [`crate::unary::UnaryError`] minus the arms that
/// only make sense once an application protocol is running (`Timeout`,
/// `Remote`): negotiation is the entire lifecycle this type describes.
#[derive(Debug, thiserror::Error)]
pub enum RawStreamError {
    /// The remote serves none of the requested protocols. The clean-refusal
    /// case: `STREAM_OPEN` for an unhandled protocol must fail fast rather than
    /// hang, because the socket client is holding its connection open waiting
    /// for the `StreamInfo` reply.
    #[error("remote does not support any of the requested protocols: {0}")]
    UnsupportedProtocol(String),
    /// We could not reach the peer at all.
    #[error("dial failed: {0}")]
    DialFailure(String),
    /// Negotiation broke below the application layer, or the connection went
    /// away while the upgrade was in flight.
    #[error("stream open failed: {0}")]
    Io(String),
}

/// A protocol nobody serves, appended to every *outbound* raw-stream
/// negotiation to keep refusals eager.
///
/// The swarm negotiates outbound substreams with `Version::V1Lazy` (see
/// `service.rs`), which wins a round trip per unary call but hands back a
/// stream before the peer has confirmed the protocol. For a raw stream that is
/// the wrong trade: [`RawStreamError::UnsupportedProtocol`] has to be decided
/// *before* the caller is handed a stream, or the control socket enters pipe
/// mode for a protocol the remote does not serve — a divergence from p2pd, and
/// one the socket client cannot recover from because the reply slot is gone.
///
/// multistream-select only takes the lazy shortcut for the **last** protocol it
/// has to offer (`dialer_select.rs`, the `protocols.peek().is_some()` branch).
/// Appending one more entry therefore keeps every real protocol on the eager
/// path, negotiated exactly as it was before, at the same one round trip: a
/// long-lived stream pays that once, so there is nothing to win here anyway.
///
/// It is a marker, not a protocol. Reaching it means every real protocol was
/// already refused, so [`Handler`] turns it straight back into
/// `UnsupportedProtocol` and drops the stream. It is never proposed on the wire
/// in the accepted case, and never advertised inbound — `listen_protocol` is
/// built from the registered set alone.
pub(crate) const NEGOTIATION_SENTINEL: &str = "kwaai.__negotiation_probe__";

/// True for the reserved sentinel name. Callers and remote registrations must
/// never use it: [`Handler::on_connection_event`] identifies the sentinel by
/// name, so a caller-supplied `kwaai.__negotiation_probe__` would make
/// `offered` read `[sentinel, sentinel]`, negotiate the *first* one eagerly and
/// successfully, then trip the guard on the name — resetting a live stream
/// under the remote while telling the caller it was unsupported.
pub(crate) fn is_reserved(proto: &UnaryProtocol) -> bool {
    proto.as_ref() == NEGOTIATION_SENTINEL
}

/// The result of an [`Behaviour::open_stream`] request: the protocol
/// multistream-select settled on, plus the stream itself.
pub type OpenResult = Result<(UnaryProtocol, RawStream), RawStreamError>;

/// One outbound open request, from `open_stream` until the upgrade resolves.
///
/// Public only because it is the handler's `FromBehaviour` type and
/// `#[derive(NetworkBehaviour)]` leaks that into the composed behaviour's
/// interface; its fields stay private and it has no public constructor.
pub struct OutboundOpen {
    /// Candidate protocols, in preference order — `StreamOpenRequest.proto` is
    /// a *list* and Go passes the whole thing to `host.NewStream`, which lets
    /// multistream-select pick the first the remote supports.
    protocols: Vec<UnaryProtocol>,
    reply: oneshot::Sender<OpenResult>,
}

impl std::fmt::Debug for OutboundOpen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OutboundOpen")
            .field("protocols", &self.protocols)
            .finish_non_exhaustive()
    }
}

/// An inbound raw stream, ready to be piped.
#[derive(Debug)]
pub struct InboundStream {
    /// The peer that opened it, taken from the connection.
    pub peer: PeerId,
    /// The negotiated protocol — authoritative, and what the `StreamInfo`
    /// prologue reports to the socket client.
    pub proto: UnaryProtocol,
    /// The stream itself.
    pub stream: RawStream,
}

/// Events surfaced to the swarm owner.
#[derive(Debug)]
pub enum Event {
    /// A remote peer opened a raw stream on a protocol we advertise.
    InboundStream(InboundStream),
}

// ============================================================================
// Connection handler
// ============================================================================

/// Per-connection handler. Unlike [`crate::unary::Handler`] it runs no futures:
/// a negotiated stream is passed straight through to the behaviour, and its
/// lifetime belongs to whoever receives it.
pub struct Handler {
    /// Shared with [`Behaviour`]; read per `listen_protocol()` so runtime
    /// registrations reach existing connections.
    inbound_protocols: Arc<RwLock<HashSet<UnaryProtocol>>>,
    /// Open requests not yet emitted as substream requests.
    pending_outbound: VecDeque<OutboundOpen>,
    /// Emitted substream requests awaiting negotiation, keyed by the id passed
    /// as `OutboundOpenInfo`.
    ///
    /// By id for the same reason as `unary::Handler`: upgrade results arrive in
    /// completion order, and the id is the only token echoed back on both the
    /// success and the failure path.
    requested_outbound: HashMap<u64, OutboundOpen>,
    next_outbound_id: u64,
    /// Negotiated streams waiting to be drained in `poll`.
    ready: VecDeque<HandlerEvent>,
}

/// What the handler hands the behaviour.
pub enum HandlerEvent {
    /// An inbound stream negotiated on `proto`.
    Inbound {
        proto: UnaryProtocol,
        stream: RawStream,
    },
}

impl std::fmt::Debug for HandlerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandlerEvent::Inbound { proto, .. } => f
                .debug_struct("Inbound")
                .field("proto", proto)
                .finish_non_exhaustive(),
        }
    }
}

impl Handler {
    fn new(inbound_protocols: Arc<RwLock<HashSet<UnaryProtocol>>>) -> Self {
        Self {
            inbound_protocols,
            pending_outbound: VecDeque::new(),
            requested_outbound: HashMap::new(),
            next_outbound_id: 0,
            ready: VecDeque::new(),
        }
    }

    fn on_dial_upgrade_error(
        &mut self,
        error: StreamUpgradeError<std::convert::Infallible>,
        id: u64,
    ) {
        let Some(open) = self.requested_outbound.remove(&id) else {
            debug!(id, "upgrade error for a raw stream with no pending request");
            return;
        };

        let names: Vec<&str> = open.protocols.iter().map(|p| p.as_ref()).collect();
        let error = match error {
            // A negotiation timeout is indistinguishable from a refusal to the
            // caller, but the wording should not claim the remote refused.
            StreamUpgradeError::Timeout => {
                RawStreamError::Io(format!("negotiation timed out for {}", names.join(", ")))
            }
            StreamUpgradeError::NegotiationFailed => {
                RawStreamError::UnsupportedProtocol(names.join(", "))
            }
            StreamUpgradeError::Io(e) => RawStreamError::Io(e.to_string()),
            StreamUpgradeError::Apply(infallible) => match infallible {},
        };
        let _ = open.reply.send(Err(error));
    }
}

impl Drop for Handler {
    /// The connection is gone; every request that never got a stream resolves
    /// with a definite error rather than a dropped channel.
    fn drop(&mut self) {
        for open in self
            .pending_outbound
            .drain(..)
            .chain(self.requested_outbound.drain().map(|(_, o)| o))
        {
            let _ = open
                .reply
                .send(Err(RawStreamError::Io("connection closed".to_string())));
        }
    }
}

impl ConnectionHandler for Handler {
    type FromBehaviour = OutboundOpen;
    type ToBehaviour = HandlerEvent;
    type InboundProtocol = Protocols;
    type OutboundProtocol = Protocols;
    type InboundOpenInfo = ();
    type OutboundOpenInfo = u64;

    fn listen_protocol(&self) -> SubstreamProtocol<Self::InboundProtocol, ()> {
        let protocols = self
            .inbound_protocols
            .read()
            .expect("raw stream protocol set lock poisoned")
            .iter()
            .cloned()
            .collect();
        SubstreamProtocol::new(Protocols::new(protocols), ())
    }

    fn on_behaviour_event(&mut self, open: Self::FromBehaviour) {
        self.pending_outbound.push_back(open);
    }

    fn poll(
        &mut self,
        _cx: &mut Context<'_>,
    ) -> Poll<ConnectionHandlerEvent<Protocols, u64, Self::ToBehaviour>> {
        if let Some(event) = self.ready.pop_front() {
            return Poll::Ready(ConnectionHandlerEvent::NotifyBehaviour(event));
        }

        // No concurrency cap here, deliberately: a raw stream is long-lived and
        // has exactly one owner (the socket connection that asked for it), so
        // the number in flight is bounded by the number of control-socket
        // clients, not by remote behaviour. The unary cap exists because there
        // a remote can open streams faster than handlers retire them.
        if let Some(open) = self.pending_outbound.pop_front() {
            // Real protocols first, then the sentinel — see
            // `NEGOTIATION_SENTINEL`. `open.protocols` itself is left alone, so
            // error messages and bookkeeping only ever mention real names.
            let mut offered = open.protocols.clone();
            offered.push(UnaryProtocol::new(NEGOTIATION_SENTINEL));
            let protocols = Protocols::new(offered);
            let id = self.next_outbound_id;
            self.next_outbound_id += 1;
            self.requested_outbound.insert(id, open);
            return Poll::Ready(ConnectionHandlerEvent::OutboundSubstreamRequest {
                protocol: SubstreamProtocol::new(protocols, id),
            });
        }

        Poll::Pending
    }

    fn on_connection_event(
        &mut self,
        event: ConnectionEvent<Self::InboundProtocol, Self::OutboundProtocol, (), u64>,
    ) {
        match event {
            ConnectionEvent::FullyNegotiatedInbound(FullyNegotiatedInbound {
                protocol: (stream, proto),
                ..
            }) => {
                trace!(%proto, "inbound raw stream negotiated");
                self.ready
                    .push_back(HandlerEvent::Inbound { proto, stream });
            }
            ConnectionEvent::FullyNegotiatedOutbound(FullyNegotiatedOutbound {
                protocol: (stream, proto),
                info,
            }) => match self.requested_outbound.remove(&info) {
                Some(open) if proto.as_ref() == NEGOTIATION_SENTINEL => {
                    // Nothing real survived negotiation. `stream` is dropped
                    // with this arm, which resets it — the remote never sees a
                    // sentinel it would have to answer.
                    let names: Vec<&str> = open.protocols.iter().map(|p| p.as_ref()).collect();
                    trace!(protocols = %names.join(", "), "raw stream refused by the remote");
                    let _ = open
                        .reply
                        .send(Err(RawStreamError::UnsupportedProtocol(names.join(", "))));
                }
                Some(open) => {
                    trace!(%proto, "outbound raw stream negotiated");
                    // If the caller has gone away the stream is dropped here,
                    // which resets it — the correct signal to the remote.
                    let _ = open.reply.send(Ok((proto, stream)));
                }
                None => debug!(
                    %proto,
                    info, "negotiated a raw stream with no pending request"
                ),
            },
            ConnectionEvent::DialUpgradeError(DialUpgradeError { error, info }) => {
                self.on_dial_upgrade_error(error, info)
            }
            _ => {}
        }
    }
}

// ============================================================================
// Behaviour
// ============================================================================

/// The raw-stream behaviour. See the module docs for the design.
#[derive(Default)]
pub struct Behaviour {
    /// Inbound protocol registrations, shared with every connection handler.
    inbound_protocols: Arc<RwLock<HashSet<UnaryProtocol>>>,
    /// Live connections per peer.
    connected: HashMap<PeerId, Vec<ConnectionId>>,
    /// Requests to not-yet-connected peers, flushed on establishment, failed on
    /// dial failure.
    pending_opens: HashMap<PeerId, Vec<OutboundOpen>>,
    /// Events for the swarm, drained in `poll`.
    pending_events: VecDeque<ToSwarm<Event, OutboundOpen>>,
    /// Peer addresses learned from the swarm, served back on dial-on-demand.
    addresses: PeerAddresses,
}

impl Behaviour {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start advertising `proto` for inbound raw streams. Idempotent, and
    /// visible to connections that already exist.
    pub fn register_protocol(&mut self, proto: UnaryProtocol) {
        if is_reserved(&proto) {
            debug!(%proto, "refusing to register the reserved negotiation sentinel");
            return;
        }
        self.inbound_protocols
            .write()
            .expect("raw stream protocol set lock poisoned")
            .insert(proto);
    }

    /// Stop advertising `proto`. Streams already negotiated keep running — the
    /// registration governs negotiation only, matching Go's
    /// `host.RemoveStreamHandler`.
    pub fn unregister_protocol(&mut self, proto: &UnaryProtocol) {
        self.inbound_protocols
            .write()
            .expect("raw stream protocol set lock poisoned")
            .remove(proto);
    }

    /// Whether `proto` is currently advertised.
    pub fn serves(&self, proto: &UnaryProtocol) -> bool {
        self.inbound_protocols
            .read()
            .expect("raw stream protocol set lock poisoned")
            .contains(proto)
    }

    /// Open a raw stream to `peer`, negotiating the first of `protocols` the
    /// remote accepts, and resolve `reply` with the outcome.
    ///
    /// Dials on demand exactly as [`crate::unary::Behaviour::send_request`]
    /// does, because Go's `doStreamOpen` calls `host.NewStream`, which dials if
    /// there is no connection. A `STREAM_OPEN` to a known-but-unconnected peer
    /// must therefore work.
    pub fn open_stream(
        &mut self,
        peer: PeerId,
        protocols: Vec<UnaryProtocol>,
        reply: oneshot::Sender<OpenResult>,
    ) {
        if protocols.iter().any(is_reserved) {
            let _ = reply.send(Err(RawStreamError::UnsupportedProtocol(format!(
                "{NEGOTIATION_SENTINEL} is reserved"
            ))));
            return;
        }
        if protocols.is_empty() {
            let _ = reply.send(Err(RawStreamError::UnsupportedProtocol(
                "no protocols requested".to_string(),
            )));
            return;
        }

        let open = OutboundOpen { protocols, reply };

        match self.connected.get(&peer) {
            Some(connections) if !connections.is_empty() => {
                self.pending_events.push_back(ToSwarm::NotifyHandler {
                    peer_id: peer,
                    handler: NotifyHandler::One(connections[0]),
                    event: open,
                });
            }
            _ => {
                let parked = self.pending_opens.entry(peer).or_default();
                // One dial per burst; the rest ride along on the connection it
                // establishes.
                if parked.is_empty() {
                    self.pending_events.push_back(ToSwarm::Dial {
                        // A new port: binding our listen port as a dial
                        // source fails `EADDRINUSE` against our own listener.
                        opts: DialOpts::peer_id(peer).allocate_new_port().build(),
                    });
                }
                parked.push(open);
            }
        }
    }

    /// Hand queued opens for `peer` to a freshly built connection handler.
    fn preload_new_handler(&mut self, handler: &mut Handler, peer: PeerId) {
        if let Some(pending) = self.pending_opens.remove(&peer) {
            for open in pending {
                handler.on_behaviour_event(open);
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
        let mut handler = Handler::new(Arc::clone(&self.inbound_protocols));
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
        _port_use: PortUse,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        let mut handler = Handler::new(Arc::clone(&self.inbound_protocols));
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
                // Streams already handed out fail through their own I/O; the
                // relay loop sees EOF or a reset and tears down.
            }
            FromSwarm::DialFailure(libp2p::swarm::DialFailure { peer_id, error, .. }) => {
                // A dial cancelled by its own peer condition means another dial
                // to the same peer is in flight; the parked opens still get a
                // connection.
                if matches!(error, libp2p::swarm::DialError::DialPeerConditionFalse(_)) {
                    return;
                }
                if let Some(peer) = peer_id {
                    if let Some(pending) = self.pending_opens.remove(&peer) {
                        let text = error.to_string();
                        for open in pending {
                            let _ = open
                                .reply
                                .send(Err(RawStreamError::DialFailure(text.clone())));
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
        match event {
            HandlerEvent::Inbound { proto, stream } => {
                self.pending_events
                    .push_back(ToSwarm::GenerateEvent(Event::InboundStream(
                        InboundStream {
                            peer,
                            proto,
                            stream,
                        },
                    )));
            }
        }
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
