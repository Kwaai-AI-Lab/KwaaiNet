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
    /// **Ignored by the server.** Ownership is taken from the authenticated
    /// caller, never from the payload — this field is self-declared and was
    /// previously written straight into the tenant record, which let anyone
    /// create a tenant attributed to anyone.
    ///
    /// Retained so the wire format is unchanged for existing clients (they all
    /// send their own id anyway, so enforcement is a no-op for them).
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
/// `StorageDb`.
///
/// Must be registered with `P2PClient::add_unary_handler_with_peer` — every
/// tenant-scoped operation authorises against the authenticated caller, so a
/// registration that discards the caller would silently restore the
/// no-authorisation behaviour this replaces.
///
/// `our_peer_id` is *this node's own* identity, reported in `Health`. It is not
/// the caller and is never used for authorisation.
#[allow(clippy::type_complexity)]
pub fn make_storage_rpc_handler(
    db: StorageDb,
    capacity_gb: f64,
    our_peer_id: String,
) -> impl Fn(
    Vec<u8>,
    PeerId,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = kwaai_p2p_daemon::Result<Vec<u8>>> + Send>,
> + Send
       + Sync
       + 'static {
    move |req_bytes: Vec<u8>, caller: PeerId| {
        let db = db.clone();
        let our_peer_id = our_peer_id.clone();
        Box::pin(async move {
            let resp = dispatch(db, capacity_gb, our_peer_id, caller.to_base58(), req_bytes).await;
            Ok(rmp_serde::to_vec_named(&resp).unwrap_or_default())
        })
    }
}

/// Deliberately identical for "no such tenant" and "not yours".
///
/// Distinguishing them would turn the endpoint into an oracle for which tenant
/// UUIDs exist on a node — which is most of what made `ListTenants` dangerous in
/// the first place.
fn tenant_denied() -> StorageResponse {
    StorageResponse::err("tenant not found or not owned by caller")
}

/// Fetch a tenant only if the authenticated caller owns it.
async fn owned_tenant(
    tm: &TenantManager,
    tid: Uuid,
    caller: &str,
) -> std::result::Result<kwaai_storage::TenantInfo, StorageResponse> {
    match tm.get(tid).await {
        Ok(Some(info)) if info.peer_id == caller => Ok(info),
        Ok(_) => Err(tenant_denied()),
        Err(e) => Err(StorageResponse::err(format!("tenant lookup: {e}"))),
    }
}

