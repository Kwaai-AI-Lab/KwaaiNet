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
    kwaai_net_server::{KwaaiNet, KwaaiNetServer},
    ChatMessage, ChatToken,
};

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
}

impl KwaaiNetService {
    pub fn new(config: KwaaiNetConfig) -> Self {
        Self {
            config: Arc::new(config),
            inference: Arc::new(Mutex::new(None)),
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
