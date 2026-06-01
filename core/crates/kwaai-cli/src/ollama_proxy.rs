//! P2P proxy for HTTP inference — tunnels `/v1/chat/completions` and friends
//! through the KwaaiNet fabric so remote nodes can offload LLM inference.
//!
//! Two protocols are supported:
//!
//! * `/kwaai/ollama-proxy/1.0.0` — forwards to `localhost:11434` (Ollama).
//! * `/kwaai/shard-proxy/1.0.0` — forwards to the local shard API port
//!   (written to `~/.kwaainet/run/shard_api.port` by `kwaainet shard api`).
//!
//! When a caller resolves a `p2p://PEER_ID` URL, this module probes the remote
//! peer to see which protocol is available and picks the best one automatically.
//!
//! Message flow (client perspective):
//! ```text
//! extract_from_text  ──HTTP──▶  local TCP proxy  ──P2P──▶  remote node
//!                                                             │  shard api / Ollama
//!                              local TCP proxy  ◀──P2P──  ◀──┘
//! ```

use anyhow::{Context, Result};
use kwaai_p2p_daemon::P2PClient;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::{
    pin::Pin,
    sync::{Arc, OnceLock},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tracing::{debug, info, warn};

pub const OLLAMA_PROXY_PROTO: &str = "/kwaai/ollama-proxy/1.0.0";
pub const SHARD_PROXY_PROTO: &str = "/kwaai/shard-proxy/1.0.0";

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

static PROXY_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn proxy_client() -> &'static reqwest::Client {
    PROXY_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("build reqwest client")
    })
}

/// Build a unary handler that forwards incoming proxy requests to the local
/// Ollama at `localhost:11434`.
///
/// Register with `client.add_unary_handler(OLLAMA_PROXY_PROTO, handler, false)`.
#[allow(clippy::type_complexity)]
pub fn make_ollama_proxy_handler() -> impl Fn(
    Vec<u8>,
) -> Pin<
    Box<dyn std::future::Future<Output = kwaai_p2p_daemon::error::Result<Vec<u8>>> + Send>,
> + Send
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

            let client = proxy_client();
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
                    let body =
                        match tokio::time::timeout(std::time::Duration::from_secs(120), r.bytes())
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

/// Build a unary handler that forwards incoming proxy requests to the local
/// shard API port written by `kwaainet shard api` at startup.
///
/// Returns 503 immediately if the shard API is not running (port file absent).
/// Register with `client.add_unary_handler(SHARD_PROXY_PROTO, handler, false)`.
#[allow(clippy::type_complexity)]
pub fn make_shard_proxy_handler() -> impl Fn(
    Vec<u8>,
) -> Pin<
    Box<dyn std::future::Future<Output = kwaai_p2p_daemon::error::Result<Vec<u8>>> + Send>,
