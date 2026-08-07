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

use kwaai_ledger::{LeaseQuote, Receipt, SignedLeaseGrant, WorkClaim};

use crate::capacity_lease::{
    ConnectionId, LeaseHolder, LeaseId, LeaseTable, NegotiationOutcome, LEASE_TTL_SECS,
};
use crate::circuit_breaker::CircuitBreaker;
use crate::ledger_node::{did_for_peer, LedgerNode};

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
    /// Provider's signed claim for the work this response represents (see
    /// `kwaai-ledger`). Carried on the response itself rather than as a
    /// separate frame so the claim and the exact bytes it is a digest of can
    /// never be correlated wrongly or arrive out of order.
    ///
    /// `None` whenever accounting doesn't apply: a legacy peer, a peer with no
    /// ledger, an unleased request (no quote to price it against), or a
    /// response with no token counts to meter (embeddings, errors).
    #[serde(default)]
    pub claim: Option<WorkClaim>,
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
        /// Provider-signed quote fixing the price for work done under this
        /// lease. Present only when `outcome` is `Granted` *and* the provider
        /// has a working ledger.
        ///
        /// Deliberately a sibling of `outcome` rather than a field inside
        /// `LeaseGrant`: the lease table is pure ephemeral admission control and
        /// `NegotiationOutcome` stays `Copy`. Accounting rides alongside
        /// capacity, it doesn't live inside it.
        #[serde(default)]
        quote: Option<SignedLeaseGrant>,
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
    /// Consumer's counter-signature, completing a receipt. No reply is awaited,
    /// but the write is flushed before the consumer considers the request done
    /// — see `InferenceMuxClient::settle`.
    ///
    /// A lost ack leaves the two sides **disagreeing**: the consumer has already
    /// recorded what it spent, while the provider still holds only an unpayable
    /// claim. With no arbiter that divergence cannot be fully eliminated, so the
    /// provider's unsigned-claim ratio is advisory only and must never be the
    /// sole basis for denying a peer.
    ReceiptAck {
        receipt: Receipt,
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

// NOTE: the StreamInfo prologue used to be read-and-discarded by a local
// reimplementation of the varint framing here. It is now parsed via
// `kwaai_p2p_daemon::stream::parse_stream_info`, which decodes the message
// properly and yields the authenticated caller's PeerId — see
// `handle_mux_stream_server`.

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
///
/// `ledger` is `None` on a node that cannot participate in accounting (RSA
/// identity, unwritable state dir) — in which case this server behaves exactly
/// as it did before the ledger existed: no quote, no claim, no receipt.
pub async fn start_inference_mux_server(
    client: &mut P2PClient,
    lease_table: Arc<LeaseTable>,
    ledger: Option<Arc<LedgerNode>>,
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
                        ledger.clone(),
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
    mut stream: TcpStream,
    lease_table: Arc<LeaseTable>,
    ledger: Option<Arc<LedgerNode>>,
    connection_id: ConnectionId,
) {
    // go-libp2p-daemon sends a varint-framed StreamInfo prologue before piping
    // data. Parse it (rather than discarding it) — `StreamInfo.peer` is the
    // *authenticated* caller, which is what a signed receipt must be bound to.
    // Done before splitting, since the shared helper takes the whole TcpStream.
    let caller = match kwaai_p2p_daemon::stream::parse_stream_info(&mut stream).await {
        Ok(info) => match libp2p::PeerId::from_bytes(&info.peer) {
            Ok(p) => p,
            Err(e) => {
                warn!("inference-mux server: unparseable peer id in StreamInfo prologue: {e}");
                return;
            }
        },
        Err(e) => {
            warn!("inference-mux server: failed to read p2pd StreamInfo prologue: {e}");
            return;
        }
    };
    debug!(
        "inference-mux server: StreamInfo prologue consumed — caller {} — entering mux frame loop",
        caller.to_base58()
    );

    let consumer_did = did_for_peer(&caller);

    let (mut reader, writer) = stream.into_split();
    let writer = Arc::new(Mutex::new(writer));

    let ttl = Duration::from_secs(LEASE_TTL_SECS);

    // Quotes we've signed on this connection, so a served request can be priced
    // against the exact quote its lease was granted under. Connection-scoped
    // rather than stored in `LeaseTable`: leases on the mux path are already
    // connection-scoped (`release_connection`), and keeping ledger state out of
    // the admission gate is what lets the gate stay ephemeral. A dropped stream
    // discards the quotes along with the leases they priced.
    let quotes: Arc<std::sync::Mutex<HashMap<LeaseId, LeaseQuote>>> =
        Arc::new(std::sync::Mutex::new(HashMap::new()));

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
                            claim: None,
                        };
                        write_mux_frame(&writer, MuxFrame::Response(resp)).await;
                        continue;
                    }
                }
                // No lease_id (legacy caller, or a caller whose negotiation
                // was denied/unsupported) forwards unconditionally, exactly
                // as before this feature existed — zero behavior change.
                //
                // An unleased request also goes unbilled: there's no quote to
                // price it against, and inventing one after the fact would be
                // exactly the unilateral pricing this design rules out.
                let quote = req
                    .lease_id
                    .and_then(|id| quotes.lock().ok().and_then(|q| q.get(&id).cloned()));
                let writer = writer.clone();
                let ledger = ledger.clone();
                tokio::spawn(async move {
                    let mut resp = call_ollama_local(&req).await;
                    if let (Some(ledger), Some(quote)) = (ledger.as_ref(), quote.as_ref()) {
                        if let Some(claim) =
                            ledger.claim_for_response(quote, req.request_id, &resp.body)
                        {
                            // Recorded before it goes out, so a consumer that
                            // takes delivery and never acks is visible rather
                            // than merely absent.
                            ledger.record_unsigned_claim(&claim);
                            resp.claim = Some(claim);
                        }
                    }
                    write_mux_frame(&writer, MuxFrame::Response(resp)).await;
                });
            }
            MuxFrame::LeaseRequest { request_id, model } => {
                let outcome = lease_table.try_grant(model.as_deref(), connection_id, ttl);

                // Quote only a lease we actually granted — a denial has no
                // price, and signing one would imply an obligation neither side
                // has.
                let quote = match (&outcome, ledger.as_ref()) {
                    (NegotiationOutcome::Granted(grant), Some(ledger)) => {
                        let signed = ledger.sign_quote(
                            grant.lease_id,
                            consumer_did.clone(),
                            model.clone().unwrap_or_default(),
                            grant.ttl_secs,
                        );
                        if let (Some(signed), Ok(mut q)) = (&signed, quotes.lock()) {
                            // Drop quotes for leases the table has already
                            // reclaimed. A lease that lapses by TTL never fires
                            // `LeaseRelease`, so without this the map would grow
                            // for the life of a long-running connection.
                            q.retain(|id, _| lease_table.is_active(*id));
                            q.insert(grant.lease_id, signed.quote.clone());
                        }
                        signed
                    }
                    _ => None,
                };

                write_mux_frame(
                    &writer,
                    MuxFrame::LeaseResponse {
                        request_id,
                        outcome,
                        quote,
                    },
                )
                .await;
            }
            MuxFrame::LeaseRelease { lease_id } => {
                lease_table.release(lease_id);
                if let Ok(mut q) = quotes.lock() {
                    q.remove(&lease_id);
                }
            }
            MuxFrame::LeaseKeepalive { lease_id } => {
                lease_table.renew(lease_id, ttl);
            }
            MuxFrame::ReceiptAck { receipt } => {
                // `record_receipt` verifies both signatures before storing, so a
                // forged or mismatched ack is dropped rather than counted. Also
                // check the counterparty: a peer may only ack work *it* consumed,
                // or one peer could settle another's debts.
                if receipt.consumer_did() != consumer_did {
                    warn!(
                        "inference-mux server: ignoring ReceiptAck naming {} from caller {}",
                        receipt.consumer_did(),
                        consumer_did
                    );
                } else if let Some(ledger) = ledger.as_ref() {
                    ledger.record_receipt(&receipt);
                }
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

/// Encode and write one `MuxFrame`, logging (not panicking) on failure —
/// mirrors the original inline write-response logic, now shared across the
/// several places the server writes a frame back.
async fn write_mux_frame(writer: &Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>, frame: MuxFrame) {
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
                // Attached by the caller once the ledger has priced it — this
                // function stays purely an HTTP forwarder.
                claim: None,
            }
        }
        Err(e) => MuxResponse {
            request_id: req.request_id,
            status: 503,
            body: format!("upstream: {e}").into_bytes(),
            claim: None,
        },
    }
}

