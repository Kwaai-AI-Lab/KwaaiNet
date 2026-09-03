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
use tokio::sync::{mpsc, oneshot, Mutex, RwLock};
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tracing::{error, info, warn};

use kwaai_p2p::{
    reachability::{Reachability, Source},
    NetworkHandle,
};
use kwaai_rpc::v1::{
    client_frame,
    error::Code as ErrorCode,
    kwaai_net_server::{KwaaiNet, KwaaiNetServer},
    server_frame, BlockCoverageRequest, BlockCoverageUpdate, BlockPeer, Cancel, ChatMessage,
    ChatToken, ClientFrame, ConnectReply, ConnectRequest, ConnectedPeer, Done, Error as RpcError,
    GenerateRequest, NetworkRequest, NetworkUpdate, PeerConnKind, PingReply, PingRequest,
    RoutingPeer, SelfStatus, ServerFrame, ShardRunRequest, StatusReply, StorageDiscoveryRequest,
    StoragePeer, StorageReachability, StorageUpdate, UpdateReason,
};
use std::collections::{BTreeSet, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use crate::config::KwaaiNetConfig;

/// Default TCP loopback port, used when nothing asks for a specific one.
pub const DEFAULT_GRPC_TCP_PORT: u16 = 8093;

/// Env var naming the TCP port to bind. Same name the GUI already uses on the
/// client side, and the only way a port survives `start --daemon`, which
/// re-execs `run-node` with no arguments but inherits the environment.
pub const GRPC_PORT_ENV: &str = "KWAAINET_GRPC_PORT";

/// Relative path (under the KwaaiNet dir) where we bind the Unix socket.
#[cfg(unix)]
const UNIX_SOCKET_RELPATH: &str = "run/kwaai.sock";

/// File under `run/` recording the port we actually bound, so a supervisor
/// that restarts (or a second GUI attaching to a live daemon) can find it.
const GRPC_PORT_RELPATH: &str = "run/kwaainet.grpc";

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
    /// The native swarm handle, filled in once the p2p node is up.
    ///
    /// Late-bound because `spawn` runs before the node starts — deliberately,
    /// so ping/status/generate answer while p2p is still coming up. Empty
    /// therefore means one of two things, and the Network op distinguishes
    /// them: the node has not started yet (transient), or this daemon is
    /// running the Go p2p path and never will (permanent).
    net: Arc<RwLock<Option<NetworkHandle>>>,
    /// Captured at service construction so StatusReply.uptime_secs can
    /// report a process-level uptime without a separate clock.
    started_at: Instant,
}

