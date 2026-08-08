//! Concurrent GPU inference multiplexer over a persistent p2p stream.
//!
//! Solves the IPC serialization bottleneck in the existing `ollama_proxy`:
//! each unary-RPC call blocks while waiting for Ollama, so N workers
//! using `p2p://PEER_ID` only get ~1 Ollama slot used at a time.
//!
//! This module opens ONE yamux stream to the remote GPU node and sends
//! multiple request frames over it concurrently, correlated by `request_id`.
//! The server side spawns a tokio task per request, saturating
//! `OLLAMA_NUM_PARALLEL` slots simultaneously.
//!
//! ## Wire format
//! Each frame: `[4-byte LE length][msgpack(MuxRequest | MuxResponse)]`
//!
//! ## URL scheme
//! Use `mux://PEER_ID` in `--inference-urls` to activate.

use anyhow::{Context, Result};
use kwaai_p2p_daemon::{P2PClient, P2PStream, DEFAULT_SOCKET_NAME};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, OnceLock,
    },
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{mpsc, oneshot, Mutex, RwLock},
    task::JoinHandle,
};
use tracing::{debug, info, warn};

use crate::capacity_lease::{
    ConnectionId, LeaseHolder, LeaseId, LeaseTable, NegotiationOutcome, LEASE_TTL_SECS,
};
use crate::circuit_breaker::CircuitBreaker;

pub const MUX_PROTO: &str = "/kwaai/inference-mux/1.0.0";

// ── Wire types ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct MuxRequest {
    pub request_id: u64,
    pub method: String,
    pub path: String,
    pub body: Vec<u8>,
    /// Attached once a lease has been negotiated (see `capacity_lease.rs`).
    /// `None` for legacy peers or callers that haven't/couldn't negotiate —
    /// the server forwards unconditionally in that case, exactly as before
    /// this feature existed.
    #[serde(default)]
    pub lease_id: Option<LeaseId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MuxResponse {
    pub request_id: u64,
    pub status: u16,
    pub body: Vec<u8>,
}

/// Status the server uses on `MuxResponse` when a request's `lease_id` is
/// present but no longer valid (expired, or unknown after a server
/// restart) — distinguishable from a real Ollama HTTP status so a future
/// client refinement could react to it specifically without another wire
/// change; today's Phase 1 client doesn't yet inspect it.
pub const LEASE_EXPIRED_STATUS: u16 = 409;

/// One frame on the mux stream — the original `Request`/`Response` pair,
/// plus the Capacity Lease negotiation frames, all sharing the same
/// `[4-byte LE length][msgpack(...)]` framing via `write_frame`/`read_frame`.
/// Wrapping every variant in one externally-tagged enum keeps that framing
/// untouched; only the `rmp_serde::from_slice::<MuxFrame>` call site differs
/// from the pre-lease `::<MuxRequest>`/`::<MuxResponse>` calls.
#[derive(Debug, Serialize, Deserialize)]
pub enum MuxFrame {
    Request(MuxRequest),
    Response(MuxResponse),
    /// `model: None` when the caller doesn't know its target model yet
    /// (today's mux proxy is a generic HTTP tunnel — see
    /// `LeaseTable::try_grant`'s doc comment).
    LeaseRequest {
        request_id: u64,
        model: Option<String>,
    },
    LeaseResponse {
        request_id: u64,
        outcome: NegotiationOutcome,
    },
    /// Fire-and-forget — no reply expected or awaited.
    LeaseRelease {
        lease_id: LeaseId,
    },
    /// Fire-and-forget renewal ping, sent only as a fallback when no real
    /// request has gone out in `ttl/2` (see `LeaseHolder::needs_keepalive_probe`).
    LeaseKeepalive {
        lease_id: LeaseId,
    },
}

// ── Frame I/O ─────────────────────────────────────────────────────────────────

async fn write_frame<W: AsyncWriteExt + Unpin>(writer: &mut W, payload: &[u8]) -> Result<()> {
    let len = payload.len() as u32;
    writer.write_all(&len.to_le_bytes()).await?;
    writer.write_all(payload).await?;
    Ok(())
}