> + Send
       + Sync
       + 'static {
    move |data: Vec<u8>| {
        Box::pin(async move {
            let port = match std::fs::read_to_string(crate::shard_cmd::shard_api_port_file())
                .ok()
                .and_then(|s| s.trim().parse::<u16>().ok())
            {
                Some(p) => p,
                None => return encode_err(503, "shard api not running"),
            };

            let req: ProxyRequest = match rmp_serde::from_slice(&data) {
                Ok(r) => r,
                Err(e) => {
                    warn!("shard_proxy server: bad request: {e}");
                    return encode_err(400, &format!("bad request: {e}"));
                }
            };

            debug!("shard_proxy server: {} {}", req.method, req.path);

            let client = proxy_client();
            let url = format!("http://127.0.0.1:{port}{}", req.path);
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
                    let body =
                        match tokio::time::timeout(std::time::Duration::from_secs(120), r.bytes())
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

fn encode_err(status: u16, msg: &str) -> kwaai_p2p_daemon::error::Result<Vec<u8>> {
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
    start_proxy_with_proto(client, peer_id, OLLAMA_PROXY_PROTO).await
}

/// Start a local TCP listener that proxies HTTP → P2P → remote shard API.
///
/// Returns `(local_port, join_handle)`.  Drop the handle to stop the proxy.
pub async fn start_local_shard_proxy(
    client: Arc<P2PClient>,
    peer_id: PeerId,
) -> Result<(u16, tokio::task::JoinHandle<()>)> {
    start_proxy_with_proto(client, peer_id, SHARD_PROXY_PROTO).await
}

async fn start_proxy_with_proto(
    client: Arc<P2PClient>,
    peer_id: PeerId,
    protocol: &'static str,
) -> Result<(u16, tokio::task::JoinHandle<()>)> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind local inference proxy")?;
    let port = listener.local_addr()?.port();

    let handle = tokio::spawn(async move {
        loop {
            let (stream, _) = match listener.accept().await {
                Ok(s) => s,
                Err(e) => {
                    warn!("inference_proxy local accept: {e}");
                    break;
                }
            };
            let client = client.clone();
            tokio::spawn(handle_connection(stream, client, peer_id, protocol));
        }
    });

    Ok((port, handle))
}

async fn handle_connection(
    mut stream: tokio::net::TcpStream,
    client: Arc<P2PClient>,
    peer_id: PeerId,
    protocol: &'static str,
) {
    // Read the full HTTP request in two phases so that large LLM prompts
    // (~20-50 KB) delivered over a relay connection are not silently truncated
    // by a single read() that only captures the first TCP segment.
    //
    // Phase 1 — accumulate until the header/body separator (\r\n\r\n) arrives.
    // Phase 2 — read the remaining body bytes indicated by Content-Length.
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
            Ok(Ok(0)) | Err(_) => break, // EOF or timeout — use what we have
            Ok(Ok(n)) => buf.extend_from_slice(&tmp[..n]),
            Ok(Err(_)) => return,
        }
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

    let req_bytes = match rmp_serde::to_vec_named(&ProxyRequest { method, path, body }) {
        Ok(b) => b,
        Err(e) => {
            warn!("ollama_proxy: serialise: {e}");
            return;
        }
    };

    let resp_bytes = match client
        .call_unary_handler(&peer_id.to_bytes(), protocol, &req_bytes)
        .await
    {
        Ok(b) => b,
        Err(e) => {
            warn!("inference_proxy: P2P call to {peer_id} via {protocol}: {e}");
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
/// Caller guarantees `raw` contains the full request (headers + body).
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

/// Probe whether the remote peer has `/kwaai/shard-proxy/1.0.0` available and
/// its shard API running (returns HTTP 200 for GET /v1/models).
/// Returns `true` if the shard proxy should be used, `false` to fall back to Ollama.
async fn probe_shard_proxy(client: &Arc<P2PClient>, peer_id: PeerId) -> bool {
    let probe = ProxyRequest {
        method: "GET".to_string(),
        path: "/v1/models".to_string(),
        body: vec![],
    };
    let probe_bytes = match rmp_serde::to_vec_named(&probe) {
        Ok(b) => b,
        Err(_) => return false,
    };
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        client.call_unary_handler(&peer_id.to_bytes(), SHARD_PROXY_PROTO, &probe_bytes),
    )
    .await;
    match result {
        Ok(Ok(resp_bytes)) => rmp_serde::from_slice::<ProxyResponse>(&resp_bytes)
            .map(|r| r.status == 200)
            .unwrap_or(false),
        _ => false,
    }
}

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
        if let Some(peer_str) = url.strip_prefix("mux://") {
            let peer_id: PeerId = peer_str
                .parse()
                .with_context(|| format!("invalid PeerId in mux:// URL: {url}"))?;
            let (port, handle) = crate::inference_mux::start_local_mux_proxy(peer_id).await?;
            resolved.push(format!("http://127.0.0.1:{port}"));
            handles.push(handle);
            info!("inference_proxy: {url} → http://127.0.0.1:{port} (via inference-mux)");
        } else if let Some(peer_str) = url.strip_prefix("p2p://") {
            let peer_id: PeerId = peer_str
                .parse()
                .with_context(|| format!("invalid PeerId in inference URL: {url}"))?;

            // Probe whether the remote peer has a shard API running.
            // Prefer shard-proxy (no Ollama needed); fall back to ollama-proxy.
            let shard_available = probe_shard_proxy(client, peer_id).await;
            let (proto_name, (port, handle)) = if shard_available {
                (
                    "shard-proxy",
                    start_local_shard_proxy(client.clone(), peer_id).await?,
                )
            } else {
                (
                    "ollama-proxy",
                    start_local_proxy(client.clone(), peer_id).await?,
                )
            };

            resolved.push(format!("http://127.0.0.1:{port}"));
            handles.push(handle);
            info!("inference_proxy: {url} → http://127.0.0.1:{port} (via {proto_name})");
        } else {
            resolved.push(url.clone());
        }
    }

    Ok((resolved, handles))
}
