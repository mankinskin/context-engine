---
description: "Implement ticket fbb5a87d: spec-mcp — MCP tool surface for spec-api"
---

# Ticket fbb5a87d — spec-mcp: MCP Tool Surface

## Goal

Create an MCP server exposing spec-api operations as MCP tools, following the exact same pattern as `ticket-mcp`. Provides create, get, update, delete, list, search, tree, health, refs, and section tools.

## Ticket State Management

```bash
# At start:
./target/debug/ticket.exe update fbb5a87d --to-state in-implementation
./target/debug/ticket.exe board check-in fbb5a87d --agent-id copilot \
  --intent "implementing spec-mcp" \
  --files "tools/mcp/spec-mcp/Cargo.toml,tools/mcp/spec-mcp/src/main.rs,tools/mcp/spec-mcp/src/lib.rs,tools/mcp/spec-mcp/src/server.rs" \
  --ttl 3600

# At end (after tests pass):
./target/debug/ticket.exe update fbb5a87d --to-state in-review
```

## Reference Implementation

**ticket-mcp** at `tools/mcp/ticket-mcp/` is the exact pattern to follow. The architecture is:

```
tools/mcp/ticket-mcp/
├── Cargo.toml
├── src/
│   ├── lib.rs          ← pub mod server;
│   ├── main.rs         ← CLI entry: resolve root, init tracing, run_mcp_server()
│   └── server.rs       ← TicketServer struct, all tool defs, MCP server setup
```

**Key patterns:**
- `#[tool_router]` + `#[tool]` macros from `rmcp` for tool registration
- `#[tool_handler]` impl for MCP server handler trait
- `with_store` / `with_store_ext` for store lifecycle (open under lock, run, drop)
- `Parameters(input)` for tool input extraction
- `Self::json_result(&value)` for serialized responses
- `McpError::invalid_params(...)` for client errors, `McpError::internal_error(...)` for server errors
- All input structs: `#[derive(Debug, Deserialize, JsonSchema)]`

## Architecture

```
tools/mcp/spec-mcp/
├── Cargo.toml
└── src/
    ├── lib.rs          ← pub mod server;
    ├── main.rs         ← entry point: resolve index root, init tracing, run_mcp_server()
    └── server.rs       ← SpecServer struct, all tool defs, MCP handler
```

## SpecStore API Reference

```rust
// Constructor
SpecStore::open(index_root: &Path) -> Result<Self, SpecError>

// Scan (&mut self)
store.scan(reindex: bool) -> Result<ScanReport, SpecError>

// Resolution (&self)
store.resolve_id(id_or_slug: &str) -> Result<Uuid, SpecError>

// CRUD
store.create(&SpecManifest, body: &str, target_root: Option<&Path>) -> Result<SpecId, SpecError>  // &mut self
store.get(id_or_slug: &str) -> Result<SpecManifest, SpecError>  // &self
store.get_full(id_or_slug: &str) -> Result<(SpecManifest, String), SpecError>  // &self
store.update(id_or_slug: &str, patch: BTreeMap<String, Value>, to_state: Option<&str>) -> Result<SpecManifest, SpecError>  // &mut self
store.update_body(id_or_slug: &str, content: &str) -> Result<(), SpecError>  // &self
store.delete(id_or_slug: &str) -> Result<(), SpecError>  // &mut self

// Sections (&self)
store.add_section(id_or_slug, name, content) -> Result<(), SpecError>
store.update_section(id_or_slug, name, content) -> Result<(), SpecError>
store.delete_section(id_or_slug, name) -> Result<(), SpecError>
store.list_sections(id_or_slug) -> Result<Vec<String>, SpecError>

// Hierarchy (&self)
store.children(id_or_slug) -> Result<Vec<SpecManifest>, SpecError>
store.ancestors(id_or_slug) -> Result<Vec<SpecManifest>, SpecError>
store.subtree(id_or_slug) -> Result<Vec<SpecManifest>, SpecError>

// Search (via entity_store)
store.entity_store().search(query_expr, limit) -> Result<Vec<SearchResult>, StorageError>
store.entity_store().list_indexed(include_deleted: bool) -> Result<Vec<IndexedEntity>, StorageError>
store.entity_store().add_scan_root(ScanRoot) -> Result<(), StorageError>
```

