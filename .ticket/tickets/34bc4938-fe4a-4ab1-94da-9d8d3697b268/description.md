# [AOH][Design] Full System Architecture — ADRs and Design Document

## Status

`IN PROGRESS` — All 10 OQs answered. ADRs fully locked pending 4 research gates.

---

## Architecture Decision Records (ADRs)

### ADR-1: Sandbox Isolation Tier ← REVISED 2026-04-09
**Decision**: **Container-based Browser-as-a-Service (BaaS)** using Docker (primary) / Podman (Linux CI).  
**Rationale**: MicroVM approaches (cloud-hypervisor, Firecracker) lack `virtio-gpu` — browsers inside them are limited to software rendering (SwiftShader), unacceptable for GPU-accelerated WASM workloads. Containers with GPU passthrough (`--gpus all` / `--device /dev/dri`) solve both isolation and GPU requirements.  
**Technology stack**:
- Orchestration: `bollard` crate (async Docker API in Rust)
- Isolation: dedicated Docker network namespace per session
- Network policy: allow-list proxy (cargo.io, github.com, Copilot API endpoint)
- GPU: `--use-gl=angle` (Windows/WSL2), `--use-gl=egl` (Linux)
- Git worktree: bind-mounted from host into container
- Secret injection: per-session one-time token server (not env vars)
- Abstraction: `ContainerRuntime` trait → Docker impl; Podman compatible  
**Residual microVM use**: cloud-hypervisor available as Tier 3 option for non-browser agent tasks needing kernel-level isolation.  
**Research gate**: `49d6fe2e` Container BaaS research must validate GPU passthrough, cold-start, and bollard lifecycle.

### ADR-2: Messaging Service ← REVISED 2026-04-09
**Decision**: **Telegram** (primary MVP) + **Discord** + **Slack** via `Notifier` trait multi-adapter.  
**Rationale**: WhatsApp dropped — requires paid Meta Business account. Telegram has the best Rust crate (`teloxide`), zero cost, and instant setup. Discord and Slack follow as second and third adapters.  
**Adapter priority**: Telegram → Discord → Slack (implement in that order).  
**Routing**: `MultiNotifier` with configurable `RoutingPolicy` (Broadcast or PrimaryOnly).  
**Research gate**: `89701593` Messaging survey ticket confirms Notifier trait design and adapter scaffolds.

### ADR-3: Git Hosting
**Decision**: GitHub for remote; **local-first PR management**. Push to remote only on explicit user trigger or merge.  
**Design gate**: `d3f76335` documents TOML PR schema, worktree lifecycle, aoh-meta branch, remote push flow.

### ADR-4: Orchestrator Entrypoint
**Decision**: Rust daemon with ratatui TUI (terminal-first, v1). VS Code extension deferred to Phase 2.

### ADR-5: Agent API Provider
**Decision**: GitHub Copilot only for v1. Thin `CopilotClient` wrapping `reqwest`. No provider trait in v1.

### ADR-6: Cross-Agent Coordination Protocol
**Decision**: ticket-api (durable, auditable) + in-process tokio broadcast channels (real-time intra-process events). No external broker for 5-20 sessions.

### ADR-7: MCP Routing for Parallel Sessions
**Decision**: Per-session MCP server sockets. Each container spawns its own MCP server; orchestrator injects socket path into agent environment.

### ADR-8: Agent Identity Scheme
**Decision**: Reusable nature-vocabulary personas (Petal, Cedar, Fern…); LRU assignment; same persona on revival.  
**Design gate**: `d45826cd` Persona store design.

### ADR-9: Session Persistence and Revival
**Decision**: Summary-injected revival. Archived `session-archive.toml` (result, summary, modified files, test results, open questions) injected into kickoff prompt. Same worktree (bind-mount volume) reused where possible.

### ADR-10: Budget Controls ← FINALIZED 2026-04-09
**Decision**: Tiered escalation with **configurable token and time limits** (sane defaults, tuned later).  
**Default thresholds** (initial, to be calibrated):
- Soft token limit: 80,000 tokens → agent receives budget warning
- Self-assessment window: agent has 2,000 tokens to decide (continue / escalate)
- User notification: orchestrator pushes assessment to messenger + waits up to 5 minutes
- Hard token limit: 200,000 tokens → unconditional termination, partial results reported
- Time soft limit: 30 minutes → warning
- Time hard limit: 90 minutes → termination
- All thresholds configurable in `orchestrator.toml`

---

## System Boundary Diagram (Post-ADR)

