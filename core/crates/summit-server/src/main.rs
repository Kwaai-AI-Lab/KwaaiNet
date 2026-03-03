use std::sync::Arc;

use anyhow::Result;
use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Embed the page at compile time — no filesystem path issues at runtime
static INDEX_HTML: &str = include_str!("../static/index.html");

async fn serve_index() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        INDEX_HTML,
    )
}

mod config;
mod db;
mod routes;
mod state;
mod vc_issuer;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "summit_server=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = config::Config::from_env()?;

    tracing::info!("Connecting to Postgres…");
    let pool = db::build_pool(&cfg.database_url).await?;
    db::migrate(&pool).await?;
    tracing::info!("Migrations applied.");

    let webauthn = Arc::new(state::build_webauthn(&cfg)?);
    let signing_key = state::build_signing_key(&cfg.signing_key_hex)?;
    let shared = Arc::new(state::AppState::new(webauthn, pool, signing_key));

    tracing::info!("Issuer DID: {}", shared.issuer_did);

    let api = Router::new()
        .route("/register/begin", post(routes::register_begin))
        .route("/register/complete", post(routes::register_complete))
        .route("/auth/begin", post(routes::auth_begin))
        .route("/auth/complete", post(routes::auth_complete))
        .route("/credentials", get(routes::get_credentials))
        .route("/node/bind", post(routes::bind_node))
        .route("/health", get(routes::health))
        .with_state(shared);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", api)
        .route("/", get(serve_index))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&cfg.bind_addr).await?;
    tracing::info!("Listening on {}", cfg.bind_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
