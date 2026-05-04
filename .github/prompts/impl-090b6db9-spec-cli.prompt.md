---
description: "Implement ticket 090b6db9: spec-cli — CRUD, search, hierarchy, health commands"
---

# Ticket 090b6db9 — spec-cli: Command-Line Interface

## Goal

Create a `spec` CLI binary mirroring the `ticket` CLI pattern. Provides CRUD, search, hierarchy navigation, section management, scan, and health check commands against `SpecStore`.

## Ticket State Management

```bash
# At start:
./target/debug/ticket.exe update 090b6db9 --to-state in-implementation
./target/debug/ticket.exe board check-in 090b6db9 --agent-id copilot \
  --intent "implementing spec-cli" \
  --files "tools/cli/spec-cli/Cargo.toml,tools/cli/spec-cli/src/bin/spec.rs,tools/cli/spec-cli/src/lib.rs,tools/cli/spec-cli/src/cli.rs,tools/cli/spec-cli/src/cli/args.rs,tools/cli/spec-cli/src/cli/dispatch.rs,tools/cli/spec-cli/src/cli/commands/mod.rs,tools/cli/spec-cli/src/cli/commands/crud.rs,tools/cli/spec-cli/src/cli/commands/query.rs,tools/cli/spec-cli/src/cli/commands/hierarchy.rs,tools/cli/spec-cli/src/cli/commands/refs.rs,tools/cli/spec-cli/src/cli/commands/sections.rs" \
  --ttl 3600

# At end (after tests pass):
./target/debug/ticket.exe update 090b6db9 --to-state in-review
```

## Reference Implementation

The ticket CLI lives at `tools/cli/ticket-cli/` and is the exact pattern to follow. Key files:

| ticket-cli file | Purpose | spec-cli equivalent |
|---|---|---|
| `src/bin/ticket.rs` | Entry point: parse args → run → format output | `src/bin/spec.rs` |
| `src/lib.rs` | `pub mod cli;` | Same |
| `src/cli.rs` | Defines `SpecCli`, `SpecCommandCli`, `CliRunError`, `CliOutput`, `run()`, `error_output()`, `parse_cli_from()` | Same |
| `src/cli/args.rs` | All clap `#[derive(Args)]` structs | Same |
| `src/cli/dispatch.rs` | Opens SpecStore, dispatches command to handler | Same |
| `src/cli/commands/mod.rs` | Re-exports all command modules | Same |
| `src/cli/commands/crud.rs` | create, get, update, delete, list | Same |
| `src/cli/commands/query.rs` | search, scan | Same |

## Architecture

```
tools/cli/spec-cli/
├── Cargo.toml
└── src/
    ├── bin/spec.rs          ← entry point
    ├── lib.rs               ← pub mod cli;
    └── cli.rs               ← SpecCli, dispatch, output
        (uses #[path = "cli/..."] mod pattern like ticket-cli)
    └── cli/
        ├── args.rs           ← all clap arg structs
        ├── dispatch.rs       ← open SpecStore, match command → handler
        └── commands/
            ├── mod.rs        ← re-exports + resolve_id helper
            ├── crud.rs       ← create, get, update, delete, list
            ├── query.rs      ← search, scan
            ├── hierarchy.rs  ← tree, children, ancestors
            ├── refs.rs       ← refs list, refs validate
            └── sections.rs   ← section add/list/get/delete
```

## SpecStore API Available

The `SpecStore` in `crates/spec-api/src/store.rs` provides:

```rust
// Constructor
SpecStore::open(index_root) -> Result<Self, SpecError>

// Scan
store.scan(reindex: bool) -> Result<ScanReport, SpecError>
store.entity_store() -> &EntityStore  // access inner for scan roots, search, edges

// Resolution
store.resolve_id(id_or_slug) -> Result<Uuid, SpecError>

// CRUD
store.create(&SpecManifest, body, target_root) -> Result<SpecId, SpecError>  // &mut self
store.get(id_or_slug) -> Result<SpecManifest, SpecError>
store.get_full(id_or_slug) -> Result<(SpecManifest, String), SpecError>
store.update(id_or_slug, patch, to_state) -> Result<SpecManifest, SpecError>  // &mut self
store.update_body(id_or_slug, content) -> Result<(), SpecError>
store.delete(id_or_slug) -> Result<(), SpecError>  // &mut self

// Sections
store.add_section(id_or_slug, name, content) -> Result<(), SpecError>
store.update_section(id_or_slug, name, content) -> Result<(), SpecError>
store.delete_section(id_or_slug, name) -> Result<(), SpecError>
store.list_sections(id_or_slug) -> Result<Vec<String>, SpecError>

// Hierarchy
store.children(id_or_slug) -> Result<Vec<SpecManifest>, SpecError>
store.ancestors(id_or_slug) -> Result<Vec<SpecManifest>, SpecError>
store.subtree(id_or_slug) -> Result<Vec<SpecManifest>, SpecError>

// Search (via entity_store)
store.entity_store().search(query, limit) -> Result<Vec<SearchResult>, StorageError>

// Scan roots (via entity_store)
store.entity_store().add_scan_root(ScanRoot)
store.entity_store().list_scan_roots()
```

