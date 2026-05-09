//! P2P relay protocol for the storage fabric.
//!
//! Protocol ID: `/kwaai/storage/1.0.0`
//!
//! Mirrors the HTTP API in kwaai-storage but routed over libp2p circuit relays,
//! so Eve nodes behind NAT can serve storage without port forwarding.
//!
//! Message flow:
//! ```text
//! Bob (client)                          Eve (server)
//!   │── StorageRequest (msgpack) ──────────────▶│
//!   │   { op, tenant_id, payload }               │
//!   │                                             │  dispatches to StorageDb
//!   │◀── StorageResponse (msgpack) ───────────────│
//!   │   { ok, payload, error }                    │
//! ```
//!
//! Each operation serialises its inputs/outputs as msgpack inside the
//! `payload` field, keeping the outer envelope stable regardless of op.

use anyhow::{bail, Context, Result};
use kwaai_p2p_daemon::{self, P2PClient};
use kwaai_storage::{SearchResult, StorageDb, TenantInfo, TenantManager, VectorStore};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const STORAGE_PROTO: &str = "/kwaai/storage/1.0.0";

// ── Outer envelope ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StorageOp {
    Health,
    CreateTenant,
    GetTenant,
    ListTenants,
    DeleteTenant,
    UploadVectors,
    SearchVectors,
    DeleteVectors,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageRequest {
    pub op: StorageOp,
    /// UUID string — required for tenant-scoped operations.
    pub tenant_id: Option<String>,
    /// msgpack-encoded op-specific input (see per-op types below).
    pub payload: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageResponse {
    pub ok: bool,
    /// msgpack-encoded op-specific output on success.
    pub payload: Vec<u8>,
    pub error: Option<String>,
}

impl StorageResponse {
    fn ok(payload: Vec<u8>) -> Self {
        Self {
            ok: true,
            payload,
            error: None,
        }
    }
    fn err(msg: impl std::fmt::Display) -> Self {
        Self {
            ok: false,
            payload: vec![],
            error: Some(msg.to_string()),
        }
    }
}

// ── Per-op payload types ──────────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct HealthPayload {
    pub status: String,
    pub tenant_count: i64,
    pub total_vectors: i64,
    pub capacity_gb_total: f64,
    pub capacity_gb_available: f64,
    pub version: String,
    pub peer_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTenantPayload {
    pub peer_id: String,
    #[serde(default = "default_capacity")]
    pub capacity_limit_mb: i64,
    pub display_name: Option<String>,
    #[serde(default = "default_dimension")]
    pub vector_dimension: usize,
}
fn default_capacity() -> i64 {
    1024
}
fn default_dimension() -> usize {
    384
}

#[derive(Serialize, Deserialize)]
pub struct VectorEntry {
    pub id: i64,
    pub embedding: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadPayload {
    pub vectors: Vec<VectorEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchPayload {
    pub query: Vec<f32>,
    #[serde(default = "default_top_k")]
    pub top_k: usize,
}
fn default_top_k() -> usize {
    5
}

#[derive(Serialize, Deserialize)]
pub struct DeleteVectorsPayload {
    pub ids: Vec<i64>,
}

// ── Server-side handler factory ───────────────────────────────────────────────

/// Build a unary handler that dispatches storage RPC requests to the local
/// `StorageDb`. Register it with `P2PClient::add_unary_handler`.
pub fn make_storage_rpc_handler(
    db: StorageDb,
    capacity_gb: f64,
    peer_id: String,
) -> impl Fn(
    Vec<u8>,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = kwaai_p2p_daemon::Result<Vec<u8>>> + Send>,
> + Send
       + Sync
       + 'static {
    move |req_bytes: Vec<u8>| {
        let db = db.clone();
        let peer_id = peer_id.clone();
        Box::pin(async move {
            let resp = dispatch(db, capacity_gb, peer_id, req_bytes).await;
            Ok(rmp_serde::to_vec_named(&resp).unwrap_or_default())
        })
    }
}

async fn dispatch(
    db: StorageDb,
    capacity_gb: f64,
    peer_id: String,
    req_bytes: Vec<u8>,
) -> StorageResponse {
    let req: StorageRequest = match rmp_serde::from_slice(&req_bytes) {
        Ok(r) => r,
        Err(e) => return StorageResponse::err(format!("deserialise request: {e}")),
    };

    let tenant_id = req
        .tenant_id
        .as_deref()
        .and_then(|s| s.parse::<Uuid>().ok());

    match req.op {
        StorageOp::Health => {
            let tm = TenantManager::new(db);
            let tenant_count = tm.count().await.unwrap_or(0);
            let total_vectors = tm.total_vectors().await.unwrap_or(0);
            let used_gb = (total_vectors as f64 * 1.5) / 1_048_576.0;
            let available = (capacity_gb - used_gb).max(0.0);
            let body = HealthPayload {
                status: "ok".into(),
                tenant_count,
                total_vectors,
                capacity_gb_total: capacity_gb,
                capacity_gb_available: available,
                version: env!("CARGO_PKG_VERSION").into(),
                peer_id,
            };
            encode_ok(&body)
        }

        StorageOp::CreateTenant => {
            let input: CreateTenantPayload = match rmp_serde::from_slice(&req.payload) {
                Ok(v) => v,
                Err(e) => return StorageResponse::err(format!("payload: {e}")),
            };
            let tm = TenantManager::new(db);
            // Reject if Eve doesn't have enough headroom for the requested quota.
            let used_bytes = tm.total_storage_bytes().await.unwrap_or(0);
            let eve_capacity_bytes = (capacity_gb * 1_073_741_824.0) as i64;
            let requested_bytes = input.capacity_limit_mb * 1024 * 1024;
            if eve_capacity_bytes > 0 && used_bytes + requested_bytes > eve_capacity_bytes {
                let available_mb = (eve_capacity_bytes - used_bytes).max(0) / (1024 * 1024);
                return StorageResponse::err(format!(
                    "Eve storage full: only {} MB available, {} MB requested",
                    available_mb, input.capacity_limit_mb,
                ));
            }
            match tm
                .create(
                    &input.peer_id,
                    input.capacity_limit_mb,
                    input.display_name.as_deref(),
                    input.vector_dimension,
                )
                .await
            {
                Ok(info) => encode_ok(&info),
                Err(e) => StorageResponse::err(e),
            }
        }

        StorageOp::GetTenant => {
            let Some(tid) = tenant_id else {
                return StorageResponse::err("missing tenant_id");
            };
            let tm = TenantManager::new(db);
            match tm.get(tid).await {
                Ok(Some(info)) => encode_ok(&info),
                Ok(None) => StorageResponse::err("tenant not found"),
                Err(e) => StorageResponse::err(e),
            }
        }

        StorageOp::ListTenants => {
            let tm = TenantManager::new(db);
            match tm.list().await {
                Ok(list) => encode_ok(&list),
                Err(e) => StorageResponse::err(e),
            }
        }

        StorageOp::DeleteTenant => {
            let Some(tid) = tenant_id else {
                return StorageResponse::err("missing tenant_id");
            };
            let tm = TenantManager::new(db);
            match tm.delete(tid).await {
                Ok(()) => StorageResponse::ok(vec![]),
                Err(e) => StorageResponse::err(e),
            }
        }

        StorageOp::UploadVectors => {
            let Some(tid) = tenant_id else {
                return StorageResponse::err("missing tenant_id");
            };
            let input: UploadPayload = match rmp_serde::from_slice(&req.payload) {
                Ok(v) => v,
                Err(e) => return StorageResponse::err(format!("payload: {e}")),
            };

            // Capacity checks before any writes.
            let tm = TenantManager::new(db.clone());
            let dim = input
                .vectors
                .first()
                .map(|v| v.embedding.len())
                .unwrap_or(384) as i64;
            let bytes_per_vec = 4 * dim + 24;
            let incoming_bytes = input.vectors.len() as i64 * bytes_per_vec;

            // 1. Per-tenant quota.
            let tenant_info = match tm.get(tid).await {
                Ok(Some(i)) => i,
                Ok(None) => return StorageResponse::err("tenant not found"),
                Err(e) => return StorageResponse::err(format!("tenant lookup: {e}")),
            };
            if tenant_info.capacity_limit_mb > 0 {
                let stats = match tm.stats(tid).await {
                    Ok(s) => s,
                    Err(e) => return StorageResponse::err(format!("stats: {e}")),
                };
                let limit_bytes = tenant_info.capacity_limit_mb * 1024 * 1024;
                if stats.storage_bytes + incoming_bytes > limit_bytes {
                    return StorageResponse::err(format!(
                        "tenant quota exceeded: {}/{} MB used",
                        (stats.storage_bytes + incoming_bytes) / (1024 * 1024),
                        tenant_info.capacity_limit_mb,
                    ));
                }
            }

            // 2. Eve total capacity.
            let eve_capacity_bytes = (capacity_gb * 1_073_741_824.0) as i64;
            if eve_capacity_bytes > 0 {
                let total_bytes = tm.total_storage_bytes().await.unwrap_or(0);
                if total_bytes + incoming_bytes > eve_capacity_bytes {
                    let used_gb = total_bytes as f64 / 1_073_741_824.0;
                    return StorageResponse::err(format!(
                        "Eve storage full: {:.2}/{:.2} GB used",
                        used_gb, capacity_gb,
                    ));
                }
            }

            let vectors: Vec<(i64, Vec<f32>)> = input
                .vectors
                .into_iter()
                .map(|v| (v.id, v.embedding))
                .collect();
            let vs = VectorStore::new(db);
            match vs.upload(tid, &vectors).await {
                Ok(n) => encode_ok(&n),
                Err(e) => StorageResponse::err(e),
            }
        }

        StorageOp::SearchVectors => {
            let Some(tid) = tenant_id else {
                return StorageResponse::err("missing tenant_id");
            };
            let input: SearchPayload = match rmp_serde::from_slice(&req.payload) {
                Ok(v) => v,
                Err(e) => return StorageResponse::err(format!("payload: {e}")),
            };
            let vs = VectorStore::new(db);
            match vs.search(tid, &input.query, input.top_k).await {
                Ok(results) => encode_ok(&results),
                Err(e) => StorageResponse::err(e),
            }
        }

        StorageOp::DeleteVectors => {
            let Some(tid) = tenant_id else {
                return StorageResponse::err("missing tenant_id");
            };
            let input: DeleteVectorsPayload = match rmp_serde::from_slice(&req.payload) {
                Ok(v) => v,
                Err(e) => return StorageResponse::err(format!("payload: {e}")),
            };
            let vs = VectorStore::new(db);
            match vs.delete(tid, &input.ids).await {
                Ok(n) => encode_ok(&n),
                Err(e) => StorageResponse::err(e),
            }
        }
    }
}

fn encode_ok<T: Serialize>(val: &T) -> StorageResponse {
    match rmp_serde::to_vec_named(val) {
        Ok(bytes) => StorageResponse::ok(bytes),
        Err(e) => StorageResponse::err(format!("serialise response: {e}")),
    }
}

// ── Client-side helpers ───────────────────────────────────────────────────────
// Public API for Bob nodes — Phase 2/3 vpk tenant commands will call these.

/// Generic one-shot RPC call to an Eve node.
async fn call_storage(
    client: &P2PClient,
    peer_id: &PeerId,
    req: StorageRequest,
) -> Result<StorageResponse> {
    let req_bytes = rmp_serde::to_vec_named(&req).context("serialise StorageRequest")?;
    let resp_bytes = client
        .call_unary_handler(&peer_id.to_bytes(), STORAGE_PROTO, &req_bytes)
        .await
        .context("call_unary_handler storage")?;
    let resp: StorageResponse =
        rmp_serde::from_slice(&resp_bytes).context("deserialise StorageResponse")?;
    if !resp.ok {
        bail!("{}", resp.error.unwrap_or_else(|| "unknown error".into()));
    }
    Ok(resp)
}

pub async fn rpc_health(client: &P2PClient, peer_id: &PeerId) -> Result<HealthPayload> {
    let resp = call_storage(
        client,
        peer_id,
        StorageRequest {
            op: StorageOp::Health,
            tenant_id: None,
            payload: vec![],
        },
    )
    .await?;
    rmp_serde::from_slice(&resp.payload).context("decode HealthPayload")
}

pub async fn rpc_create_tenant(
    client: &P2PClient,
    peer_id: &PeerId,
    input: CreateTenantPayload,
) -> Result<TenantInfo> {
    let payload = rmp_serde::to_vec_named(&input)?;
    let resp = call_storage(
        client,
        peer_id,
        StorageRequest {
            op: StorageOp::CreateTenant,
            tenant_id: None,
            payload,
        },
    )
    .await?;
    rmp_serde::from_slice(&resp.payload).context("decode TenantInfo")
}

#[allow(dead_code)]
pub async fn rpc_upload_vectors(
    client: &P2PClient,
    peer_id: &PeerId,
    tenant_id: Uuid,
    vectors: Vec<(i64, Vec<f32>)>,
) -> Result<usize> {
    let entries: Vec<VectorEntry> = vectors
        .into_iter()
        .map(|(id, embedding)| VectorEntry { id, embedding })
        .collect();
    let payload = rmp_serde::to_vec_named(&UploadPayload { vectors: entries })?;
    let resp = call_storage(
        client,
        peer_id,
        StorageRequest {
            op: StorageOp::UploadVectors,
            tenant_id: Some(tenant_id.to_string()),
            payload,
        },
    )
    .await?;
    rmp_serde::from_slice::<usize>(&resp.payload).context("decode upload count")
}

#[allow(dead_code)]
pub async fn rpc_search_vectors(
    client: &P2PClient,
    peer_id: &PeerId,
    tenant_id: Uuid,
    query: Vec<f32>,
    top_k: usize,
) -> Result<Vec<SearchResult>> {
    let payload = rmp_serde::to_vec_named(&SearchPayload { query, top_k })?;
    let resp = call_storage(
        client,
        peer_id,
        StorageRequest {
            op: StorageOp::SearchVectors,
            tenant_id: Some(tenant_id.to_string()),
            payload,
        },
    )
    .await?;
    rmp_serde::from_slice(&resp.payload).context("decode SearchResults")
}

#[allow(dead_code)]
pub async fn rpc_delete_vectors(
    client: &P2PClient,
    peer_id: &PeerId,
    tenant_id: Uuid,
    ids: Vec<i64>,
) -> Result<usize> {
    let payload = rmp_serde::to_vec_named(&DeleteVectorsPayload { ids })?;
    let resp = call_storage(
        client,
        peer_id,
        StorageRequest {
            op: StorageOp::DeleteVectors,
            tenant_id: Some(tenant_id.to_string()),
            payload,
        },
    )
    .await?;
    rmp_serde::from_slice::<usize>(&resp.payload).context("decode delete count")
}

pub async fn rpc_delete_tenant(
    client: &P2PClient,
    peer_id: &PeerId,
    tenant_id: Uuid,
) -> Result<()> {
    call_storage(
        client,
        peer_id,
        StorageRequest {
            op: StorageOp::DeleteTenant,
            tenant_id: Some(tenant_id.to_string()),
            payload: vec![],
        },
    )
    .await
    .map(|_| ())
}

pub async fn rpc_list_tenants(client: &P2PClient, peer_id: &PeerId) -> Result<Vec<TenantInfo>> {
    let resp = call_storage(
        client,
        peer_id,
        StorageRequest {
            op: StorageOp::ListTenants,
            tenant_id: None,
            payload: vec![],
        },
    )
    .await?;
    rmp_serde::from_slice(&resp.payload).context("decode TenantInfo list")
}

// ── HTTP-based client (for local Eve — bypasses P2P dial-to-self) ─────────────

#[derive(serde::Serialize)]
struct HttpCreateTenantReq<'a> {
    peer_id: &'a str,
    capacity_limit_mb: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<&'a str>,
    vector_dimension: usize,
}

#[derive(serde::Serialize)]
struct HttpVectorEntry {
    id: i64,
    embedding: Vec<f32>,
}

#[derive(serde::Serialize)]
struct HttpUploadReq {
    vectors: Vec<HttpVectorEntry>,
}

#[derive(serde::Deserialize)]
struct HttpUploadResp {
    uploaded: usize,
}

#[derive(serde::Serialize)]
struct HttpSearchReq {
    query: Vec<f32>,
    top_k: usize,
}

#[derive(serde::Deserialize)]
struct HttpSearchResp {
    results: Vec<SearchResult>,
}

#[derive(serde::Serialize)]
struct HttpDeleteReq {
    ids: Vec<i64>,
}

#[derive(serde::Deserialize)]
struct HttpDeleteResp {
    deleted: usize,
}

pub async fn http_create_tenant(
    http: &reqwest::Client,
    base_url: &str,
    payload: CreateTenantPayload,
) -> Result<TenantInfo> {
    let resp = http
        .post(format!("{base_url}/api/tenants"))
        .json(&HttpCreateTenantReq {
            peer_id: &payload.peer_id,
            capacity_limit_mb: payload.capacity_limit_mb,
            display_name: payload.display_name.as_deref(),
            vector_dimension: payload.vector_dimension,
        })
        .send()
        .await
        .context("http_create_tenant")?;
    if !resp.status().is_success() {
        bail!(
            "storage HTTP {}: {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        );
    }
    resp.json().await.context("decode TenantInfo")
}

pub async fn http_upload_vectors(
    http: &reqwest::Client,
    base_url: &str,
    tenant_id: Uuid,
    vectors: Vec<(i64, Vec<f32>)>,
) -> Result<usize> {
    let req = HttpUploadReq {
        vectors: vectors
            .into_iter()
            .map(|(id, embedding)| HttpVectorEntry { id, embedding })
            .collect(),
    };
    let resp = http
        .post(format!("{base_url}/api/tenants/{tenant_id}/vectors"))
        .json(&req)
        .send()
        .await
        .context("http_upload_vectors")?;
    if !resp.status().is_success() {
        bail!(
            "storage HTTP {}: {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        );
    }
    Ok(resp.json::<HttpUploadResp>().await?.uploaded)
}

pub async fn http_search_vectors(
    http: &reqwest::Client,
    base_url: &str,
    tenant_id: Uuid,
    query: Vec<f32>,
    top_k: usize,
) -> Result<Vec<SearchResult>> {
    let resp = http
        .post(format!("{base_url}/api/tenants/{tenant_id}/search"))
        .json(&HttpSearchReq { query, top_k })
        .send()
        .await
        .context("http_search_vectors")?;
    if !resp.status().is_success() {
        bail!(
            "storage HTTP {}: {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        );
    }
    Ok(resp.json::<HttpSearchResp>().await?.results)
}

pub async fn http_delete_vectors(
    http: &reqwest::Client,
    base_url: &str,
    tenant_id: Uuid,
    ids: Vec<i64>,
) -> Result<usize> {
    let resp = http
        .delete(format!("{base_url}/api/tenants/{tenant_id}/vectors"))
        .json(&HttpDeleteReq { ids })
        .send()
        .await
        .context("http_delete_vectors")?;
    if !resp.status().is_success() {
        bail!(
            "storage HTTP {}: {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        );
    }
    Ok(resp.json::<HttpDeleteResp>().await?.deleted)
}
