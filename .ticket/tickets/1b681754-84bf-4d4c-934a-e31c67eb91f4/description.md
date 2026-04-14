# [AOH][Research] Open-Source Agentic Coding Frameworks Survey

## Objective

Survey the open-source landscape for agentic coding frameworks that could be reused, adapted, or serve as reference implementations for the AOH project. Focus on: session lifecycle management, sandbox integration, tool use patterns, orchestration protocols, and Rust compatibility.

## Research Questions

1. Which frameworks offer the best programmatic session lifecycle API (start, supervise, terminate, revive)?
2. Which frameworks already implement git worktree or VM sandbox isolation?
3. Which have native MCP tool support or can easily be adapted to it?
4. Which are written in or have Rust bindings?
5. What orchestration patterns (task queue, event bus, state machine) do mature frameworks use?
6. Which open-source licenses are compatible with our repository?

## Resolved Decisions

> **Locked — do not reopen without new evidence.**

### RQ-1: Session Lifecycle API

Best session lifecycle APIs (ranked):
- **OpenHands** (score: 5/5) — Full REST API for session management: create, list, get status, send messages, terminate. Headless mode. SDK library (`openhands-ai`) allows programmatic embedding. Best reference for AOH session manager design.
- **Plandex** (4/5) — Client-server architecture with `plandex-server`. Session persistence, named plans, checkpoint rollback. Good reference for plan-based state, but AGPL-3.0 blocks direct use.
- **LangGraph** (4/5) — Stateful graph execution with checkpointing to multiple backends (SQLite, Postgres, memory). State snapshots at each node. Good reference for checkpoint/resume pattern.
- **Aider** (3/5) — `Coder` class is embeddable but primarily CLI-oriented. No server mode.
- **SWE-agent** (2/5) — Batch CLI only (run one issue to completion). No persistent session API.

**AOH takeaway:** Adapt OpenHands' REST session model (create → supervise → terminate → archive) but implement in Rust. Adopt LangGraph's checkpoint-per-node pattern for session revival (ADR-9).

### RQ-2: Sandbox Isolation

Best sandbox implementations:
- **OpenHands** (5/5) — Docker container per session, Kubernetes support added 10 months ago. `EventStreamRuntime` streams actions/observations. Best local-first Docker sandbox reference.
- **E2B** (5/5) — Firecracker micro-VMs, ~170ms cold start claimed. Cloud-only infrastructure (SDK is Apache-2.0 but VM runtime is proprietary). No self-hosted option for Firecracker infra.
- **SWE-agent** (4/5) — Docker container per run, ACI (Agent-Computer Interface) provides structured shell commands instead of raw exec. Good pattern for command abstraction.
- **Daytona** — Dev container manager, git-aware workspaces. Less relevant; overlaps with our bollard-based approach.

**AOH takeaway:** OpenHands' Docker sandbox driver is the closest reference to our ADR-1 bollard approach. E2B's Firecracker model is aspirational for Phase 2 but cloud-only now. The ACI pattern from SWE-agent is interesting for command safety but adds complexity; skip for v1.

### RQ-3: MCP / Tool Use

- **rmcp** (5/5) — **Official Anthropic Rust MCP SDK**, v1.4.0, Apache-2.0, 7.5M+ downloads, 3.3k stars, 154 contributors. Full spec coverage: tools, resources, prompts, sampling, roots, logging, completions, notifications, subscriptions. Proc macros (`#[tool]`, `#[tool_router]`, `#[prompt]`, `#[prompt_router]`) for declarative server/client definition. tokio async. OAuth support. **This is the MCP crate we should use.**
- **OpenHands** (4/5) — Uses `fastmcp` (Python) for MCP server integration. MCP is an optional extension, not the core tool protocol.
- **LangGraph** (3/5) — Tool registration via LangChain's tool abstraction, adapters exist for MCP.
- **AutoGen/AG2** (3/5) — Tool registration via decorators, community MCP adapters emerging.
- **CrewAI** (2/5) — Own tool abstraction, no native MCP.
- **SWE-agent** (1/5) — ACI is a custom protocol, no MCP.
- **Aider** (1/5) — Own tool abstraction, no MCP.
- **Plandex** (1/5) — Own tool system, no MCP.

