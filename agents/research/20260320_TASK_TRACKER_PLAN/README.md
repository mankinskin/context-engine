# Task Tracker — All-Rust Filesystem Plan

## Decision Baseline (Locked)

- Distributed filesystem tickets: ticket folders can live anywhere under registered scan roots.
- Global index: canonical runtime index in `redb`, mapping UUID -> filesystem path + derived metadata.
- Identity: UUID v4 only.
- Required universal fields: `id`, `created_at`.
- Workflow model: configurable ticket schemas and configurable state machines per ticket type.
- Concurrency: per-ticket locks plus short-lived global index write lock.
- History: git-backed diff history via `git2`.
- Search: full-text + metadata in a unified query language from the initial implementation track.
- Reconciliation: watcher + full scan supports orphan integration and parse diagnostics.

## Stack Decision

**Filesystem + redb (index) + git2 (history) + Tantivy (search)** — 99% Rust, `libgit2` accepted for diff history

| Layer            | Crate / Mechanism                        | Role                                          |
|------------------|------------------------------------------|-----------------------------------------------|
| Artifact store   | OS filesystem (distributed ticket folders) | Source of truth for all content & assets     |
| Global index     | `redb`                                   | Ticket metadata, edges, state, asset registry  |
| History / diffs  | `git2` (libgit2 bindings)                | Line-level diffs, apply/revert, version store  |
| FS watching      | `notify` crate                           | Detect changes, orphan integration, error diag |
| Full-text search | `tantivy`                                | FTS + metadata filter unified query language   |
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
  00_phase_contracts/
    PLAN.md                     ← Phase 0: schema engine, folder layout, index/query contracts
    EXECUTION_CHECKLIST.md      ← Phase 0 implementation checklist, gates, and exit criteria
  01_phase_minimal_backend/
    PLAN.md                     ← Phase 1: CRUD, watcher/reconcile, locks, global index
  02_phase_history_rollback/
    PLAN.md                     ← Phase 2: git-backed diff history, revert/apply, branch policy
  03_phase_search/
    PLAN.md                     ← Phase 3: unified query language, FTS + metadata, highlighting
  04_phase_advanced_refs/
    PLAN.md                     ← Phase 4: dependency graph operations, merge/queue coordination
  05_use_cases/
    INDEX.md                    ← scenario map for concurrent agent workflows
    20260320_USE_CASE_*.md      ← concrete multi-agent and merge/dependency scenarios
```

## Prerequisites

Answer all questions in `INTERVIEW.md` before executing Phase 0.
Many architectural choices (ID format, state machine, lock granularity) cascade through
every phase — wrong defaults here are expensive to change later.

## Dependency Chain

```
INTERVIEW answers
      │
      ▼
Phase 0: Contracts (schema engine, folder layout, index model, query grammar)
      │
      ▼
Phase 1: Minimal backend (create/read/update/delete + dependency edges + atomic writes + watcher/reconcile)
      │
      ├──► Phase 2: History + rollback (can run after Phase 1 stabilises)
      │
  └──► Phase 3: Search (starts as soon as Phase 1 CRUD is stable)
                    │
                    ▼
             Phase 4: Advanced refs + graph viz (depends on 1 + 2 + 3)

Use case scenarios in `05_use_cases/` inform all phases and serve as acceptance narratives.
```

## Status

- [x] INTERVIEW answers complete
- [ ] Phase 0 executed
- [ ] Phase 1 executed
- [ ] Phase 2 executed
- [ ] Phase 3 executed
- [ ] Phase 4 executed
