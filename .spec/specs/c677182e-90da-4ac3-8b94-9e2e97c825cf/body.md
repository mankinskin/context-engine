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
- Structured handoff/resume and finish: `implemented` at `memory-api/crates/session-api/src/store.rs`; validated by `exec-val-session-handoff-continuity-20260714` and `exec-val-session-workflow-finish-20260714`. Finish is authoritative: required validation outcomes come only from test-api executions (caller-supplied outcomes never certify), ticket-backed nodes require live terminal ticket state, missing/failed/misrouted ticket resolution fails closed, finished workspaces are immutable across pins, workflow mutations, and init/resume lineage updates, every runtime mutation (including lineage) serializes under a per-workspace mutation lock whose finished-check is evaluated while the lock is held so finish cannot race a concurrent mutation, and durable writes use a single atomic rename (`MoveFileExW` replace-existing on Windows, `rename(2)` on Unix) so no crash window can leave the destination absent.
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
- Terminal and Mermaid renders are deterministic, equivalent, safely escaped, and read-only.

## Handoff and finish

- Handoff persists before rendering and includes workspace ID, outgoing run, handoff ID, pins, roadmap state, blockers, validation state, and exact resume command.
- Finish is explicit and idempotent; required nodes and validation must pass.
- Optional nodes may remain incomplete only when explicitly deferred with a reason.

# Non-goals

- Copying ticket lifecycle state into the session store.
- Requiring feedback-api for context or workflow persistence.
- Semantic auto-pinning or replacing the ticket graph.

# Acceptance Criteria

1. A workspace initializes, mutates, reloads, hands off, and resumes under the same workspace ID with distinct linked runs.
2. Ticket-backed and session-only nodes can be added, updated, linked, and promoted without duplicate identity.
3. Ticket state resolves live; unavailable references produce diagnostics without corruption.
4. Terminal and Mermaid renders deterministically represent the same graph.
5. Handoff persistence precedes rendering and always provides exact resume flow.
6. Finish enforces required work and validation and records terminal success.
7. Feedback emission is optional and non-blocking.

# Traceability

- Parent spec: `8c880efc-7083-4e1d-bf06-96b8254be913`.
- Runtime context spec: `709f067a-21b6-41b6-8879-3cacef4bacaf`.
- Handoff prompt spec: `9e04ff58-9160-4766-b307-74c0fb32a92c`.
- Workflow persistence ticket: `70cd7056-c342-4433-ad60-5bc798f61aa6`.
- Rendering ticket: `cc4b0289-b6fd-412f-a97a-497f05f572f4`.
- Core handoff ticket: `0647a212-9d2e-4943-9627-f854ce3f14c4`.
- Transport ticket: `6b2dc497-188c-44f5-9106-bf35deecb7a1`.
- Prompt update ticket: `9577b114-ec11-431b-8740-c488bef05fc9`.
- Remediation ticket (authoritative finish, live ticket state, atomic durability, CLI contract): `6b1edff1-bc32-40c7-b3a9-fb1292b0213f`.
