//! Ticket Viewer — HTTP server that serves the SPA frontend and proxies API
//! calls to a running `ticket serve` instance.
//!
//! # Usage
//!
//! ```bash
//! # Serve the built SPA (default port 3002, proxies /api to localhost:4000)
//! ticket-viewer
//!
//! # Custom ports
//! ticket-viewer --port 3002 --backend-url http://localhost:4000
//! ```
//!
//! # Environment variables
//! - `PORT`              — HTTP listen port (default: 3002)
//! - `TICKET_SERVE_URL` — URL of the running `ticket serve` backend (default: http://localhost:4000)
//! - `STATIC_DIR`       — Path to pre-built SPA static files (default: <manifest>/static)

use axum::{
    Router,
    routing::get,
    response::{IntoResponse, Json},
};
use std::{env, path::PathBuf};
use tracing::info;
use viewer_api::{display_host, init_tracing, with_static_files, ServerConfig};

mod proxy;

#[derive(Clone)]
struct AppState {
    backend_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3002);

    let backend_url = env::var("TICKET_SERVE_URL")
        .unwrap_or_else(|_| "http://localhost:4000".to_string());

    let static_dir: PathBuf = env::var("STATIC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static")
        });

    init_tracing("info");
    info!(port, %backend_url, static_dir = %static_dir.display(), "Ticket Viewer starting");

    let state = AppState { backend_url: backend_url.clone() };

    let api_router = Router::new()
        .route("/api/*path", get(proxy::proxy_get).post(proxy::proxy_post))
        .with_state(state);

    let health_router = Router::new()
        .route("/healthz", get(|| async { Json(serde_json::json!({ "status": "ok", "service": "ticket-viewer" })) }));

    let app = Router::new()
        .merge(health_router)
        .merge(api_router);

    let app = with_static_files(app, Some(static_dir).filter(|p| p.exists()));

    let addr = format!("0.0.0.0:{port}");
    info!("Listening on http://{}:{port}", display_host("0.0.0.0"));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
