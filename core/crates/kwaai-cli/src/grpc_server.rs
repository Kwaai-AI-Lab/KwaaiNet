//! In-process gRPC server for the long-running `kwaainet start` daemon.
//!
//! Hosts the [`kwaai_rpc::v1::KwaaiNet`] service so the Flutter GUI (and any
//! future native CLI subcommands) can drive the daemon over a structured RPC
//! channel instead of scraping stdout.
//!
//! ## Transports
//!
//! On POSIX we bind both:
//!   - a Unix socket at `~/.kwaainet/run/kwaai.sock` (preferred by the GUI,
//!     no port collisions, filesystem permissions act as the ACL)
//!   - TCP loopback at `127.0.0.1:8093` (so a future Windows client or a
//!     remote `kwaainet chat` subcommand can still dial in)
//!
//! On non-POSIX platforms only TCP is bound.
//!
//! ## Inference path
//!
//! The Chat handler lazily constructs a `kwaai_inference::InferenceEngine`
//! and loads the configured model on the first request. Subsequent requests
//! reuse the cached engine (held inside an `Arc<Mutex<…>>` so the
//! single-threaded llama / candle context isn't contended).
//!
//! When the `llama-cpp` feature is compiled in we drive
//! [`crate::llama_local::run_inference_streaming`] for true per-token
//! streaming over the gRPC response stream. When it isn't, we fall back to
//! the sync `InferenceEngine::generate()` and emit the whole reply as a
//! single chunk (still followed by a done=true terminator so the client
//! framing is unchanged). See the TODO inside `spawn_inference` for the
//! plan to lift this restriction without forking the inference path.

use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tracing::{error, info, warn};

use kwaai_rpc::v1::{
    client_frame,
    error::Code as ErrorCode,
    kwaai_net_server::{KwaaiNet, KwaaiNetServer},
    server_frame, BlockCoverageRequest, BlockCoverageUpdate, BlockPeer, Cancel, ChatMessage,
    ChatToken, ClientFrame, Done, Error as RpcError, GenerateRequest, PingReply, PingRequest,
    ServerFrame, ShardRunRequest, StatusReply, StorageDiscoveryRequest, StoragePeer,
    StorageReachability, StorageUpdate,
};
#[cfg(feature = "rag")]
use kwaai_rpc::v1::{
    rag_progress::RagPhase, RagChunk, RagDeleteReply, RagDeleteRequest, RagDoc, RagIngestReply,
    RagIngestRequest, RagInitReply, RagInitRequest, RagKb, RagProgress, RagQueryReply,
    RagQueryRequest, RagStatusUpdate,
};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use crate::config::KwaaiNetConfig;

/// Default TCP loopback port. Picked from the IANA dynamic range; not
/// currently configurable but trivial to move into `KwaaiNetConfig` later.
pub const DEFAULT_GRPC_TCP_PORT: u16 = 8093;

/// Relative path (under `~/.kwaainet/run/`) where we bind the Unix socket.
#[cfg(unix)]
const UNIX_SOCKET_RELPATH: &str = "run/kwaai.sock";

// ---------------------------------------------------------------------------
// Service implementation
// ---------------------------------------------------------------------------

/// Shared inference state lazily initialised on the first Chat request.
///
/// Wrapping in `Arc<Mutex<…>>` keeps things simple: the underlying
/// `InferenceEngine` / `ModelHandle` aren't `Sync`-friendly to share by
/// reference, and serialising generation across clients matches what the
/// existing axum-based OpenAI endpoint does (see [`crate::api`]).
struct InferenceState {
    engine: kwaai_inference::InferenceEngine,
    handle: kwaai_inference::ModelHandle,
    model_id: String,
    /// GGUF blob path resolved from the Ollama model ref, kept around so
    /// the `llama-cpp` streaming path can reload the model into the
    /// llama.cpp backend (it owns its own model state separate from the
    /// candle-based InferenceEngine).
    #[cfg_attr(not(feature = "llama-cpp"), allow(dead_code))]
    gguf_path: Option<PathBuf>,
}

pub struct KwaaiNetService {
    config: Arc<KwaaiNetConfig>,
    inference: Arc<Mutex<Option<Arc<Mutex<InferenceState>>>>>,
    /// Captured at service construction so StatusReply.uptime_secs can
    /// report a process-level uptime without a separate clock.
    started_at: Instant,
    /// Serialises RAG data operations across the whole service.
    ///
    /// `MetaStore` / `StorageDb` are embedded, file-backed stores. The
    /// CLI only ever opens them one command at a time, so nothing in
    /// that layer is built to tolerate two writers in one process —
    /// concurrent opens can hit file locking. A single service-wide
    /// mutex is the conservative phase-1 answer: it costs us
    /// cross-KB parallelism (a per-KB mutex map would recover that)
    /// but it cannot deadlock and it matches the access pattern the
    /// storage layer was written against. Revisit if RAG ops on
    /// distinct KBs ever need to overlap.
    #[cfg(feature = "rag")]
    rag_lock: Arc<Mutex<()>>,
    /// Opened local vector stores, cached per data dir for the daemon's
    /// lifetime.
    ///
    /// `StorageDb::open` replays every stored vector into an in-memory
    /// HNSW graph — O(corpus) work that would otherwise run on every
    /// RAG op. The daemon is long-lived, so open once and reuse, exactly
    /// as `rag serve` does. All users hold `rag_lock`, so the map itself
    /// sees no contention. Inherited trade-off from `rag serve`: vectors
    /// written by another process (e.g. a CLI ingest) aren't visible
    /// until the daemon restarts.
    #[cfg(feature = "rag")]
    rag_stores: Arc<Mutex<HashMap<PathBuf, Arc<kwaai_storage::VectorStore>>>>,
}

impl KwaaiNetService {
    pub fn new(config: KwaaiNetConfig) -> Self {
        Self {
            config: Arc::new(config),
            inference: Arc::new(Mutex::new(None)),
            started_at: Instant::now(),
            #[cfg(feature = "rag")]
            rag_lock: Arc::new(Mutex::new(())),
            #[cfg(feature = "rag")]
            rag_stores: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get (or lazily initialise) the shared inference state.
    async fn get_or_init_inference(&self) -> Result<Arc<Mutex<InferenceState>>> {
        let mut guard = self.inference.lock().await;
        if let Some(existing) = guard.as_ref() {
            return Ok(existing.clone());
        }

        let cfg = self.config.clone();
        // `InferenceEngine::new` and `load_model` are sync + CPU-heavy, so do
        // the work on the blocking pool and not on the gRPC reactor thread.
        let state = tokio::task::spawn_blocking(move || build_inference_state(&cfg))
            .await
            .context("inference init task panicked")??;

        let arc = Arc::new(Mutex::new(state));
        *guard = Some(arc.clone());
        Ok(arc)
    }
}

fn build_inference_state(cfg: &KwaaiNetConfig) -> Result<InferenceState> {
    // `InferenceProvider` brings `load_model` into scope.
    use kwaai_inference::{EngineConfig, InferenceEngine, InferenceProvider as _, ModelFormat};
    use sysinfo::System;

    let system_ram = {
        let mut sys = System::new();
        sys.refresh_memory();
        sys.total_memory()
    };
    let engine_config = EngineConfig {
        max_memory: ((system_ram as f64 * 0.85) as usize).max(4 * 1024 * 1024 * 1024),
        ..EngineConfig::default()
    };
    let mut engine = InferenceEngine::new(engine_config).context("InferenceEngine::new")?;

    let model_id = cfg.model.clone();
    let is_hf = model_id.contains('/') && !model_id.starts_with("hf.co/");

    let (handle, gguf_path) = if is_hf {
        let snapshot = crate::hf::resolve_snapshot(&model_id)
            .with_context(|| format!("resolving HF snapshot for {model_id}"))?;
        let handle = engine
            .load_model(&snapshot, ModelFormat::SafeTensors)
            .with_context(|| format!("loading SafeTensors snapshot at {}", snapshot.display()))?;
        (handle, None)
    } else {
        let blob = crate::ollama::resolve_model_blob(&model_id)
            .with_context(|| format!("resolving Ollama blob for {model_id}"))?;
        let handle = engine
            .load_model(&blob, ModelFormat::Gguf)
            .with_context(|| format!("loading GGUF blob at {}", blob.display()))?;
        (handle, Some(blob))
    };

    Ok(InferenceState {
        engine,
        handle,
        model_id,
        gguf_path,
    })
}

#[tonic::async_trait]
impl KwaaiNet for KwaaiNetService {
    type ChatStream = tokio_stream::wrappers::ReceiverStream<Result<ChatToken, Status>>;
    type SessionStream = tokio_stream::wrappers::ReceiverStream<Result<ServerFrame, Status>>;

    async fn session(
        &self,
        request: Request<Streaming<ClientFrame>>,
    ) -> Result<Response<Self::SessionStream>, Status> {
        let mut inbound = request.into_inner();

        // ServerFrame fan-in. Every per-operation task emits into this
        // channel; ordering between operations is the natural emit order.
        let (out_tx, out_rx) = mpsc::channel::<Result<ServerFrame, Status>>(128);

        // Per-id cancellation registry. ClientFrame::Cancel { target_id }
        // looks up the oneshot here and fires it.
        let cancels: Arc<Mutex<HashMap<u64, oneshot::Sender<()>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        // Clone the bits each per-frame task captures.
        let cfg = self.config.clone();
        let inference_slot = self.inference.clone();
        let started_at = self.started_at;
        #[cfg(feature = "rag")]
        let rag_lock = self.rag_lock.clone();
        #[cfg(feature = "rag")]
        let rag_stores = self.rag_stores.clone();

        tokio::spawn(async move {
            loop {
                let frame = match inbound.message().await {
                    Ok(Some(f)) => f,
                    Ok(None) => {
                        info!("Session: client closed inbound stream");
                        break;
                    }
                    Err(e) => {
                        warn!("Session: inbound recv error: {e}");
                        break;
                    }
                };

                let id = frame.id;
                let body = match frame.body {
                    Some(b) => b,
                    None => {
                        let _ = out_tx
                            .send(Ok(error_frame(
                                id,
                                ErrorCode::InvalidArgument,
                                "ClientFrame missing body",
                            )))
                            .await;
                        continue;
                    }
                };

                match body {
                    client_frame::Body::Ping(_) => {
                        let _ = out_tx
                            .send(Ok(ServerFrame {
                                id,
                                body: Some(server_frame::Body::Pong(PingReply {
                                    server_time: now_rfc3339(),
                                })),
                            }))
                            .await;
                        let _ = out_tx.send(Ok(done_frame(id))).await;
                    }

                    client_frame::Body::Status(_) => {
                        let reply = StatusReply {
                            server_time: now_rfc3339(),
                            model: cfg.model.clone(),
                            shard_ready: shard_ready_path_exists(),
                            peer_count: 0, // TODO: thread through DHT routing-table size
                            uptime_secs: started_at.elapsed().as_secs(),
                            // Same constant the updater compares against, so
                            // the version reported over the wire can never
                            // drift from the one used for update checks.
                            version: crate::updater::CURRENT_VERSION.to_string(),
                        };
                        let _ = out_tx
                            .send(Ok(ServerFrame {
                                id,
                                body: Some(server_frame::Body::Status(reply)),
                            }))
                            .await;
                        let _ = out_tx.send(Ok(done_frame(id))).await;
                    }

                    client_frame::Body::Generate(req) => {
                        spawn_session_generate(
                            id,
                            req,
                            cfg.clone(),
                            inference_slot.clone(),
                            out_tx.clone(),
                            cancels.clone(),
                        )
                        .await;
                    }

                    client_frame::Body::ShardRun(req) => {
                        spawn_session_shard_run(id, req, out_tx.clone(), cancels.clone()).await;
                    }

                    client_frame::Body::BlockCoverage(req) => {
                        spawn_session_block_coverage(
                            id,
                            req,
                            cfg.clone(),
                            out_tx.clone(),
                            cancels.clone(),
                        )
                        .await;
                    }

                    client_frame::Body::StorageDiscovery(req) => {
                        spawn_session_storage_discovery(
                            id,
                            req,
                            cfg.clone(),
                            out_tx.clone(),
                            cancels.clone(),
                        )
                        .await;
                    }

                    client_frame::Body::Cancel(Cancel { target_id }) => {
                        let removed = cancels.lock().await.remove(&target_id);
                        if let Some(tx) = removed {
                            let _ = tx.send(());
                            // Acknowledge the cancel frame itself with Done.
                            let _ = out_tx.send(Ok(done_frame(id))).await;
                        } else {
                            let _ = out_tx
                                .send(Ok(error_frame(
                                    id,
                                    ErrorCode::NotFound,
                                    &format!("no in-flight operation with id {target_id}"),
                                )))
                                .await;
                        }
                    }

                    // --- RAG ops ---
                    //
                    // Each spawns its own task so a slow ingest can't
                    // stall the dispatch loop, and each takes `rag_lock`
                    // inside that task (see the field comment).
                    #[cfg(feature = "rag")]
                    client_frame::Body::RagStatus(_) => {
                        spawn_session_rag_status(id, out_tx.clone(), rag_lock.clone()).await;
                    }

                    #[cfg(feature = "rag")]
                    client_frame::Body::RagInit(req) => {
                        spawn_session_rag_init(id, req, out_tx.clone(), rag_lock.clone()).await;
                    }

                    #[cfg(feature = "rag")]
                    client_frame::Body::RagIngest(req) => {
                        spawn_session_rag_ingest(
                            id,
                            req,
                            out_tx.clone(),
                            cancels.clone(),
                            rag_lock.clone(),
                            rag_stores.clone(),
                        )
                        .await;
                    }

                    #[cfg(feature = "rag")]
                    client_frame::Body::RagQuery(req) => {
                        spawn_session_rag_query(
                            id,
                            req,
                            out_tx.clone(),
                            rag_lock.clone(),
                            rag_stores.clone(),
                        )
                        .await;
                    }

                    #[cfg(feature = "rag")]
                    client_frame::Body::RagDelete(req) => {
                        spawn_session_rag_delete(
                            id,
                            req,
                            out_tx.clone(),
                            rag_lock.clone(),
                            rag_stores.clone(),
                        )
                        .await;
                    }

                    // Built without the `rag` feature: the frames still
                    // decode (the proto is one wire format for every
                    // build), so answer UNIMPLEMENTED rather than
                    // silently dropping them.
                    #[cfg(not(feature = "rag"))]
                    client_frame::Body::RagStatus(_)
                    | client_frame::Body::RagInit(_)
                    | client_frame::Body::RagIngest(_)
                    | client_frame::Body::RagQuery(_)
                    | client_frame::Body::RagDelete(_) => {
                        let _ = out_tx
                            .send(Ok(error_frame(
                                id,
                                ErrorCode::Unimplemented,
                                "this daemon was built without the 'rag' feature",
                            )))
                            .await;
                    }
                }
            }
        });

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(
            out_rx,
        )))
    }