async fn read_frame<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<Vec<u8>> {
    let mut len_buf = [0u8; 4];
    reader
        .read_exact(&mut len_buf)
        .await
        .context("read frame length")?;
    let len = u32::from_le_bytes(len_buf) as usize;
    if len > 64 * 1024 * 1024 {
        anyhow::bail!("inference-mux frame too large: {len} bytes");
    }
    let mut buf = vec![0u8; len];
    reader
        .read_exact(&mut buf)
        .await
        .context("read frame body")?;
    Ok(buf)
}

// ── Protocol helpers ──────────────────────────────────────────────────────────

/// Read and discard the gogo-protobuf delimited `StreamInfo` message that
/// go-libp2p-daemon sends to a registered stream handler before piping data.
///
/// Wire format: varint(len) || proto_bytes  (same varint encoding as protobuf).
async fn read_p2pd_stream_info<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<()> {
    // Decode varint length prefix (≤ 10 bytes for u64).
    let mut len: u64 = 0;
    let mut shift = 0u32;
    for _ in 0..10 {
        let mut b = [0u8; 1];
        reader
            .read_exact(&mut b)
            .await
            .context("read p2pd StreamInfo varint")?;
        len |= ((b[0] & 0x7F) as u64) << shift;
        if b[0] & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift > 63 {
            anyhow::bail!("p2pd StreamInfo varint overflow");
        }
    }
    // Read and discard the message body.
    let mut buf = vec![0u8; len as usize];
    reader
        .read_exact(&mut buf)
        .await
        .context("read p2pd StreamInfo body")?;
    Ok(())
}

// ── Server ────────────────────────────────────────────────────────────────────

/// Start the inference-mux server: binds a local TCP port, registers it with
/// the daemon as the handler for `MUX_PROTO`, and spawns an accept loop.
///
/// `lease_table` is the node's single, process-wide Capacity Lease admission
/// gate — the SAME `Arc` must also be handed to the `/kwaai/capacity-lease`
/// unary handler (Phase 2), since both protocols dispatch to the same local
/// Ollama instance and must share one semaphore, not one each.
///
/// Call from `cmd_shard_serve()` alongside the unary proxy handlers.
pub async fn start_inference_mux_server(
    client: &mut P2PClient,
    lease_table: Arc<LeaseTable>,
) -> Result<JoinHandle<()>> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind inference-mux server")?;
    let port = listener.local_addr()?.port();
    let addr = format!("/ip4/127.0.0.1/tcp/{port}");

    client
        .register_stream_handler(&addr, vec![MUX_PROTO.to_string()])
        .await
        .context("register inference-mux stream handler")?;

    info!("inference-mux: listening on {addr}, registered as {MUX_PROTO}");

    // Fresh id per accepted connection — used only to scope
    // `lease_table.release_connection()` on stream death, not for admission
    // itself (the semaphore is shared process-wide, see `lease_table` above).
    let next_connection_id = Arc::new(AtomicU64::new(1));

    Ok(tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, peer)) => {
                    debug!("inference-mux server: accepted connection from {peer}");
                    let connection_id = next_connection_id.fetch_add(1, Ordering::Relaxed);
                    tokio::spawn(handle_mux_stream_server(
                        stream,
                        lease_table.clone(),
                        connection_id,
                    ));
                }
                Err(e) => {
                    warn!("inference-mux server accept error: {e}");
                    break;
                }
            }
        }
    }))
}

/// Handle one connected client stream — reads mux frames concurrently, calls
/// local Ollama for each `Request` (subject to the Capacity Lease admission
/// gate), writes responses back in any order.
async fn handle_mux_stream_server(
    stream: TcpStream,
    lease_table: Arc<LeaseTable>,
    connection_id: ConnectionId,
) {
    let (mut reader, writer) = stream.into_split();

    // go-libp2p-daemon sends a gogo-protobuf StreamInfo message before piping data.
    // Consume it before entering the mux frame loop. Only the p2pd path has this
    // prologue: it is the daemon describing the stream it is about to forward
    // over a *separate* TCP connection. The native path receives the libp2p
    // stream itself, so there is nothing in front of the first mux frame.
    if let Err(e) = read_p2pd_stream_info(&mut reader).await {
        warn!("inference-mux server: failed to read p2pd StreamInfo prologue: {e}");
        return;
    }
    debug!("inference-mux server: StreamInfo prologue consumed — entering mux frame loop");

    serve_mux_frames(reader, writer, lease_table, connection_id).await;
}

