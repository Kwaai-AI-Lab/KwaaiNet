//! OpenAI-compatible REST API server for KwaaiNet.
//!
//! Expose the locally-loaded model through an HTTP API that any OpenAI client
//! library can talk to by pointing `base_url` at `http://localhost:<port>/v1`.
//!
//! Endpoints:
//!   GET  /v1/models               — list available models
//!   POST /v1/chat/completions     — chat (streaming or non-streaming)
//!   POST /v1/completions          — legacy text completion

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{
        sse::{Event, Sse},
        IntoResponse, Json, Response,
    },
    routing::{get, post},
    Router,
};
use futures::stream;
use kwaai_inference::{InferenceEngine, InferenceProvider, ModelHandle};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::{mpsc, Arc};
use tracing::info;

// ---------------------------------------------------------------------------
// Inference worker thread
//
// The ML model (especially with Metal backend on macOS) needs to run on a
// stable, non-moving thread.  We spawn one OS thread that owns the engine
// for its entire lifetime and communicate with it via a sync channel.
// ---------------------------------------------------------------------------

enum WorkerMsg {
    Generate {
        prompt: String,
        reply: mpsc::SyncSender<kwaai_inference::InferenceResult<String>>,
    },
}

struct InferenceWorker {
    tx: mpsc::SyncSender<WorkerMsg>,
}

impl InferenceWorker {
    fn spawn(engine: InferenceEngine, handle: ModelHandle) -> Self {
        let (tx, rx) = mpsc::sync_channel::<WorkerMsg>(4);
        std::thread::Builder::new()
            .name("kwaai-inference".into())
            .spawn(move || {
                while let Ok(msg) = rx.recv() {
                    match msg {
                        WorkerMsg::Generate { prompt, reply } => {
                            let result = engine.generate(&handle, &prompt);
                            let _ = reply.send(result);
                        }
                    }
                }
            })
            .expect("failed to spawn inference thread");
        Self { tx }
    }

    /// Run inference, returning a future that resolves once generation is done.
    async fn generate(&self, prompt: String) -> Result<String> {
        let (reply_tx, reply_rx) = mpsc::sync_channel(1);
        self.tx
            .send(WorkerMsg::Generate { prompt, reply: reply_tx })
            .map_err(|_| anyhow::anyhow!("inference worker disconnected"))?;
        // recv() blocks — offload to the blocking thread pool.
        tokio::task::spawn_blocking(move || {
            reply_rx
                .recv()
                .map_err(|_| anyhow::anyhow!("inference worker disconnected"))?
                .map_err(|e| anyhow::anyhow!("{e}"))
        })
        .await?
    }
}

// ---------------------------------------------------------------------------
// Shared server state
// ---------------------------------------------------------------------------

struct AppState {
    worker: InferenceWorker,
    model_id: String,
}
type AppStateRef = Arc<AppState>;