    async fn chat(
        &self,
        request: Request<Streaming<ChatMessage>>,
    ) -> Result<Response<Self::ChatStream>, Status> {
        let mut in_stream = request.into_inner();

        // For now treat the first ChatMessage as the prompt and drop the
        // rest. Multi-turn (accumulate role=user/assistant pairs into the
        // chat-template) is the obvious follow-up — see TODO at top of file.
        let first = in_stream
            .message()
            .await
            .map_err(|e| Status::internal(format!("recv first chat msg: {e}")))?
            .ok_or_else(|| {
                Status::invalid_argument("client closed Chat stream before sending a prompt")
            })?;

        let prompt = build_prompt(&first);

        // Spawn a task that drains the rest of the inbound stream so the
        // client doesn't see backpressure on its writer half. We log but
        // otherwise ignore any subsequent messages for now.
        // TODO(multi-turn): feed these into the chat template so multi-turn
        // works without reopening the stream.
        tokio::spawn(async move {
            while let Ok(Some(msg)) = in_stream.message().await {
                tracing::debug!(
                    role = %msg.role,
                    bytes = msg.content.len(),
                    "Chat: ignoring additional inbound message (single-turn only)"
                );
            }
        });

        let inference = self.get_or_init_inference().await.map_err(|e| {
            error!("Chat: inference init failed: {e:#}");
            Status::internal(format!("inference init failed: {e}"))
        })?;

        // Channel that carries generated tokens from the worker (which runs
        // on the blocking pool because the inference engine is sync) back to
        // the gRPC response stream.
        let (tx, rx) = mpsc::channel::<Result<ChatToken, Status>>(64);

        spawn_inference(inference, prompt, tx);

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(
            rx,
        )))
    }

    /// Liveness probe. Returns the current daemon wall-clock time.
    /// Deliberately trivial — no inference, no DHT, no locks taken.
    async fn ping(&self, _request: Request<PingRequest>) -> Result<Response<PingReply>, Status> {
        let now = std::time::SystemTime::now();
        // Format as RFC 3339 via chrono for a stable, parse-friendly
        // representation. Falls back to the unix timestamp if the
        // SystemTime is somehow pre-epoch.
        let server_time = chrono::DateTime::<chrono::Utc>::from(now)
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        Ok(Response::new(PingReply { server_time }))
    }
}

// ---------------------------------------------------------------------------
// Session helpers
// ---------------------------------------------------------------------------