/// The mux frame loop, over any split stream: one task per request, replies
/// written back in completion order under a shared writer lock.
///
/// `lease_table` is the one process-wide table, so a native caller and a p2pd
/// caller contend for the same Capacity Lease slots.
async fn serve_mux_frames<R, W>(
    mut reader: R,
    writer: W,
    lease_table: Arc<LeaseTable>,
    connection_id: ConnectionId,
) where
    R: AsyncReadExt + Unpin + Send + 'static,
    W: AsyncWriteExt + Unpin + Send + 'static,
{
    let writer = Arc::new(Mutex::new(writer));
    let ttl = Duration::from_secs(LEASE_TTL_SECS);

    loop {
        let frame = match read_frame(&mut reader).await {
            Ok(f) => f,
            Err(e) => {
                debug!("inference-mux server: stream closed ({e})");
                // Eager reclaim — don't make a healthy caller's lease wait
                // out the full TTL just because THIS caller's stream died.
                lease_table.release_connection(connection_id);
                break;
            }
        };

        let mux_frame: MuxFrame = match rmp_serde::from_slice(&frame) {
            Ok(f) => f,
            Err(e) => {
                warn!("inference-mux server: bad frame: {e}");
                continue;
            }
        };

        match mux_frame {
            MuxFrame::Request(req) => {
                if let Some(lease_id) = req.lease_id {
                    if !lease_table.renew(lease_id, ttl) {
                        // Lease unknown/expired — deny without ever calling
                        // Ollama, rather than silently treating it as
                        // unleased (that would defeat the admission gate).
                        let resp = MuxResponse {
                            request_id: req.request_id,
                            status: LEASE_EXPIRED_STATUS,
                            body: b"capacity lease expired or unknown".to_vec(),
                        };
                        write_mux_frame(&writer, MuxFrame::Response(resp)).await;
                        continue;
                    }
                }
                // No lease_id (legacy caller, or a caller whose negotiation
                // was denied/unsupported) forwards unconditionally, exactly
                // as before this feature existed — zero behavior change.
                let writer = writer.clone();
                tokio::spawn(async move {
                    let resp = call_ollama_local(&req).await;
                    write_mux_frame(&writer, MuxFrame::Response(resp)).await;
                });
            }
            MuxFrame::LeaseRequest { request_id, model } => {
                let outcome = lease_table.try_grant(model.as_deref(), connection_id, ttl);
                write_mux_frame(
                    &writer,
                    MuxFrame::LeaseResponse {
                        request_id,
                        outcome,
                    },
                )
                .await;
            }
            MuxFrame::LeaseRelease { lease_id } => {
                lease_table.release(lease_id);
            }
            MuxFrame::LeaseKeepalive { lease_id } => {
                lease_table.renew(lease_id, ttl);
            }
            MuxFrame::Response(_) | MuxFrame::LeaseResponse { .. } => {
                // These are client-bound frame types; a well-behaved peer
                // never sends one to us. Ignore rather than treat as a
                // protocol error — a future/relaxed client version sending
                // an unexpected frame type shouldn't tear down the stream.
                warn!("inference-mux server: received a client-bound frame type, ignoring");
            }
        }
    }
}

/// Encode and write one `MuxFrame`, logging (not panicking) on failure.
async fn write_mux_frame<W>(writer: &Arc<Mutex<W>>, frame: MuxFrame)
where
    W: AsyncWriteExt + Unpin,
{
    match rmp_serde::to_vec_named(&frame) {
        Ok(payload) => {
            let mut w = writer.lock().await;
            if let Err(e) = write_frame(&mut *w, &payload).await {
                warn!("inference-mux server: write frame: {e}");
            }
        }
        Err(e) => warn!("inference-mux server: encode frame: {e}"),
    }
}