// ── Client ────────────────────────────────────────────────────────────────────

/// A frame queued for the writer task, plus an optional signal fired once the
/// bytes have actually reached the socket.
///
/// The distinction is load-bearing. Enqueueing is not delivery: a short-lived
/// CLI process (a one-shot `rag query`, say) can exit between the enqueue and
/// the write, and the frame is simply lost. That silently dropped every receipt
/// ack from such a process, leaving the consumer recording what it spent while
/// the provider recorded only an unsigned claim — found by running the two-node
/// live test, not by any unit test.
type Outbound = (Vec<u8>, Option<oneshot::Sender<()>>);

/// What a `LeaseResponse` frame carries back to the awaiting negotiator: the
/// admission answer plus, when granted by a ledger-capable provider, the signed
/// quote that prices work under it.
///
/// Bundled into one type rather than resolved through two oneshot channels so
/// the outcome and its quote can never be paired wrongly.
#[derive(Debug)]
struct LeaseAnswer {
    outcome: NegotiationOutcome,
    quote: Option<SignedLeaseGrant>,
}

impl LeaseAnswer {
    /// Synthesized client-side when the negotiation call itself fails. Never
    /// carries a quote — there is no provider signature to have received.
    fn unreachable() -> Self {
        Self {
            outcome: NegotiationOutcome::PeerUnreachable,
            quote: None,
        }
    }
}