```
┌─────────────────────────────────────────────────────────────────┐
│  User                                                           │
│    ↕ Telegram (primary) / Discord / Slack (via Notifier trait)  │
│    ↕ ratatui TUI (terminal-first; daemon client)                │
│    ↕ VS Code (Phase 2: extension + ticket-viewer panels)        │
└────────────────────────┬────────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────────┐
│  orchestrator-core daemon (tokio async, Unix socket API)        │
│    Researcher     ticket creation from user prompts             │
│    Session Scheduler  LRU persona + parallelism (5-20 sessions) │
│    Conflict Detector  file-scope overlap → pause + notify       │
│    PR Manager    local TOML (aoh-meta branch) + GitHub on push  │
│    Cost Watchdog  token/time counter per session; budget events │
│    MultiNotifier  Telegram → Discord → Slack dispatch           │
└──┬──────────────┬──────────────────────────────────────────────┘
   │              │
   ▼              ▼
[ticket-api]  [1..20 Agent Sessions]
(coordination)   Each session:
                  • Docker container (GPU passthrough, allow-list net)
                  • git worktree bind-mounted from host
                  • Persona identity: git config user.name/email
                  • MCP server on unix socket (ticket-mcp, context-mcp…)
                  • CopilotClient → api.githubcopilot.com
                  • Session archive on completion/revival
```

## Crate Map

New crates in `context-engine` workspace:

```
crates/
  orchestrator-core/      # scheduler, conflict detector, event bus, cost watchdog
  sandbox-manager/        # ContainerRuntime trait; Docker/Podman adapter; worktree lifecycle
  agent-session/          # session state machine, kickoff prompt, result ingestion, archive
  pr-manager/             # local PR TOML, aoh-meta branch, GitHub push integration
  notifier/               # Notifier trait + Telegram, Discord, Slack adapters; MultiNotifier
  agent-identity/         # PersonaStore, LRU assignment, git config application

tools/
  orchestrator-tui/       # ratatui TUI: sessions, PRs, budget monitor, local diff viewer
  # tools/ticket-vscode/  extended in Phase 2
```

## Session Lifecycle State Machine

```
[Ticket: ready]
       ↓ orchestrator detects
  Provisioning ── fail → ProvisionFailed (notify)
       ↓ container + worktree + persona + MCP + secrets
  KickingOff ── fail → StartFailed
       ↓ inject kickoff prompt (ticket, criteria, persona, archive if revival)
  Running ◀────────────────────────────────────────┐
       │                                            │ (revival)
       ├── budget.soft.token/time ─→ BudgetWarning  │
       │        ↓                                   │
       │   SelfAssessment (agent decides)           │
       │    ↓ continue          ↓ escalate          │
       │  Running           UserNotified            │
       │                    ↓ approve  ↓ deny       │
       │                  Running  HardTerminate     │
       │                                            │
       ├── budget.hard ──────→ HardTerminate         │
       ↓                                            │
  Reporting (agent submits result JSON)             │
       ↓                                            │
  PROpen (local TOML created on aoh-meta)          │
       ↓                                            │
       ├── review.approve ──→ Merging               │
       │        ↓ squash-merge to main              │
       │   Merged → Archiving → Archived            │
       │                                            │
       └── review.changes ──→ RevivalQueue ─────────┘
```

## Integration with Existing System

| Existing Component | Integration |
|---|---|
| `ticket-api` | `assigned_agent_id`, `session_id`, `evidence_refs` written per session |
| `context-mcp` | Provided as MCP tool to each agent container |
| `ticket-mcp` | Provided as MCP tool (ticket updates, state transitions) |
| `log-viewer` | Container logs streamed via bollard logs API |
| `ticket-viewer` | PR and session state visible (Phase 2: extended panel) |
| Bootstrap T1–T6 | Auth/lifecycle primitives extended by orchestrator-core |

## Implementation Phase Plan

**Phase A — Platform** (parallel tracks):
- A1: `sandbox-manager`: ContainerRuntime trait + Docker adapter; GPU passthrough; network isolation; secret server
- A2: `agent-identity`: PersonaStore; TOML config; LRU assignment; git config application

**Phase B — Orchestration**:
- B1: `agent-session`: session state machine; kickoff prompt; result ingestion; archive TOML
- B2: `orchestrator-core`: event bus; scheduler; conflict detector; cost watchdog

**Phase C — Integration**:
- C1: `pr-manager`: local TOML PR records; aoh-meta branch; GitHub push flow
- C2: `notifier`: Telegram adapter (MVP); Discord adapter; Slack adapter; MultiNotifier

**Phase D — UX**:
- D1: `orchestrator-tui`: session list; PR viewer; budget monitor; local diff pager

**Phase E — Quality**:
- E1: E2E integration test: 2 parallel sessions → PR → approve → merge
- E2: Security audit: credential redaction; allow-list enforcement; container escape test
- E3: Load test: 20 concurrent sessions; resource accounting

## Acceptance Criteria

- [x] All 10 ADRs finalized (all OQs answered 2026-04-09)
- [ ] System boundary diagram approved
- [ ] Crate map reviewed — no conflicts with existing crates
- [ ] Session lifecycle state machine peer-reviewed
- [ ] Integration touch-points verified (no breaking changes to existing crates)
- [ ] Phase A–E sub-tickets created and wired in dependency graph
- [ ] Architecture document saved as final version