/// Serve `MUX_PROTO` over a [`NetworkHandle`] instead of a p2pd stream handler.
///
/// The p2pd path ([`start_inference_mux_server`]) binds a local TCP listener and
/// tells the daemon to forward inbound streams to it; the daemon writes a
/// `StreamInfo` prologue and then pipes bytes. Natively there is no forwarding
/// hop at all — [`kwaai_p2p::NetworkHandle::accept_streams`] hands us the
/// negotiated libp2p stream — so there is no listener to bind, no prologue to
/// consume, and one fewer place for backpressure to be lost.
///
/// Returns the accept-loop task. It ends when the network service shuts down and
/// drops its sender.
pub async fn start_native_inference_mux_server(
    handle: &kwaai_p2p::NetworkHandle,
    lease_table: Arc<LeaseTable>,
) -> Result<JoinHandle<()>> {
    let (mut inbound, refused) = handle
        .accept_streams(vec![MUX_PROTO.to_string()])
        .await
        .context("register native inference-mux stream handler")?;
    if !refused.is_empty() {
        anyhow::bail!("inference-mux protocol already served by another handler");
    }

    info!("inference-mux: serving {MUX_PROTO} natively");

    // Same per-connection id scheme as the p2pd path — scopes
    // `release_connection()` on stream death. The two loops keep separate
    // counters, which is fine: ids are only ever compared within one table
    // entry's own connection, never across paths.
    let next_connection_id = Arc::new(AtomicU64::new(1));

    Ok(tokio::spawn(async move {
        use tokio_util::compat::FuturesAsyncReadCompatExt;
        while let Some(stream) = inbound.recv().await {
            debug!(
                "inference-mux server: accepted native stream from {}",
                stream.peer
            );
            let (reader, writer) = tokio::io::split(stream.stream.compat());
            let connection_id = next_connection_id.fetch_add(1, Ordering::Relaxed);
            tokio::spawn(serve_mux_frames(
                reader,
                writer,
                lease_table.clone(),
                connection_id,
            ));
        }
        debug!("inference-mux server: native accept loop ended");
    }))
}

static OLLAMA_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn ollama_client() -> &'static reqwest::Client {
    OLLAMA_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("build reqwest client")
    })
}

async fn call_ollama_local(req: &MuxRequest) -> MuxResponse {
    let client = ollama_client();

    let url = format!("http://127.0.0.1:11434{}", req.path);
    let method: reqwest::Method = req.method.parse().unwrap_or(reqwest::Method::POST);

    let result = client
        .request(method, &url)
        .header("Content-Type", "application/json")
        .body(req.body.clone())
        .send()
        .await;

    match result {
        Ok(r) => {
            let status = r.status().as_u16();
            let body =
                match tokio::time::timeout(std::time::Duration::from_secs(120), r.bytes()).await {
                    Ok(Ok(b)) => b.to_vec(),
                    Ok(Err(e)) => format!("body error: {e}").into_bytes(),
                    Err(_) => b"body timeout".to_vec(),
                };
            MuxResponse {
                request_id: req.request_id,
                status,
                body,
            }
        }
        Err(e) => MuxResponse {
            request_id: req.request_id,
            status: 503,
            body: format!("upstream: {e}").into_bytes(),
        },
    }
}

// ── Client ────────────────────────────────────────────────────────────────────

/// Shared client that multiplexes N concurrent inference requests over one
/// persistent yamux stream to a remote GPU node.
pub struct InferenceMuxClient {
    peer_id: PeerId,
    breaker: Arc<CircuitBreaker>,
    next_id: AtomicU64,
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<MuxResponse>>>>,
    pending_lease: Arc<Mutex<HashMap<u64, oneshot::Sender<NegotiationOutcome>>>>,
    tx: mpsc::Sender<Vec<u8>>,
    /// Set to true when the underlying stream dies. Checked in send() to fail
    /// fast instead of hanging on a oneshot that will never be resolved.
    dead: Arc<AtomicBool>,
    /// Cached lease from the most recent successful negotiation, if any.
    /// `send()` reuses it until it needs a keepalive-equivalent refresh;
    /// see `ensure_lease()`.
    lease: RwLock<Option<Arc<LeaseHolder>>>,
}

impl InferenceMuxClient {
    pub fn is_dead(&self) -> bool {
        self.dead.load(Ordering::Acquire)
    }