// ---------------------------------------------------------------------------
// OpenAI request types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ChatRequest {
    #[allow(dead_code)]
    model: String,
    messages: Vec<ChatMsg>,
    #[serde(default)]
    stream: bool,
    max_tokens: Option<u32>,
    #[allow(dead_code)]
    temperature: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChatMsg {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct CompletionRequest {
    #[allow(dead_code)]
    model: String,
    prompt: String,
    #[serde(default)]
    stream: bool,
    #[allow(dead_code)]
    max_tokens: Option<u32>,
    #[allow(dead_code)]
    temperature: Option<f64>,
}

// ---------------------------------------------------------------------------
// OpenAI response types
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Chat template
// ---------------------------------------------------------------------------

/// Format messages as a Llama 3 instruct prompt.
/// Other popular formats (Mistral, ChatML) share a similar structure.
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

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn list_models(State(state): State<AppStateRef>) -> Json<ModelsResponse> {
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
    State(state): State<AppStateRef>,
    Json(req): Json<ChatRequest>,
) -> Response {
    let prompt = build_prompt(&req.messages);
    let model_id = state.model_id.clone();

    let text = match state.worker.generate(prompt).await {
        Ok(t) => t,
        Err(e) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    };

    let id = make_id("chatcmpl");
    let created = unix_now();
    let n_tokens = estimate_tokens(&text);

    if req.stream {
        // Deliver the entire response as a single SSE content chunk followed by [DONE].
        // Token-by-token streaming requires changes to the inference engine (future work).
        let chunk = ChatChunk {
            id,
            object: "chat.completion.chunk",
            created,
            model: model_id,
            choices: vec![ChunkChoice {
                index: 0,
                delta: Delta { role: None, content: Some(text) },
                finish_reason: Some("stop"),
            }],
        };
        let data = match serde_json::to_string(&chunk) {
            Ok(s) => s,
            Err(e) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
        };
        let events: Vec<Result<Event, Infallible>> = vec![
            Ok(Event::default().data(data)),
            Ok(Event::default().data("[DONE]")),
        ];
        Sse::new(stream::iter(events)).into_response()
    } else {
        Json(ChatCompletionResponse {
            id,
            object: "chat.completion",
            created,
            model: model_id,
            choices: vec![ChatChoice {
                index: 0,
                message: ChatMsg { role: "assistant".into(), content: text },
                finish_reason: "stop",
            }],
            usage: Usage {
                prompt_tokens: 0,
                completion_tokens: n_tokens,
                total_tokens: n_tokens,
            },
        })
        .into_response()
    }
}

async fn completions(
    State(state): State<AppStateRef>,
    Json(req): Json<CompletionRequest>,
) -> Response {
    let prompt = req.prompt.clone();
    let model_id = state.model_id.clone();

    let text = match state.worker.generate(prompt).await {
        Ok(t) => t,
        Err(e) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    };

    let id = make_id("cmpl");
    let created = unix_now();
    let n_tokens = estimate_tokens(&text);

    if req.stream {
        let data = serde_json::json!({
            "id": id,
            "object": "text_completion",
            "created": created,
            "model": model_id,
            "choices": [{ "text": &text, "index": 0, "finish_reason": "stop" }]
        })
        .to_string();
        let events: Vec<Result<Event, Infallible>> = vec![
            Ok(Event::default().data(data)),
            Ok(Event::default().data("[DONE]")),
        ];
        Sse::new(stream::iter(events)).into_response()
    } else {
        Json(CompletionResponse {
            id,
            object: "text_completion",
            created,
            model: model_id,
            choices: vec![CompletionChoice { text, index: 0, finish_reason: "stop" }],
            usage: Usage {
                prompt_tokens: 0,
                completion_tokens: n_tokens,
                total_tokens: n_tokens,
            },
        })
        .into_response()
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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

/// Rough token count estimate (~4 chars/token on average).
fn estimate_tokens(text: &str) -> u32 {
    ((text.len() as u32) / 4).max(1)
}

fn api_error(status: StatusCode, msg: &str) -> Response {
    #[derive(Serialize)]
    struct ApiErr {
        error: ErrDetail,
    }
    #[derive(Serialize)]
    struct ErrDetail {
        message: String,
        #[serde(rename = "type")]
        kind: &'static str,
    }
    (
        status,
        Json(ApiErr {
            error: ErrDetail { message: msg.to_string(), kind: "server_error" },
        }),
    )
        .into_response()
}

// ---------------------------------------------------------------------------
// Server entry point
// ---------------------------------------------------------------------------

/// Start the OpenAI-compatible API server on `port`.
///
/// Loads the model into `engine`, hands ownership to a background inference
/// thread, then runs the axum HTTP server until Ctrl-C.
pub async fn run_api_server(
    port: u16,
    engine: InferenceEngine,
    handle: ModelHandle,
    model_id: String,
) -> Result<()> {
    let state: AppStateRef = Arc::new(AppState {
        worker: InferenceWorker::spawn(engine, handle),
        model_id: model_id.clone(),
    });

    let app = Router::new()
        .route("/v1/models", get(list_models))
        .route("/v1/chat/completions", post(chat_completions))
        .route("/v1/completions", post(completions))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!(
        "KwaaiNet OpenAI API server ready — http://localhost:{}/v1  (model: {})",
        port, model_id
    );
    println!();
    println!("  OpenAI base URL:  http://localhost:{}/v1", port);
    println!("  Model:            {}", model_id);
    println!();
    println!("  Try it:");
    println!("    curl http://localhost:{}/v1/models", port);
    println!("    curl http://localhost:{}/v1/chat/completions \\", port);
    println!("      -H 'Content-Type: application/json' \\");
    println!("      -d '{{\"model\":\"{}\",\"messages\":[{{\"role\":\"user\",\"content\":\"Hello!\"}}]}}'", model_id);
    println!();

    axum::serve(listener, app).await?;
    Ok(())
}
