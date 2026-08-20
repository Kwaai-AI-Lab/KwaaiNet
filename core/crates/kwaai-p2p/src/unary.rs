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
use libp2p::core::transport::PortUse;
use libp2p::core::upgrade::{InboundUpgrade, NegotiationError, OutboundUpgrade, UpgradeInfo};
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
///
/// Shared with [`crate::raw_stream`], which needs the same slash-less
/// negotiation and the same "which protocol won?" answer, and then diverges by
/// handing the stream out instead of framing on it.
pub struct Protocols {
    protocols: Vec<UnaryProtocol>,
}

impl Protocols {
    /// Advertise `protocols`, in preference order.
    pub fn new(protocols: Vec<UnaryProtocol>) -> Self {
        Self { protocols }
    }
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

    /// Outbound requests not yet emitted as substream requests. Emission is
    /// throttled in `poll` so `requested_outbound` + `outbound_workers` stays
    /// within `max_concurrent_streams`; excess requests wait here under the
    /// caller's own timeout-free patience (each has a caller awaiting it, so
    /// the queue is bounded by local concurrency).
    pending_outbound: VecDeque<OutboundMessage>,
    /// Emitted substream requests awaiting negotiation, keyed by the request
    /// id passed as `OutboundOpenInfo`.
    ///
    /// Correlation must be by id, never by position or protocol:
    /// `FullyNegotiatedOutbound`/`DialUpgradeError` arrive in *completion*
    /// order (each negotiation is an independent round-trip), and the id is
    /// the only token the swarm echoes back verbatim on both the success and
    /// the failure path.
    requested_outbound: HashMap<u64, OutboundMessage>,
    /// Source for `requested_outbound` keys, unique per handler lifetime.
    next_outbound_id: u64,

    /// Inbound stream workers (serving remote calls). Bounded by
    /// `max_concurrent_streams`, with a small overflow allowance for workers
    /// that only write a refusal frame.
    inbound_workers: FuturesUnordered<BoxFuture<'static, ()>>,
    /// Refusal-frame writers, tracked apart from `inbound_workers` so pending
    /// refusals neither consume real capacity nor inflate the measure that
    /// admits them.
    refusal_workers: FuturesUnordered<BoxFuture<'static, ()>>,
    /// Waker from the last `poll`, woken whenever a throttle-relevant slot is
    /// freed outside worker completion (`FuturesUnordered` wakes on its own
    /// completions, but `requested_outbound` shrinking on a negotiation
    /// failure does not — without this, a queued call could wait forever
    /// behind a slot freed by `DialUpgradeError`).
    waker: Option<std::task::Waker>,
    /// Outbound stream workers (our calls). Bounded by
    /// `max_concurrent_streams` via emission throttling in `poll`, so a local
    /// call burst cannot starve inbound serving. Timeouts live inside each
    /// future so every worker resolves its oneshot on every path.
    outbound_workers: FuturesUnordered<BoxFuture<'static, ()>>,

    /// Inbound requests decoded by workers, drained in `poll`. Zero-capacity:
    /// a worker parks until the handler actually forwards its request.
    inbound_rx: mpsc::Receiver<InboundStreamRequest>,
    inbound_tx: mpsc::Sender<InboundStreamRequest>,
}

