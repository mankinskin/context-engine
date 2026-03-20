# Phase 0 — Design Contracts

**Status:** READY (all INTERVIEW.md answers complete)

## Objective

Produce the canonical schemas and contracts that every later phase compiles against.
Nothing in Phase 1–4 should hard-code values that belong here.

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
- Distributed ticket discovery (Q1) means the index can be stale; define staleness
  tolerance (always-consistent via watcher vs. best-effort with manual `scan`).
- Git integration: must decide at Phase 0 whether `git2` operates on the workspace
  git repo or on a separate embedded bare repo to avoid polluting user commit history.

## TODO

- TODO: Write serde round-trip tests for all schema structs.
- TODO: Define scan root registry format (how the user registers watched directories).
- TODO: Decide git repo strategy: workspace repo with a dedicated branch vs. bare repo
  under the index root.
- TODO: Design query grammar and write parser tests before Phase 3 search work begins.