fn now_rfc3339() -> String {
    chrono::DateTime::<chrono::Utc>::from(std::time::SystemTime::now())
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

fn done_frame(id: u64) -> ServerFrame {
    ServerFrame {
        id,
        body: Some(server_frame::Body::Done(Done {})),
    }
}

fn error_frame(id: u64, code: ErrorCode, msg: &str) -> ServerFrame {
    ServerFrame {
        id,
        body: Some(server_frame::Body::Error(RpcError {
            code: code as i32,
            message: msg.to_string(),
        })),
    }
}

/// Classify a `shard_run` failure into a specific [`ErrorCode`]. The
/// dispatcher in [`crate::shard_cmd`] returns anyhow errors with
/// descriptive `bail!` messages; pattern-match the strings here so
/// the wire surface uses precise codes (clients shouldn't have to
/// grep the human message). Falls back to `Internal` for genuinely
/// unknown failures.
fn classify_shard_error(msg: &str) -> ErrorCode {
    // 30s-wait timeout exhausted without any peer announcing the model.
    if msg.contains("no peers serving model") {
        return ErrorCode::NoPeersForModel;
    }
    // Chain build couldn't find a server covering some block.
    if msg.contains("No server covers block") {
        return ErrorCode::InsufficientCoverage;
    }
    // Chain built, all candidates for a given position failed during
    // inference forwards (most commonly: peer's inference handler
    // unregistered, so /kwaai/inference/1.0.0 protocol negotiation
    // fails for every candidate).
    if msg.contains("candidate(s) for block") {
        return ErrorCode::AllCandidatesFailed;
    }
    ErrorCode::Internal
}

// ---------------------------------------------------------------------------
// RAG session ops
// ---------------------------------------------------------------------------
//
// Config freshness: the service holds an `Arc<KwaaiNetConfig>` captured at
// startup, but `rag_init` *writes* config (it persists a new KB and calls
// `cfg.save()`). If these handlers read the startup snapshot, an init would
// be invisible to every later op in the same daemon process. So each handler
// loads config from disk itself, exactly as the CLI does — the snapshot is
// never consulted on the RAG path.
//
// Blocking: MetaStore / StorageDb opens, text extraction and the BM25 build
// inside retrieve_hybrid are all sync and CPU/IO-heavy. Anything sync runs
// under `spawn_blocking` so it stays off the gRPC reactor.

/// Default `top_k` when the client sends 0.
#[cfg(feature = "rag")]
const RAG_DEFAULT_TOP_K: u32 = 20;

/// Minimum gap between `RagProgress` frames during ingest.
///
/// `ingest_text` invokes its callback once per embedded chunk, which for a
/// large document is far more often than any UI can use. Throttling to
/// ~4 frames/sec keeps the stream useful without flooding it; the terminal
/// count in `RagIngestReply` is authoritative either way, and the final
/// pre-reply frame is always sent regardless of the throttle so a client
/// never ends on a stale partial count.
#[cfg(feature = "rag")]
const RAG_PROGRESS_INTERVAL: std::time::Duration = std::time::Duration::from_millis(250);

/// Ceiling on the text-extraction step of an ingest.
///
/// pdf-extract can take effectively unbounded time on pathological PDFs
/// (huge object counts, malformed xref tables), and a blocking task cannot
/// be killed — without a ceiling the op would hold the RAG lock forever and
/// the client would never see another frame. On timeout the op fails with a
/// clear message, but note the abandoned extraction thread runs on until
/// the parse finishes; only a daemon restart reclaims it sooner.
#[cfg(feature = "rag")]
const RAG_EXTRACT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(120);

/// The service's per-data-dir cache of opened local vector stores.
/// See the `rag_stores` field comment for semantics.
#[cfg(feature = "rag")]
type RagStores = Arc<Mutex<HashMap<PathBuf, Arc<kwaai_storage::VectorStore>>>>;

/// Fetch (or open and cache) the local vector store for `data_dir`.
///
/// The open replays every stored vector into the in-memory HNSW graph,
/// so it runs off the reactor and only on first use. Callers hold
/// `rag_lock`, which is what makes the get-then-insert sequence safe.
#[cfg(feature = "rag")]
async fn cached_local_vs(
    stores: &RagStores,
    data_dir: &std::path::Path,
) -> Result<Arc<kwaai_storage::VectorStore>> {
    if let Some(vs) = stores.lock().await.get(data_dir) {
        return Ok(vs.clone());
    }
    let dir = data_dir.to_path_buf();
    let vs = tokio::task::spawn_blocking(move || crate::rag_cmd::open_local_vs(&dir))
        .await
        .context("vector store task panicked")??;
    let vs = Arc::new(vs);
    stores
        .lock()
        .await
        .insert(data_dir.to_path_buf(), vs.clone());
    Ok(vs)
}

/// Map an anyhow error off the RAG path onto a wire [`ErrorCode`].
///
/// The RAG layer signals failure classes through message text (the CLI
/// renders these strings directly), so match on them here rather than
/// making every caller grep. `load_rag_config_for` produces the
/// "not initialised" message, and `init_kb` remaps Ollama transport
/// failures into the two operator-actionable forms.
#[cfg(feature = "rag")]
fn classify_rag_error(msg: &str) -> ErrorCode {
    // Something the client named doesn't exist: a KB that was never
    // initialised (the actionable "Run: kwaainet rag init …" message comes
    // straight from load_rag_config_for), or a document the KB doesn't
    // hold. Both are NOT_FOUND — the client already knows which it asked
    // for, so the code need only say "the thing you named isn't there".
    if msg.contains("not initialised") || msg.contains("is not in KB") {
        return ErrorCode::NotFound;
    }
    // Caller-supplied arguments we rejected before touching any store.
    // A nonexistent ingest path is INVALID_ARGUMENT rather than NOT_FOUND:
    // that path is the client's own filesystem claim, not a name it looked
    // up in a reply we sent it.
    if msg.contains("no such file on the daemon host")
        || msg.contains("requires a path")
        || msg.contains("requires non-empty query text")
        || msg.contains("requires a document name")
    {
        return ErrorCode::InvalidArgument;
    }
    // Phase-1 transport limit — the KB exists but we can't serve it here.
    if msg.contains("not yet supported over gRPC") {
        return ErrorCode::Unimplemented;
    }
    // Both branches of init_kb's probe_dim remap, plus the generic
    // reqwest connect failure if it ever escapes unmapped.
    if msg.contains("Cannot reach Ollama")
        || msg.contains("is not available in Ollama")
        || msg.contains("Connection refused")
        || msg.contains("error sending request")
    {
        return ErrorCode::Unavailable;
    }
    ErrorCode::Internal
}

/// Emit an Error frame classified from an anyhow error, then return.
///
/// Uses the `{:#}` alternate form so anyhow's context chain (which is
/// where the actionable text usually lives) reaches the client.
#[cfg(feature = "rag")]
async fn send_rag_error(
    id: u64,
    err: &anyhow::Error,
    out_tx: &mpsc::Sender<Result<ServerFrame, Status>>,
) {
    let msg = format!("{err:#}");
    let _ = out_tx
        .send(Ok(error_frame(id, classify_rag_error(&msg), &msg)))
        .await;
}

/// Blank KB name means "default", matching the CLI's `--kb` default.
#[cfg(feature = "rag")]
fn rag_kb_or_default(kb: &str) -> String {
    if kb.trim().is_empty() {
        "default".to_string()
    } else {
        kb.trim().to_string()
    }
}

/// True when this KB's vectors do not live in the local embedded store.
#[cfg(feature = "rag")]
fn rag_is_remote(rag: &crate::config::RagConfig) -> bool {
    rag.storage_url.as_deref() != Some("local")
}

/// Phase 1 serves only local-transport KBs. Remote-Eve / HTTP-storage KBs
/// need P2PClient plumbing that deliberately stays out of the gRPC service
/// for now, so reject them with a message that says so rather than
/// half-working.
#[cfg(feature = "rag")]
fn reject_if_remote(kb: &str, rag: &crate::config::RagConfig) -> Result<()> {
    if rag_is_remote(rag) {
        anyhow::bail!(
            "KB '{kb}' is backed by remote storage — remote Eve KBs are not yet supported over gRPC. Use the CLI: kwaainet rag query --kb {kb}"
        );
    }
    Ok(())
}

/// `rag_status` — snapshot every configured KB.
///
/// Best-effort per KB: one KB whose MetaStore won't open (wiped data dir,
/// schema from an older build) is reported with an empty doc list instead
/// of failing the status of every other KB.
#[cfg(feature = "rag")]
async fn spawn_session_rag_status(
    id: u64,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    rag_lock: Arc<Mutex<()>>,
) {
    tokio::spawn(async move {
        let _guard = rag_lock.lock().await;

        let kbs = tokio::task::spawn_blocking(collect_rag_kbs).await;

        match kbs {
            Ok(Ok(kbs)) => {
                let _ = out_tx
                    .send(Ok(ServerFrame {
                        id,
                        body: Some(server_frame::Body::RagStatus(RagStatusUpdate { kbs })),
                    }))
                    .await;
                let _ = out_tx.send(Ok(done_frame(id))).await;
            }
            Ok(Err(e)) => send_rag_error(id, &e, &out_tx).await,
            Err(e) => {
                let _ = out_tx
                    .send(Ok(error_frame(
                        id,
                        ErrorCode::Internal,
                        &format!("rag status task panicked: {e}"),
                    )))
                    .await;
            }
        }
    });
}

/// Sync half of `rag_status`. Runs on the blocking pool.
#[cfg(feature = "rag")]
fn collect_rag_kbs() -> Result<Vec<RagKb>> {
    use kwaai_rag::meta_store::MetaStore;

    // Fresh from disk — see the module comment on config freshness.
    let cfg = KwaaiNetConfig::load_or_create()?;

    let mut out = Vec::new();
    for name in cfg.rag_kb_names() {
        let Some(rag) = cfg.get_rag_kb(&name) else {
            continue;
        };

        // A KB with no/invalid tenant id, or an unopenable store, still
        // gets reported — the client should see it exists.
        let documents: Vec<RagDoc> = rag
            .tenant_id
            .as_deref()
            .and_then(|t| t.parse::<uuid::Uuid>().ok())
            .and_then(|tid| MetaStore::open(&rag.data_dir(), tid).ok())
            .and_then(|ms| ms.list_docs_with_counts().ok())
            .unwrap_or_default()
            .into_iter()
            .map(|(name, chunks)| RagDoc { name, chunks })
            .collect();

        out.push(RagKb {
            name,
            embed_model: rag.embed_model.clone(),
            // Kept in step with `documents` for clients that predate it.
            docs: documents.iter().map(|d| d.name.clone()).collect(),
            remote: rag_is_remote(rag),
            documents,
        });
    }
    Ok(out)
}

/// `rag_init` — create or idempotently refresh a knowledge base.
#[cfg(feature = "rag")]
async fn spawn_session_rag_init(
    id: u64,
    req: RagInitRequest,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    rag_lock: Arc<Mutex<()>>,
) {
    tokio::spawn(async move {
        let _guard = rag_lock.lock().await;

        let kb = rag_kb_or_default(&req.kb);
        let embed_model = if req.embed_model.trim().is_empty() {
            crate::config::RagConfig::default().embed_model
        } else {
            req.embed_model.trim().to_string()
        };

        // init_kb is async (it awaits the Ollama probe and the tenant
        // create), so it is driven directly rather than via spawn_blocking;
        // the sync store work inside it is short.
        match crate::rag_cmd::init_kb(&kb, embed_model, None, |_| {}).await {
            Ok(_) => {
                // Re-read so the reply reflects persisted state rather
                // than what we think we just wrote.
                let kb_for_task = kb.clone();
                let built = tokio::task::spawn_blocking(move || single_rag_kb(&kb_for_task)).await;

                match built {
                    Ok(Ok(kb_msg)) => {
                        let _ = out_tx
                            .send(Ok(ServerFrame {
                                id,
                                body: Some(server_frame::Body::RagInit(RagInitReply {
                                    kb: Some(kb_msg),
                                })),
                            }))
                            .await;
                        let _ = out_tx.send(Ok(done_frame(id))).await;
                    }
                    Ok(Err(e)) => send_rag_error(id, &e, &out_tx).await,
                    Err(e) => {
                        let _ = out_tx
                            .send(Ok(error_frame(
                                id,
                                ErrorCode::Internal,
                                &format!("rag init readback task panicked: {e}"),
                            )))
                            .await;
                    }
                }
            }
            Err(e) => send_rag_error(id, &e, &out_tx).await,
        }
    });
}

/// Build one [`RagKb`] from persisted config. Sync; run on the blocking pool.
#[cfg(feature = "rag")]
fn single_rag_kb(kb: &str) -> Result<RagKb> {
    collect_rag_kbs()?
        .into_iter()
        .find(|k| k.name == kb)
        .with_context(|| format!("KB '{kb}' missing from config immediately after init"))
}

/// `rag_ingest` — chunk, embed and store one document from the daemon's disk.
#[cfg(feature = "rag")]
async fn spawn_session_rag_ingest(
    id: u64,
    req: RagIngestRequest,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    cancels: Arc<Mutex<HashMap<u64, oneshot::Sender<()>>>>,
    rag_lock: Arc<Mutex<()>>,
    rag_stores: RagStores,
) {
    // Register the cancel channel BEFORE spawning, so a Cancel frame that
    // arrives immediately after this one can still find it. Registering
    // inside the task would race the dispatch loop.
    let (cancel_tx, cancel_rx) = oneshot::channel::<()>();
    cancels.lock().await.insert(id, cancel_tx);

    tokio::spawn(async move {
        let _guard = rag_lock.lock().await;

        let result = tokio::select! {
            biased;

            _ = cancel_rx => {
                let _ = out_tx
                    .send(Ok(error_frame(id, ErrorCode::Cancelled, "ingest cancelled by client")))
                    .await;
                cancels.lock().await.remove(&id);
                return;
            }

            r = run_rag_ingest(id, req, out_tx.clone(), rag_stores) => r,
        };

        // Whatever happened, this id is no longer cancellable.
        cancels.lock().await.remove(&id);

        match result {
            Ok(reply) => {
                let _ = out_tx
                    .send(Ok(ServerFrame {
                        id,
                        body: Some(server_frame::Body::RagIngest(reply)),
                    }))
                    .await;
                let _ = out_tx.send(Ok(done_frame(id))).await;
            }
            Err(e) => send_rag_error(id, &e, &out_tx).await,
        }
    });
}

/// The ingest pipeline proper. Emits progress frames as it goes and
/// returns the terminal counts; the caller frames the outcome.
#[cfg(feature = "rag")]
async fn run_rag_ingest(
    id: u64,
    req: RagIngestRequest,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    rag_stores: RagStores,
) -> Result<RagIngestReply> {
    use kwaai_rag::{
        document,
        embedder::EmbedClient,
        ingestion::{ingest_text, IngestConfig},
        meta_store::MetaStore,
    };
    use std::pin::Pin;

    let kb = rag_kb_or_default(&req.kb);
    let (rag_cfg, tenant_id) = crate::rag_cmd::load_rag_config_for(&kb)?;
    reject_if_remote(&kb, &rag_cfg)?;

    let path = std::path::PathBuf::from(&req.path);
    if req.path.trim().is_empty() {
        anyhow::bail!("rag_ingest requires a path");
    }
    // Checked up front so a missing file is INVALID_ARGUMENT rather than
    // whatever extract_text happens to bail with. `classify_rag_error`
    // keys off "no such file" to pick the code.
    if !path.is_file() {
        anyhow::bail!("no such file on the daemon host: {}", path.display());
    }

    let doc_name = if req.doc_name.trim().is_empty() {
        path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    } else {
        req.doc_name.trim().to_string()
    };

    // Extraction is sync and can be slow on big PDFs.
    let _ = out_tx
        .send(Ok(rag_progress_frame(
            id,
            RagPhase::Extracting,
            0,
            0,
            &doc_name,
        )))
        .await;

    let extract_path = path.clone();
    let extract = tokio::task::spawn_blocking(move || document::extract_text(&extract_path));
    let text = match tokio::time::timeout(RAG_EXTRACT_TIMEOUT, extract).await {
        Ok(joined) => joined.context("text extraction task panicked")??,
        Err(_) => anyhow::bail!(
            "text extraction timed out after {}s — the file may be too large or too \
             complex for the parser (extraction continues in the background; restart \
             the daemon to reclaim the thread)",
            RAG_EXTRACT_TIMEOUT.as_secs()
        ),
    };

    let data_dir = rag_cfg.data_dir();
    let meta_dir = data_dir.clone();
    let meta = tokio::task::spawn_blocking(move || MetaStore::open(&meta_dir, tenant_id))
        .await
        .context("metadata store task panicked")??;

    let vs = cached_local_vs(&rag_stores, &data_dir).await?;

    // Persistent keyword index, kept in step with this ingest so queries
    // never rebuild it from the full corpus. Opening may backfill a
    // drifted index (sync, CPU-heavy) — run off the reactor. Best-effort:
    // on failure the ingest proceeds unindexed and the next query's
    // open_backfilled repairs the drift.
    let meta = Arc::new(meta);
    let bm25 = {
        let meta = meta.clone();
        let dir = data_dir.clone();
        tokio::task::spawn_blocking(move || {
            kwaai_rag::bm25::BM25Index::open_backfilled(&dir, &meta)
        })
        .await
        .context("BM25 open task panicked")?
        .map_err(|e| warn!(error = format!("{e:#}"), "BM25 index unavailable; ingest will not update it"))
        .ok()
        .map(Arc::new)
    };

    let embed = EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
    let mut cfg = IngestConfig::new(embed);
    cfg.bm25 = bm25;

    // `ingest_text`'s progress callback is sync and called from inside the
    // ingest future, so it can't await. Hand frames to a forwarder task
    // over an unbounded channel and throttle on the receiving side.
    let (prog_tx, mut prog_rx) = mpsc::unbounded_channel::<(usize, usize)>();
    let progress_out = out_tx.clone();
    let progress_doc = doc_name.clone();
    let forwarder = tokio::spawn(async move {
        let mut last_sent = std::time::Instant::now() - RAG_PROGRESS_INTERVAL;
        let mut pending: Option<(usize, usize)> = None;

        while let Some((done, total)) = prog_rx.recv().await {
            pending = Some((done, total));
            if last_sent.elapsed() >= RAG_PROGRESS_INTERVAL {
                let _ = progress_out
                    .send(Ok(rag_progress_frame(
                        id,
                        RagPhase::Embedding,
                        done as u32,
                        total as u32,
                        &progress_doc,
                    )))
                    .await;
                last_sent = std::time::Instant::now();
                pending = None;
            }
        }

        // Always flush the final count, even if the throttle just fired —
        // otherwise the client's last frame could show 97/100 forever.
        if let Some((done, total)) = pending {
            let _ = progress_out
                .send(Ok(rag_progress_frame(
                    id,
                    RagPhase::Embedding,
                    done as u32,
                    total as u32,
                    &progress_doc,
                )))
                .await;
        }
    });

    let upload_vs = vs.clone();
    let result = ingest_text(
        &cfg,
        &meta,
        &doc_name,
        &text,
        move |vectors| {
            let vs = upload_vs.clone();
            Box::pin(async move { vs.upload(tenant_id, &vectors).await })
                as Pin<Box<dyn std::future::Future<Output = Result<usize>> + Send>>
        },
        Some(move |done: usize, total: usize| {
            let _ = prog_tx.send((done, total));
        }),
    )
    .await?;

    // Dropping the sender ends the forwarder's recv loop.
    let _ = forwarder.await;

    Ok(RagIngestReply {
        chunks_ingested: result.chunks_ingested as u32,
        vectors_uploaded: result.vectors_uploaded as u32,
    })
}

#[cfg(feature = "rag")]
fn rag_progress_frame(
    id: u64,
    phase: RagPhase,
    done: u32,
    total: u32,
    detail: &str,
) -> ServerFrame {
    ServerFrame {
        id,
        body: Some(server_frame::Body::RagProgress(RagProgress {
            phase: phase as i32,
            done,
            total,
            detail: detail.to_string(),
        })),
    }
}

/// `rag_query` — hybrid retrieval over a local KB.
#[cfg(feature = "rag")]
async fn spawn_session_rag_query(
    id: u64,
    req: RagQueryRequest,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    rag_lock: Arc<Mutex<()>>,
    rag_stores: RagStores,
) {
    tokio::spawn(async move {
        let _guard = rag_lock.lock().await;

        match run_rag_query(req, rag_stores).await {
            Ok(chunks) => {
                let _ = out_tx
                    .send(Ok(ServerFrame {
                        id,
                        body: Some(server_frame::Body::RagQuery(RagQueryReply { chunks })),
                    }))
                    .await;
                let _ = out_tx.send(Ok(done_frame(id))).await;
            }
            Err(e) => send_rag_error(id, &e, &out_tx).await,
        }
    });
}

#[cfg(feature = "rag")]
async fn run_rag_query(req: RagQueryRequest, rag_stores: RagStores) -> Result<Vec<RagChunk>> {
    use kwaai_rag::{
        embedder::EmbedClient,
        meta_store::MetaStore,
        retriever::{retrieve_hybrid, RetrieveConfig},
    };
    use std::pin::Pin;

    let kb = rag_kb_or_default(&req.kb);
    if req.text.trim().is_empty() {
        anyhow::bail!("rag_query requires non-empty query text");
    }

    let (rag_cfg, tenant_id) = crate::rag_cmd::load_rag_config_for(&kb)?;
    reject_if_remote(&kb, &rag_cfg)?;

    let data_dir = rag_cfg.data_dir();
    let meta_dir = data_dir.clone();
    let meta = tokio::task::spawn_blocking(move || MetaStore::open(&meta_dir, tenant_id))
        .await
        .context("metadata store task panicked")??;

    let vs = cached_local_vs(&rag_stores, &data_dir).await?;

    let top_k = if req.top_k == 0 {
        RAG_DEFAULT_TOP_K
    } else {
        req.top_k
    } as usize;

    // Persistent keyword index. `open_backfilled` may rebuild from the
    // full metadata store the first time it sees a drifted KB (sync,
    // CPU-heavy), so it runs off the reactor; after that queries reuse
    // the on-disk index instead of rebuilding O(corpus) per query.
    // Best-effort: None falls back to the old in-RAM build inside
    // retrieve_hybrid — slower, never wrong.
    let meta = Arc::new(meta);
    let bm25 = {
        let meta = meta.clone();
        let dir = data_dir.clone();
        tokio::task::spawn_blocking(move || {
            kwaai_rag::bm25::BM25Index::open_backfilled(&dir, &meta)
        })
        .await
        .context("BM25 open task panicked")?
        .map_err(|e| warn!(error = format!("{e:#}"), "BM25 index unavailable; using in-RAM fallback"))
        .ok()
        .map(Arc::new)
    };

    let embed = EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
    let cfg = RetrieveConfig {
        top_k,
        min_score: req.min_score,
        use_sentence_window: false,
        hyde_inference_url: None,
        hyde_model: None,
        hyde_alpha: None,
        bm25,
        ..Default::default()
    };

    // retrieve_hybrid embeds the query (async HTTP); with cfg.bm25 set the
    // keyword half searches the persistent index rather than loading and
    // indexing every chunk. The residual sync work is small, and the
    // service-wide rag_lock keeps only one of these in flight.
    let chunks = retrieve_hybrid(&req.text, &cfg, &embed, &meta, move |emb, k| {
        let vs = vs.clone();
        Box::pin(async move {
            let raw = vs.search(tenant_id, &emb, k).await?;
            Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
        }) as Pin<Box<dyn std::future::Future<Output = Result<Vec<(i64, f64)>>> + Send>>
    })
    .await?;

    Ok(chunks
        .iter()
        .enumerate()
        .map(|(i, r)| RagChunk {
            rank: (i + 1) as u32,
            score: r.score,
            doc: r.chunk_meta.doc_name.clone(),
            chunk_index: r.chunk_meta.chunk_index,
            text: r.chunk_meta.text.clone(),
        })
        .collect())
}

/// `rag_delete` — drop one document and everything derived from it.
#[cfg(feature = "rag")]
async fn spawn_session_rag_delete(
    id: u64,
    req: RagDeleteRequest,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    rag_lock: Arc<Mutex<()>>,
    rag_stores: RagStores,
) {
    tokio::spawn(async move {
        let _guard = rag_lock.lock().await;

        match run_rag_delete(req, rag_stores).await {
            Ok(reply) => {
                let _ = out_tx
                    .send(Ok(ServerFrame {
                        id,
                        body: Some(server_frame::Body::RagDelete(reply)),
                    }))
                    .await;
                let _ = out_tx.send(Ok(done_frame(id))).await;
            }
            Err(e) => send_rag_error(id, &e, &out_tx).await,
        }
    });
}

/// Remove `doc_name` from the metadata store, the vector store and the
/// keyword index.
///
/// Order matters: the MetaStore delete is what *tells us which vectors to
/// drop* (it returns the chunk ids it removed), so it has to go first. That
/// makes it the commit point — if the vector or BM25 delete then fails, the
/// document is already gone from the corpus a query walks, so the residue
/// is unreachable rather than half-visible. Neither residue is silently
/// ignored: both are logged, and the vector store's is the one worth
/// reclaiming, so it fails the op rather than reporting a clean delete.
#[cfg(feature = "rag")]
async fn run_rag_delete(req: RagDeleteRequest, rag_stores: RagStores) -> Result<RagDeleteReply> {
    use kwaai_rag::meta_store::MetaStore;

    let kb = rag_kb_or_default(&req.kb);
    let doc_name = req.doc_name.trim().to_string();
    if doc_name.is_empty() {
        anyhow::bail!("rag_delete requires a document name");
    }

    let (rag_cfg, tenant_id) = crate::rag_cmd::load_rag_config_for(&kb)?;
    reject_if_remote(&kb, &rag_cfg)?;

    let data_dir = rag_cfg.data_dir();
    let meta_dir = data_dir.clone();
    let meta = tokio::task::spawn_blocking(move || MetaStore::open(&meta_dir, tenant_id))
        .await
        .context("metadata store task panicked")??;

    // Returns the chunk ids it removed, which are exactly the vector ids.
    // An empty list means the KB never held this document.
    let delete_doc = doc_name.clone();
    let ids = tokio::task::spawn_blocking(move || meta.delete_doc(&delete_doc))
        .await
        .context("metadata delete task panicked")??;

    if ids.is_empty() {
        anyhow::bail!("document '{doc_name}' is not in KB '{kb}'");
    }

    // `VectorStore::delete` is `async` in name only: its body is a SQLite
    // transaction and a std Mutex over the in-memory index, with nothing
    // to await. Awaiting it directly blocks a reactor thread for one
    // statement per chunk — ~900 for a single book-sized document — while
    // this op holds `rag_lock`, which stalls every other RAG request and
    // reads to the user as a hang. Hand it to the blocking pool like the
    // other sync work on this path.
    let vs = cached_local_vs(&rag_stores, &data_dir).await?;
    let delete_ids = ids.clone();
    let vs_for_delete = vs.clone();
    tokio::task::spawn_blocking(move || {
        futures::executor::block_on(vs_for_delete.delete(tenant_id, &delete_ids))
    })
    .await
    .context("vector delete task panicked")?
    .with_context(|| format!("removing {} vectors for '{doc_name}'", ids.len()))?;

    // Best-effort by design: the index is a derived cache, and a stale
    // entry only costs a keyword hit on text that no longer resolves to a
    // chunk. `open_backfilled` repairs the drift on a later ingest.
    let bm25_dir = data_dir.clone();
    let bm25_doc = doc_name.clone();
    let bm25 = tokio::task::spawn_blocking(move || {
        kwaai_rag::bm25::BM25Index::open(&bm25_dir, tenant_id)?.delete_doc(&bm25_doc)
    })
    .await
    .context("BM25 delete task panicked")?;
    if let Err(e) = bm25 {
        warn!(
            error = format!("{e:#}"),
            doc = %doc_name,
            "BM25 index not updated on delete; it will be repaired on the next ingest"
        );
    }

    Ok(RagDeleteReply {
        chunks_deleted: ids.len() as u32,
    })
}

/// Classify a local `generate` failure. The biggest category is model
/// load (HF resolve / Ollama blob miss / SafeTensors-vs-GGUF format
/// mismatch). The wrapper context strings in
/// [`build_inference_state`] make these easy to spot.
fn classify_generate_error(msg: &str) -> ErrorCode {
    if msg.contains("resolving HF snapshot")
        || msg.contains("resolving Ollama blob")
        || msg.contains("loading SafeTensors snapshot")
        || msg.contains("loading GGUF blob")
        || msg.contains("InferenceEngine::new")
    {
        return ErrorCode::ModelLoadFailed;
    }
    ErrorCode::Internal
}

/// Best-effort check that this node's local shard server is ready to
/// serve its assigned block range. Reads the `~/.kwaainet/run/shard.ready`
/// touchfile the shard server writes when it's bound + warm.
fn shard_ready_path_exists() -> bool {
    let p = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".kwaainet")
        .join("run")
        .join("shard.ready");
    p.exists()
}

