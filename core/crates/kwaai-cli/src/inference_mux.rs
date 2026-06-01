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
use kwaai_p2p_daemon::{P2PClient, P2PStream, DEFAULT_SOCKET_NAME};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, OnceLock,
    },
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{mpsc, oneshot, Mutex, RwLock},
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
    reader
        .read_exact(&mut buf)
        .await
        .context("read frame body")?;
    Ok(buf)
}

// ── Protocol helpers ──────────────────────────────────────────────────────────

/// Read and discard the gogo-protobuf delimited `StreamInfo` message that
/// go-libp2p-daemon sends to a registered stream handler before piping data.
///
/// Wire format: varint(len) || proto_bytes  (same varint encoding as protobuf).
async fn read_p2pd_stream_info<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<()> {
    // Decode varint length prefix (≤ 10 bytes for u64).
    let mut len: u64 = 0;
    let mut shift = 0u32;
    for _ in 0..10 {
        let mut b = [0u8; 1];
        reader
            .read_exact(&mut b)
            .await
            .context("read p2pd StreamInfo varint")?;
        len |= ((b[0] & 0x7F) as u64) << shift;
        if b[0] & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift > 63 {
            anyhow::bail!("p2pd StreamInfo varint overflow");
        }
    }
    // Read and discard the message body.
    let mut buf = vec![0u8; len as usize];
    reader
        .read_exact(&mut buf)
        .await
        .context("read p2pd StreamInfo body")?;
    Ok(())
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

    // go-libp2p-daemon sends a gogo-protobuf StreamInfo message before piping data.
    // Consume it before entering the mux frame loop.
    if let Err(e) = read_p2pd_stream_info(&mut reader).await {
        warn!("inference-mux server: failed to read p2pd StreamInfo prologue: {e}");
        return;
    }
    debug!("inference-mux server: StreamInfo prologue consumed — entering mux frame loop");

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

static OLLAMA_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn ollama_client() -> &'static reqwest::Client {
    OLLAMA_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("build reqwest client")
    })
}

async fn call_ollama_local(req: &MuxRequest) -> MuxResponse {
    let client = ollama_client();

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
    next_id: AtomicU64,
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<MuxResponse>>>>,
    tx: mpsc::Sender<Vec<u8>>,
    /// Set to true when the underlying stream dies. Checked in send() to fail
    /// fast instead of hanging on a oneshot that will never be resolved.
    dead: Arc<AtomicBool>,
}

impl InferenceMuxClient {
    pub fn is_dead(&self) -> bool {
        self.dead.load(Ordering::Acquire)
    }

    /// Open a stream to `peer_id` and start background I/O tasks.
    pub async fn connect(peer_id: PeerId) -> Result<Arc<Self>> {
        let sock =
            std::env::var("KWAAINET_SOCKET").unwrap_or_else(|_| DEFAULT_SOCKET_NAME.to_string());
        #[cfg(unix)]
        let addr = format!("/unix/{sock}");
        #[cfg(not(unix))]
        let addr = "/ip4/127.0.0.1/tcp/5005".to_string();

        let p2p = P2PClient::connect(&addr)
            .await
            .context("connect to p2pd for inference-mux stream")?;

        // stream_open_raw consumes the P2PClient and returns the daemon socket as the data
        // channel. The go-libp2p-daemon pipes the libp2p stream on the same socket after
        // sending StreamInfo — no separate TCP connection is needed or correct.
        let raw: P2PStream = p2p
            .stream_open_raw(&peer_id.to_bytes(), vec![MUX_PROTO.to_string()])
            .await
            .map_err(|e| {
                warn!(
                    "inference-mux stream_open_raw failed for peer {}: {:#}",
                    peer_id.to_base58(),
                    e
                );
                e
            })
            .context("stream_open_raw for inference-mux")?;

        let (mut reader, writer) = tokio::io::split(raw);
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(256);

        let pending: Arc<Mutex<HashMap<u64, oneshot::Sender<MuxResponse>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        let dead = Arc::new(AtomicBool::new(false));

        // Writer task: drains the send channel and writes frames to the stream.
        // On exit, marks dead and drains pending so in-flight send() calls
        // immediately receive an error rather than hanging forever.
        let pending_w = pending.clone();
        let dead_w = dead.clone();
        tokio::spawn(async move {
            let mut writer = writer;
            while let Some(payload) = rx.recv().await {
                if let Err(e) = write_frame(&mut writer, &payload).await {
                    warn!("inference-mux client: writer error: {e}");
                    break;
                }
            }
            dead_w.store(true, Ordering::Release);
            pending_w.lock().await.clear();
        });

        // Reader task: reads response frames and routes them to waiting callers.
        // On exit, marks dead and drains pending.
        let pending_rx = pending.clone();
        let dead_r = dead.clone();
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
            dead_r.store(true, Ordering::Release);
            pending_rx.lock().await.clear();
        });

        Ok(Arc::new(Self {
            next_id: AtomicU64::new(1),
            pending,
            tx,
            dead,
        }))
    }

    /// Send one inference request and await the response.
    /// Multiple concurrent callers share the same underlying stream.
    pub async fn send(&self, method: &str, path: &str, body: Vec<u8>) -> Result<MuxResponse> {
        if self.dead.load(Ordering::Acquire) {
            return Err(anyhow::anyhow!("inference-mux client: stream disconnected"));
        }

        let request_id = self.next_id.fetch_add(1, Ordering::Relaxed);
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

        // 120s timeout guards the rare race where the stream dies between the
        // dead-check above and this await. Normally the dead+drain path fires
        // the oneshot error in microseconds, not seconds.
        tokio::time::timeout(std::time::Duration::from_secs(120), resp_rx)
            .await
            .context("inference-mux response timeout")?
            .context("inference-mux response channel closed")
    }
}

