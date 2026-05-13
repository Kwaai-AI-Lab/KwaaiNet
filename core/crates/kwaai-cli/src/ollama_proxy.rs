//! P2P proxy for Ollama HTTP — tunnels `/v1/chat/completions` and friends
//! through the KwaaiNet fabric so remote nodes can offload LLM inference to
//! peers that have Ollama running locally.
//!
//! Protocol ID: `/kwaai/ollama-proxy/1.0.0`
//!
//! Message flow (client perspective):
//! ```text
//! extract_from_text  ──HTTP──▶  local TCP proxy  ──P2P──▶  remote node
//!                                                             │  Ollama
//!                              local TCP proxy  ◀──P2P──  ◀──┘
//! ```
//!
//! The **server** handler (`make_ollama_proxy_handler`) forwards the incoming
//! msgpack request to `localhost:11434` and sends back the response.
//!
//! The **client** helper (`start_local_proxy`) starts a local TCP listener on a
//! random port.  Callers substitute the returned `http://127.0.0.1:PORT` for any
//! `p2p://PEER_ID` URL so that the rest of the pipeline needs no modification.

use anyhow::{Context, Result};
use kwaai_p2p_daemon::P2PClient;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::{pin::Pin, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tracing::{debug, warn};

pub const OLLAMA_PROXY_PROTO: &str = "/kwaai/ollama-proxy/1.0.0";

// ── Wire types ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyRequest {
    pub method: String,
    pub path: String,
    pub body: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

// ── Server ────────────────────────────────────────────────────────────────────

/// Build a unary handler that forwards incoming proxy requests to the local
/// Ollama at `localhost:11434`.
///
/// Register with `client.add_unary_handler(OLLAMA_PROXY_PROTO, handler, false)`.
#[allow(clippy::type_complexity)]
pub fn make_ollama_proxy_handler() -> impl Fn(
    Vec<u8>,
) -> Pin<
    Box<dyn std::future::Future<Output = kwaai_p2p_daemon::error::Result<Vec<u8>>> + Send>,
>
       + Send
       + Sync
       + 'static {
    move |data: Vec<u8>| {
        Box::pin(async move {
            let req: ProxyRequest = match rmp_serde::from_slice(&data) {
                Ok(r) => r,
                Err(e) => {
                    warn!("ollama_proxy server: bad request: {e}");
                    return encode_err(400, &format!("bad request: {e}"));
                }
            };

            debug!("ollama_proxy server: {} {}", req.method, req.path);

            let client = match reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
            {
                Ok(c) => c,
                Err(e) => return encode_err(503, &format!("reqwest init: {e}")),
            };

            let url = format!("http://localhost:11434{}", req.path);
            let method: reqwest::Method = req.method.parse().unwrap_or(reqwest::Method::POST);

            let result = client
                .request(method, &url)
                .header("Content-Type", "application/json")
                .body(req.body)
                .send()
                .await;

            let (status, body) = match result {
                Ok(r) => {
                    let status = r.status().as_u16();
                    let body = match tokio::time::timeout(
                        std::time::Duration::from_secs(120),
                        r.bytes(),
                    )
                    .await
                    {
                        Ok(Ok(b)) => b.to_vec(),
                        Ok(Err(e)) => format!("body error: {e}").into_bytes(),
                        Err(_) => b"body timeout".to_vec(),
                    };
                    (status, body)
                }
                Err(e) => (503u16, format!("upstream: {e}").into_bytes()),
            };

            rmp_serde::to_vec_named(&ProxyResponse { status, body })
                .map_err(|e| kwaai_p2p_daemon::error::Error::Protocol(e.to_string()))
        })
    }
}

fn encode_err(
    status: u16,
    msg: &str,
) -> kwaai_p2p_daemon::error::Result<Vec<u8>> {
    rmp_serde::to_vec_named(&ProxyResponse {
        status,
        body: msg.as_bytes().to_vec(),
    })
    .map_err(|e| kwaai_p2p_daemon::error::Error::Protocol(e.to_string()))
}

// ── Client ────────────────────────────────────────────────────────────────────

