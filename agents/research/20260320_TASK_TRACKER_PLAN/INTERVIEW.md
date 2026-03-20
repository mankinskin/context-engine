# Task Tracker Design Interview

**Purpose:** These 10 questions resolve the design decisions that cascade through every
implementation phase. Answer all questions before Phase 0 begins.

---

## Batch 1 — Identity, Schema & Relationships
*(Shapes Phase 0: contracts, folder layout, redb table keys)*

---

### Q1 — Workspace mapping

Should each `context-engine` workspace have its **own independent ticket store**
(tickets scoped to that workspace's `.context-engine/<ws>/` directory), or should
there be a **single global ticket database per machine** that spans all workspaces?

> **Why it matters:** Determines the root path for the per-ticket folder tree and
> the location of the redb database file. Wrong choice locks you into a migration later.

**Answer:**

Tickets are **distributed across the filesystem** — a ticket folder can be created anywhere
inside a workspace tree. The service discovers them by scanning/watching the tree.
A **global reference index** is maintained either at repo root (`.context-engine/ticket-index/`)
or at user-account level (e.g. `~/.context-engine/ticket-index/`). The exact scope
(repo-root vs. user-level) is TBD but the index is always centralised and rebuilt from
scanning the distributed folders.

---

### Q2 — Ticket ID format

What format do you want for ticket IDs? Options:

| Option | Example | Notes |
|--------|---------|-------|
| A — Sequential padded int | `TCK-00001` | Simple, short, predictable filesystem paths |
| B — Date-prefixed int | `TCK-20260320-001` | Encodes creation date |
| C — Random UUID (v4) | `TCK-a3f2-...` | Collision-free, no central counter |
| D — Human slug | `add-login-page` | Readable, but collision-prone across teams |
| E — Hybrid (slug + int suffix) | `add-login-page-42` | Readable + stable |

> **Why it matters:** ID is the filesystem folder name and the redb primary key.
> Hard to change after first tickets are created.

**Answer:**

**UUID v4** — random, collision-free, no central counter required. Folder name will be
the full UUID string (e.g. `a3f2c7b1-4e9d-4f0a-8c3b-1d2e5f6a7b8c/`). Human-readable
aliases or slugs can be stored in the manifest but are not the primary key.

---

### Q3 — State machine

What are the **valid ticket states** you need, and are transitions **restricted**?

Example states: `open`, `in-progress`, `review`, `blocked`, `done`, `cancelled`

Example restriction: `done` can only be reached from `review`; `cancelled` can be
reached from any state.

Please list:
1. The states you want.
2. Any forbidden transitions (or "any transition is allowed").
3. Whether `blocked` is a first-class state or just a derived flag (blocked because a
   dependency is open).

> **Why it matters:** State transition rules become the core validation logic in
> Phase 0 contracts and Phase 1 write handlers.

**Answer:**

State machines are **fully configurable per ticket type** — the infrastructure provides
the engine but does not mandate any particular states or transitions. Each ticket type
definition (stored in the workspace or user config) declares its own states and
allowed transitions. The system only requires that a state machine definition exists
for a ticket type before tickets of that type can be created. Multiple incompatible
state machines can coexist across the same workspace.

---

### Q4 — Dependency edge types

Do you need **multiple relationship types** between tickets, or a single `depends_on`
edge?

| Type | Semantics |
|------|-----------|
| `blocks` / `is_blocked_by` | A must complete before B can start |
| `relates_to` | Weak informational link |
| `duplicate_of` | B is a duplicate; B is typically closed |
| `child_of` / `parent_of` | Hierarchical decomposition (epic → story → task) |
| `split_from` | B was split out of A |

Please list which edge types you need (or "just `depends_on` for now").

> **Why it matters:** Each edge type may need its own redb table or a discriminant
> column. It also determines what cycle-detection rules are needed.

**Answer:**

Multiple relationship types are needed. The infrastructure provides a **generic edge
registry** — specific workflows define the edge types and their semantics. The engine
need not enumerate all possible edge types at compile time; edge type is a string
discriminant stored in the edge record. Specific workflows (defined in config/plugins)
control which edge types are valid between which ticket types and whether they are
directed/undirected, enforce cycle constraints, etc.

---

### Q5 — Required fields

What fields are **required on every ticket** regardless of type?

Candidates:
- `title` (string, one-liner)
- `description` (markdown body in `description.md`, or inline field?)
- `status` (state machine state)
- `priority` (`low / medium / high / critical`)
- `assignee` (user identifier — string, list, or none)
- `due_date`
- `labels / tags` (list of strings)
- `estimated_effort` (story points / hours)

Also answer:
- Do you need **user-defined custom fields** per ticket type?
- Do you need **multiple ticket types** (bug, feature, task, epic)?

> **Why it matters:** Required fields define the `ticket.toml` schema and serde
> validation structs in Phase 0.

**Answer:**

Only two fields are universally required on every ticket:
- `id` — UUID v4, assigned at creation
- `created_at` — ISO 8601 timestamp

All other fields (title, description, status, priority, assignee, labels, etc.) are
defined by the **ticket type schema**, which is workflow-configurable. The infrastructure
provides a schema validation engine; specific workflows supply the field definitions.
Multiple ticket types (each with their own required/optional fields and state machine)
can coexist in the same workspace.

---

## Batch 2 — Backend Behaviour & Scope
*(Shapes Phases 1–4: locking, history depth, asset pipeline, search MVP threshold)*

---

### Q6 — Concurrency model

Will **only one writer** (one CLI process / one HTTP request) access a workspace at a
time, or do you need **concurrent writers** (e.g., agent + human simultaneously)?

Options:
- **Single-writer lock** (simple, matches current `context-engine` model): global
  `.lock` file per workspace, everyone else waits.
- **Per-ticket locking**: finer granularity; two concurrent writers can touch
  different tickets.
- **Optimistic concurrency**: no lock; version field checked at write time; conflicts
  rejected with error.

> **Why it matters:** Determines lock file placement, redb transaction scope, and how
> HTTP handlers queue mutations.

**Answer:**

**Per-ticket locks.** A `.lock` file (or lock record in redb) is acquired per ticket UUID
before any write to that ticket's folder or index row. Two concurrent writers can
operate on different tickets simultaneously. The global index also needs its own
short-lived write lock when inserting/removing index entries (separate from ticket locks).

---

### Q7 — Versioning depth

Which level of history do you need?

| Level | Capability |
|-------|-----------|
| A — Audit log only | Append-only event records: "who changed what when"; no restore |
| B — Point-in-time restore | Full rollback to any past version of a ticket |
| C — Diff history | View line-level diffs between versions; apply / revert individual diffs |
| D — Git-backed | Ticket folders committed into an embedded git repo via `git2` |

> **Why it matters:** Options A and B are achievable with redb append tables. C needs
> structured diff storage or diffing on read. D adds `libgit2` as a dependency (breaks
> the all-Rust native build goal slightly — `libgit2` is C).

**Answer:**

**Diff history (Option C), with git as the preferred versioning backend** if it is the
best implementation. `libgit2` / `git2` bindings are acceptable as a dependency even
though they are C — correctness and ergonomics take priority over a pure-Rust constraint
here. If the workspace is already a git repo the service should prefer to leverage that;
otherwise it can initialise an embedded bare git repo under the index root for history
storage. Line-level diffs, apply, and revert of individual diffs are required.

---

### Q8 — Binary assets

Are **file attachments** (images, PDFs, screenshots, binary trace logs) first-class in
the MVP, or is **plain text / markdown** the only content type needed initially?

If attachments are needed, please clarify:
- Max expected attachment size (affects whether to store inline in redb or path-only).
- Whether deduplication matters (same file attached to multiple tickets).
- Whether attachments need FTS indexing (PDF text extraction, etc.).

> **Why it matters:** Asset pipeline scope affects Phase 1 folder layout, redb
> `assets` table design, and whether Phase 3 search needs binary content extraction.

**Answer:**

**Attachments are first-class and in scope.** A ticket folder can contain any files
(images, PDFs, binary blobs, trace logs, etc.) either as explicit attachments or as
part of the ticket content. Storage is always path-only in the index (files live in
the ticket folder on disk, never inlined into redb). Deduplication is not required.
Full-text extraction from binary formats (PDF text, etc.) is a best-effort concern
for Phase 3 search — text-extractable formats should be indexed where possible,
binaries that cannot be parsed are indexed by filename/metadata only and do not cause errors.

---

### Q9 — Search in MVP

Does **full-text search (Tantivy)** need to be in Phase 1 (MVP), or is
**metadata-only filter** sufficient for initial use?

Metadata filter = query by status, assignee, tag, priority, date range.
Full-text search = free-text query over ticket title, description, comments.

Also: do you need **highlighted snippets** in search results (show the matched
sentence with the query term bolded)?

> **Why it matters:** Tantivy adds meaningful compile-time weight and index lifecycle
> ops. Deferring it to Phase 3 keeps Phase 1 much simpler.

**Answer:**

**Full-text search is required from the start.** Textual content (ticket fields,
description, comments) must be full-text searchable, and metadata filtering
(state, type, timestamps, edge relationships) must also be available. Both are
exposed through a **unified query language** — one query expression can combine
free-text terms and structured predicates (e.g. `status:open assigned:alice
"login page"`). Highlighted snippets in results are required.

---

### Q10 — Import / greenfield

Is this a **fresh/greenfield** ticket database, or do you need to **import** from an
existing source?

If import is needed, list the source systems (e.g., GitHub Issues, Linear, Jira,
plain markdown files, CSV export).

Also: do you want an **export** pathway (e.g., generate GitHub-compatible issue JSON,
or publish a static HTML board)?

> **Why it matters:** Import/export tooling is a separate Phase 1 concern that affects
> ID assignment, field mapping, and whether a migration crate is needed.

**Answer:**

**Filesystem-change tracking with orphan integration.** The system must:
1. **Watch tracked directories** for changes to known ticket files and update the
   index automatically (or on next scan).
2. **Detect moved/orphaned ticket folders** that appear anywhere in the watched tree
   and attempt to integrate them: parse the ticket manifest, validate against the
   registered schema, and add to the index if valid.
3. **Surface parse/schema errors** clearly when an orphaned or modified ticket cannot
   be parsed — raise an error/diagnostic rather than silently ignoring the folder.
4. A **`ticket scan` / `ticket rebuild-index`** command triggers a full FS walk and
   re-syncs the index from disk state. This is the canonical recovery mechanism.

---

## Summary Checklist

| # | Question | Answered |
|---|----------|---------|
| Q1 | Workspace mapping | ✅ |
| Q2 | Ticket ID format | ✅ |
| Q3 | State machine | ✅ |
| Q4 | Dependency edge types | ✅ |
| Q5 | Required fields | ✅ |
| Q6 | Concurrency model | ✅ |
| Q7 | Versioning depth | ✅ |
| Q8 | Binary assets | ✅ |
| Q9 | Search in MVP | ✅ |
| Q10 | Import / greenfield | ✅ |
