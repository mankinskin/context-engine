use serde_json::{Value, json};

use ticket_api::storage::TicketStore;

use crate::cli::{ClaimArgs, CloseArgs, CliRunError, IdArgs, UnclaimArgs};

pub(crate) fn cmd_close(args: CloseArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let (manifest, path) = store.close(&args.id, &args.to_state)?;
    let title = manifest.extra.get("title").and_then(Value::as_str).unwrap_or("-");
    Ok(json!({
        "command": "close",
        "status": "ok",
        "id": manifest.id,
        "title": title,
        "target_state": args.to_state,
        "traversed_states": path,
    }))
}

pub(crate) fn cmd_cancel(args: IdArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let (manifest, path) = store.close(&args.id, "cancelled")?;
    let title = manifest.extra.get("title").and_then(Value::as_str).unwrap_or("-");
    Ok(json!({
        "command": "cancel",
        "status": "ok",
        "id": manifest.id,
        "title": title,
        "traversed_states": path,
    }))
}

pub(crate) fn cmd_claim(args: ClaimArgs, store: &TicketStore) -> Result<Value, CliRunError> {
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

pub(crate) fn cmd_unclaim(args: UnclaimArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let manifest = store.get(&args.id)?;
    let title = manifest.extra.get("title").and_then(Value::as_str).unwrap_or("-");
    store.unclaim(&args.id)?;
    Ok(json!({
        "command": "unclaim",
        "status": "ok",
        "id": args.id,
        "title": title,
        "reason": args.reason,
    }))
}

pub(crate) fn cmd_leases(store: &TicketStore) -> Result<Value, CliRunError> {
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
