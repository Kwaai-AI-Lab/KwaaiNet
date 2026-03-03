//! OpenAI-compatible HTTP API backed by KwaaiNet distributed shard inference.
//!
//! `kwaainet shard api --port 8080`
//!
//! Endpoints:
//!   GET  /v1/models
//!   POST /v1/chat/completions   (per-token SSE streaming + non-streaming)
//!   POST /v1/completions        (per-token SSE streaming + non-streaming)

use anyhow::{Context, Result};
use axum::{
    extract::State,
    response::{
        sse::{Event, Sse},
        IntoResponse, Json, Response,
    },
    routing::{get, post},
    Router,
};
use candle_core::Device;
use futures::stream::{self, StreamExt as _};
use kwaai_p2p::NetworkConfig;
use kwaai_p2p_daemon::P2PClient;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, sync::Arc};
use tokio::sync::Mutex;

use crate::block_rpc::{f16_bytes_to_tensor, token_ids_to_bytes, InferenceRequest, PayloadType};
use crate::cli::ShardApiArgs;
use crate::config::KwaaiNetConfig;
use crate::display::*;
use crate::hf;
use crate::shard_cmd::{
    daemon_socket, discover_chain, forward_through_chain, sample_token, BlockServerEntry,
};

// ── Shared server state ───────────────────────────────────────────────────────

struct AppState {
    client: Arc<Mutex<P2PClient>>,
    chain: Arc<Vec<BlockServerEntry>>,
    tokenizer: Arc<kwaai_inference::tokenizer::BpeTokenizer>,
    total_blocks: usize,
    model_id: String,
    default_temp: f32,
    eos_id: u32,
    bos_id: Option<u32>,
    our_peer_id: PeerId,
}

