---
description: "Implement ticket fc18c607: spec-http — HTTP endpoints for spec-api"
---

# Ticket fc18c607 — spec-http: HTTP Endpoints for spec-api

## Goal

Create a standalone `spec-http` crate that exposes spec-api operations over a REST API using Axum, following the same patterns as `ticket-http`.

## Ticket State Management

```bash
# At start:
./target/debug/ticket.exe update fc18c607 --to-state in-implementation
./target/debug/ticket.exe board check-in fc18c607 --agent-id copilot \
  --intent "implementing spec-http" \
  --files "tools/http/spec-http/Cargo.toml,tools/http/spec-http/src/main.rs,tools/http/spec-http/src/lib.rs,tools/http/spec-http/src/routes.rs,tools/http/spec-http/src/handlers.rs,tools/http/spec-http/src/error.rs,tools/http/spec-http/src/state.rs" \
  --ttl 3600

# At end (after tests pass):
./target/debug/ticket.exe update fc18c607 --to-state in-review
```

## Reference Implementation

**ticket-http** at `tools/http/ticket-http/` is the reference pattern. Key patterns:

```
tools/http/ticket-http/
├── Cargo.toml
└── src/
    ├── lib.rs              ← pub mod serve; + pub async fn start_server()
    ├── main.rs             ← CLI entry: parse args, open store, start server
    └── serve/
        ├── mod.rs          ← AppState, ServeConfig, serve()
        ├── error.rs        ← storage_err() mapping
        ├── middleware.rs   ← write_auth
        ├── registry.rs     ← WorkspaceRegistry (lazy store cache)
        ├── routes.rs       ← build_router() with read/write route groups
        └── handlers/       ← one file per domain (tickets.rs, edges.rs, etc.)
```

**Handler pattern:**
```rust
pub async fn list_tickets(
    State(state): State<AppState>,
    Extension(rid): Extension<RequestIdExt>,
    Query(params): Query<WorkspaceParam>,
) -> Response {
    let store = match state.ensure_workspace_runtime(&params.workspace) {
        Some(s) => s,
        None => return ApiError::not_found("workspace", &rid.0)
            .into_response_with_status(StatusCode::NOT_FOUND),
    };
    match store.list(...) {
        Ok(items) => Json(response).into_response(),
        Err(e) => storage_err(e, &rid.0),
    }
}
```

**Response pattern:**
- All responses include `request_id` field for error correlation
- `201 Created` for POST create operations
- `200 OK` for GET and PATCH operations
- Errors via `viewer_api::error::ApiError` with explicit HTTP status mapping

## Architecture

Since SpecStore needs `&mut self` for writes (unlike TicketStore which is `Arc<TicketStore>`), the architecture is simpler — no WorkspaceRegistry, just a single store behind a Mutex.

```
tools/http/spec-http/
├── Cargo.toml
└── src/
    ├── lib.rs              ← pub mod routes, handlers, error, state;
    ├── main.rs             ← CLI entry: parse args, open store, start server
    ├── state.rs            ← SpecAppState with Arc<Mutex<SpecStore>>
    ├── error.rs            ← spec_err() mapping
    ├── routes.rs           ← build_router() with read/write route groups
    └── handlers/
        ├── mod.rs          ← pub mod specs, sections, tree, health;
        ├── specs.rs        ← CRUD + search + list handlers
        ├── sections.rs     ← section CRUD handlers
        ├── tree.rs         ← tree + refs handlers
        └── health.rs       ← health check handler
```

## Endpoints

All endpoints live under `/api/specs`. No workspace parameter — the server binds to a single spec store root.

### Read Endpoints (public)

```
GET    /healthz                         → healthz
GET    /api/specs                       → list_specs (?state=&component=&query=&limit=)
GET    /api/specs/search                → search_specs (?q=&limit=)
GET    /api/specs/health                → health_check (?id=&all=)
GET    /api/specs/:id                   → get_spec
GET    /api/specs/:id/full              → get_spec_full (with body + sections)
GET    /api/specs/:id/tree              → get_tree (hierarchy subtree)
GET    /api/specs/:id/refs              → get_refs (code references)
GET    /api/specs/:id/sections          → list_sections
GET    /api/specs/:id/sections/:name    → get_section
```