async fn dispatch(
    db: StorageDb,
    capacity_gb: f64,
    our_peer_id: String,
    caller: String,
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
                peer_id: our_peer_id,
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
            if !input.peer_id.is_empty() && input.peer_id != caller {
                tracing::warn!(
                    "storage: CreateTenant declared owner {} but caller is {caller}; \
                     using the authenticated caller",
                    input.peer_id
                );
            }
            match tm
                .create(
                    &caller,
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
            match owned_tenant(&tm, tid, &caller).await {
                Ok(info) => encode_ok(&info),
                Err(resp) => resp,
            }
        }

        StorageOp::ListTenants => {
            // Scoped to the caller's own tenants. Returning every tenant handed
            // out the tenant_id of every other tenant on the node — and since
            // tenant_id was the only thing gating read, write and delete, that
            // amounted to full access to all of them.
            let tm = TenantManager::new(db);
            match tm.list().await {
                Ok(list) => {
                    let mine: Vec<_> = list.into_iter().filter(|t| t.peer_id == caller).collect();
                    encode_ok(&mine)
                }
                Err(e) => StorageResponse::err(e),
            }
        }

        StorageOp::DeleteTenant => {
            let Some(tid) = tenant_id else {
                return StorageResponse::err("missing tenant_id");
            };
            let tm = TenantManager::new(db);
            if let Err(resp) = owned_tenant(&tm, tid, &caller).await {
                return resp;
            }
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
            let tenant_info = match owned_tenant(&tm, tid, &caller).await {
                Ok(i) => i,
                Err(resp) => return resp,
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
            let tm = TenantManager::new(db.clone());
            if let Err(resp) = owned_tenant(&tm, tid, &caller).await {
                return resp;
            }
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
            let tm = TenantManager::new(db.clone());
            if let Err(resp) = owned_tenant(&tm, tid, &caller).await {
                return resp;
            }
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use kwaai_storage::StorageDb;

    const ALICE: &str = "12D3KooWAliceAliceAliceAliceAliceAliceAliceAliceAlic";
    const MALLORY: &str = "12D3KooWMalloryMalloryMalloryMalloryMalloryMalloryMa";

    fn db() -> (tempfile::TempDir, StorageDb) {
        let tmp = tempfile::tempdir().unwrap();
        let db = StorageDb::open(tmp.path()).unwrap();
        (tmp, db)
    }

    fn req(op: StorageOp, tenant_id: Option<String>, payload: Vec<u8>) -> Vec<u8> {
        rmp_serde::to_vec_named(&StorageRequest {
            op,
            tenant_id,
            payload,
        })
        .unwrap()
    }

    async fn call(db: &StorageDb, caller: &str, r: Vec<u8>) -> StorageResponse {
        dispatch(db.clone(), 100.0, "eve".into(), caller.to_string(), r).await
    }

    /// Create a tenant as `owner` and return its id.
    async fn make_tenant(db: &StorageDb, owner: &str) -> Uuid {
        let payload = rmp_serde::to_vec_named(&CreateTenantPayload {
            peer_id: owner.to_string(),
            capacity_limit_mb: 16,
            display_name: None,
            vector_dimension: 4,
        })
        .unwrap();
        let resp = call(db, owner, req(StorageOp::CreateTenant, None, payload)).await;
        assert!(resp.ok, "create failed: {:?}", resp.error);
        let info: kwaai_storage::TenantInfo = rmp_serde::from_slice(&resp.payload).unwrap();
        assert_eq!(info.peer_id, owner);
        info.tenant_id
    }

    #[tokio::test]
    async fn ownership_comes_from_the_caller_not_the_payload() {
        // The original hole: peer_id was self-declared and written straight into
        // the record, so anyone could create a tenant attributed to anyone.
        let (_tmp, db) = db();
        let payload = rmp_serde::to_vec_named(&CreateTenantPayload {
            peer_id: ALICE.to_string(), // Mallory claims to be Alice
            capacity_limit_mb: 16,
            display_name: None,
            vector_dimension: 4,
        })
        .unwrap();
        let resp = call(&db, MALLORY, req(StorageOp::CreateTenant, None, payload)).await;
        assert!(resp.ok);
        let info: kwaai_storage::TenantInfo = rmp_serde::from_slice(&resp.payload).unwrap();
        assert_eq!(
            info.peer_id, MALLORY,
            "the authenticated caller must own it, not the claimed id"
        );
    }

    #[tokio::test]
    async fn list_tenants_shows_only_the_callers_own() {
        // This is the leak: ListTenants returned every TenantInfo — including
        // tenant_id, which was the only thing gating read/write/delete. One call
        // yielded full access to every tenant on the node.
        let (_tmp, db) = db();
        let alice_t = make_tenant(&db, ALICE).await;
        let _mallory_t = make_tenant(&db, MALLORY).await;

        let resp = call(&db, MALLORY, req(StorageOp::ListTenants, None, vec![])).await;
        assert!(resp.ok);
        let list: Vec<kwaai_storage::TenantInfo> = rmp_serde::from_slice(&resp.payload).unwrap();

        assert!(
            list.iter().all(|t| t.peer_id == MALLORY),
            "another peer's tenant leaked into the listing"
        );
        assert!(
            !list.iter().any(|t| t.tenant_id == alice_t),
            "Alice's tenant_id — the bearer capability — leaked to Mallory"
        );
    }

    #[tokio::test]
    async fn every_tenant_scoped_op_is_denied_to_a_non_owner() {
        let (_tmp, db) = db();
        let alice_t = make_tenant(&db, ALICE).await;
        let tid = Some(alice_t.to_string());

        let upload = rmp_serde::to_vec_named(&UploadPayload {
            vectors: vec![VectorEntry {
                id: 1,
                embedding: vec![0.1, 0.2, 0.3, 0.4],
            }],
        })
        .unwrap();
        let search = rmp_serde::to_vec_named(&SearchPayload {
            query: vec![0.1, 0.2, 0.3, 0.4],
            top_k: 1,
        })
        .unwrap();
        let del = rmp_serde::to_vec_named(&DeleteVectorsPayload { ids: vec![1] }).unwrap();

        // Knowing the tenant_id must no longer be sufficient for anything.
        for (op, payload) in [
            (StorageOp::GetTenant, vec![]),
            (StorageOp::DeleteTenant, vec![]),
            (StorageOp::UploadVectors, upload),
            (StorageOp::SearchVectors, search),
            (StorageOp::DeleteVectors, del),
        ] {
            let resp = call(&db, MALLORY, req(op, tid.clone(), payload)).await;
            assert!(!resp.ok, "{op:?} should be denied to a non-owner");
        }

        // And the owner is unaffected.
        let resp = call(&db, ALICE, req(StorageOp::GetTenant, tid, vec![])).await;
        assert!(resp.ok, "owner must still have access: {:?}", resp.error);
    }

    #[tokio::test]
    async fn denial_does_not_reveal_whether_a_tenant_exists() {
        // Otherwise the endpoint is an oracle for valid tenant UUIDs, which is
        // most of what made the listing leak dangerous.
        let (_tmp, db) = db();
        let real = make_tenant(&db, ALICE).await;

        let existing = call(
            &db,
            MALLORY,
            req(StorageOp::GetTenant, Some(real.to_string()), vec![]),
        )
        .await;
        let absent = call(
            &db,
            MALLORY,
            req(
                StorageOp::GetTenant,
                Some(Uuid::new_v4().to_string()),
                vec![],
            ),
        )
        .await;

        assert!(!existing.ok && !absent.ok);
        assert_eq!(
            existing.error, absent.error,
            "the two cases must be indistinguishable"
        );
    }

    #[tokio::test]
    async fn health_reports_this_node_not_the_caller() {
        let (_tmp, db) = db();
        let resp = call(&db, MALLORY, req(StorageOp::Health, None, vec![])).await;
        assert!(resp.ok);
        let h: HealthPayload = rmp_serde::from_slice(&resp.payload).unwrap();
        assert_eq!(h.peer_id, "eve", "Health must report our own identity");
    }
}
