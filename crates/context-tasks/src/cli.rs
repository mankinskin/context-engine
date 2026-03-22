use std::collections::BTreeMap;
use std::path::PathBuf;

use chrono::Utc;
use clap::{Args, Parser, Subcommand};
use serde::Serialize;
use serde_json::{Map, Value, json};
use uuid::Uuid;

use crate::model::edge::EdgeRecord;

use crate::execution::provider::{CopilotApiClient, CopilotApiConfig, ProviderError, StartSubagentResponse, SubagentProvider};
use crate::execution::runner::{AssignmentRunRequest, AssignmentRunner, GitSandboxProvisioner, RunnerConfig, SandboxProvisioner};
use crate::execution::sandbox::{SandboxError, SandboxHandle, SandboxSpec};
use crate::storage::store::GateStatus;
use crate::contracts::command_schema::{
    CommandEnvelope, ErrorEnvelope, export_command_schema, export_command_schema_json,
};
use crate::error::StorageError;
use crate::model::schema_registry::SchemaRegistry;
use crate::storage::TicketStore;
use crate::workspace::{self, WorkspaceConfig};

// ── CLI root ───────────────────────────────────────────────────────────────────

#[derive(Debug, Parser)]
#[command(name = "ticket", about = "Task tracker CLI", version)]
pub struct TicketCli {
    /// Return machine-readable JSON envelope output.
    #[arg(long, global = true)]
    pub json: bool,

    /// Optional request identifier propagated in JSON envelope output.
    #[arg(long, global = true)]
    pub request_id: Option<String>,

    /// Root directory for the redb index and Tantivy search index.
    /// Defaults to $TICKET_INDEX_ROOT env var, then ~/.ticket-index/.
    #[arg(long, global = true)]
    pub index_root: Option<PathBuf>,

    /// Directory containing additional ticket type schema TOML files.
    /// Each `<type-id>.toml` file overrides or supplements the built-in schemas.
    #[arg(long, global = true)]
    pub schema_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: TicketCommandCli,
}

#[derive(Debug, Subcommand)]
pub enum TicketCommandCli {
    /// Create a new ticket.
    Create(CreateArgs),
    /// Get a ticket by UUID.
    Get(IdArgs),
    /// Update a ticket with field patches and optional state transition.
    Update(UpdateArgs),
    /// List tickets with optional state/type filtering.
    List(ListArgs),
    /// Soft-delete a ticket.
    Delete(IdArgs),
    /// Run full scan/reindex over registered scan roots.
    Scan(ScanArgs),
    /// Claim a ticket lease for active work.
    Claim(ClaimArgs),
    /// Release an active ticket lease.
    Unclaim(UnclaimArgs),
    /// List all active leases.
    Leases,
    /// Full-text + metadata search over tickets.
    Search(TextArgs),
    /// Unified query expression (alias for search).
    Query(TextArgs),
    /// Register a scan root directory.
    #[command(name = "add-root")]
    AddRoot(AddRootArgs),
    /// History log for a ticket (Phase 2 — stub).
    History(HistoryArgs),
    /// Diff a ticket between revisions (Phase 2 — stub).
    Diff(DiffArgs),
    /// Revert a ticket to a historical revision (Phase 2 — stub).
    Revert(RevertArgs),
    /// Mark merge-boundary completion metadata (Phase 2 — stub).
    #[command(name = "finalize-merge")]
    FinalizeMerge(FinalizeMergeArgs),
    /// Execute a single TaskCommand JSON request from stdin (agent protocol).
    Exec(ExecArgs),
    /// Execute a batch of TaskCommand JSON requests from stdin or file.
    Batch(BatchArgs),
    /// Export the command namespace/schema for automation clients.
    #[command(name = "export-command-schema")]
    ExportCommandSchema,
    /// Add a directed edge (dependency/link) between two tickets.
    Link(LinkArgs),
    /// Remove a directed edge between two tickets.
    Unlink(UnlinkArgs),
    /// List all edges originating from a ticket.
    Links(IdArgs),
    /// Manage named workspaces (named index roots).
    Workspace(WorkspaceArgs),
    /// Watch filesystem scan roots and auto-reconcile on changes.
    Watch(WatchArgs),
    /// Dashboard: current state summary + ready tickets + parallel opportunities.
    Status(StatusArgs),
    /// Return a JSON overview of ready tickets.
    #[command(name = "ready-overview")]
    ReadyOverview(ReadyOverviewArgs),
    /// Start the HTTP server exposing the ticket API (REST + SSE).
    Serve(ServeCliArgs),
}

// ── arg structs ────────────────────────────────────────────────────────────────

