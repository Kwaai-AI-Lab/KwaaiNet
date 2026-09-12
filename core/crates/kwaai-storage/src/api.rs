//! REST API for the storage fabric.
//!
//! URL surface is unchanged — remote Bobs call the same endpoints regardless
//! of whether the backend is PostgreSQL or the embedded hnsw_rs+SQLite store.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use crate::db::StorageDb;
use crate::tenant::{TenantInfo, TenantManager, TenantStats};
use crate::vectors::{SearchResult, VectorStore};

struct AppState {
    tenants: TenantManager,
    vectors: VectorStore,
    capacity_gb: f64,
    peer_id: String,
}

/// Build the Axum router (useful for testing without binding a port).
pub fn build_app(db: StorageDb, capacity_gb: f64, peer_id: String) -> Router {
    let state = Arc::new(AppState {
        tenants: TenantManager::new(db.clone()),
        vectors: VectorStore::new(db),
        capacity_gb,
        peer_id,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/health", get(health))
        .route("/api/tenants", post(create_tenant))
        .route("/api/tenants", get(list_tenants))
        .route("/api/tenants/:id", get(get_tenant))
        .route("/api/tenants/:id", delete(delete_tenant))
        .route("/api/tenants/:id/vectors", post(upload_vectors))
        .route("/api/tenants/:id/search", post(search_vectors))
        .route("/api/tenants/:id/vectors", delete(delete_vectors))
        .layer(cors)
        .with_state(state)
}

/// Start the storage API server on `bind_addr`.
///
/// A thin wrapper over [`run_storage_api_on`] for callers that have a single
/// address rather than listeners they bound themselves.
pub async fn run_storage_api(
    db: StorageDb,
    bind_addr: &str,
    capacity_gb: f64,
    peer_id: String,
) -> anyhow::Result<()> {
    tracing::info!("storage API listening on {}", bind_addr);
    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    run_storage_api_on(db, vec![listener], capacity_gb, peer_id).await
}

/// Serve the storage API on listeners the caller already bound.
///
/// Takes a list so one server can answer on both IPv4 and IPv6: `localhost`
/// resolves to `::1` first on most modern hosts, and a v4-only listener leaves
/// those clients with connection refused. Any listener failing ends the server,
/// which is what a single `axum::serve` did.
pub async fn run_storage_api_on(
    db: StorageDb,
    listeners: Vec<tokio::net::TcpListener>,
    capacity_gb: f64,
    peer_id: String,
) -> anyhow::Result<()> {
    let app = build_app(db, capacity_gb, peer_id);
    let mut serving = tokio::task::JoinSet::new();
    for listener in listeners {
        let app = app.clone();
        serving.spawn(async move { axum::serve(listener, app).await });
    }
    // Dropping the set on the first error aborts the others, so one dead
    // listener does not leave the process half-serving.
    while let Some(joined) = serving.join_next().await {
        joined??;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Health
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    tenant_count: i64,
    total_vectors: i64,
    capacity_gb_total: f64,
    capacity_gb_available: f64,
    version: &'static str,
    peer_id: String,
}

async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let tenant_count = state.tenants.count().await.unwrap_or(0);
    let total_vectors = state.tenants.total_vectors().await.unwrap_or(0);
    // 384-dim float32 ≈ 1.5 KB per vector with HNSW overhead
    let used_gb = (total_vectors as f64 * 1.5) / 1_048_576.0;
    let available = (state.capacity_gb - used_gb).max(0.0);

    Json(HealthResponse {
        status: "ok",
        tenant_count,
        total_vectors,
        capacity_gb_total: state.capacity_gb,
        capacity_gb_available: available,
        version: env!("CARGO_PKG_VERSION"),
        peer_id: state.peer_id.clone(),
    })
}

// ---------------------------------------------------------------------------
// Tenant CRUD
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct CreateTenantRequest {
    peer_id: String,
    #[serde(default = "default_capacity")]
    capacity_limit_mb: i64,
    display_name: Option<String>,
    #[serde(default = "default_dimension")]
    vector_dimension: usize,
}

fn default_capacity() -> i64 {
    1024
}
fn default_dimension() -> usize {
    384
}

async fn create_tenant(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateTenantRequest>,
) -> Result<(StatusCode, Json<TenantInfo>), (StatusCode, String)> {
    let info = state
        .tenants
        .create(
            &req.peer_id,
            req.capacity_limit_mb,
            req.display_name.as_deref(),
            req.vector_dimension,
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(info)))
}

async fn list_tenants(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TenantInfo>>, (StatusCode, String)> {
    let tenants = state
        .tenants
        .list()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(tenants))
}

async fn get_tenant(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<TenantWithStats>, (StatusCode, String)> {
    let info = state
        .tenants
        .get(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "tenant not found".to_string()))?;

    let stats = state
        .tenants
        .stats(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(TenantWithStats { info, stats }))
}

#[derive(Serialize)]
struct TenantWithStats {
    #[serde(flatten)]
    info: TenantInfo,
    #[serde(flatten)]
    stats: TenantStats,
}

async fn delete_tenant(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .tenants
        .delete(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// Vector operations
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct VectorEntry {
    id: i64,
    embedding: Vec<f32>,
}

#[derive(Deserialize)]
struct UploadRequest {
    vectors: Vec<VectorEntry>,
}

#[derive(Serialize)]
struct UploadResponse {
    uploaded: usize,
}

async fn upload_vectors(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
    Json(req): Json<UploadRequest>,
) -> Result<Json<UploadResponse>, (StatusCode, String)> {
    state
        .tenants
        .get(tenant_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "tenant not found".to_string()))?;

    let vectors: Vec<(i64, Vec<f32>)> = req
        .vectors
        .into_iter()
        .map(|v| (v.id, v.embedding))
        .collect();

    let uploaded = state
        .vectors
        .upload(tenant_id, &vectors)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(UploadResponse { uploaded }))
}

#[derive(Deserialize)]
struct SearchRequest {
    query: Vec<f32>,
    #[serde(default = "default_top_k")]
    top_k: usize,
}

fn default_top_k() -> usize {
    5
}

#[derive(Serialize)]
struct SearchResponse {
    results: Vec<SearchResult>,
}

async fn search_vectors(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
    Json(req): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, (StatusCode, String)> {
    let results = state
        .vectors
        .search(tenant_id, &req.query, req.top_k)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(SearchResponse { results }))
}

#[derive(Deserialize)]
struct DeleteRequest {
    ids: Vec<i64>,
}

#[derive(Serialize)]
struct DeleteResponse {
    deleted: usize,
}

async fn delete_vectors(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
    Json(req): Json<DeleteRequest>,
) -> Result<Json<DeleteResponse>, (StatusCode, String)> {
    let deleted = state
        .vectors
        .delete(tenant_id, &req.ids)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(DeleteResponse { deleted }))
}