/// Per-id counter used to give worker tasks a unique log span. Reused
/// across all Session streams; rolls over after 2^64 chats, which is
/// fine.
static SESSION_TASK_SEQ: AtomicU64 = AtomicU64::new(0);

/// Drive a local-inference `generate` within a Session. Spawns a
/// blocking worker that emits ServerFrame::Token chunks into `out_tx`,
/// terminated with Done or Error.
async fn spawn_session_generate(
    id: u64,
    req: GenerateRequest,
    cfg: Arc<KwaaiNetConfig>,
    inference_slot: Arc<Mutex<Option<Arc<Mutex<InferenceState>>>>>,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    cancels: Arc<Mutex<HashMap<u64, oneshot::Sender<()>>>>,
) {
    // Register a cancel channel before we kick off work, so a Cancel
    // arriving immediately after this frame is honoured.
    let (cancel_tx, mut cancel_rx) = oneshot::channel::<()>();
    cancels.lock().await.insert(id, cancel_tx);

    let seq = SESSION_TASK_SEQ.fetch_add(1, Ordering::Relaxed);
    let inference = match get_or_init_inference(&cfg, &inference_slot).await {
        Ok(i) => i,
        Err(e) => {
            error!(seq, id, "Session chat: inference init failed: {e:#}");
            let msg = format!("inference init failed: {e:#}");
            let _ = out_tx
                .send(Ok(error_frame(id, classify_generate_error(&msg), &msg)))
                .await;
            cancels.lock().await.remove(&id);
            return;
        }
    };

    let prompt = build_prompt(&ChatMessage {
        content: req.content,
        role: req.role,
        conversation_id: req.conversation_id,
    });

    // Token channel from the blocking worker → this task's forwarder.
    let (tok_tx, mut tok_rx) = mpsc::channel::<Result<ChatToken, Status>>(64);
    spawn_inference(inference, prompt, tok_tx);

    let cancels_for_cleanup = cancels.clone();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = &mut cancel_rx => {
                    // Cancel arrived from the Session dispatcher. Drop
                    // the receiver; the inference worker will see the
                    // channel close and stop on its next yield.
                    let _ = out_tx
                        .send(Ok(error_frame(
                            id,
                            ErrorCode::Cancelled,
                            "cancelled by client",
                        )))
                        .await;
                    break;
                }
                msg = tok_rx.recv() => {
                    match msg {
                        Some(Ok(tok)) => {
                            let is_done = tok.done;
                            let _ = out_tx
                                .send(Ok(ServerFrame {
                                    id,
                                    body: Some(server_frame::Body::Token(tok)),
                                }))
                                .await;
                            if is_done {
                                let _ = out_tx.send(Ok(done_frame(id))).await;
                                break;
                            }
                        }
                        Some(Err(status)) => {
                            let msg = status.message().to_string();
                            let _ = out_tx
                                .send(Ok(error_frame(
                                    id,
                                    classify_generate_error(&msg),
                                    &msg,
                                )))
                                .await;
                            break;
                        }
                        None => {
                            // Worker dropped its sender without a done
                            // marker — treat as a clean completion to
                            // avoid leaking a half-open operation.
                            let _ = out_tx.send(Ok(done_frame(id))).await;
                            break;
                        }
                    }
                }
            }
        }
        cancels_for_cleanup.lock().await.remove(&id);
    });
}

/// Drive a distributed `shard run` within a Session. Pipes events from
/// `shard_cmd::run_streaming` onto the Session output channel.
///
/// Unlike `spawn_session_generate`, this never touches `InferenceState` —
/// distributed inference runs across peer block-servers, not the local
/// engine — so we don't take the inference slot mutex.
async fn spawn_session_shard_run(
    id: u64,
    req: ShardRunRequest,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    cancels: Arc<Mutex<HashMap<u64, oneshot::Sender<()>>>>,
) {
    // Register the cancel channel up-front so a Cancel arriving immediately
    // after this frame still finds an entry to fire.
    let (cancel_tx, mut cancel_rx) = oneshot::channel::<()>();
    cancels.lock().await.insert(id, cancel_tx);

    let opts = crate::shard_cmd::ShardRunOptions {
        prompt: req.content,
        model: req.model,
        // Today the proto carries neither max_tokens nor a circuit id; we
        // fall through to run_streaming's defaults. Both fields are reserved
        // tag-space so they can be added without a breaking change.
        max_tokens: None,
        circuit_id: None,
    };

    let cancels_for_cleanup = cancels.clone();
    tokio::spawn(async move {
        use futures::StreamExt as _;

        let mut stream = Box::pin(crate::shard_cmd::run_streaming(opts));

        loop {
            tokio::select! {
                _ = &mut cancel_rx => {
                    // Cancel arrived from the Session dispatcher. Drop the
                    // stream; the inference worker will see its sender
                    // close and stop on its next yield.
                    let _ = out_tx
                        .send(Ok(error_frame(
                            id,
                            ErrorCode::Cancelled,
                            "cancelled by client",
                        )))
                        .await;
                    break;
                }
                ev = stream.next() => {
                    match ev {
                        Some(crate::shard_cmd::ShardRunEvent::Token(piece)) => {
                            let frame = ServerFrame {
                                id,
                                body: Some(server_frame::Body::Token(ChatToken {
                                    text: piece,
                                    done: false,
                                    finish_reason: None,
                                })),
                            };
                            if out_tx.send(Ok(frame)).await.is_err() {
                                break;
                            }
                        }
                        Some(crate::shard_cmd::ShardRunEvent::Done) => {
                            // Emit a final done=true token so multi-token
                            // clients can flush their UI state, then the
                            // structural Done terminator for the op id.
                            let _ = out_tx
                                .send(Ok(ServerFrame {
                                    id,
                                    body: Some(server_frame::Body::Token(ChatToken {
                                        text: String::new(),
                                        done: true,
                                        finish_reason: Some("stop".to_string()),
                                    })),
                                }))
                                .await;
                            let _ = out_tx.send(Ok(done_frame(id))).await;
                            break;
                        }
                        Some(crate::shard_cmd::ShardRunEvent::Error(e)) => {
                            let msg = format!("{e:#}");
                            let _ = out_tx
                                .send(Ok(error_frame(
                                    id,
                                    classify_shard_error(&msg),
                                    &msg,
                                )))
                                .await;
                            break;
                        }
                        None => {
                            // Stream ended without a terminal event — treat
                            // as a clean Done so callers don't see a half-
                            // open operation.
                            let _ = out_tx.send(Ok(done_frame(id))).await;
                            break;
                        }
                    }
                }
            }
        }

        cancels_for_cleanup.lock().await.remove(&id);
    });
}

/// Refresh cadence for block-coverage subscriptions when the client
/// doesn't ask for a specific interval. Mirrors the map-server crawler.
const DEFAULT_COVERAGE_INTERVAL_SECS: u64 = 5;

/// How long a subscription may stay silent while coverage is unchanged.
///
/// Unchanged snapshots are suppressed, so without this a stable network
/// and a wedged daemon look identical from the client's side. Sending an
/// otherwise-redundant update this often bounds that ambiguity while
/// still dropping the vast majority of duplicate frames — at the default
/// 5 s cadence this keeps roughly one tick in twelve.
const HEARTBEAT: std::time::Duration = std::time::Duration::from_secs(60);

/// The parts of a [`BlockCoverageUpdate`] that decide whether it tells the
/// client anything new.
///
/// Deliberately excludes `server_time`: it changes every tick by
/// construction, so comparing whole updates would report every snapshot as
/// changed and suppress nothing. Peers are compared as a *set* — the DHT
/// returns them in whatever order responders answered in, and a pure
/// reordering is not a change worth waking the UI for.
type CoverageIdentity = (u32, u32, bool, std::collections::BTreeSet<String>);

fn coverage_identity(u: &BlockCoverageUpdate) -> CoverageIdentity {
    let peers = u
        .peers
        .iter()
        .map(|p| {
            // Every field a client renders per row, so a peer changing its
            // range, name, throughput or trust still counts as a change.
            // Floats are formatted rather than compared bitwise: the DHT
            // round-trips them through msgpack, and a stable rendering is
            // what the client actually sees.
            format!(
                "{}|{}|{}|{}|{:.3}|{:.3}|{}",
                p.peer_id,
                p.start_block,
                p.end_block,
                p.public_name,
                p.throughput,
                p.trust_score,
                p.trust_tier,
            )
        })
        .collect();
    (u.total_blocks, u.covered_blocks, u.full_coverage, peers)
}

