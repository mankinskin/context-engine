use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};
use uuid::Uuid;

use viewer_api::error::RequestIdExt;
use crate::serve::{error::storage_err, AppState};

#[derive(Deserialize)]
pub struct SubgraphQuery {
    pub workspace: String,
    pub root: Uuid,
    pub direction: Option<String>,
    pub edge_kind: Option<String>,
    #[serde(default = "default_depth")]
    pub depth: usize,
    #[serde(default = "default_limit_nodes")]
    pub limit_nodes: usize,
    #[serde(default = "default_limit_edges")]
    pub limit_edges: usize,
}

fn default_depth() -> usize { 2 }
fn default_limit_nodes() -> usize { 500 }
fn default_limit_edges() -> usize { 2000 }

#[derive(Serialize)]
pub struct NodeItem {
    pub id: String,
    pub title: Option<String>,
    pub state: Option<String>,
    pub depth: usize,
}

#[derive(Serialize)]
pub struct EdgeItem {
    pub from: String,
    pub to: String,
    pub kind: String,
}

#[derive(Serialize)]
pub struct SubgraphStats {
    pub nodes_returned: usize,
    pub edges_returned: usize,
    pub max_depth_reached: usize,
}

#[derive(Serialize)]
pub struct SubgraphResponse {
    pub request_id: String,
    pub workspace: String,
    pub nodes: Vec<NodeItem>,
    pub edges: Vec<EdgeItem>,
    pub truncated: bool,
    pub next_cursor: Option<String>,
    pub stats: SubgraphStats,
}

pub async fn subgraph(
    State(state): State<AppState>,
    Extension(rid): Extension<RequestIdExt>,
    Query(params): Query<SubgraphQuery>,
) -> Response {
    let store = match state.ensure_workspace_runtime(&params.workspace) {
        Some(s) => s,
        None => {
            return viewer_api::error::ApiError::not_found("workspace", &rid.0)
                .into_response_with_status(StatusCode::NOT_FOUND);
        }
    };

    let depth_limit = params.depth.min(8);
    let direction = params.direction.as_deref().unwrap_or("both");
    let edge_kind_filter = params.edge_kind.as_deref().unwrap_or("all");

    // BFS traversal
    let mut visited_nodes: HashSet<Uuid> = HashSet::new();
    let mut nodes: Vec<NodeItem> = Vec::new();
    let mut edges_set: Vec<EdgeItem> = Vec::new();
    let mut truncated = false;
    let mut max_depth_reached = 0;

    // Queue: (id, current_depth)
    let mut queue: VecDeque<(Uuid, usize)> = VecDeque::new();
    queue.push_back((params.root, 0));

    while let Some((current_id, depth)) = queue.pop_front() {
        if visited_nodes.contains(&current_id) {
            continue;
        }
        if nodes.len() >= params.limit_nodes {
            truncated = true;
            break;
        }

        visited_nodes.insert(current_id);
        max_depth_reached = max_depth_reached.max(depth);

        // Get ticket summary
        let summary = match store.get_indexed(&current_id) {
            Ok(Some(t)) => NodeItem {
                id: current_id.to_string(),
                title: t.title,
                state: t.state,
                depth,
            },
            Ok(None) => NodeItem {
                id: current_id.to_string(),
                title: None,
                state: None,
                depth,
            },
            Err(e) => return storage_err(e, &rid.0),
        };
        nodes.push(summary);

        if depth >= depth_limit {
            continue;
        }

        // Expand edges
        let all_edges = match store.list_all_edges() {
            Ok(e) => e,
            Err(e) => return storage_err(e, &rid.0),
        };

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
                continue
            };

            let dir_ok = match direction {
                "out" => is_outbound,
                "in" => !is_outbound,
                _ => true, // "both"
            };
            if !dir_ok {
                continue;
            }

            if edges_set.len() < params.limit_edges {
                edges_set.push(EdgeItem {
                    from: edge.from.to_string(),
                    to: edge.to.to_string(),
                    kind: edge.kind.clone(),
                });
            }

            if !visited_nodes.contains(&neighbor) {
                queue.push_back((neighbor, depth + 1));
            }
        }
    }

    // Deduplicate edges
    edges_set.dedup_by(|a, b| a.from == b.from && a.to == b.to && a.kind == b.kind);

    let stats = SubgraphStats {
        nodes_returned: nodes.len(),
        edges_returned: edges_set.len(),
        max_depth_reached,
    };

    Json(SubgraphResponse {
        request_id: rid.0,
        workspace: params.workspace,
        nodes,
        edges: edges_set,
        truncated,
        next_cursor: None,
        stats,
    })
    .into_response()
}