/// Start a local TCP listener that proxies HTTP → P2P → remote Ollama.
///
/// Returns `(local_port, join_handle)`.  Drop the handle to stop the proxy.
pub async fn start_local_proxy(
    client: Arc<P2PClient>,
    peer_id: PeerId,
) -> Result<(u16, tokio::task::JoinHandle<()>)> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind local ollama proxy")?;
    let port = listener.local_addr()?.port();

    let handle = tokio::spawn(async move {
        loop {
            let (stream, _) = match listener.accept().await {
                Ok(s) => s,
                Err(e) => {
                    warn!("ollama_proxy local accept: {e}");
                    break;
                }
            };
            let client = client.clone();
            let peer_id = peer_id;
            tokio::spawn(handle_connection(stream, client, peer_id));
        }
    });

    Ok((port, handle))
}

async fn handle_connection(
    mut stream: tokio::net::TcpStream,
    client: Arc<P2PClient>,
    peer_id: PeerId,
) {
    // Read the full HTTP request (Ollama payloads are typically < 4 KB).
    let mut buf = vec![0u8; 4 * 1024 * 1024];
    let n = match tokio::time::timeout(
        std::time::Duration::from_secs(10),
        stream.read(&mut buf),
    )
    .await
    {
        Ok(Ok(n)) if n > 0 => n,
        _ => return,
    };

    let (method, path, body) = match parse_http_request(&buf[..n]) {
        Some(t) => t,
        None => {
            let _ = stream
                .write_all(b"HTTP/1.1 400 Bad Request\r\nContent-Length: 11\r\n\r\nBad Request")
                .await;
            return;
        }
    };

    let req_bytes = match rmp_serde::to_vec_named(&ProxyRequest { method, path, body }) {
        Ok(b) => b,
        Err(e) => {
            warn!("ollama_proxy: serialise: {e}");
            return;
        }
    };

    let resp_bytes = match client
        .call_unary_handler(&peer_id.to_bytes(), OLLAMA_PROXY_PROTO, &req_bytes)
        .await
    {
        Ok(b) => b,
        Err(e) => {
            warn!("ollama_proxy: P2P call to {peer_id}: {e}");
            let msg = b"Bad Gateway";
            let _ = stream
                .write_all(
                    format!("HTTP/1.1 502 Bad Gateway\r\nContent-Length: {}\r\n\r\n", msg.len())
                        .as_bytes(),
                )
                .await;
            let _ = stream.write_all(msg).await;
            return;
        }
    };

    let resp: ProxyResponse = match rmp_serde::from_slice(&resp_bytes) {
        Ok(r) => r,
        Err(e) => {
            warn!("ollama_proxy: deserialise resp: {e}");
            return;
        }
    };

    let header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        resp.status,
        status_text(resp.status),
        resp.body.len(),
    );
    let _ = stream.write_all(header.as_bytes()).await;
    let _ = stream.write_all(&resp.body).await;
}

/// Parse a raw HTTP/1.1 request into `(method, path, body)`.
///
/// Handles the typical `POST /v1/... HTTP/1.1` shape sent by reqwest.
/// Does not attempt to handle chunked encoding or multi-read bodies.
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

fn status_text(code: u16) -> &'static str {
    match code {
        200 => "OK",
        400 => "Bad Request",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        _ => "Unknown",
    }
}

// ── URL resolution ─────────────────────────────────────────────────────────────

/// Resolve a list of inference URLs, starting a local HTTP proxy for each
/// `p2p://PEER_ID` entry.
///
/// Returns `(resolved_urls, proxy_handles)`.  Keep the handles alive while
/// `resolved_urls` are in use — dropping them stops the proxy listeners.
pub async fn resolve_inference_urls(
    urls: &[String],
    client: &Arc<P2PClient>,
) -> Result<(Vec<String>, Vec<tokio::task::JoinHandle<()>>)> {
    let mut resolved = Vec::with_capacity(urls.len());
    let mut handles = Vec::new();

    for url in urls {
        if let Some(peer_str) = url.strip_prefix("p2p://") {
            let peer_id: PeerId = peer_str
                .parse()
                .with_context(|| format!("invalid PeerId in inference URL: {url}"))?;
            let (port, handle) = start_local_proxy(client.clone(), peer_id).await?;
            resolved.push(format!("http://127.0.0.1:{port}"));
            handles.push(handle);
            tracing::info!("ollama_proxy: {url} → http://127.0.0.1:{port}");
        } else {
            resolved.push(url.clone());
        }
    }

    Ok((resolved, handles))
}
