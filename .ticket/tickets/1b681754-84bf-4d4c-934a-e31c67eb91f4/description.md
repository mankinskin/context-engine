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

- [ ] All Tier 1 and Tier 2 candidates evaluated with filled matrix rows
- [ ] At least one Tier 3 sandbox candidate benchmarked for cold-start and API surface
- [ ] MCP Rust implementation candidates identified and assessed
- [ ] Adoption recommendations recorded with rationale
- [ ] Research summary saved as ticket description update