**IMPORTANT:** `create`, `update`, `delete`, `scan` take `&mut self` because they modify the slug index. The `with_store` helper must open a mutable store.

### SpecManifest API

```rust
SpecManifest::new(slug, title, component) -> Self   // constructor
spec.id -> Uuid
spec.created_at -> DateTime<Utc>
spec.code_refs -> Vec<CodeRef>
spec.extra -> BTreeMap<String, Value>

// Accessors (all return Option<&str>):
spec.slug(), spec.title(), spec.state(), spec.component(), spec.scope(), spec.parent()

// Setters:
spec.set_slug(&str), set_title, set_state, set_component, set_scope, set_parent
```

### Code Reference Validation

```rust
use spec_api::code_ref::validate_refs;
validate_refs(code_refs: &[CodeRef], workspace_root: &Path) -> Vec<RefValidation>
// RefValidation { code_ref, file_exists, line_range_valid, message: Option<String> }
```

### SearchResult Fields

```rust
pub struct SearchResult {
    pub id: Uuid,
    pub title: Option<String>,
    pub state: Option<String>,
    pub ticket_type: Option<String>,  // will be "specification"
    pub snippet: Option<String>,
    pub score: f32,
}
```

## Implementation

### Step 1: Create `tools/mcp/spec-mcp/Cargo.toml`

```toml
[package]
name = "spec-mcp"
version = "0.1.0"
edition = "2024"
description = "MCP server for spec-api with direct store access"

[[bin]]
name = "spec-mcp"
path = "src/main.rs"

[lib]
name = "spec_mcp"
path = "src/lib.rs"

[dependencies]
spec-api = { path = "../../../crates/spec-api" }
memory-api = { path = "../../../crates/memory-api" }

rmcp = { version = "0.14", features = ["server", "transport-io"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["serde", "v4"] }
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[dev-dependencies]
tempfile = "3"
```

Add `"tools/mcp/spec-mcp"` to the workspace `Cargo.toml` members list.

### Step 2: Create `src/lib.rs`

```rust
pub mod server;
```

### Step 3: Create `src/main.rs`

Follow ticket-mcp's main.rs:

```rust
use spec_mcp::server;
use std::path::PathBuf;
use spec_api::SpecStore;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("spec_mcp=info".parse().unwrap()),
        )
        .with_writer(std::io::stderr)
        .init();

    let index_root = std::env::var("SPEC_INDEX_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            // Fallback: use TICKET_INDEX_ROOT or workspace resolution
            std::env::var("TICKET_INDEX_ROOT")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    let (path, _source) = ticket_api::workspace::resolve_workspace();
                    path
                })
        });

    // Validate store can open
    SpecStore::open(&index_root).unwrap_or_else(|e| {
        eprintln!("Failed to open spec store at {}: {e}", index_root.display());
        std::process::exit(1);
    });

    eprintln!(
        "spec-mcp starting (store: {})",
        index_root.display(),
    );

    if let Err(err) = server::run_mcp_server(index_root).await {
        eprintln!("Fatal error: {err}");
        std::process::exit(1);
    }
}
```

**Note on index root resolution:** The spec store may share the same index root as ticket-api, or use its own `SPEC_INDEX_ROOT` env var. Check whether `ticket_api::workspace::resolve_workspace()` is available as a dependency. If not, implement a simpler resolution chain: `SPEC_INDEX_ROOT` env → `TICKET_INDEX_ROOT` env → `.spec/` in CWD → `~/.spec-index/`. Avoid pulling in `ticket-api` as a dependency if it's not needed — only `spec-api` should be required.

### Step 4: Create `src/server.rs`

This is the main file. Follow the ticket-mcp server.rs pattern exactly.

#### 4a: Imports and Input Structs