#[derive(Debug, Args)]
pub struct CreateArgs {
    #[arg(long)]
    pub id: Option<Uuid>,
    #[arg(long = "type")]
    pub ticket_type: Option<String>,
    #[arg(long)]
    pub title: Option<String>,
    #[arg(long)]
    pub state: Option<String>,
    #[arg(long = "field")]
    pub fields: Vec<String>,
    /// Copy the contents of this file into the ticket as description.md.
    #[arg(long = "body-file")]
    pub body_file: Option<PathBuf>,
    /// Place the ticket in this scan root (defaults to first registered root).
    #[arg(long = "root")]
    pub target_root: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct IdArgs {
    #[arg(long)]
    pub id: Uuid,
}

#[derive(Debug, Args)]
pub struct StatusArgs {
    /// Optional prefix filter — only include tickets whose title starts with this string.
    /// E.g. "[bootstrap]" to scope the view to the bootstrap track.
    #[arg(long)]
    pub filter: Option<String>,
    /// Include blocked tickets in the output (default: omitted for brevity).
    #[arg(long, default_value_t = false)]
    pub show_blocked: bool,
}

#[derive(Debug, Args)]
pub struct ReadyOverviewArgs {
    /// Optional prefix filter — only include tickets whose title starts with this string.
    #[arg(long)]
    pub filter: Option<String>,
    /// Optional scope label included in the JSON response.
    #[arg(long)]
    pub scope: Option<String>,
}

#[derive(Debug, Args)]
pub struct WatchArgs {
    /// Debounce time in milliseconds before triggering reconcile after an event.
    #[arg(long, default_value = "200")]
    pub debounce_ms: u64,
}

#[derive(Debug, Args)]
pub struct ServeCliArgs {
    /// TCP port to bind to.
    #[arg(long, default_value = "8080")]
    pub port: u16,
    /// Host address to bind to.
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
    /// Serve a specific named workspace only (default: all registered).
    #[arg(long)]
    pub workspace: Option<String>,
}

#[derive(Debug, Args)]
pub struct LinkArgs {
    /// UUID of the source ticket.
    #[arg(long)]
    pub from: Uuid,
    /// UUID of the target ticket.
    #[arg(long)]
    pub to: Uuid,
    /// Edge kind (e.g. depends_on, blocks, linked).
    #[arg(long)]
    pub kind: String,
    /// Human-readable reason for this edge (optional, stored in response only).
    #[arg(long)]
    pub reason: Option<String>,
}

#[derive(Debug, Args)]
pub struct UnlinkArgs {
    /// UUID of the source ticket.
    #[arg(long)]
    pub from: Uuid,
    /// UUID of the target ticket.
    #[arg(long)]
    pub to: Uuid,
    /// Edge kind (e.g. depends_on, blocks, linked).
    #[arg(long)]
    pub kind: String,
    /// Human-readable reason for this removal (optional, stored in response only).
    #[arg(long)]
    pub reason: Option<String>,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    #[arg(long)]
    pub id: Uuid,
    #[arg(long = "from-state")]
    pub from_state: Option<String>,
    #[arg(long = "to-state")]
    pub to_state: Option<String>,
    #[arg(long = "field")]
    pub fields: Vec<String>,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    #[arg(long)]
    pub state: Option<String>,
    #[arg(long = "type")]
    pub ticket_type: Option<String>,
    #[arg(long)]
    pub limit: Option<usize>,
}

#[derive(Debug, Args)]
pub struct ScanArgs {
    #[arg(long = "reindex")]
    pub reindex: bool,
}

#[derive(Debug, Args)]
pub struct ClaimArgs {
    #[arg(long)]
    pub id: Uuid,
    #[arg(long = "agent")]
    pub agent_id: String,
    #[arg(long = "ttl-secs", default_value_t = 300)]
    pub ttl_secs: u64,
    #[arg(long = "intent")]
    pub work_intent: Option<String>,
}

#[derive(Debug, Args)]
pub struct UnclaimArgs {
    #[arg(long)]
    pub id: Uuid,
    #[arg(long)]
    pub reason: Option<String>,
}

#[derive(Debug, Args)]
pub struct TextArgs {
    pub expression: String,
    #[arg(long, default_value_t = 20)]
    pub limit: usize,
}

#[derive(Debug, Args)]
pub struct AddRootArgs {
    pub path: PathBuf,
    #[arg(long, default_value = "default")]
    pub label: String,
}

#[derive(Debug, Args)]
pub struct HistoryArgs {
    #[arg(long)]
    pub id: Uuid,
    #[arg(long, default_value_t = 20)]
    pub limit: usize,
}

#[derive(Debug, Args)]
pub struct DiffArgs {
    #[arg(long)]
    pub id: Uuid,
    #[arg(long)]
    pub from: String,
    #[arg(long)]
    pub to: String,
}

#[derive(Debug, Args)]
pub struct RevertArgs {
    #[arg(long)]
    pub id: Uuid,
    #[arg(long = "to")]
    pub to_sha: String,
}

#[derive(Debug, Args)]
pub struct FinalizeMergeArgs {
    #[arg(long)]
    pub id: Uuid,
    #[arg(long = "merge-commit")]
    pub merge_commit: String,
}

#[derive(Debug, Args)]
pub struct WorkspaceArgs {
    #[command(subcommand)]
    pub command: WorkspaceSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum WorkspaceSubCommand {
    /// List all registered workspaces.
    List,
    /// Register a new named workspace.
    New(WorkspaceNewArgs),
    /// Set the active workspace by name.
    Use(WorkspaceUseArgs),
    /// Show the currently active workspace and how it was resolved.
    Current,
    /// Unregister a workspace (data on disk is not removed).
    Remove(WorkspaceRemoveArgs),
}

#[derive(Debug, Args)]
pub struct WorkspaceNewArgs {
    /// Name for the new workspace.
    pub name: String,
    /// Index root path (defaults to ~/.ticket-<name>/).
    #[arg(long)]
    pub path: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct WorkspaceUseArgs {
    /// Name of the workspace to activate.
    pub name: String,
    /// Write a .ticket-workspace file in the current directory instead of
    /// updating the global active pointer.
    #[arg(long)]
    pub local: bool,
}

#[derive(Debug, Args)]
pub struct WorkspaceRemoveArgs {
    /// Name of the workspace to unregister.
    pub name: String,
}

#[derive(Debug, Args)]
pub struct ExecArgs {
    /// Execute multiple commands from stdin, one JSON object per line, as a
    /// single transaction. Rolls back all on first failure.
    #[arg(long)]
    pub batch: bool,
}

#[derive(Debug, Args)]
pub struct BatchArgs {
    /// Optional NDJSON file path (one JSON object per line). If omitted, read stdin.
    #[arg(long)]
    pub file: Option<PathBuf>,
}

// ── error type ─────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum CliRunError {
    #[error("invalid field patch: {0}")]
    InvalidFieldPatch(String),
    #[error("failed to serialize command schema: {0}")]
    CommandSchema(#[from] serde_json::Error),
    #[error("storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("index root required: pass --index-root or set TICKET_INDEX_ROOT env var")]
    IndexRootRequired,
    #[error("invalid exec command payload: {0}")]
    InvalidExecPayload(String),
    #[error("{0}")]
    BadRequest(String),
}

pub enum CliOutput {
    Json(Value),
    Text(String),
}

// ── entry point ────────────────────────────────────────────────────────────────

pub fn run(cli: TicketCli) -> Result<CliOutput, CliRunError> {
    let payload = dispatch(cli.command, cli.index_root.as_deref(), cli.schema_dir.as_deref(), cli.json)?;
    if cli.json {
        let request_id = cli.request_id.unwrap_or_else(|| Uuid::new_v4().to_string());
        let envelope = CommandEnvelope { request_id, payload };
        Ok(CliOutput::Json(json!(envelope)))
    } else {
        Ok(CliOutput::Text(render_human(payload)))
    }
}

fn dispatch(
    command: TicketCommandCli,
    index_root_override: Option<&std::path::Path>,
    schema_dir_override: Option<&std::path::Path>,
    _as_json: bool,
) -> Result<Value, CliRunError> {
    // Commands that don't need storage.
    match &command {
        TicketCommandCli::ExportCommandSchema => {
            let schema_json = export_command_schema_json()?;
            let schema: Value = serde_json::from_str(&schema_json)?;
            return Ok(json!({
                "command": "export_command_schema",
                "status": "ok",
                "schema": schema,
                "known_commands": export_command_schema().commands,
            }));
        }
        TicketCommandCli::Workspace(_) => {}
        _ => {}
    }
    if let TicketCommandCli::Workspace(args) = command {
        return Ok(cmd_workspace(args));
    }

    // The agent exec protocol requires an explicit index root to prevent
    // silent fallback to a user-specific workspace, which could send writes
    // to the wrong store.  Reject exec/watch if neither --index-root nor
    // TICKET_INDEX_ROOT is provided.
    let has_explicit_root = index_root_override.is_some()
        || std::env::var("TICKET_INDEX_ROOT").is_ok();
    if !has_explicit_root {
        if matches!(command, TicketCommandCli::Exec(_) | TicketCommandCli::Batch(_)) {
            return Err(CliRunError::IndexRootRequired);
        }
    }

    // All other commands need the store.
    let index_root = resolve_index_root(index_root_override)?;
    let mut registry = SchemaRegistry::with_builtins();
    if let Some(schema_dir) = schema_dir_override {
        registry.load_dir(schema_dir)?;
    }
    let store = TicketStore::open_with(&index_root, registry)?;

    match command {
        TicketCommandCli::Create(args) => cmd_create(args, &store),
        TicketCommandCli::Get(args) => cmd_get(args, &store),
        TicketCommandCli::Update(args) => cmd_update(args, &store),
        TicketCommandCli::List(args) => cmd_list(args, &store),
        TicketCommandCli::Delete(args) => cmd_delete(args, &store),
        TicketCommandCli::Scan(args) => cmd_scan(args, &store),
        TicketCommandCli::Claim(args) => cmd_claim(args, &store),
        TicketCommandCli::Unclaim(args) => cmd_unclaim(args, &store),
        TicketCommandCli::Leases => cmd_leases(&store),
        TicketCommandCli::Search(args) => cmd_search(args, &store),
        TicketCommandCli::Query(args) => cmd_search(args, &store),
        TicketCommandCli::AddRoot(args) => cmd_add_root(args, &store),
        TicketCommandCli::Exec(args) => cmd_exec(args, &store),
        TicketCommandCli::Batch(args) => cmd_batch(args, &store),
        TicketCommandCli::History(args) => cmd_history(args, &store),
        TicketCommandCli::Diff(args) => cmd_diff(args, &store),
        TicketCommandCli::Revert(args) => cmd_revert(args, &store),
        TicketCommandCli::FinalizeMerge(args) => Ok(json!({
            "command": "finalize_merge",
            "status": "phase2_stub",
            "id": args.id,
            "merge_commit": args.merge_commit
        })),
        TicketCommandCli::Link(args) => cmd_link(args, &store),
        TicketCommandCli::Unlink(args) => cmd_unlink(args, &store),
        TicketCommandCli::Links(args) => cmd_links(args, &store),
        TicketCommandCli::Watch(args) => cmd_watch(args, &store),
        TicketCommandCli::Status(args) => cmd_status(args, &store),
        TicketCommandCli::ReadyOverview(args) => cmd_ready_overview(args, &store),
        TicketCommandCli::Serve(args) => cmd_serve(args, store),
        TicketCommandCli::ExportCommandSchema => unreachable!("handled above"),
        TicketCommandCli::Workspace(_) => unreachable!("handled above"),
    }
}

// ── command handlers ───────────────────────────────────────────────────────────

fn cmd_create(args: CreateArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let type_id = args.ticket_type.as_deref().unwrap_or("tracker-improvement");
    let extra = parse_fields_to_json(&args.fields)?;
    let target_root = args.target_root.as_deref();

    let body = args.body_file
        .map(|p| std::fs::read_to_string(&p)
            .map_err(|e| CliRunError::InvalidFieldPatch(format!("cannot read body-file: {e}"))))
        .transpose()?;

    let id = store.create(
        args.id,
        type_id,
        args.title.as_deref(),
        args.state.as_deref(),
        extra,
        target_root,
        body.as_deref(),
    )?;

    Ok(json!({
        "command": "create",
        "status": "ok",
        "id": id,
        "type": type_id,
    }))
}

fn cmd_get(args: IdArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let manifest = store.get(&args.id)?;
    Ok(json!({
        "command": "get",
        "status": "ok",
        "ticket": {
            "id": manifest.id,
            "created_at": manifest.created_at,
            "fields": manifest.extra,
        }
    }))
}

fn cmd_update(args: UpdateArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let patch = parse_fields_to_json(&args.fields)?;
    let manifest = store.update(
        &args.id,
        patch,
        args.from_state.as_deref(),
        args.to_state.as_deref(),
    )?;
    Ok(json!({
        "command": "update",
        "status": "ok",
        "ticket": {
            "id": manifest.id,
            "fields": manifest.extra,
        }
    }))
}

fn cmd_list(args: ListArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let items = store.list(
        args.state.as_deref(),
        args.ticket_type.as_deref(),
        args.limit,
    )?;
    let items_json: Vec<Value> = items
        .iter()
        .map(|t| {
            json!({
                "id": t.id,
                "type": t.type_id,
                "title": t.title,
                "state": t.state,
                "updated_at": t.updated_at,
            })
        })
        .collect();
    Ok(json!({
        "command": "list",
        "status": "ok",
        "count": items_json.len(),
        "items": items_json,
    }))
}

fn cmd_delete(args: IdArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    store.delete(&args.id)?;
    Ok(json!({
        "command": "delete",
        "status": "ok",
        "id": args.id,
    }))
}

fn cmd_link(args: LinkArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let edge = EdgeRecord {
        from: args.from,
        to: args.to,
        kind: args.kind.clone(),
        created_at: Utc::now(),
    };
    store.add_edge(edge)?;
    Ok(json!({
        "command": "link",
        "status": "ok",
        "from": args.from,
        "to": args.to,
        "kind": args.kind,
        "reason": args.reason,
    }))
}

fn cmd_unlink(args: UnlinkArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let edge = EdgeRecord {
        from: args.from,
        to: args.to,
        kind: args.kind.clone(),
        created_at: Utc::now(),
    };
    store.remove_edge(edge)?;
    Ok(json!({
        "command": "unlink",
        "status": "ok",
        "from": args.from,
        "to": args.to,
        "kind": args.kind,
        "reason": args.reason,
    }))
}

fn cmd_links(args: IdArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let edges = store.edges_from(&args.id)?;
    let items: Vec<Value> = edges
        .iter()
        .map(|e| json!({ "from": e.from, "to": e.to, "kind": e.kind }))
        .collect();
    Ok(json!({
        "command": "links",
        "status": "ok",
        "id": args.id,
        "count": items.len(),
        "edges": items,
    }))
}

fn cmd_scan(args: ScanArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let report = store.scan(args.reindex)?;
    let diags: Vec<Value> = report
        .diagnostics
        .iter()
        .map(|d| json!({ "path": d.path, "reason": d.reason }))
        .collect();
    Ok(json!({
        "command": "scan",
        "status": "ok",
        "integrated": report.integrated,
        "diagnostics": diags,
    }))
}

fn cmd_claim(args: ClaimArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let lease = store.claim(
        &args.id,
        &args.agent_id,
        args.ttl_secs,
        args.work_intent.as_deref(),
    )?;
    Ok(json!({
        "command": "claim",
        "status": "ok",
        "ticket_id": lease.ticket_id,
        "working_by": lease.working_by,
        "expires_at": lease.lease_expires_at,
    }))
}

fn cmd_unclaim(args: UnclaimArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    store.unclaim(&args.id)?;
    Ok(json!({
        "command": "unclaim",
        "status": "ok",
        "id": args.id,
        "reason": args.reason,
    }))
}

fn cmd_leases(store: &TicketStore) -> Result<Value, CliRunError> {
    let leases = store.list_leases()?;
    let items: Vec<Value> = leases
        .iter()
        .map(|l| {
            json!({
                "ticket_id": l.ticket_id,
                "working_by": l.working_by,
                "expires_at": l.lease_expires_at,
                "expired": l.is_expired(),
                "intent": l.work_intent,
            })
        })
        .collect();
    Ok(json!({
        "command": "leases",
        "status": "ok",
        "count": items.len(),
        "leases": items,
    }))
}

fn cmd_search(args: TextArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let results = store.search_tickets(&args.expression, args.limit)?;
    let items: Vec<Value> = results
        .iter()
        .map(|r| {
            json!({
                "id": r.id,
                "title": r.title,
                "state": r.state,
                "type": r.ticket_type,
                "snippet": r.snippet,
                "score": r.score,
            })
        })
        .collect();
    Ok(json!({
        "command": "search",
        "status": "ok",
        "query": args.expression,
        "count": items.len(),
        "results": items,
    }))
}

fn cmd_add_root(args: AddRootArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    use crate::model::filesystem::ScanRoot;
    let path = args.path.canonicalize().unwrap_or(args.path.clone());
    std::fs::create_dir_all(&path).map_err(StorageError::Io)?;
    store.add_scan_root(ScanRoot { path: path.clone(), label: args.label.clone() })?;
    Ok(json!({
        "command": "add_root",
        "status": "ok",
        "path": path,
        "label": args.label,
    }))
}



fn cmd_serve(args: ServeCliArgs, store: TicketStore) -> Result<Value, CliRunError> {
    use crate::serve::{ServeConfig, WorkspaceRegistry, serve};
    use crate::workspace::WorkspaceConfig;

    // Build workspace registry.
    //
    // IMPORTANT: `store` already holds the redb lock for `index_root`.  We must
    // pre-populate the registry with this open instance so that the lazy-open
    // path in `WorkspaceRegistry::get()` is never reached for this workspace.
    // Attempting a second open of the same redb file would fail (redb does not
    // allow concurrent opens from the same process).
    let registry = if args.workspace.is_some() {
        WorkspaceRegistry::single_opened(std::sync::Arc::new(store))
    } else {
        let config = WorkspaceConfig::load();
        if config.workspaces.is_empty() {
            WorkspaceRegistry::single_opened(std::sync::Arc::new(store))
        } else {
            // Multi-workspace config: the named workspaces open lazily.
            // The current `store` (default workspace) is not in this registry;
            // those workspaces have their own paths.
            WorkspaceRegistry::from_config(&config)
        }
    };

    let config = ServeConfig {
        host: args.host,
        port: args.port,
    };

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| CliRunError::BadRequest(format!("failed to start tokio runtime: {e}")))?;

