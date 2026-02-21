//! Router configuration for the HTTP server.

use std::path::PathBuf;
use viewer_api::axum::{Router, routing::get};
use viewer_api::tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use crate::handlers::{get_session, list_logs, get_log, search_log, query_log, update_session};
use crate::source::get_source;
use crate::state::AppState;

/// Create the router with all routes
pub fn create_router(state: AppState, static_dir: Option<PathBuf>) -> Router {
    let mut router = Router::new()
        .route("/api/logs", get(list_logs))
        .route("/api/logs/:name", get(get_log))
        .route("/api/search/:name", get(search_log))
        .route("/api/query/:name", get(query_log))
        .route("/api/source/*path", get(get_source))
        .route("/api/session", get(get_session).post(update_session))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .with_state(state);
    
    // Only add static file serving if directory exists
    if let Some(dir) = static_dir {
        if dir.exists() {
            router = router.nest_service("/", ServeDir::new(&dir));
        }
    }
    
    router
}
