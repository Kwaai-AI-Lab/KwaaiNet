use std::pin::Pin;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use axum::extract::{Multipart, Path as AxumPath, State};
use axum::response::{IntoResponse, Json};
use axum::routing::{delete, get, post};
use axum::Router;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

use kwaai_rag::{
    embedder::EmbedClient,
    ingestion::{ingest_text, IngestConfig},
    meta_store::MetaStore,
    prompt::build_chat_messages,
    retriever::{retrieve, RetrieveConfig},
};

use crate::display::*;

#[cfg(feature = "storage")]
use crate::storage_rpc::{
    http_delete_vectors, http_search_vectors, http_upload_vectors, rpc_delete_vectors,
    rpc_search_vectors, rpc_upload_vectors,
};
use kwaai_p2p_daemon::P2PClient;

// ── Shared state ──────────────────────────────────────────────────────────────

struct RagState {
    tenant_id: Uuid,
    eve_peer: PeerId,
    /// Set when Eve is local; storage calls use HTTP instead of P2P.
    storage_url: Option<String>,
    /// Only populated when storage_url is None (remote Eve via P2P).
    client: Option<Arc<Mutex<P2PClient>>>,
    embed: EmbedClient,
    meta: Arc<MetaStore>,
    inference_url: String,
    top_k: usize,
    http: reqwest::Client,
}

pub async fn run(port: u16, inference_url: String, top_k: usize) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature. Rebuild with: cargo build --features storage");

    #[cfg(feature = "storage")]
    {
        use crate::config::KwaaiNetConfig;
        use crate::rag_cmd::*;

        let cfg = KwaaiNetConfig::load_or_create()?;
        let rag = cfg
            .rag
            .context("RAG not initialised. Run: kwaainet rag init")?;

        let tenant_id: Uuid = rag.tenant_id.as_deref().context("no tenant_id")?.parse()?;
        let eve_peer: PeerId = rag
            .eve_peer_id
            .as_deref()
            .context("no eve_peer_id")?
            .parse()?;
        let storage_url = rag.storage_url.clone();

        let embed = EmbedClient::new(None, Some(rag.embed_model.clone()));
        print_info("Probing embedding model…");
        embed.check_dim().await?;

        let p2p_client = if storage_url.is_none() {
            let (c, _) = crate::vpk::p2p_connect().await?;
            Some(Arc::new(Mutex::new(c)))
        } else {
            None
        };
        let meta = Arc::new(MetaStore::open(&rag.data_dir(), tenant_id)?);

        let state = Arc::new(RagState {
            tenant_id,
            eve_peer,
            storage_url,
            client: p2p_client,
            embed,
            meta,
            inference_url: inference_url.clone(),
            top_k,
            http: reqwest::Client::new(),
        });

        let app = Router::new()
            .route("/v1/models", get(list_models))
            .route("/v1/chat/completions", post(chat_completions))
            .route("/api/ingest", post(api_ingest))
            .route("/api/docs", get(api_list_docs))
            .route("/api/docs/:name", delete(api_delete_doc))
            .route("/", get(serve_ui))
            .with_state(state);

        let addr = format!("0.0.0.0:{port}");
        print_box_header("RAG Server");
        println!("  Listening on http://localhost:{port}");
        println!("  Inference:  {inference_url}");
        println!("  Tenant:     {tenant_id}");
        print_separator();

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .with_context(|| format!("binding {addr}"))?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

// ── Route handlers ────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct ModelsResponse {
    object: &'static str,
    data: Vec<ModelEntry>,
}

#[derive(Serialize)]
struct ModelEntry {
    id: &'static str,
    object: &'static str,
    owned_by: &'static str,
}

async fn list_models() -> Json<ModelsResponse> {
    Json(ModelsResponse {
        object: "list",
        data: vec![ModelEntry {
            id: "kwaai-rag",
            object: "model",
            owned_by: "kwaai",
        }],
    })
}

#[derive(Deserialize)]
struct ChatRequest {
    messages: Vec<serde_json::Value>,
    #[serde(default = "default_stream")]
    stream: bool,
    #[serde(default)]
    max_tokens: Option<u32>,
    #[serde(default)]
    temperature: Option<f32>,
}

fn default_stream() -> bool {
    false
}

async fn chat_completions(
    State(state): State<Arc<RagState>>,
    Json(req): Json<ChatRequest>,
) -> impl IntoResponse {
    match do_chat(&state, req).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => {
            let body = serde_json::json!({"error": {"message": e.to_string()}});
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
        }
    }
}