    rt.block_on(async {
        serve(config, registry)
            .await
            .map_err(|e| CliRunError::BadRequest(e.to_string()))
    })?;

    // serve() only returns on error; this is unreachable in the happy path.
    Err(CliRunError::BadRequest("server exited unexpectedly".into()))
}

fn cmd_watch(args: WatchArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    use crate::watcher::reconciler::{run_watch_loop, start_watcher};
    eprintln!("Starting filesystem watcher (debounce={}ms). Press Ctrl+C to stop.", args.debounce_ms);
    let handle = start_watcher(store)
        .map_err(|e| CliRunError::Storage(e))?;
    run_watch_loop(&handle, store, args.debounce_ms);
    // run_watch_loop blocks; this line is unreachable but satisfies the return type.
    Ok(json!({ "command": "watch", "status": "stopped" }))
}

// ── status dashboard ──────────────────────────────────────────────────────────

/// The "done" bucket: states that count as work completed.
const DONE_STATES: &[&str] = &["done", "cancelled"];
/// States that represent active in-flight work.
const ACTIVE_STATES: &[&str] = &["in-progress", "review", "validating", "validated",
                                   "release-candidate", "monitoring"];

fn cmd_status(args: StatusArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    use std::collections::{HashMap, HashSet};

    // 1. Load all non-deleted tickets.
    let all = store.list(None, None, None)?;

    // 2. Apply optional title-prefix filter.
    let tickets: Vec<_> = if let Some(ref prefix) = args.filter {
        all.into_iter()
            .filter(|t| t.title.as_deref().unwrap_or("").starts_with(prefix.as_str()))
            .collect()
    } else {
        all
    };

    // 3. Build a set of done ticket IDs (quick lookup for dep resolution).
    let done_ids: HashSet<Uuid> = tickets
        .iter()
        .filter(|t| {
            t.state.as_deref().map(|s| DONE_STATES.contains(&s)).unwrap_or(false)
        })
        .map(|t| t.id)
        .collect();

    // 4. Load all edges and index `depends_on` edges by their `from` ticket.
    let all_edges = store.list_all_edges()?;
    let mut blockers: HashMap<Uuid, Vec<Uuid>> = HashMap::new(); // ticket → [unresolved dep ids]
    for edge in &all_edges {
        if edge.kind == "depends_on" {
            // Only count it as a blocker if the dependency is NOT done.
            if !done_ids.contains(&edge.to) {
                blockers.entry(edge.from).or_default().push(edge.to);
            }
        }
    }

    // 5. Bucket each ticket.
    let mut active = Vec::new();
    let mut ready = Vec::new();
    let mut blocked_list = Vec::new();
    let mut done_count = 0usize;
    let mut total = 0usize;

    for t in &tickets {
        total += 1;
        let state = t.state.as_deref().unwrap_or("open");

        if DONE_STATES.contains(&state) {
            done_count += 1;
            continue;
        }

        let is_active = ACTIVE_STATES.contains(&state);
        let unresolved = blockers.get(&t.id).cloned().unwrap_or_default();
        let is_blocked = !unresolved.is_empty();

        let entry = json!({
            "id": t.id,
            "title": t.title,
            "state": state,
            "component": t.type_id,  // or pull from extra
        });

        if is_active {
            active.push(entry);
        } else if is_blocked {
            if args.show_blocked {
                let dep_entries: Vec<Value> = unresolved
                    .iter()
                    .map(|dep_id| {
                        // Try to find the blocker title.
                        let title = tickets.iter()
                            .find(|t| t.id == *dep_id)
                            .and_then(|t| t.title.clone())
                            .unwrap_or_else(|| dep_id.to_string());
                        let dep_state = tickets.iter()
                            .find(|t| t.id == *dep_id)
                            .and_then(|t| t.state.clone())
                            .unwrap_or_else(|| "unknown".to_string());
                        json!({ "id": dep_id, "title": title, "state": dep_state })
                    })
                    .collect();
                blocked_list.push(json!({
                    "id": t.id,
                    "title": t.title,
                    "state": state,
                    "waiting_on": dep_entries
                }));
            }
        } else {
            // open with no unresolved deps → ready
            ready.push(entry);
        }
    }

    // 6. Build parallel opportunity groups: ready tickets that share no
    //    dependency edges between each other are safe to execute in parallel.
    //    We group by component (if available in extra fields) as a hint to
    //    coordinators; tickets in different groups can all start at once.
    //
    //    Simple strategy: group by type_id (component) first, then note that
    //    tickets in *different* groups are independent by definition.
    let mut by_component: HashMap<String, Vec<&Value>> = HashMap::new();
    for entry in &ready {
        let comp = entry["component"].as_str().unwrap_or("unknown").to_string();
        by_component.entry(comp).or_default().push(entry);
    }

    let parallel_groups: Vec<Value> = by_component
        .into_iter()
        .map(|(component, entries)| json!({
            "component": component,
            "count": entries.len(),
            "tickets": entries
        }))
        .collect();

    Ok(json!({
        "command": "status",
        "status": "ok",
        "summary": {
            "total": total,
            "done": done_count,
            "active": active.len(),
            "ready": ready.len(),
            "blocked": if args.show_blocked { blocked_list.len() } else { total - done_count - active.len() - ready.len() }
        },
        "active": active,
        "ready": ready,
        "blocked": blocked_list,
        "parallel_groups": parallel_groups
    }))
}

fn cmd_ready_overview(args: ReadyOverviewArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let status_payload = cmd_status(
        StatusArgs {
            filter: args.filter.clone(),
            show_blocked: true,
        },
        store,
    )?;

    let scope = args
        .scope
        .unwrap_or_else(|| "ready tickets currently open in the active index".to_string());

    Ok(json!({
        "command": "ready_overview",
        "status": "ok",
        "date": Utc::now().format("%Y-%m-%d").to_string(),
        "scope": scope,
        "summary": status_payload["summary"],
        "ready": status_payload["ready"],
        "ready_count": status_payload["summary"]["ready"],
    }))
}

// ── batch exec undo infrastructure ───────────────────────────────────────────

/// Records enough information to undo a single batch command on rollback.
#[derive(Debug)]
enum BatchUndoOp {
    /// Created a ticket — undo by soft-deleting it.
    Delete { id: Uuid },
    /// Updated a ticket — undo by restoring the saved manifest state.
    RestoreUpdate {
        id: Uuid,
        saved_extra: BTreeMap<String, Value>,
        saved_state: Option<String>,
    },
    /// Added a graph edge — undo by removing it.
    RemoveEdge {
        from: Uuid,
        to: Uuid,
        kind: String,
    },
}

/// Pre-capture undo state for a command *before* it is executed.
fn batch_pre_capture(cmd: &Value, store: &TicketStore) -> Option<(String, Uuid, BTreeMap<String, Value>, Option<String>)> {
    let op = cmd.get("command").and_then(|v| v.as_str())?;
    let op = op.strip_prefix("task_").unwrap_or(op);
    match op {
        "update" => {
            let id: Uuid = cmd.get("id").and_then(|v| v.as_str())?.parse().ok()?;
            if let Ok(Some(indexed)) = store.get_indexed(&id) {
                // Save redb state as a minimal BTreeMap for restore.
                let mut saved = BTreeMap::new();
                if let Some(t) = &indexed.title { saved.insert("title".to_string(), serde_json::Value::String(t.clone())); }
                Some(("update".to_string(), id, saved, indexed.state.clone()))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Construct a `BatchUndoOp` from the *result* of a successfully executed command.
fn batch_post_undo(
    result: &Value,
    pre_capture: Option<(String, Uuid, BTreeMap<String, Value>, Option<String>)>,
) -> Option<BatchUndoOp> {
    let cmd = result.get("command").and_then(|v| v.as_str())?;
    match cmd {
        "create" => {
            let id: Uuid = result.get("id").and_then(|v| v.as_str())?.parse().ok()?;
            Some(BatchUndoOp::Delete { id })
        }
        "update" => {
            let (_, id, saved_extra, saved_state) = pre_capture?;
            Some(BatchUndoOp::RestoreUpdate { id, saved_extra, saved_state })
        }
        "link" => {
            let from: Uuid = result.get("from").and_then(|v| v.as_str())?.parse().ok()?;
            let to: Uuid = result.get("to").and_then(|v| v.as_str())?.parse().ok()?;
            let kind = result.get("kind").and_then(|v| v.as_str())?.to_string();
            Some(BatchUndoOp::RemoveEdge { from, to, kind })
        }
        _ => None,
    }
}

/// Apply a single undo operation. Errors are collected, not propagated.
fn apply_batch_undo(undo: BatchUndoOp, store: &TicketStore, errors: &mut Vec<String>) {
    match undo {
        BatchUndoOp::Delete { id } => {
            if let Err(e) = store.delete(&id) {
                errors.push(format!("rollback delete {id}: {e}"));
            }
        }
        BatchUndoOp::RestoreUpdate { id, saved_extra, saved_state } => {
            if let Err(e) = store.force_restore(&id, saved_extra, saved_state) {
                errors.push(format!("rollback restore {id}: {e}"));
            }
        }
        BatchUndoOp::RemoveEdge { from, to, kind } => {
            let edge = EdgeRecord { from, to, kind, created_at: Utc::now() };
            if let Err(e) = store.remove_edge(edge) {
                errors.push(format!("rollback remove_edge {from}->{to}: {e}"));
            }
        }
    }
}

// ── history / diff / revert ───────────────────────────────────────────────────

fn cmd_history(args: HistoryArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let mut revisions = store.get_history(&args.id)?;
    // Most-recent first; apply limit.
    revisions.reverse();
    revisions.truncate(args.limit);
    let entries: Vec<Value> = revisions
        .into_iter()
        .map(|r| json!({ "rev": r.rev, "ts": r.ts, "fields": r.fields }))
        .collect();
    Ok(json!({
        "command": "history",
        "status": "ok",
        "id": args.id,
        "count": entries.len(),
        "entries": entries
    }))
}

/// Parse a revision specifier: an integer string or the keyword "latest".
/// Returns `None` if the string is not a valid specifier.
fn parse_rev_spec(spec: &str, max_rev: u64) -> Option<u64> {
    if spec == "latest" {
        return Some(max_rev);
    }
    spec.parse::<u64>().ok()
}

fn cmd_diff(args: DiffArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let revisions = store.get_history(&args.id)?;
    if revisions.is_empty() {
        return Err(CliRunError::BadRequest("no history available for this ticket".into()));
    }
    let max_rev = revisions.last().map(|r| r.rev).unwrap_or(0);
    let from_rev = parse_rev_spec(&args.from, max_rev)
        .ok_or_else(|| CliRunError::BadRequest(format!("invalid revision specifier: {}", args.from)))?;
    let to_rev = parse_rev_spec(&args.to, max_rev)
        .ok_or_else(|| CliRunError::BadRequest(format!("invalid revision specifier: {}", args.to)))?;

    let find_rev = |n: u64| revisions.iter().find(|r| r.rev == n).cloned();
    let from = find_rev(from_rev)
        .ok_or_else(|| CliRunError::BadRequest(format!("revision {} not found", from_rev)))?;
    let to = find_rev(to_rev)
        .ok_or_else(|| CliRunError::BadRequest(format!("revision {} not found", to_rev)))?;

    // Build diff: added, removed, changed.
    let mut added: BTreeMap<&str, &Value> = BTreeMap::new();
    let mut removed: BTreeMap<&str, &Value> = BTreeMap::new();
    let mut changed: BTreeMap<&str, (&Value, &Value)> = BTreeMap::new();

    for (k, v) in &to.fields {
        match from.fields.get(k) {
            None => { added.insert(k.as_str(), v); }
            Some(old) if old != v => { changed.insert(k.as_str(), (old, v)); }
            _ => {}
        }
    }
    for (k, v) in &from.fields {
        if !to.fields.contains_key(k) {
            removed.insert(k.as_str(), v);
        }
    }

    let changed_json: serde_json::Map<String, Value> = changed
        .into_iter()
        .map(|(k, (old, new))| (k.to_string(), json!({ "from": old, "to": new })))
        .collect();

    Ok(json!({
        "command": "diff",
        "status": "ok",
        "id": args.id,
        "from_rev": from_rev,
        "to_rev": to_rev,
        "added": added,
        "removed": removed,
        "changed": changed_json
    }))
}

fn cmd_revert(args: RevertArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let revisions = store.get_history(&args.id)?;
    if revisions.is_empty() {
        return Err(CliRunError::BadRequest("no history available for this ticket".into()));
    }
    let max_rev = revisions.last().map(|r| r.rev).unwrap_or(0);
    let target_rev = parse_rev_spec(&args.to_sha, max_rev)
        .ok_or_else(|| CliRunError::BadRequest(format!("invalid revision specifier: {}", args.to_sha)))?;
    let snapshot = revisions
        .iter()
        .find(|r| r.rev == target_rev)
        .cloned()
        .ok_or_else(|| CliRunError::BadRequest(format!("revision {} not found", target_rev)))?;

    // Revert = forward-only: apply snapshot fields as a new revision, bypassing
    // state-machine validation (we're going backwards in state, which would
    // otherwise be rejected).
    let new_rev = store.apply_revert(&args.id, snapshot.fields)?;
    let updated = store.get(&args.id)?;

    Ok(json!({
        "command": "revert",
        "status": "ok",
        "id": args.id,
        "reverted_to": target_rev,
        "new_rev": new_rev,
        "ticket": { "fields": updated.extra }
    }))
}

fn read_batch_commands<R: std::io::BufRead>(reader: R) -> Result<Vec<Value>, CliRunError> {
    let mut commands: Vec<Value> = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(StorageError::Io)?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let cmd: Value = serde_json::from_str(line)
            .map_err(|e| CliRunError::InvalidExecPayload(e.to_string()))?;
        commands.push(cmd);
    }
    Ok(commands)
}

fn execute_batch_commands(commands: &[Value], store: &TicketStore) -> Result<Value, CliRunError> {
    // Each successfully executed command records an undo operation so the
    // batch can be rolled back atomically on the first failure.
    let mut results: Vec<Value> = Vec::with_capacity(commands.len());
    let mut undo_stack: Vec<BatchUndoOp> = Vec::with_capacity(commands.len());

    for cmd in commands {
        // Pre-capture undo information BEFORE executing.
        let undo_hint = batch_pre_capture(cmd, store);

        match exec_single_command(cmd, store) {
            Ok(result) => {
                // Record undo op based on what the command did.
                if let Some(undo) = batch_post_undo(&result, undo_hint) {
                    undo_stack.push(undo);
                }
                results.push(result);
            }
            Err(e) => {
                // Attempt best-effort rollback of all completed commands.
                let mut rollback_errors: Vec<String> = Vec::new();
                for undo in undo_stack.into_iter().rev() {
                    apply_batch_undo(undo, store, &mut rollback_errors);
                }
                return Ok(json!({
                    "command": "exec_batch",
                    "status": "error",
                    "completed": results.len(),
                    "total": commands.len(),
                    "error": e.to_string(),
                    "rolled_back": rollback_errors.is_empty(),
                    "rollback_errors": rollback_errors,
                }));
            }
        }
    }
    Ok(json!({
        "command": "exec_batch",
        "status": "ok",
        "count": results.len(),
        "results": results,
    }))
}

fn cmd_batch(args: BatchArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    use std::fs::File;
    use std::io::{self, BufReader};

    let commands = if let Some(path) = args.file {
        let file = File::open(&path)
            .map_err(|e| CliRunError::InvalidExecPayload(format!("cannot open batch file {}: {e}", path.display())))?;
        read_batch_commands(BufReader::new(file))?
    } else {
        let stdin = io::stdin();
        read_batch_commands(stdin.lock())?
    };

    execute_batch_commands(&commands, store)
}

/// `ticket exec` — read one JSON `TaskCommand` object from stdin and execute it.
/// In `--batch` mode, read one object per line until EOF and execute all atomically
/// (rolling back all on the first failure).
fn cmd_exec(args: ExecArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    use std::io::{self, Read};

    if args.batch {
        let stdin = io::stdin();
        let commands = read_batch_commands(stdin.lock())?;
        execute_batch_commands(&commands, store)
    } else {
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.lock().read_to_string(&mut input).map_err(StorageError::Io)?;
        let cmd: Value = serde_json::from_str(input.trim())
            .map_err(|e| CliRunError::InvalidExecPayload(e.to_string()))?;
        exec_single_command(&cmd, store)
    }
}

fn exec_single_command(cmd: &Value, store: &TicketStore) -> Result<Value, CliRunError> {
    let raw_op = cmd
        .get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| CliRunError::InvalidExecPayload("missing 'command' field".to_string()))?;

    // Normalize task_ prefix so both "create" and "task_create" route identically.
    let op = raw_op.strip_prefix("task_").unwrap_or(raw_op);

    match op {
        "create" => {
            let type_id = cmd.get("type").and_then(|v| v.as_str()).unwrap_or("tracker-improvement");
            let title = cmd.get("title").and_then(|v| v.as_str());
            let state = cmd.get("state").and_then(|v| v.as_str());
            let id: Option<Uuid> = cmd
                .get("id")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok());
            let extra: BTreeMap<String, Value> = cmd
                .get("fields")
                .and_then(|v| v.as_object())
                .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default();
            let created_id = store.create(id, type_id, title, state, extra, None, None)?;
            Ok(json!({ "command": "create", "status": "ok", "id": created_id }))
        }
        "get" => {
            let id = parse_uuid_field(cmd, "id")?;
            let manifest = store.get(&id)?;
            Ok(json!({
                "command": "get",
                "status": "ok",
                "ticket": { "id": manifest.id, "created_at": manifest.created_at, "fields": manifest.extra }
            }))
        }
        "update" => {
            let id = parse_uuid_field(cmd, "id")?;
            let patch: BTreeMap<String, Value> = cmd
                .get("patch")
                .and_then(|v| v.as_object())
                .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default();
            let from_state = cmd.get("from_state").and_then(|v| v.as_str());
            let to_state = cmd.get("to_state").and_then(|v| v.as_str());
            let manifest = store.update(&id, patch, from_state, to_state)?;
            Ok(json!({ "command": "update", "status": "ok", "ticket": { "id": manifest.id, "fields": manifest.extra } }))
        }
        "list" => {
            let state_filter = cmd.get("state").and_then(|v| v.as_str());
            let type_filter = cmd.get("type").and_then(|v| v.as_str());
            let limit = cmd.get("limit").and_then(|v| v.as_u64()).map(|n| n as usize);
            let items = store.list(state_filter, type_filter, limit)?;
            let items_json: Vec<Value> = items.iter().map(|t| json!({
                "id": t.id, "type": t.type_id, "title": t.title, "state": t.state,
            })).collect();
            Ok(json!({ "command": "list", "status": "ok", "count": items_json.len(), "items": items_json }))
        }
        "delete" => {
            let id = parse_uuid_field(cmd, "id")?;
            store.delete(&id)?;
            Ok(json!({ "command": "delete", "status": "ok", "id": id }))
        }
        "link" => {
            let from = parse_uuid_field(cmd, "from")?;
            let to = parse_uuid_field(cmd, "to")?;
            let kind = req_str(cmd, "kind")?.to_string();
            let reason = cmd.get("reason").and_then(|v| v.as_str()).map(|s| s.to_string());

            store.add_edge(EdgeRecord {
                from,
                to,
                kind: kind.clone(),
                created_at: Utc::now(),
            })?;

            Ok(json!({
                "command": "link",
                "status": "ok",
                "from": from,
                "to": to,
                "kind": kind,
                "reason": reason,
            }))
        }
        "links" => {
            let id = parse_uuid_field(cmd, "id")?;
            let edges = store.edges_from(&id)?;
            let items: Vec<Value> = edges.iter().map(|e| json!({
                "from": e.from,
                "to": e.to,
                "kind": e.kind,
            })).collect();
            Ok(json!({
                "command": "links",
                "status": "ok",
                "id": id,
                "count": items.len(),
                "edges": items,
            }))
        }
        "search" => {
            let expr = cmd.get("query").and_then(|v| v.as_str())
                .ok_or_else(|| CliRunError::InvalidExecPayload("missing 'query' field".to_string()))?;
            let limit = cmd.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as usize;
            let results = store.search_tickets(expr, limit)?;
            let items: Vec<Value> = results.iter().map(|r| json!({
                "id": r.id, "title": r.title, "state": r.state, "snippet": r.snippet, "score": r.score,
            })).collect();
            Ok(json!({ "command": "search", "status": "ok", "count": items.len(), "results": items }))
        }
        "assignment_start" => {
            let ticket_id = parse_uuid_field(cmd, "ticket_id")?;
            let assignment_id = req_str(cmd, "assignment_id")?;
            let prompt = req_str(cmd, "prompt")?;
            let simulate = cmd.get("simulate").and_then(|v| v.as_bool()).unwrap_or(false);

            let repo_root = cmd
                .get("repo_root")
                .and_then(|v| v.as_str())
                .map(PathBuf::from)
                .unwrap_or(std::env::current_dir().map_err(StorageError::Io)?);
            let worktrees_root = cmd
                .get("worktrees_root")
                .and_then(|v| v.as_str())
                .map(PathBuf::from)
                .unwrap_or_else(|| repo_root.join(".ticket-worktrees"));
            let base_branch = cmd
                .get("base_branch")
                .and_then(|v| v.as_str())
                .unwrap_or("main")
                .to_string();
            let branch_prefix = cmd
                .get("branch_prefix")
                .and_then(|v| v.as_str())
                .unwrap_or("tickets")
                .to_string();

            let run_request = AssignmentRunRequest {
                ticket_id: ticket_id.to_string(),
                assignment_id: assignment_id.to_string(),
                prompt: prompt.to_string(),
            };
            let run_config = RunnerConfig {
                repo_root,
                worktrees_root,
                base_branch,
                branch_prefix,
            };

            let receipt = if simulate {
                let runner = AssignmentRunner::new(SimulatedProvider, SimulatedSandbox, run_config);
                runner.start_assignment(&run_request)
            } else {
                let provider_cfg = CopilotApiConfig::from_env()
                    .map_err(|e| CliRunError::BadRequest(format!("task_assignment_start config error: {e}")))?;
                let provider = CopilotApiClient::new(provider_cfg)
                    .map_err(|e| CliRunError::BadRequest(format!("task_assignment_start client init error: {e}")))?;
                let runner = AssignmentRunner::new(provider, GitSandboxProvisioner, run_config);
                runner.start_assignment(&run_request)
            }
            .map_err(|e| CliRunError::BadRequest(format!("task_assignment_start failed: {e}")))?;

            Ok(json!({
                "command": "task_assignment_start",
                "status": "ok",
                "ticket_id": ticket_id,
                "assignment_id": assignment_id,
                "run_id": receipt.run_id,
                "run_status": receipt.status,
                "branch": receipt.branch,
                "worktree_path": receipt.worktree_path,
                "simulated": simulate,
            }))
        }
        "validate_start" => {
            let ticket_id = parse_uuid_field(cmd, "ticket_id")?;
            let assignment_id = req_str(cmd, "assignment_id")?;
            let validator_id = req_str(cmd, "validator_id")?;
            let profile = cmd.get("validation_profile").and_then(|v| v.as_str()).unwrap_or("default");
            let checks: Vec<String> = cmd
                .get("required_checks")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
                .unwrap_or_default();
            let manifest = store.validate_start(&ticket_id, assignment_id, validator_id, profile, checks)?;
            Ok(json!({
                "command": "task_validate_start",
                "status": "ok",
                "ticket": {
                    "id": manifest.id,
                    "state": manifest.extra.get("state"),
                    "validation_status": manifest.extra.get("validation_status"),
                    "validator_id": manifest.extra.get("validator_id"),
                    "validation_profile": manifest.extra.get("validation_profile"),
                }
            }))
        }
        "validate_result" => {
            let ticket_id = parse_uuid_field(cmd, "ticket_id")?;
            let assignment_id = req_str(cmd, "assignment_id")?;
            let validator_id = req_str(cmd, "validator_id")?;
            let result = req_str(cmd, "result")?;
            let evidence_refs: Vec<String> = cmd
                .get("evidence_refs")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
                .unwrap_or_default();
            let summary = cmd.get("summary").and_then(|v| v.as_str());
            let bug_links: Vec<uuid::Uuid> = cmd
                .get("bug_links")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str()?.parse().ok()).collect())
                .unwrap_or_default();
            let outcome = store.validate_result(
                &ticket_id,
                assignment_id,
                validator_id,
                result,
                evidence_refs,
                summary,
                bug_links,
            )?;
            Ok(json!({
                "command": "task_validate_result",
                "status": "ok",
                "ticket_id": outcome.ticket_id,
                "state": outcome.state,
                "validation_status": outcome.validation_status,
                "passed": outcome.passed,
            }))
        }
        "release_candidate_create" => {
            let ticket_id = parse_uuid_field(cmd, "ticket_id")?;
            let release_target = req_str(cmd, "release_target")?;
            let assignment_chain: Vec<String> = cmd
                .get("assignment_chain")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
                .unwrap_or_default();
            let manifest = store.release_candidate_create(&ticket_id, release_target, assignment_chain)?;
            Ok(json!({
                "command": "task_release_candidate_create",
                "status": "ok",
                "ticket": {
                    "id": manifest.id,
                    "state": manifest.extra.get("state"),
                    "release_target": manifest.extra.get("release_target"),
                    "assignment_chain": manifest.extra.get("assignment_chain"),
                }
            }))
        }
        "release_gate_check" => {
            let release_target = req_str(cmd, "release_target")?;
            let required_gates: Vec<String> = cmd
                .get("required_gates")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
                .unwrap_or_else(|| vec!["R1".to_string(), "R2".to_string(), "R3".to_string(), "R4".to_string()]);
            let outcome = store.release_gate_check(release_target, &required_gates)?;
            let gates_json: serde_json::Map<String, Value> = outcome
                .gates
                .iter()
                .map(|(k, v)| (k.clone(), json!(matches!(v, GateStatus::Pass))))
                .collect();
            let all_pass = outcome.blocking_reasons.is_empty();
            Ok(json!({
                "command": "task_release_gate_check",
                "status": "ok",
                "release_target": outcome.release_target,
                "all_gates_pass": all_pass,
                "gates": gates_json,
                "blocking_reasons": outcome.blocking_reasons,
            }))
        }
        "release_promote" => {
            let release_target = req_str(cmd, "release_target")?;
            let release_version = req_str(cmd, "release_version")?;
            let merge_commit = req_str(cmd, "merge_commit")?;
            let required_gates: Vec<String> = cmd
                .get("required_gates")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
                .unwrap_or_else(|| vec!["R1".to_string(), "R2".to_string(), "R3".to_string(), "R4".to_string()]);
            let outcome = store.release_promote(release_target, release_version, merge_commit, &required_gates)?;
            Ok(json!({
                "command": "task_release_promote",
                "status": "ok",
                "release_target": outcome.release_target,
                "release_version": outcome.release_version,
                "promoted_ticket_count": outcome.promoted_ticket_count,
                "monitoring_state": outcome.monitoring_state,
            }))
        }
        other => Err(CliRunError::InvalidExecPayload(format!("unknown command: {other}"))),
    }
}

struct SimulatedProvider;

impl SubagentProvider for SimulatedProvider {
    fn start_subagent(
        &self,
        request: &crate::execution::provider::StartSubagentRequest,
    ) -> Result<StartSubagentResponse, ProviderError> {
        Ok(StartSubagentResponse {
            run_id: format!("sim-{}", request.assignment_id),
            status: "started".to_string(),
        })
    }
}

struct SimulatedSandbox;

impl SandboxProvisioner for SimulatedSandbox {
    fn provision(&self, spec: &SandboxSpec) -> Result<SandboxHandle, SandboxError> {
        Ok(SandboxHandle {
            branch_name: spec.branch_name()?,
            worktree_path: spec.worktree_path()?,
        })
    }