### Write Endpoints (auth-gated)

```
POST   /api/specs                       → create_spec
PATCH  /api/specs/:id                   → update_spec
DELETE /api/specs/:id                   → delete_spec
POST   /api/specs/:id/refs/validate     → validate_refs
POST   /api/specs/:id/sections          → add_section
DELETE /api/specs/:id/sections/:name    → delete_section
POST   /api/specs/scan                  → scan (?force=)
POST   /api/specs/add-root              → add_root
```

### Deferred (stub 501)

```
GET    /api/specs/toc                   → table of contents (depends on a7b2a89c)
POST   /api/specs/skill/generate        → skill generation (depends on eddf5d2e)
```

## Implementation

### Step 1: Create `tools/http/spec-http/Cargo.toml`

```toml
[package]
name = "spec-http"
version = "0.1.0"
edition = "2024"
description = "HTTP REST API for spec-api"

[[bin]]
name = "spec-http"
path = "src/main.rs"

[lib]
name = "spec_http"
path = "src/lib.rs"

[dependencies]
spec-api = { path = "../../crates/spec-api" }
memory-api = { path = "../../crates/memory-api" }
viewer-api = { path = "../viewer/viewer-api" }

axum = { version = "0.8", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["serde", "v4"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tower-http = { version = "0.6", features = ["cors"] }

[dev-dependencies]
tempfile = "3"
tower = { version = "0.5", features = ["util"] }
```

Add `"tools/http/spec-http"` to the workspace `Cargo.toml` members list.

### Step 2: Create `src/lib.rs`

```rust
pub mod error;
pub mod handlers;
pub mod routes;
pub mod state;

use std::path::PathBuf;

pub use state::SpecAppState;
pub use routes::build_router;

/// Configuration for the HTTP server.
#[derive(Debug, Clone)]
pub struct ServeConfig {
    pub host: String,
    pub port: u16,
}

impl ServeConfig {
    pub fn addr(&self) -> std::net::SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("valid address")
    }
}

/// Start the spec-http server.
pub async fn start_server(
    config: ServeConfig,
    state: SpecAppState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Auto-scan on startup so slugs are available
    {
        let mut store = state.store.lock().await;
        let _ = store.scan(false);
    }

    let app = build_router(state);
    let addr = config.addr();
    let listener = tokio::net::TcpListener::bind(addr).await?;
    eprintln!("spec-http listening on http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
```

### Step 3: Create `src/main.rs`

```rust
use std::path::PathBuf;
use spec_api::SpecStore;
use spec_http::{ServeConfig, SpecAppState, start_server};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("spec_http=info".parse().unwrap()),
        )
        .with_writer(std::io::stderr)
        .init();

    let mut port: u16 = 4001;
    let mut host = "127.0.0.1".to_string();
    let mut index_root: Option<String> = None;

    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--port" => { i += 1; port = args[i].parse().expect("invalid port"); }
            "--host" => { i += 1; host = args[i].clone(); }
            "--index-root" => { i += 1; index_root = Some(args[i].clone()); }
            _ => {}
        }
        i += 1;
    }

    let root = index_root
        .map(PathBuf::from)
        .or_else(|| std::env::var("SPEC_INDEX_ROOT").ok().map(PathBuf::from))
        .or_else(|| std::env::var("TICKET_INDEX_ROOT").ok().map(PathBuf::from))
        .unwrap_or_else(|| {
            let cwd = std::env::current_dir().expect("cwd");
            let spec_dir = cwd.join(".spec");
            if spec_dir.exists() { spec_dir } else { cwd.join(".ticket") }
        });

    let store = SpecStore::open(&root).unwrap_or_else(|e| {
        eprintln!("Failed to open spec store at {}: {e}", root.display());
        std::process::exit(1);
    });

    let state = SpecAppState::new(store);
    let config = ServeConfig { host, port };

    if let Err(err) = start_server(config, state).await {
        eprintln!("Fatal error: {err}");
        std::process::exit(1);
    }
}
```

