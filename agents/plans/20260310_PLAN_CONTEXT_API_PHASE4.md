---
tags: `#context-api` `#phase4` `#http` `#graphql` `#adapter` `#axum`
summary: Phase 4 — Create HTTP adapter with RPC endpoint and optional GraphQL schema, reusing viewer-api server infrastructure
status: 📋
---

# Plan: context-api Phase 4 — HTTP + GraphQL Adapter

## Objective

Create `tools/context-http`, a thin binary crate that exposes the entire `context-api` command surface over HTTP. The primary interface is an RPC-style `POST /api/execute` endpoint that accepts a `Command` JSON body and returns a `CommandResult` JSON response — exactly mirroring the MCP adapter's semantics but over HTTP. Additionally, provide optional GraphQL support for typed queries, REST-like convenience endpoints for common operations, and reuse the `viewer-api` shared server infrastructure (CORS, static file serving, tracing, dev proxy) already established in the workspace.

## Context

### Prerequisites

- **Phase 1 complete** — `crates/context-api` exists with workspace management, atom/pattern commands, persistence.
- **Phase 2 complete** — Algorithm commands (search, insert, read) and the full `Command` enum with `execute()` dispatch are in place.
- **Phase 3 complete** (recommended, not required) — MCP adapter validates the `Command`/`CommandResult` JSON contract end-to-end.

### Interview Reference

- `agents/interviews/20260310_INTERVIEW_CONTEXT_API.md` — Q13 (HTTP: **RPC-style `POST /api/execute`**), Q14 (GraphQL: **optional typed query layer**)
- Master plan: `agents/plans/20260310_PLAN_CONTEXT_API_OVERVIEW.md`

### Key Decisions Affecting This Phase

- **RPC-first** — `POST /api/execute` is the primary endpoint, accepting the full `Command` enum as JSON. Same contract as the MCP `execute` tool.
- **GraphQL optional** — A secondary query layer for typed, composable queries. Not required for initial launch but planned.
- **Reuse `viewer-api`** — Use `ServerConfig`, `run_server`, `default_cors`, `with_static_files`, `init_tracing` from the shared `viewer-api` crate. Follow the same `--http`/`--mcp` flag pattern.
- **Separate binary crate** — `tools/context-http` depends on `context-api` as a library.
- **State sharing** — `WorkspaceManager` wrapped in `Arc<WorkspaceManager>` for concurrent HTTP handler access. The manager's internal locking (multi-reader/single-writer) provides thread safety.

### Dependencies (External Crates)

| Crate | Version | Purpose |
|-------|---------|---------|
| `context-api` | path | All workspace/graph/algorithm commands |
| `viewer-api` | path | Shared HTTP server infra (axum, CORS, tracing, static serve) |
| `axum` | 0.8 (via viewer-api re-export) | HTTP framework |
| `serde` | 1 | Serialization |
| `serde_json` | 1 | JSON command/result encoding |
| `tokio` | 1 | Async runtime |
| `tracing` | 0.1 | Structured logging |
| `tower-http` | 0.6 (via viewer-api re-export) | CORS middleware |
| `async-graphql` | 7 | GraphQL schema + execution (optional, feature-gated) |
| `async-graphql-axum` | 7 | GraphQL axum integration (optional, feature-gated) |

### Files Affected

All files are **new** (greenfield):

**Workspace root:**
- `Cargo.toml` — add `tools/context-http` to `[workspace.members]`

**`tools/context-http/`:**
- `Cargo.toml`
- `src/main.rs` — entry point, tracing init, launch HTTP server
- `src/state.rs` — `AppState` wrapping `Arc<WorkspaceManager>`
- `src/router.rs` — route definitions, axum `Router` construction
- `src/rpc.rs` — `POST /api/execute` handler
- `src/rest.rs` — convenience REST endpoints (health, workspace listing)
- `src/graphql.rs` — GraphQL schema + handler (feature-gated behind `graphql`)
- `src/error.rs` — HTTP error response mapping (`ApiError` → status codes + JSON)