    /// Open a stream to `peer_id` and start background I/O tasks. `breaker`
    /// is the SAME per-session `CircuitBreaker` `resolve_inference_urls()`
    /// already shares across every proxy for this peer — lease negotiation
    /// outcomes feed it via the taxonomy in `capacity_lease.rs`'s doc
    /// comment (only a transport-level failure trips it; a healthy
    /// `Denied*` answer does not).
    pub async fn connect(peer_id: PeerId, breaker: Arc<CircuitBreaker>) -> Result<Arc<Self>> {
        let sock =
            std::env::var("KWAAINET_SOCKET").unwrap_or_else(|_| DEFAULT_SOCKET_NAME.to_string());
        #[cfg(unix)]
        let addr = format!("/unix/{sock}");
        #[cfg(not(unix))]
        let addr = "/ip4/127.0.0.1/tcp/5005".to_string();

        let p2p = P2PClient::connect(&addr)
            .await
            .context("connect to p2pd for inference-mux stream")?;

        // stream_open_raw consumes the P2PClient and returns the daemon socket as the data
        // channel. The go-libp2p-daemon pipes the libp2p stream on the same socket after
        // sending StreamInfo — no separate TCP connection is needed or correct.
        let raw: P2PStream = p2p
            .stream_open_raw(&peer_id.to_bytes(), vec![MUX_PROTO.to_string()])
            .await
            .map_err(|e| {
                warn!(
                    "inference-mux stream_open_raw failed for peer {}: {:#}",
                    peer_id.to_base58(),
                    e
                );
                e
            })
            .context("stream_open_raw for inference-mux")?;

        let (mut reader, writer) = tokio::io::split(raw);
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(256);

        let pending: Arc<Mutex<HashMap<u64, oneshot::Sender<MuxResponse>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let pending_lease: Arc<Mutex<HashMap<u64, oneshot::Sender<NegotiationOutcome>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        let dead = Arc::new(AtomicBool::new(false));

        // Writer task: drains the send channel and writes frames to the stream.
        // On exit, marks dead and drains both pending maps so in-flight
        // send()/negotiate calls immediately receive an error rather than
        // hanging forever.
        let pending_w = pending.clone();
        let pending_lease_w = pending_lease.clone();
        let dead_w = dead.clone();
        tokio::spawn(async move {
            let mut writer = writer;
            while let Some(payload) = rx.recv().await {
                if let Err(e) = write_frame(&mut writer, &payload).await {
                    warn!("inference-mux client: writer error: {e}");
                    break;
                }
            }
            dead_w.store(true, Ordering::Release);
            pending_w.lock().await.clear();
            pending_lease_w.lock().await.clear();
        });

        // Reader task: reads frames and routes them to waiting callers —
        // Response frames via `pending`, LeaseResponse frames via
        // `pending_lease`. On exit, marks dead and drains both.
        let pending_rx = pending.clone();
        let pending_lease_rx = pending_lease.clone();
        let dead_r = dead.clone();
        tokio::spawn(async move {
            loop {
                let frame = match read_frame(&mut reader).await {
                    Ok(f) => f,
                    Err(e) => {
                        debug!("inference-mux client: reader closed ({e})");
                        break;
                    }
                };
                let mux_frame: MuxFrame = match rmp_serde::from_slice(&frame) {
                    Ok(f) => f,
                    Err(e) => {
                        warn!("inference-mux client: bad frame: {e}");
                        continue;
                    }
                };
                match mux_frame {
                    MuxFrame::Response(resp) => {
                        if let Some(s) = pending_rx.lock().await.remove(&resp.request_id) {
                            let _ = s.send(resp);
                        }
                    }
                    MuxFrame::LeaseResponse {
                        request_id,
                        outcome,
                    } => {
                        if let Some(s) = pending_lease_rx.lock().await.remove(&request_id) {
                            let _ = s.send(outcome);
                        }
                    }
                    MuxFrame::Request(_)
                    | MuxFrame::LeaseRequest { .. }
                    | MuxFrame::LeaseRelease { .. }
                    | MuxFrame::LeaseKeepalive { .. } => {
                        warn!("inference-mux client: received a server-bound frame type, ignoring");
                    }
                }
            }
            dead_r.store(true, Ordering::Release);
            pending_rx.lock().await.clear();
            pending_lease_rx.lock().await.clear();
        });

        Ok(Arc::new(Self {
            peer_id,
            breaker,
            next_id: AtomicU64::new(1),
            pending,
            pending_lease,
            tx,
            dead,
            lease: RwLock::new(None),
        }))
    }