### Step 4: Create `src/state.rs`

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use spec_api::SpecStore;

/// Shared application state for spec-http handlers.
///
/// SpecStore needs `&mut self` for create/update/delete/scan,
/// so we wrap it in an async Mutex. The Mutex is held only for
/// the duration of each handler call.
#[derive(Clone)]
pub struct SpecAppState {
    pub store: Arc<Mutex<SpecStore>>,
}

impl SpecAppState {
    pub fn new(store: SpecStore) -> Self {
        Self {
            store: Arc::new(Mutex::new(store)),
        }
    }
}
```

### Step 5: Create `src/error.rs`

```rust
//! Error helpers for spec-http handlers.

use axum::http::StatusCode;
use axum::response::Response;
use viewer_api::error::ApiError;

pub use viewer_api::error::RequestIdExt;

/// Map a `SpecError` to an Axum `Response` with appropriate HTTP status.
pub fn spec_err(e: spec_api::error::SpecError, rid: &str) -> Response {
    use spec_api::error::SpecError;
    match e {
        SpecError::NotFound(_) => {
            ApiError::not_found("spec", rid)
                .into_response_with_status(StatusCode::NOT_FOUND)
        }
        SpecError::InvalidSlug(msg) => {
            ApiError::new("spec.invalid_slug", &msg, rid)
                .into_response_with_status(StatusCode::BAD_REQUEST)
        }
        SpecError::DuplicateSlug(slug) => {
            ApiError::new(
                "spec.duplicate_slug",
                &format!("slug already exists: {slug}"),
                rid,
            )
            .into_response_with_status(StatusCode::CONFLICT)
        }
        SpecError::Validation(e) => {
            ApiError::new("spec.validation", &e.to_string(), rid)
                .into_response_with_status(StatusCode::UNPROCESSABLE_ENTITY)
        }
        _ => {
            tracing::error!(error = %e, "spec error in http handler");
            ApiError::internal(rid)
                .into_response_with_status(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Map a `StorageError` to an Axum `Response`.
pub fn storage_err(e: memory_api::error::StorageError, rid: &str) -> Response {
    use memory_api::error::StorageError;
    match e {
        StorageError::NotFound(_) => {
            ApiError::not_found("spec", rid)
                .into_response_with_status(StatusCode::NOT_FOUND)
        }
        _ => {
            tracing::error!(error = %e, "storage error in spec-http handler");
            ApiError::internal(rid)
                .into_response_with_status(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
```

### Step 6: Create `src/routes.rs`

```rust
//! Route table for spec-http.

use axum::{
    Router,
    middleware,
    routing::{delete, get, patch, post},
};
use tower_http::cors::{CorsLayer, Any};
use viewer_api::middleware::request_id::add_request_id;

use crate::handlers;
use crate::state::SpecAppState;

/// Build the full Axum router.
pub fn build_router(state: SpecAppState) -> Router {
    let read_routes = Router::new()
        .route("/healthz", get(handlers::health::healthz))
        .route("/api/specs", get(handlers::specs::list_specs))
        .route("/api/specs/search", get(handlers::specs::search_specs))
        .route("/api/specs/health", get(handlers::health::health_check))
        .route("/api/specs/{id}", get(handlers::specs::get_spec))
        .route("/api/specs/{id}/full", get(handlers::specs::get_spec_full))
        .route("/api/specs/{id}/tree", get(handlers::tree::get_tree))
        .route("/api/specs/{id}/refs", get(handlers::tree::get_refs))
        .route("/api/specs/{id}/sections", get(handlers::sections::list_sections))
        .route("/api/specs/{id}/sections/{name}", get(handlers::sections::get_section));

    let write_routes = Router::new()
        .route("/api/specs", post(handlers::specs::create_spec))
        .route(
            "/api/specs/{id}",
            patch(handlers::specs::update_spec).delete(handlers::specs::delete_spec),
        )
        .route("/api/specs/{id}/refs/validate", post(handlers::tree::validate_refs))
        .route("/api/specs/{id}/sections", post(handlers::sections::add_section))
        .route("/api/specs/{id}/sections/{name}", delete(handlers::sections::delete_section))
        .route("/api/specs/scan", post(handlers::health::scan))
        .route("/api/specs/add-root", post(handlers::health::add_root));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    read_routes
        .merge(write_routes)
        .layer(cors)
        .layer(middleware::from_fn(add_request_id))
        .with_state(state)
}
```

**Note:** Auth middleware is NOT included initially — can be added later if needed. CORS is enabled per the acceptance criteria.

### Step 7: Create `src/handlers/mod.rs`

```rust
pub mod health;
pub mod sections;
pub mod specs;
pub mod tree;
```

### Step 8: Create `src/handlers/specs.rs`

This is the main handler file. Follow ticket-http's `tickets.rs` pattern.

```rust
use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

use viewer_api::error::{ApiError, RequestIdExt};
use crate::error::spec_err;
use crate::state::SpecAppState;
use spec_api::SpecManifest;

// ── Query/Path extractors ────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ListParams {
    pub state: Option<String>,
    pub component: Option<String>,
    pub query: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
    pub limit: Option<usize>,
}

// ── Response types ───────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct SpecSummary {
    pub id: String,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub state: Option<String>,
    pub component: Option<String>,
}

#[derive(Serialize)]
pub struct SpecListResponse {
    pub request_id: String,
    pub count: usize,
    pub items: Vec<SpecSummary>,
}

#[derive(Serialize)]
pub struct SpecDetailResponse {
    pub request_id: String,
    pub spec: SpecDetail,
}

#[derive(Serialize)]
pub struct SpecDetail {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub fields: BTreeMap<String, Value>,
    pub code_refs: Vec<spec_api::code_ref::CodeRef>,
}

#[derive(Serialize)]
pub struct SpecFullResponse {
    pub request_id: String,
    pub spec: SpecDetail,
    pub body: String,
    pub sections: Vec<String>,
}

// ── Create request ───────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateSpecRequest {
    pub title: String,
    pub slug: String,
    pub component: String,
    pub parent: Option<String>,
    pub scope: Option<String>,
    pub body: Option<String>,
}

#[derive(Serialize)]
pub struct CreateSpecResponse {
    pub request_id: String,
    pub id: String,
    pub slug: String,
}

// ── Update request ───────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct UpdateSpecRequest {
    #[serde(default)]
    pub fields: BTreeMap<String, Value>,
    pub to_state: Option<String>,
    pub body: Option<String>,
}

// ── Handlers ─────────────────────────────────────────────────────────────

fn spec_to_summary(spec: &SpecManifest) -> SpecSummary {
    SpecSummary {
        id: spec.id.to_string(),
        slug: spec.slug().map(str::to_string),
        title: spec.title().map(str::to_string),
        state: spec.state().map(str::to_string),
        component: spec.component().map(str::to_string),
    }
}

fn spec_to_detail(spec: &SpecManifest) -> SpecDetail {
    SpecDetail {
        id: spec.id.to_string(),
        created_at: spec.created_at,
        fields: spec.extra.clone(),
        code_refs: spec.code_refs.clone(),
    }
}

pub async fn list_specs(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Query(params): Query<ListParams>,
) -> Response {
    let mut store = state.store.lock().await;
    let _ = store.scan(false); // ensure slug index is fresh

    // List all indexed specs, then filter
    let all = match store.entity_store().list_indexed(false) {
        Ok(a) => a,
        Err(e) => return crate::error::storage_err(e, &rid.0),
    };

    let mut items = Vec::new();
    for indexed in &all {
        let spec = match store.get(&indexed.id.to_string()) {
            Ok(s) => s,
            Err(_) => continue,
        };
        if let Some(ref st) = params.state {
            if spec.state().map(str::to_string).as_deref() != Some(st.as_str()) {
                continue;
            }
        }
        if let Some(ref comp) = params.component {
            if spec.component().map(str::to_string).as_deref() != Some(comp.as_str()) {
                continue;
            }
        }
        items.push(spec_to_summary(&spec));
        if let Some(limit) = params.limit {
            if items.len() >= limit { break; }
        }
    }

    Json(SpecListResponse {
        request_id: rid.0,
        count: items.len(),
        items,
    })
    .into_response()
}

pub async fn search_specs(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Query(params): Query<SearchParams>,
) -> Response {
    let store = state.store.lock().await;
    let limit = params.limit.unwrap_or(20).min(100);
    match store.entity_store().search(&params.q, limit) {
        Ok(results) => {
            let items: Vec<SpecSummary> = results.iter().map(|r| SpecSummary {
                id: r.id.to_string(),
                slug: None,
                title: r.title.clone(),
                state: r.state.clone(),
                component: None,
            }).collect();
            Json(SpecListResponse {
                request_id: rid.0,
                count: items.len(),
                items,
            }).into_response()
        }
        Err(e) => crate::error::storage_err(e, &rid.0),
    }
}

/// GET /api/specs/:id — accepts UUID, UUID prefix, or slug.
pub async fn get_spec(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Path(id): Path<String>,
) -> Response {
    let mut store = state.store.lock().await;
    let _ = store.scan(false);
    match store.get(&id) {
        Ok(spec) => Json(SpecDetailResponse {
            request_id: rid.0,
            spec: spec_to_detail(&spec),
        }).into_response(),
        Err(e) => spec_err(e, &rid.0),
    }
}

/// GET /api/specs/:id/full — includes body and sections list.
pub async fn get_spec_full(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Path(id): Path<String>,
) -> Response {
    let mut store = state.store.lock().await;
    let _ = store.scan(false);
    let (spec, body) = match store.get_full(&id) {
        Ok(r) => r,
        Err(e) => return spec_err(e, &rid.0),
    };
    let sections = match store.list_sections(&id) {
        Ok(s) => s,
        Err(e) => return spec_err(e, &rid.0),
    };
    Json(SpecFullResponse {
        request_id: rid.0,
        spec: spec_to_detail(&spec),
        body,
        sections,
    }).into_response()
}

/// POST /api/specs — create a new spec.
pub async fn create_spec(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Json(req): Json<CreateSpecRequest>,
) -> Response {
    let mut store = state.store.lock().await;
    let _ = store.scan(false);

    let mut manifest = SpecManifest::new(&req.slug, &req.title, &req.component);
    if let Some(parent) = &req.parent {
        match store.resolve_id(parent) {
            Ok(pid) => manifest.set_parent(&pid.to_string()),
            Err(e) => return spec_err(e, &rid.0),
        }
    }
    if let Some(scope) = &req.scope {
        manifest.set_scope(scope);
    }
    let body = req.body.as_deref().unwrap_or("");

    match store.create(&manifest, body, None) {
        Ok(id) => (
            StatusCode::CREATED,
            Json(CreateSpecResponse {
                request_id: rid.0,
                id: id.to_string(),
                slug: req.slug,
            }),
        ).into_response(),
        Err(e) => spec_err(e, &rid.0),
    }
}

/// PATCH /api/specs/:id — update fields, state, and/or body.
pub async fn update_spec(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Path(id): Path<String>,
    Json(req): Json<UpdateSpecRequest>,
) -> Response {
    let mut store = state.store.lock().await;
    let _ = store.scan(false);

    if let Some(body) = &req.body {
        if let Err(e) = store.update_body(&id, body) {
            return spec_err(e, &rid.0);
        }
    }

    match store.update(&id, req.fields, req.to_state.as_deref()) {
        Ok(spec) => Json(SpecDetailResponse {
            request_id: rid.0,
            spec: spec_to_detail(&spec),
        }).into_response(),
        Err(e) => spec_err(e, &rid.0),
    }
}

/// DELETE /api/specs/:id — soft-delete.
pub async fn delete_spec(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Path(id): Path<String>,
) -> Response {
    let mut store = state.store.lock().await;
    let _ = store.scan(false);
    match store.delete(&id) {
        Ok(()) => Json(serde_json::json!({
            "request_id": rid.0,
            "status": "ok",
        })).into_response(),
        Err(e) => spec_err(e, &rid.0),
    }
}
```

### Step 9: Create `src/handlers/sections.rs`

```rust
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};

use viewer_api::error::RequestIdExt;
use crate::error::spec_err;
use crate::state::SpecAppState;

#[derive(Deserialize)]
pub struct AddSectionRequest {
    pub name: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct SectionsResponse {
    pub request_id: String,
    pub spec: String,
    pub count: usize,
    pub sections: Vec<String>,
}

/// GET /api/specs/:id/sections
pub async fn list_sections(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Path(id): Path<String>,
) -> Response {
    let mut store = state.store.lock().await;
    let _ = store.scan(false);
    match store.list_sections(&id) {
        Ok(sections) => Json(SectionsResponse {
            request_id: rid.0,
            spec: id,
            count: sections.len(),
            sections,
        }).into_response(),
        Err(e) => spec_err(e, &rid.0),
    }
}

/// GET /api/specs/:id/sections/:name
pub async fn get_section(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Path((id, name)): Path<(String, String)>,
) -> Response {
    let mut store = state.store.lock().await;
    let _ = store.scan(false);

    // Resolve spec to find its folder, then read the section file
    let uuid = match store.resolve_id(&id) {
        Ok(u) => u,
        Err(e) => return spec_err(e, &rid.0),
    };
    let indexed = match store.entity_store().get_indexed(&uuid) {
        Ok(Some(i)) => i,
        Ok(None) => {
            return viewer_api::error::ApiError::not_found("spec", &rid.0)
                .into_response_with_status(StatusCode::NOT_FOUND);
        }
        Err(e) => return crate::error::storage_err(e, &rid.0),
    };
    let file_name = if name.ends_with(".md") { name.clone() } else { format!("{name}.md") };
    let path = indexed.path.join("sections").join(&file_name);
    match std::fs::read_to_string(&path) {
        Ok(content) => Json(serde_json::json!({
            "request_id": rid.0,
            "spec": id,
            "section": name,
            "content": content,
        })).into_response(),
        Err(_) => {
            viewer_api::error::ApiError::not_found("section", &rid.0)
                .into_response_with_status(StatusCode::NOT_FOUND)
        }
    }
}

/// POST /api/specs/:id/sections
pub async fn add_section(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Path(id): Path<String>,
    Json(req): Json<AddSectionRequest>,
) -> Response {
    let mut store = state.store.lock().await;
    let _ = store.scan(false);
    match store.add_section(&id, &req.name, &req.content) {
        Ok(()) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "request_id": rid.0,
                "spec": id,
                "section": req.name,
                "status": "ok",
            })),
        ).into_response(),
        Err(e) => spec_err(e, &rid.0),
    }
}