---

## Analysis

### Current State (After Phase 2)

The `context-api` crate provides:
- `WorkspaceManager` with all workspace lifecycle operations
- Full `Command` enum with `execute()` dispatch → `Result<CommandResult, ApiError>`
- All types derive `Serialize`/`Deserialize`
- All error types with descriptive messages

The `viewer-api` crate provides:
- `ServerConfig` — port, host, static dir configuration
- `run_server()` — HTTP/MCP mode dispatch with `--http`/`--mcp` flags
- `default_cors()` — permissive CORS for development
- `with_static_files()` — optional static file serving
- `init_tracing()` / `init_tracing_full()` — logging setup
- `dev_proxy` — reverse proxy to Vite dev server for frontend development

The `log-viewer` demonstrates the established HTTP server pattern:
- `AppState` struct holding application-specific data
- `create_router()` function building an axum `Router` with `.with_state(state)`
- Route handlers as async functions taking axum extractors
- JSON responses via `axum::Json`

### Desired State

A standalone `context-http` binary that:
1. Accepts any `Command` via `POST /api/execute` and returns `CommandResult`
2. Provides convenience endpoints: `GET /api/health`, `GET /api/workspaces`
3. Optionally exposes a GraphQL endpoint at `POST /api/graphql` (feature-gated)
4. Serves a future web frontend from static files (optional)
5. Follows the exact same patterns as `log-viewer` for consistency
6. Maps `ApiError` variants to appropriate HTTP status codes

### HTTP Status Code Mapping

| ApiError Variant | HTTP Status | Reason |
|-----------------|-------------|--------|
| `Workspace(NotFound)` | 404 | Resource doesn't exist |
| `Workspace(AlreadyExists)` | 409 | Conflict |
| `Workspace(NotOpen)` | 400 | Client must open first |
| `Workspace(LockConflict)` | 423 | Locked by another process |
| `Workspace(IoError)` | 500 | Server-side I/O failure |
| `Workspace(SerializationError)` | 500 | Server-side failure |
| `Atom(WorkspaceNotOpen)` | 400 | Client error |
| `Atom(InvalidChar)` | 422 | Unprocessable input |
| `Pattern(AtomNotFound)` | 404 | Referenced atom missing |
| `Pattern(TooShort)` | 422 | Invalid input |
| `Pattern(AtomAlreadyInPattern)` | 409 | Conflict |
| `Pattern(DuplicateAtomInInput)` | 422 | Invalid input |
| `Search(TokenNotFound)` | 404 | Referenced token missing |
| `Search(QueryTooShort)` | 422 | Invalid input |
| `Search(InternalError)` | 500 | Server-side failure |
| `Insert(*)` | Same as Search | Same pattern |
| `Read(VertexNotFound)` | 404 | Referenced vertex missing |
| `Read(InternalError)` | 500 | Server-side failure |

### Why GraphQL is Feature-Gated

GraphQL adds significant compile-time dependencies (`async-graphql` + proc macros). Not all consumers need it. By feature-gating:
- Default builds are lean and fast
- Web frontend developers opt in with `--features graphql`
- The RPC endpoint covers 100% of functionality regardless

---

## Execution Steps

### Step 1: Add to Workspace

Add `tools/context-http` to the root `Cargo.toml`:

```toml
[workspace]
members = [
    # ... existing members ...
    "tools/context-http",
]
```

- [ ] Edit root `Cargo.toml`
- [ ] Verification: `cargo check --workspace` still passes (new crate doesn't exist yet, so it will error — that's fine, just ensure no other breakage)

---

### Step 2: Create Cargo.toml

