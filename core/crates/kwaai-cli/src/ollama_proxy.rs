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

use crate::capacity_lease::{LeaseId, LeaseTable};
use crate::circuit_breaker::CircuitBreaker;

pub const OLLAMA_PROXY_PROTO: &str = "/kwaai/ollama-proxy/1.0.0";
pub const SHARD_PROXY_PROTO: &str = "/kwaai/shard-proxy/1.0.0";

// ── Wire types ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyRequest {
    pub method: String,
    pub path: String,
    pub body: Vec<u8>,
    /// Attached once `resolve_inference_urls()` has negotiated a Capacity
    /// Lease for this peer (see `capacity_lease.rs`). `None` for legacy
    /// servers, denied/unsupported negotiations, or the shard-proxy path
    /// (which doesn't dispatch to Ollama and has no admission gate) — the
    /// server forwards unconditionally in that case, exactly as before this
    /// feature existed.
    #[serde(default)]
    pub lease_id: Option<LeaseId>,
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
/// `lease_table` is the SAME Capacity Lease admission gate shared with the
/// inference-mux server (see `node.rs`/`shard_cmd.rs`) — both protocols
/// dispatch to this same local Ollama instance. A request carrying a
/// `lease_id` is renewed against the table before forwarding; an unknown or
/// expired lease is denied without ever reaching Ollama. A request with no
/// `lease_id` (legacy caller, or a negotiation that was denied/unsupported)
/// forwards unconditionally, exactly as before this feature existed.
///
/// Register with `client.add_unary_handler(OLLAMA_PROXY_PROTO, handler, false)`.
#[allow(clippy::type_complexity)]
pub fn make_ollama_proxy_handler(
    lease_table: Arc<LeaseTable>,
) -> impl Fn(
    Vec<u8>,
) -> Pin<
    Box<dyn std::future::Future<Output = kwaai_p2p_daemon::error::Result<Vec<u8>>> + Send>,
