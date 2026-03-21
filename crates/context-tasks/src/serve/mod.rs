//! HTTP serve mode for `ticket serve`.
//!
//! Exposes the ticket store over a REST/SSE API with bearer-token auth.
//! See `api-contract-v0.1.md` (ticket `21a1b9ca`) for the full endpoint spec.

pub mod auth_state;
pub mod error;
mod handlers;
pub mod registry;
mod routes;

use std::{net::SocketAddr, sync::Arc};

use tokio::net::TcpListener;

pub use auth_state::AuthState;
pub use registry::WorkspaceRegistry;

/// Configuration for `ticket serve`.
#[derive(Debug, Clone)]
pub struct ServeConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServeConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}

impl ServeConfig {
    pub fn addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("valid socket address")
    }
}

/// Shared application state passed to all Axum handlers.
#[derive(Clone)]
pub struct AppState {
    pub registry: Arc<WorkspaceRegistry>,
    pub auth: Arc<AuthState>,
}

/// Start the HTTP server.
pub async fn serve(
    config: ServeConfig,
    registry: WorkspaceRegistry,
    auth: AuthState,
) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState {
        registry: Arc::new(registry),
        auth: Arc::new(auth),
    };

    let app = routes::build_router(state);
    let addr = config.addr();
    let listener = TcpListener::bind(addr).await?;

    eprintln!("ticket serve listening on http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