```toml
[package]
name = "context-http"
version = "0.1.0"
edition = "2024"
description = "HTTP + GraphQL adapter for context-engine API"

[[bin]]
name = "context-http"
path = "src/main.rs"

[features]
default = []
graphql = ["dep:async-graphql", "dep:async-graphql-axum"]

[dependencies]
context-api = { path = "../../crates/context-api" }
viewer-api = { path = "../viewer-api" }

axum = { version = "0.8", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tower-http = { version = "0.6", features = ["cors"] }

# Optional GraphQL support
async-graphql = { version = "7", optional = true }
async-graphql-axum = { version = "7", optional = true }
```

- [ ] Create `tools/context-http/Cargo.toml`
- [ ] Verification: `cargo check -p context-http` (will fail until src exists)

---

### Step 3: Application State

Create `src/state.rs`:

```pseudo
use std::sync::Arc;
use context_api::WorkspaceManager;

/// Shared application state for HTTP handlers.
#[derive(Clone)]
pub struct AppState {
    /// The workspace manager (thread-safe via internal locking)
    pub manager: Arc<WorkspaceManager>,
}

impl AppState {
    pub fn new(manager: WorkspaceManager) -> Self {
        Self {
            manager: Arc::new(manager),
        }
    }
}
```

- [ ] Create `tools/context-http/src/state.rs`

---

### Step 4: HTTP Error Response Mapping

Create `src/error.rs`:

```pseudo
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use context_api::error::ApiError;
use serde::Serialize;

/// JSON error response body.
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    pub details: Option<serde_json::Value>,
}

/// Map ApiError to HTTP status code.
fn status_for_error(err: &ApiError) -> StatusCode {
    match err {
        ApiError::Workspace(e) => match e {
            WorkspaceError::NotFound { .. } => StatusCode::NOT_FOUND,
            WorkspaceError::AlreadyExists { .. } => StatusCode::CONFLICT,
            WorkspaceError::NotOpen { .. } => StatusCode::BAD_REQUEST,
            WorkspaceError::LockConflict { .. } => StatusCode::LOCKED,
            WorkspaceError::IoError(_) | WorkspaceError::SerializationError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        },
        ApiError::Atom(e) => match e {
            AtomError::WorkspaceNotOpen { .. } => StatusCode::BAD_REQUEST,
            AtomError::InvalidChar { .. } => StatusCode::UNPROCESSABLE_ENTITY,
        },
        ApiError::Pattern(e) => match e {
            PatternError::AtomNotFound { .. } => StatusCode::NOT_FOUND,
            PatternError::TooShort { .. } | PatternError::DuplicateAtomInInput { .. } => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
            PatternError::AtomAlreadyInPattern { .. } => StatusCode::CONFLICT,
            PatternError::WorkspaceNotOpen { .. } => StatusCode::BAD_REQUEST,
        },
        ApiError::Search(e) => match e {
            SearchError::TokenNotFound { .. } => StatusCode::NOT_FOUND,
            SearchError::QueryTooShort => StatusCode::UNPROCESSABLE_ENTITY,
            SearchError::WorkspaceNotOpen { .. } => StatusCode::BAD_REQUEST,
            SearchError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        },
        ApiError::Insert(e) => match e {
            InsertError::TokenNotFound { .. } => StatusCode::NOT_FOUND,
            InsertError::WorkspaceNotOpen { .. } => StatusCode::BAD_REQUEST,
            InsertError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        },
        ApiError::Read(e) => match e {
            ReadError::VertexNotFound { .. } => StatusCode::NOT_FOUND,
            ReadError::WorkspaceNotOpen { .. } => StatusCode::BAD_REQUEST,
            ReadError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        },
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = status_for_error(&self);
        let body = ErrorResponse {
            error: self.to_string(),
            code: format!("{:?}", self),  // Variant name as code
            details: None,
        };
        (status, Json(body)).into_response()
    }
}
```

- [ ] Create `tools/context-http/src/error.rs`

---

### Step 5: RPC Endpoint

Create `src/rpc.rs` — the primary command dispatch endpoint:

```pseudo
use axum::{extract::State, Json};
use context_api::{Command, CommandResult};
use tracing::{info, warn};

use crate::state::AppState;
use crate::error::ErrorResponse;

/// POST /api/execute
///
/// Accepts a Command JSON body, dispatches it through the WorkspaceManager,
/// and returns a CommandResult JSON response.
///
/// # Request Body
/// ```json
/// {"command": "create_workspace", "name": "my-graph"}
/// ```
///
/// # Response
/// - 200 OK with CommandResult JSON on success
/// - 4xx/5xx with ErrorResponse JSON on failure
pub async fn execute(
    State(state): State<AppState>,
    Json(command): Json<Command>,
) -> Result<Json<CommandResult>, ApiError> {
    info!(?command, "Executing command via HTTP RPC");

    let result = state.manager.execute(command).await?;

    Ok(Json(result))
}
```

Note: If `WorkspaceManager::execute` is synchronous (likely, since it's in-memory with file-locking), wrap in `tokio::task::spawn_blocking` to avoid blocking the async runtime:

```pseudo
pub async fn execute(
    State(state): State<AppState>,
    Json(command): Json<Command>,
) -> Result<Json<CommandResult>, ApiError> {
    info!(?command, "Executing command via HTTP RPC");

    let manager = state.manager.clone();
    let result = tokio::task::spawn_blocking(move || {
        manager.execute(command)
    })
    .await
    .map_err(|e| ApiError::internal(format!("Task join error: {}", e)))??;

    Ok(Json(result))
}
```

- [ ] Create `tools/context-http/src/rpc.rs`

---

### Step 6: Convenience REST Endpoints

Create `src/rest.rs` — lightweight REST endpoints for common operations that don't need the full Command enum:

```pseudo
use axum::{extract::State, Json};
use serde::Serialize;

use crate::state::AppState;

/// GET /api/health
///
/// Health check endpoint. Always returns 200 if the server is running.
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

/// GET /api/workspaces
///
/// List all available workspaces. Convenience shorthand for
/// `POST /api/execute {"command": "list_workspaces"}`.
pub async fn list_workspaces(
    State(state): State<AppState>,
) -> Result<Json<Vec<WorkspaceInfo>>, ApiError> {
    let manager = state.manager.clone();
    let workspaces = tokio::task::spawn_blocking(move || {
        manager.list_workspaces()
    })
    .await
    .map_err(|e| ApiError::internal(format!("Task join error: {}", e)))??;

    Ok(Json(workspaces))
}

