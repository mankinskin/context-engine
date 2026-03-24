use chrono::Utc;
use serde_json::{Value, json};

use ticket_api::model::edge::EdgeRecord;
use ticket_api::storage::TicketStore;

use crate::cli::{CliRunError, IdArgs, LinkArgs, UnlinkArgs};

pub(crate) fn cmd_link(args: LinkArgs, store: &TicketStore) -> Result<Value, CliRunError> {
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

pub(crate) fn cmd_unlink(args: UnlinkArgs, store: &TicketStore) -> Result<Value, CliRunError> {
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
