use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write;

use chrono::Utc;
use serde_json::{Value, json};
use uuid::Uuid;

use ticket_api::model::edge::EdgeRecord;
use ticket_api::storage::TicketStore;

use crate::cli::{CliRunError, LinkArgs, LinksArgs, SubgraphArgs, UnlinkArgs};

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

pub(crate) fn cmd_links(args: LinksArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let raw_edges = if args.all {
        store.list_all_edges()?
    } else {
        let id = args.id.expect("clap ensures --id is present when --all is not set");
        store.edges_from(&id)?
    };

    let items: Vec<Value> = raw_edges
        .iter()
        .filter(|e| match &args.kind {
            Some(k) => e.kind == *k,
            None => true,
        })
        .map(|e| json!({ "from": e.from, "to": e.to, "kind": e.kind }))
        .collect();
    Ok(json!({
        "command": "links",
        "status": "ok",
        "id": args.id,
        "all": args.all,
        "count": items.len(),
        "edges": items,
    }))
}

// ── subgraph ───────────────────────────────────────────────────────────────────

pub(crate) fn cmd_subgraph(args: SubgraphArgs, store: &TicketStore) -> Result<Value, CliRunError> {
    let root = resolve_uuid_prefix(&args.root, store)?;
    let depth_limit = args.depth.min(8);
    let direction = args.direction.as_str();
    let edge_kind_filter = args.edge_kind.as_str();

    let all_edges = store.list_all_edges()?;

    // BFS traversal
    let mut visited: HashSet<Uuid> = HashSet::new();
    let mut node_list: Vec<(Uuid, Option<String>, Option<String>, usize)> = Vec::new();
    let mut collected_edges: Vec<&EdgeRecord> = Vec::new();
    let mut queue: VecDeque<(Uuid, usize)> = VecDeque::new();
    queue.push_back((root, 0));

    while let Some((current_id, depth)) = queue.pop_front() {
        if !visited.insert(current_id) {
            continue;
        }

        let (title, state) = match store.get_indexed(&current_id)? {
            Some(t) if !t.deleted => (t.title, t.state),
            _ => (None, None),
        };
        node_list.push((current_id, title, state, depth));

        if depth >= depth_limit {
            continue;
        }

        for edge in &all_edges {
            let kind_ok = edge_kind_filter == "all" || edge.kind == edge_kind_filter;
            if !kind_ok {
                continue;
            }

            let (neighbor, is_outbound) = if edge.from == current_id {
                (edge.to, true)
            } else if edge.to == current_id {
                (edge.from, false)
            } else {
                continue;
            };

            let dir_ok = match direction {
                "out" => is_outbound,
                "in" => !is_outbound,
                _ => true,
            };
            if !dir_ok {
                continue;
            }

            collected_edges.push(edge);

            if !visited.contains(&neighbor) {
                queue.push_back((neighbor, depth + 1));
            }
        }
    }

    // Deduplicate edges
    let mut seen_edges: HashSet<(Uuid, Uuid, &str)> = HashSet::new();
    let unique_edges: Vec<&EdgeRecord> = collected_edges
        .into_iter()
        .filter(|e| seen_edges.insert((e.from, e.to, &e.kind)))
        .collect();

    // Build JSON nodes/edges for --json output
    let json_nodes: Vec<Value> = node_list
        .iter()
        .map(|(id, title, state, depth)| {
            json!({
                "id": id.to_string(),
                "title": title,
                "state": state,
                "depth": depth,
            })
        })
        .collect();

    let json_edges: Vec<Value> = unique_edges
        .iter()
        .map(|e| json!({ "from": e.from.to_string(), "to": e.to.to_string(), "kind": &e.kind }))
        .collect();

    // Build ASCII tree for plain output
    let tree = render_ascii_tree(root, &node_list, &unique_edges);

    Ok(json!({
        "command": "subgraph",
        "status": "ok",
        "tree": tree,
        "nodes": json_nodes,
        "edges": json_edges,
        "truncated": false,
        "stats": {
            "nodes_returned": json_nodes.len(),
            "edges_returned": json_edges.len(),
        },
    }))
}