```rust
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rmcp::handler::server::tool::ToolRouter;
use rmcp::model::{CallToolResult, Content, ServerCapabilities, ServerInfo};
use rmcp::schemars::JsonSchema;
use rmcp::tool;
use rmcp::{ServerHandler, tool_handler, tool_router};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::transport::stdio;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::sync::Mutex;

use spec_api::{SpecManifest, SpecStore};
use spec_api::error::SpecError;
use spec_api::code_ref::validate_refs;
use memory_api::model::filesystem::ScanRoot;

// ── Input types ──

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SpecRefInput {
    /// Spec UUID, prefix, or slug.
    pub id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateSpecInput {
    /// Spec title.
    pub title: String,
    /// Hierarchical slug.
    pub slug: String,
    /// Component this spec belongs to.
    pub component: String,
    /// Parent spec ID or slug.
    #[serde(default)]
    pub parent: Option<String>,
    /// Scope (e.g. "public", "internal").
    #[serde(default)]
    pub scope: Option<String>,
    /// Body content (markdown).
    #[serde(default)]
    pub body: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetSpecInput {
    /// Spec UUID, prefix, or slug.
    pub id: String,
    /// Include body and sections.
    #[serde(default)]
    pub full: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateSpecInput {
    /// Spec UUID, prefix, or slug.
    pub id: String,
    /// Field patches as key=value pairs.
    #[serde(default)]
    pub fields: Vec<String>,
    /// Optional state to transition to.
    #[serde(default)]
    pub to_state: Option<String>,
    /// Optional body content to replace.
    #[serde(default)]
    pub body: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListSpecsInput {
    /// Filter by field=value predicates.
    #[serde(default)]
    pub where_clauses: Vec<String>,
    /// Maximum results.
    #[serde(default)]
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchSpecsInput {
    /// Full-text search query.
    pub query: String,
    /// Maximum results.
    #[serde(default = "default_search_limit")]
    pub limit: usize,
}

fn default_search_limit() -> usize { 20 }

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TreeInput {
    /// Root spec ID or slug (omit for all roots).
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct HealthInput {
    /// Spec UUID, prefix, or slug (omit with all=true for all specs).
    #[serde(default)]
    pub id: Option<String>,
    /// Check all specs.
    #[serde(default)]
    pub all: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RefsValidateInput {
    /// Spec UUID, prefix, or slug.
    pub id: String,
    /// Workspace root for resolving file paths.
    #[serde(default = "default_workspace_root")]
    pub workspace_root: String,
}

fn default_workspace_root() -> String { ".".to_string() }

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SectionAddInput {
    /// Spec UUID, prefix, or slug.
    pub id: String,
    /// Section name.
    pub name: String,
    /// Section content (markdown).
    pub content: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SectionRefInput {
    /// Spec UUID, prefix, or slug.
    pub id: String,
    /// Section name.
    pub name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScanInput {
    /// Force full reindex.
    #[serde(default)]
    pub force: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddRootInput {
    /// Directory path to register as a scan root.
    pub path: String,
    /// Optional label for this root.
    #[serde(default)]
    pub label: Option<String>,
}
```

#### 4b: Server Struct and Store Lifecycle

```rust
use rmcp::McpError;

#[derive(Clone)]
pub struct SpecServer {
    index_root: PathBuf,
    tool_router: ToolRouter<Self>,
    store_lock: Arc<Mutex<()>>,
}

impl SpecServer {
    pub fn new(index_root: PathBuf) -> Self {
        Self {
            index_root,
            tool_router: Self::tool_router(),
            store_lock: Arc::new(Mutex::new(())),
        }
    }

    fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, McpError> {
        let text = serde_json::to_string_pretty(value)
            .map_err(|e| McpError::internal_error(format!("serialization: {e}"), None))?;
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    fn spec_err(e: SpecError) -> McpError {
        match &e {
            SpecError::NotFound(_) => McpError::invalid_params(e.to_string(), None),
            SpecError::InvalidSlug(_) => McpError::invalid_params(e.to_string(), None),
            SpecError::DuplicateSlug(_) => McpError::invalid_params(e.to_string(), None),
            _ => McpError::internal_error(format!("spec error: {e}"), None),
        }
    }

    fn storage_err(e: memory_api::error::StorageError) -> McpError {
        McpError::internal_error(format!("storage error: {e}"), None)
    }

    /// Open a mutable SpecStore under the serialization lock, run the
    /// closure, then drop both store and lock before returning.
    ///
    /// Handles `&mut SpecStore` since create/update/delete/scan mutate
    /// the slug index.
    async fn with_store<T>(
        &self,
        f: impl FnOnce(&mut SpecStore) -> Result<T, McpError>,
    ) -> Result<T, McpError> {
        let _guard = self.store_lock.lock().await;
        let mut store = SpecStore::open(&self.index_root).map_err(Self::spec_err)?;
        // Auto-scan to pick up new spec folders
        store.scan(false).map_err(Self::spec_err)?;
        let result = f(&mut store);
        drop(store);
        result
    }
}
```