/// Serve a `block_coverage` op within a Session: query the DHT for the
/// model's block servers and emit one BlockCoverageUpdate (one-shot) or
/// one per refresh interval until cancelled (subscribe).
async fn spawn_session_block_coverage(
    id: u64,
    req: BlockCoverageRequest,
    cfg: Arc<KwaaiNetConfig>,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    cancels: Arc<Mutex<HashMap<u64, oneshot::Sender<()>>>>,
) {
    // Register the cancel channel up-front so a Cancel arriving immediately
    // after this frame still finds an entry to fire.
    let (cancel_tx, mut cancel_rx) = oneshot::channel::<()>();
    cancels.lock().await.insert(id, cancel_tx);

    let cancels_for_cleanup = cancels.clone();
    tokio::spawn(async move {
        use std::time::Duration;

        let dht_prefix = req
            .dht_prefix
            .clone()
            .filter(|p| !p.is_empty())
            .unwrap_or_else(|| cfg.effective_dht_prefix());
        let total_blocks = match req.total_blocks {
            Some(n) if n > 0 => n as usize,
            _ => cfg.model_total_blocks().max(1) as usize,
        };
        let interval = if req.interval_secs > 0 {
            req.interval_secs as u64
        } else {
            DEFAULT_COVERAGE_INTERVAL_SECS
        };

        // The gRPC server binds before p2pd is up during daemon startup,
        // so the p2pd connection is (re)established lazily: a one-shot
        // fetch fails fast, a subscription just retries next tick.
        let mut discovery: Option<(kwaai_p2p_daemon::P2PClient, libp2p::PeerId, Vec<String>)> =
            None;

        // Identity of the last snapshot actually sent, and when it went
        // out — together these drive the change/heartbeat decision below.
        let mut last_sent: Option<CoverageIdentity> = None;
        let mut last_send_at: Option<std::time::Instant> = None;

        loop {
            if discovery.is_none() {
                match crate::shard_cmd::connect_for_discovery(&cfg).await {
                    Ok(conn) => discovery = Some(conn),
                    Err(e) if !req.subscribe => {
                        let _ = out_tx
                            .send(Ok(error_frame(
                                id,
                                ErrorCode::Unavailable,
                                &format!("{e:#}"),
                            )))
                            .await;
                        break;
                    }
                    Err(e) => {
                        tracing::debug!(id, "block coverage: p2pd not reachable yet: {e:#}");
                    }
                }
            }

            if let Some((client, our_peer_id, bootstrap_peers)) = discovery.as_mut() {
                let chain = crate::shard_cmd::discover_chain(
                    client,
                    our_peer_id,
                    &dht_prefix,
                    total_blocks,
                    bootstrap_peers,
                )
                .await;

                let update = build_coverage_update(&cfg, &dht_prefix, total_blocks, &chain);

                // Suppress updates that would tell the client nothing new.
                //
                // Coverage is derived from DHT records with a 360 s TTL that
                // peers re-announce every ~300 s, so at a 5 s cadence the
                // overwhelming majority of ticks produce a byte-identical
                // snapshot. Sending them anyway costs a frame and a client
                // rebuild per tick to convey nothing.
                //
                // The heartbeat is what keeps that safe: silence alone is
                // ambiguous — a client cannot distinguish "nothing changed"
                // from "the daemon wedged" — so an unchanged snapshot is
                // still sent every HEARTBEAT interval. That preserves a
                // liveness signal and keeps any client-side "last updated"
                // display honest.
                let changed = last_sent.as_ref() != Some(&coverage_identity(&update));
                let heartbeat_due = last_send_at
                    .map(|t: std::time::Instant| t.elapsed() >= HEARTBEAT)
                    .unwrap_or(true);

                if changed || heartbeat_due || !req.subscribe {
                    last_sent = Some(coverage_identity(&update));
                    last_send_at = Some(std::time::Instant::now());

                    let frame = ServerFrame {
                        id,
                        body: Some(server_frame::Body::BlockCoverage(update)),
                    };
                    if out_tx.send(Ok(frame)).await.is_err() {
                        break; // client went away
                    }
                }

                if !req.subscribe {
                    let _ = out_tx.send(Ok(done_frame(id))).await;
                    break;
                }
            }

            tokio::select! {
                _ = &mut cancel_rx => {
                    let _ = out_tx
                        .send(Ok(error_frame(
                            id,
                            ErrorCode::Cancelled,
                            "cancelled by client",
                        )))
                        .await;
                    break;
                }
                _ = tokio::time::sleep(Duration::from_secs(interval)) => {}
            }
        }

        cancels_for_cleanup.lock().await.remove(&id);
    });
}

/// Assemble a [`BlockCoverageUpdate`] from a discovered chain, enriching
/// each peer with its local reputation score/tier when enabled.
fn build_coverage_update(
    cfg: &KwaaiNetConfig,
    dht_prefix: &str,
    total_blocks: usize,
    chain: &[crate::shard_cmd::BlockServerEntry],
) -> BlockCoverageUpdate {
    let rep_store = if cfg.reputation.enabled {
        Some(crate::reputation::ReputationStore::load())
    } else {
        None
    };

    let mut covered = vec![false; total_blocks];
    let peers: Vec<BlockPeer> = chain
        .iter()
        .map(|e| {
            let start = e.start_block.min(total_blocks);
            let end = e.end_block.min(total_blocks);
            if start < end {
                covered[start..end].fill(true);
            }
            let peer_b58 = e.peer_id.to_base58();
            let (trust_score, trust_tier) = match rep_store.as_ref() {
                Some(store) => {
                    let s = store.score(&peer_b58);
                    (s.score, s.tier.as_str().to_string())
                }
                None => (0.0, String::new()),
            };
            BlockPeer {
                peer_id: peer_b58,
                start_block: e.start_block as u32,
                end_block: e.end_block as u32,
                public_name: e.public_name.clone(),
                throughput: e.throughput,
                trust_score,
                trust_tier,
            }
        })
        .collect();
    let covered_blocks = covered.iter().filter(|&&c| c).count();

    BlockCoverageUpdate {
        server_time: now_rfc3339(),
        model: cfg.model.clone(),
        dht_prefix: dht_prefix.to_string(),
        total_blocks: total_blocks as u32,
        covered_blocks: covered_blocks as u32,
        full_coverage: covered_blocks == total_blocks,
        peers,
    }
}

/// Default cadence for storage-discovery subscriptions.
///
/// Much slower than block coverage: a round dials every advertised node,
/// and VPK records are re-announced on a ~120 s cycle, so polling faster
/// spends real network work to observe a registry that has not moved.
const DEFAULT_STORAGE_INTERVAL_SECS: u64 = 30;

/// How long a single node's health probe may take before it counts as
/// unreachable.
///
/// Probes run concurrently, so this bounds the whole probe phase rather
/// than each node in sequence — the wall-clock cost of a round is one
/// timeout, not one per unreachable node.
const STORAGE_PROBE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

/// The parts of a [`StorageUpdate`] that decide whether it tells the
/// client anything new.
///
/// Deliberately excludes `server_time`: it changes every round by
/// construction, so comparing whole updates would report every snapshot
/// as changed and suppress nothing. Peers are compared as a *set* — the
/// DHT returns them in whatever order responders answered in, and the
/// wire order is already normalised by sorting, so a pure reordering is
/// not a change worth waking the UI for.
///
/// `probes_pending` is not part of the identity: only resolved snapshots
/// are ever compared, so it is false on both sides by construction.
type StorageIdentity = std::collections::BTreeSet<String>;

fn storage_identity(u: &StorageUpdate) -> StorageIdentity {
    u.peers
        .iter()
        .map(|p| {
            // Every field a client renders per row, so a node changing
            // its capacity, tenancy, reachability or trust still counts
            // as a change. Floats are formatted rather than compared
            // bitwise: they round-trip through msgpack, and a stable
            // rendering is what the client actually sees.
            format!(
                "{}|{}|{}|{}|{:.3}|{}|{}|{:.3}|{:.3}|{}",
                p.peer_id,
                p.public_name,
                p.mode,
                p.vpk_version,
                p.capacity_gb,
                p.tenant_count,
                p.reachability,
                p.capacity_gb_free,
                p.trust_score,
                p.trust_tier,
            )
        })
        .collect()
}

/// Serve a `storage_discovery` op within a Session: look up the VPK node
/// registry in the DHT, probe each node's reachability, and emit the
/// resolved snapshot.
///
/// The *first* round sends two updates, because the two halves have very
/// different latencies — the DHT answers in about a second, while probing
/// nodes that will never answer costs a full timeout. Emitting the
/// registry first lets a client show the node list while reachability is
/// still resolving, rather than holding a blank view.
///
/// Later rounds send at most one, and usually none: re-announcing every
/// peer as unprobed would flicker the client's status column back to
/// "checking" on each round, and a resolved snapshot identical to the
/// last one is suppressed outright. See the comments inline.
async fn spawn_session_storage_discovery(
    id: u64,
    req: StorageDiscoveryRequest,
    cfg: Arc<KwaaiNetConfig>,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    cancels: Arc<Mutex<HashMap<u64, oneshot::Sender<()>>>>,
) {
    // Registered up-front so a Cancel arriving immediately after this
    // frame still finds an entry to fire.
    let (cancel_tx, mut cancel_rx) = oneshot::channel::<()>();
    cancels.lock().await.insert(id, cancel_tx);

    let cancels_for_cleanup = cancels.clone();
    tokio::spawn(async move {
        use std::time::Duration;

        let interval = if req.interval_secs > 0 {
            req.interval_secs as u64
        } else {
            DEFAULT_STORAGE_INTERVAL_SECS
        };

        // As with block coverage, the gRPC server binds before p2pd is
        // up, so the connection is established lazily: a one-shot fails
        // fast, a subscription retries on the next tick.
        let mut discovery: Option<(kwaai_p2p_daemon::P2PClient, libp2p::PeerId, Vec<String>)> =
            None;

        // Identity of the last resolved snapshot actually sent, and when
        // it went out — together these drive the change/heartbeat
        // decision below. `last_sent` doubles as "has the client seen a
        // resolved snapshot yet", which is what gates the pending phase.
        let mut last_sent: Option<StorageIdentity> = None;
        let mut last_send_at: Option<std::time::Instant> = None;

        loop {
            if discovery.is_none() {
                match crate::shard_cmd::connect_for_discovery(&cfg).await {
                    Ok(conn) => discovery = Some(conn),
                    Err(e) if !req.subscribe => {
                        let _ = out_tx
                            .send(Ok(error_frame(
                                id,
                                ErrorCode::Unavailable,
                                &format!("{e:#}"),
                            )))
                            .await;
                        break;
                    }
                    Err(e) => {
                        tracing::debug!(id, "storage discovery: p2pd not reachable yet: {e:#}");
                    }
                }
            }

            let mut client_died = false;

            if let Some((client, our_peer_id, bootstrap_peers)) = discovery.as_mut() {
                match crate::vpk::discover_nodes(client, our_peer_id, bootstrap_peers).await {
                    Ok(entries) => {
                        let mut peers = build_storage_peers(&cfg, &entries);

                        // Phase 1 — the registry, as advertised, before
                        // anything has been probed.
                        //
                        // Only worth sending when the client has nothing
                        // better on screen. Once a resolved snapshot has
                        // been sent, re-sending every peer as UNKNOWN
                        // would throw the whole status column back to
                        // "checking" each round — a visible flicker that
                        // replaces good data with worse. Later rounds
                        // therefore stay quiet until their probes land.
                        //
                        // Skipped entirely when probing is off: there is
                        // no second update coming, so `probes_pending`
                        // would be a lie.
                        if !req.skip_probes {
                            if last_sent.is_none() {
                                let frame = ServerFrame {
                                    id,
                                    body: Some(server_frame::Body::Storage(StorageUpdate {
                                        server_time: now_rfc3339(),
                                        probes_pending: true,
                                        peers: peers.clone(),
                                    })),
                                };
                                if out_tx.send(Ok(frame)).await.is_err() {
                                    break; // client went away
                                }
                            }

                            probe_storage_peers(client, &mut peers).await;
                        }

                        let update = StorageUpdate {
                            server_time: now_rfc3339(),
                            probes_pending: false,
                            peers,
                        };

                        // Suppress rounds that would tell the client
                        // nothing new.
                        //
                        // The registry only moves on the ~120 s announce
                        // cycle and most nodes' reachability is stable, so
                        // at a 30 s cadence the majority of rounds produce
                        // an identical snapshot. Sending them anyway costs
                        // a frame and a client rebuild to convey nothing —
                        // and, because the table re-sorts and re-renders,
                        // shows up as a flicker.
                        //
                        // The heartbeat keeps that safe: silence alone is
                        // ambiguous, so an unchanged snapshot still goes
                        // out every HEARTBEAT to prove the daemon is alive
                        // and keep the client's staleness cue honest.
                        let changed = last_sent.as_ref() != Some(&storage_identity(&update));
                        let heartbeat_due = last_send_at
                            .map(|t: std::time::Instant| t.elapsed() >= HEARTBEAT)
                            .unwrap_or(true);

                        if changed || heartbeat_due || !req.subscribe {
                            last_sent = Some(storage_identity(&update));
                            last_send_at = Some(std::time::Instant::now());

                            let frame = ServerFrame {
                                id,
                                body: Some(server_frame::Body::Storage(update)),
                            };
                            if out_tx.send(Ok(frame)).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        // The p2pd handle may be stale (daemon restarted);
                        // drop it so the next tick reconnects.
                        client_died = true;
                        if !req.subscribe {
                            let _ = out_tx
                                .send(Ok(error_frame(
                                    id,
                                    ErrorCode::Unavailable,
                                    &format!("{e:#}"),
                                )))
                                .await;
                            break;
                        }
                        tracing::debug!(id, "storage discovery round failed: {e:#}");
                    }
                }

                if !req.subscribe {
                    let _ = out_tx.send(Ok(done_frame(id))).await;
                    break;
                }
            }

            if client_died {
                discovery = None;
            }

            tokio::select! {
                _ = &mut cancel_rx => {
                    let _ = out_tx
                        .send(Ok(error_frame(
                            id,
                            ErrorCode::Cancelled,
                            "cancelled by client",
                        )))
                        .await;
                    break;
                }
                _ = tokio::time::sleep(Duration::from_secs(interval)) => {}
            }
        }

        cancels_for_cleanup.lock().await.remove(&id);
    });
}

/// Convert decoded DHT advertisements into wire peers, enriching each
/// with its local reputation score/tier when enabled.
///
/// Sorted by name then peer id so a client's row order is stable across
/// rounds — the DHT returns entries in whatever order responders
/// answered in.
fn build_storage_peers(
    cfg: &KwaaiNetConfig,
    entries: &[crate::vpk::VpkNodeEntry],
) -> Vec<StoragePeer> {
    let rep_store = if cfg.reputation.enabled {
        Some(crate::reputation::ReputationStore::load())
    } else {
        None
    };

    let mut peers: Vec<StoragePeer> = entries
        .iter()
        .map(|e| {
            let (trust_score, trust_tier) = match rep_store.as_ref() {
                Some(store) => {
                    let s = store.score(&e.peer_id);
                    (s.score, s.tier.as_str().to_string())
                }
                None => (0.0, String::new()),
            };
            StoragePeer {
                peer_id: e.peer_id.clone(),
                public_name: e.public_name.clone(),
                mode: e.mode.clone(),
                vpk_version: e.vpk_version.clone(),
                capacity_gb: e.capacity_gb,
                tenant_count: e.tenant_count,
                reachability: StorageReachability::Unknown as i32,
                capacity_gb_free: 0.0,
                trust_score,
                trust_tier,
            }
        })
        .collect();

    peers.sort_by(|a, b| {
        a.public_name
            .cmp(&b.public_name)
            .then_with(|| a.peer_id.cmp(&b.peer_id))
    });
    peers
}

/// Probe every peer's storage health concurrently, filling in
/// `reachability` and `capacity_gb_free` in place.
///
/// Unlike the CLI's serial loop, the probes are issued together: a round
/// then costs about one timeout rather than one per unreachable node,
/// which matters because unreachable nodes are the common case on a
/// NAT-heavy network.
async fn probe_storage_peers(client: &kwaai_p2p_daemon::P2PClient, peers: &mut [StoragePeer]) {
    let probes = peers.iter().map(|peer| {
        let parsed = peer.peer_id.parse::<libp2p::PeerId>();
        async move {
            let Ok(pid) = parsed else {
                // An unparseable id can never be dialled, so this is a
                // permanent negative rather than a probe failure.
                return None;
            };
            tokio::time::timeout(
                STORAGE_PROBE_TIMEOUT,
                crate::storage_rpc::rpc_health(client, &pid),
            )
            .await
            .ok()
            .and_then(|r| r.ok())
        }
    });

    let results = futures::future::join_all(probes).await;

    for (peer, health) in peers.iter_mut().zip(results) {
        match health {
            Some(h) => {
                peer.reachability = StorageReachability::Reachable as i32;
                peer.capacity_gb_free = h.capacity_gb_available;
                // The health reply is live; the DHT record may be up to
                // an announce cycle stale, so prefer the probe.
                peer.tenant_count = h.tenant_count.max(0) as u32;
            }
            None => {
                peer.reachability = StorageReachability::Unreachable as i32;
            }
        }
    }
}

/// Free-function variant of `KwaaiNetService::get_or_init_inference` so
/// session worker tasks can call it without holding a `&self` borrow.
async fn get_or_init_inference(
    cfg: &Arc<KwaaiNetConfig>,
    slot: &Arc<Mutex<Option<Arc<Mutex<InferenceState>>>>>,
) -> Result<Arc<Mutex<InferenceState>>> {
    let mut guard = slot.lock().await;
    if let Some(existing) = guard.as_ref() {
        return Ok(existing.clone());
    }
    let cfg = cfg.clone();
    let state = tokio::task::spawn_blocking(move || build_inference_state(&cfg))
        .await
        .context("inference init task panicked")??;
    let arc = Arc::new(Mutex::new(state));
    *guard = Some(arc.clone());
    Ok(arc)
}

/// Build the model-specific chat prompt from the first inbound message.
///
/// Kept deliberately simple — mirrors the Llama 3 instruct template used by
/// the OpenAI-compatible REST surface in `api.rs`. We can switch on the
/// model name later (Mistral / ChatML / etc).
fn build_prompt(msg: &ChatMessage) -> String {
    let role = if msg.role.is_empty() {
        "user"
    } else {
        msg.role.as_str()
    };
    format!(
        "<|begin_of_text|><|start_header_id|>{role}<|end_header_id|>\n\n{}<|eot_id|>\
         <|start_header_id|>assistant<|end_header_id|>\n\n",
        msg.content,
    )
}

/// Run inference and forward generated text into `tx` as ChatToken chunks.
///
/// Runs on `spawn_blocking` because both the candle-based InferenceEngine
/// and the llama.cpp backend hold non-async, non-Send-friendly state and
/// must execute on a dedicated OS thread (matches the InferenceWorker
/// pattern used by [`crate::api`]).
fn spawn_inference(
    inference: Arc<Mutex<InferenceState>>,
    prompt: String,
    tx: mpsc::Sender<Result<ChatToken, Status>>,
) {
    tokio::task::spawn_blocking(move || {
        // Block on the mutex from a sync context — fine, the contention
        // window is "one chat at a time" which is intentional today.
        let state = match inference.try_lock() {
            Ok(g) => g,
            Err(_) => {
                // Another chat is already running. Bounce the client rather
                // than block the worker pool indefinitely.
                let _ = tx.blocking_send(Err(Status::resource_exhausted(
                    "another inference is already in progress on this node",
                )));
                return;
            }
        };

        info!(
            model = %state.model_id,
            prompt_bytes = prompt.len(),
            "gRPC Chat: dispatching to inference engine",
        );

        // ── llama.cpp streaming path ────────────────────────────────────
        // When the `llama-cpp` feature is compiled in we get true per-token
        // streaming via run_inference_streaming's on_token callback. This
        // is the path the GUI will hit in normal builds.
        #[cfg(feature = "llama-cpp")]
        if let Some(ref gguf_path) = state.gguf_path {
            stream_via_llama_cpp(gguf_path, &prompt, &tx);
            let _ = tx.blocking_send(Ok(ChatToken {
                text: String::new(),
                done: true,
                finish_reason: Some("stop".to_string()),
            }));
            return;
        }

        // ── Fallback: buffered candle path ──────────────────────────────
        // TODO(streaming-candle): the InferenceEngine API returns the whole
        // String at the end of generation. To make this realtime end-to-end
        // we need to either (a) add an `InferenceProvider::generate_stream`
        // method on the trait and thread a callback through candle's decode
        // loop, or (b) always require the llama-cpp feature for the gRPC
        // server. For now we emit a single chunk so the wire framing is
        // correct, knowing the GUI will see one big delta on non-llama-cpp
        // builds. The Flutter agent should pick a build that turns on
        // --features llama-cpp until this is fixed.
        use kwaai_inference::InferenceProvider as _;
        match state.engine.generate(&state.handle, &prompt) {
            Ok(text) => {
                let _ = tx.blocking_send(Ok(ChatToken {
                    text,
                    done: false,
                    finish_reason: None,
                }));
                let _ = tx.blocking_send(Ok(ChatToken {
                    text: String::new(),
                    done: true,
                    finish_reason: Some("stop".to_string()),
                }));
            }
            Err(e) => {
                error!("Chat: generation failed: {e}");
                let _ = tx.blocking_send(Err(Status::internal(format!("inference failed: {e}"))));
            }
        }
    });
}

/// Drive llama.cpp streaming inference, pushing each generated text piece
/// onto `tx` as its own ChatToken. The done=true terminator is emitted by
/// the caller in `spawn_inference` so it's only sent once on the success
/// path.
#[cfg(feature = "llama-cpp")]
fn stream_via_llama_cpp(
    gguf_path: &std::path::Path,
    prompt: &str,
    tx: &mpsc::Sender<Result<ChatToken, Status>>,
) {
    let (backend, model) = match crate::llama_local::load_model(gguf_path) {
        Ok(p) => p,
        Err(e) => {
            error!("llama.cpp model load failed: {e}");
            let _ = tx.blocking_send(Err(Status::internal(format!("model load: {e}"))));
            return;
        }
    };

    // Sensible defaults — promote into the proto / config when we add
    // sampling controls. max_tokens=512 keeps a single response under
    // ~6 s on Apple Silicon at 80 tok/s.
    let max_tokens: usize = 512;
    let temperature: f32 = 0.8;
    let top_k: usize = 40;
    let top_p: f32 = 0.95;

    let tx_cb = tx.clone();
    let _ = crate::llama_local::run_inference_streaming(
        &backend,
        &model,
        prompt,
        max_tokens,
        temperature,
        top_k,
        top_p,
        |piece| {
            // Returning false stops generation early — used when the gRPC
            // client disconnects so we don't waste decode cycles on tokens
            // nobody will read.
            tx_cb
                .blocking_send(Ok(ChatToken {
                    text: piece,
                    done: false,
                    finish_reason: None,
                }))
                .is_ok()
        },
    );
}

// ---------------------------------------------------------------------------
// Bind / serve
// ---------------------------------------------------------------------------

/// Resolve `~/.kwaainet/run/kwaai.sock`.
#[cfg(unix)]
fn unix_socket_path() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join(".kwaainet").join(UNIX_SOCKET_RELPATH)
}

