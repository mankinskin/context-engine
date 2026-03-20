# Task Tracker — All-Rust Filesystem Plan

## Decision Baseline (Locked)

- Distributed filesystem tickets: ticket folders can live anywhere under registered scan roots.
- Global index: canonical runtime index in `redb`, mapping UUID -> filesystem path + derived metadata.
- Identity: UUID v4 only.
- Required universal fields: `id`, `created_at`.
- Workflow model: hardcoded default schema (`tracker-improvement`) first; trait boundaries for future extensibility. Full runtime schema engine deferred to post-dogfooding.
- Concurrency: per-ticket locks plus short-lived global index write lock.
- History: git-backed diff history via `git2` (embedded bare repo by default).
- Search: full-text + metadata in a unified query language, integrated into the core backend phase.
- Protocol split: `TaskCommand` JSON is the canonical agent protocol; human CLI subcommands are an adapter on top.
- Agent transport rollout: Phase 1 `ticket exec`, Phase 1.5 `ticket serve --stdio`, Phase 5 HTTP + MCP adapters.
- Reconciliation: watcher + full scan supports orphan integration and parse diagnostics.
- Schema compatibility: version-pinned at creation, additive-only in-place, breaking changes require explicit migration.

## Stack Decision

**Filesystem + redb (index) + git2 (history) + Tantivy (search)** — 99% Rust, `libgit2` accepted for diff history

| Layer            | Crate / Mechanism                        | Role                                          |
|------------------|------------------------------------------|-----------------------------------------------|
| Artifact store   | OS filesystem (distributed ticket folders) | Source of truth for all content & assets     |
| Global index     | `redb`                                   | Ticket metadata, edges, state, asset registry  |
| History / diffs  | `git2` (libgit2 bindings)                | Line-level diffs, apply/revert, version store  |
| FS watching      | `notify` crate                           | Detect changes, orphan integration, error diag |
| Full-text search | `tantivy`                                | FTS + metadata filter unified query language   |
| Lease protocol   | redb `LEASES` table + heartbeat           | Agent coordination, claim/unclaim, conflict     |
| Serialization    | `serde` + TOML/JSON                      | Human-readable manifests; configurable schemas |
| Compression      | `zstd`                                   | Optional snapshot / export compaction          |

## External Project Pattern Adoption

### Problem/Solution/Reference

1. Problem: deterministic multi-agent convergence is hard under concurrent writes.
Solution: adopt deterministic conflict-resolution concepts (write ordering, explicit delete semantics, validation invariants) in our own model.
Reference: `delightful-ai/beads-rs`.

2. Problem: agent/operator UX needs practical command ergonomics and robust machine-readable output.
Solution: adopt CLI ergonomics and JSON-first command surface patterns.
Reference: `Dicklesworthstone/beads_rust`.

3. Problem: direct adoption of either upstream architecture would constrain our distributed-folder + workflow-configurable schema design.
Solution: borrow patterns, do not adopt either codebase wholesale as backend.
Reference: both projects.

## Plan Structure

```
20260320_TASK_TRACKER_PLAN/
  INTERVIEW.md                  ← design questions + your answers (start here)
  README.md                     ← this file
  EXECUTION_CHECKLIST.md        ← global WIP-limited execution board (active topics)
  PROTOCOL_LAYER.md             ← canonical human-vs-agent protocol split
  VALIDATION_RELEASE_GOVERNANCE.md  ← coordinator, validator agents, bug + release gates
  DEFERRED_EXECUTOR.md          ← parked: executor abstraction + Zeroboot (post-dogfooding)
  00_phase_contracts/
    PLAN.md                     ← Phase 0: contracts (DONE)
    EXECUTION_CHECKLIST.md      ← Phase 0 checklist and formal closure
  01_phase_minimal_backend/
    PLAN.md                     ← Phase 1: Core Backend + Search (merged)
  015_phase_lease_protocol/
    PLAN.md                     ← Phase 1.5: Lease Protocol
  02_phase_history_rollback/
    PLAN.md                     ← Phase 2: History + Rollback (git strategy locked)
  04_phase_advanced_refs/
    PLAN.md                     ← Phase 3: Advanced Refs + Graph
  05_use_cases/
    INDEX.md                    ← scenario map for concurrent agent workflows
    20260320_USE_CASE_*.md      ← concrete multi-agent and merge/dependency scenarios
  06_transition_dogfooding/
    PLAN.md                     ← Phase 4: Dogfooding Transition
  07_phase_integrations/
    PLAN.md                     ← Phase 5: Integrations (viz endpoints + messenger)
```

Executor abstraction and Zeroboot integration are deferred to post-dogfooding. See `DEFERRED_EXECUTOR.md`.

## Prerequisites

Answer all questions in `INTERVIEW.md` before executing Phase 0.
Many architectural choices (ID format, state machine, lock granularity) cascade through
every phase — wrong defaults here are expensive to change later.

## Dependency Chain

```
Phase 0 (DONE)
    │
    ▼
Phase 1: Core Backend + Search
    │
    ├──► Phase 1.5: Lease Protocol (starts when Phase 1 CRUD stabilizes)
    │
    ├──► Phase 2: History + Rollback (starts when Phase 1 CRUD stabilizes)
    │
    └──► Phase 3: Advanced Refs + Graph (depends on 1 + 1.5 + 2)
              │
              ▼
         Phase 4: Dogfooding Transition
              │
              ▼
         Phase 5: Integrations (viz endpoints + messenger delivery)

Use case scenarios in `05_use_cases/` inform all phases and serve as acceptance narratives.
```

Protocol details for all phases are centralized in `PROTOCOL_LAYER.md`.
Validation and release policy is centralized in `VALIDATION_RELEASE_GOVERNANCE.md`.
