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
    response::Json,
};
use std::{
    env,
    path::PathBuf,
    process::{Child, Command},
};
use tracing::info;
use viewer_api::{display_host, init_tracing, with_static_files};

mod proxy;

#[derive(Clone)]
struct AppState {
    backend_url: String,
}

struct CliOptions {
    port: u16,
    backend_url: String,
    static_dir: PathBuf,
    auto_start_backend: bool,
}

struct BackendProcess {
    child: Child,
}

impl Drop for BackendProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn parse_cli_options() -> CliOptions {
    let mut port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3002);

    let mut backend_url = env::var("TICKET_SERVE_URL")
        .unwrap_or_else(|_| "http://localhost:4000".to_string());

    let mut static_dir: PathBuf = env::var("STATIC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static"));

    let mut auto_start_backend = false;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--port" => {
                if let Some(value) = args.next() {
                    if let Ok(parsed) = value.parse::<u16>() {
                        port = parsed;
                    }
                }
            }
            "--backend-url" => {
                if let Some(value) = args.next() {
                    backend_url = value;
                }
            }
            "--static-dir" => {
                if let Some(value) = args.next() {
                    static_dir = PathBuf::from(value);
                }
            }
            "--auto-start-backend" => {
                auto_start_backend = true;
            }
            _ => {}
        }
    }

    CliOptions {
        port,
        backend_url,
        static_dir,
        auto_start_backend,
    }
}

fn start_backend_process() -> Result<BackendProcess, Box<dyn std::error::Error>> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;

    let child = Command::new("cargo")
        .args([
            "run",
            "-p",
            "context-tasks",
            "--bin",
            "ticket",
            "--",
            "serve",
            "--port",
            "4000",
        ])
        .current_dir(workspace_root)
        .spawn()?;

    Ok(BackendProcess { child })
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = parse_cli_options();

    init_tracing("info");
    info!(
        port = options.port,
        backend_url = %options.backend_url,
        static_dir = %options.static_dir.display(),
        auto_start_backend = options.auto_start_backend,
        "Ticket Viewer starting"
    );

    let _backend = if options.auto_start_backend {
        info!("Starting ticket backend on http://localhost:4000");
        Some(start_backend_process()?)
    } else {
        None
    };

    let state = AppState { backend_url: options.backend_url.clone() };

    let api_router = Router::new()
        .route("/api/*path", get(proxy::proxy_get).post(proxy::proxy_post))
        .with_state(state);

    let health_router = Router::new()
        .route("/healthz", get(|| async { Json(serde_json::json!({ "status": "ok", "service": "ticket-viewer" })) }));

    let app = Router::new()
        .merge(health_router)
        .merge(api_router);

    let app = with_static_files(app, Some(options.static_dir).filter(|p| p.exists()));

    let addr = format!("0.0.0.0:{}", options.port);
    info!("Listening on http://{}:{}", display_host("0.0.0.0"), options.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}