/// DELETE /api/specs/:id/sections/:name
pub async fn delete_section(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Path((id, name)): Path<(String, String)>,
) -> Response {
    let mut store = state.store.lock().await;
    let _ = store.scan(false);
    match store.delete_section(&id, &name) {
        Ok(()) => Json(serde_json::json!({
            "request_id": rid.0,
            "spec": id,
            "section": name,
            "status": "ok",
        })).into_response(),
        Err(e) => spec_err(e, &rid.0),
    }
}
```

### Step 10: Create `src/handlers/tree.rs`

```rust
use axum::{
    extract::{Extension, Path, State},
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use std::path::PathBuf;

use viewer_api::error::RequestIdExt;
use crate::error::spec_err;
use crate::state::SpecAppState;

#[derive(Serialize)]
pub struct TreeNodeSummary {
    pub id: String,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub state: Option<String>,
    pub parent: Option<String>,
}

/// GET /api/specs/:id/tree — hierarchy subtree.
pub async fn get_tree(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Path(id): Path<String>,
) -> Response {
    let mut store = state.store.lock().await;
    let _ = store.scan(false);

    let root = match store.get(&id) {
        Ok(s) => s,
        Err(e) => return spec_err(e, &rid.0),
    };
    let descendants = match store.subtree(&id) {
        Ok(d) => d,
        Err(e) => return spec_err(e, &rid.0),
    };

    Json(serde_json::json!({
        "request_id": rid.0,
        "root": {
            "id": root.id.to_string(),
            "slug": root.slug(),
            "title": root.title(),
            "state": root.state(),
        },
        "descendants": descendants.iter().map(|c| serde_json::json!({
            "id": c.id.to_string(),
            "slug": c.slug(),
            "title": c.title(),
            "state": c.state(),
            "parent": c.parent(),
        })).collect::<Vec<_>>(),
    })).into_response()
}

/// GET /api/specs/:id/refs — list code references.
pub async fn get_refs(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Path(id): Path<String>,
) -> Response {
    let mut store = state.store.lock().await;
    let _ = store.scan(false);
    match store.get(&id) {
        Ok(spec) => Json(serde_json::json!({
            "request_id": rid.0,
            "id": spec.id.to_string(),
            "count": spec.code_refs.len(),
            "refs": spec.code_refs,
        })).into_response(),
        Err(e) => spec_err(e, &rid.0),
    }
}

/// POST /api/specs/:id/refs/validate — validate code references.
pub async fn validate_refs(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Path(id): Path<String>,
    axum::Json(body): axum::Json<ValidateRefsRequest>,
) -> Response {
    let mut store = state.store.lock().await;
    let _ = store.scan(false);
    let spec = match store.get(&id) {
        Ok(s) => s,
        Err(e) => return spec_err(e, &rid.0),
    };
    let workspace_root = PathBuf::from(&body.workspace_root);
    let results = spec_api::code_ref::validate_refs(&spec.code_refs, &workspace_root);
    let all_valid = results.iter().all(|r| r.file_exists && r.line_range_valid);
    let items: Vec<serde_json::Value> = results.iter().map(|r| serde_json::json!({
        "file": r.code_ref.file,
        "symbol": r.code_ref.symbol,
        "kind": format!("{:?}", r.code_ref.kind),
        "file_exists": r.file_exists,
        "line_range_valid": r.line_range_valid,
        "message": r.message,
    })).collect();

    Json(serde_json::json!({
        "request_id": rid.0,
        "id": spec.id.to_string(),
        "valid": all_valid,
        "count": items.len(),
        "results": items,
    })).into_response()
}

#[derive(serde::Deserialize)]
pub struct ValidateRefsRequest {
    #[serde(default = "default_workspace_root")]
    pub workspace_root: String,
}

fn default_workspace_root() -> String { ".".to_string() }
```

### Step 11: Create `src/handlers/health.rs`

```rust
use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Deserialize;
use std::path::PathBuf;

use viewer_api::error::{ApiError, RequestIdExt};
use crate::error::{spec_err, storage_err};
use crate::state::SpecAppState;

/// GET /healthz
pub async fn healthz() -> &'static str {
    "ok"
}