**Key difference from ticket-mcp:** `with_store` provides `&mut SpecStore` since `create`, `update`, `delete` all need `&mut self`. The `scan(false)` auto-call ensures slug resolution works. Read-only methods (`get`, `list_sections`, `children`, etc.) work fine with `&mut` since Rust allows `&mut` → `&` coercion.

#### 4c: Tool Implementations

```rust
#[tool_router]
impl SpecServer {
    #[tool(name = "spec_create", description = "Create a new spec with title, slug, component, and optional body.")]
    async fn spec_create(
        &self,
        Parameters(input): Parameters<CreateSpecInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            let mut manifest = SpecManifest::new(&input.slug, &input.title, &input.component);
            if let Some(parent) = &input.parent {
                let parent_id = store.resolve_id(parent).map_err(Self::spec_err)?;
                manifest.set_parent(&parent_id.to_string());
            }
            if let Some(scope) = &input.scope {
                manifest.set_scope(scope);
            }
            let body = input.body.as_deref().unwrap_or("");
            let id = store.create(&manifest, body, None).map_err(Self::spec_err)?;
            Self::json_result(&json!({
                "status": "ok",
                "id": id,
                "slug": input.slug,
                "title": input.title,
                "component": input.component,
            }))
        }).await
    }

    #[tool(name = "spec_get", description = "Get a spec by ID or slug, optionally with body and sections.")]
    async fn spec_get(
        &self,
        Parameters(input): Parameters<GetSpecInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            if input.full {
                let (spec, body) = store.get_full(&input.id).map_err(Self::spec_err)?;
                let sections = store.list_sections(&input.id).map_err(Self::spec_err)?;
                Self::json_result(&json!({
                    "status": "ok",
                    "spec": {
                        "id": spec.id,
                        "created_at": spec.created_at,
                        "fields": spec.extra,
                        "code_refs": spec.code_refs,
                    },
                    "body": body,
                    "sections": sections,
                }))
            } else {
                let spec = store.get(&input.id).map_err(Self::spec_err)?;
                Self::json_result(&json!({
                    "status": "ok",
                    "spec": {
                        "id": spec.id,
                        "created_at": spec.created_at,
                        "fields": spec.extra,
                        "code_refs": spec.code_refs,
                    },
                }))
            }
        }).await
    }

    #[tool(name = "spec_update", description = "Update a spec's fields, state, or body.")]
    async fn spec_update(
        &self,
        Parameters(input): Parameters<UpdateSpecInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            let mut patch = BTreeMap::new();
            for raw in &input.fields {
                let (k, v) = raw.split_once('=').ok_or_else(|| {
                    McpError::invalid_params(
                        format!("invalid field format '{raw}', expected key=value"), None,
                    )
                })?;
                patch.insert(k.trim().to_string(), Value::String(v.trim().to_string()));
            }

            if let Some(body) = &input.body {
                store.update_body(&input.id, body).map_err(Self::spec_err)?;
            }

            let spec = store.update(&input.id, patch, input.to_state.as_deref())
                .map_err(Self::spec_err)?;
            Self::json_result(&json!({
                "status": "ok",
                "id": spec.id,
                "fields": spec.extra,
            }))
        }).await
    }

    #[tool(name = "spec_delete", description = "Soft-delete a spec.")]
    async fn spec_delete(
        &self,
        Parameters(input): Parameters<SpecRefInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            let id = store.resolve_id(&input.id).map_err(Self::spec_err)?;
            store.delete(&input.id).map_err(Self::spec_err)?;
            Self::json_result(&json!({
                "status": "ok",
                "id": id,
            }))
        }).await
    }

    #[tool(name = "spec_list", description = "List specs with optional field filters.")]
    async fn spec_list(
        &self,
        Parameters(input): Parameters<ListSpecsInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            let all = store.entity_store().list_indexed(false)
                .map_err(Self::storage_err)?;
            let mut items: Vec<Value> = Vec::new();
            'outer: for indexed in &all {
                let spec = match store.get(&indexed.id.to_string()) {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                for clause in &input.where_clauses {
                    if let Some((k, v)) = clause.split_once('=') {
                        let field_val = spec.extra.get(k).and_then(|fv| fv.as_str());
                        if field_val != Some(v) {
                            continue 'outer;
                        }
                    }
                }
                items.push(json!({
                    "id": indexed.id,
                    "slug": spec.slug(),
                    "title": spec.title(),
                    "state": spec.state(),
                    "component": spec.component(),
                }));
                if let Some(limit) = input.limit {
                    if items.len() >= limit { break; }
                }
            }
            Self::json_result(&json!({
                "status": "ok",
                "count": items.len(),
                "items": items,
            }))
        }).await
    }

    #[tool(name = "spec_search", description = "Full-text search across specs.")]
    async fn spec_search(
        &self,
        Parameters(input): Parameters<SearchSpecsInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            let results = store.entity_store().search(&input.query, input.limit)
                .map_err(Self::storage_err)?;
            let items: Vec<Value> = results.iter().map(|r| json!({
                "id": r.id,
                "title": r.title,
                "state": r.state,
                "type": r.ticket_type,
                "score": r.score,
                "snippet": r.snippet,
            })).collect();
            Self::json_result(&json!({
                "status": "ok",
                "query": input.query,
                "count": items.len(),
                "items": items,
            }))
        }).await
    }

    #[tool(name = "spec_tree", description = "Get hierarchy subtree for a spec, or list all root specs.")]
    async fn spec_tree(
        &self,
        Parameters(input): Parameters<TreeInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            if let Some(root_id) = &input.id {
                let root = store.get(root_id).map_err(Self::spec_err)?;
                let descendants = store.subtree(root_id).map_err(Self::spec_err)?;
                Self::json_result(&json!({
                    "status": "ok",
                    "root": {
                        "id": root.id,
                        "slug": root.slug(),
                        "title": root.title(),
                        "state": root.state(),
                    },
                    "descendants": descendants.iter().map(|c| json!({
                        "id": c.id,
                        "slug": c.slug(),
                        "title": c.title(),
                        "state": c.state(),
                        "parent": c.parent(),
                    })).collect::<Vec<_>>(),
                }))
            } else {
                let all = store.entity_store().list_indexed(false)
                    .map_err(Self::storage_err)?;
                let mut roots = Vec::new();
                for indexed in &all {
                    if let Ok(spec) = store.get(&indexed.id.to_string()) {
                        if spec.parent().is_none() {
                            let children = store.children(&indexed.id.to_string())
                                .map_err(Self::spec_err)?;
                            roots.push(json!({
                                "id": spec.id,
                                "slug": spec.slug(),
                                "title": spec.title(),
                                "children_count": children.len(),
                            }));
                        }
                    }
                }
                Self::json_result(&json!({
                    "status": "ok",
                    "roots": roots,
                }))
            }
        }).await
    }

    #[tool(name = "spec_health", description = "Run health checks on specs (completeness of required fields).")]
    async fn spec_health(
        &self,
        Parameters(input): Parameters<HealthInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            let specs = if input.all {
                let all = store.entity_store().list_indexed(false)
                    .map_err(Self::storage_err)?;
                all.iter()
                    .filter_map(|e| store.get(&e.id.to_string()).ok())
                    .collect::<Vec<_>>()
            } else if let Some(id) = &input.id {
                vec![store.get(id).map_err(Self::spec_err)?]
            } else {
                return Err(McpError::invalid_params(
                    "provide spec ID or set all=true", None,
                ));
            };

            let mut issues = Vec::new();
            for spec in &specs {
                if spec.slug().is_none() {
                    issues.push(json!({"id": spec.id, "issue": "missing slug"}));
                }
                if spec.title().is_none() {
                    issues.push(json!({"id": spec.id, "issue": "missing title"}));
                }
                if spec.component().is_none() {
                    issues.push(json!({"id": spec.id, "issue": "missing component"}));
                }
            }
            Self::json_result(&json!({
                "status": "ok",
                "specs_checked": specs.len(),
                "issues_count": issues.len(),
                "issues": issues,
            }))
        }).await
    }

    #[tool(name = "spec_refs_validate", description = "Validate code references for a spec (check file existence and line ranges).")]
    async fn spec_refs_validate(
        &self,
        Parameters(input): Parameters<RefsValidateInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            let spec = store.get(&input.id).map_err(Self::spec_err)?;
            let workspace_root = PathBuf::from(&input.workspace_root);
            let results = validate_refs(&spec.code_refs, &workspace_root);
            let items: Vec<Value> = results.iter().map(|r| json!({
                "file": r.code_ref.file,
                "symbol": r.code_ref.symbol,
                "kind": format!("{:?}", r.code_ref.kind),
                "file_exists": r.file_exists,
                "line_range_valid": r.line_range_valid,
                "message": r.message,
            })).collect();
            let all_valid = results.iter().all(|r| r.file_exists && r.line_range_valid);
            Self::json_result(&json!({
                "status": "ok",
                "id": spec.id,
                "valid": all_valid,
                "count": items.len(),
                "results": items,
            }))
        }).await
    }

    #[tool(name = "spec_section_add", description = "Add a section to a spec.")]
    async fn spec_section_add(
        &self,
        Parameters(input): Parameters<SectionAddInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            store.add_section(&input.id, &input.name, &input.content)
                .map_err(Self::spec_err)?;
            Self::json_result(&json!({
                "status": "ok",
                "spec": input.id,
                "section": input.name,
            }))
        }).await
    }

    #[tool(name = "spec_section_list", description = "List sections of a spec.")]
    async fn spec_section_list(
        &self,
        Parameters(input): Parameters<SpecRefInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            let sections = store.list_sections(&input.id).map_err(Self::spec_err)?;
            Self::json_result(&json!({
                "status": "ok",
                "spec": input.id,
                "count": sections.len(),
                "sections": sections,
            }))
        }).await
    }

    #[tool(name = "spec_section_get", description = "Get section content.")]
    async fn spec_section_get(
        &self,
        Parameters(input): Parameters<SectionRefInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            let uuid = store.resolve_id(&input.id).map_err(Self::spec_err)?;
            let indexed = store.entity_store().get_indexed(&uuid)
                .map_err(Self::storage_err)?
                .ok_or_else(|| McpError::invalid_params("spec not found", None))?;
            let file_name = if input.name.ends_with(".md") {
                input.name.clone()
            } else {
                format!("{}.md", input.name)
            };
            let path = indexed.path.join("sections").join(&file_name);
            let content = std::fs::read_to_string(&path)
                .map_err(|e| McpError::invalid_params(
                    format!("section not found: {e}"), None,
                ))?;
            Self::json_result(&json!({
                "status": "ok",
                "spec": input.id,
                "section": input.name,
                "content": content,
            }))
        }).await
    }

    #[tool(name = "spec_section_delete", description = "Delete a section from a spec.")]
    async fn spec_section_delete(
        &self,
        Parameters(input): Parameters<SectionRefInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            store.delete_section(&input.id, &input.name).map_err(Self::spec_err)?;
            Self::json_result(&json!({
                "status": "ok",
                "spec": input.id,
                "section": input.name,
            }))
        }).await
    }

    #[tool(name = "spec_scan", description = "Scan and reindex all spec roots.")]
    async fn spec_scan(
        &self,
        Parameters(input): Parameters<ScanInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            let report = store.scan(input.force).map_err(Self::spec_err)?;
            Self::json_result(&json!({
                "status": "ok",
                "force": input.force,
                "integrated": report.integrated,
                "pruned": report.pruned,
                "diagnostics_count": report.diagnostics.len(),
            }))
        }).await
    }

    #[tool(name = "spec_add_root", description = "Register a directory as a scan root for specs.")]
    async fn spec_add_root(
        &self,
        Parameters(input): Parameters<AddRootInput>,
    ) -> Result<CallToolResult, McpError> {
        self.with_store(|store| {
            let path = PathBuf::from(&input.path);
            let label = input.label.unwrap_or_else(|| {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("specs")
                    .to_string()
            });
            store.entity_store().add_scan_root(ScanRoot {
                path: path.clone(),
                label: label.clone(),
            }).map_err(Self::storage_err)?;
            Self::json_result(&json!({
                "status": "ok",
                "path": path,
                "label": label,
            }))
        }).await
    }
}
```