    /// Negotiate a `Capacity Lease` slot, or reuse the cached one if it
    /// doesn't yet need a keepalive-equivalent refresh. Returns `None` on
    /// any non-`Granted` outcome (denied, unsupported peer, or the
    /// negotiation call itself failing) — callers proceed without a
    /// `lease_id` in that case, exactly as before this feature existed.
    ///
    /// `model: None` because today's mux proxy is a generic HTTP tunnel
    /// with no visibility into which model a forwarded request targets
    /// (see `LeaseTable::try_grant`'s doc comment) — this is a known
    /// simplification, not a final answer; real model-aware negotiation
    /// needs `resolve_inference_urls()` to learn the target model first,
    /// which is a separate, not-yet-resolved plumbing question.
    async fn ensure_lease(&self, model: Option<&str>) -> Option<Arc<LeaseHolder>> {
        {
            let g = self.lease.read().await;
            if let Some(holder) = g.as_ref() {
                if !holder.needs_keepalive_probe() {
                    return Some(holder.clone());
                }
            }
        }

        match self.negotiate_lease(model).await {
            NegotiationOutcome::Granted(grant) => {
                let holder = Arc::new(LeaseHolder::new(grant));
                *self.lease.write().await = Some(holder.clone());
                Some(holder)
            }
            _ => {
                // Denied*/unreachable: proceed unleased for this request.
                // Not caching a permanent "unsupported" verdict here is a
                // deliberate Phase 1 simplification — a legacy/full peer
                // gets re-negotiated roughly every ttl/2, not every single
                // request. Phase 3's DHT capability flag is the real fix
                // for skipping negotiation entirely against known-legacy
                // peers, before ever opening this stream.
                None
            }
        }
    }

    /// Send one `LeaseRequest` and translate the outcome into the
    /// appropriate `CircuitBreaker` signal — this is the one place that
    /// implements the taxonomy documented on `NegotiationOutcome`: only a
    /// transport-level failure (timeout, channel closed) is `PeerUnreachable`
    /// and trips the breaker; every answer the peer actually sends back,
    /// Granted or Denied*, is proof of life.
    async fn negotiate_lease(&self, model: Option<&str>) -> NegotiationOutcome {
        if self.dead.load(Ordering::Acquire) {
            return NegotiationOutcome::PeerUnreachable;
        }

        let request_id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending_lease.lock().await.insert(request_id, tx);

        let frame = MuxFrame::LeaseRequest {
            request_id,
            model: model.map(String::from),
        };
        let outcome = match rmp_serde::to_vec_named(&frame) {
            Ok(payload) => {
                if self.tx.send(payload).await.is_err() {
                    NegotiationOutcome::PeerUnreachable
                } else {
                    // Negotiation is a pure in-memory check on the remote
                    // side (no LLM call involved) — a short timeout is
                    // enough, and keeps a genuinely unreachable peer from
                    // stalling the caller for anywhere near the 120s a real
                    // inference request is allowed to take.
                    match tokio::time::timeout(Duration::from_secs(10), rx).await {
                        Ok(Ok(outcome)) => outcome,
                        Ok(Err(_)) | Err(_) => NegotiationOutcome::PeerUnreachable,
                    }
                }
            }
            Err(_) => NegotiationOutcome::PeerUnreachable,
        };

        self.pending_lease.lock().await.remove(&request_id);

        crate::capacity_lease::apply_breaker_outcome(&outcome, &self.peer_id, &self.breaker);

        outcome
    }

