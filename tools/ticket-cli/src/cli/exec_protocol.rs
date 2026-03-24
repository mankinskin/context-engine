use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;

use chrono::Utc;
use serde_json::{Value, json};
use uuid::Uuid;

use ticket_api::error::StorageError;
use ticket_api::execution::provider::{
    CopilotApiClient, CopilotApiConfig, ProviderError, StartSubagentResponse, SubagentProvider,
};
use ticket_api::execution::runner::{
    AssignmentRunRequest, AssignmentRunner, GitSandboxProvisioner, RunnerConfig,
    SandboxProvisioner,
};
use ticket_api::execution::sandbox::{SandboxError, SandboxHandle, SandboxSpec};
use ticket_api::model::edge::EdgeRecord;
use ticket_api::storage::TicketStore;
use ticket_api::storage::store::GateStatus;
use ticket_api::storage::ticket_fs::TicketFs;

use super::commands;
use super::*;

#[derive(Debug)]
enum BatchUndoOp {
    Delete { id: Uuid },
    RestoreUpdate {
        id: Uuid,
        saved_extra: BTreeMap<String, Value>,
        saved_state: Option<String>,
    },
    RemoveEdge { from: Uuid, to: Uuid, kind: String },
}

fn batch_pre_capture(
    cmd: &Value,
    store: &TicketStore,
) -> Option<(String, Uuid, BTreeMap<String, Value>, Option<String>)> {
    let op = cmd.get("command").and_then(|v| v.as_str())?;
    let op = op.strip_prefix("task_").unwrap_or(op);
    match op {
        "update" => {
            let id: Uuid = cmd.get("id").and_then(|v| v.as_str())?.parse().ok()?;
            if let Ok(Some(indexed)) = store.get_indexed(&id) {
                let mut saved = BTreeMap::new();
                if let Some(t) = &indexed.title {
                    saved.insert("title".to_string(), serde_json::Value::String(t.clone()));
                }
                Some(("update".to_string(), id, saved, indexed.state.clone()))
            } else {
                None
            }
        }
        _ => None,
    }
}

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
            Some(BatchUndoOp::RestoreUpdate {
                id,
                saved_extra,
                saved_state,
            })
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

