//! HTTP-level integration tests for the storage REST API.
//!
//! Uses `tower::ServiceExt::oneshot` to drive the Axum router directly
//! without binding a port.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use kwaai_storage::{build_app, StorageDb};
use serde_json::{json, Value};
use tempfile::TempDir;
use tower::ServiceExt;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_app() -> (TempDir, axum::Router) {
    let tmp = TempDir::new().unwrap();
    let db = StorageDb::open(tmp.path()).unwrap();
    let app = build_app(db, 10.0, "test-peer".to_string());
    (tmp, app)
}

async fn body_json(body: Body) -> Value {
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

fn json_body(v: Value) -> Body {
    Body::from(serde_json::to_vec(&v).unwrap())
}

// ---------------------------------------------------------------------------
// Health
// ---------------------------------------------------------------------------

#[tokio::test]
async fn health_returns_ok_with_expected_fields() {
    let (_tmp, app) = make_app();
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp.into_body()).await;
    assert_eq!(body["status"], "ok");
    assert_eq!(body["peer_id"], "test-peer");
    assert_eq!(body["capacity_gb_total"], 10.0);
    assert!(body["tenant_count"].is_number());
    assert!(body["total_vectors"].is_number());
}

// ---------------------------------------------------------------------------
// Tenant CRUD
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_tenant_returns_201_with_info() {
    let (_tmp, app) = make_app();
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/tenants")
                .header("content-type", "application/json")
                .body(json_body(json!({
                    "peer_id": "alice",
                    "capacity_limit_mb": 256,
                    "display_name": "Alice's Store",
                    "vector_dimension": 8
                })))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = body_json(resp.into_body()).await;
    assert_eq!(body["peer_id"], "alice");
    assert_eq!(body["capacity_limit_mb"], 256);
    assert_eq!(body["display_name"], "Alice's Store");
    assert_eq!(body["vector_dimension"], 8);
    assert_eq!(body["status"], "Active");
    assert!(body["tenant_id"].is_string());
}

#[tokio::test]
async fn create_tenant_uses_defaults_for_optional_fields() {
    let (_tmp, app) = make_app();
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/tenants")
                .header("content-type", "application/json")
                .body(json_body(json!({ "peer_id": "bob" })))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = body_json(resp.into_body()).await;
    // defaults: capacity_limit_mb = 1024, vector_dimension = 384
    assert_eq!(body["capacity_limit_mb"], 1024);
    assert_eq!(body["vector_dimension"], 384);
}

#[tokio::test]
async fn list_tenants_returns_created_tenant() {
    let (_tmp, app) = make_app();

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/tenants")
                .header("content-type", "application/json")
                .body(json_body(json!({ "peer_id": "carol" })))
                .unwrap(),
        )
        .await
        .unwrap();

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/tenants")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp.into_body()).await;
    assert!(body.as_array().unwrap().len() >= 1);
    assert!(body
        .as_array()
        .unwrap()
        .iter()
        .any(|t| t["peer_id"] == "carol"));
}

#[tokio::test]
async fn get_tenant_returns_info_and_stats() {
    let (_tmp, app) = make_app();

    let create_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/tenants")
                .header("content-type", "application/json")
                .body(json_body(json!({ "peer_id": "dave", "vector_dimension": 4 })))
                .unwrap(),
        )
        .await
        .unwrap();

    let created = body_json(create_resp.into_body()).await;
    let tid = created["tenant_id"].as_str().unwrap();

    let resp = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/tenants/{tid}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp.into_body()).await;
    assert_eq!(body["peer_id"], "dave");
    assert_eq!(body["vector_count"], 0);
    assert!(body["storage_bytes"].is_number());
}

#[tokio::test]
async fn get_unknown_tenant_returns_404() {
    let (_tmp, app) = make_app();
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/tenants/00000000-0000-0000-0000-000000000001")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_tenant_returns_204_and_removes_it() {
    let (_tmp, app) = make_app();

    let create_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/tenants")
                .header("content-type", "application/json")
                .body(json_body(json!({ "peer_id": "eve" })))
                .unwrap(),
        )
        .await
        .unwrap();

    let created = body_json(create_resp.into_body()).await;
    let tid = created["tenant_id"].as_str().unwrap();

    let del_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/tenants/{tid}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(del_resp.status(), StatusCode::NO_CONTENT);

    let get_resp = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/tenants/{tid}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get_resp.status(), StatusCode::NOT_FOUND);
}

