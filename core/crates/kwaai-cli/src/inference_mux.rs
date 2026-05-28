//! Concurrent GPU inference multiplexer over a persistent p2p stream.
//!
//! Solves the IPC serialization bottleneck in the existing `ollama_proxy`:
//! each unary-RPC call blocks while waiting for Ollama, so N workers
//! using `p2p://PEER_ID` only get ~1 Ollama slot used at a time.
//!
//! This module opens ONE yamux stream to the remote GPU node and sends
//! multiple request frames over it concurrently, correlated by `request_id`.
//! The server side spawns a tokio task per request, saturating
//! `OLLAMA_NUM_PARALLEL` slots simultaneously.
//!
//! ## Wire format
//! Each frame: `[4-byte LE length][msgpack(MuxRequest | MuxResponse)]`
//!
//! ## URL scheme
//! Use `mux://PEER_ID` in `--inference-urls` to activate.

use anyhow::{Context, Result};
use kwaai_p2p_daemon::{P2PClient, DEFAULT_SOCKET_NAME};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{mpsc, oneshot, Mutex},
    task::JoinHandle,
};
use tracing::{debug, info, warn};

pub const MUX_PROTO: &str = "/kwaai/inference-mux/1.0.0";

// ── Wire types ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct MuxRequest {
    pub request_id: u64,
    pub method: String,
    pub path: String,
    pub body: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MuxResponse {
    pub request_id: u64,
    pub status: u16,
    pub body: Vec<u8>,
}

// ── Frame I/O ─────────────────────────────────────────────────────────────────

async fn write_frame<W: AsyncWriteExt + Unpin>(writer: &mut W, payload: &[u8]) -> Result<()> {
    let len = payload.len() as u32;
    writer.write_all(&len.to_le_bytes()).await?;
    writer.write_all(payload).await?;
    Ok(())
}

async fn read_frame<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<Vec<u8>> {
    let mut len_buf = [0u8; 4];
    reader
        .read_exact(&mut len_buf)
        .await
        .context("read frame length")?;
    let len = u32::from_le_bytes(len_buf) as usize;
    if len > 64 * 1024 * 1024 {
        anyhow::bail!("inference-mux frame too large: {len} bytes");
    }
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).await.context("read frame body")?;
    Ok(buf)
}

// ── Server ────────────────────────────────────────────────────────────────────

/// Start the inference-mux server: binds a local TCP port, registers it with
/// the daemon as the handler for `MUX_PROTO`, and spawns an accept loop.
///
/// Call from `cmd_shard_serve()` alongside the unary proxy handlers.
pub async fn start_inference_mux_server(client: &mut P2PClient) -> Result<JoinHandle<()>> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind inference-mux server")?;
    let port = listener.local_addr()?.port();
    let addr = format!("/ip4/127.0.0.1/tcp/{port}");

    client
        .register_stream_handler(&addr, vec![MUX_PROTO.to_string()])
        .await
        .context("register inference-mux stream handler")?;

    info!("inference-mux: listening on {addr}, registered as {MUX_PROTO}");

    Ok(tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, peer)) => {
                    debug!("inference-mux server: accepted connection from {peer}");
                    tokio::spawn(handle_mux_stream_server(stream));
                }
                Err(e) => {
                    warn!("inference-mux server accept error: {e}");
                    break;
                }
            }
        }
    }))
}

/// Handle one connected client stream — reads MuxRequest frames concurrently,
/// calls local Ollama for each, writes MuxResponse frames back in any order.
async fn handle_mux_stream_server(stream: TcpStream) {
    let (mut reader, writer) = stream.into_split();
    let writer = Arc::new(Mutex::new(writer));

    loop {
        let frame = match read_frame(&mut reader).await {
            Ok(f) => f,
            Err(e) => {
                debug!("inference-mux server: stream closed ({e})");
                break;
            }
        };

        let req: MuxRequest = match rmp_serde::from_slice(&frame) {
            Ok(r) => r,
            Err(e) => {
                warn!("inference-mux server: bad MuxRequest: {e}");
                continue;
            }
        };

        let writer = writer.clone();
        tokio::spawn(async move {
            let resp = call_ollama_local(&req).await;
            match rmp_serde::to_vec_named(&resp) {
                Ok(payload) => {
                    let mut w = writer.lock().await;
                    if let Err(e) = write_frame(&mut *w, &payload).await {
                        warn!("inference-mux server: write response: {e}");
                    }
                }
                Err(e) => warn!("inference-mux server: encode response: {e}"),
            }
        });
    }
}

async fn call_ollama_local(req: &MuxRequest) -> MuxResponse {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return MuxResponse {
                request_id: req.request_id,
                status: 503,
                body: format!("reqwest init: {e}").into_bytes(),
            }
        }
    };

    let url = format!("http://127.0.0.1:11434{}", req.path);
    let method: reqwest::Method = req.method.parse().unwrap_or(reqwest::Method::POST);

    let result = client
        .request(method, &url)
        .header("Content-Type", "application/json")
        .body(req.body.clone())
        .send()
        .await;

    match result {
        Ok(r) => {
            let status = r.status().as_u16();
            let body =
                match tokio::time::timeout(std::time::Duration::from_secs(120), r.bytes()).await {
                    Ok(Ok(b)) => b.to_vec(),
                    Ok(Err(e)) => format!("body error: {e}").into_bytes(),
                    Err(_) => b"body timeout".to_vec(),
                };
            MuxResponse {
                request_id: req.request_id,
                status,
                body,
            }
        }
        Err(e) => MuxResponse {
            request_id: req.request_id,
            status: 503,
            body: format!("upstream: {e}").into_bytes(),
        },
    }
}

