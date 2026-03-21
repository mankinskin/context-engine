//! Route table for `ticket serve`.

use axum::{
    Router,
    middleware,
    routing::get,
};
use std::sync::Arc;

use viewer_api::{
    auth::bearer_auth_mw,
    middleware::request_id::add_request_id,
};

use super::{AppState, handlers};

/// Build the full Axum router.
pub fn build_router(state: AppState) -> Router {
    let auth_token_set = Arc::clone(&state.auth)
        .token_set_arc();

    // Public routes (no auth)
    let public_routes = Router::new()
        .route("/healthz", get(handlers::health::healthz));

    // Authenticated API routes
    let api_routes = Router::new()
        .route("/api/workspaces", get(handlers::workspaces::list_workspaces))
        .route("/api/tickets", get(handlers::tickets::list_tickets))
        .route("/api/tickets/{id}", get(handlers::tickets::get_ticket))
        .route("/api/edges", get(handlers::edges::list_edges))
        .route("/api/graph/subgraph", get(handlers::graph::subgraph))
        .route("/api/stream", get(handlers::stream::stream_handler))
        .layer(middleware::from_fn_with_state(
            auth_token_set,
            bearer_auth_mw,
        ));

    Router::new()
        .merge(public_routes)
        .merge(api_routes)
        .layer(middleware::from_fn(add_request_id))
        .with_state(state)
}
