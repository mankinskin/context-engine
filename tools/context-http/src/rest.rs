//! Convenience REST endpoints for common operations.
//!
//! These provide ergonomic HTTP access to frequently-used queries without
//! requiring callers to construct a full `Command` JSON body.  Every
//! endpoint here is a thin wrapper that builds the corresponding `Command`,
//! dispatches it through the workspace manager, and returns the relevant
//! slice of the `CommandResult`.

use context_api::{
    commands::{
        Command,
        CommandResult,
    },
    types::{
        AtomInfo,
        GraphStatistics,
        TokenInfo,
        WorkspaceInfo,
    },
};

use crate::error::HttpError;
use serde::Serialize;
use tracing::{
    info,
    instrument,
};
use viewer_api::axum::{
    extract::{
        Path,
        State,
    },
    Json,
};

use crate::state::AppState;

// ---------------------------------------------------------------------------
// GET /api/health
// ---------------------------------------------------------------------------

/// Health-check response.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}

/// `GET /api/health`
///
/// Always returns 200 if the server is running.
#[instrument(name = "health")]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

// ---------------------------------------------------------------------------
// GET /api/workspaces
// ---------------------------------------------------------------------------

/// `GET /api/workspaces`
///
/// List all available workspaces.  Convenience shorthand for
/// `POST /api/execute { "ListWorkspaces" }`.
#[instrument(name = "list_workspaces", skip(state))]
pub async fn list_workspaces(
    State(state): State<AppState>
) -> Result<Json<Vec<WorkspaceInfo>>, HttpError> {
    info!("Listing workspaces via REST");

    let manager = state.manager.clone();
    let result = tokio::task::spawn_blocking(
        move || -> Result<CommandResult, HttpError> {
            let mut mgr = manager
                .lock()
                .map_err(|e| HttpError::internal(e.to_string()))?;
            context_api::commands::execute(&mut mgr, Command::ListWorkspaces)
                .map_err(HttpError::from)
        },
    )
    .await
    .map_err(|e| HttpError::internal(format!("Task join error: {e}")))??;

    match result {
        CommandResult::WorkspaceInfoList { workspaces } => Ok(Json(workspaces)),
        other => Err(HttpError::internal(format!(
            "Unexpected result variant: {:?}",
            std::mem::discriminant(&other)
        ))),
    }
}

// ---------------------------------------------------------------------------
// GET /api/workspaces/:name/snapshot
// ---------------------------------------------------------------------------

/// `GET /api/workspaces/:name/snapshot`
///
/// Get the full graph snapshot for a workspace.
///
/// The response is the raw JSON representation of the snapshot (a
/// `CommandResult::Snapshot` value) because `GraphSnapshot` lives in
/// `context-trace` and may not serialise to a predictable shape for
/// direct extraction.  Returning `serde_json::Value` keeps the endpoint
/// simple and forward-compatible.
#[instrument(name = "get_snapshot", skip(state))]
pub async fn get_snapshot(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<serde_json::Value>, HttpError> {
    info!(workspace = %name, "Getting snapshot via REST");

    let manager = state.manager.clone();
    let result = tokio::task::spawn_blocking(
        move || -> Result<CommandResult, HttpError> {
            let mut mgr = manager
                .lock()
                .map_err(|e| HttpError::internal(e.to_string()))?;
            context_api::commands::execute(
                &mut mgr,
                Command::GetSnapshot { workspace: name },
            )
            .map_err(HttpError::from)
        },
    )
    .await
    .map_err(|e| HttpError::internal(format!("Task join error: {e}")))??;

    // Serialize the entire CommandResult::Snapshot variant to a JSON Value.
    // This avoids needing to import the concrete `GraphSnapshot` type from
    // `context-trace`.
    match result {
        CommandResult::Snapshot(snapshot) => {
            let value = serde_json::to_value(&snapshot)
                .map_err(|e| HttpError::internal(e.to_string()))?;
            Ok(Json(value))
        },
        other => Err(HttpError::internal(format!(
            "Unexpected result variant: {:?}",
            std::mem::discriminant(&other)
        ))),
    }
}

// ---------------------------------------------------------------------------
// GET /api/workspaces/:name/atoms
// ---------------------------------------------------------------------------

/// `GET /api/workspaces/:name/atoms`
///
/// List all atoms in a workspace.
#[instrument(name = "list_atoms", skip(state))]
pub async fn list_atoms(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<Vec<AtomInfo>>, HttpError> {
    info!(workspace = %name, "Listing atoms via REST");

    let manager = state.manager.clone();
    let result = tokio::task::spawn_blocking(
        move || -> Result<CommandResult, HttpError> {
            let mut mgr = manager
                .lock()
                .map_err(|e| HttpError::internal(e.to_string()))?;
            context_api::commands::execute(
                &mut mgr,
                Command::ListAtoms { workspace: name },
            )
            .map_err(HttpError::from)
        },
    )
    .await
    .map_err(|e| HttpError::internal(format!("Task join error: {e}")))??;

    match result {
        CommandResult::AtomInfoList { atoms } => Ok(Json(atoms)),
        other => Err(HttpError::internal(format!(
            "Unexpected result variant: {:?}",
            std::mem::discriminant(&other)
        ))),
    }
}

// ---------------------------------------------------------------------------
// GET /api/workspaces/:name/vertices
// ---------------------------------------------------------------------------

/// `GET /api/workspaces/:name/vertices`
///
/// List all vertices (tokens) in a workspace.
#[instrument(name = "list_vertices", skip(state))]
pub async fn list_vertices(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<Vec<TokenInfo>>, HttpError> {
    info!(workspace = %name, "Listing vertices via REST");

    let manager = state.manager.clone();
    let result = tokio::task::spawn_blocking(
        move || -> Result<CommandResult, HttpError> {
            let mut mgr = manager
                .lock()
                .map_err(|e| HttpError::internal(e.to_string()))?;
            context_api::commands::execute(
                &mut mgr,
                Command::ListVertices { workspace: name },
            )
            .map_err(HttpError::from)
        },
    )
    .await
    .map_err(|e| HttpError::internal(format!("Task join error: {e}")))??;

    match result {
        CommandResult::TokenInfoList { tokens } => Ok(Json(tokens)),
        other => Err(HttpError::internal(format!(
            "Unexpected result variant: {:?}",
            std::mem::discriminant(&other)
        ))),
    }
}

// ---------------------------------------------------------------------------
// GET /api/workspaces/:name/statistics
// ---------------------------------------------------------------------------

/// `GET /api/workspaces/:name/statistics`
///
/// Get graph statistics for a workspace.
#[instrument(name = "get_statistics", skip(state))]
pub async fn get_statistics(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<GraphStatistics>, HttpError> {
    info!(workspace = %name, "Getting statistics via REST");

    let manager = state.manager.clone();
    let result = tokio::task::spawn_blocking(
        move || -> Result<CommandResult, HttpError> {
            let mut mgr = manager
                .lock()
                .map_err(|e| HttpError::internal(e.to_string()))?;
            context_api::commands::execute(
                &mut mgr,
                Command::GetStatistics { workspace: name },
            )
            .map_err(HttpError::from)
        },
    )
    .await
    .map_err(|e| HttpError::internal(format!("Task join error: {e}")))??;

    match result {
        CommandResult::Statistics(stats) => Ok(Json(stats)),
        other => Err(HttpError::internal(format!(
            "Unexpected result variant: {:?}",
            std::mem::discriminant(&other)
        ))),
    }
}