    /// Send one inference request and await the response.
    /// Multiple concurrent callers share the same underlying stream.
    pub async fn send(&self, method: &str, path: &str, body: Vec<u8>) -> Result<MuxResponse> {
        if self.dead.load(Ordering::Acquire) {
            return Err(anyhow::anyhow!("inference-mux client: stream disconnected"));
        }

        let holder = self.ensure_lease(None).await;
        let lease_id = holder.as_ref().map(|h| h.lease_id);

        let request_id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (resp_tx, resp_rx) = oneshot::channel();
        self.pending.lock().await.insert(request_id, resp_tx);

        let req = MuxRequest {
            request_id,
            method: method.to_string(),
            path: path.to_string(),
            body,
            lease_id,
        };
        let payload =
            rmp_serde::to_vec_named(&MuxFrame::Request(req)).context("encode MuxRequest")?;
        self.tx
            .send(payload)
            .await
            .map_err(|_| anyhow::anyhow!("inference-mux send channel closed"))?;

        // 120s timeout guards the rare race where the stream dies between the
        // dead-check above and this await. Normally the dead+drain path fires
        // the oneshot error in microseconds, not seconds.
        let result = tokio::time::timeout(Duration::from_secs(120), resp_rx)
            .await
            .context("inference-mux response timeout")?
            .context("inference-mux response channel closed");

        if result.is_ok() {
            if let Some(h) = holder.as_ref() {
                h.mark_request_sent();
            }
        }
        result
    }
}

// ── Local HTTP shim ───────────────────────────────────────────────────────────

// None = not yet connected; lazily opened on first request.
type SharedMuxClient = Arc<RwLock<Option<Arc<InferenceMuxClient>>>>;

/// Start a local HTTP proxy that routes all requests through a shared
/// `InferenceMuxClient` to the remote GPU node.
///
/// The stream to the remote peer is opened lazily on the first request,
/// avoiding an idle connection that the relay would drop before inference starts.
/// The proxy reconnects automatically whenever the stream dies.
///
/// Returns `(local_port, join_handle)`. Drop the handle to stop the proxy.
pub async fn start_local_mux_proxy(
    peer_id: PeerId,
    breaker: Arc<CircuitBreaker>,
) -> Result<(u16, JoinHandle<()>)> {
    let shared: SharedMuxClient = Arc::new(RwLock::new(None));

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind inference-mux local proxy")?;
    let port = listener.local_addr()?.port();

    info!(
        "inference-mux: local proxy on 127.0.0.1:{port} → mux://{peer_id} (lazy connect)",
        peer_id = peer_id.to_base58()
    );

    let handle = tokio::spawn(async move {
        loop {
            let (stream, _) = match listener.accept().await {
                Ok(s) => s,
                Err(e) => {
                    warn!("inference-mux proxy accept: {e}");
                    break;
                }
            };
            let shared = shared.clone();
            let breaker = breaker.clone();
            tokio::spawn(handle_mux_proxy_connection(
                stream, shared, peer_id, breaker,
            ));
        }
    });

    Ok((port, handle))
}

/// Return the current live client, connecting or reconnecting as needed.
/// Double-checked locking: fast read-lock path, slow write-lock path only when needed.
async fn ensure_mux_client(
    shared: &SharedMuxClient,
    peer_id: PeerId,
    breaker: Arc<CircuitBreaker>,
) -> Result<Arc<InferenceMuxClient>> {
    {
        let g = shared.read().await;
        if let Some(c) = g.as_ref() {
            if !c.is_dead() {
                return Ok(c.clone());
            }
        }
    }
    let mut g = shared.write().await;
    if let Some(c) = g.as_ref() {
        if !c.is_dead() {
            return Ok(c.clone());
        }
    }
    info!("inference-mux: (re)connecting to {}", peer_id.to_base58());
    let new_client = InferenceMuxClient::connect(peer_id, breaker).await?;
    *g = Some(new_client.clone());
    Ok(new_client)
}