/// Spawn the gRPC server task(s) and return a handle that, when dropped,
/// signals graceful shutdown.
///
/// On POSIX we bind both transports concurrently; on other platforms we bind
/// TCP only. Either failure is logged but non-fatal: the daemon must keep
/// running even if the IPC surface didn't come up (the node still serves
/// p2p traffic).
pub fn spawn(config: KwaaiNetConfig) -> GrpcServerHandle {
    spawn_on_port(config, DEFAULT_GRPC_TCP_PORT)
}

/// [`spawn`], but binding TCP on an explicit port.
///
/// Exists for tests: `spawn` uses a fixed port, so a real `kwaainet start`
/// already running on the machine owns it and a test server would either
/// fail to bind or — worse — the test client would connect to the real
/// daemon and assert against the wrong process. Passing 0 lets the OS
/// allocate a free port; the caller reads it back off the returned handle.
pub fn spawn_on_port(config: KwaaiNetConfig, tcp_port: u16) -> GrpcServerHandle {
    let (shutdown_tcp_tx, shutdown_tcp_rx) = oneshot::channel::<()>();
    #[cfg(unix)]
    let (shutdown_unix_tx, shutdown_unix_rx) = oneshot::channel::<()>();

    let svc_state = KwaaiNetService::new(config);
    let service = KwaaiNetServer::new(svc_state);

    // TCP: every platform. Bind synchronously so the caller can learn the
    // actual port (which matters when `tcp_port` is 0) before we hand the
    // listener to tonic.
    let tcp_addr: std::net::SocketAddr = format!("127.0.0.1:{tcp_port}")
        .parse()
        .expect("valid loopback addr");
    let listener = std::net::TcpListener::bind(tcp_addr);
    let bound_port = listener
        .as_ref()
        .ok()
        .and_then(|l| l.local_addr().ok())
        .map(|a| a.port());

    let tcp_service = service.clone();
    tokio::spawn(async move {
        info!("gRPC: binding TCP at {tcp_addr}");
        let listener = match listener {
            Ok(l) => l,
            Err(e) => {
                warn!("gRPC TCP bind failed: {e}");
                return;
            }
        };
        if let Err(e) = listener.set_nonblocking(true) {
            warn!("gRPC TCP set_nonblocking failed: {e}");
            return;
        }
        let listener = match tokio::net::TcpListener::from_std(listener) {
            Ok(l) => l,
            Err(e) => {
                warn!("gRPC TCP listener conversion failed: {e}");
                return;
            }
        };
        let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);
        let serve = Server::builder()
            .add_service(tcp_service)
            .serve_with_incoming_shutdown(incoming, async {
                let _ = shutdown_tcp_rx.await;
            });
        if let Err(e) = serve.await {
            warn!("gRPC TCP server exited with error: {e}");
        }
    });

    // Unix socket: POSIX only.
    #[cfg(unix)]
    {
        let unix_path = unix_socket_path();
        let unix_service = service;
        tokio::spawn(async move {
            if let Err(e) = serve_unix(unix_path.clone(), unix_service, shutdown_unix_rx).await {
                warn!("gRPC Unix server exited with error: {e}");
            }
        });
        GrpcServerHandle {
            shutdown_tcp: Some(shutdown_tcp_tx),
            #[cfg(unix)]
            shutdown_unix: Some(shutdown_unix_tx),
            tcp_port: bound_port,
        }
    }
    #[cfg(not(unix))]
    {
        drop(service); // suppress unused warning on non-unix
        GrpcServerHandle {
            shutdown_tcp: Some(shutdown_tcp_tx),
            tcp_port: bound_port,
        }
    }
}

#[cfg(unix)]
async fn serve_unix(
    path: PathBuf,
    service: KwaaiNetServer<KwaaiNetService>,
    shutdown: oneshot::Receiver<()>,
) -> Result<()> {
    use tokio::net::UnixListener;
    use tokio_stream::wrappers::UnixListenerStream;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    // Stale socket from a previous run blocks bind() with EADDRINUSE.
    if path.exists() {
        let _ = std::fs::remove_file(&path);
    }

    let listener =
        UnixListener::bind(&path).with_context(|| format!("binding {}", path.display()))?;
    info!("gRPC: binding Unix socket at {}", path.display());

    // 0600 — only the user that started the daemon can dial in.
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
    }

    let incoming = UnixListenerStream::new(listener);
    Server::builder()
        .add_service(service)
        .serve_with_incoming_shutdown(incoming, async {
            let _ = shutdown.await;
        })
        .await
        .context("Unix gRPC serve")?;

    // Clean up the socket file so a future bind doesn't trip on a stale entry.
    let _ = std::fs::remove_file(&path);
    Ok(())
}

/// Drop-to-shutdown handle for the gRPC server task(s). Sending on the
/// embedded oneshot triggers `serve_with_shutdown` to return cleanly.
pub struct GrpcServerHandle {
    shutdown_tcp: Option<oneshot::Sender<()>>,
    #[cfg(unix)]
    shutdown_unix: Option<oneshot::Sender<()>>,
    /// Port the TCP transport actually bound, or `None` if the bind
    /// failed. Only interesting when [`spawn_on_port`] was given 0, which
    /// today is the test path — hence `allow(dead_code)` for release
    /// builds, where nothing reads it back.
    #[cfg_attr(not(test), allow(dead_code))]
    tcp_port: Option<u16>,
}

impl GrpcServerHandle {
    /// The port the TCP transport bound, if it came up.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn tcp_port(&self) -> Option<u16> {
        self.tcp_port
    }

    /// Trigger a graceful shutdown of both transports. Safe to call multiple
    /// times; subsequent calls are no-ops.
    pub fn shutdown(&mut self) {
        if let Some(tx) = self.shutdown_tcp.take() {
            let _ = tx.send(());
        }
        #[cfg(unix)]
        if let Some(tx) = self.shutdown_unix.take() {
            let _ = tx.send(());
        }
    }
}