    fn cleanup(&self, _spec: &SandboxSpec, _handle: &SandboxHandle) -> Result<(), SandboxError> {
        Ok(())
    }
}

/// Extract a required string field from a JSON exec command payload.
fn req_str<'a>(cmd: &'a Value, field: &str) -> Result<&'a str, CliRunError> {
    cmd.get(field)
        .and_then(|v| v.as_str())
        .ok_or_else(|| CliRunError::InvalidExecPayload(format!("missing required field '{field}'")))
}

// ── utilities ──────────────────────────────────────────────────────────────────

fn resolve_index_root(override_path: Option<&std::path::Path>) -> Result<PathBuf, CliRunError> {
    // Layer 1: explicit --index-root flag
    if let Some(p) = override_path {
        return Ok(p.to_path_buf());
    }
    // Layer 1b: TICKET_INDEX_ROOT env var
    if let Ok(env_val) = std::env::var("TICKET_INDEX_ROOT") {
        return Ok(PathBuf::from(env_val));
    }
    // Layers 2-4: workspace resolution chain (.ticket-workspace → active workspace → default)
    let (path, _source) = workspace::resolve_workspace();
    Ok(path)
}
fn parse_uuid_field(cmd: &Value, field: &str) -> Result<Uuid, CliRunError> {
    cmd.get(field)
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| CliRunError::InvalidExecPayload(format!("missing or invalid '{field}' field")))
}

