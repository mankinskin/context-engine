# Goal
Extend `session-api` (`memory-api/crates/session-api`) with a runtime "cognitive workspace": a `session_context.json` document and core operations to create/resume it, pin/unpin entities (as cross-store URNs), render short headers, and record pin usage — without disturbing the existing capture/transcript/worktree path.

# Problem
`SessionStoreConfig` today only writes `session.json` (manifest) and `transcript.json` (append-only) and reads/queries them. There is no per-session mutable attention state, so an agent cannot record "these are the rules/specs/tickets I am actively working against," cannot resume that state across turns, and produces no usage signal.

# Scope
- Add a `session_context.json` document persisted under the session directory, alongside the existing `session.json`/`transcript.json`.
- Core operations on `SessionStoreConfig`:
  - `init_context(session_id)` — **load-or-create**; creates the session directory if absent; idempotent (re-init resumes and never clobbers existing pins).
  - `pin(session_id, entity)` / `unpin(session_id, entity)` — add/remove an entity (by URN) from `pinned_entities`; pinning the same URN twice is a no-op (no duplicates); each successful `pin` emits a **usage event** for that entity.
  - `read_context(session_id)` — return the parsed context.
  - `render_view(session_id)` — return pinned entities as **short headers only** (urn, type, title|slug, relation, reason); never full bodies.
- File-backed persistence: every mutating call flushes `session_context.json` to disk before returning.
- The archive/capture path (`persist_capture`, `read_session`, `query_sessions`, worktree check-in) is untouched; `session_context.json` is additive and ignored by those paths.

# session_context.json schema (frozen for this slice)
```jsonc
{
  "session_id": "string",              // matches the capture session id (D1)
  "schema_version": 1,
  "created_at": "RFC3339",
  "updated_at": "RFC3339",             // bumped on every mutation (resume activity)
  "pinned_entities": {
    "tickets": [{ "urn": "ce://<ws>/<store>/<id>", "relation": "primary_focus|blocked_by|related", "reason": "string?" }],
    "specs":   [{ "urn": "ce://<ws>/<store>/<id>", "section": "string?", "reason": "string?" }],
    "rules":   [{ "urn": "ce://<ws>/<store>/<id>", "filter": "string?", "reason": "string?" }]
  }
}
```
Notes: no `current_mode` (D8). Entity references are URNs (D2). A general-chat session is a valid context with all three buckets empty (D8).

# Non-goals
- The cascade auto-pin logic (separate child spec).
- CLI/MCP surfaces (separate child spec; this spec is the core lib contract they call).
- The usage/feedback storage backend itself — this slice **emits** usage events into the generic memory-api usage/feedback model; it does not implement that model.
- Resolving entity bodies — `render_view` returns headers only; body fetch is the agent's explicit follow-up.

# Acceptance Criteria (test-validatable)
1. `init_context` on a fresh session creates the session directory and `session_context.json` with `schema_version`, empty `pinned_entities`, and timestamps. *(unit test asserts dir + file exist + parsed fields)*
2. `init_context` called twice for the same session resumes the existing context and preserves any pins added between calls (no clobber); `updated_at` is non-decreasing. *(unit test pins then re-inits, asserts pin survives)*
3. `pin` adds an entity to the correct bucket by URN and persists immediately; re-pinning the same URN does not create a duplicate. *(unit test asserts len==1 after double pin)*
4. Each successful `pin` emits exactly one usage event for the entity URN. *(unit test asserts one event recorded via the injected usage sink)*
5. `unpin` removes a previously pinned URN and persists; unpinning a missing URN is a no-op without error. *(unit test)*
6. `render_view` returns short headers (urn, type, title|slug, relation, reason) for each pinned entity and **never** includes full bodies. *(unit test asserts header fields present and body field absent)*
7. Persisting a capture for the same session (`persist_capture`) leaves `session_context.json` byte-identical, and writing `session_context.json` leaves `session.json`/`transcript.json` byte-identical. *(regression unit test)*
8. All new behavior is covered by focused `cargo test -p session-api` unit tests against a `TempDir`.

# Traceability
- Parent: `memory-api/session-api/dynamic-session-bootstrapping`
- Ticket: [412964a3 runtime session-context model](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/412964a3-e1c3-47da-94ad-268ff20441c0/ticket.toml)
- Depends on (cross-store): [82d6ada4 URN resolver](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/82d6ada4-ac35-45a7-9df6-7b7501d58e70/ticket.toml)
- Consumed by: [6b2dc497 init/pin/unpin/view surfaces](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/6b2dc497-188c-44f5-9106-bf35deecb7a1/ticket.toml), [d8f76965 cascade context gathering](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/d8f76965-1ff3-4a0a-bb24-773b9637fae4/ticket.toml)

# Validation
- ValidationSpec: focused `session-api` unit tests for context create/resume/pin/unpin/view, usage emission, and capture-path regression.
- ValidationExecution (planned): `cargo test -p session-api`.