#[derive(Deserialize)]
pub struct HealthParams {
    pub id: Option<String>,
    #[serde(default)]
    pub all: bool,
}

/// GET /api/specs/health
pub async fn health_check(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Query(params): Query<HealthParams>,
) -> Response {
    let mut store = state.store.lock().await;
    let _ = store.scan(false);

    let specs = if params.all {
        match store.entity_store().list_indexed(false) {
            Ok(all) => all
                .iter()
                .filter_map(|e| store.get(&e.id.to_string()).ok())
                .collect::<Vec<_>>(),
            Err(e) => return storage_err(e, &rid.0),
        }
    } else if let Some(id) = &params.id {
        match store.get(id) {
            Ok(s) => vec![s],
            Err(e) => return spec_err(e, &rid.0),
        }
    } else {
        return ApiError::new("spec.missing_param", "provide id or all=true", &rid.0)
            .into_response_with_status(StatusCode::BAD_REQUEST);
    };

    let mut issues = Vec::new();
    for spec in &specs {
        if spec.slug().is_none() {
            issues.push(serde_json::json!({"id": spec.id.to_string(), "issue": "missing slug"}));
        }
        if spec.title().is_none() {
            issues.push(serde_json::json!({"id": spec.id.to_string(), "issue": "missing title"}));
        }
        if spec.component().is_none() {
            issues.push(serde_json::json!({"id": spec.id.to_string(), "issue": "missing component"}));
        }
    }

    Json(serde_json::json!({
        "request_id": rid.0,
        "specs_checked": specs.len(),
        "issues_count": issues.len(),
        "issues": issues,
    })).into_response()
}

