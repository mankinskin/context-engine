use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};

use chrono::Utc;
use serde_json::{Value, json};
use uuid::Uuid;

use ticket_api::error::StorageError;
use ticket_api::storage::TicketStore;

use crate::cli::{
    AddRootArgs, AttachArgs, CliRunError, IdArgs, ReadyOverviewArgs, ScanArgs, ServeCliArgs,
    StatusArgs, WatchArgs,
};

pub(crate) fn cmd_scan(args: ScanArgs, store: &TicketStore) -> Result<Value, CliRunError> {
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

pub(crate) fn cmd_attach(args: AttachArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let id = super::resolve_uuid_prefix(&args.id, store)?;
    let dest = store.attach(&id, &args.path, args.asset_name.as_deref())?;
    let title = store.get(&id).ok()
        .and_then(|m| m.extra.get("title").and_then(Value::as_str).map(String::from))
        .unwrap_or_else(|| "-".to_string());
    Ok(json!({
        "command": "attach",
        "status": "ok",
        "id": id,
        "title": title,
        "asset_path": dest.display().to_string(),
    }))
}

pub(crate) fn cmd_assets(args: IdArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let id = super::resolve_uuid_prefix(&args.id, store)?;
    let names = store.list_assets(&id)?;
    Ok(json!({
        "command": "assets",
        "status": "ok",
        "id": id,
        "count": names.len(),
        "assets": names,
    }))
}

pub(crate) fn cmd_audit(store: &TicketStore) -> Result<Value, CliRunError> {
    let all = store.list(None, None, None)?;
    let deleted = store
        .list_extended(None, None, None, true, &[])?
        .into_iter()
        .filter(|t| t.deleted)
        .count();
    let total = all.len() + deleted;

    let mut state_counts = BTreeMap::new();
    for t in &all {
        let state = t.state.as_deref().unwrap_or("unknown");
        *state_counts.entry(state.to_string()).or_insert(0usize) += 1;
    }

    let mut type_counts = BTreeMap::new();
    for t in &all {
        *type_counts.entry(t.type_id.clone()).or_insert(0usize) += 1;
    }

    Ok(json!({
        "command": "audit",
        "status": "ok",
        "total": total,
        "active": all.len(),
        "deleted": deleted,
        "by_state": state_counts,
        "by_type": type_counts,
    }))
}

pub(crate) fn cmd_add_root(args: AddRootArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    use ticket_api::model::filesystem::ScanRoot;
    let path = args.path.canonicalize().unwrap_or(args.path.clone());
    std::fs::create_dir_all(&path).map_err(StorageError::Io)?;
    store.add_scan_root(ScanRoot {
        path: path.clone(),
        label: args.label.clone(),
    })?;
    Ok(json!({
        "command": "add_root",
        "status": "ok",
        "path": path,
        "label": args.label,
    }))
}

pub(crate) fn cmd_serve(args: ServeCliArgs, store: TicketStore) -> Result<Value, CliRunError> {
    use ticket_api::workspace::WorkspaceConfig;
    use ticket_http::serve::{ServeConfig, WorkspaceRegistry, serve};

    let registry = if args.workspace.is_some() {
        WorkspaceRegistry::single_opened(std::sync::Arc::new(store))
    } else {
        let config = WorkspaceConfig::load();
        if config.workspaces.is_empty() {
            WorkspaceRegistry::single_opened(std::sync::Arc::new(store))
        } else {
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

    Err(CliRunError::BadRequest("server exited unexpectedly".into()))
}

pub(crate) fn cmd_watch(args: WatchArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    use ticket_api::watcher::reconciler::{run_watch_loop, start_watcher};
    eprintln!(
        "Starting filesystem watcher (debounce={}ms). Press Ctrl+C to stop.",
        args.debounce_ms
    );
    let handle = start_watcher(store).map_err(CliRunError::Storage)?;
    run_watch_loop(&handle, store, args.debounce_ms);
    Ok(json!({ "command": "watch", "status": "stopped" }))
}

const DONE_STATES: &[&str] = &["done", "cancelled"];
const ACTIVE_STATES: &[&str] = &[
    "in-progress",
    "review",
    "validating",
    "validated",
    "release-candidate",
    "monitoring",
];

pub(crate) fn cmd_status(args: StatusArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let all = store.list(None, None, None)?;

    let tickets: Vec<_> = if let Some(ref prefix) = args.filter {
        all.into_iter()
            .filter(|t| {
                t.title
                    .as_deref()
                    .unwrap_or("")
                    .starts_with(prefix.as_str())
            })
            .collect()
    } else {
        all
    };

    let done_ids: HashSet<Uuid> = tickets
        .iter()
        .filter(|t| {
            t.state
                .as_deref()
                .map(|s| DONE_STATES.contains(&s))
                .unwrap_or(false)
        })
        .map(|t| t.id)
        .collect();

    let all_edges = store.list_all_edges()?;
    let mut blockers: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for edge in &all_edges {
        if edge.kind == "depends_on" && !done_ids.contains(&edge.to) {
            blockers.entry(edge.from).or_default().push(edge.to);
        }
    }

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
            "component": t.type_id,
        });

        if is_active {
            active.push(entry);
        } else if is_blocked {
            if args.show_blocked {
                let dep_entries: Vec<Value> = unresolved
                    .iter()
                    .map(|dep_id| {
                        let title = tickets
                            .iter()
                            .find(|t| t.id == *dep_id)
                            .and_then(|t| t.title.clone())
                            .unwrap_or_else(|| dep_id.to_string());
                        let dep_state = tickets
                            .iter()
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
            ready.push(entry);
        }
    }

    let mut by_component: HashMap<String, Vec<&Value>> = HashMap::new();
    for entry in &ready {
        let comp = entry["component"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();
        by_component.entry(comp).or_default().push(entry);
    }

    let parallel_groups: Vec<Value> = by_component
        .into_iter()
        .map(|(component, entries)| {
            json!({
                "component": component,
                "count": entries.len(),
                "tickets": entries
            })
        })
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

pub(crate) fn cmd_ready_overview(
    args: ReadyOverviewArgs,
    store: &TicketStore,
) -> Result<Value, CliRunError> {
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
