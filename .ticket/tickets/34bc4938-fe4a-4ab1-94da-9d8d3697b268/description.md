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
**Decision**: `ticket-api` (tickets + draftboard) as the sole coordination layer in v1. Agents read/write ticket fields and board entries; orchestrator polls state. No in-process event bus in v1. `tokio::broadcast` deferred to v2.

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

### ADR-11: Branch Naming Convention
**Decision**: `aoh/{agent-id}/{ticket-slug}`  
**Rationale**: The `aoh/` prefix identifies all AOH-managed branches; agent-id scopes to a session; ticket-slug gives human-readable context without embedding the full UUID in the branch name.  
**Applies to**: git worktrees, remote branch pushes, PR metadata, archive artifact references, and reconciliation queries.

### ADR-12: Operator Authorization and Messenger Control
**Decision**: Full messenger control with a flat operator allow-list (Option C feature set, Option B identity model for v1).  
**Scope**: All control actions (approve review, request changes, reject, retry, stop session, extend budget, terminate) are accessible from Telegram, Discord, and Slack.  
**Identity (v1)**: Operator messenger user IDs (e.g. Telegram user IDs, Discord user IDs) are listed in `orchestrator.toml`. Every inbound command is checked against this allow-list before dispatch. Designed for extensibility toward per-action grants.  
**Implementation**: Each messenger adapter implements both `Notifier` (outbound) and `CommandListener` (inbound). Inbound commands enter the orchestrator via the same operator-command channel as TUI commands and traverse identical review-coordinator transition guards.

### ADR-13: Secret Delivery
**Decision**: One-time HTTP or Unix-socket fetch from the orchestrator secret endpoint during container kickoff (Option A).  
**Mechanism**: Orchestrator generates a nonce-keyed URL for each session. The container fetches its secrets from that URL exactly once; the nonce is consumed on first read and subsequent requests return 404. Default TTL: 60 seconds.  
**Restriction**: Env vars are forbidden in CI/prod. A local-dev convenience flag in `orchestrator.toml` may allow env vars for development only.

### ADR-14: Session Archive Layout and Retention
**Decision**: `.aoh/archive/{ticket-id}/{session-id}/` inside the repository working tree, excluded from git via `.aoh/` in `.gitignore`.  
**Layout**: `.aoh/archive/{ticket-id}/{session-id}/`, containing:
- `session-archive.toml` — canonical manifest
- `stdout.log`, `stderr.log`
- `cargo-check.txt`, `test-results.json`, `diff.patch`

**Retention**: Indefinite — archives are kept until the operator explicitly prunes them via `aoh archive prune`.  
**Revival chaining**: Each `session-archive.toml` includes an optional `revival_of` field referencing the prior session ID for the same ticket, forming a linked revival chain.

### ADR-15: Revival Strategy
**Decision**: On revival, the orchestrator rebases the agent branch onto the latest `main`. The agent (running in-container) resolves any rebase conflicts. After the rebase, the agent branch is the authoritative source of truth. The archive TOML supplies context but does not override branch content.  
**Sequence**:
1. Orchestrator fetches latest `main` (or configured base branch).
2. Orchestrator applies `git rebase origin/main` on the agent branch.
3. If conflicts exist, the agent receives them and resolves them within its session.
4. Resolved, rebased branch is the state for the next review cycle.

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