async fn do_chat(state: &RagState, req: ChatRequest) -> Result<serde_json::Value> {
    // Extract last user message.
    let query = req
        .messages
        .iter()
        .rev()
        .find(|m| m.get("role").and_then(|r| r.as_str()) == Some("user"))
        .and_then(|m| m.get("content")?.as_str())
        .unwrap_or("")
        .to_string();

    #[cfg(not(feature = "storage"))]
    bail!("storage feature required");

    #[cfg(feature = "storage")]
    {
        let tenant_id = state.tenant_id;
        let eve_peer = state.eve_peer;

        let cfg = RetrieveConfig {
            top_k: state.top_k,
            min_score: 0.0,
            use_sentence_window: false,
        };

        let chunks = if let Some(ref url) = state.storage_url {
            let http = state.http.clone();
            let url = url.clone();
            retrieve(&query, &cfg, &state.embed, &state.meta, move |emb, k| {
                let h = http.clone();
                let u = url.clone();
                Box::pin(async move {
                    let raw = http_search_vectors(&h, &u, tenant_id, emb, k).await?;
                    Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                })
                    as Pin<Box<dyn std::future::Future<Output = Result<Vec<(i64, f64)>>> + Send>>
            })
            .await?
        } else {
            let client = state.client.as_ref().unwrap().clone();
            retrieve(&query, &cfg, &state.embed, &state.meta, move |emb, k| {
                let c = client.clone();
                Box::pin(async move {
                    let guard = c.lock().await;
                    let raw = rpc_search_vectors(&*guard, &eve_peer, tenant_id, emb, k).await?;
                    Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                })
                    as Pin<Box<dyn std::future::Future<Output = Result<Vec<(i64, f64)>>> + Send>>
            })
            .await?
        };

        // Build history from prior messages (all but last).
        let history: Vec<kwaai_rag::prompt::ChatMessage> = req
            .messages
            .iter()
            .rev()
            .skip(1) // skip the user message we already extracted
            .rev()
            .filter_map(|m| {
                let role = m.get("role")?.as_str()?.to_string();
                let content = m.get("content")?.as_str()?.to_string();
                Some(kwaai_rag::prompt::ChatMessage { role, content })
            })
            .collect();

        let messages = build_chat_messages(&query, &chunks, &history, 8192);

        // Forward to shard API.
        let mut payload = serde_json::json!({
            "model": "default",
            "messages": messages,
            "stream": false,
        });
        if let Some(t) = req.temperature {
            payload["temperature"] = serde_json::json!(t);
        }
        if let Some(m) = req.max_tokens {
            payload["max_tokens"] = serde_json::json!(m);
        }

        let resp = state
            .http
            .post(format!("{}/v1/chat/completions", state.inference_url))
            .json(&payload)
            .send()
            .await
            .context("calling shard API")?;

        let body: serde_json::Value = resp.json().await.context("parsing shard response")?;
        Ok(body)
    }
}

#[derive(Serialize)]
struct DocsResponse {
    docs: Vec<String>,
}