/// Shared client that multiplexes N concurrent inference requests over one
/// persistent yamux stream to a remote GPU node.
pub struct InferenceMuxClient {
    peer_id: PeerId,
    breaker: Arc<CircuitBreaker>,
    next_id: AtomicU64,
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<MuxResponse>>>>,
    pending_lease: Arc<Mutex<HashMap<u64, oneshot::Sender<LeaseAnswer>>>>,
    tx: mpsc::Sender<Outbound>,
    /// Set to true when the underlying stream dies. Checked in send() to fail
    /// fast instead of hanging on a oneshot that will never be resolved.
    dead: Arc<AtomicBool>,
    /// Cached lease from the most recent successful negotiation, if any.
    /// `send()` reuses it until it needs a keepalive-equivalent refresh;
    /// see `ensure_lease()`.
    lease: RwLock<Option<Lease>>,
    /// This node's ledger, or `None` if it can't do accounting. When `None` the
    /// client simply never counter-signs, which costs the provider a payable
    /// receipt but changes nothing about the inference itself.
    ledger: Option<Arc<LedgerNode>>,
}

/// A granted lease plus the quote that prices work under it. The quote is
/// `None` against a provider with no ledger (or a legacy one), in which case no
/// receipt is ever produced for this lease.
#[derive(Clone)]
struct Lease {
    holder: Arc<LeaseHolder>,
    grant: Option<Arc<SignedLeaseGrant>>,
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
        let (tx, mut rx) = mpsc::channel::<Outbound>(256);

