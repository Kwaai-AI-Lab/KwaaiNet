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

use kwaai_hivemind_dht::DHTStorage;
use kwaai_p2p::{
    reachability::{Reachability, Source},
    NetworkHandle,
};
use kwaai_rpc::v1::{
    client_frame,
    error::Code as ErrorCode,
    kwaai_net_server::{KwaaiNet, KwaaiNetServer},
    server_frame, BlockCoverageRequest, BlockCoverageUpdate, BlockPeer, Cancel, ChatMessage,
    ChatToken, ClientFrame, ConnectReply, ConnectRequest, ConnectedPeer, DhtRecordsReply,
    DhtRecordsRequest, Done, Error as RpcError, GenerateRequest, HopFailure, InferenceEvent,
    InferenceHop, InferencePhase, NetworkRequest, NetworkUpdate, PeerConnKind, PingReply,
    PingRequest, RoutingPeer, SelfStatus, ServerFrame, ShardRunRequest, StatusReply,
    StorageDiscoveryRequest, StoragePeer, StorageReachability, StorageUpdate, UpdateReason,
};
#[cfg(feature = "rag")]
use kwaai_rpc::v1::{
    rag_progress::RagPhase, RagChunk, RagDeleteReply, RagDeleteRequest, RagDoc, RagIngestReply,
    RagIngestRequest, RagInitReply, RagInitRequest, RagKb, RagProgress, RagQueryReply,
    RagQueryRequest, RagStatusUpdate,
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
    /// therefore means the node has not started yet.
    net: Arc<RwLock<Option<NetworkHandle>>>,
    /// This node's own hivemind DHT store, filled in alongside `net`. Late-bound
    /// for the same reason, and empty means the same two things.
    dht: Arc<RwLock<Option<DHTStorage>>>,
    /// Captured at service construction so StatusReply.uptime_secs can
    /// report a process-level uptime without a separate clock.
    started_at: Instant,
    #[cfg(feature = "rag")]
    rag: RagShared,
}

/// RAG state shared by every session of the service.
#[cfg(feature = "rag")]
#[derive(Clone)]
struct RagShared {
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
    lock: Arc<Mutex<()>>,
    /// Orders `rag_init`'s config write against `rag_status`'s read, so
    /// status never parses a half-written file yet never waits on `lock`.
    config_lock: Arc<Mutex<()>>,
    /// Opened local vector stores, cached per data dir for the daemon's
    /// lifetime.
    ///
    /// `StorageDb::open` replays every stored vector into an in-memory
    /// HNSW graph — O(corpus) work that would otherwise run on every
    /// RAG op. The daemon is long-lived, so open once and reuse, exactly
    /// as `rag serve` does. All users hold `lock`, so the map itself
    /// sees no contention. Inherited trade-off from `rag serve`: vectors
    /// written by another process (e.g. a CLI ingest) aren't visible
    /// until the daemon restarts.
    stores: RagStores,
    /// Caps uploads in flight across all sessions; each spools to disk.
    upload_slots: Arc<tokio::sync::Semaphore>,
}

impl KwaaiNetService {
    pub fn new(config: KwaaiNetConfig) -> Self {
        Self {
            config: Arc::new(config),
            inference: Arc::new(Mutex::new(None)),
            net: Arc::new(RwLock::new(None)),
            dht: Arc::new(RwLock::new(None)),
            started_at: Instant::now(),
            #[cfg(feature = "rag")]
            rag: RagShared {
                lock: Arc::new(Mutex::new(())),
                config_lock: Arc::new(Mutex::new(())),
                stores: Arc::new(Mutex::new(HashMap::new())),
                upload_slots: Arc::new(tokio::sync::Semaphore::new(RAG_MAX_UPLOADS)),
            },
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
        let dht_slot = self.dht.clone();
        let started_at = self.started_at;
        #[cfg(feature = "rag")]
        let rag = self.rag.clone();
        #[cfg(feature = "rag")]
        let rag_uploads: RagUploads = Arc::new(Mutex::new(HashMap::new()));

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
                        // swarm is not up yet and there is no handle to ask.
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
                            out_tx.clone(),
                            cancels.clone(),
                        )
                        .await;
                    }

                    client_frame::Body::Connect(req) => {
                        spawn_session_connect(id, req, net_slot.clone(), out_tx.clone()).await;
                    }

                    client_frame::Body::DhtRecords(req) => {
                        spawn_session_dht_records(
                            id,
                            req,
                            dht_slot.clone(),
                            cfg.clone(),
                            out_tx.clone(),
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
                    // stall the dispatch loop, and takes its locks inside
                    // that task (see `RagShared`). Upload pieces never wait.
                    #[cfg(feature = "rag")]
                    client_frame::Body::RagStatus(_) => {
                        spawn_session_rag_status(id, out_tx.clone(), rag.config_lock.clone());
                    }

                    #[cfg(feature = "rag")]
                    client_frame::Body::RagInit(req) => {
                        spawn_session_rag_init(id, req, out_tx.clone(), rag.clone());
                    }

                    #[cfg(feature = "rag")]
                    client_frame::Body::RagIngest(req) => {
                        spawn_session_rag_ingest(
                            id,
                            req,
                            out_tx.clone(),
                            cancels.clone(),
                            rag.clone(),
                            rag_uploads.clone(),
                        )
                        .await;
                    }

                    #[cfg(feature = "rag")]
                    client_frame::Body::RagIngestData(piece) => {
                        route_rag_piece(&rag_uploads, id, piece.data).await;
                    }

                    #[cfg(feature = "rag")]
                    client_frame::Body::RagQuery(req) => {
                        spawn_session_rag_query(id, req, out_tx.clone(), rag.clone());
                    }

                    #[cfg(feature = "rag")]
                    client_frame::Body::RagDelete(req) => {
                        spawn_session_rag_delete(id, req, out_tx.clone(), rag.clone());
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

                    // Its ingest was already answered UNIMPLEMENTED.
                    #[cfg(not(feature = "rag"))]
                    client_frame::Body::RagIngestData(_) => {}
                }
            }

            // Uploads still waiting on pieces can never get them now.
            #[cfg(feature = "rag")]
            end_session_uploads(&rag_uploads, &cancels).await;
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

/// Map a daemon-side [`InferenceProgress`] onto the wire type.
///
/// `shard_cmd` deliberately keeps its own proto-free mirror so it stays
/// usable from the CLI without the rpc crate, which leaves exactly one place
/// — here — that has to know both shapes.
fn progress_to_proto(p: crate::shard_cmd::InferenceProgress) -> InferenceEvent {
    use crate::shard_cmd::{HopFailureKind, ProgressPhase};

    let phase = match p.phase {
        ProgressPhase::Resolved => InferencePhase::Resolved,
        ProgressPhase::DiscoveryStart => InferencePhase::DiscoveryStart,
        ProgressPhase::DiscoveryResult => InferencePhase::DiscoveryResult,
        ProgressPhase::CircuitLoaded => InferencePhase::CircuitLoaded,
        ProgressPhase::ChainPinned => InferencePhase::ChainPinned,
        ProgressPhase::PeerDialStart => InferencePhase::PeerDialStart,
        ProgressPhase::PeerDial => InferencePhase::PeerDial,
        ProgressPhase::HopStart => InferencePhase::HopStart,
        ProgressPhase::HopOk => InferencePhase::HopOk,
        ProgressPhase::HopFailed => InferencePhase::HopFailed,
        ProgressPhase::PathRebuild => InferencePhase::PathRebuild,
        ProgressPhase::TokenSampled => InferencePhase::TokenSampled,
        ProgressPhase::Complete => InferencePhase::Complete,
    };

    let failure = match p.failure {
        None => HopFailure::Unspecified,
        Some(HopFailureKind::NoHandler) => HopFailure::NoHandler,
        Some(HopFailureKind::Transient) => HopFailure::Transient,
        Some(HopFailureKind::Timeout) => HopFailure::Timeout,
        Some(HopFailureKind::Other) => HopFailure::Other,
    };

    InferenceEvent {
        elapsed_ms: p.elapsed_ms,
        phase: phase as i32,
        message: p.message,
        peer_id: p.peer_id.unwrap_or_default(),
        peer_name: p.peer_name.unwrap_or_default(),
        is_self: p.is_self,
        block_start: p.block_start.map(|v| v as u32),
        block_end: p.block_end.map(|v| v as u32),
        total_blocks: p.total_blocks.map(|v| v as u32),
        covered_blocks: p.covered_blocks.map(|v| v as u32),
        duration_ms: p.duration_ms,
        token_index: p.token_index.map(|v| v as u32),
        is_prefill: p.is_prefill,
        candidate_index: p.candidate_index.map(|v| v as u32),
        attempt: p.attempt.map(|v| v as u32),
        ok: p.ok,
        session_id: p.session_id.unwrap_or_default(),
        seq_pos: p.seq_pos,
        failure: failure as i32,
        model: p.model.unwrap_or_default(),
        dht_prefix: p.dht_prefix.unwrap_or_default(),
        peer_count: p.peer_count.map(|v| v as u32),
        circuit_id: p.circuit_id.unwrap_or_default(),
        hops: p
            .hops
            .into_iter()
            .map(|h| InferenceHop {
                peer_id: h.peer_id,
                peer_name: h.peer_name,
                block_start: h.block_start as u32,
                block_end: h.block_end as u32,
                is_self: h.is_self,
                trust_score: h.trust_score,
                throughput: h.throughput,
            })
            .collect(),
        dropped_events: p
            .dropped_events
            .map(|v| u32::try_from(v).unwrap_or(u32::MAX)),
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
// Blocking: MetaStore / StorageDb opens, vector-store calls, text extraction
// and the BM25 build are sync and CPU/IO-heavy, so they run under
// `spawn_blocking`. Two kinds stay on the reactor: config loads (one small
// file) and the metadata/BM25 writes `ingest_text` makes inline.

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

/// Largest document `rag_ingest` accepts.
#[cfg(feature = "rag")]
const RAG_INGEST_MAX_BYTES: u64 = 256 * 1024 * 1024;

/// An upload that sends nothing, or reads no acknowledgement, for this long
/// is abandoned.
#[cfg(feature = "rag")]
const RAG_UPLOAD_IDLE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);

/// The upload window: pieces a client may have unacknowledged. The server
/// buffers exactly this many, so a client that overruns it fails the ingest.
#[cfg(feature = "rag")]
const RAG_UPLOAD_BUFFER: usize = 8;

/// Uploads a daemon runs at once, across sessions; each spools to disk.
#[cfg(feature = "rag")]
const RAG_MAX_UPLOADS: usize = 4;

/// Name prefix of upload spool files in the temp dir.
#[cfg(feature = "rag")]
const RAG_SPOOL_PREFIX: &str = "kwaai-rag-ingest-";

/// A spool file older than this belongs to no live upload.
#[cfg(feature = "rag")]
const RAG_SPOOL_STALE_AFTER: std::time::Duration = std::time::Duration::from_secs(24 * 60 * 60);

/// One in-flight upload, as the session loop sees it.
#[cfg(feature = "rag")]
struct RagUpload {
    tx: mpsc::Sender<Vec<u8>>,
    /// Set when a piece found the buffer full. Also identifies the upload.
    overrun: Arc<std::sync::atomic::AtomicBool>,
}

/// In-flight uploads by ingest op id; `rag_ingest_data` frames are routed here.
#[cfg(feature = "rag")]
type RagUploads = Arc<Mutex<HashMap<u64, RagUpload>>>;

/// The ingest's end of one upload.
#[cfg(feature = "rag")]
struct UploadRx {
    pieces: mpsc::Receiver<Vec<u8>>,
    overrun: Arc<std::sync::atomic::AtomicBool>,
    /// Held until the spool file is deleted.
    slot: tokio::sync::OwnedSemaphorePermit,
}

/// Vectors an ingest stored before committing their metadata.
#[cfg(feature = "rag")]
#[derive(Default)]
struct IngestResidue {
    store: Option<(Arc<kwaai_storage::VectorStore>, uuid::Uuid, PathBuf)>,
    ids: Vec<i64>,
}

#[cfg(feature = "rag")]
type Residue = Arc<std::sync::Mutex<IngestResidue>>;

#[cfg(feature = "rag")]
fn residue_lock(r: &Residue) -> std::sync::MutexGuard<'_, IngestResidue> {
    r.lock().unwrap_or_else(|e| e.into_inner())
}

/// The service's per-data-dir cache of opened local vector stores.
/// See [`RagShared::stores`] for semantics.
#[cfg(feature = "rag")]
type RagStores = Arc<Mutex<HashMap<PathBuf, Arc<kwaai_storage::VectorStore>>>>;

/// Fetch (or open and cache) the local vector store for `data_dir`.
///
/// The open replays every stored vector into the in-memory HNSW graph,
/// so it runs off the reactor and only on first use. Callers hold
/// the RAG lock, which is what makes the get-then-insert sequence safe.
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
    // Caller-supplied arguments we rejected before touching any store,
    // including an upload that broke its own declared size or stalled.
    if msg.contains("rag_ingest upload")
        || msg.contains("rag_init cannot switch")
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

/// Delete upload spool files left by an earlier daemon: on Windows an
/// abandoned extraction thread keeps its file open, so the drop-time delete fails.
#[cfg(feature = "rag")]
fn sweep_stale_spools(dir: &std::path::Path, older_than: std::time::Duration) -> usize {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    let now = std::time::SystemTime::now();
    entries
        .flatten()
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with(RAG_SPOOL_PREFIX)
        })
        .filter(|e| {
            e.metadata()
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| now.duration_since(t).ok())
                .is_some_and(|age| age > older_than)
        })
        .filter(|e| std::fs::remove_file(e.path()).is_ok())
        .count()
}

