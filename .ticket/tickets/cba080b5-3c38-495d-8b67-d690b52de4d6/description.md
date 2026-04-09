# [AOH][Research] VS Code + GitHub Copilot Agent API and MCP Integration

## Objective

Determine what programmatic control surface exists for creating and supervising GitHub Copilot agent sessions from a Rust orchestrator, and how MCP tools can be shared across multiple agent sessions running in parallel.

## Research Questions

1. Does the GitHub Copilot API expose a session creation and management API (not just completions)?
2. Can we programmatically start a Copilot "agent mode" session with a custom system prompt and tool configuration?
3. What is the Copilot Extensions API — can we build a custom extension that acts as the orchestrator control surface?
4. How does VS Code's language model API (`vscode.lm`) work for agent orchestration?
5. Can we stream and observe agent tool use in real time from outside VS Code?
6. How do multiple MCP server instances get routed to the correct agent sessions?
7. What are the rate limits and quotas for Copilot Pro+?

## GitHub Copilot API Surface

### Completions API (current public)
- `POST https://api.githubcopilot.com/chat/completions` — OpenAI-compatible
- Headers: `Authorization: Bearer {token}`, `Copilot-Integration-Id: <id>`
- Supports: `messages`, `tools` (function calling), `stream: true`
- Model: `gpt-4o`, `claude-3.5-sonnet`, others per plan
- **Research**: token scoping, per-user rate limits, max context window

### Copilot Extensions (GitHub Marketplace Extensions)
- https://docs.github.com/en/copilot/building-copilot-extensions
- Extension receives Copilot conversation events and can respond with tool calls
- Extension can be a Rust HTTP server with `/agent` endpoint
- **Research**: can the extension receive and route to external orchestrator?

### VS Code Language Model API
- `vscode.lm.selectChatModels()` — available model list
- `model.sendRequest(messages, options, token)` — send message with tools
- `LanguageModelToolResult` — typed tool responses
- `LanguageModelChatTool` — register custom tools
- **Research**: can a VS Code extension create multiple simultaneous agent sessions?

### VS Code Agent Mode (Chat Participants)
- `vscode.chat.createChatParticipant(id, handler)` — register agent
- Participant receives `@agent-name <message>` in Copilot chat
- Can invoke tools, stream responses, and reference editor context
- **Research**: multi-instance participants; can one participant spawn sub-sessions?

## MCP Integration

### Current MCP Architecture (context-engine)
- `context-mcp`, `doc-viewer`, `log-viewer`, `ticket-mcp` are running MCP servers
- Single VS Code session connects to these via MCP config
- **Gap**: how do multiple parallel agent sessions share or get isolated MCP tool access?

### Multi-Agent MCP Routing Options

**Option A: Shared MCP servers, session-scoped context injection**
- All agents connect to the same MCP servers
- Each agent's ticket/session ID is injected via system prompt
- MCP tools return data filtered by session context
- Risk: agents can accidentally read each other's state

**Option B: Per-session MCP server instances**
- Orchestrator spins up isolated MCP server processes per agent
- Each MCP server instance has access only to that agent's sandbox
- Ports: dynamic allocation or Unix sockets
- Risk: resource overhead per agent

**Option C: MCP proxy/router**
- Single MCP proxy that routes requests to the correct backend based on session ID header
- Agents declare their session ID; proxy enforces isolation
- Most scalable but adds proxy complexity

### MCP Protocol Capabilities
- `tools/list` — agent discovers available tools
- `tools/call` — agent invokes tool with args
- `resources/read` — agent reads resources (files, docs)
- Sampling: LLM-in-the-loop tool responses
- **Research**: MCP auth/identity model; can session ID be passed as metadata?

## VS Code Extension Architecture for Orchestrator

### Option A: Extension as Orchestrator UI
```
VS Code Extension
  ├── WebviewPanel (sessions dashboard, ticket viewer)
  ├── Chat Participant (user-facing @orchestrator commands)
  ├── Task Provider (session lifecycle tasks)
  └── StatusBar (active sessions count, cost meter)
  
Calls → Rust orchestrator daemon via stdio or HTTP
```

### Option B: Extension + External Daemon
```
Rust Orchestrator Daemon (TCP/stdio server)
  ├── Ticket scanner (ready-ticket poller)
  ├── Session scheduler
  ├── Branch/sandbox manager
  └── MCP proxy

VS Code Extension → thin UI client over IPC
```

## Rust Crates for Copilot/VS Code Integration

| Crate/Library | Purpose |
|---|---|
| `reqwest` + serde | Copilot completions API HTTP client |
| `tokio` | Async session supervision |
| Our existing MCP servers | Tool provision per agent |
| `tower-http` + axum | MCP proxy / orchestrator HTTP API |

## Key Unknowns (Needs External Research)

1. Is there a publicly documented Copilot session API beyond completions? (May require GitHub Developer Preview access)
2. Can we create Copilot agent sessions headlessly (without VS Code UI)?
3. What is the Copilot Pro+ rate limit per minute/hour for tool-calling sessions?
4. Does the Copilot Extensions API allow external webhook events (for session state push)?

## Acceptance Criteria

- [ ] Copilot completions API call flow documented with rate limits
- [ ] Copilot Extensions API capabilities mapped; extension skeleton evaluated
- [ ] VS Code Language Model API for multi-session documented
- [ ] MCP routing strategy (A/B/C) evaluated and recommended
- [ ] VS Code extension architecture option selected with rationale
- [ ] Key unknowns answered (or escalated with a workaround plan)