async fn api_list_docs(State(state): State<Arc<RagState>>) -> impl IntoResponse {
    match state.meta.list_docs() {
        Ok(docs) => Json(DocsResponse { docs }).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn api_delete_doc(
    State(state): State<Arc<RagState>>,
    AxumPath(name): AxumPath<String>,
) -> impl IntoResponse {
    #[cfg(not(feature = "storage"))]
    return (
        axum::http::StatusCode::NOT_IMPLEMENTED,
        "storage feature required",
    )
        .into_response();

    #[cfg(feature = "storage")]
    {
        let ids = match state.meta.delete_doc(&name) {
            Ok(ids) => ids,
            Err(e) => {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                    .into_response()
            }
        };
        if ids.is_empty() {
            return (axum::http::StatusCode::NOT_FOUND, "document not found").into_response();
        }
        if let Some(ref url) = state.storage_url {
            let _ = http_delete_vectors(&state.http, url, state.tenant_id, ids.clone()).await;
        } else {
            let client = state.client.as_ref().unwrap().lock().await;
            let _ =
                rpc_delete_vectors(&*client, &state.eve_peer, state.tenant_id, ids.clone()).await;
        }
        Json(serde_json::json!({"deleted": ids.len()})).into_response()
    }
}

async fn api_ingest(
    State(state): State<Arc<RagState>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    #[cfg(not(feature = "storage"))]
    return (
        axum::http::StatusCode::NOT_IMPLEMENTED,
        "storage feature required",
    )
        .into_response();

    #[cfg(feature = "storage")]
    {
        while let Ok(Some(field)) = multipart.next_field().await {
            let doc_name = field.file_name().unwrap_or("upload.txt").to_string();
            let bytes = match field.bytes().await {
                Ok(b) => b,
                Err(e) => {
                    return (axum::http::StatusCode::BAD_REQUEST, e.to_string()).into_response()
                }
            };
            let text = match String::from_utf8(bytes.to_vec()) {
                Ok(t) => t,
                Err(_) => {
                    return (
                        axum::http::StatusCode::BAD_REQUEST,
                        "file must be valid UTF-8",
                    )
                        .into_response()
                }
            };

            let tenant_id = state.tenant_id;
            let eve_peer = state.eve_peer;
            let meta = state.meta.clone();
            let cfg = IngestConfig::new(state.embed.clone());

            let result = if let Some(ref url) = state.storage_url {
                let http = state.http.clone();
                let url = url.clone();
                ingest_text(
                    &cfg,
                    &meta,
                    &doc_name,
                    &text,
                    move |vectors| {
                        let h = http.clone();
                        let u = url.clone();
                        Box::pin(
                            async move { http_upload_vectors(&h, &u, tenant_id, vectors).await },
                        )
                            as Pin<Box<dyn std::future::Future<Output = Result<usize>> + Send>>
                    },
                    None::<fn(usize, usize)>,
                )
                .await
            } else {
                let client = state.client.as_ref().unwrap().clone();
                ingest_text(
                    &cfg,
                    &meta,
                    &doc_name,
                    &text,
                    move |vectors| {
                        let c = client.clone();
                        Box::pin(async move {
                            let guard = c.lock().await;
                            rpc_upload_vectors(&*guard, &eve_peer, tenant_id, vectors).await
                        })
                            as Pin<Box<dyn std::future::Future<Output = Result<usize>> + Send>>
                    },
                    None::<fn(usize, usize)>,
                )
                .await
            };

            match result {
                Ok(r) => {
                    return Json(serde_json::json!({
                        "doc_name": doc_name,
                        "chunks": r.chunks_ingested,
                        "vectors": r.vectors_uploaded,
                    }))
                    .into_response()
                }
                Err(e) => {
                    return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                        .into_response()
                }
            }
        }
        (axum::http::StatusCode::BAD_REQUEST, "no file in multipart").into_response()
    }
}

async fn serve_ui() -> impl IntoResponse {
    (
        axum::http::StatusCode::OK,
        [("content-type", "text/html; charset=utf-8")],
        RAG_UI_HTML,
    )
}

// Minimal embedded UI.
const RAG_UI_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>KwaaiNet RAG</title>
<style>
  body { font-family: system-ui, sans-serif; max-width: 800px; margin: 40px auto; padding: 0 20px; background: #0d1117; color: #c9d1d9; }
  h1 { color: #58a6ff; }
  textarea, input[type=text] { width: 100%; background: #161b22; color: #c9d1d9; border: 1px solid #30363d; border-radius: 6px; padding: 8px; font-size: 14px; }
  button { background: #238636; color: #fff; border: none; border-radius: 6px; padding: 8px 16px; cursor: pointer; margin: 4px 2px; }
  button:hover { background: #2ea043; }
  #chat { border: 1px solid #30363d; border-radius: 6px; padding: 12px; min-height: 300px; margin: 12px 0; white-space: pre-wrap; font-size: 14px; }
  .user { color: #58a6ff; } .assistant { color: #3fb950; }
  #upload-status { color: #3fb950; margin: 8px 0; font-size: 13px; }
  #docs-list { margin: 8px 0; font-size: 13px; }
</style>
</head>
<body>
<h1>KwaaiNet RAG</h1>

<h3>Upload Document</h3>
<input type="file" id="file-input" accept=".txt,.md">
<button onclick="uploadFile()">Ingest</button>
<div id="upload-status"></div>

<h3>Documents</h3>
<button onclick="loadDocs()">Refresh</button>
<div id="docs-list"></div>

<h3>Chat</h3>
<div id="chat"></div>
<textarea id="query" rows="2" placeholder="Ask a question…"></textarea>
<button onclick="sendChat()">Send</button>
<button onclick="document.getElementById('chat').textContent=''">Clear</button>

<script>
let history = [];

async function uploadFile() {
  const file = document.getElementById('file-input').files[0];
  if (!file) return alert('Select a file first');
  const fd = new FormData(); fd.append('file', file, file.name);
  const status = document.getElementById('upload-status');
  status.textContent = 'Uploading…';
  try {
    const r = await fetch('/api/ingest', { method: 'POST', body: fd });
    const j = await r.json();
    status.textContent = `✓ Ingested ${j.chunks} chunks from "${j.doc_name}"`;
    loadDocs();
  } catch(e) { status.textContent = '✗ ' + e; }
}

async function loadDocs() {
  const r = await fetch('/api/docs');
  const j = await r.json();
  const el = document.getElementById('docs-list');
  el.textContent = j.docs.length ? j.docs.map(d => '• ' + d).join('\n') : 'No documents ingested yet.';
}

async function sendChat() {
  const q = document.getElementById('query').value.trim();
  if (!q) return;
  const chat = document.getElementById('chat');
  chat.innerHTML += `<span class="user">You: ${q}\n</span>`;
  history.push({ role: 'user', content: q });
  document.getElementById('query').value = '';

  try {
    const r = await fetch('/v1/chat/completions', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ model: 'kwaai-rag', messages: history, stream: false })
    });
    const j = await r.json();
    const answer = j.choices?.[0]?.message?.content ?? JSON.stringify(j);
    chat.innerHTML += `<span class="assistant">Assistant: ${answer}\n\n</span>`;
    history.push({ role: 'assistant', content: answer });
    chat.scrollTop = chat.scrollHeight;
  } catch(e) { chat.innerHTML += `<span style="color:#f85149">Error: ${e}\n</span>`; }
}

document.getElementById('query').addEventListener('keydown', e => {
  if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); sendChat(); }
});

loadDocs();
</script>
</body>
</html>"#;
