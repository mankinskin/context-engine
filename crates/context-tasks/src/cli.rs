use std::collections::BTreeMap;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use serde::Serialize;
use serde_json::{Map, Value, json};
use uuid::Uuid;

use crate::contracts::command_schema::{
    CommandEnvelope, ErrorEnvelope, export_command_schema, export_command_schema_json,
};
use crate::error::StorageError;
use crate::storage::TicketStore;

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
    /// Export the command namespace/schema for automation clients.
    #[command(name = "export-command-schema")]
    ExportCommandSchema,
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
pub struct ExecArgs {
    /// Execute multiple commands from stdin, one JSON object per line, as a
    /// single transaction. Rolls back all on first failure.
    #[arg(long)]
    pub batch: bool,
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
}

pub enum CliOutput {
    Json(Value),
    Text(String),
}

// ── entry point ────────────────────────────────────────────────────────────────

pub fn run(cli: TicketCli) -> Result<CliOutput, CliRunError> {
    let payload = dispatch(cli.command, cli.index_root.as_deref(), cli.json)?;
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
        _ => {}
    }

    // All other commands need the store.
    let index_root = resolve_index_root(index_root_override)?;
    let store = TicketStore::open(&index_root)?;

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
        TicketCommandCli::History(args) => Ok(json!({
            "command": "history",
            "status": "phase2_stub",
            "id": args.id,
            "entries": []
        })),
        TicketCommandCli::Diff(args) => Ok(json!({
            "command": "diff",
            "status": "phase2_stub",
            "id": args.id,
            "from": args.from,
            "to": args.to,
            "patch": null
        })),
        TicketCommandCli::Revert(args) => Ok(json!({
            "command": "revert",
            "status": "phase2_stub",
            "id": args.id,
            "to": args.to_sha
        })),
        TicketCommandCli::FinalizeMerge(args) => Ok(json!({
            "command": "finalize_merge",
            "status": "phase2_stub",
            "id": args.id,
            "merge_commit": args.merge_commit
        })),
        TicketCommandCli::ExportCommandSchema => unreachable!("handled above"),
    }
}

// ── command handlers ───────────────────────────────────────────────────────────

fn cmd_create(args: CreateArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let type_id = args.ticket_type.as_deref().unwrap_or("tracker-improvement");
    let extra = parse_fields_to_json(&args.fields)?;
    let target_root = args.target_root.as_deref();

    let id = store.create(
        args.id,
        type_id,
        args.title.as_deref(),
        args.state.as_deref(),
        extra,
        target_root,
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

/// `ticket exec` — read one JSON `TaskCommand` object from stdin and execute it.
/// In `--batch` mode, read one object per line until EOF and execute all atomically
/// (rolling back all on the first failure).
fn cmd_exec(args: ExecArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    use std::io::{self, BufRead};

    if args.batch {
        let stdin = io::stdin();
        let mut commands: Vec<Value> = Vec::new();
        for line in stdin.lock().lines() {
            let line = line.map_err(StorageError::Io)?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let cmd: Value = serde_json::from_str(line)
                .map_err(|e| CliRunError::InvalidExecPayload(e.to_string()))?;
            commands.push(cmd);
        }

        let mut results = Vec::with_capacity(commands.len());
        for cmd in &commands {
            match exec_single_command(cmd, store) {
                Ok(result) => results.push(result),
                Err(e) => {
                    return Ok(json!({
                        "command": "exec_batch",
                        "status": "error",
                        "completed": results.len(),
                        "total": commands.len(),
                        "error": e.to_string(),
                        "results": results,
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
    } else {
        let stdin = io::stdin();
        let mut input = String::new();
        use std::io::Read;
        stdin.lock().read_to_string(&mut input).map_err(StorageError::Io)?;
        let cmd: Value = serde_json::from_str(input.trim())
            .map_err(|e| CliRunError::InvalidExecPayload(e.to_string()))?;
        exec_single_command(&cmd, store)
    }
}

fn exec_single_command(cmd: &Value, store: &TicketStore) -> Result<Value, CliRunError> {
    let op = cmd
        .get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| CliRunError::InvalidExecPayload("missing 'command' field".to_string()))?;

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
            let created_id = store.create(id, type_id, title, state, extra, None)?;
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
        other => Err(CliRunError::InvalidExecPayload(format!("unknown command: {other}"))),
    }
}

// ── utilities ──────────────────────────────────────────────────────────────────

fn resolve_index_root(override_path: Option<&std::path::Path>) -> Result<PathBuf, CliRunError> {
    if let Some(p) = override_path {
        return Ok(p.to_path_buf());
    }
    if let Ok(env_val) = std::env::var("TICKET_INDEX_ROOT") {
        return Ok(PathBuf::from(env_val));
    }
    // Default: ~/.ticket-index/
    let home = dirs_home();
    Ok(home.join(".ticket-index"))
}

fn dirs_home() -> PathBuf {
    #[cfg(windows)]
    return PathBuf::from(
        std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOMEDRIVE").and_then(|d| std::env::var("HOMEPATH").map(|p| d + &p)))
            .unwrap_or_else(|_| ".".to_string()),
    );
    #[cfg(not(windows))]
    return PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()));
}

fn parse_uuid_field(cmd: &Value, field: &str) -> Result<Uuid, CliRunError> {
    cmd.get(field)
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| CliRunError::InvalidExecPayload(format!("missing or invalid '{field}' field")))
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