#### 4d: MCP Handler Trait and Server Startup

```rust
#[tool_handler]
impl ServerHandler for SpecServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "spec-mcp provides direct access to the spec store. Use named tools for spec operations."
                    .to_string(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

pub async fn run_mcp_server(
    index_root: PathBuf,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server = SpecServer::new(index_root);

    tracing::info!("Starting spec-mcp server on stdio (direct store access)");

    let service = server.serve(stdio()).await.inspect_err(|err| {
        eprintln!("Server error: {err:?}");
    })?;

    service.waiting().await?;
    Ok(())
}
```

### Step 5: Resolve Index Root Dependency

The `main.rs` above references `ticket_api::workspace::resolve_workspace()`. If you want to avoid pulling in `ticket-api` as a dependency:

**Option A** (recommended): Add `ticket-api` as an optional dependency just for workspace resolution and reuse the same root.

**Option B**: Implement a simple standalone resolution:
```rust
fn resolve_index_root() -> PathBuf {
    if let Ok(p) = std::env::var("SPEC_INDEX_ROOT") {
        return PathBuf::from(p);
    }
    if let Ok(p) = std::env::var("TICKET_INDEX_ROOT") {
        return PathBuf::from(p);
    }
    let cwd_spec = std::env::current_dir().ok().map(|d| d.join(".spec"));
    if let Some(p) = cwd_spec.filter(|p| p.exists()) {
        return p;
    }
    if let Ok(home) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
        return PathBuf::from(home).join(".spec-index");
    }
    PathBuf::from(".spec")
}
```