/// GET /api/workspaces/:name/snapshot
///
/// Get graph snapshot for a workspace. Convenience shorthand for
/// `POST /api/execute {"command": "get_snapshot", "workspace": "<name>"}`.
pub async fn get_snapshot(
    State(state): State<AppState>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> Result<Json<GraphSnapshot>, ApiError> {
    let manager = state.manager.clone();
    let snapshot = tokio::task::spawn_blocking(move || {
        manager.get_snapshot(&name)
    })
    .await
    .map_err(|e| ApiError::internal(format!("Task join error: {}", e)))??;

    Ok(Json(snapshot))
}

/// GET /api/workspaces/:name/atoms
///
/// List all atoms in a workspace. Convenience shorthand.
pub async fn list_atoms(
    State(state): State<AppState>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> Result<Json<Vec<AtomInfo>>, ApiError> {
    let manager = state.manager.clone();
    let atoms = tokio::task::spawn_blocking(move || {
        manager.list_atoms(&name)
    })
    .await
    .map_err(|e| ApiError::internal(format!("Task join error: {}", e)))??;

    Ok(Json(atoms))
}

/// GET /api/workspaces/:name/statistics
///
/// Get graph statistics. Convenience shorthand.
pub async fn get_statistics(
    State(state): State<AppState>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> Result<Json<GraphStatistics>, ApiError> {
    let manager = state.manager.clone();
    let stats = tokio::task::spawn_blocking(move || {
        manager.get_statistics(&name)
    })
    .await
    .map_err(|e| ApiError::internal(format!("Task join error: {}", e)))??;

    Ok(Json(stats))
}
```

- [ ] Create `tools/context-http/src/rest.rs`

---

### Step 7: Router Construction

Create `src/router.rs` — assemble all routes into an axum `Router`:

```pseudo
use axum::{routing::{get, post}, Router};
use viewer_api::default_cors;

use crate::state::AppState;
use crate::rpc;
use crate::rest;

/// Create the HTTP router with all routes.
///
/// Follows the same pattern as log-viewer's `create_router()`.
pub fn create_router(state: AppState) -> Router {
    let mut router = Router::new()
        // Primary RPC endpoint — handles ALL commands
        .route("/api/execute", post(rpc::execute))
        // Convenience REST endpoints
        .route("/api/health", get(rest::health))
        .route("/api/workspaces", get(rest::list_workspaces))
        .route("/api/workspaces/:name/snapshot", get(rest::get_snapshot))
        .route("/api/workspaces/:name/atoms", get(rest::list_atoms))
        .route("/api/workspaces/:name/statistics", get(rest::get_statistics))
        .layer(default_cors());

    // Conditionally add GraphQL routes
    #[cfg(feature = "graphql")]
    {
        router = crate::graphql::add_graphql_routes(router, state.clone());
    }

    router.with_state(state)
}
```

- [ ] Create `tools/context-http/src/router.rs`

---

### Step 8: GraphQL Schema (Feature-Gated)

Create `src/graphql.rs` — optional GraphQL endpoint:

```pseudo
#![cfg(feature = "graphql")]

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, routing::{get, post}, Router};

use crate::state::AppState;

/// GraphQL query root.
///
/// Provides typed read-only queries into the workspace.
/// Mutations go through the RPC endpoint (POST /api/execute) — keeping
/// a single source of truth for write operations.
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// List all available workspaces.
    async fn workspaces(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<WorkspaceInfo>> {
        let state = ctx.data::<AppState>()?;
        let manager = state.manager.clone();
        let workspaces = tokio::task::spawn_blocking(move || {
            manager.list_workspaces()
        })
        .await??;
        Ok(workspaces)
    }

    /// Get info about a specific workspace.
    async fn workspace(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> async_graphql::Result<Option<WorkspaceInfo>> {
        let state = ctx.data::<AppState>()?;
        let manager = state.manager.clone();
        let info = tokio::task::spawn_blocking(move || {
            manager.open_workspace(&name).ok()
        })
        .await?;
        Ok(info)
    }

    /// List all atoms in a workspace.
    async fn atoms(
        &self,
        ctx: &Context<'_>,
        workspace: String,
    ) -> async_graphql::Result<Vec<AtomInfo>> {
        let state = ctx.data::<AppState>()?;
        let manager = state.manager.clone();
        let atoms = tokio::task::spawn_blocking(move || {
            manager.list_atoms(&workspace)
        })
        .await??;
        Ok(atoms)
    }

    /// List all vertices (tokens) in a workspace.
    async fn vertices(
        &self,
        ctx: &Context<'_>,
        workspace: String,
    ) -> async_graphql::Result<Vec<TokenInfo>> {
        let state = ctx.data::<AppState>()?;
        let manager = state.manager.clone();
        let tokens = tokio::task::spawn_blocking(move || {
            manager.list_vertices(&workspace)
        })
        .await??;
        Ok(tokens)
    }

    /// Get graph statistics for a workspace.
    async fn statistics(
        &self,
        ctx: &Context<'_>,
        workspace: String,
    ) -> async_graphql::Result<GraphStatistics> {
        let state = ctx.data::<AppState>()?;
        let manager = state.manager.clone();
        let stats = tokio::task::spawn_blocking(move || {
            manager.get_statistics(&workspace)
        })
        .await??;
        Ok(stats)
    }

    /// Get graph snapshot for a workspace.
    async fn snapshot(
        &self,
        ctx: &Context<'_>,
        workspace: String,
    ) -> async_graphql::Result<GraphSnapshot> {
        let state = ctx.data::<AppState>()?;
        let manager = state.manager.clone();
        let snapshot = tokio::task::spawn_blocking(move || {
            manager.get_snapshot(&workspace)
        })
        .await??;
        Ok(snapshot)
    }

    /// Read a pattern as text.
    async fn read_as_text(
        &self,
        ctx: &Context<'_>,
        workspace: String,
        index: usize,
    ) -> async_graphql::Result<String> {
        let state = ctx.data::<AppState>()?;
        let manager = state.manager.clone();
        let text = tokio::task::spawn_blocking(move || {
            manager.read_as_text(&workspace, index)
        })
        .await??;
        Ok(text)
    }
}

/// Type alias for the GraphQL schema.
pub type ContextSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

/// Build the GraphQL schema with application state embedded.
pub fn build_schema(state: AppState) -> ContextSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(state)
        .finish()
}

/// Handler for POST /api/graphql
async fn graphql_handler(
    State(schema): State<ContextSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// Handler for GET /api/graphql — GraphQL Playground (development)
async fn graphql_playground() -> impl axum::response::IntoResponse {
    axum::response::Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/api/graphql"),
    ))
}

/// Add GraphQL routes to the router.
pub fn add_graphql_routes(router: Router<AppState>, state: AppState) -> Router<AppState> {
    let schema = build_schema(state);

    router
        .route("/api/graphql", post(graphql_handler).get(graphql_playground))
        .with_state(schema)
}
```

**Design note:** The GraphQL schema is **query-only**. All mutations go through `POST /api/execute`. This avoids duplicating write logic and keeps the `Command` enum as the single source of truth for all state-changing operations.

- [ ] Create `tools/context-http/src/graphql.rs`

---

### Step 9: Main Entry Point

Create `src/main.rs`:

```pseudo
mod error;
mod graphql;
mod rest;
mod router;
mod rpc;
mod state;

use std::path::PathBuf;

use context_api::WorkspaceManager;
use viewer_api::{
    init_tracing, ServerConfig, TracingConfig,
};
use tracing::info;

use crate::state::AppState;
use crate::router::create_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing (console logging)
    let tracing_config = TracingConfig::from_env("context-http", PathBuf::from("logs"))
        .with_level("info");
    viewer_api::init_tracing_full(&tracing_config);

    info!("context-http starting...");

    // Determine workspace base directory
    // Default: ./.context-engine/ (project-local)
    let base_dir = std::env::var("CONTEXT_ENGINE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(".context-engine"));

    info!(?base_dir, "Workspace storage directory");

    // Create workspace manager
    let manager = WorkspaceManager::new(base_dir)?;

    // Create application state
    let state = AppState::new(manager);

    // Configure server
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3100);

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    let addr = format!("{}:{}", host, port);

    info!("Starting HTTP server on http://{}", addr);

    // Build router
    let app = create_router(state);

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("HTTP server listening on http://{}", addr);
    #[cfg(feature = "graphql")]
    info!("GraphQL Playground available at http://{}/api/graphql", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
```

- [ ] Create `tools/context-http/src/main.rs`

---

### Step 10: Integration Tests

Create tests that exercise the HTTP endpoints using `axum::test`:

```pseudo
// tests/http_tests.rs

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt; // for oneshot

use context_api::WorkspaceManager;
use context_http::router::create_router;
use context_http::state::AppState;

/// Helper: create test app with a temp directory
fn test_app() -> Router {
    let dir = tempfile::tempdir().unwrap();
    let manager = WorkspaceManager::new(dir.path().to_path_buf()).unwrap();
    let state = AppState::new(manager);
    create_router(state)
}

#[tokio::test]
async fn health_check() {
    let app = test_app();
    let resp = app
        .oneshot(Request::get("/api/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    // Parse body and check status == "ok"
}

#[tokio::test]
async fn execute_create_workspace() {
    let app = test_app();
    let resp = app
        .oneshot(
            Request::post("/api/execute")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"command":"create_workspace","name":"test-ws"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn execute_workspace_not_found() {
    let app = test_app();
    let resp = app
        .oneshot(
            Request::post("/api/execute")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"command":"list_atoms","workspace":"nonexistent"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    // Should get 404 or 400 depending on error mapping
    assert!(resp.status().is_client_error());
}

#[tokio::test]
async fn execute_round_trip() {
    let app = test_app();

    // 1. Create workspace
    // 2. Add atoms
    // 3. Add simple pattern
    // 4. Insert sequence
    // 5. Search sequence
    // 6. Read as text
    // 7. Save workspace
    // Each step sends POST /api/execute and validates the response
}

#[tokio::test]
async fn list_workspaces_rest() {
    let app = test_app();

    // Create a workspace via RPC first
    app.clone()
        .oneshot(
            Request::post("/api/execute")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"command":"create_workspace","name":"rest-test"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    // Then list via REST
    let resp = app
        .oneshot(Request::get("/api/workspaces").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    // Parse body and check that "rest-test" is in the list
}

#[tokio::test]
async fn malformed_json_returns_422() {
    let app = test_app();
    let resp = app
        .oneshot(
            Request::post("/api/execute")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"not_a_command": true}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[cfg(feature = "graphql")]
#[tokio::test]
async fn graphql_workspaces_query() {
    let app = test_app();
    let resp = app
        .oneshot(
            Request::post("/api/graphql")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"query":"{ workspaces { name } }"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
```

- [ ] Create `tools/context-http/tests/http_tests.rs`
- [ ] Verification: `cargo test -p context-http`

---

### Step 11: Documentation

Create a brief README for the crate:

```pseudo
# context-http

HTTP + GraphQL adapter for the context-engine API.

## Quick Start

```bash
# Start HTTP server (default port 3100)
cargo run -p context-http

# With GraphQL enabled
cargo run -p context-http --features graphql

# Custom port and host
PORT=8080 HOST=0.0.0.0 cargo run -p context-http
```

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/execute` | Execute any Command (RPC) |
| GET | `/api/health` | Health check |
| GET | `/api/workspaces` | List workspaces |
| GET | `/api/workspaces/:name/snapshot` | Get graph snapshot |
| GET | `/api/workspaces/:name/atoms` | List atoms |
| GET | `/api/workspaces/:name/statistics` | Graph statistics |
| POST | `/api/graphql` | GraphQL endpoint (requires `graphql` feature) |
| GET | `/api/graphql` | GraphQL Playground (requires `graphql` feature) |

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3100` | HTTP port |
| `HOST` | `127.0.0.1` | Bind address |
| `CONTEXT_ENGINE_DIR` | `.context-engine` | Workspace storage directory |
| `LOG_LEVEL` | `info` | Log level (trace, debug, info, warn, error) |
| `LOG_FILE` | unset | Set to enable file logging |
```

- [ ] Create `tools/context-http/README.md`

---

### Step 12: Final Verification

- [ ] `cargo check --workspace` — no errors
- [ ] `cargo test -p context-http` — all tests pass
- [ ] `cargo build -p context-http` — binary builds
- [ ] `cargo build -p context-http --features graphql` — binary builds with GraphQL
- [ ] Manual test: `cargo run -p context-http` → `curl http://localhost:3100/api/health` → 200 OK
- [ ] Manual test: `curl -X POST http://localhost:3100/api/execute -H 'Content-Type: application/json' -d '{"command":"create_workspace","name":"http-test"}'` → 200 with WorkspaceInfo
- [ ] Manual test: `curl http://localhost:3100/api/workspaces` → 200 with workspace list
- [ ] Manual test (GraphQL): `cargo run -p context-http --features graphql` → visit `http://localhost:3100/api/graphql` → Playground loads
- [ ] CORS headers present in responses (test with `Origin` header)

---

## API Reference

### POST /api/execute

The primary endpoint. Accepts the exact same `Command` JSON that the MCP `execute` tool accepts.

**Request:**
```json
POST /api/execute
Content-Type: application/json

{"command": "create_workspace", "name": "my-graph"}
```

**Success Response:**
```json
HTTP/1.1 200 OK
Content-Type: application/json

{
  "WorkspaceInfo": {
    "name": "my-graph",
    "created_at": "2026-03-10T14:30:00Z",
    "modified_at": "2026-03-10T14:30:00Z",
    "vertex_count": 0,
    "atom_count": 0
  }
}
```

**Error Response:**
```json
HTTP/1.1 404 Not Found
Content-Type: application/json

{
  "error": "Workspace not found: nonexistent",
  "code": "Workspace(NotFound)",
  "details": null
}
```

### GraphQL (Optional)

When built with `--features graphql`, the server exposes a GraphQL endpoint.

**Query example:**
```graphql
{
  workspaces {
    name
    created_at
    vertex_count
  }
  atoms(workspace: "my-graph") {
    char
    index
  }
  readAsText(workspace: "my-graph", index: 5)
}
```

**Design principle:** GraphQL provides **read-only queries**. All mutations (write operations) go through `POST /api/execute`. This keeps the `Command` enum as the single source of truth for state changes and avoids duplicating validation logic.

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| `WorkspaceManager` blocking ops stall async runtime | Medium | Medium | All manager calls wrapped in `spawn_blocking` |
| `async-graphql` proc macros increase compile time | Medium | Low | GraphQL is feature-gated, only enabled when needed |
| GraphQL type mapping for `GraphSnapshot` / complex types | Medium | Medium | Use `async_graphql::Json<T>` wrapper for complex types that are hard to map to GraphQL natively |
| CORS too permissive for production | Low | Medium | `default_cors()` is for development; document how to restrict for production deployments |
| Concurrent requests to same workspace cause lock contention | Medium | Low | `WorkspaceManager`'s internal locking handles this correctly. Document that write-heavy workloads may see serialization. |
| `axum::test` cannot share state across requests easily | Low | Low | Use `Router::clone()` or create a shared harness |
| Port conflict with log-viewer (3000) or other tools | Low | Low | Default port 3100 avoids conflict; configurable via `PORT` env var |

## Notes

### Relationship to viewer-api

This crate **reuses** `viewer-api` for shared infrastructure (CORS, tracing, static files) but does NOT use `viewer-api::run_server()` directly. The reason: `run_server()` takes a `create_router(state, static_dir) -> Router` function signature that expects `ServerArgs` (--http/--mcp flags). `context-http` is an HTTP-only server — the MCP adapter is a separate binary (`context-mcp`). Therefore, `context-http` uses `viewer-api` utilities (CORS layer, tracing init) but builds its own `main()`.

If in the future we want a single `context-server` binary that does both HTTP + MCP, we can use `viewer-api::run_server()` at that point.

### Future Enhancements

- **WebSocket subscriptions** — Real-time graph change notifications via WebSocket or SSE. Could integrate with `async-graphql` subscriptions.
- **Authentication** — Add API key or JWT auth middleware for production deployments.
- **Rate limiting** — Add `tower::limit` middleware for public-facing deployments.
- **Pagination** — Add `limit`/`offset` parameters to list endpoints and GraphQL queries.
- **OpenAPI spec** — Generate OpenAPI/Swagger documentation from the `Command`/`CommandResult` types using `utoipa` or similar.
- **Static frontend** — Serve a web dashboard from `static_dir` for visual graph exploration (could reuse log-viewer patterns).
- **Batch endpoint** — `POST /api/batch` accepting an array of Commands for transactional multi-step operations.

### Deviations from Plan
*(To be filled during execution)*

### Lessons Learned
*(To be filled after execution)*