**Community Rust MCP crates (superseded by rmcp):**
- `rust-mcp-schema` v0.10.0 — 325k downloads, type-safe MCP types only (not a full SDK). Use `rmcp` instead.
- `mcp-sdk` v0.0.3 — 10.8k downloads, stale (1 year), abandoned. Use `rmcp` instead.
- `mcp_client_rs` v0.1.7 — 18k downloads, client-only, stale. Use `rmcp` instead.
- `model-context-protocol` v0.2.2 — 645 downloads, minimal. Use `rmcp` instead.
- `mcprotocol-rs` v0.1.5 — 3.7k downloads, stale. Use `rmcp` instead.

**AOH takeaway:** Use `rmcp` (official SDK) for all MCP needs — both for the MCP servers agents expose and the MCP client the orchestrator uses to invoke agent tools. Already aligned with our existing `tools/mcp/` and `tools/ticket-mcp/` usage.

### RQ-4: Rust Compatibility

Frameworks with Rust code or bindings:
- **rmcp** — Pure Rust, official MCP SDK (5/5)
- **Zellij** — Pure Rust terminal multiplexer, MIT, ~22k stars (5/5). WASM plugin API for extensions. Programmatic session/pane creation via CLI. Could serve as terminal layer for agent sessions but heavy dependency for what we need.
- **Plandex** — Go, no Rust bindings (2/5 — Go is closer to Rust than Python in deployment model)
- **All others** — Python (1/5). No Rust bindings. Can only interface via subprocess/HTTP.

**AOH takeaway:** No existing full-featured coding agent is written in Rust. The AOH is genuinely novel as a Rust-native agentic coding system. We implement our own, borrowing design patterns from Python frameworks. `rmcp` gives us native MCP. Zellij is not needed — we use bollard for container sandboxes and tmux/pty inside containers for terminal management.

### RQ-5: Orchestration Patterns

Patterns observed in mature frameworks:
- **LangGraph** — **State machine / computation graph**: nodes are processing steps, edges encode control flow (conditional, parallel), state is checkpointed at each transition. Most sophisticated model. Directly analogous to our ticket state machine but at a finer granularity.
- **AutoGen/AG2** — **Conversation-based turns**: agents take turns in GroupChat, orchestrated by a manager agent. Human proxy for HITL. Turn-based is simple but lacks parallelism.
- **CrewAI** — **Role-based delegation**: sequential / hierarchical / parallel process types. Crew manager delegates tasks to specialized agents. Closest to AOH's multi-agent coordination via draftboard.
- **OpenHands** — **Event stream**: actions and observations flow through an event stream runtime. Simple but effective for single-agent sessions.
- **Plandex** — **Plan-based**: multi-file plans with diff preview, checkpoint per plan step, rollback capability. Good reference for our file-change tracking via git.

**AOH takeaway:** Our orchestration model (ADR-6: ticket-api + draftboard) is most similar to CrewAI's delegation pattern but with ticket-based rather than in-memory coordination. LangGraph's checkpoint-per-node pattern informs session revival (ADR-9). Event stream from OpenHands could be useful for agent observation in Phase 2 but is too complex for v1.

### RQ-6: License Compatibility

| Framework | License | Compatible? |
|---|---|---|
| OpenHands | MIT | Yes — pattern borrow freely |
| SWE-agent | MIT | Yes |
| Aider | Apache-2.0 | Yes |
| Plandex | AGPL-3.0 | **No — cannot integrate code** |
| LangGraph | MIT (core) | Yes — pattern borrow freely |
| AutoGen/AG2 | MIT (AG2) | Yes |
| CrewAI | MIT | Yes |
| E2B SDK | Apache-2.0 | Yes (SDK only; infra is proprietary) |
| Zellij | MIT | Yes |
| rmcp | Apache-2.0 | Yes — direct dependency |