#[cfg(test)]
mod dual_stack {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    /// A minimal HTTP/1.1 GET, so the test needs no client dependency.
    async fn get_status_line(addr: &str) -> String {
        let mut s = tokio::net::TcpStream::connect(addr)
            .await
            .unwrap_or_else(|e| panic!("connecting to {addr}: {e}"));
        s.write_all(b"GET /api/health HTTP/1.1\r\nHost: storage\r\nConnection: close\r\n\r\n")
            .await
            .expect("write request");
        let mut buf = Vec::new();
        s.read_to_end(&mut buf).await.expect("read response");
        String::from_utf8_lossy(&buf).into_owned()
    }

    /// One server, two listeners: a client that resolves `localhost` to `::1`
    /// first must reach the same store as one that picks `127.0.0.1`.
    #[tokio::test]
    async fn one_server_answers_on_both_families() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let db = StorageDb::open(tmp.path()).expect("open store");

        let v4 = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("v4 must bind");
        let port = v4.local_addr().unwrap().port();
        // The port is already taken on v4, so a v6 bind failing here means the
        // host has no v6 loopback rather than a clash.
        let v6 = tokio::net::TcpListener::bind(format!("[::1]:{port}"))
            .await
            .ok();
        let have_v6 = v6.is_some();

        let listeners = std::iter::once(v4).chain(v6).collect();
        let server = tokio::spawn(run_storage_api_on(db, listeners, 1.0, "peer".to_string()));

        let v4_response = get_status_line(&format!("127.0.0.1:{port}")).await;
        assert!(
            v4_response.starts_with("HTTP/1.1 200 OK"),
            "v4 health: {v4_response}"
        );
        assert!(v4_response.contains("\"status\":\"ok\""));

        if have_v6 {
            let v6_response = get_status_line(&format!("[::1]:{port}")).await;
            assert!(
                v6_response.starts_with("HTTP/1.1 200 OK"),
                "v6 health: {v6_response}"
            );
            assert!(v6_response.contains("\"status\":\"ok\""));
        } else {
            println!("skipping the v6 half: no IPv6 loopback on this host");
        }

        server.abort();
    }
}
