<!-- aligned-structure:v2 -->

# Summary

Add the durable runtime context foundation for pinned entities and workspace/run identity without disturbing capture/archive storage.

## Motivation ("why")

A transcript does not preserve active attention state in a compact, mutable form. Agents need to resume the same logical workspace, retain selected entities, and start a distinct linked run without replaying prior turns.

## Single session identity decision

`session_id` is the one and only session identity. Its value is the capture/provisioning UUID. `workspace_session_id` is eliminated as a distinct identity concept. A human-readable slug is a display/topic label only: it is never a key, lookup handle, or command-accepted identifier.

Two aliasable identity handles produced a bootstrap chain incompatible with itself. `init_runtime_context` assigned `session_id = workspace_session_id`, and unconditional active-marker reuse preserved stale slugs such as `epic-kickoff-8fdfe135`. `session init` then returned the stale slug while worktree provisioning used UUID `a2659767-3224-48e2-a9a9-72c9582c8515`; init later minted UUID `a808ee90-191a-47e2-a609-4a49a967d778` in the same worktree, preventing the provisioning-UUID check-in.

The active marker at the incident time was `{"workspace_session_id":"epic-kickoff-8fdfe135","updated_at":"2026-08-12T10:35:58.180321300Z"}`. Session persistence is anchored at `memory-api/crates/session-api/src/store/config/persistence.rs` for `session.json` (about line 108), `context.json` (about line 146), and marker reuse (about line 208); identity input validation is in `memory-api/crates/session-api/src/store/helpers/storage.rs` (about line 267).

## Dependent expectation

If this spec is implemented, dependents can rely on one UUID-shaped session handle for capture, provisioning, initialization, runtime-context lookup, and session-requiring commands; durable pinned entity URNs; distinct linked run IDs; immediate file-backed persistence; headers-only views; and optional non-blocking feedback emission. Once a session is finished, ordinary init is a byte-stable read and new run lineage is rejected.

## Guards

- `val-session-bootstrap-schema-validation`.
- `val-session-init-idempotency`.
- `val-session-context-capture-isolation`.
- `val-session-run-lineage`.
- `val-session-workflow-finish`.

## Positions

- Existing capture models: `implemented` at `memory-api/crates/session-api/src/model.rs`.
- Existing capture persistence: `implemented` at `memory-api/crates/session-api/src/store.rs`.
- Runtime context model and operations: `implemented` at `memory-api/crates/session-api/src/model.rs` and `memory-api/crates/session-api/src/store.rs`; validated by `exec-val-session-init-idempotency-20260714`, `exec-val-session-run-lineage-20260714`, `exec-val-session-context-capture-isolation-20260714`, and the latest `val-session-workflow-finish` execution.
- Single-handle identity, UUID validation, and marker freshness: `partial` at `memory-api/crates/session-api/src/store/config/persistence.rs` and `memory-api/crates/session-api/src/store/helpers/storage.rs`; the current unconditional marker reuse and path-segment-only validation do not meet this contract.

## Governing-rule requirement

This contract is governed by `.agents/instructions/spec/spec-system.instructions.md` and its aligned-structure v2 requirement.

# Contract

- `session_id` is the only session identity and is the capture/provisioning UUID.
- A slug is display/topic metadata only; a slug is never a persistence key, lookup handle, or session-identity command argument.
- Runtime context belongs to the session and persists as `.session/sessions/<uuid>/context.json`, keyed by the session UUID in the same directory family as `session.json`. This is a keying rule, not a storage migration.
- Session-requiring commands reject a non-UUID-shaped identity before storage lookup with an actionable error that names the UUID requirement.
- `.session/local/active_workspace_session.json` is reused only after an explicit current-conversation binding and freshness check; a stale marker cannot determine `session_id`.
- Every agent execution has a distinct `run_id` and optional `predecessor_run_id`.
- Initialization is load-or-create and never clobbers pins or lineage.
- Finished-workspace ordinary init returns persisted context without updating timestamps or rewriting `context.json` or `active_workspace_session.json`; force-new-run init and resume reject.
- Init, resume, finish, and runtime mutations serialize under one OS-held exclusive workspace lock; lock age alone never permits reclamation.
- Pinned tickets, specs, and rules are cross-store URNs with relation/reason metadata.
- Mutations flush before returning within the platform guarantees declared by the workflow spec.
- Read/view returns short headers and metadata, never full entity bodies.
- Duplicate pin and missing unpin are idempotent no-ops.
- Feedback usage emission is optional through an injected sink; sink absence or failure cannot corrupt or reject a successful pin.
- Existing capture manifests and transcripts remain byte-identical when context mutates, and context remains byte-identical when capture persists.

# Non-goals

- Workflow graph persistence, rendering, handoff, and finish, owned by `c677182e-90da-4ac3-8b94-9e2e97c825cf`.
- Deleting, migrating, or restructuring the valid legacy slug-keyed runtime-context directories `epic-kickoff-8fdfe135` and `structured-ticket-entities-iteration`.
- Cascade auto-pinning.
- CLI/MCP transport.
- Full-body entity resolution.

# Acceptance Criteria

1. Fresh initialization creates durable context under `.session/sessions/<uuid>/context.json` with the same UUID as capture and provisioning, schema version, timestamps, empty pins, and initial run lineage.
2. No runtime-context operation persists, keys, resolves, or accepts `workspace_session_id` as a distinct identity; slugs remain display/topic labels only.
3. Session-requiring commands reject non-UUID-shaped values before storage lookup with an actionable UUID-requirement error.
4. Active-marker reuse requires an explicit current-conversation binding and freshness check; a stale marker cannot set `session_id`.
5. Existing slug-keyed runtime-context directories, including `epic-kickoff-8fdfe135` and `structured-ticket-entities-iteration`, remain readable and unchanged.
6. Resume preserves pins and adds a distinct linked run without reusing the outgoing run ID before finish; resume rejects after finish.
7. Plain init after finish is read-only and leaves runtime context and active-workspace files byte-identical.
8. Pin/unpin is idempotent and immediately persistent.
9. Headers-only view never includes full bodies.
10. Optional feedback sink receives successful pin usage when configured and cannot block persistence.
11. Capture/context byte-isolation regressions pass.
12. Focused `cargo test -p session-api` coverage validates the contract.

# Traceability

- Parent spec: `8c880efc-7083-4e1d-bf06-96b8254be913`.
- Implementation ticket: `412964a3-e1c3-47da-94ad-268ff20441c0`.
- Remediation ticket: `6b1edff1-bc32-40c7-b3a9-fb1292b0213f`.
- Downstream workflow ticket: `70cd7056-c342-4433-ad60-5bc798f61aa6`.
- Downstream transport ticket: `6b2dc497-188c-44f5-9106-bf35deecb7a1`.
- Identity-decoupling ticket: `76c64b38-25e9-484c-818c-365f15114c89` (session-api, high priority, "Decouple Copilot session UUID from workspace runtime context identity").