> + Send
       + Sync
       + 'static {
    move |data: Vec<u8>| {
        let lease_table = lease_table.clone();
        Box::pin(async move {
            let req: ProxyRequest = match rmp_serde::from_slice(&data) {
                Ok(r) => r,
                Err(e) => {
                    warn!("ollama_proxy server: bad request: {e}");
                    return encode_err(400, &format!("bad request: {e}"));
                }
            };

            debug!("ollama_proxy server: {} {}", req.method, req.path);

            if let Some(lease_id) = req.lease_id {
                let ttl = std::time::Duration::from_secs(crate::capacity_lease::LEASE_TTL_SECS);
                if !lease_table.renew(lease_id, ttl) {
                    return encode_err(409, "capacity lease expired or unknown");
                }
            }

            let client = proxy_client();
            let url = format!("http://localhost:11434{}", req.path);
            let method: reqwest::Method = req.method.parse().unwrap_or(reqwest::Method::POST);

            let result = client
                .request(method, &url)
                .header("Content-Type", "application/json")
                .body(req.body)
                .send()
                .await;

            // Pull and generate/chat can run for many minutes — use a generous timeout.
            let body_timeout_secs = if req.path.contains("/api/pull") {
                7200
            } else {
                600
            };

            let (status, body) = match result {
                Ok(r) => {
                    let status = r.status().as_u16();
                    let body = match tokio::time::timeout(
                        std::time::Duration::from_secs(body_timeout_secs),
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
/// `lease_id` is the Capacity Lease negotiated once by the caller (see
/// `resolve_inference_urls()`) before this proxy starts — `None` when
/// negotiation was denied, unsupported, or skipped; every request through
/// this proxy for the life of the returned handle carries the same
/// `lease_id`, renewed implicitly by the server on each one.
///
/// Returns `(local_port, join_handle)`.  Drop the handle to stop the proxy.
pub async fn start_local_proxy(
    client: Arc<P2PClient>,
    peer_id: PeerId,
    breaker: Arc<CircuitBreaker>,
    lease_id: Option<LeaseId>,
) -> Result<(u16, tokio::task::JoinHandle<()>)> {
    // Wrapped in a `ProxyLease` so a lapse can be replaced in flight; see its
    // docs for why a frozen id silently loses work.
    let lease = Some(ProxyLease::new(lease_id));
    start_proxy_with_proto(client, peer_id, OLLAMA_PROXY_PROTO, breaker, lease).await
}

/// Start a local TCP listener that proxies HTTP → P2P → remote shard API.
///
/// The shard API is a different local resource than Ollama and has no
/// Capacity Lease admission gate — requests through this proxy never carry
/// a `lease_id`.
///
/// Returns `(local_port, join_handle)`.  Drop the handle to stop the proxy.
pub async fn start_local_shard_proxy(
    client: Arc<P2PClient>,
    peer_id: PeerId,
    breaker: Arc<CircuitBreaker>,
) -> Result<(u16, tokio::task::JoinHandle<()>)> {
    start_proxy_with_proto(client, peer_id, SHARD_PROXY_PROTO, breaker, None).await
}

async fn start_proxy_with_proto(
    client: Arc<P2PClient>,
    peer_id: PeerId,
    protocol: &'static str,
    breaker: Arc<CircuitBreaker>,
    lease: Option<Arc<ProxyLease>>,
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
            let breaker = breaker.clone();
            let lease = lease.clone();
            tokio::spawn(handle_connection(
                stream, client, peer_id, protocol, breaker, lease,
            ));
        }
    });

    Ok((port, handle))
}

/// The Capacity Lease this proxy dispatches under, shared by every connection
/// and **replaceable**.
///
/// It used to be a bare `Option<LeaseId>` copied into each connection, frozen
/// for the life of the proxy. The server renews a lease only when a request
/// arrives under it, and `LEASE_TTL_SECS` is 30 — so a caller whose *own*
/// inference calls take longer than 30s lets the lease lapse between requests
/// and every later request is refused 409. Measured on a distributed D6 graph
/// build: ~26s mean per call, 38 of 200 chunks lost, and all 38 burned their
/// full three retries without one recovery, because the retry re-sent the same
/// dead `lease_id` on a schedule (immediate, +15s, +30s) that only widened the
/// gap it was failing on.
///
/// `mux://` has never had this problem — `inference_mux` holds a [`LeaseHolder`]
/// and sends a `LeaseKeepalive` frame when `needs_keepalive_probe()` says the
/// lease is going stale. That machinery simply was never wired to the `p2p://`
/// path, which is the transport every RAG pipeline actually uses.
///
/// [`LeaseHolder`]: crate::capacity_lease::LeaseHolder
struct ProxyLease {
    current: tokio::sync::RwLock<Option<LeaseId>>,
}

impl ProxyLease {
    fn new(lease_id: Option<LeaseId>) -> Arc<Self> {
        Arc::new(Self {
            current: tokio::sync::RwLock::new(lease_id),
        })
    }

    async fn get(&self) -> Option<LeaseId> {
        *self.current.read().await
    }

    /// Negotiate a replacement after the peer refused the current one.
    ///
    /// Held across the negotiation so a burst of concurrent 409s produces one
    /// new lease rather than one per connection; whoever loses the race sees
    /// the winner's lease and retries under it.
    async fn renegotiate(
        &self,
        client: &P2PClient,
        peer_id: PeerId,
        breaker: &CircuitBreaker,
        refused: Option<LeaseId>,
    ) -> Option<LeaseId> {
        let mut slot = self.current.write().await;
        if *slot != refused {
            return *slot; // someone else already replaced it
        }
        // `model: None` for the same reason the initial negotiation uses it —
        // the target model lives in the request body, not in the URL.
        match crate::capacity_lease::negotiate_lease_unary(client, peer_id, None, breaker).await {
            crate::capacity_lease::NegotiationOutcome::Granted(grant) => {
                debug!(%peer_id, lease_id = grant.lease_id, "capacity lease renewed after 409");
                *slot = Some(grant.lease_id);
                *slot
            }
            other => {
                warn!(%peer_id, ?other, "capacity lease could not be renewed after 409");
                *slot = None; // fall through to uncontracted dispatch
                None
            }
        }
    }
}

async fn handle_connection(
    mut stream: tokio::net::TcpStream,
    client: Arc<P2PClient>,
    peer_id: PeerId,
    protocol: &'static str,
    breaker: Arc<CircuitBreaker>,
    lease: Option<Arc<ProxyLease>>,
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

    // Circuit breaker check: fail fast if this peer has had too many
    // consecutive connection/timeout failures recently.
    if !breaker.allow(&peer_id) {
        let msg = b"Circuit open - peer temporarily unavailable";
        let _ = stream
            .write_all(
                format!(
                    "HTTP/1.1 503 Service Unavailable\r\nContent-Length: {}\r\nRetry-After: 30\r\n\r\n",
                    msg.len()
                )
                .as_bytes(),
            )
            .await;
        let _ = stream.write_all(msg).await;
        return;
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

    // One round trip under a given lease. Kept as a closure so the 409 retry
    // below re-sends the *same* request under a fresh lease rather than
    // reconstructing it — the body is the expensive part and must not change.
    let send = |lease_id: Option<LeaseId>| {
        let client = client.clone();
        let method = method.clone();
        let path = path.clone();
        let body = body.clone();
        async move {
            let req_bytes = rmp_serde::to_vec_named(&ProxyRequest {
                method,
                path,
                body,
                lease_id,
            })
            .map_err(|e| format!("serialise: {e}"))?;
            client
                .call_unary_handler(&peer_id.to_bytes(), protocol, &req_bytes)
                .await
                .map_err(|e| format!("P2P call to {peer_id} via {protocol}: {e}"))
        }
    };

    let lease_in_use = match &lease {
        Some(l) => l.get().await,
        None => None,
    };

    let resp_bytes = match send(lease_in_use).await {
        Ok(b) => b,
        Err(e) => {
            warn!("inference_proxy: {e}");
            breaker.record_failure(&peer_id);
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
    breaker.record_success(&peer_id);

    let mut resp: ProxyResponse = match rmp_serde::from_slice(&resp_bytes) {
        Ok(r) => r,
        Err(e) => {
            warn!("ollama_proxy: deserialise resp: {e}");
            return;
        }
    };

    // 409 means the peer refused this lease, not that the work failed. Renew
    // and re-send once. Retrying under the refused lease — which is what the
    // caller's generic retry loop does — can never succeed, so this has to be
    // handled here, where the lease actually lives.
    if resp.status == 409 && lease_in_use.is_some() {
        if let Some(l) = &lease {
            if let Some(fresh) = l
                .renegotiate(&client, peer_id, &breaker, lease_in_use)
                .await
            {
                match send(Some(fresh)).await {
                    Ok(b) => match rmp_serde::from_slice::<ProxyResponse>(&b) {
                        Ok(r) => resp = r,
                        Err(e) => warn!("ollama_proxy: deserialise retry resp: {e}"),
                    },
                    Err(e) => warn!("inference_proxy: retry after lease renewal: {e}"),
                }
            }
        }
    }

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
        lease_id: None,
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

    // One circuit breaker shared across all peers in this session.
    // Keyed by PeerId so each peer trips/resets independently.
    let breaker = CircuitBreaker::new();

    for url in urls {
        if let Some(peer_str) = url.strip_prefix("mux://") {
            let peer_id: PeerId = peer_str
                .parse()
                .with_context(|| format!("invalid PeerId in mux:// URL: {url}"))?;
            let (port, handle) =
                crate::inference_mux::start_local_mux_proxy(peer_id, breaker.clone()).await?;
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
                    start_local_shard_proxy(client.clone(), peer_id, breaker.clone()).await?,
                )
            } else {
                // Negotiate a Capacity Lease once per URL, before starting
                // the local proxy — not once per request. `model: None`
                // because this function has no visibility into which model
                // a forwarded request will target (that's embedded in the
                // JSON body, not known at resolve time); a peer that hasn't
                // registered the lease protocol at all (legacy, or simply
                // unreachable right now) falls through to today's
                // uncontracted dispatch via `lease_id: None`.
                let outcome =
                    crate::capacity_lease::negotiate_lease_unary(client, peer_id, None, &breaker)
                        .await;
                let lease_id = match outcome {
                    crate::capacity_lease::NegotiationOutcome::Granted(grant) => {
                        Some(grant.lease_id)
                    }
                    _ => None,
                };
                (
                    "ollama-proxy",
                    start_local_proxy(client.clone(), peer_id, breaker.clone(), lease_id).await?,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Regression for the 2026-08-11 distributed-build data loss.
    ///
    /// The server renews a lease only when a request arrives under it, with a
    /// 30s TTL. A caller whose inference calls take longer than that lets the
    /// lease lapse *between* its own requests, and every later request is
    /// refused — 38 of 200 chunks lost on a D6 graph build, every one of them
    /// burning all three retries because the retry re-sent the same dead id.
    ///
    /// This pins the two halves of the remedy: a lapsed lease really is
    /// refused (so the 409 is not spurious), and a freshly granted one really
    /// is accepted (so renegotiating is the fix, where retrying is not).
    #[tokio::test]
    async fn a_lapsed_lease_is_refused_but_a_renewed_one_is_accepted() {
        use crate::capacity_lease::LeaseTable;
        use std::time::Duration;

        let table = LeaseTable::new("test-model".to_string(), 4);
        let grant = match table.try_grant(None, 1u64, Duration::from_millis(40)) {
            crate::capacity_lease::NegotiationOutcome::Granted(g) => g,
            other => panic!("expected a grant, got {other:?}"),
        };

        // Still inside its TTL: renewal succeeds, so no 409 would be sent.
        assert!(
            table.renew(grant.lease_id, Duration::from_millis(40)),
            "a live lease must renew"
        );

        // Simulate a single inference call outrunning the TTL.
        tokio::time::sleep(Duration::from_millis(90)).await;
        table.sweep_expired();
        assert!(
            !table.renew(grant.lease_id, Duration::from_millis(40)),
            "a lapsed lease must be refused — this is the 409 the caller sees"
        );

        // Retrying under the refused id stays refused, however long you wait:
        // this is why the generic retry loop recovered 0 of 38.
        tokio::time::sleep(Duration::from_millis(20)).await;
        assert!(
            !table.renew(grant.lease_id, Duration::from_millis(40)),
            "retrying under a refused lease can never succeed"
        );

        // Renegotiating does.
        let fresh = match table.try_grant(None, 2u64, Duration::from_millis(40)) {
            crate::capacity_lease::NegotiationOutcome::Granted(g) => g,
            other => panic!("expected a fresh grant, got {other:?}"),
        };
        assert_ne!(fresh.lease_id, grant.lease_id);
        assert!(
            table.renew(fresh.lease_id, Duration::from_millis(40)),
            "a renegotiated lease must be accepted"
        );
    }

    #[test]
    fn proxy_request_lease_id_defaults_to_none_for_legacy_wire_bytes() {
        // Regression: `lease_id` was added after this wire format was
        // already in use. A pre-Phase-2 caller's encoded ProxyRequest (no
        // lease_id key at all) must still decode cleanly, defaulting to
        // None, rather than failing to deserialize.
        #[derive(Debug, Serialize)]
        struct LegacyProxyRequest {
            method: String,
            path: String,
            body: Vec<u8>,
        }
        let legacy = LegacyProxyRequest {
            method: "POST".to_string(),
            path: "/api/chat".to_string(),
            body: b"{}".to_vec(),
        };
        let bytes = rmp_serde::to_vec_named(&legacy).unwrap();

        let decoded: ProxyRequest = rmp_serde::from_slice(&bytes).unwrap();
        assert_eq!(decoded.method, "POST");
        assert_eq!(decoded.path, "/api/chat");
        assert_eq!(decoded.lease_id, None);
    }

    #[test]
    fn proxy_request_lease_id_round_trips_when_present() {
        let req = ProxyRequest {
            method: "POST".to_string(),
            path: "/api/chat".to_string(),
            body: vec![],
            lease_id: Some(42),
        };
        let bytes = rmp_serde::to_vec_named(&req).unwrap();
        let decoded: ProxyRequest = rmp_serde::from_slice(&bytes).unwrap();
        assert_eq!(decoded.lease_id, Some(42));
    }
}
