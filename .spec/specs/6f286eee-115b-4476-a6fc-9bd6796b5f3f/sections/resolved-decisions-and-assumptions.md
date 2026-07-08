These resolve the five open planning questions. They were decided from repo
conventions and the design blueprint during the refinement pass. No live
stakeholder was available, so each is an **assumption of record** the operator
may revise before implementation; revisions must update this section and the epic.

### D1 — Crate paths and naming
New top-level domain directory `agent-harness/`, mirroring the repo's grouped
layout (`memory-api/`, `context-stack/`, `viewer-api/`). Members, added to the
root `Cargo.toml` workspace:
- `agent-harness/agent-shared` — protocol types, tagged serde lifecycle events.
- `agent-harness/agent-core` — ReAct loop, session model, provider abstraction, hooks.
- `agent-harness/agent-server` — Axum lifecycle + websocket broadcast.
- `agent-harness/agent-tui` — Ratatui native operator client.
- `agent-harness/agent-web/frontend/dioxus` — Dioxus/WASM operator client (matches
  the repo's `frontend/dioxus` viewer convention).
- `agent-harness/tools/cli/agent-cli` — headless operator CLI (optional, thin).

### D2 — Checkpoint / event persistence backend
v1 uses an **append-only NDJSON event log per session** on the local filesystem
with a materialized index, matching the repo's established
`append-only-history-materialized-index` pattern (ticket/spec stores). Checkpoints
are periodic snapshots referencing the last applied event offset. SQLite is
deferred until a concrete scale or query need is demonstrated.

### D3 — Isolation model beyond per-session sandbox
v1 boundary = per-session working directory + Docker-based command sandbox
(`bollard`) for any host command execution. Multi-tenant OS-level isolation,
seccomp/user-namespace hardening, and network egress policy are **non-goals for
v1** (single trusted operator, single host). The sandbox policy is pluggable so a
stricter backend can replace it later without changing the session model.

### D4 — Concurrency limits and default budgets
Defaults, all configurable:
- `max_concurrent_loops = 4` (interactive sessions are not counted against this).
- Per-loop token budget default `200_000` tokens; hard stop with a resumable checkpoint.
- Per-iteration caps: max tool calls, max wall-clock per iteration, max consecutive
  no-progress iterations before watchdog pause.
- Preflight budget/policy hooks run before each expensive provider call and before
  each command execution.

### D5 — `.agentguidance` merge precedence
Deterministic layering, most-specific wins, reusing the repo's
`instruction-precedence-and-exceptions` model:
`repo-root global` < `path-scoped` < `session/task-specific`. On equal specificity,
the newer/explicitly-scoped layer wins and the override is recorded in the session
audit trail. Merge output is deterministic for a given input set (test-asserted).