/// Extra inbound worker slots reserved for writing "at capacity" refusal
/// frames once the real slots are full. Beyond cap + this, streams are
/// dropped without a reply — the remote's own timeout is then the backstop.
const REFUSAL_SLOTS: usize = 16;

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
            requested_outbound: HashMap::new(),
            next_outbound_id: 0,
            inbound_workers: FuturesUnordered::new(),
            refusal_workers: FuturesUnordered::new(),
            outbound_workers: FuturesUnordered::new(),
            waker: None,
            inbound_rx,
            inbound_tx,
        }
    }

    fn on_fully_negotiated_inbound(&mut self, mut stream: Stream, proto: UnaryProtocol) {
        if self.inbound_workers.len() >= self.config.max_concurrent_streams {
            // Refuse politely while the overflow allowance lasts: read the
            // request (its callId is needed for any reply), answer with the
            // error arm, close. Beyond the allowance, drop outright.
            if self.refusal_workers.len() < REFUSAL_SLOTS {
                warn!(%proto, "refusing inbound unary stream: at capacity");
                self.refusal_workers.push(
                    async move {
                        let refusal = async {
                            let frame = wire::read_framed_futures(&mut stream).await?;
                            let (call_id, ..) = wire::decode_unary_request(&frame)?;
                            stream
                                .write_all(&wire::encode_unary_response(
                                    &call_id,
                                    Err("node at capacity".to_string()),
                                ))
                                .await?;
                            stream.flush().await?;
                            stream.close().await?;
                            Ok::<_, wire::WireError>(())
                        };
                        // A short budget: a refusal that cannot complete
                        // quickly is not worth a slot.
                        let _ = tokio::time::timeout(Duration::from_secs(5), refusal).await;
                    }
                    .boxed(),
                );
            } else {
                warn!(%proto, "dropping inbound unary stream: at capacity");
            }
            return;
        }

        let mut to_handler = self.inbound_tx.clone();
        let timeout = self.config.request_timeout;

        self.inbound_workers.push(
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

    /// Re-run `poll` after a throttle-relevant slot was freed outside the
    /// worker pools (they wake the task themselves; map mutations do not).
    fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    fn on_fully_negotiated_outbound(&mut self, mut stream: Stream, proto: UnaryProtocol, id: u64) {
        self.wake();
        let Some(message) = self.requested_outbound.remove(&id) else {
            debug!(%proto, id, "negotiated an outbound stream without a pending message");
            return;
        };
        debug_assert_eq!(
            message.proto, proto,
            "a substream request proposes exactly one protocol"
        );

        let callee = self.remote.to_bytes();
        let timeout = self.config.request_timeout;

        self.outbound_workers.push(
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
                        .map_err(|e| wire_or_refusal(e, &proto))?;
                    stream
                        .flush()
                        .await
                        .map_err(|e| wire_or_refusal(e, &proto))?;

                    let frame = wire::read_framed_futures(&mut stream)
                        .await
                        .map_err(|e| wire_or_refusal(e, &proto))?;
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

    /// A failed upgrade carries no negotiated protocol, but it does echo the
    /// request id we passed as `OutboundOpenInfo`, so attribution is exact on
    /// the failure path too.
    fn on_dial_upgrade_error(
        &mut self,
        error: StreamUpgradeError<std::convert::Infallible>,
        id: u64,
    ) {
        self.wake();
        let Some(message) = self.requested_outbound.remove(&id) else {
            debug!(
                id,
                "upgrade error for an outbound stream without a pending message"
            );
            return;
        };

        let error = match error {
            StreamUpgradeError::Timeout => UnaryError::Timeout,
            StreamUpgradeError::NegotiationFailed => {
                UnaryError::UnsupportedProtocol(message.proto.to_string())
            }
            // No refusal arm here on purpose. Under `V1Lazy` the upgrade
            // *succeeds* and the refusal surfaces during the exchange, so it is
            // classified by `wire_or_refusal`. A guard on this arm would be dead
            // code either way: `to_stream_upgrade_error`
            // (libp2p-swarm `connection.rs`) maps `NegotiationError::Failed` to
            // `NegotiationFailed` above, and only ever puts a bare io error or a
            // `ProtocolError` inside `Io`.
            StreamUpgradeError::Io(e) => UnaryError::Wire(e.to_string()),
            StreamUpgradeError::Apply(infallible) => match infallible {},
        };
        let _ = message.reply.send(Err(error));
    }
}

/// True when an error is really "the peer does not speak this protocol"
/// wearing a different hat.
///
/// The swarm negotiates outbound substreams with `Version::V1Lazy` (see
/// `service.rs`), which hands back the stream without waiting for the peer to
/// confirm the protocol. The upgrade therefore *succeeds*, and a refusal is
/// only discovered when the exchange below does its first real I/O — surfacing
/// as an ordinary read/write failure rather than as
/// [`StreamUpgradeError::NegotiationFailed`]. Nothing is lost in the process:
/// `NegotiationError` converts into an `io::Error` that keeps the original
/// enum as its inner error, so the cause is still there to be read.
///
/// Only [`NegotiationError::Failed`] counts. `NegotiationError::ProtocolError`
/// means negotiation itself broke down — a truncated or malformed message —
/// which is a real wire failure and keeps mapping to [`UnaryError::Wire`].
///
/// **A refusal and a hang-up are different, and that is correct.** Only a
/// literal `na` reaches us as `NegotiationError::Failed`. A peer that resets or
/// hangs up mid-negotiation takes a different path — `negotiated.rs`
/// `State::Expecting` maps EOF to `ProtocolError::IoError(UnexpectedEof)`, and
/// `From<NegotiationError> for io::Error` unwraps a `ProtocolError` rather than
/// preserving the enum — so it classifies as [`UnaryError::Wire`].
///
/// This matches p2pd, which is the point: go-multistream returns its refusal
/// error only for a literal `na`, and a reset surfaces as
/// `failed to negotiate protocol: stream reset` — a wire error, never a
/// refusal. The pre-V1Lazy native path was the outlier in conflating the two.
/// Do not "fix" this toward treating a hang-up as a refusal; that breaks parity.
///
/// (The eager `dialer_select.rs::AwaitProtocol` path *does* fold EOF into
/// `Failed`, which is where the old comment came from. V1Lazy does not reach it
/// for a single-protocol offer.)
fn is_negotiation_failure(e: &(dyn std::error::Error + 'static)) -> bool {
    let mut cause = Some(e);
    while let Some(err) = cause {
        if matches!(
            err.downcast_ref::<NegotiationError>(),
            Some(NegotiationError::Failed)
        ) {
            return true;
        }
        // `io::Error` hides its inner error from `source()`, which reports the
        // inner error's *own* source instead. Step into it explicitly, or the
        // walk jumps clean past the variant we are looking for.
        cause = match err
            .downcast_ref::<std::io::Error>()
            .and_then(|io| io.get_ref())
        {
            Some(inner) => Some(inner as &(dyn std::error::Error + 'static)),
            None => err.source(),
        };
    }
    false
}

/// Classify a failed exchange: a deferred protocol refusal reads as
/// [`UnaryError::UnsupportedProtocol`], anything else as [`UnaryError::Wire`].
fn wire_or_refusal<E: std::error::Error + 'static>(e: E, proto: &UnaryProtocol) -> UnaryError {
    if is_negotiation_failure(&e) {
        UnaryError::UnsupportedProtocol(proto.to_string())
    } else {
        UnaryError::Wire(e.to_string())
    }
}

impl Drop for Handler {
    /// The connection is gone; every queued and still-negotiating request
    /// resolves with a definite error rather than a silently dropped channel.
    /// (In-flight exchanges resolve through their worker futures' own drop —
    /// the caller sees the channel close, which the handle maps to a service
    /// error; only the not-yet-started calls can be given the precise cause.)
    fn drop(&mut self) {
        for message in self
            .pending_outbound
            .drain(..)
            .chain(self.requested_outbound.drain().map(|(_, m)| m))
        {
            let _ = message
                .reply
                .send(Err(UnaryError::Wire("connection closed".to_string())));
        }
    }
}

impl ConnectionHandler for Handler {
    type FromBehaviour = OutboundMessage;
    type ToBehaviour = InboundStreamRequestEvent;
    type InboundProtocol = Protocols;
    type OutboundProtocol = Protocols;
    type InboundOpenInfo = ();
    type OutboundOpenInfo = u64;

    fn listen_protocol(&self) -> SubstreamProtocol<Self::InboundProtocol, ()> {
        let protocols = self
            .inbound_protocols
            .read()
            .expect("inbound protocol set lock poisoned")
            .iter()
            .cloned()
            .collect();
        SubstreamProtocol::new(Protocols::new(protocols), ())
    }

    fn on_behaviour_event(&mut self, message: Self::FromBehaviour) {
        self.pending_outbound.push_back(message);
    }

    fn poll(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<ConnectionHandlerEvent<Protocols, u64, Self::ToBehaviour>> {
        // Drive both worker pools; outcomes travel through oneshots, so
        // completion here is just cleanup.
        while let Poll::Ready(Some(())) = self.inbound_workers.poll_next_unpin(cx) {}
        while let Poll::Ready(Some(())) = self.refusal_workers.poll_next_unpin(cx) {}
        while let Poll::Ready(Some(())) = self.outbound_workers.poll_next_unpin(cx) {}

        if let Poll::Ready(Some(request)) = self.inbound_rx.poll_next_unpin(cx) {
            return Poll::Ready(ConnectionHandlerEvent::NotifyBehaviour(
                InboundStreamRequestEvent(request),
            ));
        }

        // Emit the next outbound request only while within the cap, counting
        // both still-negotiating and in-flight streams. Requests beyond it
        // wait in `pending_outbound` and are re-examined when a worker
        // completes (worker completion wakes this poll).
        if self.requested_outbound.len() + self.outbound_workers.len()
            < self.config.max_concurrent_streams
        {
            if let Some(message) = self.pending_outbound.pop_front() {
                let protocols = Protocols::new(vec![message.proto.clone()]);
                let id = self.next_outbound_id;
                self.next_outbound_id += 1;
                self.requested_outbound.insert(id, message);
                return Poll::Ready(ConnectionHandlerEvent::OutboundSubstreamRequest {
                    protocol: SubstreamProtocol::new(protocols, id),
                });
            }
        }

        self.waker = Some(cx.waker().clone());
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
            }) => self.on_fully_negotiated_inbound(stream, proto),
            ConnectionEvent::FullyNegotiatedOutbound(FullyNegotiatedOutbound {
                protocol: (stream, proto),
                info,
            }) => self.on_fully_negotiated_outbound(stream, proto, info),
            ConnectionEvent::DialUpgradeError(DialUpgradeError { error, info }) => {
                self.on_dial_upgrade_error(error, info)
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
                        // A new port: binding our listen port as a dial
                        // source fails `EADDRINUSE` against our own listener.
                        opts: DialOpts::peer_id(peer).allocate_new_port().build(),
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
        _port_use: PortUse,
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

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::core::upgrade::ProtocolError;

    /// How multistream-select actually surfaces a refusal: the enum is moved
    /// into an `io::Error` as its inner error (`From<NegotiationError>`).
    fn refusal() -> std::io::Error {
        NegotiationError::Failed.into()
    }

    // -- the case that costs a round trip if it regresses -----------------

    #[test]
    fn a_wrapped_negotiation_refusal_is_recognised() {
        assert!(is_negotiation_failure(&refusal()));
    }

    #[test]
    fn a_refusal_nested_behind_another_error_is_recognised() {
        // `wire::WireError::Io` and friends wrap the io error rather than
        // replacing it, so the cause has to be walked, not just read.
        #[derive(Debug, thiserror::Error)]
        #[error("framing: {0}")]
        struct Wrapper(#[from] std::io::Error);

        assert!(is_negotiation_failure(&Wrapper(refusal())));
    }

    #[test]
    fn the_walk_steps_through_io_errors_own_inner_error() {
        // Guards the reason this is not a plain `source()` loop: `io::Error`
        // reports the *inner error's* source, never the inner error itself, so
        // a naive walk skips straight past `NegotiationError`.
        let e = refusal();
        assert!(
            std::error::Error::source(&e).is_none(),
            "io::Error still hides its inner error from source(); \
             if this ever changes the walk can be simplified"
        );
        assert!(is_negotiation_failure(&e));
    }

    // -- what must keep reading as a wire failure -------------------------

    #[test]
    fn a_broken_negotiation_is_not_a_refusal() {
        // A malfunctioning negotiation must not read as "unsupported" — that
        // would report a corrupt stream as a peer that does not serve the
        // handler.
        //
        // Construct the wrapper explicitly rather than via
        // `NegotiationError::ProtocolError(..).into()`: that conversion unwraps
        // to a plain `ProtocolError`-derived io error, so the assertion would
        // hold no matter what `is_negotiation_failure` did.
        #[derive(Debug, thiserror::Error)]
        #[error("negotiation: {0}")]
        struct Wrapped(#[source] NegotiationError);

        let e = Wrapped(NegotiationError::ProtocolError(
            ProtocolError::InvalidMessage,
        ));
        assert!(
            !is_negotiation_failure(&e),
            "only NegotiationError::Failed is a refusal; ProtocolError is a wire failure"
        );
    }

    #[test]
    fn an_ordinary_io_error_is_not_a_refusal() {
        let e = std::io::Error::new(std::io::ErrorKind::ConnectionReset, "reset by peer");
        assert!(!is_negotiation_failure(&e));
    }

    // -- the classifier the exchange path uses ----------------------------

    #[test]
    fn wire_or_refusal_names_the_protocol_it_could_not_negotiate() {
        let proto = UnaryProtocol::new("DHTProtocol.rpc_store");
        match wire_or_refusal(refusal(), &proto) {
            UnaryError::UnsupportedProtocol(p) => assert_eq!(p, "DHTProtocol.rpc_store"),
            other => panic!("a refusal must not read as a wire failure: {other:?}"),
        }
    }

    #[test]
    fn wire_or_refusal_leaves_real_failures_alone() {
        let proto = UnaryProtocol::new("hello");
        let e = std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "truncated frame");
        assert!(matches!(wire_or_refusal(e, &proto), UnaryError::Wire(_)));
    }
}
