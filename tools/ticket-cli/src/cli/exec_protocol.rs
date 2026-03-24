use std::collections::BTreeMap;
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

pub(crate) fn cmd_batch(args: BatchArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    use std::fs::File;
    use std::io::{self, BufReader};

    let commands = if let Some(path) = args.file {
        let file = File::open(&path).map_err(|e| {
            CliRunError::InvalidExecPayload(format!("cannot open batch file {}: {e}", path.display()))
        })?;
        read_batch_commands(BufReader::new(file))?
    } else {
        let stdin = io::stdin();
        read_batch_commands(stdin.lock())?
    };

    execute_batch_commands(&commands, store)
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