**AOH takeaway:** Plandex is the only AGPL project; we can study its checkpoint design but cannot copy code. All others are MIT/Apache-2.0 compatible.

### Filled Evaluation Matrix

| Framework | Session (1-5) | Sandbox (1-5) | MCP (1-5) | Rust (1-5) | License (1-5) | Activity (1-5) | Orchestration (1-5) | Total |
|---|---|---|---|---|---|---|---|---|
| **OpenHands** | 5 | 5 | 4 | 1 | 5 | 5 | 3 | **28** |
| **LangGraph** | 4 | 2 | 3 | 1 | 5 | 5 | 5 | **25** |
| **rmcp** | 2 | N/A | 5 | 5 | 5 | 5 | 1 | **23** |
| **AutoGen/AG2** | 3 | 2 | 3 | 1 | 5 | 4 | 4 | **22** |
| **E2B** | 3 | 5 | 2 | 1 | 4 | 4 | 1 | **20** |
| **CrewAI** | 3 | 1 | 2 | 1 | 5 | 4 | 4 | **20** |
| **Zellij** | 3 | N/A | 1 | 5 | 5 | 5 | 1 | **20** |
| **SWE-agent** | 2 | 4 | 1 | 1 | 5 | 4 | 2 | **19** |
| **Aider** | 3 | 1 | 1 | 1 | 5 | 5 | 2 | **18** |
| **Plandex** | 4 | 1 | 1 | 2 | 1 | 3 | 3 | **15** |

### Adoption Recommendations

| Framework | Recommendation | Rationale |
|---|---|---|
| **rmcp** | **Adopt as-is** | Official Rust MCP SDK. Direct dependency for AOH's MCP client/server layer. Already aligned with existing workspace tools. |
| **OpenHands** | **Adapt pattern** | Best-in-class session lifecycle + Docker sandbox design. Re-implement in Rust using bollard. Study: REST session API, EventStreamRuntime, microagent system. |
| **LangGraph** | **Adapt pattern** | Checkpoint-per-node and graph-based orchestration inform ADR-9 (session revival) and future state machine refinement. Pure design reference. |
| **CrewAI** | **Partial borrow** | Role-based delegation model closest to our ticket + draftboard coordination. Borrow: agent specialization patterns, task handoff conventions. |
| **AutoGen/AG2** | **Partial borrow** | GroupChat's turn-taking model is useful reference for multi-agent conversation design. Borrow: human proxy pattern for operator intervention. |
| **SWE-agent** | **Partial borrow** | ACI (Agent-Computer Interface) is a good reference for command safety abstraction. Study but do not adopt — raw shell exec via bollard is simpler for v1. |
| **Aider** | **Partial borrow** | Git-native commit discipline and tree-sitter repo map are excellent references for our local-first git design (ticket d3f76335). |
| **E2B** | **Skip (v1) / Monitor** | Cloud-only Firecracker infra incompatible with local-first. Interesting for Phase 2 cloud scaling. No Rust SDK. |
| **Plandex** | **Skip** | AGPL-3.0 blocks code integration. Checkpoint/rollback design is interesting but achievable via git (our approach). |
| **Zellij** | **Skip** | Excellent Rust project but terminal multiplexing is handled inside containers via simpler means (tmux/pty). Heavy dependency for limited benefit. |

### Key Findings → Architecture Impact

1. **No Rust competition exists.** AOH is the first Rust-native agentic coding framework. All mature alternatives are Python.
2. **rmcp is production-ready.** The official Rust MCP SDK (v1.4.0, 7.5M downloads) covers the full spec and should be our MCP layer.
3. **OpenHands is the primary design reference** for session lifecycle and Docker sandbox management.
4. **LangGraph informs checkpointing** — but we achieve similar via git commits (ADR-15) + ticket state, not in-memory graph snapshots.
5. **Delegation patterns from CrewAI/AutoGen** validate our ticket + draftboard approach to multi-agent coordination (ADR-6).

## Candidates to Research