impl Drop for GrpcServerHandle {
    fn drop(&mut self) {
        self.shutdown();
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
//
// These live inline (not in `tests/grpc_server_lifecycle.rs`) because
// `kwaai-cli` is a binary-only crate — there is no `lib.rs`, so an
// integration test file under `tests/` cannot reach `crate::grpc_server`.
// Adding a `lib.rs` would require re-exporting half the cli surface
// (`config`, `hf`, `ollama`, `llama_local`, plus their transitive deps),
// which is the opposite of "minimal pub changes". Keeping the tests inline
// gives us the same coverage at zero visibility cost.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::KwaaiNetConfig;
    use std::sync::LazyLock;
    use std::time::{Duration, Instant};
    use tokio::sync::Mutex as AsyncMutex;

    /// All tests in this module mutate process-wide env (`HOME`,
    /// `KWAAINET_HOME`) AND bind the same hardcoded loopback port
    /// (`DEFAULT_GRPC_TCP_PORT`). Cargo runs tests in a single binary on
    /// multiple threads by default; this mutex forces our tests onto a
    /// single-file conga line so they don't trample each other's env or
    /// race for the port. Async-aware so the guard is safe to hold across
    /// the .await calls that span each test's body.
    static TEST_LOCK: LazyLock<AsyncMutex<()>> = LazyLock::new(|| AsyncMutex::new(()));

    /// Sets `HOME` and `KWAAINET_HOME` to the given dir for the duration
    /// of a test. Returned `EnvGuard` restores the previous values on
    /// drop so a panic mid-test doesn't leak a fake HOME into the next.
    struct EnvGuard {
        prev_home: Option<std::ffi::OsString>,
        prev_kwaainet_home: Option<std::ffi::OsString>,
    }

    impl EnvGuard {
        fn set(dir: &std::path::Path) -> Self {
            let prev_home = std::env::var_os("HOME");
            let prev_kwaainet_home = std::env::var_os("KWAAINET_HOME");
            std::env::set_var("HOME", dir);
            std::env::set_var("KWAAINET_HOME", dir);
            Self {
                prev_home,
                prev_kwaainet_home,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match self.prev_home.take() {
                Some(v) => std::env::set_var("HOME", v),
                None => std::env::remove_var("HOME"),
            }
            match self.prev_kwaainet_home.take() {
                Some(v) => std::env::set_var("KWAAINET_HOME", v),
                None => std::env::remove_var("KWAAINET_HOME"),
            }
        }
    }

    /// Poll an async predicate until it succeeds or `timeout` elapses.
    /// Returns true on success, false on timeout. Used for "wait until the
    /// listener is up" / "wait until the listener is down" without a fixed
    /// sleep that's either flaky-short or slow-long.
    async fn wait_for<F, Fut>(timeout: Duration, mut probe: F) -> bool
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let deadline = Instant::now() + timeout;
        loop {
            if probe().await {
                return true;
            }
            if Instant::now() >= deadline {
                return false;
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    }

    /// True iff a fresh TCP connect to `port` on loopback succeeds.
    ///
    /// Takes the port explicitly: these tests bind an OS-allocated port
    /// rather than `DEFAULT_GRPC_TCP_PORT`, because a developer running a
    /// real `kwaainet start` owns the default one — probing it would
    /// report *that* daemon's liveness and the shutdown assertions could
    /// never pass.
    async fn tcp_accepting(port: u16) -> bool {
        tokio::net::TcpStream::connect(("127.0.0.1", port))
            .await
            .is_ok()
    }

    /// True iff a fresh TCP connect to `port` is refused quickly (used to
    /// assert the listener is gone after shutdown).
    async fn tcp_refused(port: u16) -> bool {
        // ConnectionRefused is the happy-path answer; any other Err (e.g.
        // network unreachable) we also treat as "not accepting". We bound
        // the dial with a short timeout so a slow stack can't lie to us.
        match tokio::time::timeout(
            Duration::from_millis(250),
            tokio::net::TcpStream::connect(("127.0.0.1", port)),
        )
        .await
        {
            Ok(Ok(_)) => false, // still accepting
            Ok(Err(_)) => true, // refused / unreachable
            Err(_) => false,    // timed out = something is listening but not answering yet
        }
    }

    /// Server lifecycle smoke test.
    ///
    /// Spawns the gRPC server against a throwaway HOME, asserts both the
    /// TCP listener and (on POSIX) the Unix socket come up cleanly with
    /// the right filesystem permissions, then drops the handle and
    /// asserts the TCP listener goes away. We deliberately do NOT drive
    /// a Chat request — `get_or_init_inference` would try to load a
    /// real model, which is platform-specific and slow.
    #[tokio::test]
    async fn server_binds_and_shuts_down_cleanly() {
        let _serial = TEST_LOCK.lock().await;

        let tmp = tempfile::tempdir().expect("create tempdir for fake HOME");
        let _env = EnvGuard::set(tmp.path());

        let config = KwaaiNetConfig::default();
        // Port 0: let the OS pick, so this doesn't collide with a real
        // daemon already holding DEFAULT_GRPC_TCP_PORT.
        let handle = spawn_on_port(config, 0);
        let port = handle.tcp_port().expect("server should bind a TCP port");

        // The server task is spawned on tokio; give it up to ~2 s to wire
        // up the TCP listener. In practice this happens in <50 ms locally.
        let up = wait_for(Duration::from_secs(2), || tcp_accepting(port)).await;
        assert!(up, "gRPC TCP listener never came up on 127.0.0.1:{port}");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let sock_path = unix_socket_path();
            // The Unix bind happens on a separate task; wait briefly for the
            // socket file to materialise rather than racing it.
            let socket_path_for_probe = sock_path.clone();
            let sock_present = wait_for(Duration::from_secs(2), || {
                let p = socket_path_for_probe.clone();
                async move { tokio::fs::metadata(&p).await.is_ok() }
            })
            .await;
            assert!(
                sock_present,
                "Unix socket {} never appeared",
                sock_path.display()
            );

            let meta = std::fs::metadata(&sock_path).expect("stat unix socket");
            // mask off file-type bits; we only care about the permission bits.
            let mode = meta.permissions().mode() & 0o777;
            assert_eq!(
                mode,
                0o600,
                "Unix socket {} must be mode 0o600 (got {:#o})",
                sock_path.display(),
                mode
            );
        }

        // Drop triggers GrpcServerHandle::shutdown -> oneshot send ->
        // tonic's serve_with_shutdown returns -> listener is closed.
        drop(handle);

        let down = wait_for(Duration::from_secs(2), || tcp_refused(port)).await;
        assert!(
            down,
            "TCP listener on 127.0.0.1:{port} did not close within 2s of dropping the handle"
        );

        #[cfg(unix)]
        {
            // The Unix serve task removes the socket file on graceful
            // shutdown (see serve_unix). Allow it a moment to run.
            let sock_path = unix_socket_path();
            let socket_path_for_probe = sock_path.clone();
            let gone = wait_for(Duration::from_secs(2), || {
                let p = socket_path_for_probe.clone();
                async move { tokio::fs::metadata(&p).await.is_err() }
            })
            .await;
            assert!(
                gone,
                "Unix socket {} not removed after shutdown",
                sock_path.display()
            );
        }
    }

    /// Chat handler "client closed before sending a prompt" path.
    ///
    /// We can't construct `tonic::Streaming<ChatMessage>` from outside
    /// tonic (the constructors are crate-private), so we drive the
    /// handler via a real loopback gRPC client. The client opens a Chat
    /// stream and immediately closes the send half by dropping the
    /// inbound channel — the server's `in_stream.message().await` then
    /// returns `Ok(None)` and the handler must surface
    /// `Status::invalid_argument`. Crucially, the inference engine is
    /// never touched on this path: we lock down the cheap, model-free
    /// branch without needing a GGUF on disk.
    #[tokio::test]
    async fn chat_returns_invalid_argument_on_empty_inbound_stream() {
        use kwaai_rpc::v1::kwaai_net_client::KwaaiNetClient;
        use tokio_stream::wrappers::ReceiverStream;

        let _serial = TEST_LOCK.lock().await;

        let tmp = tempfile::tempdir().expect("create tempdir for fake HOME");
        let _env = EnvGuard::set(tmp.path());

        let handle = spawn_on_port(KwaaiNetConfig::default(), 0);
        let port = handle.tcp_port().expect("server should bind a TCP port");

        // Make sure the server is accepting before we dial.
        let up = wait_for(Duration::from_secs(2), || tcp_accepting(port)).await;
        assert!(up, "gRPC TCP listener never came up");

        let endpoint = format!("http://127.0.0.1:{port}");
        let channel = tonic::transport::Endpoint::from_shared(endpoint)
            .expect("valid endpoint")
            .connect()
            .await
            .expect("connect to loopback gRPC server");

        let mut client = KwaaiNetClient::new(channel);

        // Build an outbound stream that produces zero messages — the
        // moment the client gives this to chat(), the server sees an
        // immediately-closed inbound stream.
        let (tx, rx) = tokio::sync::mpsc::channel::<ChatMessage>(1);
        drop(tx); // close send side -> empty stream.
        let outbound = ReceiverStream::new(rx);

        let result = client.chat(outbound).await;
        let err = result.expect_err(
            "chat() should reject an immediately-closed inbound stream with Status::invalid_argument",
        );
        assert_eq!(
            err.code(),
            tonic::Code::InvalidArgument,
            "expected InvalidArgument, got {:?} ({})",
            err.code(),
            err.message()
        );

        drop(handle);
        let down = wait_for(Duration::from_secs(2), || tcp_refused(port)).await;
        assert!(down, "TCP listener did not close after handle drop");
    }

    #[test]
    fn coverage_update_counts_blocks_and_clamps_ranges() {
        let mut cfg = KwaaiNetConfig::default();
        // Keep the test hermetic: enabled reputation would read the real
        // user's on-disk ReputationStore.
        cfg.reputation.enabled = false;

        let entry = |start, end| crate::shard_cmd::BlockServerEntry {
            peer_id: libp2p::PeerId::random(),
            start_block: start,
            end_block: end,
            public_name: "peer".into(),
            throughput: 1.0,
            trust_score: None,
            lease_v1: false,
        };

        // Gap at block 3: [0,3) + [4,8).
        let update = build_coverage_update(&cfg, "Model-X", 8, &[entry(0, 3), entry(4, 8)]);
        assert_eq!(update.dht_prefix, "Model-X");
        assert_eq!(update.total_blocks, 8);
        assert_eq!(update.covered_blocks, 7);
        assert!(!update.full_coverage);
        assert_eq!(update.peers.len(), 2);
        assert!(update.peers[0].trust_tier.is_empty());

        // Overlapping ranges cover fully, and an end past total_blocks
        // must clamp for the bitmap while surviving verbatim on the peer.
        let update = build_coverage_update(&cfg, "Model-X", 8, &[entry(0, 5), entry(3, 12)]);
        assert_eq!(update.covered_blocks, 8);
        assert!(update.full_coverage);
        assert_eq!(update.peers[1].end_block, 12);
    }

    #[test]
    fn storage_peers_start_unprobed_and_sort_stably() {
        let mut cfg = KwaaiNetConfig::default();
        // Keep the test hermetic: enabled reputation would read the real
        // user's on-disk ReputationStore.
        cfg.reputation.enabled = false;

        let entry = |name: &str, peer: &str, cap: f64| crate::vpk::VpkNodeEntry {
            peer_id: peer.into(),
            mode: "eve".into(),
            capacity_gb: cap,
            tenant_count: 2,
            vpk_version: "0.5.0".into(),
            public_name: name.into(),
        };

        // Deliberately out of order — the DHT returns entries in whatever
        // order responders answered in.
        let peers = build_storage_peers(
            &cfg,
            &[
                entry("metro", "12D3KooWZ", 48.3),
                entry("arach", "12D3KooWA", 34.5),
                entry("metro", "12D3KooWB", 1.0),
            ],
        );

        // Name first, peer id as the tie-break.
        let order: Vec<&str> = peers.iter().map(|p| p.peer_id.as_str()).collect();
        assert_eq!(order, ["12D3KooWA", "12D3KooWB", "12D3KooWZ"]);

        // Nothing has been probed yet, so every row must read as pending
        // rather than as unreachable, and free capacity is not yet known.
        for p in &peers {
            assert_eq!(p.reachability, StorageReachability::Unknown as i32);
            assert_eq!(p.capacity_gb_free, 0.0);
        }

        // Advertised values survive verbatim; trust stays empty while the
        // reputation system is off.
        assert_eq!(peers[0].capacity_gb, 34.5);
        assert_eq!(peers[0].tenant_count, 2);
        assert_eq!(peers[0].mode, "eve");
        assert!(peers[0].trust_tier.is_empty());
    }

    #[test]
    fn storage_identity_ignores_time_and_peer_order() {
        let peer = |id: &str, reach: i32, free: f64| StoragePeer {
            peer_id: id.into(),
            public_name: "node".into(),
            mode: "eve".into(),
            vpk_version: "0.5.0".into(),
            capacity_gb: 42.0,
            tenant_count: 1,
            reachability: reach,
            capacity_gb_free: free,
            trust_score: 0.5,
            trust_tier: "KNOWN".into(),
        };
        let update = |time: &str, peers: Vec<StoragePeer>| StorageUpdate {
            server_time: time.into(),
            probes_pending: false,
            peers,
        };

        let a = update("t0", vec![peer("A", 1, 10.0), peer("B", 2, 0.0)]);

        // A later round with the same nodes tells the client nothing new,
        // even though server_time always advances.
        let later = update("t1", vec![peer("A", 1, 10.0), peer("B", 2, 0.0)]);
        assert_eq!(storage_identity(&a), storage_identity(&later));

        // Responder order is not information.
        let reordered = update("t2", vec![peer("B", 2, 0.0), peer("A", 1, 10.0)]);
        assert_eq!(storage_identity(&a), storage_identity(&reordered));

        // A node dropping out of the registry is.
        let departed = update("t3", vec![peer("A", 1, 10.0)]);
        assert_ne!(storage_identity(&a), storage_identity(&departed));

        // So is one coming back within reach — the status column changes.
        let recovered = update("t4", vec![peer("A", 1, 10.0), peer("B", 1, 7.0)]);
        assert_ne!(storage_identity(&a), storage_identity(&recovered));

        // And so is free space moving, which is what the cylinder draws.
        let filled = update("t5", vec![peer("A", 1, 3.0), peer("B", 2, 0.0)]);
        assert_ne!(storage_identity(&a), storage_identity(&filled));
    }

    #[test]
    fn coverage_identity_ignores_time_and_peer_order() {
        let peer = |id: &str, start, end| BlockPeer {
            peer_id: id.to_string(),
            start_block: start,
            end_block: end,
            public_name: "peer".into(),
            throughput: 1.0,
            trust_score: 0.5,
            trust_tier: String::new(),
        };
        let update = |time: &str, peers: Vec<BlockPeer>| BlockCoverageUpdate {
            server_time: time.to_string(),
            model: "m".into(),
            dht_prefix: "Model-X".into(),
            total_blocks: 8,
            covered_blocks: 8,
            full_coverage: true,
            peers,
        };

        let a = update(
            "2026-01-01T00:00:00Z",
            vec![peer("A", 0, 4), peer("B", 4, 8)],
        );

        // server_time advances every tick by construction; if it counted,
        // nothing would ever be suppressed and the diff would be useless.
        let later = update(
            "2026-01-01T00:00:05Z",
            vec![peer("A", 0, 4), peer("B", 4, 8)],
        );
        assert_eq!(coverage_identity(&a), coverage_identity(&later));

        // Responder order is not information — the same peers arriving in
        // a different order must not wake the client.
        let reordered = update(
            "2026-01-01T00:00:05Z",
            vec![peer("B", 4, 8), peer("A", 0, 4)],
        );
        assert_eq!(coverage_identity(&a), coverage_identity(&reordered));

        // A peer leaving is the change the subscription exists to report.
        let departed = update("2026-01-01T00:00:05Z", vec![peer("A", 0, 4)]);
        assert_ne!(coverage_identity(&a), coverage_identity(&departed));

        // So is one silently changing the range it serves.
        let reranged = update(
            "2026-01-01T00:00:05Z",
            vec![peer("A", 0, 4), peer("B", 4, 6)],
        );
        assert_ne!(coverage_identity(&a), coverage_identity(&reranged));

        // And so is a trust re-tiering, which the client renders per row.
        let mut retiered_peer = peer("B", 4, 8);
        retiered_peer.trust_tier = "TRUSTED".into();
        let retiered = update("2026-01-01T00:00:05Z", vec![peer("A", 0, 4), retiered_peer]);
        assert_ne!(coverage_identity(&a), coverage_identity(&retiered));
    }

    // -----------------------------------------------------------------
    // RAG session ops
    // -----------------------------------------------------------------
    //
    // These are hermetic: `EnvGuard` points HOME *and* KWAAINET_HOME at a
    // tempdir, so `KwaaiNetConfig::load_or_create` reads and writes there
    // and never touches the developer's real ~/.kwaainet. No Ollama and
    // no network are required — every case below fails (or succeeds)
    // before any embedding call would happen.

    #[cfg(feature = "rag")]
    mod rag {
        use super::*;
        use kwaai_rpc::v1::{
            client_frame, kwaai_net_client::KwaaiNetClient, server_frame, ClientFrame,
            RagIngestRequest, RagQueryRequest, RagStatusRequest,
        };
        use tokio_stream::wrappers::ReceiverStream;
        use tokio_stream::StreamExt as _;

        /// Boilerplate shared by every RAG round-trip test: bring the
        /// server up, open a Session, send exactly one ClientFrame, and
        /// collect ServerFrames until the op terminates.
        ///
        /// Returns every frame received for that id, terminator included,
        /// so a caller can assert on both the payload and how it ended.
        ///
        /// Takes the port the server under test actually bound, rather
        /// than assuming `DEFAULT_GRPC_TCP_PORT`. These tests spawn with
        /// port 0 (see `spawn_test_server`), because a real
        /// `kwaainet start` on the developer's machine owns the default
        /// port — dialing it would silently interrogate *that* daemon and
        /// assert against the wrong process.
        async fn round_trip(port: u16, body: client_frame::Body) -> Vec<server_frame::Body> {
            let endpoint = format!("http://127.0.0.1:{port}");
            let channel = tonic::transport::Endpoint::from_shared(endpoint)
                .expect("valid endpoint")
                .connect()
                .await
                .expect("connect to the test server");
            let mut client = KwaaiNetClient::new(channel);

            let (tx, rx) = tokio::sync::mpsc::channel::<ClientFrame>(4);
            tx.send(ClientFrame {
                id: 1,
                body: Some(body),
            })
            .await
            .expect("send request frame");
            // Keep the send half open: dropping it closes the session
            // before the server has replied.

            let mut stream = client
                .session(ReceiverStream::new(rx))
                .await
                .expect("open Session")
                .into_inner();

            let mut bodies = Vec::new();
            // Bounded so a hang fails loudly instead of blocking the suite.
            let deadline = tokio::time::Instant::now() + Duration::from_secs(20);
            loop {
                let next = tokio::time::timeout_at(deadline, stream.next()).await;
                let frame = match next {
                    Ok(Some(Ok(f))) => f,
                    Ok(Some(Err(e))) => panic!("session stream error: {e}"),
                    Ok(None) => break,
                    Err(_) => panic!("timed out waiting for a terminal frame; got {bodies:?}"),
                };
                if let Some(b) = frame.body {
                    let terminal = matches!(
                        b,
                        server_frame::Body::Done(_) | server_frame::Body::Error(_)
                    );
                    bodies.push(b);
                    if terminal {
                        break;
                    }
                }
            }
            bodies
        }

        /// Spawn a server on an OS-allocated port and return it with the
        /// port it landed on. Every RAG test uses this rather than
        /// `spawn`, so the suite never contends with (or accidentally
        /// talks to) a real daemon on the fixed port.
        fn spawn_test_server() -> (GrpcServerHandle, u16) {
            let handle = spawn_on_port(KwaaiNetConfig::default(), 0);
            let port = handle
                .tcp_port()
                .expect("test server should have bound an ephemeral TCP port");
            (handle, port)
        }

        /// Assert the op ended in an Error frame with `want`, and return
        /// its message for further assertions.
        fn expect_error(bodies: &[server_frame::Body], want: ErrorCode) -> String {
            let last = bodies.last().expect("at least one frame");
            match last {
                server_frame::Body::Error(e) => {
                    assert_eq!(
                        e.code, want as i32,
                        "expected {want:?}, got code={} message={:?}",
                        e.code, e.message
                    );
                    e.message.clone()
                }
                other => panic!("expected an Error frame, got {other:?}"),
            }
        }

        /// A daemon with no RAG config at all reports zero KBs and ends
        /// cleanly — this is how the GUI tells "nothing initialised yet"
        /// apart from "this build has no RAG" (which is UNIMPLEMENTED).
        #[tokio::test]
        async fn rag_status_on_fresh_config_is_empty() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());

            let (handle, port) = spawn_test_server();
            let bodies = round_trip(port, client_frame::Body::RagStatus(RagStatusRequest {})).await;

            assert!(
                matches!(bodies.last(), Some(server_frame::Body::Done(_))),
                "rag_status should end in Done, got {bodies:?}"
            );
            match bodies.first() {
                Some(server_frame::Body::RagStatus(u)) => assert!(
                    u.kbs.is_empty(),
                    "fresh config should report no KBs, got {:?}",
                    u.kbs
                ),
                other => panic!("expected a RagStatus frame first, got {other:?}"),
            }

            drop(handle);
        }

        /// Querying a KB that was never initialised must be NOT_FOUND and
        /// must carry the actionable CLI hint, not a bare "missing key".
        #[tokio::test]
        async fn rag_query_uninitialised_kb_is_not_found() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());