Go with whichever approach keeps the dependency tree cleanest.

## Scoping Note — Deferred Tools

`spec_skill_generate` and `spec_toc` depend on other tickets:
- Skill generation → ticket `eddf5d2e`
- Table of contents → ticket `a7b2a89c`

Omit these tools for now. They'll be added when their respective tickets are worked.

## Validation

```bash
cargo build -p spec-mcp
cargo test -p spec-mcp

# Smoke test (will start on stdio — Ctrl+C to stop):
SPEC_INDEX_ROOT=/tmp/test-spec-mcp ./target/debug/spec-mcp
```

Since MCP servers communicate via stdio JSON-RPC, automated testing is best done via integration tests. Consider writing a basic integration test that:
1. Creates a temp dir with a SpecStore
2. Instantiates `SpecServer::new(temp_path)`
3. Calls tool methods directly (they return `Result<CallToolResult, McpError>`)

## Key Constraints

1. **Follow ticket-mcp patterns exactly** — same struct layout, same macro usage, same error handling.
2. **`&mut SpecStore`** — the `with_store` closure must receive `&mut SpecStore` since create/update/delete/scan mutate the slug index.
3. **Auto-scan on store open** — call `store.scan(false)` inside `with_store` after opening to ensure slug resolution works.
4. **Slug resolution everywhere** — all tools that take an ID should accept UUID, UUID prefix, or slug (handled by `store.resolve_id()`).
5. **rmcp version** — use the same `rmcp = "0.14"` as ticket-mcp.
6. **No `workspace` field** — unlike ticket-mcp which has multi-workspace support, spec-mcp uses a single store. Do not add a `workspace` field to input structs unless multi-workspace support is needed.
