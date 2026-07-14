<!-- aligned-structure:v2 -->

# Summary

Add a durable logical session workspace that carries pinned entities, an evolving execution roadmap, run lineage, and structured handoff state across agent runs.

## Motivation ("why")

Captured transcripts preserve evidence but do not give a resumed agent an authoritative roadmap. A session needs durable attention state and a mutable workflow that survives handoff without replaying the transcript or copying ticket state into a second source of truth.

## Dependent expectation

If this spec is implemented, dependents can rely on one durable `workspace_session_id` spanning multiple capture `run_id` values; a persisted workflow containing ticket-backed and session-only nodes; live resolution of ticket state; structured handoff records; terminal and Mermaid rendering; and explicit graph-gated finish.

## Guards

- `val-session-workflow-persistence`.
- `val-session-workflow-rendering`.
- `val-session-handoff-continuity`.
- `val-session-workflow-finish`.
- `val-session-cli-suite`.
- `val-session-mcp-suite`.

## Positions

- Runtime session capture and transcript store: `implemented` at `memory-api/crates/session-api/src/store.rs`.
- Runtime session model: `partial` at `memory-api/crates/session-api/src/model.rs`.
- Workflow persistence and mutation: `implemented` at `memory-api/crates/session-api/src/model.rs` and `memory-api/crates/session-api/src/store.rs`; validated by `exec-val-session-workflow-persistence-20260714`.
- Terminal and Mermaid rendering: `implemented` at `memory-api/crates/session-api/src/store.rs`; validated by `exec-val-session-workflow-rendering-20260714`.
- Structured handoff/resume and finish: `implemented` at `memory-api/crates/session-api/src/store.rs`; validated by `exec-val-session-handoff-continuity-20260714` and the latest `val-session-workflow-finish` execution. Finish is authoritative: required validation outcomes come only from test-api executions, ticket-backed nodes require live terminal ticket state, and unavailable ticket resolution fails closed. Finished workspaces reject mutations and new lineage; ordinary idempotent init is read-only. Runtime changes and finish serialize under an OS-held exclusive file lock with no age-only reclamation, and releasing an owner does not unlink the stable lock file used by successors.
- Durable JSON writes: `implemented` at `memory-api/crates/session-api/src/store_helpers.rs`. The temp file is synced before `std::fs::rename`; replacement errors preserve the previous destination. Unix parent-directory sync errors are propagated. Windows replace-existing atomicity and directory-entry power-loss durability are not guaranteed by this contract beyond tested failure preservation.
- CLI/MCP surfaces: `implemented` at `memory-api/tools/cli/session-cli/src/lib.rs` and `memory-api/tools/mcp/session-mcp/src/server.rs`; canonical nested `session workflow <subcommand>` hierarchy with flat `workflow-*` compatibility aliases; validated by `exec-session-cli-suite-20260714` and `exec-session-mcp-suite-20260714`.

## Governing-rule requirement

This contract is governed by `.agents/instructions/spec-system.instructions.md` and its aligned-structure v2 requirement.

# Contract

## Identity and lineage

- `workspace_session_id` identifies the durable workspace and is reused across handoffs.
- Every execution receives a distinct `run_id` and optional `predecessor_run_id`.

## Workflow model

- Ticket nodes persist authoritative ticket URNs plus cached display metadata; current state resolves live.
- Session-only nodes represent actions, decisions, checkpoints, or validation work discovered during execution.
- Nodes have stable IDs, required/optional classification, status, and timestamps.
- Directed edges express dependency or execution order and may be added during execution.
- Promotion to a ticket preserves the session node identity and records the resulting ticket URN.

## Persistence and rendering

- Workflow state is stored separately from transcript capture and flushed per mutation.
- A live runtime lock cannot be stolen solely because it is old, and lock release cannot remove a successor's lock instance.
- JSON writes sync the temporary file before rename. Rename failure preserves the prior destination; Unix parent-directory sync is checked. No stronger Windows replace-existing or power-loss guarantee is implied.
- Terminal and Mermaid renders are deterministic, equivalent, safely escaped, and read-only.

## Handoff and finish

- Handoff persists before rendering and includes workspace ID, outgoing run, handoff ID, pins, roadmap state, blockers, validation state, and exact resume command.
- Finish is explicit and idempotent; required nodes and validation must pass.
- Finish and all mutation/init/resume paths serialize under the same runtime lock.
- After finish, ordinary init returns persisted state without rewriting runtime files; init that creates a run, resume, and all other mutations reject.
- Optional nodes may remain incomplete only when explicitly deferred with a reason.

# Non-goals

- Copying ticket lifecycle state into the session store.
- Requiring feedback-api for context or workflow persistence.
- Semantic auto-pinning or replacing the ticket graph.
- Claiming untested Windows replacement or crash/power-loss durability semantics.

# Acceptance Criteria

1. A workspace initializes, mutates, reloads, hands off, and resumes under the same workspace ID with distinct linked runs.
2. Ticket-backed and session-only nodes can be added, updated, linked, and promoted without duplicate identity.
3. Ticket state resolves live; unavailable references produce diagnostics without corruption.
4. Terminal and Mermaid renders deterministically represent the same graph.
5. Handoff persistence precedes rendering and always provides exact resume flow.
6. Finish enforces required work and validation and records terminal success.
7. Feedback emission is optional and non-blocking.
8. Deterministic regressions prove aged live locks remain exclusive, release is ownership-safe, finished init is byte-stable, and finish excludes mutation/init/resume interleavings.

# Traceability

- Parent spec: `8c880efc-7083-4e1d-bf06-96b8254be913`.
- Runtime context spec: `709f067a-21b6-41b6-8879-3cacef4bacaf`.
- Handoff prompt spec: `9e04ff58-9160-4766-b307-74c0fb32a92c`.
- Workflow persistence ticket: `70cd7056-c342-4433-ad60-5bc798f61aa6`.
- Rendering ticket: `cc4b0289-b6fd-412f-a97a-497f05f572f4`.
- Core handoff ticket: `0647a212-9d2e-4943-9627-f854ce3f14c4`.
- Transport ticket: `6b2dc497-188c-44f5-9106-bf35deecb7a1`.
- Prompt update ticket: `9577b114-ec11-431b-8740-c488bef05fc9`.
- Remediation ticket: `6b1edff1-bc32-40c7-b3a9-fb1292b0213f`.