#[derive(Deserialize)]
pub struct ScanParams {
    #[serde(default)]
    pub force: bool,
}

/// POST /api/specs/scan
pub async fn scan(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Query(params): Query<ScanParams>,
) -> Response {
    let mut store = state.store.lock().await;
    match store.scan(params.force) {
        Ok(report) => Json(serde_json::json!({
            "request_id": rid.0,
            "status": "ok",
            "force": params.force,
            "integrated": report.integrated,
            "pruned": report.pruned,
            "diagnostics_count": report.diagnostics.len(),
        })).into_response(),
        Err(e) => spec_err(e, &rid.0),
    }
}

#[derive(Deserialize)]
pub struct AddRootRequest {
    pub path: String,
    pub label: Option<String>,
}

/// POST /api/specs/add-root
pub async fn add_root(
    State(state): State<SpecAppState>,
    Extension(rid): Extension<RequestIdExt>,
    Json(req): Json<AddRootRequest>,
) -> Response {
    let mut store = state.store.lock().await;
    let path = PathBuf::from(&req.path);
    let label = req.label.unwrap_or_else(|| {
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("specs")
            .to_string()
    });
    match store.entity_store().add_scan_root(memory_api::model::filesystem::ScanRoot {
        path: path.clone(),
        label: label.clone(),
    }) {
        Ok(()) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "request_id": rid.0,
                "status": "ok",
                "path": path,
                "label": label,
            })),
        ).into_response(),
        Err(e) => storage_err(e, &rid.0),
    }
}
```

## Key Design Decisions

1. **Separate crate** (`spec-http`) rather than extending ticket-http. SpecStore has different lifecycle requirements (`&mut self` for writes) and the API surfaces are logically independent.

2. **`Arc<Mutex<SpecStore>>`** for state — unlike ticket-http's `Arc<TicketStore>`, SpecStore needs mutable access for create/update/delete/scan. The Mutex ensures serialized access.

3. **No WorkspaceRegistry** — spec-http serves a single store root, not multi-workspace. Keeps it simple.

4. **Auto-scan on reads** — `store.scan(false)` is called in each handler to ensure the slug index is fresh. This mirrors the spec-mcp pattern.

5. **CORS enabled** — `tower-http::cors::CorsLayer` with permissive defaults per acceptance criteria.

6. **Slug resolution in `:id`** — All `:id` path parameters accept UUID, UUID prefix, or slug. Handled transparently by `store.get()` / `store.resolve_id()`.

7. **`viewer-api` integration** — Uses `ApiError`, `RequestIdExt`, and `add_request_id` middleware for consistent error format and request tracing.

8. **Default port 4001** — Avoids conflict with ticket-http's default port 4000.

## Validation

```bash
cargo build -p spec-http
cargo test -p spec-http

# Smoke test:
SPEC_INDEX_ROOT=/tmp/test-spec-http ./target/debug/spec-http --port 4001

# In another terminal:
curl http://localhost:4001/healthz
curl http://localhost:4001/api/specs
curl -X POST http://localhost:4001/api/specs \
  -H "Content-Type: application/json" \
  -d '{"title":"Test","slug":"test/spec","component":"test"}'
```

## Key Constraints

1. **Follow ticket-http patterns exactly** for route organization, response format, error mapping.
2. **Every response includes `request_id`** — use `Extension(rid): Extension<RequestIdExt>` in all handlers.
3. **Use `viewer_api::error::ApiError`** for error responses — not custom error types.
4. **`:id` accepts UUID, prefix, or slug** — pass through to spec-api which handles resolution.
5. **201 for creates, 200 for reads/updates** — match ticket-http status code conventions.
6. **CORS must be enabled** — browser access is an acceptance criterion.
