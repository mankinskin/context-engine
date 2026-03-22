use reqwest::Client;
use rmcp::{
    ErrorData as McpError, ServerHandler, ServiceExt,
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars::{self, JsonSchema},
    tool, tool_handler, tool_router,
    transport::stdio,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TicketOperation {
    Health,
    ListWorkspaces,
    ListTickets,
    GetTicket,
    GetTicketDescription,
    ListEdges,
    Subgraph,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RequestInput {
    pub operation: TicketOperation,
    #[serde(default)]
    pub workspace: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub root: Option<String>,
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
    #[serde(default)]
    pub base_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestOutput {
    pub success: bool,
    pub url: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct HealthInput {
    #[serde(default)]
    pub base_url: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListWorkspacesInput {
    #[serde(default)]
    pub base_url: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListTicketsInput {
    pub workspace: String,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub base_url: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TicketRefInput {
    pub workspace: String,
    pub id: String,
    #[serde(default)]
    pub base_url: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListEdgesInput {
    pub workspace: String,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
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
    #[serde(default)]
    pub base_url: Option<String>,
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
    #[serde(default)]
    pub base_url: Option<String>,
}

fn default_workflow_name() -> WorkflowName {
    WorkflowName::List
}

#[derive(Clone)]
pub struct TicketServer {
    default_base_url: String,
    client: Client,
    tool_router: ToolRouter<Self>,
}

impl TicketServer {
    pub fn new(default_base_url: String) -> Self {
        Self {
            default_base_url: default_base_url.trim_end_matches('/').to_string(),
            client: Client::new(),
            tool_router: Self::tool_router(),
        }
    }

    fn resolve_base_url(&self, override_base_url: Option<String>) -> String {
        override_base_url
            .map(|v| v.trim_end_matches('/').to_string())
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| self.default_base_url.clone())
    }

    fn required<'a>(value: &'a Option<String>, name: &str) -> Result<&'a str, String> {
        value
            .as_deref()
            .filter(|v| !v.is_empty())
            .ok_or_else(|| format!("missing required field: {name}"))
    }

    fn query_pairs(input: &RequestInput) -> Result<Vec<(String, String)>, String> {
        let mut pairs = Vec::new();

        match input.operation {
            TicketOperation::Health | TicketOperation::ListWorkspaces => {}
            TicketOperation::ListTickets => {
                pairs.push((
                    "workspace".to_string(),
                    Self::required(&input.workspace, "workspace")?.to_string(),
                ));
                if let Some(state) = &input.state {
                    pairs.push(("state".to_string(), state.clone()));
                }
                if let Some(query) = &input.query {
                    pairs.push(("query".to_string(), query.clone()));
                }
                if let Some(limit) = input.limit {
                    pairs.push(("limit".to_string(), limit.to_string()));
                }
            }
            TicketOperation::GetTicket | TicketOperation::GetTicketDescription => {
                pairs.push((
                    "workspace".to_string(),
                    Self::required(&input.workspace, "workspace")?.to_string(),
                ));
            }
            TicketOperation::ListEdges => {
                pairs.push((
                    "workspace".to_string(),
                    Self::required(&input.workspace, "workspace")?.to_string(),
                ));
                if let Some(kind) = &input.kind {
                    pairs.push(("kind".to_string(), kind.clone()));
                }
            }
            TicketOperation::Subgraph => {
                pairs.push((
                    "workspace".to_string(),
                    Self::required(&input.workspace, "workspace")?.to_string(),
                ));
                pairs.push((
                    "root".to_string(),
                    Self::required(&input.root, "root")?.to_string(),
                ));
                if let Some(direction) = &input.direction {
                    pairs.push(("direction".to_string(), direction.clone()));
                }
                if let Some(edge_kind) = &input.edge_kind {
                    pairs.push(("edge_kind".to_string(), edge_kind.clone()));
                }
                if let Some(depth) = input.depth {
                    pairs.push(("depth".to_string(), depth.to_string()));
                }
                if let Some(limit_nodes) = input.limit_nodes {
                    pairs.push(("limit_nodes".to_string(), limit_nodes.to_string()));
                }
                if let Some(limit_edges) = input.limit_edges {
                    pairs.push(("limit_edges".to_string(), limit_edges.to_string()));
                }
            }
        }

        Ok(pairs)
    }

    fn path(input: &RequestInput) -> Result<String, String> {
        match input.operation {
            TicketOperation::Health => Ok("/healthz".to_string()),
            TicketOperation::ListWorkspaces => Ok("/api/workspaces".to_string()),
            TicketOperation::ListTickets => Ok("/api/tickets".to_string()),
            TicketOperation::GetTicket => {
                let id = Self::required(&input.id, "id")?;
                Ok(format!("/api/tickets/{id}"))
            }
            TicketOperation::GetTicketDescription => {
                let id = Self::required(&input.id, "id")?;
                Ok(format!("/api/tickets/{id}/description"))
            }
            TicketOperation::ListEdges => Ok("/api/edges".to_string()),
            TicketOperation::Subgraph => Ok("/api/graph/subgraph".to_string()),
        }
    }

    async fn perform_request(&self, input: RequestInput) -> Result<RequestOutput, String> {
        let base_url = self.resolve_base_url(input.base_url.clone());
        let path = Self::path(&input)?;
        let query = Self::query_pairs(&input)?;

        let mut url = format!("{base_url}{path}");
        if !query.is_empty() {
            let query_str = query
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}={}",
                        urlencoding::encode(k),
                        urlencoding::encode(v)
                    )
                })
                .collect::<Vec<_>>()
                .join("&");
            url = format!("{url}?{query_str}");
        }

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|err| format!("request failed: {err}"))?;

        let status = response.status().as_u16();
        let body = response
            .json::<Value>()
            .await
            .map_err(|err| format!("failed to parse JSON response: {err}"))?;

        let success = (200..300).contains(&status);
        let error = if success {
            None
        } else {
            body.get("error")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned)
                .or_else(|| Some(format!("HTTP status {status}")))
        };

        Ok(RequestOutput {
            success,
            url,
            status,
            result: Some(body),
            error,
        })
    }

    fn render_output(output: &RequestOutput) -> Result<CallToolResult, McpError> {
        let text = serde_json::to_string_pretty(output).map_err(|err| {
            McpError::internal_error(format!("serialization failed: {err}"), None)
        })?;
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    async fn request_from_input(&self, input: RequestInput) -> Result<CallToolResult, McpError> {
        match self.perform_request(input).await {
            Ok(output) => Self::render_output(&output),
            Err(err) => {
                let output = RequestOutput {
                    success: false,
                    url: String::new(),
                    status: 0,
                    result: None,
                    error: Some(err),
                };
                Self::render_output(&output)
            }
        }
    }
}

