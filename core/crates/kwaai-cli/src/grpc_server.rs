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
    client_frame, error::Code as ErrorCode, kwaai_net_server::{KwaaiNet, KwaaiNetServer},
    server_frame, Cancel, ChatMessage, ChatToken, ClientFrame, Done,
    Error as RpcError, GenerateRequest, PingReply, PingRequest, ServerFrame,
    ShardRunRequest, StatusReply, StatusRequest,
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
}

impl KwaaiNetService {
    pub fn new(config: KwaaiNetConfig) -> Self {
        Self {
            config: Arc::new(config),
            inference: Arc::new(Mutex::new(None)),
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
        let (out_tx, out_rx) =
            mpsc::channel::<Result<ServerFrame, Status>>(128);

        // Per-id cancellation registry. ClientFrame::Cancel { target_id }
        // looks up the oneshot here and fires it.
        let cancels: Arc<Mutex<HashMap<u64, oneshot::Sender<()>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        // Clone the bits each per-frame task captures.
        let cfg = self.config.clone();
        let inference_slot = self.inference.clone();
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
                        let reply = StatusReply {
                            server_time: now_rfc3339(),
                            model: cfg.model.clone(),
                            shard_ready: shard_ready_path_exists(),
                            peer_count: 0, // TODO: thread through DHT routing-table size
                            uptime_secs: started_at.elapsed().as_secs(),
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
                        // TODO(shard): wire the distributed-inference
                        // dispatcher (shard_cmd / block_rpc) into here.
                        // Until that lands, surface a clear unimpl so
                        // callers know to fall back to local Generate.
                        let _ = out_tx
                            .send(Ok(error_frame(
                                id,
                                ErrorCode::Unimplemented,
                                &format!(
                                    "shard_run not yet wired; requested model={}",
                                    req.model.unwrap_or_else(|| cfg.model.clone())
                                ),
                            )))
                            .await;
                    }

                    client_frame::Body::Cancel(Cancel { target_id }) => {
                        let removed =
                            cancels.lock().await.remove(&target_id);
                        if let Some(tx) = removed {
                            let _ = tx.send(());
                            // Acknowledge the cancel frame itself with Done.
                            let _ = out_tx.send(Ok(done_frame(id))).await;
                        } else {
                            let _ = out_tx
                                .send(Ok(error_frame(
                                    id,
                                    ErrorCode::NotFound,
                                    &format!(
                                        "no in-flight operation with id {target_id}"
                                    ),
                                )))
                                .await;
                        }
                    }
                }
            }
        });

        Ok(Response::new(
            tokio_stream::wrappers::ReceiverStream::new(out_rx),
        ))
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
            .ok_or_else(|| Status::invalid_argument("client closed Chat stream before sending a prompt"))?;

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

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    /// Liveness probe. Returns the current daemon wall-clock time.
    /// Deliberately trivial — no inference, no DHT, no locks taken.
    async fn ping(
        &self,
        _request: Request<PingRequest>,
    ) -> Result<Response<PingReply>, Status> {
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
            let _ = out_tx
                .send(Ok(error_frame(
                    id,
                    ErrorCode::Internal,
                    &format!("inference init failed: {e}"),
                )))
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
    let (tok_tx, mut tok_rx) =
        mpsc::channel::<Result<ChatToken, Status>>(64);
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
                            let _ = out_tx
                                .send(Ok(error_frame(
                                    id,
                                    ErrorCode::Internal,
                                    &status.message().to_string(),
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
                let _ = tx.blocking_send(Err(Status::internal(format!(
                    "inference failed: {e}"
                ))));
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
    let (shutdown_tcp_tx, shutdown_tcp_rx) = oneshot::channel::<()>();
    #[cfg(unix)]
    let (shutdown_unix_tx, shutdown_unix_rx) = oneshot::channel::<()>();

    let svc_state = KwaaiNetService::new(config);
    let service = KwaaiNetServer::new(svc_state);

    // TCP: every platform.
    let tcp_addr: std::net::SocketAddr =
        format!("127.0.0.1:{DEFAULT_GRPC_TCP_PORT}").parse().expect("valid loopback addr");
    let tcp_service = service.clone();
    tokio::spawn(async move {
        info!("gRPC: binding TCP at {tcp_addr}");
        let serve = Server::builder()
            .add_service(tcp_service)
            .serve_with_shutdown(tcp_addr, async {
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
        }
    }
    #[cfg(not(unix))]
    {
        drop(service); // suppress unused warning on non-unix
        GrpcServerHandle {
            shutdown_tcp: Some(shutdown_tcp_tx),
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
}

impl GrpcServerHandle {
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
    use std::sync::Mutex;
    use std::time::{Duration, Instant};

    /// All tests in this module mutate process-wide env (`HOME`,
    /// `KWAAINET_HOME`) AND bind the same hardcoded loopback port
    /// (`DEFAULT_GRPC_TCP_PORT`). Cargo runs tests in a single binary on
    /// multiple threads by default; this mutex forces our tests onto a
    /// single-file conga line so they don't trample each other's env or
    /// race for the port.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

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

    /// True iff a fresh TCP connect to the gRPC loopback port succeeds.
    async fn tcp_accepting() -> bool {
        tokio::net::TcpStream::connect(("127.0.0.1", DEFAULT_GRPC_TCP_PORT))
            .await
            .is_ok()
    }

    /// True iff a fresh TCP connect to the gRPC loopback port is refused
    /// quickly (used to assert the listener is gone after shutdown).
    async fn tcp_refused() -> bool {
        // ConnectionRefused is the happy-path answer; any other Err (e.g.
        // network unreachable) we also treat as "not accepting". We bound
        // the dial with a short timeout so a slow stack can't lie to us.
        match tokio::time::timeout(
            Duration::from_millis(250),
            tokio::net::TcpStream::connect(("127.0.0.1", DEFAULT_GRPC_TCP_PORT)),
        )
        .await
        {
            Ok(Ok(_)) => false,        // still accepting
            Ok(Err(_)) => true,        // refused / unreachable
            Err(_) => false,           // timed out = something is listening but not answering yet
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
        let _serial = TEST_LOCK.lock().expect("test lock not poisoned");

        let tmp = tempfile::tempdir().expect("create tempdir for fake HOME");
        let _env = EnvGuard::set(tmp.path());

        let config = KwaaiNetConfig::default();
        let handle = spawn(config);

        // The server task is spawned on tokio; give it up to ~2 s to wire
        // up the TCP listener. In practice this happens in <50 ms locally.
        let up = wait_for(Duration::from_secs(2), tcp_accepting).await;
        assert!(
            up,
            "gRPC TCP listener never came up on 127.0.0.1:{}",
            DEFAULT_GRPC_TCP_PORT
        );

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
                mode, 0o600,
                "Unix socket {} must be mode 0o600 (got {:#o})",
                sock_path.display(),
                mode
            );
        }

        // Drop triggers GrpcServerHandle::shutdown -> oneshot send ->
        // tonic's serve_with_shutdown returns -> listener is closed.
        drop(handle);

        let down = wait_for(Duration::from_secs(2), tcp_refused).await;
        assert!(
            down,
            "TCP listener on 127.0.0.1:{} did not close within 2s of dropping the handle",
            DEFAULT_GRPC_TCP_PORT
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

        let _serial = TEST_LOCK.lock().expect("test lock not poisoned");

        let tmp = tempfile::tempdir().expect("create tempdir for fake HOME");
        let _env = EnvGuard::set(tmp.path());

        let handle = spawn(KwaaiNetConfig::default());

        // Make sure the server is accepting before we dial.
        let up = wait_for(Duration::from_secs(2), tcp_accepting).await;
        assert!(up, "gRPC TCP listener never came up");

        let endpoint = format!("http://127.0.0.1:{DEFAULT_GRPC_TCP_PORT}");
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
        let down = wait_for(Duration::from_secs(2), tcp_refused).await;
        assert!(down, "TCP listener did not close after handle drop");
    }
}