// ── Local HTTP shim ───────────────────────────────────────────────────────────

// None = not yet connected; lazily opened on first request.
type SharedMuxClient = Arc<RwLock<Option<Arc<InferenceMuxClient>>>>;

/// Start a local HTTP proxy that routes all requests through a shared
/// `InferenceMuxClient` to the remote GPU node.
///
/// The stream to the remote peer is opened lazily on the first request,
/// avoiding an idle connection that the relay would drop before inference starts.
/// The proxy reconnects automatically whenever the stream dies.
///
/// Returns `(local_port, join_handle)`. Drop the handle to stop the proxy.
pub async fn start_local_mux_proxy(peer_id: PeerId) -> Result<(u16, JoinHandle<()>)> {
    let shared: SharedMuxClient = Arc::new(RwLock::new(None));

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind inference-mux local proxy")?;
    let port = listener.local_addr()?.port();

    info!(
        "inference-mux: local proxy on 127.0.0.1:{port} → mux://{peer_id} (lazy connect)",
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
            let shared = shared.clone();
            tokio::spawn(handle_mux_proxy_connection(stream, shared, peer_id));
        }
    });

    Ok((port, handle))
}

/// Return the current live client, connecting or reconnecting as needed.
/// Double-checked locking: fast read-lock path, slow write-lock path only when needed.
async fn ensure_mux_client(
    shared: &SharedMuxClient,
    peer_id: PeerId,
) -> Result<Arc<InferenceMuxClient>> {
    {
        let g = shared.read().await;
        if let Some(c) = g.as_ref() {
            if !c.is_dead() {
                return Ok(c.clone());
            }
        }
    }
    let mut g = shared.write().await;
    if let Some(c) = g.as_ref() {
        if !c.is_dead() {
            return Ok(c.clone());
        }
    }
    info!("inference-mux: (re)connecting to {}", peer_id.to_base58());
    let new_client = InferenceMuxClient::connect(peer_id).await?;
    *g = Some(new_client.clone());
    Ok(new_client)
}

/// Parse one HTTP request from a worker, forward via mux, write HTTP response back.
/// Connects lazily on first call; reconnects automatically on any stream failure.
async fn handle_mux_proxy_connection(
    mut stream: TcpStream,
    shared: SharedMuxClient,
    peer_id: PeerId,
) {
    // Two-phase read — same approach as ollama_proxy::handle_connection.
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
            Ok(Ok(0)) | Err(_) => break,
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

    let resp = 'send: {
        for attempt in 0u32..2 {
            let client = match ensure_mux_client(&shared, peer_id).await {
                Ok(c) => c,
                Err(e) => {
                    warn!("inference-mux: connect failed (attempt {attempt}): {e:#}");
                    if attempt < 1 {
                        continue;
                    }
                    break 'send Err(e);
                }
            };
            match client.send(&method, &path, body.clone()).await {
                Ok(r) => break 'send Ok(r),
                Err(e) => {
                    warn!("inference-mux: send failed (attempt {attempt}): {e}");
                    if attempt == 0 {
                        // Invalidate so ensure_mux_client reconnects next iteration.
                        let mut g = shared.write().await;
                        if let Some(c) = g.as_ref() {
                            if Arc::ptr_eq(c, &client) {
                                *g = None;
                            }
                        }
                    } else {
                        break 'send Err(e);
                    }
                }
            }
        }
        Err(anyhow::anyhow!(
            "inference-mux: all retry attempts exhausted"
        ))
    };

    let resp = match resp {
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