**Note:** Methods that mutate `slug_index` (`create`, `update`, `delete`) take `&mut self`. This means the dispatch function must hold a `mut store`.

## Implementation

### Step 1: Create `tools/cli/spec-cli/Cargo.toml`

```toml
[package]
name = "spec-cli"
version = "0.1.0"
edition = "2024"
description = "CLI interface for spec-api: the `spec` command"

[[bin]]
name = "spec"
path = "src/bin/spec.rs"

[lib]
name = "spec_cli"
path = "src/lib.rs"

[dependencies]
spec-api = { path = "../../../crates/spec-api" }
memory-api = { path = "../../../crates/memory-api" }

clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["serde", "v4"] }
thiserror = "2"

[dev-dependencies]
pretty_assertions = "1"
tempfile = "3"
```

Also add `spec-cli` to the workspace `Cargo.toml` members list.

### Step 2: Create `src/bin/spec.rs`

Follow `tools/cli/ticket-cli/src/bin/ticket.rs` exactly — parse args, run, format JSON or text output.

```rust
use spec_cli::cli::{CliOutput, error_output, parse_cli_from, run};

fn main() {
    let cli = match parse_cli_from(std::env::args_os()) {
        Ok(cli) => cli,
        Err(err) => {
            let wants_json = std::env::args().any(|a| a == "--json");
            let rendered = error_output(&err.to_string(), wants_json);
            eprintln!("{rendered}");
            std::process::exit(2);
        }
    };

    match run(cli) {
        Ok(CliOutput::Json(value)) => {
            match serde_json::to_string_pretty(&value) {
                Ok(rendered) => println!("{rendered}"),
                Err(err) => {
                    eprintln!("{}", error_output(&err.to_string(), true));
                    std::process::exit(1);
                }
            }
        }
        Ok(CliOutput::Text(text)) => println!("{text}"),
        Err(err) => {
            let wants_json = std::env::args().any(|a| a == "--json");
            eprintln!("{}", error_output(&err.to_string(), wants_json));
            std::process::exit(1);
        }
    }
}
```

### Step 3: Create `src/lib.rs`

```rust
pub mod cli;
```

### Step 4: Create `src/cli.rs`

This is the central module that ties everything together. Follow ticket-cli's `cli.rs` pattern:

```rust
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde_json::{Value, json};
use uuid::Uuid;

use spec_api::error::SpecError;

#[path = "cli/args.rs"]
mod args;
#[path = "cli/commands/mod.rs"]
mod commands;
#[path = "cli/dispatch.rs"]
mod dispatch;

pub use args::*;

#[derive(Debug, Parser)]
#[command(name = "spec", about = "Specification system CLI", version)]
pub struct SpecCli {
    /// Return machine-readable JSON output.
    #[arg(long, global = true)]
    pub json: bool,

    /// Root directory for the SQLite index and Tantivy search index.
    #[arg(long, global = true)]
    pub index_root: Option<PathBuf>,

    #[command(subcommand)]
    pub command: SpecCommandCli,
}

#[derive(Debug, Subcommand)]
pub enum SpecCommandCli {
    /// Create a new spec.
    Create(CreateArgs),
    /// Get a spec by ID or slug.
    Get(GetArgs),
    /// Update a spec's fields or state.
    Update(UpdateArgs),
    /// Soft-delete a spec.
    Delete(IdArgs),
    /// List specs with optional filtering.
    List(ListArgs),
    /// Full-text search over specs.
    Search(SearchArgs),
    /// Run full scan/reindex over registered scan roots.
    Scan(ScanArgs),
    /// Register a scan root directory.
    #[command(name = "add-root")]
    AddRoot(AddRootArgs),
    /// Show hierarchy as a tree.
    Tree(TreeArgs),
    /// List code references for a spec.
    Refs(RefsArgs),
    /// Manage spec sections.
    Section(SectionArgs),
    /// Run health checks on specs.
    Health(HealthArgs),
}

// ── error type ──

#[derive(Debug, thiserror::Error)]
pub enum CliRunError {
    #[error("spec error: {0}")]
    Spec(#[from] SpecError),
    #[error("storage error: {0}")]
    Storage(#[from] memory_api::error::StorageError),
    #[error("{0}")]
    BadRequest(String),
}

pub enum CliOutput {
    Json(Value),
    Text(String),
}

// ── entry point ──

pub fn run(cli: SpecCli) -> Result<CliOutput, CliRunError> {
    let payload = dispatch::dispatch(
        cli.command,
        cli.index_root.as_deref(),
        cli.json,
    )?;
    if cli.json {
        Ok(CliOutput::Json(payload))
    } else {
        Ok(CliOutput::Text(render_human(&payload)))
    }
}

fn render_human(payload: &Value) -> String {
    // Simple human-readable rendering
    serde_json::to_string_pretty(payload).unwrap_or_else(|_| format!("{:?}", payload))
}

pub fn error_output(message: &str, as_json: bool) -> String {
    if as_json {
        json!({"status": "error", "message": message}).to_string()
    } else {
        message.to_string()
    }
}

pub fn parse_cli_from<I, T>(args: I) -> Result<SpecCli, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    SpecCli::try_parse_from(args)
}
```

### Step 5: Create `src/cli/args.rs`

```rust
use std::path::PathBuf;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Spec title (required).
    #[arg(long)]
    pub title: String,
    /// Hierarchical slug (e.g. "ticket-api/storage/store").
    #[arg(long)]
    pub slug: String,
    /// Component this spec belongs to.
    #[arg(long)]
    pub component: String,
    /// Parent spec ID or slug for hierarchy.
    #[arg(long)]
    pub parent: Option<String>,
    /// Scope (e.g. "public", "internal").
    #[arg(long)]
    pub scope: Option<String>,
    /// Read spec body from this file.
    #[arg(long = "body-file")]
    pub body_file: Option<PathBuf>,
    /// Place the spec in this scan root.
    #[arg(long = "root")]
    pub target_root: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct GetArgs {
    /// Spec UUID, prefix, or slug.
    pub id: String,
    /// Include body and sections in output.
    #[arg(long, default_value_t = false)]
    pub full: bool,
}

#[derive(Debug, Args)]
pub struct IdArgs {
    /// Spec UUID, prefix, or slug.
    pub id: String,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Spec UUID, prefix, or slug.
    pub id: String,
    /// Field patches as key=value pairs.
    #[arg(long = "field")]
    pub fields: Vec<String>,
    /// Transition to this state.
    #[arg(long = "state")]
    pub to_state: Option<String>,
    /// Update body from file.
    #[arg(long = "body-file")]
    pub body_file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter by field=value predicates.
    #[arg(long = "where")]
    pub where_clauses: Vec<String>,
    /// Maximum results.
    #[arg(long)]
    pub limit: Option<usize>,
}

#[derive(Debug, Args)]
pub struct SearchArgs {
    /// Search query.
    pub query: String,
    /// Maximum results.
    #[arg(long, default_value = "20")]
    pub limit: usize,
}

#[derive(Debug, Args)]
pub struct ScanArgs {
    /// Force full reindex (rebuilds search index).
    #[arg(long, default_value_t = false)]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct AddRootArgs {
    /// Directory path to register as a scan root.
    pub path: PathBuf,
    /// Optional label for this root.
    #[arg(long)]
    pub label: Option<String>,
}

#[derive(Debug, Args)]
pub struct TreeArgs {
    /// Root spec ID or slug to start from (shows full tree if omitted).
    pub id: Option<String>,
}

#[derive(Debug, Args)]
pub struct RefsArgs {
    /// Spec UUID, prefix, or slug.
    pub id: String,
    #[command(subcommand)]
    pub subcommand: Option<RefsSubcommand>,
}

#[derive(Debug, Subcommand)]
pub enum RefsSubcommand {
    /// Validate code references (check file existence, line ranges).
    Validate {
        /// Workspace root for resolving file paths.
        #[arg(long, default_value = ".")]
        workspace_root: PathBuf,
    },
}

#[derive(Debug, Args)]
pub struct SectionArgs {
    #[command(subcommand)]
    pub command: SectionCommand,
}

#[derive(Debug, Subcommand)]
pub enum SectionCommand {
    /// Add a section to a spec.
    Add {
        /// Spec UUID, prefix, or slug.
        id: String,
        /// Section name (will be used as filename, .md appended if missing).
        #[arg(long)]
        name: String,
        /// Read section content from this file.
        #[arg(long)]
        file: PathBuf,
    },
    /// List sections of a spec.
    List {
        /// Spec UUID, prefix, or slug.
        id: String,
    },
    /// Get section content.
    Get {
        /// Spec UUID, prefix, or slug.
        id: String,
        /// Section name.
        name: String,
    },
    /// Delete a section.
    Delete {
        /// Spec UUID, prefix, or slug.
        id: String,
        /// Section name.
        name: String,
    },
}

#[derive(Debug, Args)]
pub struct HealthArgs {
    /// Spec UUID, prefix, or slug (omit with --all for all specs).
    pub id: Option<String>,
    /// Check all specs.
    #[arg(long, default_value_t = false)]
    pub all: bool,
}
```