// ── OpenAI request types ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct ChatRequest {
    #[allow(dead_code)]
    model: String,
    messages: Vec<ChatMsg>,
    #[serde(default)]
    stream: bool,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_k: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct CompletionRequest {
    #[allow(dead_code)]
    model: String,
    prompt: String,
    #[serde(default)]
    stream: bool,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_k: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChatMsg {
    role: String,
    content: String,
}

// ── OpenAI response types ─────────────────────────────────────────────────────

#[derive(Serialize)]
struct ModelsResponse {
    object: &'static str,
    data: Vec<ModelObject>,
}

#[derive(Serialize)]
struct ModelObject {
    id: String,
    object: &'static str,
    created: u64,
    owned_by: &'static str,
}

#[derive(Serialize)]
struct ChatCompletionResponse {
    id: String,
    object: &'static str,
    created: u64,
    model: String,
    choices: Vec<ChatChoice>,
    usage: Usage,
}

#[derive(Serialize)]
struct ChatChoice {
    index: u32,
    message: ChatMsg,
    finish_reason: &'static str,
}

#[derive(Serialize)]
struct ChatChunk {
    id: String,
    object: &'static str,
    created: u64,
    model: String,
    choices: Vec<ChunkChoice>,
}

#[derive(Serialize)]
struct ChunkChoice {
    index: u32,
    delta: Delta,
    finish_reason: Option<&'static str>,
}

#[derive(Serialize)]
struct Delta {
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
}

#[derive(Serialize)]
struct CompletionResponse {
    id: String,
    object: &'static str,
    created: u64,
    model: String,
    choices: Vec<CompletionChoice>,
    usage: Usage,
}

#[derive(Serialize)]
struct CompletionChoice {
    text: String,
    index: u32,
    finish_reason: &'static str,
}

#[derive(Serialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

// ── Chat template ─────────────────────────────────────────────────────────────

fn build_prompt(messages: &[ChatMsg]) -> String {
    let mut s = String::from("<|begin_of_text|>");
    for msg in messages {
        s.push_str(&format!(
            "<|start_header_id|>{}<|end_header_id|>\n\n{}<|eot_id|>",
            msg.role, msg.content
        ));
    }
    s.push_str("<|start_header_id|>assistant<|end_header_id|>\n\n");
    s
}

// ── Inference loop ────────────────────────────────────────────────────────────

/// Run distributed inference, sending each decoded token piece via `tx`.
/// Acquires the P2PClient lock for the full duration (requests are serialized).
async fn run_inference(
    state: Arc<AppState>,
    prompt: String,
    max_tokens: usize,
    temperature: f32,
    top_k: usize,
    top_p: f32,
    tx: tokio::sync::mpsc::Sender<String>,
) {
    use candle_core::IndexOp as _;
    use kwaai_inference::tokenizer::Tokenizer as _;

    let mut token_ids: Vec<u32> = match state.tokenizer.encode(&prompt) {
        Ok(ids) => ids,
        Err(e) => {
            let _ = tx.send(format!("[tokenizer error: {e}]")).await;
            return;
        }
    };
    if let Some(bos) = state.bos_id {
        token_ids.insert(0, bos);
    }

    let session_id: u64 = {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(42)
    };

    let mut generated = 0usize;
    let mut seq_pos = 0usize;
    let mut current_ids = token_ids;
    let device = Device::Cpu;

    let mut client_guard = state.client.lock().await;

    loop {
        let (shape, data) = token_ids_to_bytes(&current_ids);
        let request = InferenceRequest {
            session_id,
            seq_pos: seq_pos as u32,
            payload_type: PayloadType::TokenIds,
            shape,
            data,
        };

        let logits_bytes = match forward_through_chain(
            &mut client_guard,
            &state.chain,
            state.total_blocks,
            session_id,
            seq_pos as u32,
            request,
            Some(&state.our_peer_id),
        )
        .await
        {
            Ok(r) => r,
            Err(e) => {
                let _ = tx.send(format!("[inference error: {e}]")).await;
                break;
            }
        };

        let logits_shape = &logits_bytes.shape;
        let logits_tensor = match f16_bytes_to_tensor(&logits_bytes.data, logits_shape, &device) {
            Ok(t) => t,
            Err(e) => {
                let _ = tx.send(format!("[tensor error: {e}]")).await;
                break;
            }
        };

        let last_logits = if logits_shape.len() == 3 && logits_shape[1] > 1 {
            let seq_len = logits_shape[1] as usize;
            match logits_tensor.i((0, seq_len - 1, ..)) {
                Ok(t) => t,
                Err(e) => {
                    let _ = tx.send(format!("[slice error: {e}]")).await;
                    break;
                }
            }
        } else {
            match logits_tensor.flatten_all() {
                Ok(t) => t,
                Err(e) => {
                    let _ = tx.send(format!("[flatten error: {e}]")).await;
                    break;
                }
            }
        };

        let next_id = match sample_token(&last_logits, temperature, top_k, top_p) {
            Ok(id) => id as u32,
            Err(e) => {
                let _ = tx.send(format!("[sample error: {e}]")).await;
                break;
            }
        };

        if let Ok(piece) = state.tokenizer.decode(&[next_id]) {
            if tx.send(piece).await.is_err() {
                break; // client disconnected
            }
        }

        generated += 1;
        seq_pos += current_ids.len();

        if next_id == state.eos_id || generated >= max_tokens {
            break;
        }
        current_ids = vec![next_id];
    }
    // tx dropped here → channel closes → SSE stream ends
}

// ── Handlers ──────────────────────────────────────────────────────────────────

async fn list_models(State(state): State<Arc<AppState>>) -> Json<ModelsResponse> {
    Json(ModelsResponse {
        object: "list",
        data: vec![ModelObject {
            id: state.model_id.clone(),
            object: "model",
            created: unix_now(),
            owned_by: "kwaai",
        }],
    })
}

async fn chat_completions(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ChatRequest>,
) -> Response {
    let prompt = build_prompt(&req.messages);
    let model_id = state.model_id.clone();
    let max_tokens = req.max_tokens.unwrap_or(200) as usize;
    let temperature = req.temperature.unwrap_or(state.default_temp);
    let top_k = req.top_k.unwrap_or(0) as usize;
    let top_p = req.top_p.unwrap_or(1.0);

    let (tx, rx) = tokio::sync::mpsc::channel::<String>(512);
    let state_c = state.clone();
    tokio::spawn(async move {
        run_inference(state_c, prompt, max_tokens, temperature, top_k, top_p, tx).await;
    });

    if req.stream {
        make_chat_sse(rx, model_id)
    } else {
        collect_chat(rx, model_id).await
    }
}

async fn completions(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CompletionRequest>,
) -> Response {
    let prompt = req.prompt.clone();
    let model_id = state.model_id.clone();
    let max_tokens = req.max_tokens.unwrap_or(200) as usize;
    let temperature = req.temperature.unwrap_or(state.default_temp);
    let top_k = req.top_k.unwrap_or(0) as usize;
    let top_p = req.top_p.unwrap_or(1.0);

    let (tx, rx) = tokio::sync::mpsc::channel::<String>(512);
    let state_c = state.clone();
    tokio::spawn(async move {
        run_inference(state_c, prompt, max_tokens, temperature, top_k, top_p, tx).await;
    });

    if req.stream {
        make_completion_sse(rx, model_id)
    } else {
        collect_completion(rx, model_id).await
    }
}

// ── SSE helpers ───────────────────────────────────────────────────────────────

/// State threaded through `stream::unfold` for SSE.
struct SseCtx {
    rx: tokio::sync::mpsc::Receiver<String>,
    id: String,
    model_id: String,
    created: u64,
}

fn make_chat_sse(rx: tokio::sync::mpsc::Receiver<String>, model_id: String) -> Response {
    let ctx = SseCtx {
        rx,
        id: make_id("chatcmpl"),
        model_id,
        created: unix_now(),
    };

    let token_stream = stream::unfold(ctx, |mut ctx| async move {
        ctx.rx.recv().await.map(|piece| {
            let chunk = ChatChunk {
                id: ctx.id.clone(),
                object: "chat.completion.chunk",
                created: ctx.created,
                model: ctx.model_id.clone(),
                choices: vec![ChunkChoice {
                    index: 0,
                    delta: Delta {
                        role: None,
                        content: Some(piece),
                    },
                    finish_reason: None,
                }],
            };
            let data = serde_json::to_string(&chunk).unwrap_or_default();
            let event: Result<Event, Infallible> = Ok(Event::default().data(data));
            (event, ctx)
        })
    });

    let done = stream::once(async { Ok::<Event, Infallible>(Event::default().data("[DONE]")) });

    Sse::new(token_stream.chain(done)).into_response()
}

fn make_completion_sse(rx: tokio::sync::mpsc::Receiver<String>, model_id: String) -> Response {
    let ctx = SseCtx {
        rx,
        id: make_id("cmpl"),
        model_id,
        created: unix_now(),
    };

    let token_stream = stream::unfold(ctx, |mut ctx| async move {
        ctx.rx.recv().await.map(|piece| {
            let data = serde_json::json!({
                "id":      ctx.id,
                "object":  "text_completion",
                "created": ctx.created,
                "model":   ctx.model_id,
                "choices": [{ "text": piece, "index": 0, "finish_reason": null }],
            })
            .to_string();
            let event: Result<Event, Infallible> = Ok(Event::default().data(data));
            (event, ctx)
        })
    });

    let done = stream::once(async { Ok::<Event, Infallible>(Event::default().data("[DONE]")) });

    Sse::new(token_stream.chain(done)).into_response()
}

async fn collect_chat(mut rx: tokio::sync::mpsc::Receiver<String>, model_id: String) -> Response {
    let mut text = String::new();
    while let Some(piece) = rx.recv().await {
        text.push_str(&piece);
    }
    let n = estimate_tokens(&text);
    Json(ChatCompletionResponse {
        id: make_id("chatcmpl"),
        object: "chat.completion",
        created: unix_now(),
        model: model_id,
        choices: vec![ChatChoice {
            index: 0,
            message: ChatMsg {
                role: "assistant".into(),
                content: text,
            },
            finish_reason: "stop",
        }],
        usage: Usage {
            prompt_tokens: 0,
            completion_tokens: n,
            total_tokens: n,
        },
    })
    .into_response()
}

async fn collect_completion(
    mut rx: tokio::sync::mpsc::Receiver<String>,
    model_id: String,
) -> Response {
    let mut text = String::new();
    while let Some(piece) = rx.recv().await {
        text.push_str(&piece);
    }
    let n = estimate_tokens(&text);
    Json(CompletionResponse {
        id: make_id("cmpl"),
        object: "text_completion",
        created: unix_now(),
        model: model_id,
        choices: vec![CompletionChoice {
            text,
            index: 0,
            finish_reason: "stop",
        }],
        usage: Usage {
            prompt_tokens: 0,
            completion_tokens: n,
            total_tokens: n,
        },
    })
    .into_response()
}

// ── Utilities ─────────────────────────────────────────────────────────────────

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn make_id(prefix: &str) -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    format!("{}-{}{:05}", prefix, unix_now(), nanos % 100_000)
}