// ── Client ────────────────────────────────────────────────────────────────────

/// Shared client that multiplexes N concurrent inference requests over one
/// persistent yamux stream to a remote GPU node.
pub struct InferenceMuxClient {
    next_id: std::sync::atomic::AtomicU64,
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<MuxResponse>>>>,
    tx: mpsc::Sender<Vec<u8>>,
}

impl InferenceMuxClient {
    /// Open a stream to `peer_id` and start background I/O tasks.
    ///
    /// Creates a fresh P2PClient connection just for `stream_open()` (cheap
    /// Unix socket call); the TcpStream lives on independently afterward.
    pub async fn connect(peer_id: PeerId) -> Result<Arc<Self>> {
        let mut p2p = P2PClient::connect(DEFAULT_SOCKET_NAME)
            .await
            .context("connect to p2pd for inference-mux stream")?;

        let stream = p2p
            .stream_open(&peer_id.to_bytes(), vec![MUX_PROTO.to_string()])
            .await
            .context("stream_open for inference-mux")?;

        let (mut reader, writer) = stream.into_split();
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(256);

        let pending: Arc<Mutex<HashMap<u64, oneshot::Sender<MuxResponse>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        // Writer task: drains the send channel and writes frames to the stream.
        tokio::spawn(async move {
            let mut writer = writer;
            while let Some(payload) = rx.recv().await {
                if let Err(e) = write_frame(&mut writer, &payload).await {
                    warn!("inference-mux client: writer error: {e}");
                    break;
                }
            }
        });

        // Reader task: reads response frames and routes them to waiting callers.
        let pending_rx = pending.clone();
        tokio::spawn(async move {
            loop {
                let frame = match read_frame(&mut reader).await {
                    Ok(f) => f,
                    Err(e) => {
                        debug!("inference-mux client: reader closed ({e})");
                        break;
                    }
                };
                let resp: MuxResponse = match rmp_serde::from_slice(&frame) {
                    Ok(r) => r,
                    Err(e) => {
                        warn!("inference-mux client: bad MuxResponse: {e}");
                        continue;
                    }
                };
                let sender = pending_rx.lock().await.remove(&resp.request_id);
                if let Some(s) = sender {
                    let _ = s.send(resp);
                }
            }
        });

        Ok(Arc::new(Self {
            next_id: std::sync::atomic::AtomicU64::new(1),
            pending,
            tx,
        }))
    }

    /// Send one inference request and await the response.
    /// Multiple concurrent callers share the same underlying stream.
    pub async fn send(
        &self,
        method: &str,
        path: &str,
        body: Vec<u8>,
    ) -> Result<MuxResponse> {
        let request_id = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let (resp_tx, resp_rx) = oneshot::channel();
        self.pending.lock().await.insert(request_id, resp_tx);

        let req = MuxRequest {
            request_id,
            method: method.to_string(),
            path: path.to_string(),
            body,
        };
        let payload = rmp_serde::to_vec_named(&req).context("encode MuxRequest")?;
        self.tx
            .send(payload)
            .await
            .map_err(|_| anyhow::anyhow!("inference-mux send channel closed"))?;

        resp_rx
            .await
            .context("inference-mux response channel closed")
    }
}

// ── Local HTTP shim ───────────────────────────────────────────────────────────

/// Start a local HTTP proxy that routes all requests through a shared
/// `InferenceMuxClient` to the remote GPU node.
///
/// Returns `(local_port, join_handle)`. Drop the handle to stop the proxy.
pub async fn start_local_mux_proxy(
    peer_id: PeerId,
) -> Result<(u16, JoinHandle<()>)> {
    let client = InferenceMuxClient::connect(peer_id).await?;

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind inference-mux local proxy")?;
    let port = listener.local_addr()?.port();

    info!(
        "inference-mux: local proxy on 127.0.0.1:{port} → mux://{peer_id}",
        peer_id = peer_id.to_base58()
    );

    let handle = tokio::spawn(async move {
        loop {
            let (stream, _) = match listener.accept().await {
                Ok(s) => s,
                Err(e) => {
                    warn!("inference-mux proxy accept: {e}");
                    break;
                }
            };
            let client = client.clone();
            tokio::spawn(handle_mux_proxy_connection(stream, client));
        }
    });

    Ok((port, handle))
}

/// Parse one HTTP request from a worker, forward via mux, write HTTP response back.
async fn handle_mux_proxy_connection(mut stream: TcpStream, client: Arc<InferenceMuxClient>) {
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

    let resp = match client.send(&method, &path, body).await {
        Ok(r) => r,
        Err(e) => {
            warn!("inference-mux proxy: send failed: {e}");
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

    let status_text = match resp.status {
        200 => "OK",
        400 => "Bad Request",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        _ => "Unknown",
    };
    let header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        resp.status,
        status_text,
        resp.body.len(),
    );
    let _ = stream.write_all(header.as_bytes()).await;
    let _ = stream.write_all(&resp.body).await;
}

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