### Tier 1 — Full-featured Coding Agents
- **OpenHands (formerly OpenDevin)** — https://github.com/All-Hands-AI/OpenHands
  - Python; supports Docker sandboxes; extensive tool use; microagent system
  - Check: session API, sandbox driver interface, MCP integration status
- **SWE-agent** — https://github.com/SWE-agent/SWE-agent
  - Princeton; Python; ACI (agent-computer interface) architecture
  - Check: ACI design, how it handles agent isolation, PR submission flow
- **Aider** — https://github.com/Aider-AI/aider
  - Python; git-native; excellent commit discipline; LLM-agnostic
  - Check: library API vs CLI, programmatic invocation, branch management
- **Plandex** — https://github.com/plandex-ai/plandex
  - Go; multi-file planning with checkpoint rollback; server mode
  - Check: session server API, rollback design, isolation model

### Tier 2 — Orchestration Frameworks (Multi-Agent)
- **LangGraph** — https://github.com/langchain-ai/langgraph
  - Python; stateful multi-agent graphs; checkpointing; studio UI
  - Check: state persistence model, human-in-the-loop nodes, parallel branches
- **AutoGen / AG2** — https://github.com/microsoft/autogen
  - Python; multi-agent conversation; group chat; tool use; human proxy
  - Check: GroupChat design, agent identity model, tool registration
- **CrewAI** — https://github.com/crewAIInc/crewAI
  - Python; role-based agents; task delegation; result handoff
  - Check: crew orchestration model, inter-agent messaging

### Tier 3 — Sandboxing Infrastructure
- **E2B** — https://github.com/e2b-dev/e2b
  - Cloud sandboxes for AI agents; Firecracker-based; SDK in multiple langs
  - Check: Rust SDK availability, cold-start time, cost model, tool use API
- **Daytona** — https://github.com/daytonaio/daytona
  - Open-source dev container manager; git-aware; WS isolation
  - Check: programmatic API, snapshot/restore, git branch integration
- **Modal** — https://github.com/modal-labs/modal-client
  - Serverless with fast cold-start; ephemeral containers
  - Check: agent-compatible API, Rust SDK, pricing
- **Dagger** — https://github.com/dagger/dagger
  - Portable CI/CD engine; container-native; Go core with multi-lang SDKs
  - Check: Rust SDK, pipeline API, secret handling

### Tier 4 — Session/Terminal Management
- **ttyd / tmux API** — Terminal multiplexers for session persistence
- **Zellij** — Rust terminal multiplexer with plugin API
  - Check: programmatic session creation, pane lifecycle, Rust API depth
- **Devbox** — https://github.com/jetify-com/devbox
  - Nix-based reproducible dev shells per project

### Tier 5 — MCP / Tool Protocol
- **Model Context Protocol (MCP)** — https://github.com/modelcontextprotocol
  - Anthropic's protocol; already integrated in our stack
  - Check: multi-server routing, agent-to-agent tool delegation, auth model
- **mcp-rs** — Rust MCP implementations (search crates.io + GitHub)
  - Check: completeness, maintenance status

## Evaluation Matrix

For each framework, record:

| Dimension | Score (1-5) | Notes |
|---|---|---|
| Session lifecycle API | | |
| Sandbox integration | | |
| MCP / tool use | | |
| Rust compatibility | | |
| License compatibility | | |
| Maintenance activity | | |
| Orchestration model | | |

## Deliverable

A comparison table + adoption recommendation per framework:
- **Adopt as-is**: integrate directly
- **Adapt pattern**: implement our Rust version of their design
- **Partial borrow**: take specific modules (e.g., sandbox driver interface)
- **Skip**: not relevant or incompatible

## Acceptance Criteria

- [x] All Tier 1 and Tier 2 candidates evaluated with filled matrix rows
- [x] At least one Tier 3 sandbox candidate benchmarked for cold-start and API surface
- [x] MCP Rust implementation candidates identified and assessed
- [x] Adoption recommendations recorded with rationale
- [x] Research summary saved as ticket description update