        let pending: Arc<Mutex<HashMap<u64, oneshot::Sender<MuxResponse>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let pending_lease: Arc<Mutex<HashMap<u64, oneshot::Sender<LeaseAnswer>>>> =
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
            while let Some((payload, flushed)) = rx.recv().await {
                if let Err(e) = write_frame(&mut writer, &payload).await {
                    warn!("inference-mux client: writer error: {e}");
                    break;
                }
                // Signal only after the bytes are actually handed to the socket.
                // Dropping the sender on the error path above is what tells a
                // waiter the write did not happen.
                if let Some(f) = flushed {
                    let _ = f.send(());
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
                        quote,
                    } => {
                        if let Some(s) = pending_lease_rx.lock().await.remove(&request_id) {
                            let _ = s.send(LeaseAnswer { outcome, quote });
                        }
                    }
                    MuxFrame::Request(_)
                    | MuxFrame::LeaseRequest { .. }
                    | MuxFrame::LeaseRelease { .. }
                    | MuxFrame::LeaseKeepalive { .. }
                    | MuxFrame::ReceiptAck { .. } => {
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
            ledger: LedgerNode::shared(),
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
    async fn ensure_lease(&self, model: Option<&str>) -> Option<Lease> {
        {
            let g = self.lease.read().await;
            if let Some(lease) = g.as_ref() {
                if !lease.holder.needs_keepalive_probe() {
                    return Some(lease.clone());
                }
            }
        }

        let answer = self.negotiate_lease(model).await;
        match answer.outcome {
            NegotiationOutcome::Granted(grant) => {
                // A quote naming someone else is not ours to work under —
                // rejecting it is the whole reason the quote binds a consumer.
                // Drop just the quote, not the lease: capacity was still
                // granted, so serve the request unbilled rather than refusing.
                let quote = answer.quote.filter(|g| {
                    let ours = self
                        .ledger
                        .as_ref()
                        .is_some_and(|l| l.did() == g.quote.consumer_did);
                    if !ours {
                        warn!(
                            "inference-mux client: ignoring a quote issued to {} — not us",
                            g.quote.consumer_did
                        );
                    }
                    ours
                });
                let lease = Lease {
                    holder: Arc::new(LeaseHolder::new(grant)),
                    grant: quote.map(Arc::new),
                };
                *self.lease.write().await = Some(lease.clone());
                Some(lease)
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
    async fn negotiate_lease(&self, model: Option<&str>) -> LeaseAnswer {
        if self.dead.load(Ordering::Acquire) {
            return LeaseAnswer::unreachable();
        }

        let request_id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending_lease.lock().await.insert(request_id, tx);

        let frame = MuxFrame::LeaseRequest {
            request_id,
            model: model.map(String::from),
        };
        let answer = match rmp_serde::to_vec_named(&frame) {
            Ok(payload) => {
                if self.tx.send((payload, None)).await.is_err() {
                    LeaseAnswer::unreachable()
                } else {
                    // Negotiation is a pure in-memory check on the remote
                    // side (no LLM call involved) — a short timeout is
                    // enough, and keeps a genuinely unreachable peer from
                    // stalling the caller for anywhere near the 120s a real
                    // inference request is allowed to take.
                    match tokio::time::timeout(Duration::from_secs(10), rx).await {
                        Ok(Ok(answer)) => answer,
                        Ok(Err(_)) | Err(_) => LeaseAnswer::unreachable(),
                    }
                }
            }
            Err(_) => LeaseAnswer::unreachable(),
        };

        self.pending_lease.lock().await.remove(&request_id);

        crate::capacity_lease::apply_breaker_outcome(&answer.outcome, &self.peer_id, &self.breaker);

        answer
    }

    /// Send one inference request and await the response.
    /// Multiple concurrent callers share the same underlying stream.
    pub async fn send(&self, method: &str, path: &str, body: Vec<u8>) -> Result<MuxResponse> {
        if self.dead.load(Ordering::Acquire) {
            return Err(anyhow::anyhow!("inference-mux client: stream disconnected"));
        }

        let lease = self.ensure_lease(None).await;
        let lease_id = lease.as_ref().map(|l| l.holder.lease_id);

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
            .send((payload, None))
            .await
            .map_err(|_| anyhow::anyhow!("inference-mux send channel closed"))?;

        // 120s timeout guards the rare race where the stream dies between the
        // dead-check above and this await. Normally the dead+drain path fires
        // the oneshot error in microseconds, not seconds.
        let mut result = tokio::time::timeout(Duration::from_secs(120), resp_rx)
            .await
            .context("inference-mux response timeout")?
            .context("inference-mux response channel closed");

        if let Ok(resp) = result.as_mut() {
            if let Some(l) = lease.as_ref() {
                l.holder.mark_request_sent();
            }
            self.settle(resp, lease.as_ref()).await;
        }
        result
    }

    /// Counter-sign the provider's claim for this response and ack it.
    ///
    /// Takes the claim out of the response: it has served its purpose by this
    /// point, and leaving it attached would hand every caller of `send()` an
    /// artifact it has no reason to inspect.
    ///
    /// Silent on every failure path. A response we cannot bill for is still a
    /// perfectly good response, and refusing to return inference results because
    /// the accounting did not line up would make the ledger a liability rather
    /// than a feature.
    async fn settle(&self, resp: &mut MuxResponse, lease: Option<&Lease>) {
        let Some(claim) = resp.claim.take() else {
            return;
        };
        let Some(ledger) = self.ledger.as_ref() else {
            return;
        };
        let Some(grant) = lease.and_then(|l| l.grant.as_ref()) else {
            // A claim with no quote to check it against — either the provider
            // sent one unsolicited, or our quote was rejected above. Either way
            // there is nothing to verify the price against.
            debug!("inference-mux client: dropping a claim that arrived without a quote");
            return;
        };

        // Verification and counter-signing are pure CPU over a few hundred bytes
        // (one Ed25519 verify each, plus one sign), so doing it inline is
        // cheaper than dispatching to a blocking pool.
        let Some(receipt) = ledger.counter_sign(claim, grant, &resp.body) else {
            return;
        };

        let payload = match rmp_serde::to_vec_named(&MuxFrame::ReceiptAck { receipt }) {
            Ok(p) => p,
            Err(e) => {
                warn!("inference-mux client: encoding ReceiptAck: {e}");
                return;
            }
        };

        let (flushed_tx, flushed_rx) = oneshot::channel();
        if self.tx.send((payload, Some(flushed_tx))).await.is_err() {
            debug!("inference-mux client: could not queue ReceiptAck, stream closed");
            return;
        }

        // Wait for the ack to actually leave the socket before reporting the
        // request complete. This is not a round trip — we never wait on the
        // provider — but without it a process that exits promptly after its last
        // response drops the ack on the floor, and the two sides then disagree
        // about what was owed.
        match tokio::time::timeout(Duration::from_secs(5), flushed_rx).await {
            Ok(Ok(())) => {}
            Ok(Err(_)) => {
                debug!("inference-mux client: writer died before the ReceiptAck went out")
            }
            Err(_) => warn!("inference-mux client: timed out flushing a ReceiptAck"),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    // These mirror the v0.5.5 wire types exactly — the shape running on
    // metro-linux and metro-win right now. Decoding *our* frames into *these*
    // is the compatibility direction that actually matters: an old peer must
    // tolerate the ledger fields it has never heard of.
    #[derive(Debug, Deserialize)]
    struct LegacyMuxResponse {
        request_id: u64,
        status: u16,
        body: Vec<u8>,
    }

    // Every v0.5.5 variant is listed even though the tests only read two,
    // because the point of this type is to be a faithful copy of what the old
    // peers accept — trimming it to what's used would quietly stop proving that.
    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    enum LegacyMuxFrame {
        Request(MuxRequest),
        Response(LegacyMuxResponse),
        LeaseRequest {
            request_id: u64,
            model: Option<String>,
        },
        LeaseResponse {
            request_id: u64,
            outcome: NegotiationOutcome,
        },
        LeaseRelease {
            lease_id: LeaseId,
        },
        LeaseKeepalive {
            lease_id: LeaseId,
        },
    }

    /// Same encoder the wire uses, so these tests exercise the real framing
    /// rather than a convenient stand-in.
    fn enc(frame: &MuxFrame) -> Vec<u8> {
        rmp_serde::to_vec_named(frame).expect("encode frame")
    }

    /// Build a valid go-libp2p-daemon StreamInfo prologue naming `peer` as the
    /// authenticated caller — the same bytes the real daemon writes before it
    /// pipes a stream to our handler.
    ///
    /// Hand-encoded rather than built with `prost`: kwaai-cli is on prost 0.12
    /// (workspace) while kwaai-p2p-daemon generates `StreamInfo` against prost
    /// 0.13, so `StreamInfo`'s `Message` impl is for a trait this crate cannot
    /// name. Two length-delimited fields is less machinery than reconciling that
    /// skew, and keeps the test pinned to the wire format rather than to a
    /// generated type.
    fn stream_info_prologue(peer: &PeerId) -> Vec<u8> {
        fn len_delimited(field: u8, bytes: &[u8], out: &mut Vec<u8>) {
            out.push((field << 3) | 2); // wire type 2 = length-delimited
            let mut buf = unsigned_varint::encode::u64_buffer();
            out.extend_from_slice(unsigned_varint::encode::u64(bytes.len() as u64, &mut buf));
            out.extend_from_slice(bytes);
        }

        // StreamInfo { peer = 1, addr = 2, proto = 3 } — `addr` is unset, which
        // proto3 encodes as absent.
        let mut payload = Vec::new();
        len_delimited(1, &peer.to_bytes(), &mut payload);
        len_delimited(3, MUX_PROTO.as_bytes(), &mut payload);

        // The prologue itself is varint-length-framed; see
        // `kwaai_p2p_daemon::stream::parse_stream_info`.
        let mut buf = unsigned_varint::encode::u64_buffer();
        let mut out = unsigned_varint::encode::u64(payload.len() as u64, &mut buf).to_vec();
        out.extend_from_slice(&payload);
        out
    }

    fn identity_from(seed: [u8; 32]) -> (ed25519_dalek::SigningKey, PeerId, String) {
        let signing = ed25519_dalek::SigningKey::from_bytes(&seed);
        let pk = libp2p::identity::ed25519::PublicKey::try_from_bytes(
            signing.verifying_key().as_bytes(),
        )
        .expect("valid ed25519 public key");
        let peer = PeerId::from_public_key(&libp2p::identity::PublicKey::from(pk));
        let did = did_for_peer(&peer);
        (signing, peer, did)
    }

    async fn read_one_frame(stream: &mut TcpStream) -> MuxFrame {
        let bytes = read_frame(stream).await.expect("read frame");
        rmp_serde::from_slice(&bytes).expect("decode frame")
    }

    async fn send_one_frame(stream: &mut TcpStream, frame: &MuxFrame) {
        let payload = rmp_serde::to_vec_named(frame).expect("encode frame");
        write_frame(stream, &payload).await.expect("write frame");
    }

    /// Full three-step exchange against the real server handler over a real TCP
    /// connection, with a real StreamInfo prologue, real Ed25519 signatures, a
    /// live local Ollama, and two independent SQLite ledgers.
    ///
    /// Ignored because it needs Ollama on 127.0.0.1:11434 with llama3.2:3b. Run
    /// with:
    ///   cargo test -p kwaainet --profile ci -- --ignored --nocapture full_receipt_exchange
    #[tokio::test]
    #[ignore = "requires a local Ollama with llama3.2:3b"]
    async fn full_receipt_exchange_between_two_real_ledgers() {
        let (provider_key, _provider_peer, provider_did) = identity_from([21u8; 32]);
        let (consumer_key, consumer_peer, consumer_did) = identity_from([22u8; 32]);

        let provider_ledger = LedgerNode::in_memory(provider_key, provider_did.clone());
        let consumer_ledger = LedgerNode::in_memory(consumer_key, consumer_did.clone());

        // Real admission gate, one slot, so the grant path is the real one.
        let lease_table = crate::capacity_lease::LeaseTable::new("llama3.2:3b".to_string(), 1);

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().unwrap();
        {
            let lease_table = lease_table.clone();
            let ledger = provider_ledger.clone();
            tokio::spawn(async move {
                let (sock, _) = listener.accept().await.expect("accept");
                handle_mux_stream_server(sock, lease_table, Some(ledger), 1).await;
            });
        }

        let mut client = TcpStream::connect(addr).await.expect("connect");
        client
            .write_all(&stream_info_prologue(&consumer_peer))
            .await
            .expect("write prologue");

        // ── 1. Negotiate, and get a signed quote back ────────────────────────
        send_one_frame(
            &mut client,
            &MuxFrame::LeaseRequest {
                request_id: 1,
                model: None,
            },
        )
        .await;

        let (lease_id, grant) = match read_one_frame(&mut client).await {
            MuxFrame::LeaseResponse {
                outcome: NegotiationOutcome::Granted(g),
                quote: Some(q),
                ..
            } => (g.lease_id, q),
            other => panic!("expected a granted lease with a quote, got {other:?}"),
        };
        grant
            .verify()
            .expect("the quote must carry a valid provider signature");
        assert_eq!(
            grant.quote.consumer_did, consumer_did,
            "the quote must be bound to the authenticated caller, not a self-declared id"
        );
        assert_eq!(grant.quote.provider_did, provider_did);

        // ── 2. Real inference, and a claim over the exact bytes returned ─────
        let body = serde_json::json!({
            "model": "llama3.2:3b",
            "messages": [{"role": "user", "content": "Reply with the single word: ok"}],
            "stream": false,
            "options": {"num_ctx": 2048, "num_predict": 8},
        });
        send_one_frame(
            &mut client,
            &MuxFrame::Request(MuxRequest {
                request_id: 7,
                method: "POST".into(),
                path: "/api/chat".into(),
                body: serde_json::to_vec(&body).unwrap(),
                lease_id: Some(lease_id),
            }),
        )
        .await;

        let resp = match read_one_frame(&mut client).await {
            MuxFrame::Response(r) => r,
            other => panic!("expected a Response, got {other:?}"),
        };
        assert_eq!(
            resp.status,
            200,
            "ollama said: {}",
            String::from_utf8_lossy(&resp.body)
        );
        let claim = resp
            .claim
            .expect("a leased, metered response must carry a claim");

        // The provider is holding this as unpayable until we counter-sign.
        let pending = provider_ledger.balances().unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].earned, 0, "an unsigned claim is not income");
        assert_eq!(pending[0].unsigned_claims, 1);

        // ── 3. Counter-sign and ack ──────────────────────────────────────────
        let (prompt_tokens, completion_tokens) =
            (claim.payload.prompt_tokens, claim.payload.completion_tokens);
        let receipt = consumer_ledger
            .counter_sign(claim, &grant, &resp.body)
            .expect("the claim must verify against the quote and the delivered bytes");
        let receipt_id = receipt.receipt_id().unwrap();
        send_one_frame(&mut client, &MuxFrame::ReceiptAck { receipt }).await;

        // The ack is fire-and-forget, so poll rather than assume it has landed.
        let mut provider_view = None;
        for _ in 0..50 {
            tokio::time::sleep(Duration::from_millis(20)).await;
            let b = provider_ledger.balances().unwrap();
            if b.first().is_some_and(|p| p.receipts == 1) {
                provider_view = Some(b);
                break;
            }
        }
        let p = &provider_view.expect("provider should have recorded the receipt")[0];
        let c = &consumer_ledger.balances().unwrap()[0];

        assert_eq!(p.peer_did, consumer_did);
        assert_eq!(c.peer_did, provider_did);
        assert_eq!(p.unsigned_claims, 0, "the receipt must retire the claim");
        assert_eq!(
            p.earned, c.spent,
            "both sides must independently agree on the amount"
        );
        assert_eq!(
            p.net(),
            -c.net(),
            "and therefore on the net, with opposite sign"
        );
        assert!(
            p.earned > 0,
            "real tokens were served, so the amount must be non-zero"
        );

        // Both parties must hold the same content-addressed artifact.
        assert!(
            consumer_ledger.has_receipt(&receipt_id).unwrap()
                && provider_ledger.has_receipt(&receipt_id).unwrap(),
            "the receipt_id must match on both sides"
        );

        println!(
            "receipt {} — {prompt_tokens} prompt + {completion_tokens} completion tokens \
             — {} micro-credits, agreed by both sides",
            hex::encode(receipt_id),
            p.earned,
        );
    }

    /// Live interop against a real remote peer over the real relay, using this
    /// node's real p2pd and real `~/.kwaainet/ledger.db`.
    ///
    /// Doubles as both compatibility checks, decided by what the remote runs:
    ///
    /// * remote on v0.5.4/v0.5.5 (no ledger) → 200, `claim: None`, no receipt,
    ///   no error. This is the graceful-degradation bar.
    /// * remote on this branch → 200, a claim, and a receipt persisted on both
    ///   sides.
    ///
    /// Run with a peer id and the node running:
    ///   KWAAI_TEST_PEER=12D3Koo… cargo test -p kwaainet --profile ci -- \
    ///     --ignored --nocapture live_mux_request
    #[tokio::test]
    #[ignore = "requires a running node and KWAAI_TEST_PEER"]
    async fn live_mux_request_against_a_remote_peer() {
        let peer_b58 =
            std::env::var("KWAAI_TEST_PEER").expect("set KWAAI_TEST_PEER to the remote peer id");
        let model = std::env::var("KWAAI_TEST_MODEL").unwrap_or_else(|_| "llama3.2:3b".to_string());
        let peer: PeerId = peer_b58.parse().expect("valid peer id");

        // `CircuitBreaker::new()` already hands back an Arc.
        let client = InferenceMuxClient::connect(peer, CircuitBreaker::new())
            .await
            .expect("open a mux stream to the remote peer");

        let ledger = LedgerNode::shared();
        println!(
            "our did: {}",
            ledger.as_ref().map(|l| l.did()).unwrap_or("<no ledger>")
        );
        // Count receipts, not peers: a peer we have already transacted with is
        // already in the list, so a peer-count comparison silently reports "no
        // ledger" on the second run against the same provider.
        let receipts_before: u64 = ledger
            .as_ref()
            .map(|l| l.balances().unwrap().iter().map(|b| b.receipts).sum())
            .unwrap_or(0);

        // KWAAI_TEST_STREAM=1 exercises the path that used to bill nothing:
        // Ollama replies with NDJSON and only the final line carries the counts.
        let stream = std::env::var("KWAAI_TEST_STREAM").is_ok();
        println!("stream: {stream}");
        let body = serde_json::json!({
            "model": model,
            "messages": [{"role": "user", "content": "Reply with the single word: ok"}],
            "stream": stream,
            "options": {"num_ctx": 2048, "num_predict": 8},
        });
        let resp = client
            .send("POST", "/api/chat", serde_json::to_vec(&body).unwrap())
            .await
            .expect("the remote peer must answer");

        println!("status: {}", resp.status);
        println!(
            "body:   {}",
            String::from_utf8_lossy(&resp.body)
                .chars()
                .take(300)
                .collect::<String>()
        );
        assert_eq!(
            resp.status,
            200,
            "remote said: {}",
            String::from_utf8_lossy(&resp.body)
        );

        // `send()` takes the claim out once it has settled it, so what we assert
        // on is whether a receipt landed — not the transient field.
        match ledger.as_ref() {
            None => println!("no local ledger — nothing to settle"),
            Some(l) => {
                let after = l.balances().unwrap();
                println!("peer balances now: {}", after.len());
                for b in &after {
                    println!(
                        "  {} earned={} spent={} receipts={} unsigned={}",
                        b.peer_did, b.earned, b.spent, b.receipts, b.unsigned_claims
                    );
                }
                let receipts_after: u64 = after.iter().map(|b| b.receipts).sum();
                if receipts_after > receipts_before {
                    println!("→ remote HAS the ledger: a receipt was co-signed");
                } else {
                    println!(
                        "→ remote has NO ledger (expected on v0.5.4/v0.5.5): \
                         served correctly with no receipt and no error"
                    );
                }
            }
        }
    }

    #[test]
    fn a_v055_peer_can_still_decode_our_response_carrying_a_claim() {
        // The claim field is populated by the provider; an old consumer must
        // read the status and body and silently ignore the rest.
        let frame = MuxFrame::Response(MuxResponse {
            request_id: 7,
            status: 200,
            body: b"{}".to_vec(),
            claim: None,
        });
        let legacy: LegacyMuxFrame = rmp_serde::from_slice(&enc(&frame)).expect("legacy decode");
        match legacy {
            LegacyMuxFrame::Response(r) => {
                assert_eq!(r.request_id, 7);
                assert_eq!(r.status, 200);
                assert_eq!(r.body, b"{}");
            }
            other => panic!("expected Response, got {other:?}"),
        }
    }

    #[test]
    fn a_v055_peer_can_still_decode_our_lease_response_carrying_a_quote() {
        let frame = MuxFrame::LeaseResponse {
            request_id: 3,
            outcome: NegotiationOutcome::DeniedAtCapacity {
                retry_after_secs: 5,
            },
            quote: None,
        };
        let legacy: LegacyMuxFrame = rmp_serde::from_slice(&enc(&frame)).expect("legacy decode");
        match legacy {
            LegacyMuxFrame::LeaseResponse {
                request_id,
                outcome,
            } => {
                assert_eq!(request_id, 3);
                assert!(matches!(
                    outcome,
                    NegotiationOutcome::DeniedAtCapacity {
                        retry_after_secs: 5
                    }
                ));
            }
            other => panic!("expected LeaseResponse, got {other:?}"),
        }
    }

    #[test]
    fn we_can_decode_a_v055_response_that_has_no_claim_field_at_all() {
        // The other direction: our client talking to an old provider. The
        // `#[serde(default)]` is what makes this work, and losing it would
        // break every legacy peer at once.
        #[derive(Serialize)]
        enum OldFrame {
            Response {
                request_id: u64,
                status: u16,
                body: Vec<u8>,
            },
        }
        let bytes = rmp_serde::to_vec_named(&OldFrame::Response {
            request_id: 1,
            status: 200,
            body: b"hi".to_vec(),
        })
        .unwrap();

        match rmp_serde::from_slice::<MuxFrame>(&bytes).expect("decode old response") {
            MuxFrame::Response(r) => {
                assert_eq!(r.body, b"hi");
                assert!(
                    r.claim.is_none(),
                    "no claim means no accounting, not an error"
                );
            }
            other => panic!("expected Response, got {other:?}"),
        }
    }

    #[test]
    fn we_can_decode_a_v055_lease_response_that_has_no_quote_field() {
        #[derive(Serialize)]
        enum OldFrame {
            LeaseResponse {
                request_id: u64,
                outcome: NegotiationOutcome,
            },
        }
        let bytes = rmp_serde::to_vec_named(&OldFrame::LeaseResponse {
            request_id: 9,
            outcome: NegotiationOutcome::Granted(crate::capacity_lease::LeaseGrant {
                lease_id: 4,
                ttl_secs: 30,
            }),
        })
        .unwrap();

        match rmp_serde::from_slice::<MuxFrame>(&bytes).expect("decode old lease response") {
            MuxFrame::LeaseResponse {
                request_id,
                outcome,
                quote,
            } => {
                assert_eq!(request_id, 9);
                assert!(matches!(outcome, NegotiationOutcome::Granted(_)));
                assert!(
                    quote.is_none(),
                    "an unquoted grant must still be a usable grant — capacity \
                     does not depend on accounting"
                );
            }
            other => panic!("expected LeaseResponse, got {other:?}"),
        }
    }

    #[test]
    fn a_v055_peer_rejects_a_receipt_ack_frame_without_tearing_down_the_stream() {
        // ReceiptAck is a brand-new *variant*, not a new field, so an old peer
        // genuinely cannot decode it. The contract is that this surfaces as a
        // decode error — which both frame loops answer with `warn!` + `continue`
        // — rather than as a malformed frame that desynchronises the stream.
        // Framing is length-prefixed, so one undecodable frame is skipped whole.
        let (provider, consumer) = (
            ed25519_dalek::SigningKey::from_bytes(&[1u8; 32]),
            ed25519_dalek::SigningKey::from_bytes(&[2u8; 32]),
        );
        let did = |k: &ed25519_dalek::SigningKey| {
            let pk =
                libp2p::identity::ed25519::PublicKey::try_from_bytes(k.verifying_key().as_bytes())
                    .unwrap();
            did_for_peer(&PeerId::from_public_key(
                &libp2p::identity::PublicKey::from(pk),
            ))
        };
        let body = br#"{"prompt_eval_count":1,"eval_count":1}"#;
        let quote = LeaseQuote {
            version: kwaai_ledger::PAYLOAD_VERSION,
            lease_id: 1,
            provider_did: did(&provider),
            consumer_did: did(&consumer),
            model: "m".into(),
            price_micro_per_1k_tokens: 1000,
            ttl_secs: 30,
            granted_at_unix_ms: 0,
            nonce: 1,
            key_epoch: 1,
        };
        let credits = quote.credits_for_tokens(2).unwrap();
        let receipt = kwaai_ledger::WorkClaimPayload {
            version: kwaai_ledger::PAYLOAD_VERSION,
            lease_id: 1,
            request_id: 1,
            provider_did: quote.provider_did.clone(),
            consumer_did: quote.consumer_did.clone(),
            prompt_tokens: 1,
            completion_tokens: 1,
            response_digest: kwaai_ledger::response_digest(body),
            credits_owed: credits,
            valid_until_unix_ms: u64::MAX,
            nonce: 2,
            key_epoch: 1,
        }
        .sign(&provider)
        .unwrap()
        .counter_sign(&consumer, 1)
        .unwrap();

        let bytes = enc(&MuxFrame::ReceiptAck { receipt });
        assert!(
            rmp_serde::from_slice::<LegacyMuxFrame>(&bytes).is_err(),
            "an old peer should reject the variant outright"
        );
        // And we must round-trip it ourselves.
        assert!(matches!(
            rmp_serde::from_slice::<MuxFrame>(&bytes).expect("our own decode"),
            MuxFrame::ReceiptAck { .. }
        ));
    }
}
