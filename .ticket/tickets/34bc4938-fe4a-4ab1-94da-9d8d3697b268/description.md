# [AOH][Design] Full System Architecture — ADRs and Design Document

## Status

`IN PROGRESS` — Core ADRs are locked from the interview, but final implementation readiness is still blocked on three follow-up design/refinement tickets:
- `02412b9a` — reconcile AOH architecture with existing Phase 2 execution tickets
- `db784443` — operator authorization, secret lifecycle, and trust boundaries
- `ffa5361a` — session archive, artifact retention, and revival schema

---

## Architecture Decision Records (ADRs)

### ADR-1: Sandbox Isolation Tier
**Decision**: Container-based Browser-as-a-Service (BaaS) using Docker (primary) / Podman (Linux CI).  
**Rationale**: cloud-hypervisor / Firecracker lack `virtio-gpu`; browser workloads require GPU-capable containers.  
**Stack**:
- `bollard` for orchestration
- dedicated per-session Docker network
- allow-list proxy for outbound network control
- GPU flags: `--use-gl=angle` (Windows/WSL2), `--use-gl=egl` (Linux)
- bind-mounted git worktree
- `ContainerRuntime` trait for Docker/Podman portability
**Residual microVM use**: future non-browser isolation only.  
**Research gate**: `49d6fe2e`

### ADR-2: Messaging Service
**Decision**: Telegram (primary MVP) + Discord + Slack behind a `Notifier` trait.  
**Rationale**: WhatsApp was dropped due to paid Meta Business requirements.  
**Routing**: `MultiNotifier` with configurable routing policy.  
**Research gate**: `89701593`

### ADR-3: Git Hosting
**Decision**: GitHub remote with local-first PR management. Push remote branches only on explicit user trigger or merge.  
**Design gate**: `d3f76335`

### ADR-4: Orchestrator Entrypoint
**Decision**: Rust daemon + ratatui TUI in v1. VS Code extension deferred to Phase 2.

### ADR-5: Agent API Provider
**Decision**: GitHub Copilot only in v1. Thin `CopilotClient` over `reqwest`. No provider abstraction in v1.

### ADR-6: Cross-Agent Coordination Protocol
**Decision**: `ticket-api` for durable coordination + in-process `tokio::broadcast` channels for real-time events. No external broker in v1.

### ADR-7: MCP Routing for Parallel Sessions
**Decision**: Per-session MCP server sockets. Each session gets isolated MCP tool access.

### ADR-8: Agent Identity Scheme
**Decision**: Reusable nature-vocabulary personas with LRU assignment and same-persona revival.  
**Design gate**: `d45826cd`

### ADR-9: Session Persistence and Revival
**Decision**: Summary-injected revival using `session-archive.toml` plus archived artifacts. Same worktree reused where possible.  
**Design gate**: `ffa5361a`

### ADR-10: Budget Controls
**Decision**: Tiered escalation with configurable token/time limits and sane defaults.  
**Defaults**:
- Soft token limit: 80,000
- Self-assessment window: 2,000 tokens
- User notify wait: 5 minutes
- Hard token limit: 200,000
- Time soft limit: 30 minutes
- Time hard limit: 90 minutes
- All thresholds configured in `orchestrator.toml`

---

## Remaining Design Blockers

### `02412b9a` — implementation decomposition reconciliation
The repository already has an execution-layer implementation subtree under `d5ced7e2`. AOH must **reuse and refine that subtree**, not create a duplicate Phase A–E implementation tree.

### `db784443` — trust boundaries and authorization
Messenger approvals, secret fetch, remote git actions, and operator controls need an explicit security contract before implementation starts.

### `ffa5361a` — archive and revival contract
ADR-9 needs a concrete schema for archives, artifacts, retention, and revival semantics.

---

## System Boundary Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│ User                                                            │
│   ↕ Telegram / Discord / Slack                                  │
│   ↕ ratatui TUI                                                 │
│   ↕ VS Code (Phase 2 only)                                      │
└────────────────────────┬────────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────────┐
│ orchestrator-core daemon                                        │
│   Researcher          ticket creation / interview flow          │
│   Session Scheduler   persona-aware parallel session planning   │
│   Conflict Detector   overlap detection + pause/notify          │
│   PR Manager          local PR records + optional remote push   │
│   Cost Watchdog       token/time budgets                        │
│   MultiNotifier       Telegram → Discord → Slack                │
└──┬──────────────┬──────────────────────────────────────────────┘
   │              │
   ▼              ▼
[ticket-api]   [1..20 Agent Sessions]
               • containerized execution
               • bind-mounted worktree
               • per-session MCP server/socket
               • persona-bound git identity
               • archived session artifacts
```

## Crate Map

Planned AOH crates:

```
crates/
  orchestrator-core/
  sandbox-manager/
  agent-session/
  pr-manager/
  notifier/
  agent-identity/

tools/
  orchestrator-tui/
```

## Session Lifecycle State Machine

```
[Ticket: ready]
       ↓
  Provisioning ── fail → ProvisionFailed
       ↓
  KickingOff ── fail → StartFailed
       ↓
  Running
       ├── soft budget → BudgetWarning → SelfAssessment → continue/escalate
       ├── hard budget → HardTerminate
       ↓
  Reporting
       ↓
  PROpen (local PR record)
       ├── approve → Merging → Merged → Archiving → Archived
       └── changes requested → RevivalQueue → Running
```

## Canonical Implementation Mapping

AOH implementation should refine the existing Phase 2 execution tickets instead of creating a second parallel tree:

- `8c185de3` — execution provider contracts + Copilot API auth client
- `51471c3e` — sandbox manager
- `a8632357` — assignment runner
- `d0cc3c8b` — review coordinator
- `8db8ef2f` — notifier adapters
- `5af54f6c` — terminal UI
- `0135d961` — E2E integration / fault injection

AOH planning tickets feed these implementation owners through research/design outputs.

## Integration with Existing System

| Existing Component | Integration |
|---|---|
| `ticket-api` | durable coordination, evidence refs, assigned agent IDs |
| `context-mcp` | per-session MCP tool |
| `ticket-mcp` | ticket state/update tool |
| `log-viewer` | streamed container/session logs |
| `ticket-viewer` | Phase 2 UI integration |
| bootstrap T1–T6 | execution lifecycle primitives that AOH builds on |

## Acceptance Criteria

- [x] All 10 interview-driven ADRs finalized
- [ ] System boundary diagram approved
- [ ] Crate map reviewed — no conflicts with existing crates
- [ ] Session lifecycle state machine peer-reviewed
- [ ] Integration touch-points verified
- [ ] Existing Phase 2 implementation tickets reconciled with AOH planning (`02412b9a`)
- [ ] Trust-boundary / operator-authorization design approved (`db784443`)
- [ ] Archive / revival schema approved (`ffa5361a`)
- [ ] Architecture document saved as final version