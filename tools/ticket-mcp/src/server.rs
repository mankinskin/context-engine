use std::collections::{BTreeMap, HashSet, VecDeque};
use std::sync::Arc;

use rmcp::{
    ErrorData as McpError, ServerHandler, ServiceExt,
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars::{self, JsonSchema},
    tool, tool_handler, tool_router,
    transport::stdio,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use ticket_api::storage::store::TicketStore;
use ticket_api::storage::ticket_fs::TicketFs;

// ── Output types ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct TicketSummary {
    id: String,
    #[serde(rename = "type")]
    type_id: String,
    title: Option<String>,
    state: Option<String>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
struct TicketDetail {
    id: String,
    created_at: chrono::DateTime<chrono::Utc>,
    fields: BTreeMap<String, Value>,
}

#[derive(Serialize)]
struct EdgeItem {
    from: String,
    to: String,
    kind: String,
}

#[derive(Serialize)]
struct NodeItem {
    id: String,
    title: Option<String>,
    state: Option<String>,
    depth: usize,
}

#[derive(Serialize)]
struct SubgraphResponse {
    workspace: String,
    nodes: Vec<NodeItem>,
    edges: Vec<EdgeItem>,
    truncated: bool,
    stats: SubgraphStats,
}

#[derive(Serialize)]
struct SubgraphStats {
    nodes_returned: usize,
    edges_returned: usize,
    max_depth_reached: usize,
}

// ── Input types ──────────────────────────────────────────────────────────────

// ── Input types ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListTicketsInput {
    pub workspace: String,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TicketRefInput {
    pub workspace: String,
    pub id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListEdgesInput {
    pub workspace: String,
    #[serde(default)]
    pub kind: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SubgraphInput {
    pub workspace: String,
    pub root: String,
    #[serde(default)]
    pub direction: Option<String>,
    #[serde(default)]
    pub edge_kind: Option<String>,
    #[serde(default)]
    pub depth: Option<usize>,
    #[serde(default)]
    pub limit_nodes: Option<usize>,
    #[serde(default)]
    pub limit_edges: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowName {
    List,
    TriageOpenTickets,
    FetchTicketContext,
    InspectDependencies,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WorkflowInput {
    #[serde(default = "default_workflow_name")]
    pub name: WorkflowName,
    #[serde(default)]
    pub workspace: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub query: Option<String>,
}

fn default_workflow_name() -> WorkflowName {
    WorkflowName::List
}

// ── Server ───────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct TicketServer {
    store: Arc<TicketStore>,
    tool_router: ToolRouter<Self>,
}

impl TicketServer {
    pub fn new(store: Arc<TicketStore>) -> Self {
        Self {
            store,
            tool_router: Self::tool_router(),
        }
    }

    fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, McpError> {
        let text = serde_json::to_string_pretty(value)
            .map_err(|e| McpError::internal_error(format!("serialization: {e}"), None))?;
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    fn store_err(e: ticket_api::error::StorageError) -> McpError {
        McpError::internal_error(format!("store error: {e}"), None)
    }

    fn parse_uuid(s: &str) -> Result<Uuid, McpError> {
        s.parse::<Uuid>()
            .map_err(|e| McpError::invalid_params(format!("invalid UUID '{s}': {e}"), None))
    }
}

#[tool_router]
impl TicketServer {
    #[tool(name = "health", description = "Check that the ticket store is accessible.")]
    async fn health(&self) -> Result<CallToolResult, McpError> {
        match self.store.list(None, None, Some(0)) {
            Ok(_) => Self::json_result(&serde_json::json!({
                "status": "ok",
                "service": "ticket-mcp",
                "mode": "direct",
            })),
            Err(e) => Self::json_result(&serde_json::json!({
                "status": "error",
                "error": e.to_string(),
            })),
        }
    }

    #[tool(
        name = "list_workspaces",
        description = "List available ticket workspaces."
    )]
    async fn list_workspaces(&self) -> Result<CallToolResult, McpError> {
        let config = ticket_api::workspace::WorkspaceConfig::load();
        let names: Vec<String> = if config.workspaces.is_empty() {
            vec!["default".to_string()]
        } else {
            config.workspaces.keys().cloned().collect()
        };
        Self::json_result(&serde_json::json!({
            "workspaces": names,
            "active": config.active,
        }))
    }

    #[tool(
        name = "list_tickets",
        description = "List tickets with optional state/query/limit filters."
    )]
    async fn list_tickets(
        &self,
        Parameters(input): Parameters<ListTicketsInput>,
    ) -> Result<CallToolResult, McpError> {
        if let Some(q) = &input.query {
            let limit = input.limit.unwrap_or(100).min(1000);
            let results = self.store.search_tickets(q, limit).map_err(Self::store_err)?;
            let items: Vec<TicketSummary> = results
                .into_iter()
                .map(|r| {
                    let updated_at = self
                        .store
                        .get_indexed(&r.id)
                        .ok()
                        .flatten()
                        .map(|t| t.updated_at)
                        .unwrap_or_else(|| chrono::DateTime::<chrono::Utc>::from(std::time::SystemTime::UNIX_EPOCH));
                    TicketSummary {
                        id: r.id.to_string(),
                        type_id: r.ticket_type.unwrap_or_default(),
                        title: r.title,
                        state: r.state,
                        updated_at,
                    }
                })
                .collect();
            Self::json_result(&serde_json::json!({
                "workspace": input.workspace,
                "items": items,
            }))
        } else {
            let limit = input.limit.map(|l| l.min(1000));
            let items: Vec<TicketSummary> = self
                .store
                .list(input.state.as_deref(), None, limit)
                .map_err(Self::store_err)?
                .into_iter()
                .map(|t| TicketSummary {
                    id: t.id.to_string(),
                    type_id: t.type_id,
                    title: t.title,
                    state: t.state,
                    updated_at: t.updated_at,
                })
                .collect();
            Self::json_result(&serde_json::json!({
                "workspace": input.workspace,
                "items": items,
            }))
        }
    }

    #[tool(name = "get_ticket", description = "Get one ticket by id.")]
    async fn get_ticket(
        &self,
        Parameters(input): Parameters<TicketRefInput>,
    ) -> Result<CallToolResult, McpError> {
        let id = Self::parse_uuid(&input.id)?;
        let manifest = self.store.get(&id).map_err(Self::store_err)?;
        Self::json_result(&serde_json::json!({
            "workspace": input.workspace,
            "ticket": TicketDetail {
                id: manifest.id.to_string(),
                created_at: manifest.created_at,
                fields: manifest.extra,
            },
        }))
    }

    #[tool(
        name = "get_ticket_description",
        description = "Get ticket markdown description by id."
    )]
    async fn get_ticket_description(
        &self,
        Parameters(input): Parameters<TicketRefInput>,
    ) -> Result<CallToolResult, McpError> {
        let id = Self::parse_uuid(&input.id)?;
        let indexed = self
            .store
            .get_indexed(&id)
            .map_err(Self::store_err)?
            .ok_or_else(|| McpError::invalid_params(format!("ticket not found: {id}"), None))?;

        if indexed.deleted {
            return Err(McpError::invalid_params(format!("ticket deleted: {id}"), None));
        }

        let description = TicketFs::read_description(&indexed.path);
        Self::json_result(&serde_json::json!({
            "workspace": input.workspace,
            "id": id.to_string(),
            "description": description,
        }))
    }

    #[tool(
        name = "list_edges",
        description = "List ticket graph edges, optionally filtered by edge kind."
    )]
    async fn list_edges(
        &self,
        Parameters(input): Parameters<ListEdgesInput>,
    ) -> Result<CallToolResult, McpError> {
        let all = self.store.list_all_edges().map_err(Self::store_err)?;
        let items: Vec<EdgeItem> = all
            .into_iter()
            .filter(|e| match &input.kind {
                Some(k) => k == "all" || e.kind == *k,
                None => true,
            })
            .map(|e| EdgeItem {
                from: e.from.to_string(),
                to: e.to.to_string(),
                kind: e.kind,
            })
            .collect();
        Self::json_result(&serde_json::json!({
            "workspace": input.workspace,
            "items": items,
        }))
    }

    #[tool(
        name = "subgraph",
        description = "Fetch dependency subgraph for a root ticket via BFS traversal."
    )]
    async fn subgraph(
        &self,
        Parameters(input): Parameters<SubgraphInput>,
    ) -> Result<CallToolResult, McpError> {
        let root = Self::parse_uuid(&input.root)?;
        let depth_limit = input.depth.unwrap_or(2).min(8);
        let node_limit = input.limit_nodes.unwrap_or(500);
        let edge_limit = input.limit_edges.unwrap_or(2000);
        let direction = input.direction.as_deref().unwrap_or("both");
        let edge_kind_filter = input.edge_kind.as_deref().unwrap_or("all");

        let mut visited: HashSet<Uuid> = HashSet::new();
        let mut nodes: Vec<NodeItem> = Vec::new();
        let mut edges: Vec<EdgeItem> = Vec::new();
        let mut truncated = false;
        let mut max_depth_reached = 0;

        let mut queue: VecDeque<(Uuid, usize)> = VecDeque::new();
        queue.push_back((root, 0));

        while let Some((current_id, depth)) = queue.pop_front() {
            if visited.contains(&current_id) {
                continue;
            }
            if nodes.len() >= node_limit {
                truncated = true;
                break;
            }

            visited.insert(current_id);
            max_depth_reached = max_depth_reached.max(depth);

            let node = match self.store.get_indexed(&current_id) {
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
                Err(e) => return Err(Self::store_err(e)),
            };
            nodes.push(node);

            if depth >= depth_limit {
                continue;
            }

            let all_edges = self.store.list_all_edges().map_err(Self::store_err)?;

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

                if edges.len() < edge_limit {
                    edges.push(EdgeItem {
                        from: edge.from.to_string(),
                        to: edge.to.to_string(),
                        kind: edge.kind.clone(),
                    });
                }

                if !visited.contains(&neighbor) {
                    queue.push_back((neighbor, depth + 1));
                }
            }
        }

        edges.dedup_by(|a, b| a.from == b.from && a.to == b.to && a.kind == b.kind);

        let stats = SubgraphStats {
            nodes_returned: nodes.len(),
            edges_returned: edges.len(),
            max_depth_reached,
        };
        Self::json_result(&SubgraphResponse {
            workspace: input.workspace,
            nodes,
            edges,
            truncated,
            stats,
        })
    }

    #[tool(
        name = "workflow",
        description = "Show ready-to-run ticket MCP call sequences for common tasks."
    )]
    async fn workflow(
        &self,
        Parameters(input): Parameters<WorkflowInput>,
    ) -> Result<CallToolResult, McpError> {
        let workspace = input.workspace.unwrap_or_else(|| "default".to_string());
        let id = input.id.unwrap_or_else(|| "<ticket-id>".to_string());
        let query = input.query.unwrap_or_else(|| "<query>".to_string());

        let payload = match input.name {
            WorkflowName::List => serde_json::json!({
                "available": [
                    "triage_open_tickets",
                    "fetch_ticket_context",
                    "inspect_dependencies"
                ],
                "note": "Use one of the named workflows to get an ordered sequence of tool calls."
            }),
            WorkflowName::TriageOpenTickets => serde_json::json!({
                "name": "triage_open_tickets",
                "steps": [
                    {"tool": "health", "input": {}},
                    {"tool": "list_workspaces", "input": {}},
                    {"tool": "list_tickets", "input": {"workspace": workspace, "state": "open", "limit": 50}},
                    {"tool": "list_tickets", "input": {"workspace": workspace, "state": "in-progress", "limit": 50}}
                ]
            }),
            WorkflowName::FetchTicketContext => serde_json::json!({
                "name": "fetch_ticket_context",
                "steps": [
                    {"tool": "get_ticket", "input": {"workspace": workspace, "id": id}},
                    {"tool": "get_ticket_description", "input": {"workspace": workspace, "id": id}},
                    {"tool": "list_edges", "input": {"workspace": workspace}},
                    {"tool": "subgraph", "input": {"workspace": workspace, "root": id, "depth": 2}}
                ]
            }),
            WorkflowName::InspectDependencies => serde_json::json!({
                "name": "inspect_dependencies",
                "steps": [
                    {"tool": "list_tickets", "input": {"workspace": workspace, "query": query, "limit": 20}},
                    {"tool": "list_edges", "input": {"workspace": workspace, "kind": "depends_on"}},
                    {"tool": "subgraph", "input": {"workspace": workspace, "root": id, "direction": "both", "depth": 3}}
                ]
            }),
        };

        Self::json_result(&payload)
    }

    #[tool(
        name = "help",
        description = "List ticket-mcp tools and their parameters."
    )]
    async fn help(&self) -> Result<CallToolResult, McpError> {
        let payload = serde_json::json!({
            "mode": "direct (no HTTP backend required)",
            "tools": [
                "health",
                "list_workspaces",
                "list_tickets",
                "get_ticket",
                "get_ticket_description",
                "list_edges",
                "subgraph",
                "workflow",
            ],
            "operations": {
                "health": {
                    "description": "Check store is accessible",
                    "required": [],
                },
                "list_workspaces": {
                    "description": "List available workspaces",
                    "required": [],
                },
                "list_tickets": {
                    "description": "List/search tickets",
                    "required": ["workspace"],
                    "optional": ["state", "query", "limit"],
                },
                "get_ticket": {
                    "description": "Get full ticket manifest",
                    "required": ["workspace", "id"],
                },
                "get_ticket_description": {
                    "description": "Get ticket markdown description",
                    "required": ["workspace", "id"],
                },
                "list_edges": {
                    "description": "List graph edges",
                    "required": ["workspace"],
                    "optional": ["kind"],
                },
                "subgraph": {
                    "description": "BFS dependency subgraph",
                    "required": ["workspace", "root"],
                    "optional": ["direction", "edge_kind", "depth", "limit_nodes", "limit_edges"],
                },
            },
            "notes": [
                "Direct store access — no HTTP backend required.",
                "Set TICKET_INDEX_ROOT to override workspace resolution.",
            ],
        });
        Self::json_result(&payload)
    }
}

#[tool_handler]
impl ServerHandler for TicketServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "ticket-mcp provides direct access to the ticket store. No HTTP backend required. Use named tools for ticket operations."
                    .to_string(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

pub async fn run_mcp_server(
    store: Arc<TicketStore>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server = TicketServer::new(store);

    tracing::info!("Starting ticket-mcp server on stdio (direct store access)");

    let service = server.serve(stdio()).await.inspect_err(|err| {
        eprintln!("Server error: {err:?}");
    })?;

    service.waiting().await?;
    Ok(())
}
