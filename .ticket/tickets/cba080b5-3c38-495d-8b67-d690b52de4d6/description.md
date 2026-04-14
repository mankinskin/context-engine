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

---

## Resolved Decisions (locked 2026-07-11)

**Status: COMPLETE** — All research questions answered, key unknowns resolved or escalated with workarounds.

### RQ-1: Session management API
**Answer: No.** The Copilot API (`POST /chat/completions`) is OpenAI-compatible completions only. There is no public session creation, lifecycle management, or agent orchestration API. Copilot agent mode is an internal VS Code feature with no external control surface.

### RQ-2: Programmatic agent mode sessions
**Answer: Partially.** The VS Code Language Model API (`vscode.lm.selectChatModels()` + `model.sendRequest()`) allows extensions to send prompts with custom tool configurations. However, this runs within VS Code — it cannot be invoked headlessly from a Rust daemon. A VS Code extension can create multiple `sendRequest` calls in parallel, effectively running multi-session by maintaining separate message arrays per session.

**Workaround for v1**: The Rust orchestrator spawns VS Code instances (or uses the Copilot completions API directly via `reqwest`). Each agent session maintains its own conversation history and tool configuration. No VS Code dependency for the core loop — use the REST API directly.

### RQ-3: Copilot Extensions API
**Answer: Exists but not suitable for orchestration.** Copilot Extensions are GitHub Marketplace integrations that receive conversation events via webhook and respond. They cannot initiate sessions or push state. The extension receives `@extension-name <message>` invocations and returns tool call results. This is useful for exposing the orchestrator as a Copilot Extension (e.g., `@aoh status`) but not for programmatic agent control.

### RQ-4: VS Code Language Model API
**Key facts verified (2026-07-11):**
- `vscode.lm.selectChatModels({ vendor: 'copilot', family: 'gpt-4o' })` — selects models
- `model.sendRequest(messages, options, token)` — streaming response
- Supported families: `gpt-4o`, `gpt-4o-mini`, `o1`, `o1-mini`, `claude-3.5-sonnet`
- Max input tokens: 64K (GPT-4o)
- **Does NOT support system messages** — use User messages for system context
- Requires user consent dialog (authentication) before first use
- Chat participants: `vscode.chat.createChatParticipant()` for `@agent` commands

### RQ-5: Streaming/observing tool use externally
**Answer: Not directly.** Tool use within VS Code agent mode is internal to the VS Code process. There is no external observation API. **Workaround**: Use the completions API directly from Rust, where all tool calls are explicit in the response stream. The orchestrator controls the full agentic loop (send prompt → receive tool call → execute → send result → continue).

### RQ-6: MCP routing
**Already resolved by ADR-7**: Per-session MCP server instances (Option B). Each agent session gets its own MCP server processes with access scoped to that session's sandbox. Communication via per-session Unix sockets or dynamic ports.

### RQ-7: Rate limits
**Not publicly documented with specific numbers.** The VS Code Language Model API docs confirm rate limiting exists and that `LanguageModelError` is thrown on quota exhaustion. Copilot Pro+ limits are not published. **Mitigation**: Implement exponential backoff with jitter. Budget system (ADR-10, tiered budget per session) provides an independent cost ceiling regardless of upstream limits.

### Key Unknowns — Resolved

| Unknown | Resolution |
|---|---|
| 1. Public session API beyond completions? | **No.** Use completions API directly with tool-calling for the agentic loop. |
| 2. Headless Copilot agent sessions? | **No.** Build our own agentic loop in Rust via completions API. VS Code extension is a Phase 2 UI layer only (ADR-4). |
| 3. Copilot Pro+ rate limits? | **Not publicly documented.** Implement backoff + budget ceiling (ADR-10). |
| 4. Copilot Extensions webhook push? | **No push capability.** Extensions are request-response only. Useful as a user-facing surface (`@aoh`) in Phase 2. |

### Architecture Decision

**v1 (ratatui TUI, ADR-4):** Rust orchestrator uses Copilot completions API directly via `reqwest`. The orchestrator owns the full agentic loop: prompt → tool call response → execute tool → inject result → continue. No VS Code dependency for the core agent runtime.

**v2 (VS Code extension, deferred):** Extension Option B (thin UI client over IPC to Rust daemon). Extension provides: WebviewPanel dashboard, Chat Participant (`@aoh`), StatusBar cost meter. Extension does NOT own the agentic loop — it delegates to the Rust daemon.

### MCP Routing Strategy
**Option B selected (ADR-7):** Per-session MCP server instances. Justification: strongest isolation guarantee, no cross-session data leakage risk, resource overhead acceptable for <10 concurrent agents.

---

## Acceptance Criteria

- [ ] Copilot completions API call flow documented with rate limits
- [ ] Copilot Extensions API capabilities mapped; extension skeleton evaluated
- [ ] VS Code Language Model API for multi-session documented
- [ ] MCP routing strategy (A/B/C) evaluated and recommended
- [ ] VS Code extension architecture option selected with rationale
- [ ] Key unknowns answered (or escalated with a workaround plan)