/// Parse one HTTP request from a worker, forward via mux, write HTTP response back.
/// Connects lazily on first call; reconnects automatically on any stream failure.
async fn handle_mux_proxy_connection(
    mut stream: TcpStream,
    shared: SharedMuxClient,
    peer_id: PeerId,
    breaker: Arc<CircuitBreaker>,
) {
    // Two-phase read — same approach as ollama_proxy::handle_connection.
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(30);
    let mut buf: Vec<u8> = Vec::with_capacity(64 * 1024);

    let header_end = loop {
        if let Some(pos) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            break pos;
        }
        let mut tmp = [0u8; 16 * 1024];
        match tokio::time::timeout_at(deadline, stream.read(&mut tmp)).await {
            Ok(Ok(0)) | Ok(Err(_)) | Err(_) => return,
            Ok(Ok(n)) => buf.extend_from_slice(&tmp[..n]),
        }
    };

    let content_length: usize = std::str::from_utf8(&buf[..header_end])
        .unwrap_or("")
        .lines()
        .find(|l| l.to_ascii_lowercase().starts_with("content-length:"))
        .and_then(|l| l.split_once(':').map(|x| x.1))
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(0);

    let total = header_end + 4 + content_length;
    while buf.len() < total {
        let mut tmp = [0u8; 16 * 1024];
        let want = (total - buf.len()).min(tmp.len());
        match tokio::time::timeout_at(deadline, stream.read(&mut tmp[..want])).await {
            Ok(Ok(0)) | Err(_) => break,
            Ok(Ok(n)) => buf.extend_from_slice(&tmp[..n]),
            Ok(Err(_)) => return,
        }
    }

    // Circuit breaker check: fail fast if this peer has had too many
    // consecutive connection/stream failures recently.
    if !breaker.allow(&peer_id) {
        let msg = b"Circuit open - peer temporarily unavailable";
        let _ = stream
            .write_all(
                format!(
                    "HTTP/1.1 503 Service Unavailable\r\nContent-Length: {}\r\nRetry-After: 30\r\n\r\n",
                    msg.len()
                )
                .as_bytes(),
            )
            .await;
        let _ = stream.write_all(msg).await;
        return;
    }

    let (method, path, body) = match parse_http_request(&buf) {
        Some(t) => t,
        None => {
            let _ = stream
                .write_all(b"HTTP/1.1 400 Bad Request\r\nContent-Length: 11\r\n\r\nBad Request")
                .await;
            return;
        }
    };

    let resp = 'send: {
        for attempt in 0u32..2 {
            let client = match ensure_mux_client(&shared, peer_id, breaker.clone()).await {
                Ok(c) => c,
                Err(e) => {
                    warn!("inference-mux: connect failed (attempt {attempt}): {e:#}");
                    if attempt < 1 {
                        continue;
                    }
                    breaker.record_failure(&peer_id);
                    break 'send Err(e);
                }
            };
            match client.send(&method, &path, body.clone()).await {
                Ok(r) => {
                    breaker.record_success(&peer_id);
                    break 'send Ok(r);
                }
                Err(e) => {
                    warn!("inference-mux: send failed (attempt {attempt}): {e}");
                    if attempt == 0 {
                        // Invalidate so ensure_mux_client reconnects next iteration.
                        let mut g = shared.write().await;
                        if let Some(c) = g.as_ref() {
                            if Arc::ptr_eq(c, &client) {
                                *g = None;
                            }
                        }
                    } else {
                        breaker.record_failure(&peer_id);
                        break 'send Err(e);
                    }
                }
            }
        }
        Err(anyhow::anyhow!(
            "inference-mux: all retry attempts exhausted"
        ))
    };

    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            warn!("inference-mux proxy: send failed: {e}");
            let msg = b"Bad Gateway";
            let _ = stream
                .write_all(
                    format!(
                        "HTTP/1.1 502 Bad Gateway\r\nContent-Length: {}\r\n\r\n",
                        msg.len()
                    )
                    .as_bytes(),
                )
                .await;
            let _ = stream.write_all(msg).await;
            return;
        }
    };

    let status_text = match resp.status {
        200 => "OK",
        400 => "Bad Request",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        _ => "Unknown",
    };
    let header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        resp.status,
        status_text,
        resp.body.len(),
    );
    let _ = stream.write_all(header.as_bytes()).await;
    let _ = stream.write_all(&resp.body).await;
}

fn parse_http_request(raw: &[u8]) -> Option<(String, String, Vec<u8>)> {
    let sep = raw.windows(4).position(|w| w == b"\r\n\r\n")?;
    let headers = std::str::from_utf8(&raw[..sep]).ok()?;
    let body = raw[sep + 4..].to_vec();
    let mut lines = headers.lines();
    let req_line = lines.next()?;
    let mut parts = req_line.splitn(3, ' ');
    let method = parts.next()?.to_string();
    let path = parts.next()?.to_string();
    Some((method, path, body))
}