// ---------------------------------------------------------------------------
// Vector operations
// ---------------------------------------------------------------------------

#[tokio::test]
async fn upload_and_search_vectors() {
    let (_tmp, app) = make_app();

    // Create tenant.
    let cr = body_json(
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/tenants")
                    .header("content-type", "application/json")
                    .body(json_body(json!({ "peer_id": "frank", "vector_dimension": 4 })))
                    .unwrap(),
            )
            .await
            .unwrap()
            .into_body(),
    )
    .await;
    let tid = cr["tenant_id"].as_str().unwrap();

    // Upload two orthogonal unit vectors.
    let up_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/tenants/{tid}/vectors"))
                .header("content-type", "application/json")
                .body(json_body(json!({
                    "vectors": [
                        { "id": 1, "embedding": [1.0, 0.0, 0.0, 0.0] },
                        { "id": 2, "embedding": [0.0, 1.0, 0.0, 0.0] }
                    ]
                })))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(up_resp.status(), StatusCode::OK);
    let up_body = body_json(up_resp.into_body()).await;
    assert_eq!(up_body["uploaded"], 2);

    // Search — query aligned with doc 1.
    let sr = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/tenants/{tid}/search"))
                .header("content-type", "application/json")
                .body(json_body(json!({
                    "query": [1.0, 0.0, 0.0, 0.0],
                    "top_k": 1
                })))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(sr.status(), StatusCode::OK);
    let s_body = body_json(sr.into_body()).await;
    let results = s_body["results"].as_array().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["id"], 1);
    let score = results[0]["score"].as_f64().unwrap();
    assert!((score - 1.0).abs() < 1e-5, "score={score}");
}

#[tokio::test]
async fn delete_vectors_removes_them_from_search() {
    let (_tmp, app) = make_app();

    let cr = body_json(
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/tenants")
                    .header("content-type", "application/json")
                    .body(json_body(json!({ "peer_id": "grace", "vector_dimension": 4 })))
                    .unwrap(),
            )
            .await
            .unwrap()
            .into_body(),
    )
    .await;
    let tid = cr["tenant_id"].as_str().unwrap();

    // Upload two vectors.
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/tenants/{tid}/vectors"))
                .header("content-type", "application/json")
                .body(json_body(json!({
                    "vectors": [
                        { "id": 10, "embedding": [1.0, 0.0, 0.0, 0.0] },
                        { "id": 11, "embedding": [0.0, 1.0, 0.0, 0.0] }
                    ]
                })))
                .unwrap(),
        )
        .await
        .unwrap();

    // Delete doc 10.
    let del_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/tenants/{tid}/vectors"))
                .header("content-type", "application/json")
                .body(json_body(json!({ "ids": [10] })))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(del_resp.status(), StatusCode::OK);
    let del_body = body_json(del_resp.into_body()).await;
    assert_eq!(del_body["deleted"], 1);

    // Search should no longer return doc 10.
    let sr = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/tenants/{tid}/search"))
                .header("content-type", "application/json")
                .body(json_body(json!({
                    "query": [1.0, 0.0, 0.0, 0.0],
                    "top_k": 10
                })))
                .unwrap(),
        )
        .await
        .unwrap();
    let s_body = body_json(sr.into_body()).await;
    let ids: Vec<i64> = s_body["results"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].as_i64().unwrap())
        .collect();
    assert!(!ids.contains(&10));
}

#[tokio::test]
async fn upload_to_unknown_tenant_returns_404() {
    let (_tmp, app) = make_app();
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/tenants/00000000-0000-0000-0000-000000000002/vectors")
                .header("content-type", "application/json")
                .body(json_body(json!({
                    "vectors": [{ "id": 1, "embedding": [1.0, 0.0] }]
                })))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