fn estimate_tokens(text: &str) -> u32 {
    ((text.len() as u32) / 4).max(1)
}

// ── Entry point ───────────────────────────────────────────────────────────────

pub async fn run(args: ShardApiArgs) -> Result<()> {
    use kwaai_inference::tokenizer::Tokenizer as _;

    let cfg = KwaaiNetConfig::load_or_create()?;
    let model_ref = args.model.as_deref().unwrap_or(&cfg.model).to_string();
    let dht_prefix = match &cfg.model_dht_prefix {
        Some(p) => p.clone(),
        None => {
            let base = model_ref.split('/').next_back().unwrap_or(&model_ref);
            base.replace('.', "-")
        }
    };
    let total_blocks = args
        .total_blocks
        .unwrap_or_else(|| cfg.model_total_blocks() as usize);

    // Resolve tokenizer directory
    let model_dir: std::path::PathBuf = if let Some(p) = args.model_path {
        p
    } else {
        hf::resolve_snapshot(&model_ref)?
    };
    let tokenizer_path = model_dir.join("tokenizer.json");
    let tokenizer = kwaai_inference::tokenizer::BpeTokenizer::from_file(&tokenizer_path)
        .context("Failed to load tokenizer")?;
    let eos_id = tokenizer.eos_token_id().unwrap_or(2);
    let bos_id = tokenizer.bos_token_id();

    // Connect to p2pd
    let daemon_addr = daemon_socket();
    let mut client = P2PClient::connect(&daemon_addr)
        .await
        .context("Cannot connect to node — start it first with `kwaainet start --daemon`")?;

    let peer_id_hex = client.identify().await.context("identify peer")?;
    let our_peer_id = PeerId::from_bytes(&hex::decode(&peer_id_hex)?).context("parse peer ID")?;

    let bootstrap_peers: Vec<String> = if cfg.initial_peers.is_empty() {
        NetworkConfig::with_petals_bootstrap().bootstrap_peers
    } else {
        cfg.initial_peers.clone()
    };

    print_box_header("🌐 KwaaiNet Shard API");
    println!("  Model:        {}", model_ref);
    println!("  DHT prefix:   {}", dht_prefix);
    println!("  Total blocks: {}", total_blocks);
    println!("  Port:         {}", args.port);
    println!();

    use std::io::Write as _;
    print!("  Discovering block chain… ");
    std::io::stdout().flush().ok();

    let chain = discover_chain(
        &mut client,
        &our_peer_id,
        &dht_prefix,
        total_blocks,
        &bootstrap_peers,
    )
    .await;

    if chain.is_empty() {
        println!("no nodes found");
        println!();
        print_warning("No block servers found — start serving first: kwaainet shard serve");
        print_separator();
        return Ok(());
    }
    println!("{} node(s)", chain.len());

    for (i, entry) in chain.iter().enumerate() {
        println!(
            "  [{:>2}] blocks {:>3}–{:>3}  {}",
            i + 1,
            entry.start_block,
            entry.end_block - 1,
            entry.public_name,
        );
    }
    println!();

    // Pre-connect to all block-server peers
    for entry in &chain {
        let hint = format!("/p2p/{}", entry.peer_id.to_base58());
        let _ = client.connect_peer(&hint).await;
    }

    let state: Arc<AppState> = Arc::new(AppState {
        client: Arc::new(Mutex::new(client)),
        chain: Arc::new(chain),
        tokenizer: Arc::new(tokenizer),
        total_blocks,
        model_id: model_ref.clone(),
        default_temp: args.temperature,
        eos_id,
        bos_id,
        our_peer_id,
    });

    let app = Router::new()
        .route("/v1/models", get(list_models))
        .route("/v1/chat/completions", post(chat_completions))
        .route("/v1/completions", post(completions))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    print_success(&format!(
        "API server ready — http://localhost:{}/v1",
        args.port
    ));
    println!("  Model:  {}", model_ref);
    println!();
    println!("  Try:");
    println!("    curl http://localhost:{}/v1/models", args.port);
    println!(
        "    curl http://localhost:{}/v1/chat/completions \\",
        args.port
    );
    println!("      -H 'Content-Type: application/json' \\");
    println!(
        "      -d '{{\"model\":\"{model_ref}\",\"messages\":[{{\"role\":\"user\",\"content\":\"Hello!\"}}]}}'"
    );
    println!();
    print_separator();

    axum::serve(listener, app).await?;
    Ok(())
}
