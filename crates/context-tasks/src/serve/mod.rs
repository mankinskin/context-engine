//! HTTP serve mode for `ticket serve`.
//!
//! Exposes the ticket store over a REST/SSE API with bearer-token auth.
//! See `api-contract-v0.1.md` (ticket `21a1b9ca`) for the full endpoint spec.

pub mod auth_state;
pub mod error;
mod handlers;
pub mod registry;
mod routes;
pub mod stream;

use std::{net::SocketAddr, sync::Arc};

use tokio::net::TcpListener;

pub use auth_state::AuthState;
pub use registry::WorkspaceRegistry;
pub use stream::{HookEmitter, StreamBroker};

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
    pub broker: Arc<StreamBroker>,
}

/// Start the HTTP server.
///
/// Creates a `StreamBroker`, wires up per-workspace `HookEmitter`s and
/// reconcile loops, then starts Axum.
pub async fn serve(
    config: ServeConfig,
    registry: WorkspaceRegistry,
    auth: AuthState,
) -> Result<(), Box<dyn std::error::Error>> {
    let broker = Arc::new(StreamBroker::new());

    // Pre-create channels and reconcile loops for every known workspace.
    let workspace_names = registry.workspace_names();
    for ws in &workspace_names {
        broker.ensure_channel(ws);
        if let Some(store) = registry.get(ws) {
            let emitter = HookEmitter::new(ws.clone(), Arc::clone(&broker));

            // Attach hook emitter to the store so mutations fan out events.
            store.set_hook(emitter.clone());

            // Start the background reconcile loop.
            stream::reconcile::spawn_reconcile(store, emitter);
        }
    }

    let state = AppState {
        registry: Arc::new(registry),
        auth: Arc::new(auth),
        broker,
    };

    let app = routes::build_router(state);
    let addr = config.addr();
    let listener = TcpListener::bind(addr).await?;

    eprintln!("ticket serve listening on http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
