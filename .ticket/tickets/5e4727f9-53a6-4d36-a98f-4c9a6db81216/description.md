# Phase 0 — Design Contracts

**Status:** DONE (formally closed — see EXECUTION_CHECKLIST.md for handoff)

Global progress tracking: `../EXECUTION_CHECKLIST.md`.
Checkboxes in this file are phase-scope contract gates.

## Objective

Produce the canonical schemas and contracts that every later phase compiles against.
Nothing in Phase 1–4 should hard-code values that belong here.

Execution checklist: `EXECUTION_CHECKLIST.md`

## Problem/Solution/Reference Baseline

1. Problem: state divergence under concurrent agent updates.
Solution: define deterministic conflict semantics and validation invariants in our contracts.
Reference: `delightful-ai/beads-rs` specification patterns.

2. Problem: brittle agent integrations from unstable command/output shape.
Solution: enforce explicit, machine-readable command contracts where `TaskCommand` JSON is canonical and CLI is only a human adapter.
Reference: `Dicklesworthstone/beads_rust` CLI ergonomics.

3. Problem: upstream architectures do not match distributed-folder + configurable-schema requirements.
Solution: borrow patterns only; keep our own storage/domain model.
Reference: both projects.

## Deliverables

- [ ] Universal ticket manifest — only `id` (UUID v4) + `created_at` (ISO 8601) required;
      all other fields declared in type schema
- [ ] Ticket type definition format: configurable field schema + state machine definition
      (states + allowed transitions as TOML/JSON config, loaded at runtime)
- [ ] `EdgeKind` as open string discriminant (not a Rust enum) — type definitions declare
      valid edge kinds and their constraint rules
- [ ] Ticket folder layout spec: `ticket.toml` (manifest), arbitrary content files,
      `assets/` for attachments, `.ticket-lock` for per-ticket locks
- [ ] Global index location strategy: repo-root (`.context-engine/ticket-index/`) with
      fallback to user-level (`~/.context-engine/ticket-index/`); documented scan roots
- [ ] `redb` table map (table names, key types, value types)
- [ ] Event/history design: git-backed diff history via `git2`; decide embedded bare repo
      vs. leveraging existing workspace git repo
- [ ] Query language grammar spec: unified FTS + metadata predicate syntax
      (e.g. `status:open assigned:alice "login page"`)
- [ ] FS watcher event taxonomy: CREATED, MODIFIED, MOVED, DELETED, PARSE_ERROR
- [ ] Command contract schema for human CLI adapter + agent protocol surfaces (`ticket exec`, `ticket serve --stdio`, HTTP, MCP)
- [ ] Self-containment rule for machine protocol commands: explicit `index_root`, full UUIDs, structured patch payloads
- [ ] Response projection contract: optional `fields` selector for agent responses
- [ ] Request envelope contract for persistent stdio transport: request ID, command payload, structured result/error envelope

## Key Interview Answers Consumed Here

| Interview Q | Contract output |
|-------------|----------------|
| Q1 — Distributed FS | Index root path strategy + scan root registry |
| Q2 — UUID | `TicketId = Uuid` type, folder = UUID string |
| Q3 — Configurable SM | `TicketTypeSchema` struct with `states: Vec<State>` + `transitions` |
| Q4 — Open edge kinds | `EdgeRecord { from, to, kind: String, ... }` |
| Q5 — Minimal required fields | `TicketManifest { id: Uuid, created_at: DateTime, extra: Map<String, Value> }` |
| Q6 — Per-ticket lock | `.ticket-lock` file spec + lock acquisition protocol |
| Q7 — Git diff history | git2 integration design: embedded bare repo or workspace repo |
| Q9 — Unified query | Query AST design: `Expr::Fts(str)` \| `Expr::Field(key, op, val)` \| `Expr::And(...)` |
| Q10 — FS tracking | Orphan integration protocol + parse error reporting contract |

## Command Surface Contract

The command model is layered:

- `TaskCommand` is the canonical machine contract.
- Human CLI parses flags and positional args into `TaskCommand`.
- `ticket exec` deserializes stdin JSON directly into `TaskCommand`.
- `ticket serve --stdio` deserializes JSONL requests directly into `TaskCommand`.
- HTTP and MCP adapters forward the same contract.

Machine protocol modes must not depend on cwd inference, shell quoting, or short UUID matching.

## Expected crate layout

```
crates/context-tasks/
  src/
    lib.rs
    model/
      ticket.rs        ← TicketId, TicketManifest (id + created_at only)
      schema.rs        ← TicketTypeSchema (configurable fields + state machine)
      edge.rs          ← EdgeRecord (open kind string)
      event.rs         ← HistoryEntry (wraps git commit SHA + diff)
    storage/
      mod.rs           ← TaskStore trait
      index.rs         ← RedbIndexStore (global UUID → path + metadata)
      ticket_fs.rs     ← TicketFs (atomic FS writes, per-ticket lock)
      history.rs       ← GitHistory (git2-backed diff store)
    search/
      mod.rs           ← SearchIndex trait
      tantivy.rs       ← TantivySearchIndex
      query.rs         ← Query AST + parser
    watcher/
      mod.rs           ← FsWatcher (notify-based event loop)
      reconcile.rs     ← orphan detection + parse error reporting
    error.rs
  Cargo.toml
```

## Risks

- Configurable state machines loaded at runtime need a clear schema version story;
  changing a type's state machine after tickets exist requires a migration policy.
  **Mitigated:** schema compatibility policy added below.
- Distributed ticket discovery (Q1) means the index can be stale; define staleness
  tolerance (always-consistent via watcher vs. best-effort with manual `scan`).
  **Mitigated:** read-consistency model defined in Phase 1.
- Git integration: must decide at Phase 0 whether `git2` operates on the workspace
  git repo or on a separate embedded bare repo to avoid polluting user commit history.
  **Decided:** embedded bare repo by default; workspace-git as opt-in flag.

## Schema Compatibility Policy

- Schema version is pinned on each ticket at creation time (`schema_version` field in manifest).
- Additive changes (new optional fields, new states, new edge kinds) are applied in-place
  without requiring migration. Existing tickets gain the new fields with default/null values.
- Breaking changes (removed fields, renamed states, changed transition rules) require an
  explicit `ticket migrate --type <type> --from <v1> --to <v2>` command that:
  1. Reports affected tickets.
  2. Applies a deterministic transformation.
  3. Records the migration in history.
- Tickets created under an older schema version that has no registered migration path
  are validated against their pinned schema version, not the current one.

## TODO

- ~~TODO: Write serde round-trip tests for all schema structs.~~ DONE
- ~~TODO: Define scan root registry format.~~ DONE
- ~~TODO: Decide git repo strategy.~~ DECIDED: embedded bare repo default
- ~~TODO: Design query grammar and write parser tests.~~ DONE
- TODO: Extend command schema export to document `ticket exec` request envelopes, transactional batch semantics, and stdio request IDs.
- ~~TODO: Track progress via EXECUTION_CHECKLIST.md.~~ DONE