### Step 6: Create `src/cli/dispatch.rs`

```rust
use std::path::{Path, PathBuf};
use serde_json::Value;

use spec_api::SpecStore;
use crate::cli::{CliRunError, SpecCommandCli, commands};

pub(super) fn dispatch(
    command: SpecCommandCli,
    index_root_override: Option<&Path>,
    _as_json: bool,
) -> Result<Value, CliRunError> {
    let index_root = resolve_index_root(index_root_override);
    let mut store = SpecStore::open(&index_root)?;

    // Auto-scan to pick up any new spec folders
    store.scan(false)?;

    match command {
        SpecCommandCli::Create(args) => commands::cmd_create(args, &mut store),
        SpecCommandCli::Get(args) => commands::cmd_get(args, &store),
        SpecCommandCli::Update(args) => commands::cmd_update(args, &mut store),
        SpecCommandCli::Delete(args) => commands::cmd_delete(args, &mut store),
        SpecCommandCli::List(args) => commands::cmd_list(args, &store),
        SpecCommandCli::Search(args) => commands::cmd_search(args, &store),
        SpecCommandCli::Scan(args) => commands::cmd_scan(args, &mut store),
        SpecCommandCli::AddRoot(args) => commands::cmd_add_root(args, &store),
        SpecCommandCli::Tree(args) => commands::cmd_tree(args, &store),
        SpecCommandCli::Refs(args) => commands::cmd_refs(args, &store),
        SpecCommandCli::Section(args) => commands::cmd_section(args, &store),
        SpecCommandCli::Health(args) => commands::cmd_health(args, &store),
    }
}

fn resolve_index_root(override_path: Option<&Path>) -> PathBuf {
    if let Some(p) = override_path {
        return p.to_path_buf();
    }
    if let Ok(env_val) = std::env::var("SPEC_INDEX_ROOT") {
        return PathBuf::from(env_val);
    }
    // Default: .spec/ in current working directory, or ~/.spec-index/
    let cwd_spec = std::env::current_dir()
        .ok()
        .map(|d| d.join(".spec"));
    if let Some(p) = cwd_spec.filter(|p| p.exists()) {
        return p;
    }
    dirs::home_dir()
        .map(|h| h.join(".spec-index"))
        .unwrap_or_else(|| PathBuf::from(".spec"))
}
```

**Note on `dirs` crate**: If you don't want to add the `dirs` dependency, use a simpler fallback like `$HOME/.spec-index` via env var, or just default to `.spec` in CWD.

### Step 7: Create `src/cli/commands/mod.rs`

```rust
mod crud;
mod query;
mod hierarchy;
mod refs;
mod sections;

pub(crate) use crud::*;
pub(crate) use query::*;
pub(crate) use hierarchy::*;
pub(crate) use refs::*;
pub(crate) use sections::*;
```

### Step 8: Create `src/cli/commands/crud.rs`

Handle `create`, `get`, `update`, `delete`, `list`:

```rust
use std::collections::BTreeMap;
use serde_json::{Value, json};
use spec_api::{SpecManifest, SpecStore};
use crate::cli::{CliRunError, CreateArgs, GetArgs, IdArgs, ListArgs, UpdateArgs};

pub(crate) fn cmd_create(args: CreateArgs, store: &mut SpecStore) -> Result<Value, CliRunError> {
    let mut manifest = SpecManifest::new(&args.slug, &args.title, &args.component);
    if let Some(parent) = &args.parent {
        let parent_id = store.resolve_id(parent)?;
        manifest.set_parent(&parent_id.to_string());
    }
    if let Some(scope) = &args.scope {
        manifest.set_scope(scope);
    }
    let body = args.body_file
        .map(|p| std::fs::read_to_string(&p)
            .map_err(|e| CliRunError::BadRequest(format!("cannot read body-file: {e}"))))
        .transpose()?
        .unwrap_or_default();

    let id = store.create(&manifest, &body, args.target_root.as_deref())?;
    Ok(json!({
        "command": "create",
        "status": "ok",
        "id": id,
        "slug": args.slug,
        "title": args.title,
        "component": args.component,
        "state": "draft",
    }))
}

pub(crate) fn cmd_get(args: GetArgs, store: &SpecStore) -> Result<Value, CliRunError> {
    if args.full {
        let (spec, body) = store.get_full(&args.id)?;
        let sections = store.list_sections(&args.id)?;
        Ok(json!({
            "command": "get",
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
        let spec = store.get(&args.id)?;
        Ok(json!({
            "command": "get",
            "status": "ok",
            "spec": {
                "id": spec.id,
                "created_at": spec.created_at,
                "fields": spec.extra,
                "code_refs": spec.code_refs,
            },
        }))
    }
}

pub(crate) fn cmd_update(args: UpdateArgs, store: &mut SpecStore) -> Result<Value, CliRunError> {
    let mut patch = BTreeMap::new();
    for f in &args.fields {
        let (k, v) = f.split_once('=')
            .ok_or_else(|| CliRunError::BadRequest(format!("invalid field patch: {f}")))?;
        patch.insert(k.to_string(), Value::String(v.to_string()));
    }

    // Update body if provided
    if let Some(body_file) = &args.body_file {
        let content = std::fs::read_to_string(body_file)
            .map_err(|e| CliRunError::BadRequest(format!("cannot read body-file: {e}")))?;
        store.update_body(&args.id, &content)?;
    }

    let spec = store.update(&args.id, patch, args.to_state.as_deref())?;
    Ok(json!({
        "command": "update",
        "status": "ok",
        "id": spec.id,
        "fields": spec.extra,
    }))
}

pub(crate) fn cmd_delete(args: IdArgs, store: &mut SpecStore) -> Result<Value, CliRunError> {
    let id = store.resolve_id(&args.id)?;
    store.delete(&args.id)?;
    Ok(json!({
        "command": "delete",
        "status": "ok",
        "id": id,
    }))
}

pub(crate) fn cmd_list(args: ListArgs, store: &SpecStore) -> Result<Value, CliRunError> {
    let all = store.entity_store().list_indexed(false)?;
    let mut items: Vec<Value> = Vec::new();

    for indexed in &all {
        // Apply where-clause filters
        let spec = match store.get(&indexed.id.to_string()) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let mut matches = true;
        for clause in &args.where_clauses {
            if let Some((k, v)) = clause.split_once('=') {
                let field_val = spec.extra.get(k).and_then(|fv| fv.as_str());
                if field_val != Some(v) {
                    matches = false;
                    break;
                }
            }
        }
        if !matches { continue; }

        items.push(json!({
            "id": indexed.id,
            "slug": spec.slug(),
            "title": spec.title(),
            "state": spec.state(),
            "component": spec.component(),
        }));

        if let Some(limit) = args.limit {
            if items.len() >= limit { break; }
        }
    }

    Ok(json!({
        "command": "list",
        "status": "ok",
        "count": items.len(),
        "items": items,
    }))
}
```

### Step 9: Create `src/cli/commands/query.rs`

Handle `search`, `scan`, `add-root`:

```rust
use serde_json::{Value, json};
use memory_api::model::filesystem::ScanRoot;
use spec_api::SpecStore;
use crate::cli::{CliRunError, SearchArgs, ScanArgs, AddRootArgs};

pub(crate) fn cmd_search(args: SearchArgs, store: &SpecStore) -> Result<Value, CliRunError> {
    let results = store.entity_store().search(&args.query, args.limit)?;
    let items: Vec<Value> = results.iter().map(|r| json!({
        "id": r.id,
        "title": r.title,
        "state": r.state,
        "type": r.ticket_type,
        "score": r.score,
        "snippet": r.snippet,
    })).collect();
    Ok(json!({
        "command": "search",
        "status": "ok",
        "query": args.query,
        "count": items.len(),
        "items": items,
    }))
}

pub(crate) fn cmd_scan(args: ScanArgs, store: &mut SpecStore) -> Result<Value, CliRunError> {
    let report = store.scan(args.force)?;
    Ok(json!({
        "command": "scan",
        "status": "ok",
        "force": args.force,
        "integrated": report.integrated,
        "pruned": report.pruned,
        "diagnostics_count": report.diagnostics.len(),
    }))
}

pub(crate) fn cmd_add_root(args: AddRootArgs, store: &SpecStore) -> Result<Value, CliRunError> {
    let path = std::fs::canonicalize(&args.path)
        .unwrap_or_else(|_| args.path.clone());
    let label = args.label.unwrap_or_else(|| {
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("specs")
            .to_string()
    });
    store.entity_store().add_scan_root(ScanRoot {
        path: path.clone(),
        label: label.clone(),
    })?;
    Ok(json!({
        "command": "add_root",
        "status": "ok",
        "path": path,
        "label": label,
    }))
}
```

### Step 10: Create `src/cli/commands/hierarchy.rs`

Handle `tree`:

```rust
use serde_json::{Value, json};
use spec_api::SpecStore;
use crate::cli::{CliRunError, TreeArgs};

pub(crate) fn cmd_tree(args: TreeArgs, store: &SpecStore) -> Result<Value, CliRunError> {
    if let Some(root_id) = &args.id {
        // Show subtree from a specific root
        let root = store.get(root_id)?;
        let children = store.subtree(root_id)?;
        Ok(json!({
            "command": "tree",
            "status": "ok",
            "root": {
                "id": root.id,
                "slug": root.slug(),
                "title": root.title(),
                "state": root.state(),
            },
            "descendants": children.iter().map(|c| json!({
                "id": c.id,
                "slug": c.slug(),
                "title": c.title(),
                "state": c.state(),
                "parent": c.parent(),
            })).collect::<Vec<_>>(),
        }))
    } else {
        // Show all root specs (no parent) with their children
        let all = store.entity_store().list_indexed(false)?;
        let mut roots = Vec::new();
        for indexed in &all {
            if let Ok(spec) = store.get(&indexed.id.to_string()) {
                if spec.parent().is_none() {
                    let children = store.children(&indexed.id.to_string())?;
                    roots.push(json!({
                        "id": spec.id,
                        "slug": spec.slug(),
                        "title": spec.title(),
                        "children_count": children.len(),
                    }));
                }
            }
        }
        Ok(json!({
            "command": "tree",
            "status": "ok",
            "roots": roots,
        }))
    }
}
```

### Step 11: Create `src/cli/commands/refs.rs`

```rust
use std::path::Path;
use serde_json::{Value, json};
use spec_api::SpecStore;
use spec_api::code_ref::validate_refs;
use crate::cli::{CliRunError, RefsArgs, RefsSubcommand};

pub(crate) fn cmd_refs(args: RefsArgs, store: &SpecStore) -> Result<Value, CliRunError> {
    let spec = store.get(&args.id)?;

    match args.subcommand {
        Some(RefsSubcommand::Validate { workspace_root }) => {
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
            Ok(json!({
                "command": "refs_validate",
                "status": "ok",
                "id": spec.id,
                "valid": all_valid,
                "count": items.len(),
                "results": items,
            }))
        }
        None => {
            // List code refs
            let refs: Vec<Value> = spec.code_refs.iter().map(|r| json!({
                "file": r.file,
                "symbol": r.symbol,
                "kind": format!("{:?}", r.kind),
                "line_start": r.line_start,
                "line_end": r.line_end,
                "description": r.description,
            })).collect();
            Ok(json!({
                "command": "refs",
                "status": "ok",
                "id": spec.id,
                "count": refs.len(),
                "refs": refs,
            }))
        }
    }
}
```