/// `rag_status` — snapshot every configured KB.
///
/// Best-effort per KB: one KB whose MetaStore won't open (wiped data dir,
/// schema from an older build) is reported with an empty doc list instead
/// of failing the status of every other KB.
///
/// Takes only the config lock, not the RAG lock: it only reads, and an
/// ingest holds the RAG lock for minutes. SQLite (WAL) serves its reads
/// alongside the ingest's writes.
#[cfg(feature = "rag")]
fn spawn_session_rag_status(
    id: u64,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    config_lock: Arc<Mutex<()>>,
) {
    tokio::spawn(async move {
        let _guard = config_lock.lock().await;

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
            remote: rag_is_remote(rag),
            documents,
        });
    }
    Ok(out)
}

/// `rag_init` — create or idempotently refresh a knowledge base.
#[cfg(feature = "rag")]
fn spawn_session_rag_init(
    id: u64,
    req: RagInitRequest,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    rag: RagShared,
) {
    tokio::spawn(async move {
        let _guard = rag.lock.lock().await;
        let _config = rag.config_lock.lock().await;

        let kb = rag_kb_or_default(&req.kb);
        let requested = req.embed_model.trim().to_string();
        let kb_for_args = kb.clone();
        let args = tokio::task::spawn_blocking(move || rag_init_args(&kb_for_args, &requested))
            .await
            .context("rag init task panicked")
            .and_then(|r| r);
        let (embed_model, rag_dir) = match args {
            Ok(args) => args,
            Err(e) => return send_rag_error(id, &e, &out_tx).await,
        };

        // init_kb is async (it awaits the Ollama probe and the tenant
        // create), so it is driven directly rather than via spawn_blocking;
        // the sync store work inside it is short.
        match crate::rag_cmd::init_kb(&kb, embed_model, rag_dir, |_| {}).await {
            Ok(outcome) => {
                // A tenant init created is absent from a store cached before it.
                rag.stores.lock().await.remove(&outcome.data_dir);

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

/// `init_kb`'s model and data dir for `kb`. An existing KB keeps its data
/// dir, and its model unless the request names another and it holds no
/// documents, whose vectors the new model could not be compared against.
#[cfg(feature = "rag")]
fn rag_init_args(kb: &str, requested: &str) -> Result<(String, Option<PathBuf>)> {
    use kwaai_rag::meta_store::MetaStore;

    let cfg = KwaaiNetConfig::load_or_create()?;
    let Some(existing) = cfg.get_rag_kb(kb) else {
        let model = if requested.is_empty() {
            crate::config::RagConfig::default().embed_model
        } else {
            requested.to_string()
        };
        return Ok((model, None));
    };
    // init_kb would replace a remote KB with a fresh local one.
    reject_if_remote(kb, existing)?;
    let data_dir = existing.data_dir();
    if requested.is_empty() || requested == existing.embed_model {
        return Ok((existing.embed_model.clone(), Some(data_dir)));
    }
    let holds_docs = existing
        .tenant_id
        .as_deref()
        .and_then(|t| t.parse::<uuid::Uuid>().ok())
        .and_then(|tid| MetaStore::open(&data_dir, tid).ok())
        .and_then(|ms| ms.list_docs().ok())
        .is_some_and(|docs| !docs.is_empty());
    if holds_docs {
        anyhow::bail!(
            "rag_init cannot switch KB '{kb}' from {} to {requested}: its documents were \
             embedded with {}. Delete them first, or init a new KB",
            existing.embed_model,
            existing.embed_model
        );
    }
    Ok((requested.to_string(), Some(data_dir)))
}

/// Build one [`RagKb`] from persisted config. Sync; run on the blocking pool.
#[cfg(feature = "rag")]
fn single_rag_kb(kb: &str) -> Result<RagKb> {
    collect_rag_kbs()?
        .into_iter()
        .find(|k| k.name == kb)
        .with_context(|| format!("KB '{kb}' missing from config immediately after init"))
}

/// Route a piece to its upload without waiting: a full buffer means the
/// client overran its window, which ends the upload. Pieces for no upload,
/// or for one already complete, are dropped.
#[cfg(feature = "rag")]
async fn route_rag_piece(uploads: &RagUploads, id: u64, data: Vec<u8>) {
    let mut uploads = uploads.lock().await;
    let Some(up) = uploads.get(&id) else {
        return;
    };
    if let Err(mpsc::error::TrySendError::Full(_)) = up.tx.try_send(data) {
        up.overrun.store(true, Ordering::SeqCst);
        // Closing the channel wakes an ingest waiting on a piece that was dropped.
        uploads.remove(&id);
    }
}

/// Cancel a closed session's uploads that are still waiting on pieces.
/// Ingests already past their upload run to completion.
#[cfg(feature = "rag")]
async fn end_session_uploads(
    uploads: &RagUploads,
    cancels: &Arc<Mutex<HashMap<u64, oneshot::Sender<()>>>>,
) {
    let waiting: Vec<u64> = uploads
        .lock()
        .await
        .iter()
        .filter(|(_, up)| !up.tx.is_closed())
        .map(|(id, _)| *id)
        .collect();
    let mut cancels = cancels.lock().await;
    for id in waiting {
        if let Some(tx) = cancels.remove(&id) {
            let _ = tx.send(());
        }
    }
}

/// `rag_ingest` — receive one document over the session, then chunk, embed
/// and store it.
#[cfg(feature = "rag")]
async fn spawn_session_rag_ingest(
    id: u64,
    req: RagIngestRequest,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    cancels: Arc<Mutex<HashMap<u64, oneshot::Sender<()>>>>,
    rag: RagShared,
    rag_uploads: RagUploads,
) {
    // A reused id would cross-wire this op's pieces and cancel with another's.
    let in_flight =
        rag_uploads.lock().await.contains_key(&id) || cancels.lock().await.contains_key(&id);
    if in_flight {
        let msg = format!("rag_ingest id {id} is already in flight");
        let _ = out_tx
            .send(Ok(error_frame(id, ErrorCode::InvalidArgument, &msg)))
            .await;
        return;
    }
    let Ok(slot) = rag.upload_slots.clone().try_acquire_owned() else {
        let msg = format!("{RAG_MAX_UPLOADS} uploads already in flight; retry when one ends");
        let _ = out_tx
            .send(Ok(error_frame(id, ErrorCode::Unavailable, &msg)))
            .await;
        return;
    };

    // Register the cancel channel BEFORE spawning, so a Cancel frame that
    // arrives immediately after this one can still find it. Registering
    // inside the task would race the dispatch loop.
    let (cancel_tx, cancel_rx) = oneshot::channel::<()>();
    cancels.lock().await.insert(id, cancel_tx);
    // Likewise the upload, or the first data frame could arrive before it.
    let (data_tx, pieces) = mpsc::channel::<Vec<u8>>(RAG_UPLOAD_BUFFER);
    let overrun = Arc::new(std::sync::atomic::AtomicBool::new(false));
    rag_uploads.lock().await.insert(
        id,
        RagUpload {
            tx: data_tx,
            overrun: overrun.clone(),
        },
    );

    tokio::spawn(async move {
        let residue = Residue::default();
        let upload = UploadRx {
            pieces,
            overrun: overrun.clone(),
            slot,
        };
        let ingest = run_rag_ingest(
            id,
            req,
            upload,
            out_tx.clone(),
            rag.clone(),
            residue.clone(),
        );
        let result = tokio::select! {
            biased;
            _ = cancel_rx => None,
            r = ingest => Some(r),
        };

        // Free the id before the terminal frame tells the client it may reuse
        // it. Only this op's entries go: its cancel receiver is gone, so a
        // sender still open is another op's.
        {
            let mut cancels = cancels.lock().await;
            if cancels.get(&id).is_some_and(|tx| tx.is_closed()) {
                cancels.remove(&id);
            }
        }
        {
            let mut uploads = rag_uploads.lock().await;
            if uploads
                .get(&id)
                .is_some_and(|up| Arc::ptr_eq(&up.overrun, &overrun))
            {
                uploads.remove(&id);
            }
        }

        match result {
            None => {
                let _ = out_tx
                    .send(Ok(error_frame(
                        id,
                        ErrorCode::Cancelled,
                        "ingest cancelled by client",
                    )))
                    .await;
            }
            Some(Ok(reply)) => {
                let _ = out_tx
                    .send(Ok(ServerFrame {
                        id,
                        body: Some(server_frame::Body::RagIngest(reply)),
                    }))
                    .await;
                let _ = out_tx.send(Ok(done_frame(id))).await;
            }
            Some(Err(e)) => send_rag_error(id, &e, &out_tx).await,
        }

        discard_unindexed_vectors(&residue, &rag.lock).await;
    });
}

/// Best-effort removal of vectors a failed or cancelled ingest stored
/// without their metadata, which leaves them unreachable even by
/// `rag_delete`. Ids the metadata store knows (a re-ingest) are kept.
#[cfg(feature = "rag")]
async fn discard_unindexed_vectors(residue: &Residue, lock: &Arc<Mutex<()>>) {
    let Some((vs, tenant_id, data_dir)) = residue_lock(residue).store.take() else {
        return;
    };
    // Before reading the ids: a cancelled ingest's last upload may still be
    // running, and it holds the lock until it has recorded them.
    let _guard = lock.lock().await;
    let ids = std::mem::take(&mut residue_lock(residue).ids);
    if ids.is_empty() {
        return;
    }
    let removed = tokio::task::spawn_blocking(move || -> Result<usize> {
        let meta = kwaai_rag::meta_store::MetaStore::open(&data_dir, tenant_id)?;
        let orphans: Vec<i64> = ids
            .iter()
            .zip(meta.get_chunks(&ids)?)
            .filter(|(_, known)| known.is_none())
            .map(|(id, _)| *id)
            .collect();
        futures::executor::block_on(vs.delete(tenant_id, &orphans))
    })
    .await;
    match removed {
        Ok(Ok(n)) => info!(vectors = n, "discarded the vectors of an unfinished ingest"),
        Ok(Err(e)) => warn!(
            error = format!("{e:#}"),
            "could not discard the vectors of an unfinished ingest"
        ),
        Err(e) => warn!("vector discard task panicked: {e}"),
    }
}

/// The ingest pipeline proper. Emits progress frames as it goes and
/// returns the terminal counts; the caller frames the outcome.
#[cfg(feature = "rag")]
async fn run_rag_ingest(
    id: u64,
    req: RagIngestRequest,
    upload: UploadRx,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    rag: RagShared,
    residue: Residue,
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

    let doc_name = req.doc_name.trim().to_string();
    if doc_name.is_empty() {
        anyhow::bail!("rag_ingest upload requires a document name: its extension picks the parser");
    }
    if req.size == 0 {
        anyhow::bail!("rag_ingest upload is empty: size must be above 0");
    }
    if req.size > RAG_INGEST_MAX_BYTES {
        anyhow::bail!(
            "rag_ingest upload of {} bytes exceeds the {}-byte limit",
            req.size,
            RAG_INGEST_MAX_BYTES
        );
    }

    // Received before taking the RAG lock, so another ingest's pipeline
    // cannot hold this upload's pieces up in the session loop.
    let UploadRx {
        pieces,
        overrun,
        slot,
    } = upload;
    let spool = receive_upload(id, &doc_name, req.size, pieces, &overrun, &out_tx).await?;
    // Shared with each vector upload, so one this op abandons on cancel
    // still lands under the lock.
    let guard = Arc::new(rag.lock.clone().lock_owned().await);

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

    let extract_path = spool.path().to_path_buf();
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
    // The spool file is only for the parsers; delete it now, not after embedding.
    drop(spool);
    drop(slot);

    let data_dir = rag_cfg.data_dir();
    let meta_dir = data_dir.clone();
    let meta = tokio::task::spawn_blocking(move || MetaStore::open(&meta_dir, tenant_id))
        .await
        .context("metadata store task panicked")??;

    let vs = cached_local_vs(&rag.stores, &data_dir).await?;
    residue_lock(&residue).store = Some((vs.clone(), tenant_id, data_dir.clone()));

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
        .map_err(|e| {
            warn!(
                error = format!("{e:#}"),
                "BM25 index unavailable; ingest will not update it"
            )
        })
        .ok()
        .map(Arc::new)
    };

    let embed = EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
    let mut cfg = IngestConfig::new(embed);
    cfg.bm25 = bm25;

    // `ingest_text`'s progress callback is sync and called from inside the
    // ingest future, so it can't await. Hand counts to a forwarder over an
    // unbounded channel and throttle there. The forwarder is joined with the
    // ingest, not spawned, so no progress frame can outlive this op.
    let (prog_tx, mut prog_rx) = mpsc::unbounded_channel::<(usize, usize)>();
    let progress_doc = doc_name.clone();
    let forward = async {
        let mut last_sent = std::time::Instant::now() - RAG_PROGRESS_INTERVAL;
        let mut pending: Option<(usize, usize)> = None;

        while let Some((done, total)) = prog_rx.recv().await {
            pending = Some((done, total));
            if last_sent.elapsed() >= RAG_PROGRESS_INTERVAL {
                let _ = out_tx
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
            let _ = out_tx
                .send(Ok(rag_progress_frame(
                    id,
                    RagPhase::Embedding,
                    done as u32,
                    total as u32,
                    &progress_doc,
                )))
                .await;
        }
    };

    let upload_vs = vs.clone();
    let upload_residue = residue.clone();
    let ingest = ingest_text(
        &cfg,
        &meta,
        &doc_name,
        &text,
        move |vectors| {
            let vs = upload_vs.clone();
            let residue = upload_residue.clone();
            let held = guard.clone();
            Box::pin(async move {
                tokio::task::spawn_blocking(move || {
                    let _held = held;
                    let n = futures::executor::block_on(vs.upload(tenant_id, &vectors))?;
                    residue_lock(&residue)
                        .ids
                        .extend(vectors.iter().map(|(id, _)| *id));
                    Ok(n)
                })
                .await
                .context("vector upload task panicked")?
            }) as Pin<Box<dyn std::future::Future<Output = Result<usize>> + Send>>
        },
        // Dropped when the ingest future completes, which ends the forwarder.
        Some(move |done: usize, total: usize| {
            let _ = prog_tx.send((done, total));
        }),
    );
    let (result, ()) = tokio::join!(ingest, forward);
    let result = result?;
    // Metadata committed: the vectors are no longer residue.
    residue_lock(&residue).store = None;

    Ok(RagIngestReply {
        chunks_ingested: result.chunks_ingested as u32,
        vectors_uploaded: result.vectors_uploaded as u32,
    })
}

/// Spool an upload to a private temp file carrying the document's extension,
/// which is what the parsers dispatch on. Only buffered pieces are in memory.
#[cfg(feature = "rag")]
async fn receive_upload(
    id: u64,
    doc_name: &str,
    size: u64,
    mut pieces: mpsc::Receiver<Vec<u8>>,
    overrun: &std::sync::atomic::AtomicBool,
    out_tx: &mpsc::Sender<Result<ServerFrame, Status>>,
) -> Result<tempfile::NamedTempFile> {
    use tokio::io::AsyncWriteExt as _;
    use tokio::sync::mpsc::error::SendTimeoutError;

    let suffix = std::path::Path::new(doc_name)
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let spool = tempfile::Builder::new()
        .prefix(RAG_SPOOL_PREFIX)
        .suffix(&suffix)
        .tempfile()
        .context("creating the upload spool file")?;
    let mut file =
        tokio::fs::File::from_std(spool.reopen().context("opening the upload spool file")?);

    let mut received: u64 = 0;
    while received < size {
        let next = tokio::time::timeout(RAG_UPLOAD_IDLE_TIMEOUT, pieces.recv()).await;
        if overrun.load(Ordering::SeqCst) {
            anyhow::bail!("rag_ingest upload exceeded the {RAG_UPLOAD_BUFFER}-piece window");
        }
        let piece = match next {
            Ok(Some(piece)) => piece,
            Ok(None) => anyhow::bail!("rag_ingest upload ended after {received} of {size} bytes"),
            Err(_) => anyhow::bail!(
                "rag_ingest upload stalled: nothing for {}s after {received} of {size} bytes",
                RAG_UPLOAD_IDLE_TIMEOUT.as_secs()
            ),
        };
        received += piece.len() as u64;
        if received > size {
            anyhow::bail!("rag_ingest upload sent more than the declared {size} bytes");
        }
        file.write_all(&piece)
            .await
            .context("writing the upload spool file")?;

        // Every piece is acknowledged, unthrottled: each ack reopens one slot
        // of the client's window. Bounded, so an unread session can't wedge us.
        let ack = rag_progress_frame(
            id,
            RagPhase::Receiving,
            received as u32,
            size as u32,
            doc_name,
        );
        match out_tx.send_timeout(Ok(ack), RAG_UPLOAD_IDLE_TIMEOUT).await {
            Ok(()) => {}
            Err(SendTimeoutError::Timeout(_)) => anyhow::bail!(
                "rag_ingest upload stalled: acknowledgements unread for {}s",
                RAG_UPLOAD_IDLE_TIMEOUT.as_secs()
            ),
            Err(SendTimeoutError::Closed(_)) => {
                anyhow::bail!("rag_ingest upload abandoned: the session closed")
            }
        }
    }
    file.flush()
        .await
        .context("writing the upload spool file")?;
    Ok(spool)
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
fn spawn_session_rag_query(
    id: u64,
    req: RagQueryRequest,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    rag: RagShared,
) {
    tokio::spawn(async move {
        let _guard = rag.lock.lock().await;

        match run_rag_query(req, rag.stores.clone()).await {
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
        .map_err(|e| {
            warn!(
                error = format!("{e:#}"),
                "BM25 index unavailable; using in-RAM fallback"
            )
        })
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
    // indexing every chunk. The HNSW search is sync, so it runs off the reactor.
    let chunks = retrieve_hybrid(&req.text, &cfg, &embed, &meta, move |emb, k| {
        let vs = vs.clone();
        Box::pin(async move {
            let raw = tokio::task::spawn_blocking(move || {
                futures::executor::block_on(vs.search(tenant_id, &emb, k))
            })
            .await
            .context("vector search task panicked")??;
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
fn spawn_session_rag_delete(
    id: u64,
    req: RagDeleteRequest,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
    rag: RagShared,
) {
    tokio::spawn(async move {
        let _guard = rag.lock.lock().await;

        match run_rag_delete(req, rag.stores.clone()).await {
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
    // this op holds the RAG lock, which stalls every other RAG request and
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
        // Opt-in per request: the client asks for routing telemetry only when
        // it has somewhere to show it.
        events: req.events,
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
                        Some(crate::shard_cmd::ShardRunEvent::Progress(p)) => {
                            let frame = ServerFrame {
                                id,
                                body: Some(server_frame::Body::InferenceEvent(
                                    progress_to_proto(*p),
                                )),
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
            // `last_contact_secs` is excluded for the same reason as `rtt_ms`:
            // it moves every tick. `distance` and `bucket` are fixed for the
            // life of an entry, so including them costs no wakeups and keeps
            // the identity a complete description of what a client renders.
            format!(
                "{}|{}|{}|{}|{}|{}|{}|{}|{}",
                r.peer_id,
                r.connected,
                r.is_bootstrap,
                addrs.join(","),
                r.kad_status,
                r.bucket,
                r.bucket_entries,
                r.bucket_pending,
                r.distance,
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
                // The slot fills as the node starts, so retrying is right.
                let (code, msg) = (
                    ErrorCode::Unavailable,
                    "p2p node is still starting; retry shortly",
                );
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

/// Records returned when the request names no `limit`, and the ceiling any
/// `limit` is clamped to.
///
/// A reply has to fit one gRPC frame, and tonic's client default
/// `max_decoding_message_size` is 4 MiB. The store's own bound is a million
/// entries, whose metadata alone would be ~100 MB.
const DHT_RECORDS_DEFAULT_LIMIT: usize = 500;
const DHT_RECORDS_MAX_LIMIT: usize = 5_000;

/// Serve a `dht_records` op: enumerate this node's own hivemind DHT store.
///
/// One-shot: the store moves on the ~120 s announce cycle, so a client that
/// wants to watch it can poll.
async fn spawn_session_dht_records(
    id: u64,
    req: DhtRecordsRequest,
    dht: Arc<RwLock<Option<DHTStorage>>>,
    cfg: Arc<KwaaiNetConfig>,
    out_tx: mpsc::Sender<Result<ServerFrame, Status>>,
) {
    tokio::spawn(async move {
        let Some(storage) = dht.read().await.clone() else {
            let _ = out_tx
                .send(Ok(error_frame(
                    id,
                    ErrorCode::Unimplemented,
                    "dht_records requires the native p2p stack",
                )))
                .await;
            return;
        };

        // Values are up to MAX_VALUE_BYTES each, so a whole-store listing that
        // carried them could not fit a frame at any useful limit.
        if req.include_values && req.key_ids.is_empty() {
            let _ = out_tx
                .send(Ok(error_frame(
                    id,
                    ErrorCode::InvalidArgument,
                    "include_values needs key_ids: values are only served for named records",
                )))
                .await;
            return;
        }

        let known = crate::dht_records::known_keys(&cfg);
        let names: HashMap<&str, &str> = known
            .iter()
            .map(|k| (k.key_id.as_str(), k.name.as_str()))
            .collect();

        // Filtered on the hex id rather than the raw key so the client can echo
        // back exactly what it was shown.
        let wanted: BTreeSet<&str> = req.key_ids.iter().map(String::as_str).collect();
        let now = kwaai_hivemind_dht::value::get_dht_time();
        let limit = match req.limit as usize {
            0 => DHT_RECORDS_DEFAULT_LIMIT,
            n => n.min(DHT_RECORDS_MAX_LIMIT),
        };

        let matched: Vec<_> = storage
            .local_records()
            .into_iter()
            .filter(|record| {
                wanted.is_empty() || wanted.contains(hex::encode(&record.summary.key).as_str())
            })
            .collect();
        // Counted before truncation: a client comparing it against `records`
        // is how it learns the listing was cut short.
        let total_records = matched.len() as u32;

        let records = matched
            .into_iter()
            .take(limit)
            .map(|record| {
                let key_id = hex::encode(&record.summary.key);
                // A second lock acquisition per record, so it is taken only for
                // records that will carry a value — and from this row's own
                // tier, or a cache row would show the primary tier's value.
                let stored = req
                    .include_values
                    .then(|| storage.local_record(&record.summary.key, record.in_cache))
                    .flatten();
                crate::dht_records::local_record(
                    &record,
                    stored.as_ref().map(|(v, _)| v),
                    names.get(key_id.as_str()).copied().unwrap_or_default(),
                    now,
                )
            })
            .collect();

        let reply = DhtRecordsReply {
            server_time: now_rfc3339(),
            records,
            known_keys: known,
            total_records,
        };
        let _ = out_tx
            .send(Ok(ServerFrame {
                id,
                body: Some(server_frame::Body::DhtRecords(reply)),
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
    use kwaai_rpc::v1::{DhtRole as WireDhtRole, KadStatus as WireKadStatus};
    use libp2p::kad::NodeStatus;

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

    // Linear scans over `last_contact` would be quadratic in the routing table;
    // it is a small bounded cache, so one map serves the whole projection.
    let last_contact: std::collections::HashMap<_, _> = snapshot
        .last_contact
        .iter()
        .map(|(p, ago)| (*p, *ago))
        .collect();

    let mut routing: Vec<RoutingPeer> = snapshot
        .routing
        .iter()
        .map(|p| RoutingPeer {
            peer_id: p.peer_id.to_base58(),
            connected: connected_ids.contains(&p.peer_id),
            is_bootstrap: bootstraps.contains(&p.peer_id),
            addrs: p.addrs.iter().map(|a| a.to_string()).collect(),
            kad_status: match p.status {
                NodeStatus::Connected => WireKadStatus::Connected as i32,
                NodeStatus::Disconnected => WireKadStatus::Disconnected as i32,
            },
            bucket: p.bucket as u32,
            bucket_entries: p.bucket_entries as u32,
            bucket_capacity: libp2p::kad::K_VALUE.get() as u32,
            bucket_pending: p.bucket_pending,
            // Fixed-width so distances sort and compare as strings, which is
            // the only thing a client can usefully do with a 256-bit number.
            distance: format!("{:064x}", p.distance.0),
            last_contact_secs: last_contact.get(&p.peer_id).map(|ago| ago.as_secs() as u32),
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
            ipv6: snapshot.ipv6.as_str().to_string(),
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
/// `grpc_port` config key, then the default. The env var sits above the file
/// because it is a per-launch override (the GUI allocates a port per spawn).
/// The flag says the port was *asked for* rather than defaulted, which is
/// what makes a bind failure fatal instead of a warning.
fn resolve_tcp_port(requested: Option<u16>, configured: Option<u16>) -> (u16, bool) {
    if let Some(p) = requested {
        return (p, true);
    }
    let from_env = std::env::var(GRPC_PORT_ENV)
        .ok()
        .and_then(|s| s.trim().parse::<u16>().ok());
    match from_env.or(configured) {
        Some(p) => (p, true),
        None => (DEFAULT_GRPC_TCP_PORT, false),
    }
}

/// Spawn the gRPC server task(s) and return a handle that, when dropped,
/// signals graceful shutdown.
///
/// `requested_port` is the `--grpc-port` flag; absent, [`GRPC_PORT_ENV`], the
/// config's `grpc_port`, then [`DEFAULT_GRPC_TCP_PORT`] apply. Port 0 binds an
/// ephemeral port and reports the real one in [`grpc_port_file`].
pub fn spawn(config: KwaaiNetConfig, requested_port: Option<u16>) -> Result<GrpcServerHandle> {
    #[cfg(feature = "rag")]
    std::thread::spawn(|| {
        let swept = sweep_stale_spools(&std::env::temp_dir(), RAG_SPOOL_STALE_AFTER);
        if swept > 0 {
            info!(files = swept, "removed stale RAG upload spool files");
        }
    });
    let (tcp_port, explicit) = resolve_tcp_port(requested_port, config.grpc_port);
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
    let ipv6 = config.ipv6();
    let (shutdown_tcp_tx, shutdown_tcp_rx) = tokio::sync::broadcast::channel::<()>(1);
    #[cfg(unix)]
    let (shutdown_unix_tx, shutdown_unix_rx) = oneshot::channel::<()>();

    let svc_state = KwaaiNetService::new(config);
    let net_slot = svc_state.net.clone();
    let dht_slot = svc_state.dht.clone();
    let service = KwaaiNetServer::new(svc_state);

    // Bind up front rather than inside the serve task: a clash has to be
    // visible here, and it is what resolves port 0 to a real number.
    let bound = match crate::net::bind_dual_stack(crate::net::Scope::Loopback, tcp_port, ipv6) {
        Ok(b) => Some(b),
        // `ipv6: true` is a promise; v4 was free, so keeping quiet here would
        // be a daemon that runs half of it.
        Err(e @ crate::net::BindError::V6Required(_)) => {
            return Err(anyhow::anyhow!("binding gRPC TCP port {tcp_port}: {e}"));
        }
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
    if let Some(bound) = bound {
        let port = bound.port;
        // The v4 listener decides the reported port, and both are torn down by
        // the one shutdown signal — a `broadcast` rather than the `oneshot`
        // this had, since two serve tasks now wait on it.
        let listeners = bound.into_tokio().context("adopting gRPC TCP listeners")?;
        bound_port = Some(port);
        shutdown_tcp = Some(shutdown_tcp_tx);

        for listener in listeners {
            let addr = listener.local_addr().context("gRPC TCP local_addr")?;
            let tcp_service = service.clone();
            let mut shutdown = shutdown_tcp_rx.resubscribe();
            tokio::spawn(async move {
                info!("gRPC: binding TCP at {addr}");
                let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);
                let serve = Server::builder()
                    .add_service(tcp_service)
                    .serve_with_incoming_shutdown(incoming, async {
                        let _ = shutdown.recv().await;
                    });
                if let Err(e) = serve.await {
                    warn!("gRPC TCP server exited with error: {e}");
                }
            });
        }
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
            dht: dht_slot,
            port_file,
        })
    }
    #[cfg(not(unix))]
    {
        drop(service); // suppress unused warning on non-unix
        Ok(GrpcServerHandle {
            shutdown_tcp,
            net: net_slot,
            dht: dht_slot,
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
    shutdown_tcp: Option<tokio::sync::broadcast::Sender<()>>,
    #[cfg(unix)]
    shutdown_unix: Option<oneshot::Sender<()>>,
    /// The service's swarm-handle slot, shared so the node can fill it once
    /// p2p is up. See [`GrpcServerHandle::attach_network`].
    net: Arc<RwLock<Option<NetworkHandle>>>,
    /// The service's DHT-store slot; see [`GrpcServerHandle::attach_dht_storage`].
    dht: Arc<RwLock<Option<DHTStorage>>>,
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

    /// Hand this node's DHT store to the gRPC service, so the records op can
    /// enumerate it. Same lifecycle as [`Self::attach_network`].
    pub async fn attach_dht_storage(&self, storage: DHTStorage) {
        *self.dht.write().await = Some(storage);
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
    fn tcp_port_resolves_flag_then_env_then_config_then_default() {
        let _serial = TEST_LOCK.blocking_lock();
        let prev = std::env::var_os(GRPC_PORT_ENV);
        let restore = || match prev.clone() {
            Some(v) => std::env::set_var(GRPC_PORT_ENV, v),
            None => std::env::remove_var(GRPC_PORT_ENV),
        };

        std::env::remove_var(GRPC_PORT_ENV);
        assert_eq!(resolve_tcp_port(None, None), (DEFAULT_GRPC_TCP_PORT, false));
        assert_eq!(resolve_tcp_port(Some(9101), None), (9101, true));
        // The config key is a persisted request, so a clash on it is fatal too.
        assert_eq!(resolve_tcp_port(None, Some(9104)), (9104, true));

        std::env::set_var(GRPC_PORT_ENV, "9102");
        assert_eq!(resolve_tcp_port(None, None), (9102, true));
        // A flag still outranks the env var, and the env var outranks config.
        assert_eq!(resolve_tcp_port(Some(9103), None), (9103, true));
        assert_eq!(resolve_tcp_port(None, Some(9104)), (9102, true));

        // Garbage falls back to config, then the default, rather than failing to start.
        std::env::set_var(GRPC_PORT_ENV, "not-a-port");
        assert_eq!(resolve_tcp_port(None, Some(9104)), (9104, true));
        assert_eq!(resolve_tcp_port(None, None), (DEFAULT_GRPC_TCP_PORT, false));

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

        // The v6 twin is the point of the dual-stack bind: a client that
        // resolves `localhost` to `::1` first must reach the same daemon.
        // Skipped where the host has no v6 loopback to bind.
        if std::net::TcpListener::bind("[::1]:0").is_ok() && kwaai_p2p::IPV6_BUILD {
            let up_v6 = wait_for(Duration::from_secs(2), || async move {
                tokio::net::TcpStream::connect(format!("[::1]:{port}"))
                    .await
                    .is_ok()
            })
            .await;
            assert!(up_v6, "gRPC listener never came up on [::1]:{port}");
        }

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
                ipv6: "active".into(),
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
        }];
        let mut plain_routed = a.clone();
        plain_routed.routing = vec![RoutingPeer {
            peer_id: "A".into(),
            connected: false,
            is_bootstrap: false,
            addrs: vec![],
            ..Default::default()
        }];
        assert_ne!(
            network_identity(&bootstrap_routed),
            network_identity(&plain_routed)
        );
    }

    /// The k-bucket detail behind the DHT-entry dialog: kad's own verdict and
    /// the bucket's occupancy are things the client renders, so they have to
    /// wake it — but `last_contact_secs` grows every tick and must not, or the
    /// dialog's own detail fields would defeat suppression for the whole feed.
    #[test]
    fn network_identity_tracks_kad_detail_but_not_last_contact() {
        let base = |f: fn(&mut RoutingPeer)| {
            let mut u = net_update("2026-01-01T00:00:00Z", vec![net_peer("A")]);
            let mut r = RoutingPeer {
                peer_id: "A".into(),
                connected: true,
                kad_status: kwaai_rpc::v1::KadStatus::Connected as i32,
                bucket: 250,
                bucket_entries: 3,
                bucket_capacity: 20,
                ..Default::default()
            };
            f(&mut r);
            u.routing = vec![r];
            u
        };

        let a = base(|_| {});

        // Kad lags the swarm by an event-loop turn, so this flips on its own.
        assert_ne!(
            network_identity(&a),
            network_identity(&base(
                |r| r.kad_status = kwaai_rpc::v1::KadStatus::Disconnected as i32
            ))
        );

        // Whether the bucket is full decides if a newly-found peer can be
        // inserted at all — the dialog says so, so it must stay current.
        assert_ne!(
            network_identity(&a),
            network_identity(&base(|r| r.bucket_entries = 20))
        );
        assert_ne!(
            network_identity(&a),
            network_identity(&base(|r| r.bucket_pending = true))
        );

        // Elapsed time, on every entry, on every tick.
        assert_eq!(
            network_identity(&a),
            network_identity(&base(|r| r.last_contact_secs = Some(41)))
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

    /// A drop count past u32 saturates rather than wrapping to a small number.
    #[test]
    fn progress_to_proto_carries_dropped_events() {
        use crate::shard_cmd::{InferenceProgress, ProgressPhase};
        let ev = |n| {
            let mut p = InferenceProgress::new(std::time::Instant::now(), ProgressPhase::Complete);
            p.dropped_events = n;
            progress_to_proto(p).dropped_events
        };
        assert_eq!(ev(None), None);
        assert_eq!(ev(Some(7)), Some(7));
        assert_eq!(ev(Some(u64::MAX)), Some(u32::MAX));
    }

    /// Drive a real `shard_run` over a real Session and assert the routing
    /// telemetry reaches the wire.
    ///
    /// There is no p2p daemon here, so the run fails as soon as it tries to
    /// connect to one — which still exercises the whole path end to end:
    /// `run_streaming_inner` builds the event, the sink pushes it onto the
    /// run's channel, `progress_to_proto` maps it, and it leaves the socket
    /// as a real `ServerFrame`. Only `Resolved` precedes that failure, so
    /// that is what this can honestly assert; the phases that need peers
    /// are covered by the NAT topology, not by a unit test.
    #[tokio::test]
    async fn shard_run_streams_inference_events_when_requested() {
        use futures::StreamExt as _;
        use kwaai_rpc::v1::kwaai_net_client::KwaaiNetClient;
        use kwaai_rpc::v1::InferencePhase;
        use tokio_stream::wrappers::ReceiverStream;

        let _serial = TEST_LOCK.lock().await;

        let tmp = tempfile::tempdir().expect("create tempdir for fake HOME");
        let _env = EnvGuard::set(tmp.path());

        let port = ephemeral_port();
        let handle = spawn_on_tcp_port(KwaaiNetConfig::default(), port);
        let up = wait_for(Duration::from_secs(2), || tcp_accepting(port)).await;
        assert!(up, "gRPC TCP listener never came up on 127.0.0.1:{port}");

        let endpoint = format!("http://127.0.0.1:{port}");
        let channel = tonic::transport::Endpoint::from_shared(endpoint)
            .expect("valid endpoint")
            .connect()
            .await
            .expect("connect to loopback gRPC server");
        let mut client = KwaaiNetClient::new(channel);

        let (tx, rx) = tokio::sync::mpsc::channel::<ClientFrame>(4);
        tx.send(ClientFrame {
            id: 1,
            body: Some(client_frame::Body::ShardRun(ShardRunRequest {
                role: "user".to_string(),
                content: "hello".to_string(),
                model: None,
                conversation_id: None,
                events: true,
            })),
        })
        .await
        .expect("send shard_run frame");

        let mut inbound = client
            .session(ReceiverStream::new(rx))
            .await
            .expect("open Session")
            .into_inner();

        // Collect until the operation terminates, or we give up waiting.
        let mut phases = Vec::new();
        let collect = async {
            while let Some(Ok(frame)) = inbound.next().await {
                match frame.body {
                    Some(server_frame::Body::InferenceEvent(e)) => phases.push(e.phase),
                    Some(server_frame::Body::Done(_)) | Some(server_frame::Body::Error(_)) => break,
                    _ => {}
                }
            }
        };
        // Generous: the daemon polls the DHT before giving up on discovery.
        let _ = tokio::time::timeout(Duration::from_secs(45), collect).await;

        assert!(
            phases.contains(&(InferencePhase::Resolved as i32)),
            "expected a Resolved event on the wire, got {phases:?}"
        );

        // Close our end first: an open Session keeps the connection — and
        // so the listener — alive past the handle drop.
        drop(inbound);
        drop(tx);
        drop(client);
        drop(handle);
        wait_for_close(port).await.unwrap_or_else(|last| {
            panic!("TCP listener still {last:?} {LISTENER_CLOSE_BUDGET:?} after handle drop")
        });
    }

    /// The mirror of the above: with `events` unset the daemon must stay
    /// silent, since that is what keeps the flag worth having.
    #[tokio::test]
    async fn shard_run_stays_silent_without_the_events_flag() {
        use futures::StreamExt as _;
        use kwaai_rpc::v1::kwaai_net_client::KwaaiNetClient;
        use tokio_stream::wrappers::ReceiverStream;

        let _serial = TEST_LOCK.lock().await;

        let tmp = tempfile::tempdir().expect("create tempdir for fake HOME");
        let _env = EnvGuard::set(tmp.path());

        let port = ephemeral_port();
        let handle = spawn_on_tcp_port(KwaaiNetConfig::default(), port);
        let up = wait_for(Duration::from_secs(2), || tcp_accepting(port)).await;
        assert!(up, "gRPC TCP listener never came up on 127.0.0.1:{port}");

        let endpoint = format!("http://127.0.0.1:{port}");
        let channel = tonic::transport::Endpoint::from_shared(endpoint)
            .expect("valid endpoint")
            .connect()
            .await
            .expect("connect to loopback gRPC server");
        let mut client = KwaaiNetClient::new(channel);

        let (tx, rx) = tokio::sync::mpsc::channel::<ClientFrame>(4);
        tx.send(ClientFrame {
            id: 1,
            body: Some(client_frame::Body::ShardRun(ShardRunRequest {
                role: "user".to_string(),
                content: "hello".to_string(),
                model: None,
                conversation_id: None,
                events: false,
            })),
        })
        .await
        .expect("send shard_run frame");

        let mut inbound = client
            .session(ReceiverStream::new(rx))
            .await
            .expect("open Session")
            .into_inner();

        let mut saw_event = false;
        let collect = async {
            while let Some(Ok(frame)) = inbound.next().await {
                match frame.body {
                    Some(server_frame::Body::InferenceEvent(_)) => {
                        saw_event = true;
                        break;
                    }
                    Some(server_frame::Body::Done(_)) | Some(server_frame::Body::Error(_)) => break,
                    _ => {}
                }
            }
        };
        let _ = tokio::time::timeout(Duration::from_secs(45), collect).await;

        assert!(!saw_event, "events were emitted without being requested");

        drop(inbound);
        drop(tx);
        drop(client);
        drop(handle);
        wait_for_close(port).await.unwrap_or_else(|last| {
            panic!("TCP listener still {last:?} {LISTENER_CLOSE_BUDGET:?} after handle drop")
        });
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
            client_frame, kwaai_net_client::KwaaiNetClient, rag_progress::RagPhase, server_frame,
            ClientFrame, RagIngestData, RagIngestRequest, RagQueryRequest, RagStatusRequest,
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
            round_trip_frames(port, vec![body]).await
        }

        /// [`round_trip`] for an op that sends more than one frame, all
        /// under id 1 — an ingest followed by its data.
        async fn round_trip_frames(
            port: u16,
            bodies_out: Vec<client_frame::Body>,
        ) -> Vec<server_frame::Body> {
            let frames = bodies_out.into_iter().map(|b| (1, b)).collect();
            session_frames(port, frames, 1, 1, false)
                .await
                .into_iter()
                .map(|(_, b)| b)
                .collect()
        }

        /// Send `frames` on one session and collect every reply until op
        /// `id` has ended `terminals` times. With `close`, the client
        /// half-closes the session once everything is sent.
        async fn session_frames(
            port: u16,
            frames: Vec<(u64, client_frame::Body)>,
            id: u64,
            terminals: usize,
            close: bool,
        ) -> Vec<(u64, server_frame::Body)> {
            let endpoint = format!("http://127.0.0.1:{port}");
            let channel = tonic::transport::Endpoint::from_shared(endpoint)
                .expect("valid endpoint")
                .connect()
                .await
                .expect("connect to the test server");
            let mut client = KwaaiNetClient::new(channel);

            // Sized to hold every frame, so all are queued before the
            // session opens.
            let (tx, rx) = tokio::sync::mpsc::channel::<ClientFrame>(frames.len().max(1));
            for (frame_id, body) in frames {
                tx.send(ClientFrame {
                    id: frame_id,
                    body: Some(body),
                })
                .await
                .expect("send request frame");
            }
            // Otherwise keep the send half open: dropping it closes the
            // session before the server has replied.
            let _open = (!close).then_some(tx);

            let mut stream = client
                .session(ReceiverStream::new(rx))
                .await
                .expect("open Session")
                .into_inner();

            let mut bodies = Vec::new();
            let mut ended = 0;
            // Bounded so a hang fails loudly instead of blocking the suite.
            let deadline = tokio::time::Instant::now() + Duration::from_secs(20);
            while ended < terminals {
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
                    if terminal && frame.id == id {
                        ended += 1;
                    }
                    bodies.push((frame.id, b));
                }
            }
            bodies
        }

        /// The terminal frames op `id` ended with, in order.
        fn terminals_of(bodies: &[(u64, server_frame::Body)], id: u64) -> Vec<server_frame::Body> {
            bodies
                .iter()
                .filter(|(i, b)| {
                    *i == id
                        && matches!(
                            b,
                            server_frame::Body::Done(_) | server_frame::Body::Error(_)
                        )
                })
                .map(|(_, b)| b.clone())
                .collect()
        }

        /// Spawn a server on an OS-allocated port and return it with the
        /// port it landed on. Every RAG test uses this rather than
        /// `spawn`, so the suite never contends with (or accidentally
        /// talks to) a real daemon on the fixed port.
        async fn spawn_test_server() -> (GrpcServerHandle, u16) {
            let port = ephemeral_port();
            let handle = spawn_on_tcp_port(KwaaiNetConfig::default(), port);
            // The TCP bind happens on a spawned task, so the listener is not
            // up yet when `spawn_on_tcp_port` returns. `round_trip` connects
            // immediately, and would see ConnectionRefused without this.
            let up = wait_for(Duration::from_secs(2), || tcp_accepting(port)).await;
            assert!(up, "test gRPC server never came up on 127.0.0.1:{port}");
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

            let (handle, port) = spawn_test_server().await;
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

            let (handle, port) = spawn_test_server().await;
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

        /// Save a local KB named "default" so an op gets past the
        /// not-initialised gate. `embed_url` points embedding somewhere.
        fn save_local_kb(tmp: &std::path::Path, embed_url: Option<&str>) {
            let mut cfg = KwaaiNetConfig::load_or_create().expect("load config");
            cfg.set_rag_kb(
                "default",
                crate::config::RagConfig {
                    tenant_id: Some(uuid::Uuid::new_v4().to_string()),
                    storage_url: Some("local".to_string()),
                    rag_data_dir: Some(tmp.join("ragdata").to_string_lossy().into_owned()),
                    embed_url: embed_url.map(str::to_string),
                    ..crate::config::RagConfig::default()
                },
            );
            cfg.save().expect("save config");
        }

        fn ingest(doc_name: &str, size: u64) -> client_frame::Body {
            client_frame::Body::RagIngest(RagIngestRequest {
                kb: "default".into(),
                doc_name: doc_name.into(),
                size,
            })
        }

        fn piece(data: &[u8]) -> client_frame::Body {
            client_frame::Body::RagIngestData(RagIngestData {
                data: data.to_vec(),
            })
        }

        /// Spool files left in the temp dir, to check an ingest cleans up.
        fn spool_files() -> usize {
            std::fs::read_dir(std::env::temp_dir())
                .map(|d| {
                    d.flatten()
                        .filter(|e| {
                            e.file_name()
                                .to_string_lossy()
                                .starts_with("kwaai-rag-ingest-")
                        })
                        .count()
                })
                .unwrap_or(0)
        }

        /// The name picks the parser, so an ingest without one is refused
        /// before any bytes are read.
        #[tokio::test]
        async fn rag_ingest_without_doc_name_is_invalid_argument() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());
            save_local_kb(tmp.path(), None);

            let (handle, port) = spawn_test_server().await;
            let bodies = round_trip_frames(port, vec![ingest("  ", 5), piece(b"hello")]).await;

            let msg = expect_error(&bodies, ErrorCode::InvalidArgument);
            assert!(msg.contains("document name"), "got {msg:?}");
            drop(handle);
        }

        /// A declared size over the limit is refused before the client
        /// has to send any of it.
        #[tokio::test]
        async fn rag_ingest_over_the_size_limit_is_invalid_argument() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());
            save_local_kb(tmp.path(), None);

            let (handle, port) = spawn_test_server().await;
            let bodies = round_trip(port, ingest("big.pdf", RAG_INGEST_MAX_BYTES + 1)).await;

            let msg = expect_error(&bodies, ErrorCode::InvalidArgument);
            assert!(msg.contains("exceeds"), "got {msg:?}");
            drop(handle);
        }

        /// A piece that runs past the declared size fails the ingest and
        /// leaves no spool file behind.
        #[tokio::test]
        async fn rag_ingest_piece_past_the_declared_size_is_invalid_argument() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());
            save_local_kb(tmp.path(), None);
            let spools_before = spool_files();

            let (handle, port) = spawn_test_server().await;
            let bodies =
                round_trip_frames(port, vec![ingest("notes.txt", 4), piece(b"too long")]).await;

            let msg = expect_error(&bodies, ErrorCode::InvalidArgument);
            assert!(msg.contains("more than the declared"), "got {msg:?}");
            assert_eq!(spool_files(), spools_before, "spool file left behind");
            drop(handle);
        }

        /// The document arrives in pieces, is reported as received in
        /// bytes, and reaches extraction and embedding — which fails here
        /// on an unreachable embedder, so this needs no Ollama.
        #[tokio::test]
        async fn rag_ingest_receives_the_upload_then_extracts_it() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());
            save_local_kb(tmp.path(), Some("http://127.0.0.1:1"));
            let spools_before = spool_files();

            let doc = b"KwaaiNet is a distributed inference network. ".repeat(40);
            let (first, second) = doc.split_at(doc.len() / 2);
            let (handle, port) = spawn_test_server().await;
            let bodies = round_trip_frames(
                port,
                vec![
                    ingest("notes.txt", doc.len() as u64),
                    piece(first),
                    piece(second),
                ],
            )
            .await;

            let phases: Vec<(i32, u32, u32)> = bodies
                .iter()
                .filter_map(|b| match b {
                    server_frame::Body::RagProgress(p) => Some((p.phase, p.done, p.total)),
                    _ => None,
                })
                .collect();
            let len = doc.len() as u32;
            let acks: Vec<u32> = phases
                .iter()
                .filter(|p| p.0 == RagPhase::Receiving as i32)
                .map(|p| p.1)
                .collect();
            assert_eq!(
                acks,
                vec![first.len() as u32, len],
                "every piece should be acknowledged, got {phases:?}"
            );
            assert!(
                phases.iter().any(|p| p.0 == RagPhase::Extracting as i32),
                "extraction should follow the upload, got {phases:?}"
            );
            expect_error(&bodies, ErrorCode::Unavailable);
            assert_eq!(spool_files(), spools_before, "spool file left behind");
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

            let (handle, port) = spawn_test_server().await;
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

            let (handle, port) = spawn_test_server().await;
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

            let (handle, port) = spawn_test_server().await;
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

            let (handle, port) = spawn_test_server().await;
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

            let (handle, port) = spawn_test_server().await;
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

            let (handle, port) = spawn_test_server().await;
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

        /// Cancelling mid-upload ends the ingest at once and deletes its spool.
        #[tokio::test]
        async fn rag_ingest_cancel_during_upload_is_cancelled() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());
            save_local_kb(tmp.path(), None);

            let (handle, port) = spawn_test_server().await;
            let spools_before = spool_files();
            let frames = vec![
                (1, ingest("notes.txt", 100)),
                (1, piece(b"0123456789")),
                (2, client_frame::Body::Cancel(Cancel { target_id: 1 })),
            ];
            let bodies = session_frames(port, frames, 1, 1, false).await;

            match terminals_of(&bodies, 1).as_slice() {
                [server_frame::Body::Error(e)] => {
                    assert_eq!(e.code, ErrorCode::Cancelled as i32, "got {e:?}")
                }
                other => panic!("expected one Cancelled error, got {other:?}"),
            }
            assert_eq!(spool_files(), spools_before, "spool file left behind");
            drop(handle);
        }

        /// A session that closes mid-upload ends the ingest promptly rather
        /// than after the idle timeout.
        #[tokio::test]
        async fn rag_ingest_session_close_mid_upload_ends_the_ingest() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());
            save_local_kb(tmp.path(), None);

            let (handle, port) = spawn_test_server().await;
            let frames = vec![(1, ingest("notes.txt", 100)), (1, piece(b"0123456789"))];
            let started = std::time::Instant::now();
            let bodies = session_frames(port, frames, 1, 1, true).await;

            assert!(
                started.elapsed() < RAG_UPLOAD_IDLE_TIMEOUT / 2,
                "the ingest waited out the idle timeout"
            );
            match terminals_of(&bodies, 1).as_slice() {
                [server_frame::Body::Error(e)] => {
                    assert_eq!(e.code, ErrorCode::Cancelled as i32, "got {e:?}")
                }
                other => panic!("expected one Cancelled error, got {other:?}"),
            }
            drop(handle);
        }

        /// Pieces after exactly `size` bytes are discarded, not an error:
        /// the ingest goes on to extraction (then fails on the embedder).
        #[tokio::test]
        async fn rag_ingest_trailing_piece_is_discarded() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());
            save_local_kb(tmp.path(), Some("http://127.0.0.1:1"));

            let doc = b"KwaaiNet is a distributed inference network. ".repeat(40);
            let (handle, port) = spawn_test_server().await;
            let bodies = round_trip_frames(
                port,
                vec![
                    ingest("notes.txt", doc.len() as u64),
                    piece(&doc),
                    piece(b"zz"),
                ],
            )
            .await;

            let acks: Vec<u32> = bodies
                .iter()
                .filter_map(|b| match b {
                    server_frame::Body::RagProgress(p) if p.phase == RagPhase::Receiving as i32 => {
                        Some(p.done)
                    }
                    _ => None,
                })
                .collect();
            assert_eq!(acks, vec![doc.len() as u32], "got {bodies:?}");
            expect_error(&bodies, ErrorCode::Unavailable);
            drop(handle);
        }

        /// A RagIngest reusing an id in flight is refused, and the original
        /// keeps its pieces.
        #[tokio::test]
        async fn rag_ingest_duplicate_id_is_invalid_argument() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());
            save_local_kb(tmp.path(), Some("http://127.0.0.1:1"));

            let doc = b"KwaaiNet is a distributed inference network. ".repeat(40);
            let size = doc.len() as u64;
            let (handle, port) = spawn_test_server().await;
            let frames = vec![
                (1, ingest("notes.txt", size)),
                (1, ingest("other.txt", size)),
                (1, piece(&doc)),
            ];
            let bodies = session_frames(port, frames, 1, 2, false).await;

            match terminals_of(&bodies, 1).as_slice() {
                [server_frame::Body::Error(dup), server_frame::Body::Error(orig)] => {
                    assert_eq!(dup.code, ErrorCode::InvalidArgument as i32, "got {dup:?}");
                    assert!(dup.message.contains("already in flight"), "got {dup:?}");
                    // The original received the whole document and reached the embedder.
                    assert_eq!(orig.code, ErrorCode::Unavailable as i32, "got {orig:?}");
                }
                other => panic!("expected two errors, got {other:?}"),
            }
            drop(handle);
        }

        /// A client that sends far past the 8-piece window, without waiting
        /// for acknowledgements, fails the ingest instead of stalling the session.
        #[tokio::test]
        async fn rag_ingest_overrunning_the_window_is_invalid_argument() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());
            save_local_kb(tmp.path(), Some("http://127.0.0.1:1"));

            let (handle, port) = spawn_test_server().await;
            let pieces = 64;
            let mut bodies_out = vec![ingest("notes.txt", pieces)];
            bodies_out.extend((0..pieces).map(|_| piece(b"x")));
            let bodies = round_trip_frames(port, bodies_out).await;

            let msg = expect_error(&bodies, ErrorCode::InvalidArgument);
            assert!(msg.contains("8-piece window"), "got {msg:?}");
            drop(handle);
        }

        /// A daemon runs at most `RAG_MAX_UPLOADS` uploads; one more is refused.
        #[tokio::test]
        async fn rag_ingest_beyond_the_upload_cap_is_unavailable() {
            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());
            save_local_kb(tmp.path(), None);

            let (handle, port) = spawn_test_server().await;
            let last = RAG_MAX_UPLOADS as u64 + 1;
            let frames = (1..=last)
                .map(|id| (id, ingest("notes.txt", 100)))
                .collect();
            let bodies = session_frames(port, frames, last, 1, false).await;

            match terminals_of(&bodies, last).as_slice() {
                [server_frame::Body::Error(e)] => {
                    assert_eq!(e.code, ErrorCode::Unavailable as i32, "got {e:?}")
                }
                other => panic!("expected one Unavailable error, got {other:?}"),
            }
            for id in 1..last {
                assert!(terminals_of(&bodies, id).is_empty(), "op {id} ended early");
            }
            drop(handle);
        }

        /// Switching the model of a KB that holds documents is refused before
        /// Ollama is asked anything: the stored vectors would stop matching.
        #[tokio::test]
        async fn rag_init_switching_the_model_of_a_kb_with_documents_is_invalid_argument() {
            use kwaai_rag::meta_store::MetaStore;
            use kwaai_rpc::v1::RagInitRequest;

            let _serial = TEST_LOCK.lock().await;
            let tmp = tempfile::tempdir().expect("tempdir");
            let _env = EnvGuard::set(tmp.path());
            save_local_kb(tmp.path(), None);
            let (rag_cfg, tenant_id) =
                crate::rag_cmd::load_rag_config_for("default").expect("KB saved");
            MetaStore::open(&rag_cfg.data_dir(), tenant_id)
                .expect("open meta store")
                .put_chunks(&[chunk_meta("handbook.txt")], &[7])
                .expect("store a chunk");

            let (handle, port) = spawn_test_server().await;
            let other = if rag_cfg.embed_model == "mxbai-embed-large" {
                "nomic-embed-text"
            } else {
                "mxbai-embed-large"
            };
            let bodies = round_trip(
                port,
                client_frame::Body::RagInit(RagInitRequest {
                    kb: "default".into(),
                    embed_model: other.into(),
                }),
            )
            .await;

            let msg = expect_error(&bodies, ErrorCode::InvalidArgument);
            assert!(msg.contains("cannot switch"), "got {msg:?}");
            drop(handle);
        }

        fn chunk_meta(doc: &str) -> kwaai_rag::meta_store::ChunkMeta {
            kwaai_rag::meta_store::ChunkMeta {
                doc_name: doc.into(),
                chunk_index: 0,
                text: "text".into(),
                surrounding: String::new(),
                page_num: None,
                ingested_at: String::new(),
                section_name: None,
                skip_extraction: false,
                section_note: None,
                section_type: Default::default(),
            }
        }

        /// An unfinished ingest's vectors are removed unless the metadata
        /// store knows them, as it does for a document being re-ingested.
        #[tokio::test]
        async fn discard_unindexed_vectors_keeps_only_known_ids() {
            let tmp = tempfile::tempdir().expect("tempdir");
            let dir = tmp.path().to_path_buf();
            let db = kwaai_storage::StorageDb::open(&dir).expect("open store");
            let tenant_id = kwaai_storage::TenantManager::new(db.clone())
                .create("peer", 0, None, 2)
                .await
                .expect("create tenant")
                .tenant_id;
            let vs = Arc::new(kwaai_storage::VectorStore::new(db));
            let vectors: Vec<(i64, Vec<f32>)> =
                (1..=3).map(|id| (id, vec![1.0, id as f32])).collect();
            vs.upload(tenant_id, &vectors).await.expect("upload");
            kwaai_rag::meta_store::MetaStore::open(&dir, tenant_id)
                .expect("open meta store")
                .put_chunks(&[chunk_meta("kept.txt")], &[1])
                .expect("store a chunk");

            let residue = Residue::default();
            *residue_lock(&residue) = IngestResidue {
                store: Some((vs.clone(), tenant_id, dir)),
                ids: vec![1, 2, 3],
            };
            discard_unindexed_vectors(&residue, &Arc::new(Mutex::new(()))).await;

            let left: Vec<i64> = vs
                .search(tenant_id, &[1.0, 2.0], 10)
                .await
                .expect("search")
                .into_iter()
                .map(|r| r.id)
                .collect();
            assert_eq!(left, vec![1]);
        }

        /// Only spool files past the age limit are swept.
        #[test]
        fn sweep_removes_only_stale_spool_files() {
            let tmp = tempfile::tempdir().expect("tempdir");
            let make = |name: &str, age: Duration| {
                let path = tmp.path().join(name);
                let file = std::fs::File::create(&path).expect("create");
                file.set_modified(std::time::SystemTime::now() - age)
                    .expect("set mtime");
                path
            };
            let day = Duration::from_secs(24 * 60 * 60);
            let stale = make(&format!("{RAG_SPOOL_PREFIX}old.pdf"), 2 * day);
            let live = make(&format!("{RAG_SPOOL_PREFIX}new.pdf"), Duration::ZERO);
            let other = make("someone-elses.pdf", 2 * day);

            assert_eq!(sweep_stale_spools(tmp.path(), day), 1);
            assert!(!stale.exists());
            assert!(live.exists() && other.exists());
        }
    }
}
