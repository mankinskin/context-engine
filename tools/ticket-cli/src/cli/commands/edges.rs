use chrono::Utc;
use serde_json::{Value, json};

use ticket_api::model::edge::EdgeRecord;
use ticket_api::storage::TicketStore;

use crate::cli::{CliRunError, IdArgs, LinkArgs, UnlinkArgs};

pub(crate) fn cmd_link(args: LinkArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let from_title = store.get(&args.from).ok()
        .and_then(|m| m.extra.get("title").and_then(Value::as_str).map(String::from))
        .unwrap_or_else(|| args.from.to_string());
    let to_title = store.get(&args.to).ok()
        .and_then(|m| m.extra.get("title").and_then(Value::as_str).map(String::from))
        .unwrap_or_else(|| args.to.to_string());
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
        "from_title": from_title,
        "to": args.to,
        "to_title": to_title,
        "kind": args.kind,
        "reason": args.reason,
    }))
}

pub(crate) fn cmd_unlink(args: UnlinkArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let from_title = store.get(&args.from).ok()
        .and_then(|m| m.extra.get("title").and_then(Value::as_str).map(String::from))
        .unwrap_or_else(|| args.from.to_string());
    let to_title = store.get(&args.to).ok()
        .and_then(|m| m.extra.get("title").and_then(Value::as_str).map(String::from))
        .unwrap_or_else(|| args.to.to_string());
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
        "from_title": from_title,
        "to": args.to,
        "to_title": to_title,
        "kind": args.kind,
        "reason": args.reason,
    }))
}

pub(crate) fn cmd_links(args: IdArgs, store: &TicketStore) -> Result<Value, CliRunError> {
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