            let (handle, port) = spawn_test_server();
            let bodies = round_trip(
                port,
                client_frame::Body::RagQuery(RagQueryRequest {
                    kb: "nope".into(),
                    text: "what is kwaainet".into(),
                    top_k: 5,
                    min_score: 0.0,
                }),
            )
            .await;

            let msg = expect_error(&bodies, ErrorCode::NotFound);
            assert!(
                msg.contains("not initialised") && msg.contains("rag init"),
                "message should tell the operator how to fix it, got {msg:?}"
            );

            drop(handle);
        }

        /// A bad path is INVALID_ARGUMENT (NOT_FOUND is reserved for a
        /// missing KB), and it is reported *before* any embedding work,
        /// so this needs no Ollama.
        #[tokio::test]
        async fn rag_ingest_missing_file_is_invalid_argument() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());

            // Give the KB a real local config so we get past the
            // not-initialised gate and reach the path check.
            let mut cfg = KwaaiNetConfig::load_or_create().expect("load config");
            cfg.set_rag_kb(
                "default",
                crate::config::RagConfig {
                    tenant_id: Some(uuid::Uuid::new_v4().to_string()),
                    storage_url: Some("local".to_string()),
                    rag_data_dir: Some(tmp.path().join("ragdata").to_string_lossy().into_owned()),
                    ..crate::config::RagConfig::default()
                },
            );
            cfg.save().expect("save config");

            let (handle, port) = spawn_test_server();
            let missing = tmp.path().join("definitely-not-here.txt");
            let bodies = round_trip(
                port,
                client_frame::Body::RagIngest(RagIngestRequest {
                    kb: "default".into(),
                    path: missing.to_string_lossy().into_owned(),
                    doc_name: String::new(),
                }),
            )
            .await;

            let msg = expect_error(&bodies, ErrorCode::InvalidArgument);
            assert!(
                msg.contains("no such file"),
                "message should name the problem, got {msg:?}"
            );

            drop(handle);
        }

        /// A KB pointed at remote storage is rejected up front rather
        /// than half-served. Also proves the remote check runs before any
        /// network call, so this stays hermetic.
        #[tokio::test]
        async fn rag_query_remote_kb_is_unimplemented() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());

            let mut cfg = KwaaiNetConfig::load_or_create().expect("load config");
            cfg.set_rag_kb(
                "remote",
                crate::config::RagConfig {
                    tenant_id: Some(uuid::Uuid::new_v4().to_string()),
                    // Anything other than "local" means remote storage.
                    storage_url: Some("http://192.0.2.1:7432".to_string()),
                    ..crate::config::RagConfig::default()
                },
            );
            cfg.save().expect("save config");

            let (handle, port) = spawn_test_server();
            let bodies = round_trip(
                port,
                client_frame::Body::RagQuery(RagQueryRequest {
                    kb: "remote".into(),
                    text: "anything".into(),
                    top_k: 0,
                    min_score: 0.0,
                }),
            )
            .await;

            let msg = expect_error(&bodies, ErrorCode::Unimplemented);
            assert!(
                msg.contains("not yet supported over gRPC"),
                "message should explain the phase-1 limit, got {msg:?}"
            );

            drop(handle);
        }

        /// An empty query is rejected as INVALID_ARGUMENT before the KB
        /// is even resolved.
        #[tokio::test]
        async fn rag_query_empty_text_is_invalid_argument() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());

            let (handle, port) = spawn_test_server();
            let bodies = round_trip(
                port,
                client_frame::Body::RagQuery(RagQueryRequest {
                    kb: String::new(),
                    text: "   ".into(),
                    top_k: 0,
                    min_score: 0.0,
                }),
            )
            .await;

            expect_error(&bodies, ErrorCode::InvalidArgument);

            drop(handle);
        }

        /// `rag_init` against an embedding endpoint that nothing is
        /// listening on must surface UNAVAILABLE, not a generic Internal.
        ///
        /// Points at 127.0.0.1:1 (reserved, reliably refused) so the
        /// connect fails immediately — no timeout tuning needed and the
        /// test stays fast. `EmbedClient::new` falls back to
        /// OLLAMA_BASE_URL when no explicit URL is passed, which is
        /// exactly the path `init_kb` takes, so no real Ollama is
        /// contacted even if one is running on this machine.
        #[tokio::test]
        async fn rag_init_with_unreachable_embed_url_is_unavailable() {
            use kwaai_rpc::v1::RagInitRequest;

            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());

            let prev_ollama = std::env::var_os("OLLAMA_BASE_URL");
            std::env::set_var("OLLAMA_BASE_URL", "http://127.0.0.1:1");

            let (handle, port) = spawn_test_server();
            let bodies = round_trip(
                port,
                client_frame::Body::RagInit(RagInitRequest {
                    kb: "probe".into(),
                    embed_model: "nomic-embed-text".into(),
                }),
            )
            .await;

            match prev_ollama {
                Some(v) => std::env::set_var("OLLAMA_BASE_URL", v),
                None => std::env::remove_var("OLLAMA_BASE_URL"),
            }

            expect_error(&bodies, ErrorCode::Unavailable);

            drop(handle);
        }

        /// Deleting from a KB that was never initialised is NOT_FOUND,
        /// same as querying one.
        #[tokio::test]
        async fn rag_delete_uninitialised_kb_is_not_found() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());

            let (handle, port) = spawn_test_server();
            let bodies = round_trip(
                port,
                client_frame::Body::RagDelete(RagDeleteRequest {
                    kb: "nope".into(),
                    doc_name: "handbook.pdf".into(),
                }),
            )
            .await;

            let msg = expect_error(&bodies, ErrorCode::NotFound);
            assert!(
                msg.contains("not initialised"),
                "message should name the real problem, got {msg:?}"
            );

            drop(handle);
        }

        /// Deleting a document the KB doesn't hold is NOT_FOUND rather
        /// than a silent success — a stale UI row must not report that it
        /// deleted something that was never there.
        ///
        /// Reaches the MetaStore (which a fresh KB opens empty), so it
        /// exercises the real lookup rather than an early argument gate.
        #[tokio::test]
        async fn rag_delete_unknown_doc_is_not_found() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());

            let mut cfg = KwaaiNetConfig::load_or_create().expect("load config");
            cfg.set_rag_kb(
                "default",
                crate::config::RagConfig {
                    tenant_id: Some(uuid::Uuid::new_v4().to_string()),
                    storage_url: Some("local".to_string()),
                    ..crate::config::RagConfig::default()
                },
            );
            cfg.save().expect("save config");

            let (handle, port) = spawn_test_server();
            let bodies = round_trip(
                port,
                client_frame::Body::RagDelete(RagDeleteRequest {
                    kb: "default".into(),
                    doc_name: "never-ingested.pdf".into(),
                }),
            )
            .await;

            let msg = expect_error(&bodies, ErrorCode::NotFound);
            assert!(
                msg.contains("never-ingested.pdf"),
                "message should name the document, got {msg:?}"
            );

            drop(handle);
        }

        /// An empty document name is rejected before the KB is resolved.
        #[tokio::test]
        async fn rag_delete_empty_doc_name_is_invalid_argument() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());

            let (handle, port) = spawn_test_server();
            let bodies = round_trip(
                port,
                client_frame::Body::RagDelete(RagDeleteRequest {
                    kb: String::new(),
                    doc_name: "   ".into(),
                }),
            )
            .await;

            expect_error(&bodies, ErrorCode::InvalidArgument);

            drop(handle);
        }
    }
}