#[tool_router]
impl TicketServer {
    #[tool(
        name = "request",
        description = "Execute a thin ticket API request against ticket-http. This maps operation names to GET endpoints and forwards query parameters."
    )]
    async fn request(
        &self,
        Parameters(input): Parameters<RequestInput>,
    ) -> Result<CallToolResult, McpError> {
        self.request_from_input(input).await
    }

    #[tool(name = "health", description = "Check ticket API health endpoint.")]
    async fn health(
        &self,
        Parameters(input): Parameters<HealthInput>,
    ) -> Result<CallToolResult, McpError> {
        self.request_from_input(RequestInput {
            operation: TicketOperation::Health,
            workspace: None,
            id: None,
            state: None,
            query: None,
            limit: None,
            kind: None,
            root: None,
            direction: None,
            edge_kind: None,
            depth: None,
            limit_nodes: None,
            limit_edges: None,
            base_url: input.base_url,
        })
        .await
    }

    #[tool(
        name = "list_workspaces",
        description = "List available ticket workspaces from the ticket API."
    )]
    async fn list_workspaces(
        &self,
        Parameters(input): Parameters<ListWorkspacesInput>,
    ) -> Result<CallToolResult, McpError> {
        self.request_from_input(RequestInput {
            operation: TicketOperation::ListWorkspaces,
            workspace: None,
            id: None,
            state: None,
            query: None,
            limit: None,
            kind: None,
            root: None,
            direction: None,
            edge_kind: None,
            depth: None,
            limit_nodes: None,
            limit_edges: None,
            base_url: input.base_url,
        })
        .await
    }

    #[tool(
        name = "list_tickets",
        description = "List tickets for a workspace with optional state/query/limit filters."
    )]
    async fn list_tickets(
        &self,
        Parameters(input): Parameters<ListTicketsInput>,
    ) -> Result<CallToolResult, McpError> {
        self.request_from_input(RequestInput {
            operation: TicketOperation::ListTickets,
            workspace: Some(input.workspace),
            id: None,
            state: input.state,
            query: input.query,
            limit: input.limit,
            kind: None,
            root: None,
            direction: None,
            edge_kind: None,
            depth: None,
            limit_nodes: None,
            limit_edges: None,
            base_url: input.base_url,
        })
        .await
    }

    #[tool(
        name = "get_ticket",
        description = "Get one ticket by id from a workspace."
    )]
    async fn get_ticket(
        &self,
        Parameters(input): Parameters<TicketRefInput>,
    ) -> Result<CallToolResult, McpError> {
        self.request_from_input(RequestInput {
            operation: TicketOperation::GetTicket,
            workspace: Some(input.workspace),
            id: Some(input.id),
            state: None,
            query: None,
            limit: None,
            kind: None,
            root: None,
            direction: None,
            edge_kind: None,
            depth: None,
            limit_nodes: None,
            limit_edges: None,
            base_url: input.base_url,
        })
        .await
    }

    #[tool(
        name = "get_ticket_description",
        description = "Get ticket markdown description by id from a workspace."
    )]
    async fn get_ticket_description(
        &self,
        Parameters(input): Parameters<TicketRefInput>,
    ) -> Result<CallToolResult, McpError> {
        self.request_from_input(RequestInput {
            operation: TicketOperation::GetTicketDescription,
            workspace: Some(input.workspace),
            id: Some(input.id),
            state: None,
            query: None,
            limit: None,
            kind: None,
            root: None,
            direction: None,
            edge_kind: None,
            depth: None,
            limit_nodes: None,
            limit_edges: None,
            base_url: input.base_url,
        })
        .await
    }

    #[tool(
        name = "list_edges",
        description = "List ticket graph edges for a workspace, optionally filtered by edge kind."
    )]
    async fn list_edges(
        &self,
        Parameters(input): Parameters<ListEdgesInput>,
    ) -> Result<CallToolResult, McpError> {
        self.request_from_input(RequestInput {
            operation: TicketOperation::ListEdges,
            workspace: Some(input.workspace),
            id: None,
            state: None,
            query: None,
            limit: None,
            kind: input.kind,
            root: None,
            direction: None,
            edge_kind: None,
            depth: None,
            limit_nodes: None,
            limit_edges: None,
            base_url: input.base_url,
        })
        .await
    }

    #[tool(
        name = "subgraph",
        description = "Fetch dependency subgraph for a root ticket in a workspace."
    )]
    async fn subgraph(
        &self,
        Parameters(input): Parameters<SubgraphInput>,
    ) -> Result<CallToolResult, McpError> {
        self.request_from_input(RequestInput {
            operation: TicketOperation::Subgraph,
            workspace: Some(input.workspace),
            id: None,
            state: None,
            query: None,
            limit: None,
            kind: None,
            root: Some(input.root),
            direction: input.direction,
            edge_kind: input.edge_kind,
            depth: input.depth,
            limit_nodes: input.limit_nodes,
            limit_edges: input.limit_edges,
            base_url: input.base_url,
        })
        .await
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
        let base_url = input.base_url.unwrap_or_else(|| self.default_base_url.clone());

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
                    {"tool": "health", "input": {"base_url": base_url}},
                    {"tool": "list_workspaces", "input": {"base_url": base_url}},
                    {"tool": "list_tickets", "input": {"workspace": workspace, "state": "open", "limit": 50, "base_url": base_url}},
                    {"tool": "list_tickets", "input": {"workspace": workspace, "state": "in-progress", "limit": 50, "base_url": base_url}}
                ]
            }),
            WorkflowName::FetchTicketContext => serde_json::json!({
                "name": "fetch_ticket_context",
                "steps": [
                    {"tool": "get_ticket", "input": {"workspace": workspace, "id": id, "base_url": base_url}},
                    {"tool": "get_ticket_description", "input": {"workspace": workspace, "id": id, "base_url": base_url}},
                    {"tool": "list_edges", "input": {"workspace": workspace, "base_url": base_url}},
                    {"tool": "subgraph", "input": {"workspace": workspace, "root": id, "depth": 2, "base_url": base_url}}
                ]
            }),
            WorkflowName::InspectDependencies => serde_json::json!({
                "name": "inspect_dependencies",
                "steps": [
                    {"tool": "list_tickets", "input": {"workspace": workspace, "query": query, "limit": 20, "base_url": base_url}},
                    {"tool": "list_edges", "input": {"workspace": workspace, "kind": "depends_on", "base_url": base_url}},
                    {"tool": "subgraph", "input": {"workspace": workspace, "root": id, "direction": "both", "depth": 3, "base_url": base_url}}
                ]
            }),
        };

        let text = serde_json::to_string_pretty(&payload).map_err(|err| {
            McpError::internal_error(format!("serialization failed: {err}"), None)
        })?;

        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    #[tool(
        name = "help",
        description = "List ticket-mcp tools, endpoint mapping, and required parameters."
    )]
    async fn help(&self) -> Result<CallToolResult, McpError> {
        let mut ops = Map::new();
        ops.insert(
            "health".to_string(),
            serde_json::json!({
                "path": "/healthz",
                "required": [],
            }),
        );
        ops.insert(
            "list_workspaces".to_string(),
            serde_json::json!({
                "path": "/api/workspaces",
                "required": [],
            }),
        );
        ops.insert(
            "list_tickets".to_string(),
            serde_json::json!({
                "path": "/api/tickets",
                "required": ["workspace"],
                "optional": ["state", "query", "limit"],
            }),
        );
        ops.insert(
            "get_ticket".to_string(),
            serde_json::json!({
                "path": "/api/tickets/{id}",
                "required": ["workspace", "id"],
            }),
        );
        ops.insert(
            "get_ticket_description".to_string(),
            serde_json::json!({
                "path": "/api/tickets/{id}/description",
                "required": ["workspace", "id"],
            }),
        );
        ops.insert(
            "list_edges".to_string(),
            serde_json::json!({
                "path": "/api/edges",
                "required": ["workspace"],
                "optional": ["kind"],
            }),
        );
        ops.insert(
            "subgraph".to_string(),
            serde_json::json!({
                "path": "/api/graph/subgraph",
                "required": ["workspace", "root"],
                "optional": ["direction", "edge_kind", "depth", "limit_nodes", "limit_edges"],
            }),
        );

        let payload = serde_json::json!({
            "default_base_url": self.default_base_url,
            "tools": [
                "health",
                "list_workspaces",
                "list_tickets",
                "get_ticket",
                "get_ticket_description",
                "list_edges",
                "subgraph",
                "workflow",
                "request"
            ],
            "primary_pattern": "Use named tools first. Use request only for generic/fallback operation routing.",
            "operations": ops,
            "notes": [
                "All operations are HTTP GET wrappers around ticket-http endpoints.",
                "You can override the server per call with base_url.",
                "Non-2xx responses are returned as success=false with status and raw response body.",
            ],
        });

        let text = serde_json::to_string_pretty(&payload).map_err(|err| {
            McpError::internal_error(format!("serialization failed: {err}"), None)
        })?;
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }
}

#[tool_handler]
impl ServerHandler for TicketServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "ticket-mcp forwards MCP tool calls to ticket-http endpoints. Prefer named tools and call workflow/help for guided usage."
                    .to_string(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

pub async fn run_mcp_server(
    default_base_url: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server = TicketServer::new(default_base_url);

    tracing::info!("Starting ticket-mcp server on stdio");

    let service = server.serve(stdio()).await.inspect_err(|err| {
        eprintln!("Server error: {err:?}");
    })?;

    service.waiting().await?;
    Ok(())
}