/// Resolve a UUID string that may be a full UUID or a hex prefix (>= 8 chars).
fn resolve_uuid_prefix(s: &str, store: &TicketStore) -> Result<Uuid, CliRunError> {
    if let Ok(id) = s.parse::<Uuid>() {
        return Ok(id);
    }

    let trimmed = s.trim();
    if trimmed.len() >= 8 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        let tickets = store.list(None, None, None)?;
        let prefix_lower = trimmed.to_ascii_lowercase();
        let matches: Vec<Uuid> = tickets
            .iter()
            .filter(|t| t.id.simple().to_string().starts_with(&prefix_lower))
            .map(|t| t.id)
            .collect();

        return match matches.len() {
            1 => Ok(matches[0]),
            0 => Err(CliRunError::BadRequest(format!(
                "no ticket found matching prefix '{trimmed}'"
            ))),
            n => Err(CliRunError::BadRequest(format!(
                "ambiguous prefix '{trimmed}': matches {n} tickets"
            ))),
        };
    }

    Err(CliRunError::BadRequest(format!(
        "invalid UUID '{s}': expected full UUID or hex prefix (>= 8 chars)"
    )))
}

/// Render an ASCII dependency tree from BFS-collected nodes and edges.
fn render_ascii_tree(
    root: Uuid,
    nodes: &[(Uuid, Option<String>, Option<String>, usize)],
    edges: &[&EdgeRecord],
) -> String {
    // Build lookup: id -> (title, state)
    let node_info: HashMap<Uuid, (&Option<String>, &Option<String>)> = nodes
        .iter()
        .map(|(id, title, state, _)| (*id, (title, state)))
        .collect();

    // Build adjacency: parent -> [(kind, child)]
    let mut children: HashMap<Uuid, Vec<(&str, Uuid)>> = HashMap::new();
    for edge in edges {
        children.entry(edge.from).or_default().push((&edge.kind, edge.to));
    }

    let mut out = String::new();
    let short_id = &root.to_string()[..8];
    let (title, state) = node_info
        .get(&root)
        .map(|(t, s)| (t.as_deref().unwrap_or("?"), s.as_deref().unwrap_or("?")))
        .unwrap_or(("?", "?"));
    let _ = writeln!(out, "{title} ({short_id}) [{state}]");

    // Track visited to handle diamond dependencies
    let mut visited = HashSet::new();
    visited.insert(root);

    render_children(&mut out, &mut visited, root, &children, &node_info, "");
    out
}

fn render_children(
    out: &mut String,
    visited: &mut HashSet<Uuid>,
    parent: Uuid,
    children: &HashMap<Uuid, Vec<(&str, Uuid)>>,
    node_info: &HashMap<Uuid, (&Option<String>, &Option<String>)>,
    prefix: &str,
) {
    let Some(kids) = children.get(&parent) else {
        return;
    };

    for (i, (kind, child_id)) in kids.iter().enumerate() {
        let is_last = i == kids.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last { "    " } else { "│   " };

        let short_id = &child_id.to_string()[..8];
        let (title, state) = node_info
            .get(child_id)
            .map(|(t, s)| (t.as_deref().unwrap_or("?"), s.as_deref().unwrap_or("?")))
            .unwrap_or(("?", "?"));

        let already_visited = !visited.insert(*child_id);
        if already_visited {
            // Show the node but don't recurse (diamond reference)
            let _ = writeln!(out, "{prefix}{connector}{kind} → {title} ({short_id}) [{state}] (→ see above)");
        } else {
            let _ = writeln!(out, "{prefix}{connector}{kind} → {title} ({short_id}) [{state}]");
            let next_prefix = format!("{prefix}{child_prefix}");
            render_children(out, visited, *child_id, children, node_info, &next_prefix);
        }
    }
}