impl KwaaiNetService {
    pub fn new(config: KwaaiNetConfig) -> Self {
        Self {
            config: Arc::new(config),
            inference: Arc::new(Mutex::new(None)),
            net: Arc::new(RwLock::new(None)),
            started_at: Instant::now(),
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
        let net_slot = self.net.clone();
        let started_at = self.started_at;

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
                        // peer_count is the routing-table size, matching the
                        // field's documented meaning. All three are 0 when the
                        // swarm is not up yet, or on the Go p2p path where
                        // there is no handle to ask — the same value this
                        // reported unconditionally before.
                        let (peer_count, bootstrap_total, bootstrap_reachable) =
                            match net_slot.read().await.clone() {
                                Some(handle) => match handle.network_snapshot().await {
                                    Ok(snapshot) => {
                                        let bootstraps = crate::peers_view::bootstrap_peer_ids();
                                        let (total, reachable) =
                                            bootstrap_health(&snapshot, &bootstraps);
                                        (snapshot.routing.len() as u32, total, reachable)
                                    }
                                    Err(_) => (0, 0, 0),
                                },
                                None => (0, 0, 0),
                            };

                        let reply = StatusReply {
                            server_time: now_rfc3339(),
                            model: cfg.model.clone(),
                            shard_ready: shard_ready_path_exists(),
                            peer_count,
                            uptime_secs: started_at.elapsed().as_secs(),
                            // Same constant the updater compares against, so
                            // the version reported over the wire can never
                            // drift from the one used for update checks.
                            version: crate::updater::CURRENT_VERSION.to_string(),
                            bootstrap_total,
                            bootstrap_reachable,
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

                    client_frame::Body::Network(req) => {
                        spawn_session_network(
                            id,
                            req,
                            net_slot.clone(),
                            cfg.clone(),
                            out_tx.clone(),
                            cancels.clone(),
                        )
                        .await;
                    }

                    client_frame::Body::Connect(req) => {
                        spawn_session_connect(id, req, net_slot.clone(), out_tx.clone()).await;
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
/// doesn't ask for a specific interval. Mirrors the map's DHT crawler.
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

/// Default refresh cadence for a Network subscription, in seconds.
///
/// Faster than the two DHT-backed ops because a snapshot is a read of local
/// swarm state — one trip through the event loop, no network. The cost of a
/// tick is low enough that the interesting question is how quickly a
/// connect/disconnect should surface, and 5s is about the limit of what feels
/// live.
const DEFAULT_NETWORK_INTERVAL_SECS: u64 = 5;

/// The comparable content of a [`NetworkUpdate`].
///
/// Deliberately excludes `server_time` (moves every tick), `reason` (describes
/// *why* we are sending, not what changed) and — critically — `rtt_ms`, which
/// jitters on every ping and would defeat suppression entirely, turning a quiet
/// node into a 5-second frame generator.
///
/// Connections are compared as a set: the swarm iterates a `HashMap`, so
/// ordering is arbitrary and a pure reordering is not a change worth waking the
/// UI for. The routing table is compared the same way.
type NetworkIdentity = (
    String,
    bool,
    bool,
    String,
    BTreeSet<String>,
    BTreeSet<String>,
    // (bootstrap_total, bootstrap_reachable). A bootstrap appearing already
    // changes the connected set, but the count can also *fall* with no other
    // change — the contact window expiring on a bootstrap that went quiet —
    // and that flip is exactly what the GUI's banner is watching for.
    (u32, u32),
);

fn network_identity(u: &NetworkUpdate) -> NetworkIdentity {
    let self_status = u.self_status.clone().unwrap_or_default();
    let connected = u
        .connected
        .iter()
        .map(|p| {
            // Every field a client renders per row *except* rtt_ms — see the
            // type comment. Protocols are joined rather than nested so the
            // whole row stays one comparable string.
            // `dcutr` is in here deliberately: an upgrade flips a connection
            // from relayed to direct, which is exactly the kind of change the
            // user is watching this page for.
            format!(
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                p.peer_id,
                p.addr,
                p.kind,
                p.direction,
                p.is_bootstrap,
                p.is_trusted_relay,
                p.protocols.join(","),
                p.agent_version,
                p.via,
                p.dcutr,
            )
        })
        .collect();
    let routing = u
        .routing
        .iter()
        .map(|r| {
            // Addresses are part of the identity: an entry losing its last
            // dialable address is a real change of state, and omitting them
            // here would suppress the update that reports it.
            let mut addrs: Vec<&str> = r.addrs.iter().map(String::as_str).collect();
            addrs.sort_unstable();
            format!(
                "{}|{}|{}|{}",
                r.peer_id,
                r.connected,
                r.is_bootstrap,
                addrs.join(",")
            )
        })
        .collect();

    (
        self_status.reachability,
        self_status.using_relay,
        self_status.announceable,
        // Registering or removing a handler changes what this node serves, and
        // the view shows it, so it has to wake the client.
        self_status.local_protocols.join(","),
        connected,
        routing,
        (u.bootstrap_total, u.bootstrap_reachable),
    )
}

/// `NetworkRequest` — local swarm state, sampled and pushed.
///
/// Two things distinguish this from its DHT-backed siblings. First, it needs
/// the native swarm: an empty handle slot is answered with a typed error rather
/// than a blank view (see [`KwaaiNetService::net`]). Second, it is not purely
/// polled — reachability is a real event source, so the loop waits on the
/// announce watch channel alongside the interval timer and reports which one
/// woke it in `NetworkUpdate.reason`.
async fn spawn_session_network(
    id: u64,
    req: NetworkRequest,
    net: Arc<RwLock<Option<NetworkHandle>>>,
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

        // Resolve the handle once. The slot is filled during node startup and
        // never cleared, so a subscription that finds it empty would spin
        // forever — better to say so immediately and let the client decide.
        let handle = match net.read().await.clone() {
            Some(h) => h,
            None => {
                // Two very different situations, and the client acts on them
                // differently: on the Go p2p path the slot is never filled, so
                // retrying is pointless; during native startup it is about to
                // be, so retrying is exactly right.
                let (code, msg) = if cfg.native_p2p() {
                    (
                        ErrorCode::Unavailable,
                        "p2p node is still starting; retry shortly",
                    )
                } else {
                    // UNIMPLEMENTED, not UNAVAILABLE: on the Go p2p path this
                    // operation is not merely down, it is absent, and the slot
                    // will never fill. A client that retries UNAVAILABLE would
                    // do so forever.
                    (
                        ErrorCode::Unimplemented,
                        "network view requires the native p2p stack \
                         (`kwaainet config set native_p2p true`)",
                    )
                };
                let _ = out_tx.send(Ok(error_frame(id, code, msg))).await;
                cancels_for_cleanup.lock().await.remove(&id);
                return;
            }
        };

        let interval = if req.interval_secs > 0 {
            req.interval_secs as u64
        } else {
            DEFAULT_NETWORK_INTERVAL_SECS
        };

        let bootstraps = crate::peers_view::bootstrap_peer_ids();
        let trusted_relays = crate::peers_view::trusted_relay_peer_ids();

        // The reachability event source. `changed()` fires only on a genuine
        // change — address churn and repeated identifies do not move it — so
        // this arm cannot become chatty.
        let mut announce_rx = handle.announce_state();

        let mut last_sent: Option<NetworkIdentity> = None;
        let mut last_send_at: Option<std::time::Instant> = None;
        // Why we are building this particular snapshot. The first one is a
        // plain sample; later iterations set it from whichever select! arm won.
        let mut reason = UpdateReason::Tick;

        loop {
            let snapshot = match handle.network_snapshot().await {
                Ok(s) => s,
                Err(e) => {
                    // The swarm is gone (shutdown, or the service task died).
                    // Nothing a retry can fix, so end the operation rather than
                    // looping on a dead handle.
                    let _ = out_tx
                        .send(Ok(error_frame(
                            id,
                            ErrorCode::Unavailable,
                            &format!("network snapshot failed: {e}"),
                        )))
                        .await;
                    break;
                }
            };

            let update = build_network_update(&snapshot, &bootstraps, &trusted_relays, reason);

            // Same change/heartbeat contract as block coverage: suppress
            // snapshots that would tell the client nothing, but never go
            // silent for longer than HEARTBEAT, so silence stays readable as
            // "nothing changed" rather than "the daemon wedged".
            //
            // A reachability wake-up bypasses the suppression check entirely.
            // It is an event, and the whole reason for the watch arm is that
            // the client should see it immediately.
            let identity = network_identity(&update);
            let changed = last_sent.as_ref() != Some(&identity);
            let heartbeat_due = last_send_at
                .map(|t: std::time::Instant| t.elapsed() >= HEARTBEAT)
                .unwrap_or(true);
            let is_event = reason == UpdateReason::Reachability;

            if changed || heartbeat_due || is_event || !req.subscribe {
                // A tick that changed nothing and is only going out to prove
                // liveness says so, rather than masquerading as fresh news.
                let update = if !changed && !is_event && req.subscribe {
                    NetworkUpdate {
                        reason: UpdateReason::Heartbeat as i32,
                        ..update
                    }
                } else if changed && reason == UpdateReason::Tick && req.subscribe {
                    // A sampled tick that *did* change is a peer-set change:
                    // reachability changes arrive on the other arm.
                    NetworkUpdate {
                        reason: UpdateReason::Peers as i32,
                        ..update
                    }
                } else {
                    update
                };

                last_sent = Some(identity);
                last_send_at = Some(std::time::Instant::now());

                let frame = ServerFrame {
                    id,
                    body: Some(server_frame::Body::Network(update)),
                };
                if out_tx.send(Ok(frame)).await.is_err() {
                    break; // client went away
                }
            }

            if !req.subscribe {
                let _ = out_tx.send(Ok(done_frame(id))).await;
                break;
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
                // Reachability moved. Rebuild immediately rather than waiting
                // out the remaining interval — a NAT transition is exactly the
                // thing a user is watching this page for.
                changed = announce_rx.changed() => {
                    if changed.is_err() {
                        // Sender dropped: the swarm is shutting down.
                        break;
                    }
                    reason = UpdateReason::Reachability;
                }
                _ = tokio::time::sleep(Duration::from_secs(interval)) => {
                    reason = UpdateReason::Tick;
                }
            }
        }

        cancels_for_cleanup.lock().await.remove(&id);
    });
}

/// `ConnectRequest` — dial a peer we know of but are not connected to.
///
/// One-shot: a reply then Done, with no subscription to manage. The dial
/// itself can take a while (a DHT lookup, then the dial), so it runs in its
/// own task rather than blocking the session loop.
async fn spawn_session_connect(
    id: u64,
    req: ConnectRequest,
    net: Arc<RwLock<Option<NetworkHandle>>>,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
) {
    tokio::spawn(async move {
        let Some(handle) = net.read().await.clone() else {
            let _ = out_tx
                .send(Ok(error_frame(
                    id,
                    ErrorCode::Unimplemented,
                    "connect requires the native p2p stack",
                )))
                .await;
            return;
        };

        // A bare `/p2p/<id>` has no transport, which is precisely what makes
        // the handle resolve it through the DHT rather than dial an address.
        let reply = match handle.connect_peer(&format!("/p2p/{}", req.peer_id)).await {
            Ok(_) => ConnectReply {
                connected: true,
                error: String::new(),
            },
            // Already connected is success: the caller wanted a connection to
            // this peer and there is one.
            Err(kwaai_p2p::P2PError::AlreadyConnected) => ConnectReply {
                connected: true,
                error: String::new(),
            },
            Err(e) => ConnectReply {
                connected: false,
                error: e.to_string(),
            },
        };

        let _ = out_tx
            .send(Ok(ServerFrame {
                id,
                body: Some(server_frame::Body::Connect(reply)),
            }))
            .await;
        let _ = out_tx.send(Ok(done_frame(id))).await;
    });
}

/// A bootstrap counts as reachable while it is connected or accepted a
/// connection within this window. Recency rather than the live set because
/// bootstraps close idle connections (~30 s), and the announce loop
/// re-contacts every bootstrap every ~300 s ± 30 s — so a healthy idle node
/// refreshes well inside the window, while one that cannot reach any
/// bootstrap ages out after missing two-plus cycles. The routing table is
/// deliberately not consulted: kad seeds it with the configured addresses
/// before any dial succeeds, so membership proves nothing.
const BOOTSTRAP_CONTACT_WINDOW: std::time::Duration = std::time::Duration::from_secs(900);

/// `(bootstrap_total, bootstrap_reachable)` for the configured bootstrap
/// set, as defined on `StatusReply` in kwaai.proto — the one computation
/// behind both the status frame and the network feed, so the two surfaces
/// can never disagree.
fn bootstrap_health(
    snapshot: &kwaai_p2p::NetworkSnapshot,
    bootstraps: &std::collections::HashSet<libp2p::PeerId>,
) -> (u32, u32) {
    use std::collections::HashSet;
    // Both checks are load-bearing: `last_contact` is stamped at establish
    // time, so a connection older than the window that is still open would
    // read as gone without the live-set check.
    let connected: HashSet<_> = snapshot.peers.iter().map(|p| p.peer_id).collect();
    let recent: HashSet<_> = snapshot
        .last_contact
        .iter()
        .filter(|(_, ago)| *ago <= BOOTSTRAP_CONTACT_WINDOW)
        .map(|(p, _)| *p)
        .collect();
    let reachable = bootstraps
        .iter()
        .filter(|b| connected.contains(b) || recent.contains(b))
        .count();
    (bootstraps.len() as u32, reachable as u32)
}

/// Project a [`kwaai_p2p::NetworkSnapshot`] onto the wire type.
///
/// Classification (relay-vs-direct, bootstrap, trusted-relay) and the sort
/// order come from [`crate::peers_view`], shared with `kwaainet p2p peers
/// list` so both surfaces describe the same network the same way.
fn build_network_update(
    snapshot: &kwaai_p2p::NetworkSnapshot,
    bootstraps: &std::collections::HashSet<libp2p::PeerId>,
    trusted_relays: &std::collections::HashSet<libp2p::PeerId>,
    reason: UpdateReason,
) -> NetworkUpdate {
    use crate::peers_view::{classify_addr, dht_role, group_index, ConnKind, DhtRole};
    use kwaai_rpc::v1::DhtRole as WireDhtRole;

    let connected_ids: std::collections::HashSet<libp2p::PeerId> =
        snapshot.peers.iter().map(|p| p.peer_id).collect();

    let mut connected: Vec<(u8, String, ConnectedPeer)> = snapshot
        .peers
        .iter()
        .map(|p| {
            let kind = classify_addr(&p.addr);
            let is_bootstrap = bootstraps.contains(&p.peer_id);
            let is_trusted_relay = trusted_relays.contains(&p.peer_id);
            let peer_id = p.peer_id.to_base58();

            let wire = ConnectedPeer {
                peer_id: peer_id.clone(),
                addr: p.addr.to_string(),
                kind: match kind {
                    ConnKind::Direct => PeerConnKind::Direct as i32,
                    ConnKind::Relay => PeerConnKind::Relay as i32,
                },
                direction: p.direction.as_str().to_string(),
                is_bootstrap,
                is_trusted_relay,
                protocols: p.protocols.clone(),
                // Saturating rather than wrapping: an absurd RTT should show as
                // "very large", never as a small number.
                rtt_ms: p
                    .rtt
                    .map(|d| d.as_millis().min(u32::MAX as u128) as u32)
                    .unwrap_or(0),
                agent_version: p.agent_version.clone().unwrap_or_default(),
                via: p.via.as_ref().map(|a| a.to_string()).unwrap_or_default(),
                dcutr: p.dcutr,
                dht_role: match dht_role(&p.protocols) {
                    DhtRole::Server => WireDhtRole::Server as i32,
                    DhtRole::Client => WireDhtRole::Client as i32,
                    DhtRole::Unknown => WireDhtRole::Unknown as i32,
                },
            };
            (
                group_index(is_bootstrap, is_trusted_relay, kind),
                peer_id,
                wire,
            )
        })
        .collect();

    // Stable across polls: group, then peer id. An unstable order makes a live
    // view flicker as rows swap under the user.
    connected.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    let mut routing: Vec<RoutingPeer> = snapshot
        .routing
        .iter()
        .map(|p| RoutingPeer {
            peer_id: p.peer_id.to_base58(),
            connected: connected_ids.contains(&p.peer_id),
            is_bootstrap: bootstraps.contains(&p.peer_id),
            addrs: p.addrs.iter().map(|a| a.to_string()).collect(),
        })
        .collect();
    routing.sort_by(|a, b| a.peer_id.cmp(&b.peer_id));

    // Spelled out rather than derived from Debug: these strings are wire
    // contract (documented in kwaai.proto and matched by the GUI), so they must
    // not follow a Rust variant rename.
    let (reachability, reachability_source) = match &snapshot.reachability {
        Reachability::Unknown => ("unknown", ""),
        Reachability::Public { source, .. } => (
            "public",
            match source {
                Source::IdentifyConsensus => "identify",
                Source::Upnp => "upnp",
                Source::AutoNat => "autonat",
                Source::Declared => "declared",
            },
        ),
        Reachability::Private => ("private", ""),
    };

    let (bootstrap_total, bootstrap_reachable) = bootstrap_health(snapshot, bootstraps);

    NetworkUpdate {
        server_time: now_rfc3339(),
        reason: reason as i32,
        self_status: Some(SelfStatus {
            peer_id: snapshot.local_peer_id.to_base58(),
            reachability: reachability.to_string(),
            reachability_source: reachability_source.to_string(),
            using_relay: !snapshot.relay_addrs.is_empty(),
            // Mirrors the announce loop's own gate: a node that does not know
            // where it stands should not be telling the network it is direct.
            announceable: !matches!(snapshot.reachability, Reachability::Unknown),
            listen_addrs: snapshot
                .listen_addrs
                .iter()
                .map(|a| a.to_string())
                .collect(),
            observed_addrs: snapshot
                .observed_addrs
                .iter()
                .map(|(a, _)| a.to_string())
                .collect(),
            relay_addrs: snapshot.relay_addrs.iter().map(|a| a.to_string()).collect(),
            local_protocols: snapshot.local_protocols.clone(),
        }),
        connected: connected.into_iter().map(|(_, _, w)| w).collect(),
        routing,
        bootstrap_total,
        bootstrap_reachable,
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

/// Resolve `<kwaainet dir>/run/kwaai.sock`. `kwaainet_dir()` rather than
/// `dirs::home_dir()`, so the socket follows `KWAAINET_HOME` like the rest of
/// `run/` — two daemons sharing one path meant the second one's bind
/// *unlinked* the first one's live socket.
#[cfg(unix)]
fn unix_socket_path() -> PathBuf {
    crate::config::kwaainet_dir().join(UNIX_SOCKET_RELPATH)
}

/// Resolve `<kwaainet dir>/run/kwaainet.grpc`.
pub fn grpc_port_file() -> PathBuf {
    crate::config::kwaainet_dir().join(GRPC_PORT_RELPATH)
}

/// Resolve the TCP port: explicit request, then [`GRPC_PORT_ENV`], then the
/// default. The flag says the port was *asked for* rather than defaulted,
/// which is what makes a bind failure fatal instead of a warning.
fn resolve_tcp_port(requested: Option<u16>) -> (u16, bool) {
    if let Some(p) = requested {
        return (p, true);
    }
    match std::env::var(GRPC_PORT_ENV)
        .ok()
        .and_then(|s| s.trim().parse::<u16>().ok())
    {
        Some(p) => (p, true),
        None => (DEFAULT_GRPC_TCP_PORT, false),
    }
}

/// Spawn the gRPC server task(s) and return a handle that, when dropped,
/// signals graceful shutdown.
///
/// `requested_port` is the `--grpc-port` flag; absent, [`GRPC_PORT_ENV`] then
/// [`DEFAULT_GRPC_TCP_PORT`] apply. Port 0 binds an ephemeral port and reports
/// the real one in [`grpc_port_file`].
pub fn spawn(config: KwaaiNetConfig, requested_port: Option<u16>) -> Result<GrpcServerHandle> {
    let (tcp_port, explicit) = resolve_tcp_port(requested_port);
    spawn_bound(config, tcp_port, explicit)
}

/// Bind on an explicit port. Tests take an ephemeral one rather than the
/// well-known port, which a running `kwaainet run-node` holds for the life of
/// the machine — the listener they assert has closed would be the daemon's.
///
/// Panics on a bind failure: a test that cannot have the port it picked has
/// nothing left to assert.
#[cfg(test)]
pub(crate) fn spawn_on_tcp_port(config: KwaaiNetConfig, tcp_port: u16) -> GrpcServerHandle {
    spawn_bound(config, tcp_port, true).expect("test gRPC server binds")
}

/// As [`spawn`], but with the port already resolved.
///
/// `explicit` decides what a failed TCP bind means. Defaulting to the
/// well-known port and losing it is survivable — the node still serves p2p, and
/// that leniency predates this argument. But when a supervisor *asked* for a
/// port, coming up without it strands it: a live pid, a written status file,
/// and nothing listening. That case exits instead, so the caller can retry.
fn spawn_bound(config: KwaaiNetConfig, tcp_port: u16, explicit: bool) -> Result<GrpcServerHandle> {
    let (shutdown_tcp_tx, shutdown_tcp_rx) = oneshot::channel::<()>();
    #[cfg(unix)]
    let (shutdown_unix_tx, shutdown_unix_rx) = oneshot::channel::<()>();

    let svc_state = KwaaiNetService::new(config);
    let net_slot = svc_state.net.clone();
    let service = KwaaiNetServer::new(svc_state);

    // Bind up front rather than inside the serve task: a clash has to be
    // visible here, and it is what resolves port 0 to a real number.
    let listener = match std::net::TcpListener::bind(("127.0.0.1", tcp_port)) {
        Ok(l) => Some(l),
        Err(e) if explicit => {
            return Err(anyhow::anyhow!("binding gRPC TCP port {tcp_port}: {e}"));
        }
        Err(e) => {
            warn!("gRPC: could not bind TCP port {tcp_port} ({e}) — TCP disabled");
            None
        }
    };

    let mut bound_port = None;
    let mut shutdown_tcp = None;
    if let Some(listener) = listener {
        let addr = listener.local_addr().context("gRPC TCP local_addr")?;
        listener
            .set_nonblocking(true)
            .context("gRPC TCP set_nonblocking")?;
        let listener =
            tokio::net::TcpListener::from_std(listener).context("adopting gRPC TCP listener")?;
        bound_port = Some(addr.port());
        shutdown_tcp = Some(shutdown_tcp_tx);

        let tcp_service = service.clone();
        tokio::spawn(async move {
            info!("gRPC: binding TCP at {addr}");
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
    }

    // Written only once the listener exists, so the file's presence means
    // "connectable" — the backlog holds dials until tonic starts accepting.
    let port_file = bound_port.and_then(|p| write_port_file(p).ok());

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
        Ok(GrpcServerHandle {
            shutdown_tcp,
            #[cfg(unix)]
            shutdown_unix: Some(shutdown_unix_tx),
            net: net_slot,
            port_file,
        })
    }
    #[cfg(not(unix))]
    {
        drop(service); // suppress unused warning on non-unix
        Ok(GrpcServerHandle {
            shutdown_tcp,
            net: net_slot,
            port_file,
        })
    }
}

/// Record the bound port under `run/`, returning the path so shutdown can
/// clear it. Best-effort: a daemon that cannot write here still serves.
fn write_port_file(port: u16) -> Result<PathBuf> {
    let path = grpc_port_file();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    std::fs::write(&path, format!("{port}\n"))
        .with_context(|| format!("writing {}", path.display()))?;
    Ok(path)
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
    /// The service's swarm-handle slot, shared so the node can fill it once
    /// p2p is up. See [`GrpcServerHandle::attach_network`].
    net: Arc<RwLock<Option<NetworkHandle>>>,
    /// `run/kwaainet.grpc`, when we managed to write it. Removed on shutdown.
    port_file: Option<PathBuf>,
}

impl GrpcServerHandle {
    /// Hand the running swarm to the gRPC service.
    ///
    /// Until this is called the Network op reports that it has nothing to
    /// serve. Only the native p2p path calls it; on the Go daemon path the
    /// slot stays empty for the process's lifetime, which is what makes
    /// "unsupported" distinguishable from "still starting".
    pub async fn attach_network(&self, handle: NetworkHandle) {
        *self.net.write().await = Some(handle);
    }

    /// Trigger a graceful shutdown of both transports. Safe to call multiple
    /// times; subsequent calls are no-ops.
    pub fn shutdown(&mut self) {
        // Both files mean "the listener is up", so both have to go. The serve
        // tasks clean up after themselves too, but a SIGTERM'd process exits
        // before they observe the signal — and a stale socket file makes a
        // client's exists() check say yes to a daemon that has gone.
        if let Some(path) = self.port_file.take() {
            let _ = std::fs::remove_file(path);
        }
        #[cfg(unix)]
        {
            let _ = std::fs::remove_file(unix_socket_path());
        }
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
        /// `TEST_LOCK` above serialises these tests against each other only.
        /// `KWAAINET_HOME` is process-global, so the guard also holds the
        /// crate-wide lock the tests in other modules take.
        _home_env: std::sync::MutexGuard<'static, ()>,
    }

    impl EnvGuard {
        fn set(dir: &std::path::Path) -> Self {
            let _home_env = crate::config::HOME_ENV_LOCK
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            let prev_home = std::env::var_os("HOME");
            let prev_kwaainet_home = std::env::var_os("KWAAINET_HOME");
            std::env::set_var("HOME", dir);
            std::env::set_var("KWAAINET_HOME", dir);
            Self {
                prev_home,
                prev_kwaainet_home,
                _home_env,
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

    /// `KWAAINET_HOME` must move the gRPC endpoint files, not just
    /// pid/lock/status. When it did not, a second daemon's bind unlinked the
    /// first one's socket.
    #[test]
    fn run_artifacts_follow_kwaainet_home() {
        let _serial = TEST_LOCK.blocking_lock();
        let tmp = tempfile::tempdir().expect("create tempdir for fake HOME");
        let _env = EnvGuard::set(tmp.path());

        // The socket is POSIX-only; the port file is how Windows finds the
        // daemon, so that half of the assertion has to run everywhere.
        #[cfg(unix)]
        assert_eq!(unix_socket_path(), tmp.path().join(UNIX_SOCKET_RELPATH));
        assert_eq!(grpc_port_file(), tmp.path().join(GRPC_PORT_RELPATH));
    }

    /// Precedence: explicit flag, then the env var, then the default. The
    /// bool is what makes a failed bind fatal, so it is asserted too.
    #[test]
    fn tcp_port_resolves_flag_then_env_then_default() {
        let _serial = TEST_LOCK.blocking_lock();
        let prev = std::env::var_os(GRPC_PORT_ENV);
        let restore = || match prev.clone() {
            Some(v) => std::env::set_var(GRPC_PORT_ENV, v),
            None => std::env::remove_var(GRPC_PORT_ENV),
        };

        std::env::remove_var(GRPC_PORT_ENV);
        assert_eq!(resolve_tcp_port(None), (DEFAULT_GRPC_TCP_PORT, false));
        assert_eq!(resolve_tcp_port(Some(9101)), (9101, true));

        std::env::set_var(GRPC_PORT_ENV, "9102");
        assert_eq!(resolve_tcp_port(None), (9102, true));
        // A flag still outranks the env var.
        assert_eq!(resolve_tcp_port(Some(9103)), (9103, true));

        // Garbage falls back to the default rather than failing to start.
        std::env::set_var(GRPC_PORT_ENV, "not-a-port");
        assert_eq!(resolve_tcp_port(None), (DEFAULT_GRPC_TCP_PORT, false));

        restore();
    }

    /// A port we asked for and could not get must abort, so the supervisor
    /// can retry — the old behaviour left a live pid with nothing listening.
    /// The same clash on the *default* port stays survivable.
    #[tokio::test]
    async fn explicit_port_clash_is_fatal_and_default_clash_is_not() {
        let _serial = TEST_LOCK.lock().await;
        let tmp = tempfile::tempdir().expect("create tempdir for fake HOME");
        let _env = EnvGuard::set(tmp.path());

        let squatter = std::net::TcpListener::bind("127.0.0.1:0").expect("bind squatter");
        let taken = squatter.local_addr().expect("squatter addr").port();

        assert!(
            spawn_bound(KwaaiNetConfig::default(), taken, true).is_err(),
            "explicit port clash must abort"
        );

        let lenient = spawn_bound(KwaaiNetConfig::default(), taken, false)
            .expect("default-port clash must still come up");
        // No listener means no port file to mislead a client into dialling.
        assert!(!grpc_port_file().exists());
        drop(lenient);
    }

    /// The port file is the contract that lets a restarted GUI find a live
    /// daemon: written once bound, gone once shut down.
    #[tokio::test]
    async fn port_file_records_the_bound_port_and_clears_on_shutdown() {
        let _serial = TEST_LOCK.lock().await;
        let tmp = tempfile::tempdir().expect("create tempdir for fake HOME");
        let _env = EnvGuard::set(tmp.path());

        // Port 0 exercises the ephemeral path the GUI's retry falls back on.
        let handle = spawn_bound(KwaaiNetConfig::default(), 0, true).expect("ephemeral bind");

        let recorded = std::fs::read_to_string(grpc_port_file()).expect("port file written");
        let port: u16 = recorded.trim().parse().expect("port file holds a number");
        assert_ne!(
            port, 0,
            "the resolved port must be reported, not the request"
        );
        assert!(
            std::net::TcpStream::connect(("127.0.0.1", port)).is_ok(),
            "the recorded port must actually be listening"
        );

        drop(handle);
        assert!(!grpc_port_file().exists(), "port file removed on shutdown");
    }

    /// Ask the OS for a free loopback port.
    ///
    /// The listener is dropped before the value is returned, so there is a
    /// small window where something else could claim it. That is still far
    /// safer than the well-known port, which a running `kwaainet run-node`
    /// holds for the life of the machine.
    fn ephemeral_port() -> u16 {
        std::net::TcpListener::bind("127.0.0.1:0")
            .expect("bind ephemeral loopback port")
            .local_addr()
            .expect("read ephemeral local addr")
            .port()
    }

    /// True iff a fresh TCP connect to `port` succeeds.
    async fn tcp_accepting(port: u16) -> bool {
        tokio::net::TcpStream::connect(("127.0.0.1", port))
            .await
            .is_ok()
    }

    /// Whether a fresh listener can be bound to `port` — which is the
    /// question these tests actually ask, and unlike "is the port refusing
    /// connections?" it has an immediate, unambiguous answer on every
    /// platform.
    ///
    /// The connect-probe this replaces had to tell "closed" from "slow to
    /// answer" by waiting, and on Windows a refused loopback connect is only
    /// reported after an in-stack SYN retry — ~2.04s on one box, ~2.30s on a
    /// slower one, scaling with the hardware. Every version of that test was a
    /// guess at a threshold, and two of them shipped wrong in the direction
    /// that cannot pass at all.
    ///
    /// `bind` needs no threshold. Measured on Windows 10 22H2, and matching
    /// how the server binds (`std::net::TcpListener::bind`, no socket options
    /// set — std sets `SO_REUSEADDR` on Unix only):
    ///
    /// - against a live listener it fails in under a millisecond with
    ///   `WSAEADDRINUSE`; on Unix `SO_REUSEADDR` permits `TIME_WAIT`, never an
    ///   active listener;
    /// - after a connection closed *server-side*, leaving `TIME_WAIT` on this
    ///   very port, it succeeds in ~0.3ms with the entry still in `netstat`.
    ///   That was the failure mode worth fearing, and it does not occur.
    ///
    /// The one way this differs from the old probe: it answers "is the port
    /// bindable", not "did *our* listener close". Something else taking the
    /// port in between would report a leak that isn't there. That is a
    /// spurious *failure*, never a silent pass — a leaked listener still holds
    /// the port and still fails the assertion — which is the direction to err
    /// in for a test guarding a shutdown regression.
    fn port_is_free(port: u16) -> bool {
        // Bound and dropped inside the call: holding it would be indis-
        // tinguishable from the leak we are testing for.
        std::net::TcpListener::bind(("127.0.0.1", port)).is_ok()
    }

    /// How long to wait for an asynchronous shutdown to release the port.
    ///
    /// An ordinary liveness budget, not a threshold anything is inferred
    /// from: each probe is decisive on its own, so this only bounds how long
    /// a genuine leak takes to be reported. `drop` -> oneshot ->
    /// `serve_with_incoming_shutdown` returns -> listener closed is normally
    /// well under a millisecond.
    const LISTENER_CLOSE_BUDGET: Duration = Duration::from_secs(10);

    /// Poll until `port` can be bound again, i.e. the listener is gone.
    async fn wait_for_close(port: u16) -> Result<(), ()> {
        let deadline = Instant::now() + LISTENER_CLOSE_BUDGET;
        loop {
            if port_is_free(port) {
                return Ok(());
            }
            if Instant::now() >= deadline {
                return Err(());
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
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
        let port = ephemeral_port();
        let handle = spawn_on_tcp_port(config, port);

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

        wait_for_close(port).await.unwrap_or_else(|()| {
            panic!(
                "127.0.0.1:{port} still not bindable {LISTENER_CLOSE_BUDGET:?} after dropping the handle"
            )
        });

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

        let port = ephemeral_port();
        let handle = spawn_on_tcp_port(KwaaiNetConfig::default(), port);

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
        // Wait for the listener to actually go away before the next test
        // tries to bind the same port.
        wait_for_close(port).await.unwrap_or_else(|()| {
            panic!("port still not bindable {LISTENER_CLOSE_BUDGET:?} after handle drop")
        });
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
            dial_addrs: Vec::new(),
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
    fn net_peer(id: &str) -> ConnectedPeer {
        ConnectedPeer {
            peer_id: id.to_string(),
            addr: "/ip4/198.18.0.10/tcp/8000".into(),
            kind: PeerConnKind::Direct as i32,
            direction: "outbound".into(),
            is_bootstrap: false,
            is_trusted_relay: false,
            protocols: vec!["/ipfs/kad/1.0.0".into()],
            rtt_ms: 10,
            agent_version: "kwaainet/0.5.4".into(),
            via: String::new(),
            dcutr: false,
            // Consistent with the kad entry in `protocols` above.
            dht_role: kwaai_rpc::v1::DhtRole::Server as i32,
        }
    }

    fn net_update(time: &str, peers: Vec<ConnectedPeer>) -> NetworkUpdate {
        NetworkUpdate {
            server_time: time.to_string(),
            reason: UpdateReason::Tick as i32,
            self_status: Some(SelfStatus {
                peer_id: "12D3KooWSelf".into(),
                reachability: "public".into(),
                reachability_source: "autonat".into(),
                using_relay: false,
                announceable: true,
                listen_addrs: vec!["/ip4/0.0.0.0/tcp/4001".into()],
                observed_addrs: vec![],
                relay_addrs: vec![],
                local_protocols: vec!["/ipfs/kad/1.0.0".into()],
            }),
            connected: peers,
            routing: vec![],
            ..Default::default()
        }
    }

    /// The suppression contract. If any of these trip, a quiet node turns into
    /// a frame generator (or, worse, a changing one goes silent).
    #[test]
    fn network_identity_ignores_time_reason_and_rtt() {
        let a = net_update("2026-01-01T00:00:00Z", vec![net_peer("A"), net_peer("B")]);

        // server_time advances every tick by construction.
        let later = net_update("2026-01-01T00:00:05Z", vec![net_peer("A"), net_peer("B")]);
        assert_eq!(network_identity(&a), network_identity(&later));

        // `reason` says why we are sending, not what changed. If it counted,
        // a heartbeat would look like news.
        let mut different_reason = later.clone();
        different_reason.reason = UpdateReason::Heartbeat as i32;
        assert_eq!(network_identity(&a), network_identity(&different_reason));

        // RTT is the important one: it moves on every ping, for every peer.
        // Counting it would defeat suppression entirely.
        let mut jittery = net_peer("A");
        jittery.rtt_ms = 999;
        let rtt_moved = net_update("2026-01-01T00:00:05Z", vec![jittery, net_peer("B")]);
        assert_eq!(network_identity(&a), network_identity(&rtt_moved));

        // The swarm iterates a HashMap, so connection order is arbitrary.
        let reordered = net_update("2026-01-01T00:00:05Z", vec![net_peer("B"), net_peer("A")]);
        assert_eq!(network_identity(&a), network_identity(&reordered));
    }

    /// The converse: everything a client actually renders must count.
    #[test]
    fn network_identity_tracks_rendered_fields() {
        let a = net_update("2026-01-01T00:00:00Z", vec![net_peer("A")]);

        // A peer joining or leaving.
        let joined = net_update("2026-01-01T00:00:00Z", vec![net_peer("A"), net_peer("B")]);
        assert_ne!(network_identity(&a), network_identity(&joined));

        // A hole-punch upgrade: same peer, relayed path becomes direct.
        let mut upgraded = net_peer("A");
        upgraded.kind = PeerConnKind::Relay as i32;
        assert_ne!(
            network_identity(&a),
            network_identity(&net_update("2026-01-01T00:00:00Z", vec![upgraded]))
        );

        // Identify landing after the connection established.
        let mut identified = net_peer("A");
        identified.protocols = vec!["/ipfs/kad/1.0.0".into(), "/libp2p/dcutr".into()];
        assert_ne!(
            network_identity(&a),
            network_identity(&net_update("2026-01-01T00:00:00Z", vec![identified]))
        );

        // A NAT transition — the headline event this whole view exists for.
        let mut gone_private = a.clone();
        gone_private.self_status.as_mut().unwrap().reachability = "private".into();
        assert_ne!(network_identity(&a), network_identity(&gone_private));

        // Picking up a relay reservation.
        let mut relaying = a.clone();
        relaying.self_status.as_mut().unwrap().using_relay = true;
        assert_ne!(network_identity(&a), network_identity(&relaying));

        // A routing-table entry changing connected-state. The routing table
        // and the connected set move independently, so this must register.
        let mut routed = a.clone();
        routed.routing = vec![RoutingPeer {
            peer_id: "A".into(),
            connected: true,
            is_bootstrap: false,
            addrs: vec![],
        }];
        assert_ne!(network_identity(&a), network_identity(&routed));

        // An entry losing its last dialable address is a state change worth
        // pushing: the peer goes from "reachable, just not connected" to
        // "known but undialable", which is what the addrs field exists to
        // distinguish. Suppressing it would leave the view claiming the
        // former while the latter is true.
        let mut addressed = a.clone();
        addressed.routing = vec![RoutingPeer {
            peer_id: "A".into(),
            connected: false,
            is_bootstrap: false,
            addrs: vec!["/ip4/198.18.0.32/tcp/8080".into()],
        }];
        let mut stripped = addressed.clone();
        stripped.routing[0].addrs.clear();
        assert_ne!(network_identity(&addressed), network_identity(&stripped));

        // A DCUtR upgrade. The connection was already direct in `kind` terms
        // by the time dcutr reports, so without this field in the identity the
        // upgrade would be suppressed and never reach the client — despite
        // being one of the most interesting things that can happen here.
        let mut upgraded_peer = net_peer("A");
        upgraded_peer.dcutr = true;
        assert_ne!(
            network_identity(&a),
            network_identity(&net_update("2026-01-01T00:00:00Z", vec![upgraded_peer]))
        );

        // The relay an inbound connection arrives through. A peer moving from
        // one relay to another is a real change in how we reach it, and the
        // view shows it, so it must not be suppressed.
        let mut via_changed = net_peer("A");
        via_changed.via = "/ip4/1.2.3.4/tcp/1/p2p/QmRelay/p2p-circuit".into();
        assert_ne!(
            network_identity(&a),
            network_identity(&net_update("2026-01-01T00:00:00Z", vec![via_changed]))
        );

        // Registering a handler changes what this node serves, and the view
        // shows it, so it has to reach the client.
        let mut serving_more = a.clone();
        serving_more
            .self_status
            .as_mut()
            .unwrap()
            .local_protocols
            .push("/kwaai/inference/1.0.0".into());
        assert_ne!(network_identity(&a), network_identity(&serving_more));

        // A routing peer being recognised as a bootstrap.
        let mut bootstrap_routed = a.clone();
        bootstrap_routed.routing = vec![RoutingPeer {
            peer_id: "A".into(),
            connected: false,
            is_bootstrap: true,
            addrs: vec![],
        }];
        let mut plain_routed = a.clone();
        plain_routed.routing = vec![RoutingPeer {
            peer_id: "A".into(),
            connected: false,
            is_bootstrap: false,
            addrs: vec![],
        }];
        assert_ne!(
            network_identity(&bootstrap_routed),
            network_identity(&plain_routed)
        );
    }

    /// Address churn is not a reachability change. `observed_addrs` moves as
    /// peers come and go without the verdict changing, and waking the UI for it
    /// would make the page twitch for no reason.
    #[test]
    fn network_identity_ignores_observed_address_churn() {
        let a = net_update("2026-01-01T00:00:00Z", vec![net_peer("A")]);
        let mut churned = a.clone();
        churned.self_status.as_mut().unwrap().observed_addrs =
            vec!["/ip4/203.0.113.7/tcp/4001".into()];
        assert_eq!(network_identity(&a), network_identity(&churned));
    }
}