fn apply_batch_undo(undo: BatchUndoOp, store: &TicketStore, errors: &mut Vec<String>) {
    match undo {
        BatchUndoOp::Delete { id } => {
            if let Err(e) = store.delete(&id) {
                errors.push(format!("rollback delete {id}: {e}"));
            }
        }
        BatchUndoOp::RestoreUpdate {
            id,
            saved_extra,
            saved_state,
        } => {
            if let Err(e) = store.force_restore(&id, saved_extra, saved_state) {
                errors.push(format!("rollback restore {id}: {e}"));
            }
        }
        BatchUndoOp::RemoveEdge { from, to, kind } => {
            let edge = EdgeRecord {
                from,
                to,
                kind,
                created_at: Utc::now(),
            };
            if let Err(e) = store.remove_edge(edge) {
                errors.push(format!("rollback remove_edge {from}->{to}: {e}"));
            }
        }
    }
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
    let mut results: Vec<Value> = Vec::with_capacity(commands.len());
    let mut undo_stack: Vec<BatchUndoOp> = Vec::with_capacity(commands.len());

    for cmd in commands {
        let undo_hint = batch_pre_capture(cmd, store);

        match exec_single_command(cmd, store) {
            Ok(result) => {
                if let Some(undo) = batch_post_undo(&result, undo_hint) {
                    undo_stack.push(undo);
                }
                results.push(result);
            }
            Err(e) => {
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

// ── CLI-syntax batch ─────────────────────────────────────────────────────────

#[derive(clap::Parser)]
#[command(name = "ticket")]
struct BatchLineParser {
    #[command(subcommand)]
    command: TicketCommandCli,
}

fn parse_batch_line(line: &str) -> Result<TicketCommandCli, CliRunError> {
    let mut tokens = shell_words::split(line)
        .map_err(|e| CliRunError::InvalidExecPayload(format!("cannot parse line: {e}")))?;
    if tokens.is_empty() {
        return Err(CliRunError::InvalidExecPayload("empty command line".to_string()));
    }
    tokens.insert(0, "ticket".to_string());
    BatchLineParser::try_parse_from(tokens)
        .map(|p| p.command)
        .map_err(|e| CliRunError::InvalidExecPayload(format!("{e}")))
}

fn read_cli_batch_commands(file: Option<std::path::PathBuf>) -> Result<Vec<TicketCommandCli>, CliRunError> {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};

    let lines: Vec<String> = if let Some(path) = file {
        let f = File::open(&path).map_err(|e| {
            CliRunError::InvalidExecPayload(format!("cannot open batch file {}: {e}", path.display()))
        })?;
        BufReader::new(f)
            .lines()
            .collect::<io::Result<_>>()
            .map_err(StorageError::Io)?
    } else {
        let stdin = io::stdin();
        stdin
            .lock()
            .lines()
            .collect::<io::Result<_>>()
            .map_err(StorageError::Io)?
    };

    let mut cmds = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let cmd = parse_batch_line(trimmed)
            .map_err(|e| CliRunError::InvalidExecPayload(format!("line {}: {e}", i + 1)))?;
        cmds.push(cmd);
    }
    Ok(cmds)
}

fn execute_cli_batch(cmds: Vec<TicketCommandCli>, store: &TicketStore) -> Result<Value, CliRunError> {
    let total = cmds.len();
    let mut results: Vec<Value> = Vec::with_capacity(total);
    let mut undo_stack: Vec<BatchUndoOp> = Vec::with_capacity(total);

    for cmd in cmds {
        let is_create = matches!(cmd, TicketCommandCli::Create(_));
        let is_link = matches!(cmd, TicketCommandCli::Link(_));
        let update_pre: Option<(Uuid, BTreeMap<String, Value>, Option<String>)> =
            if let TicketCommandCli::Update(ref args) = cmd {
                commands::resolve_uuid_prefix(&args.id, store)
                    .ok()
                    .and_then(|id| {
                        store.get_indexed(&id).ok().flatten().map(|indexed| {
                            let mut saved = BTreeMap::new();
                            if let Some(t) = &indexed.title {
                                saved.insert("title".to_string(), Value::String(t.clone()));
                            }
                            (id, saved, indexed.state.clone())
                        })
                    })
            } else {
                None
            };

        match batch_dispatch(cmd, store) {
            Ok(result) => {
                let undo = if is_create {
                    result
                        .get("id")
                        .and_then(|v| v.as_str())
                        .and_then(|s| s.parse().ok())
                        .map(|id| BatchUndoOp::Delete { id })
                } else if let Some((id, saved_extra, saved_state)) = update_pre {
                    Some(BatchUndoOp::RestoreUpdate { id, saved_extra, saved_state })
                } else if is_link {
                    let from = result
                        .get("from")
                        .and_then(|v| v.as_str())
                        .and_then(|s| s.parse().ok());
                    let to = result
                        .get("to")
                        .and_then(|v| v.as_str())
                        .and_then(|s| s.parse().ok());
                    let kind = result
                        .get("kind")
                        .and_then(|v| v.as_str())
                        .map(str::to_string);
                    match (from, to, kind) {
                        (Some(from), Some(to), Some(kind)) => {
                            Some(BatchUndoOp::RemoveEdge { from, to, kind })
                        }
                        _ => None,
                    }
                } else {
                    None
                };
                if let Some(u) = undo {
                    undo_stack.push(u);
                }
                results.push(result);
            }
            Err(e) => {
                let mut rollback_errors: Vec<String> = Vec::new();
                for undo in undo_stack.into_iter().rev() {
                    apply_batch_undo(undo, store, &mut rollback_errors);
                }
                return Ok(json!({
                    "command": "batch",
                    "status": "error",
                    "completed": results.len(),
                    "total": total,
                    "error": e.to_string(),
                    "rolled_back": rollback_errors.is_empty(),
                    "rollback_errors": rollback_errors,
                }));
            }
        }
    }

    Ok(json!({
        "command": "batch",
        "status": "ok",
        "count": results.len(),
        "results": results,
    }))
}

fn batch_dispatch(cmd: TicketCommandCli, store: &TicketStore) -> Result<Value, CliRunError> {
    match cmd {
        TicketCommandCli::Create(args) => commands::cmd_create(args, store),
        TicketCommandCli::Get(args) => commands::cmd_get(args, store),
        TicketCommandCli::Describe(args) => commands::cmd_describe(args, store),
        TicketCommandCli::Update(args) => commands::cmd_update(args, store),
        TicketCommandCli::Repro(args) => commands::cmd_repro(args, store),
        TicketCommandCli::List(args) => commands::cmd_list(args, store),
        TicketCommandCli::Delete(args) => commands::cmd_delete(args, store),
        TicketCommandCli::Link(args) => commands::cmd_link(args, store),
        TicketCommandCli::Unlink(args) => commands::cmd_unlink(args, store),
        TicketCommandCli::Links(args) => commands::cmd_links(args, store),
        TicketCommandCli::Subgraph(args) => commands::cmd_subgraph(args, store),
        TicketCommandCli::Topgraph(args) => commands::cmd_topgraph(args, store),
        TicketCommandCli::Search(args) | TicketCommandCli::Query(args) => {
            commands::cmd_search(args, store)
        }
        TicketCommandCli::Health(args) => commands::cmd_health(args, store),
        TicketCommandCli::Close(args) => commands::cmd_close(args, store),
        TicketCommandCli::Cancel(args) => commands::cmd_cancel(args, store),
        TicketCommandCli::Status(args) => commands::cmd_status(args, store),
        TicketCommandCli::ReadyOverview(args) => commands::cmd_ready_overview(args, store),
        TicketCommandCli::Attach(args) => commands::cmd_attach(args, store),
        TicketCommandCli::Assets(args) => commands::cmd_assets(args, store),
        TicketCommandCli::History(args) => commands::cmd_history(args, store),
        TicketCommandCli::Diff(args) => commands::cmd_diff(args, store),
        TicketCommandCli::Revert(args) => commands::cmd_revert(args, store),
        TicketCommandCli::Audit => commands::cmd_audit(store),
        TicketCommandCli::Serve(_) => {
            Err(CliRunError::BadRequest("'serve' cannot be used in a batch".to_string()))
        }
        TicketCommandCli::Watch(_) => {
            Err(CliRunError::BadRequest("'watch' cannot be used in a batch".to_string()))
        }
        TicketCommandCli::Exec(_) => {
            Err(CliRunError::BadRequest("'exec' cannot be used in a batch".to_string()))
        }
        TicketCommandCli::Batch(_) => {
            Err(CliRunError::BadRequest("'batch' cannot be nested".to_string()))
        }
        TicketCommandCli::Scan(_) => {
            Err(CliRunError::BadRequest("'scan' cannot be used in a batch".to_string()))
        }
        TicketCommandCli::Claim(_) | TicketCommandCli::Unclaim(_) | TicketCommandCli::Leases => {
            Err(CliRunError::BadRequest(
                "lease commands cannot be used in a batch".to_string(),
            ))
        }
        TicketCommandCli::AddRoot(_) => {
            Err(CliRunError::BadRequest("'add-root' cannot be used in a batch".to_string()))
        }
        TicketCommandCli::ExportCommandSchema => Err(CliRunError::BadRequest(
            "'export-command-schema' cannot be used in a batch".to_string(),
        )),
        TicketCommandCli::Workspace(_) => {
            Err(CliRunError::BadRequest("'workspace' cannot be used in a batch".to_string()))
        }
        TicketCommandCli::FinalizeMerge(_) => Err(CliRunError::BadRequest(
            "'finalize-merge' is not supported in a batch".to_string(),
        )),
    }
}

pub(crate) fn cmd_batch(args: BatchArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let cmds = read_cli_batch_commands(args.file)?;
    if cmds.is_empty() {
        return Ok(json!({
            "command": "batch",
            "status": "ok",
            "count": 0,
            "results": [],
        }));
    }
    execute_cli_batch(cmds, store)
}

pub(crate) fn cmd_exec(args: ExecArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    use std::io::{self, Read};

    if args.batch {
        let stdin = io::stdin();
        let commands = read_batch_commands(stdin.lock())?;
        execute_batch_commands(&commands, store)
    } else {
        let stdin = io::stdin();
        let mut input = String::new();
        stdin
            .lock()
            .read_to_string(&mut input)
            .map_err(StorageError::Io)?;
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

    let op = raw_op.strip_prefix("task_").unwrap_or(raw_op);

    match op {
        "create" => {
            let type_id = cmd
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("tracker-improvement");
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
            let items_json: Vec<Value> = items
                .iter()
                .map(|t| {
                    json!({
                        "id": t.id, "type": t.type_id, "title": t.title, "state": t.state,
                    })
                })
                .collect();
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
            let reason = cmd
                .get("reason")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

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
            let items: Vec<Value> = edges
                .iter()
                .map(|e| {
                    json!({
                        "from": e.from,
                        "to": e.to,
                        "kind": e.kind,
                    })
                })
                .collect();
            Ok(json!({
                "command": "links",
                "status": "ok",
                "id": id,
                "count": items.len(),
                "edges": items,
            }))
        }
        "search" => {
            let expr = cmd
                .get("query")
                .and_then(|v| v.as_str())
                .ok_or_else(|| CliRunError::InvalidExecPayload("missing 'query' field".to_string()))?;
            let limit = cmd.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as usize;
            let results = store.search_tickets(expr, limit)?;
            let items: Vec<Value> = results
                .iter()
                .map(|r| {
                    json!({
                        "id": r.id, "title": r.title, "state": r.state, "snippet": r.snippet, "score": r.score,
                    })
                })
                .collect();
            Ok(json!({ "command": "search", "status": "ok", "count": items.len(), "results": items }))
        }
        "assignment_start" => {
            let ticket_id = parse_uuid_field(cmd, "ticket_id")?;
            let assignment_id = req_str(cmd, "assignment_id")?;
            let prompt = req_str(cmd, "prompt")?;
            let simulate = cmd
                .get("simulate")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

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
                let provider_cfg = CopilotApiConfig::from_env().map_err(|e| {
                    CliRunError::BadRequest(format!("task_assignment_start config error: {e}"))
                })?;
                let provider = CopilotApiClient::new(provider_cfg).map_err(|e| {
                    CliRunError::BadRequest(format!("task_assignment_start client init error: {e}"))
                })?;
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
            let profile = cmd
                .get("validation_profile")
                .and_then(|v| v.as_str())
                .unwrap_or("default");
            let checks: Vec<String> = cmd
                .get("required_checks")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default();
            let manifest =
                store.validate_start(&ticket_id, assignment_id, validator_id, profile, checks)?;
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
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect()
                })
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
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect()
                })
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
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_else(|| {
                    vec![
                        "R1".to_string(),
                        "R2".to_string(),
                        "R3".to_string(),
                        "R4".to_string(),
                    ]
                });
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
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_else(|| {
                    vec![
                        "R1".to_string(),
                        "R2".to_string(),
                        "R3".to_string(),
                        "R4".to_string(),
                    ]
                });
            let outcome =
                store.release_promote(release_target, release_version, merge_commit, &required_gates)?;
            Ok(json!({
                "command": "task_release_promote",
                "status": "ok",
                "release_target": outcome.release_target,
                "release_version": outcome.release_version,
                "promoted_ticket_count": outcome.promoted_ticket_count,
                "monitoring_state": outcome.monitoring_state,
            }))
        }
        "subgraph" | "topgraph" => {
            let id = parse_uuid_field(cmd, "root")?;
            let default_dir = if op == "topgraph" { "in" } else { "out" };
            let direction = cmd.get("direction").and_then(|v| v.as_str()).unwrap_or(default_dir);
            let edge_kind = cmd.get("edge_kind").and_then(|v| v.as_str()).unwrap_or("all");
            let depth_limit = cmd.get("depth").and_then(|v| v.as_u64()).unwrap_or(4).min(8) as usize;

            let all_edges = store.list_all_edges()?;
            let mut visited: HashSet<Uuid> = HashSet::new();
            let mut nodes: Vec<Value> = Vec::new();
            let mut edges_out: Vec<Value> = Vec::new();
            let mut queue: VecDeque<(Uuid, usize)> = VecDeque::new();
            queue.push_back((id, 0));
            let mut max_depth = 0usize;

            while let Some((current, d)) = queue.pop_front() {
                if !visited.insert(current) { continue; }
                max_depth = max_depth.max(d);
                let indexed = store.get_indexed(&current)?;
                nodes.push(json!({
                    "id": current,
                    "title": indexed.as_ref().and_then(|t| t.title.as_deref()),
                    "state": indexed.as_ref().and_then(|t| t.state.as_deref()),
                    "depth": d,
                }));
                if d >= depth_limit { continue; }
                for edge in &all_edges {
                    let kind_ok = edge_kind == "all" || edge.kind == edge_kind;
                    if !kind_ok { continue; }
                    let (neighbor, is_out) = if edge.from == current {
                        (edge.to, true)
                    } else if edge.to == current {
                        (edge.from, false)
                    } else { continue; };
                    let dir_ok = match direction {
                        "out" => is_out, "in" => !is_out, _ => true,
                    };
                    if dir_ok {
                        edges_out.push(json!({"from": edge.from, "to": edge.to, "kind": &edge.kind}));
                        if !visited.contains(&neighbor) {
                            queue.push_back((neighbor, d + 1));
                        }
                    }
                }
            }
            Ok(json!({
                "command": op,
                "status": "ok",
                "nodes": nodes,
                "edges": edges_out,
                "stats": { "nodes_returned": nodes.len(), "edges_returned": edges_out.len(), "max_depth_reached": max_depth },
            }))
        }
        "health" => {
            let all_flag = cmd.get("all").and_then(|v| v.as_bool()).unwrap_or(false);
            let ids_arr: Vec<String> = cmd.get("ids")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
                .unwrap_or_default();
            let depth_limit = cmd.get("depth").and_then(|v| v.as_u64()).unwrap_or(6).min(8) as usize;
            let direction = cmd.get("direction").and_then(|v| v.as_str()).unwrap_or("out");

            let all_edges = store.list_all_edges()?;

            let tickets: Vec<_> = if !ids_arr.is_empty() {
                let mut result = Vec::new();
                for id_str in &ids_arr {
                    let uid: Uuid = id_str.parse().map_err(|_|
                        CliRunError::InvalidExecPayload(format!("invalid UUID in ids: {id_str}")))?;
                    if let Some(t) = store.get_indexed(&uid)? {
                        if !t.deleted { result.push(t); }
                    }
                }
                result
            } else if all_flag {
                store.list(None, None, None)?
            } else {
                let root = parse_uuid_field(cmd, "root")?;
                let mut visited: HashSet<Uuid> = HashSet::new();
                let mut collected: Vec<Uuid> = Vec::new();
                let mut queue: VecDeque<(Uuid, usize)> = VecDeque::new();
                queue.push_back((root, 0));
                while let Some((cur, d)) = queue.pop_front() {
                    if !visited.insert(cur) { continue; }
                    collected.push(cur);
                    if d >= depth_limit { continue; }
                    for edge in &all_edges {
                        let kind_ok = edge.kind == "depends_on" || edge.kind == "linked";
                        if !kind_ok { continue; }
                        let (neighbor, is_out) = if edge.from == cur {
                            (edge.to, true)
                        } else if edge.to == cur {
                            (edge.from, false)
                        } else { continue; };
                        let dir_ok = match direction {
                            "out" => is_out, "in" => !is_out, _ => true,
                        };
                        if dir_ok && !visited.contains(&neighbor) {
                            queue.push_back((neighbor, d + 1));
                        }
                    }
                }
                collected.iter()
                    .filter_map(|id| store.get_indexed(id).ok().flatten())
                    .filter(|t| !t.deleted)
                    .collect()
            };

            let ticket_ids: HashSet<Uuid> = tickets.iter().map(|t| t.id).collect();
            let done_states: HashSet<&str> = ["done", "cancelled"].into_iter().collect();
            let done_ids: HashSet<Uuid> = tickets.iter()
                .filter(|t| t.state.as_deref().map(|s| done_states.contains(s)).unwrap_or(false))
                .map(|t| t.id).collect();

            let mut unresolved_deps: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
            for edge in &all_edges {
                if edge.kind == "depends_on" && ticket_ids.contains(&edge.from) && !done_ids.contains(&edge.to) {
                    unresolved_deps.entry(edge.from).or_default().push(edge.to);
                }
            }

            let mut findings: Vec<Value> = Vec::new();
            let mut summary: BTreeMap<String, u64> = BTreeMap::new();

            for t in &tickets {
                if done_ids.contains(&t.id) { continue; }
                let short_id = &t.id.to_string()[..8];
                let title = t.title.as_deref().unwrap_or("?");

                let desc = TicketFs::read_description(&t.path);
                if desc.is_none() {
                    *summary.entry("missing_description".into()).or_insert(0) += 1;
                    findings.push(json!({"ticket_id": t.id, "short_id": short_id, "title": title,
                        "check": "missing_description", "severity": "warning",
                        "message": "No description.md file — ticket lacks detailed context."}));
                } else if let Some(ref body) = desc {
                    let trimmed_len = body.trim().len();
                    if trimmed_len < 50 {
                        *summary.entry("short_description".into()).or_insert(0) += 1;
                        findings.push(json!({"ticket_id": t.id, "short_id": short_id, "title": title,
                            "check": "short_description", "severity": "info",
                            "message": format!("description.md is very short ({trimmed_len} chars) — consider adding more detail.")}));
                    }
                }

                if t.title.is_none() || t.title.as_deref() == Some("") {
                    *summary.entry("missing_title".into()).or_insert(0) += 1;
                    findings.push(json!({"ticket_id": t.id, "short_id": short_id, "title": "(none)",
                        "check": "missing_title", "severity": "error", "message": "Ticket has no title."}));
                }

                let state = t.state.as_deref().unwrap_or("");
                let has_unresolved = unresolved_deps.contains_key(&t.id);
                if state == "blocked" && !has_unresolved {
                    *summary.entry("blocked_but_resolved".into()).or_insert(0) += 1;
                    findings.push(json!({"ticket_id": t.id, "short_id": short_id, "title": title,
                        "check": "blocked_but_resolved", "severity": "warning",
                        "message": "Ticket is blocked but all dependencies are done — may be ready to unblock."}));
                }
                if has_unresolved && state != "blocked" && state != "open" {
                    let dep_count = unresolved_deps[&t.id].len();
                    *summary.entry("unblocked_with_deps".into()).or_insert(0) += 1;
                    findings.push(json!({"ticket_id": t.id, "short_id": short_id, "title": title,
                        "check": "unblocked_with_deps", "severity": "info",
                        "message": format!("Ticket is '{state}' but has {dep_count} unresolved dependency/ies — may need state review.")}));
                }

                for edge in &all_edges {
                    if edge.from == t.id && edge.kind == "depends_on" {
                        let target_exists = store.get_indexed(&edge.to).ok().flatten()
                            .map(|tgt| !tgt.deleted).unwrap_or(false);
                        if !target_exists {
                            let target_short = &edge.to.to_string()[..8];
                            *summary.entry("dangling_edge".into()).or_insert(0) += 1;
                            findings.push(json!({"ticket_id": t.id, "short_id": short_id, "title": title,
                                "check": "dangling_edge", "severity": "error",
                                "message": format!("depends_on edge points to {target_short} which is deleted or missing.")}));
                        }
                    }
                }
            }

            let total_checked = tickets.iter().filter(|t| !done_ids.contains(&t.id)).count();
            Ok(json!({
                "command": "health",
                "status": "ok",
                "tickets_checked": total_checked,
                "finding_count": findings.len(),
                "summary": summary,
                "findings": findings,
            }))
        }
        other => Err(CliRunError::InvalidExecPayload(format!("unknown command: {other}"))),
    }
}

struct SimulatedProvider;

impl SubagentProvider for SimulatedProvider {
    fn start_subagent(
        &self,
        request: &ticket_api::execution::provider::StartSubagentRequest,
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

fn req_str<'a>(cmd: &'a Value, field: &str) -> Result<&'a str, CliRunError> {
    cmd.get(field)
        .and_then(|v| v.as_str())
        .ok_or_else(|| CliRunError::InvalidExecPayload(format!("missing required field '{field}'")))
}
