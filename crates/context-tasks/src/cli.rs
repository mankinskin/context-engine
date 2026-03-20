use std::collections::BTreeMap;

use clap::{Args, Parser, Subcommand};
use serde::Serialize;
use serde_json::{Map, Value, json};
use uuid::Uuid;

use crate::contracts::command_schema::{
    CommandEnvelope, ErrorEnvelope, export_command_schema, export_command_schema_json,
};

#[derive(Debug, Parser)]
#[command(name = "ticket", about = "Task tracker CLI (initial draft)", version)]
pub struct TicketCli {
    /// Return machine-readable JSON envelope output.
    #[arg(long, global = true)]
    pub json: bool,

    /// Optional request identifier propagated in JSON envelope output.
    #[arg(long, global = true)]
    pub request_id: Option<String>,

    #[command(subcommand)]
    pub command: TicketCommandCli,
}

#[derive(Debug, Subcommand)]
pub enum TicketCommandCli {
    /// Create a new ticket manifest and return its identifier.
    Create(CreateArgs),
    /// Get a ticket by UUID.
    Get(IdArgs),
    /// Update a ticket with field patches and optional state transition.
    Update(UpdateArgs),
    /// List tickets with optional state/type filtering.
    List(ListArgs),
    /// Soft-delete a ticket.
    Delete(IdArgs),
    /// Run full scan/reindex over registered roots.
    Scan(ScanArgs),
    /// Claim a ticket lease for active work.
    Claim(ClaimArgs),
    /// Release an active ticket lease.
    Unclaim(UnclaimArgs),
    /// Full-text search expression (draft placeholder for Phase 3).
    Search(TextArgs),
    /// Unified query expression parser (draft placeholder for Phase 3).
    Query(TextArgs),
    /// Show history log for ticket.
    History(HistoryArgs),
    /// Show diff for ticket between revisions.
    Diff(DiffArgs),
    /// Revert ticket to a historical revision (forward commit semantics).
    Revert(RevertArgs),
    /// Mark merge-boundary completion metadata.
    #[command(name = "finalize-merge")]
    FinalizeMerge(FinalizeMergeArgs),
    /// Export command namespace/schema used for automation clients.
    #[command(name = "export-command-schema")]
    ExportCommandSchema,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Optional UUID (auto-generated when omitted).
    #[arg(long)]
    pub id: Option<Uuid>,
    /// Ticket type identifier.
    #[arg(long = "type")]
    pub ticket_type: Option<String>,
    /// Human-readable title.
    #[arg(long)]
    pub title: Option<String>,
    /// Initial state.
    #[arg(long)]
    pub state: Option<String>,
    /// Dynamic fields in key=value form. Repeatable.
    #[arg(long = "field")]
    pub fields: Vec<String>,
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
    /// Optional explicit transition source state.
    #[arg(long = "from-state")]
    pub from_state: Option<String>,
    /// Optional explicit transition target state.
    #[arg(long = "to-state")]
    pub to_state: Option<String>,
    /// Dynamic fields in key=value form. Repeatable.
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
    /// Rebuild derived indexes from filesystem source of truth.
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
    /// Required capabilities for optional isolated execution backends.
    #[arg(long = "capability")]
    pub capabilities: Vec<String>,
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

#[derive(Debug, thiserror::Error)]
pub enum CliRunError {
    #[error("invalid field patch: {0}")]
    InvalidFieldPatch(String),
    #[error("failed to serialize command schema: {0}")]
    CommandSchema(#[from] serde_json::Error),
}

pub enum CliOutput {
    Json(Value),
    Text(String),
}

pub fn run(cli: TicketCli) -> Result<CliOutput, CliRunError> {
    let payload = execute_draft(cli.command)?;
    if cli.json {
        let request_id = cli
            .request_id
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        let envelope = CommandEnvelope {
            request_id,
            payload,
        };
        Ok(CliOutput::Json(json!(envelope)))
    } else {
        Ok(CliOutput::Text(render_human(payload)))
    }
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

fn execute_draft(command: TicketCommandCli) -> Result<Value, CliRunError> {
    match command {
        TicketCommandCli::Create(args) => {
            let id = args.id.unwrap_or_else(Uuid::new_v4);
            let fields = parse_fields(&args.fields)?;
            Ok(json!({
                "command": "create",
                "status": "draft",
                "ticket": {
                    "id": id,
                    "ticket_type": args.ticket_type,
                    "title": args.title,
                    "state": args.state,
                    "fields": fields,
                }
            }))
        }
        TicketCommandCli::Get(args) => Ok(json!({
            "command": "get",
            "status": "draft",
            "id": args.id,
            "message": "backend read not wired yet"
        })),
        TicketCommandCli::Update(args) => {
            let fields = parse_fields(&args.fields)?;
            Ok(json!({
                "command": "update",
                "status": "draft",
                "id": args.id,
                "from_state": args.from_state,
                "to_state": args.to_state,
                "fields": fields
            }))
        }
        TicketCommandCli::List(args) => Ok(json!({
            "command": "list",
            "status": "draft",
            "filter": {
                "state": args.state,
                "ticket_type": args.ticket_type,
                "limit": args.limit
            },
            "items": []
        })),
        TicketCommandCli::Delete(args) => Ok(json!({
            "command": "delete",
            "status": "draft",
            "id": args.id
        })),
        TicketCommandCli::Scan(args) => Ok(json!({
            "command": "scan",
            "status": "draft",
            "reindex": args.reindex
        })),
        TicketCommandCli::Claim(args) => Ok(json!({
            "command": "claim",
            "status": "draft",
            "id": args.id,
            "agent_id": args.agent_id,
            "ttl_secs": args.ttl_secs,
            "capabilities": args.capabilities,
            "executor_backend": "local"
        })),
        TicketCommandCli::Unclaim(args) => Ok(json!({
            "command": "unclaim",
            "status": "draft",
            "id": args.id,
            "reason": args.reason
        })),
        TicketCommandCli::Search(args) => Ok(json!({
            "command": "search",
            "status": "draft",
            "expression": args.expression,
            "matches": []
        })),
        TicketCommandCli::Query(args) => Ok(json!({
            "command": "query",
            "status": "draft",
            "expression": args.expression,
            "matches": []
        })),
        TicketCommandCli::History(args) => Ok(json!({
            "command": "history",
            "status": "draft",
            "id": args.id,
            "limit": args.limit,
            "entries": []
        })),
        TicketCommandCli::Diff(args) => Ok(json!({
            "command": "diff",
            "status": "draft",
            "id": args.id,
            "from": args.from,
            "to": args.to,
            "patch": null
        })),
        TicketCommandCli::Revert(args) => Ok(json!({
            "command": "revert",
            "status": "draft",
            "id": args.id,
            "to": args.to_sha
        })),
        TicketCommandCli::FinalizeMerge(args) => Ok(json!({
            "command": "finalize_merge",
            "status": "draft",
            "id": args.id,
            "merge_commit": args.merge_commit
        })),
        TicketCommandCli::ExportCommandSchema => {
            let schema_json = export_command_schema_json()?;
            let schema: Value = serde_json::from_str(&schema_json)?;
            Ok(json!({
                "command": "export_command_schema",
                "status": "ok",
                "schema": schema,
                "known_commands": export_command_schema().commands,
            }))
        }
    }
}

fn render_human(payload: Value) -> String {
    serde_json::to_string_pretty(&payload).unwrap_or_else(|_| payload.to_string())
}

fn parse_fields(raw_fields: &[String]) -> Result<BTreeMap<String, String>, CliRunError> {
    let mut fields = BTreeMap::new();
    for raw in raw_fields {
        let Some((k, v)) = raw.split_once('=') else {
            return Err(CliRunError::InvalidFieldPatch(raw.clone()));
        };
        if k.trim().is_empty() {
            return Err(CliRunError::InvalidFieldPatch(raw.clone()));
        }
        fields.insert(k.trim().to_string(), v.trim().to_string());
    }
    Ok(fields)
}

#[derive(Debug, Serialize)]
struct _MachineError<'a> {
    code: &'a str,
    message: &'a str,
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