// ── workspace command handler ─────────────────────────────────────────────────

fn cmd_workspace(args: WorkspaceArgs) -> Value {
    match args.command {
        WorkspaceSubCommand::List => {
            let config = WorkspaceConfig::load();
            let active = config.active.as_deref().unwrap_or("");
            let workspaces: Vec<Value> = config
                .workspaces
                .iter()
                .map(|(name, path)| {
                    json!({
                        "name": name,
                        "path": path,
                        "active": name == active,
                    })
                })
                .collect();
            json!({
                "command": "workspace_list",
                "status": "ok",
                "active": if active.is_empty() { Value::Null } else { Value::String(active.to_string()) },
                "workspaces": workspaces,
            })
        }
        WorkspaceSubCommand::New(args) => {
            let path = args.path.unwrap_or_else(|| {
                // Default: .ticket/ inside the current directory (repo-local)
                std::env::current_dir()
                    .unwrap_or_else(|_| PathBuf::from("."))
                    .join(".ticket")
            });
            let mut config = WorkspaceConfig::load();
            match config.add(&args.name, path.clone()) {
                Err(e) => json!({ "command": "workspace_new", "status": "error", "message": e }),
                Ok(()) => {
                    if let Err(e) = config.save() {
                        return json!({ "command": "workspace_new", "status": "error", "message": e.to_string() });
                    }
                    json!({
                        "command": "workspace_new",
                        "status": "ok",
                        "name": args.name,
                        "path": path.to_string_lossy(),
                    })
                }
            }
        }
        WorkspaceSubCommand::Use(use_args) => {
            if use_args.local {
                let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                let local_path = cwd.join(crate::workspace::LOCAL_WORKSPACE_FILE);

                // Resolve the index path: registry lookup first, then treat name as path.
                let config = WorkspaceConfig::load();
                let index_path = config
                    .workspaces
                    .get(&use_args.name)
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from(&use_args.name));

                // Write a repo-relative path so the file is self-contained
                // (no dependency on the user-level workspace registry).
                let rel = crate::workspace::make_relative_path(&cwd, &index_path);
                let content = rel.to_string_lossy().replace('\\', "/");

                match std::fs::write(&local_path, &content) {
                    Err(e) => json!({ "command": "workspace_use", "status": "error", "message": e.to_string() }),
                    Ok(()) => json!({
                        "command": "workspace_use",
                        "status": "ok",
                        "name": use_args.name,
                        "scope": "local",
                        "path": content,
                        "file": local_path.to_string_lossy(),
                    }),
                }
            } else {
                let mut config = WorkspaceConfig::load();
                match config.set_active(&use_args.name) {
                    Err(e) => json!({ "command": "workspace_use", "status": "error", "message": e }),
                    Ok(()) => {
                        if let Err(e) = config.save() {
                            return json!({ "command": "workspace_use", "status": "error", "message": e.to_string() });
                        }
                        json!({
                            "command": "workspace_use",
                            "status": "ok",
                            "name": use_args.name,
                            "scope": "global",
                        })
                    }
                }
            }
        }
        WorkspaceSubCommand::Current => {
            // Reproduce the full resolution chain with source annotation
            let (path, source) = workspace::resolve_workspace();
            json!({
                "command": "workspace_current",
                "status": "ok",
                "path": path.to_string_lossy(),
                "source": source.description(),
            })
        }
        WorkspaceSubCommand::Remove(args) => {
            let mut config = WorkspaceConfig::load();
            match config.remove(&args.name) {
                Err(e) => json!({ "command": "workspace_remove", "status": "error", "message": e }),
                Ok(()) => {
                    if let Err(e) = config.save() {
                        return json!({ "command": "workspace_remove", "status": "error", "message": e.to_string() });
                    }
                    json!({
                        "command": "workspace_remove",
                        "status": "ok",
                        "name": args.name,
                    })
                }
            }
        }
    }
}