### Step 12: Create `src/cli/commands/sections.rs`

```rust
use serde_json::{Value, json};
use spec_api::SpecStore;
use crate::cli::{CliRunError, SectionArgs, SectionCommand};

pub(crate) fn cmd_section(args: SectionArgs, store: &SpecStore) -> Result<Value, CliRunError> {
    match args.command {
        SectionCommand::Add { id, name, file } => {
            let content = std::fs::read_to_string(&file)
                .map_err(|e| CliRunError::BadRequest(format!("cannot read file: {e}")))?;
            store.add_section(&id, &name, &content)?;
            Ok(json!({
                "command": "section_add",
                "status": "ok",
                "spec": id,
                "section": name,
            }))
        }
        SectionCommand::List { id } => {
            let sections = store.list_sections(&id)?;
            Ok(json!({
                "command": "section_list",
                "status": "ok",
                "spec": id,
                "count": sections.len(),
                "sections": sections,
            }))
        }
        SectionCommand::Get { id, name } => {
            // Read section content directly from filesystem
            let uuid = store.resolve_id(&id)?;
            let indexed = store.entity_store().get_indexed(&uuid)?
                .ok_or_else(|| CliRunError::BadRequest("spec not found".into()))?;
            let file_name = if name.ends_with(".md") { name.clone() } else { format!("{}.md", name) };
            let path = indexed.path.join("sections").join(&file_name);
            let content = std::fs::read_to_string(&path)
                .map_err(|e| CliRunError::BadRequest(format!("section not found: {e}")))?;
            Ok(json!({
                "command": "section_get",
                "status": "ok",
                "spec": id,
                "section": name,
                "content": content,
            }))
        }
        SectionCommand::Delete { id, name } => {
            store.delete_section(&id, &name)?;
            Ok(json!({
                "command": "section_delete",
                "status": "ok",
                "spec": id,
                "section": name,
            }))
        }
    }
}
```

### Step 13: Add `cmd_health`

Add a basic health check in one of the command modules (e.g. `query.rs` or a separate file):

```rust
pub(crate) fn cmd_health(args: HealthArgs, store: &SpecStore) -> Result<Value, CliRunError> {
    let specs = if args.all {
        let all = store.entity_store().list_indexed(false)?;
        all.iter()
            .filter_map(|e| store.get(&e.id.to_string()).ok())
            .collect::<Vec<_>>()
    } else if let Some(id) = &args.id {
        vec![store.get(id)?]
    } else {
        return Err(CliRunError::BadRequest("provide spec ID or --all".into()));
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

    Ok(json!({
        "command": "health",
        "status": "ok",
        "specs_checked": specs.len(),
        "issues_count": issues.len(),
        "issues": issues,
    }))
}
```

### Step 14: Add to workspace Cargo.toml

Add `"tools/cli/spec-cli"` to the `[workspace] members` list in the root `Cargo.toml`.

## Scoping Note — Deferred Commands

The ticket description mentions `toc` and `skill generate` commands. These depend on other tickets:
- `toc` → ticket `a7b2a89c` (Table of contents)
- `skill generate` → ticket `eddf5d2e` (Skill generation)

**Implement these as stubs** (print a "not yet implemented" message) or omit them entirely from this ticket. They'll be added when their respective tickets are worked.

## Validation

```bash
cargo build -p spec-cli
cargo test -p spec-cli
# Quick smoke test:
./target/debug/spec --help
./target/debug/spec create --title "Test" --slug "test/example" --component "test" --json
./target/debug/spec list --json
./target/debug/spec get test/example --json
```

## Key Constraints

1. **Follow ticket-cli patterns exactly** — same module structure, same `#[path = "cli/..."]` mod pattern, same JSON output structure with `command`/`status` fields.
2. **`&mut store`** — the dispatch function must hold `mut store` because `create`, `update`, `delete`, `scan` mutate the slug index.
3. **Auto-scan on startup** — call `store.scan(false)` after opening to pick up any new spec folders. This ensures slug resolution works immediately.
4. **Slug resolution everywhere** — all commands that take an ID should accept UUID, UUID prefix, or slug.
5. **No `dirs` crate** unless already in the workspace — use env vars for fallback index root.
6. **Stub `toc` and `skill generate`** or omit — those are separate tickets.