fn render_human(payload: Value) -> String {
    serde_json::to_string_pretty(&payload).unwrap_or_else(|_| payload.to_string())
}

pub fn error_output(message: &str, as_json: bool) -> String {
    if as_json {
        serde_json::to_string_pretty(&ErrorEnvelope {
            code: "invalid_request".to_string(),
            message: message.to_string(),
        })
        .unwrap_or_else(|_| format!("{{\"code\":\"invalid_request\",\"message\":\"{}\"}}", message))
    } else {
        message.to_string()
    }
}

pub fn parse_cli_from<I, T>(args: I) -> Result<TicketCli, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    TicketCli::try_parse_from(args)
}

pub fn payload_as_json_object(payload: &Value) -> Option<&Map<String, Value>> {
    payload.as_object()
}

fn parse_fields(raw_fields: &[String]) -> Result<BTreeMap<String, String>, CliRunError> {
    let mut fields = BTreeMap::new();
    for raw in raw_fields {
        let Some((k, v)) = raw.split_once('=') else {
            return Err(CliRunError::InvalidFieldPatch(raw.clone()));
        };
        fields.insert(k.trim().to_string(), v.trim().to_string());
    }
    Ok(fields)
}

fn parse_fields_to_json(raw_fields: &[String]) -> Result<BTreeMap<String, Value>, CliRunError> {
    parse_fields(raw_fields).map(|m| {
        m.into_iter()
            .map(|(k, v)| (k, Value::String(v)))
            .collect()
    })
}

#[derive(Debug, Serialize)]
struct _MachineError<'a> {
    code: &'a str,
    message: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fields_supports_key_values() {
        let got = parse_fields(&["owner=alice".to_string(), "priority=high".to_string()])
            .expect("field parsing should succeed");
        assert_eq!(got.get("owner").map(String::as_str), Some("alice"));
        assert_eq!(got.get("priority").map(String::as_str), Some("high"));
    }

    #[test]
    fn parse_fields_rejects_invalid_format() {
        let err = parse_fields(&["broken".to_string()]).expect_err("must reject missing '='");
        assert!(matches!(err, CliRunError::InvalidFieldPatch(_)));
    }
}

