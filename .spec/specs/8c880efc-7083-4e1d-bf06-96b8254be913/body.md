<!-- aligned-structure:v2 -->

# Summary

Turn `session-api` from capture/archive-only storage into a durable runtime workspace with selective pinned context, an evolving workflow roadmap, structured handoff continuity, and explicit completion gates.

## Motivation ("why")

Always-on instruction loading and transcript replay consume model context without telling a resumed agent what work remains. Durable session state must preserve active knowledge and execution intent while keeping ticket state authoritative in the ticket store.

## Dependent expectation

If this spec is implemented, dependents can rely on a durable `workspace_session_id` spanning distinct linked run IDs; pinned entity URNs; a mutable ticket-backed workflow; terminal and Mermaid views; structured handoff/resume; and graph-gated finish.

## Guards

- `val-session-bootstrap-schema-validation`.
- `val-session-init-idempotency`.
- `val-session-workflow-persistence`.
- `val-session-handoff-continuity`.
- `val-session-workflow-finish`.

## Positions

- Capture/archive store: `implemented` at `memory-api/crates/session-api/src/store.rs`.
- Capture models: `implemented` at `memory-api/crates/session-api/src/model.rs`.
- Runtime pinned context: `implemented` at `memory-api/crates/session-api/src/store.rs`; delivered by ticket `412964a3-e1c3-47da-94ad-268ff20441c0`.
- Durable workflow persistence: `implemented` at `memory-api/crates/session-api/src/store.rs`; delivered by ticket `70cd7056-c342-4433-ad60-5bc798f61aa6`.
- Terminal/Mermaid rendering: `implemented` at `memory-api/crates/session-api/src/store.rs`; delivered by ticket `cc4b0289-b6fd-412f-a97a-497f05f572f4`.
- Structured handoff/resume and finish: `implemented` at `memory-api/crates/session-api/src/store.rs`; delivered by ticket `0647a212-9d2e-4943-9627-f854ce3f14c4` and remediated by ticket `6b1edff1-bc32-40c7-b3a9-fb1292b0213f`.
- CLI/MCP bootstrap surfaces: `implemented` at `memory-api/tools/cli/session-cli/src/lib.rs` and `memory-api/tools/mcp/session-mcp/src/server.rs`; delivered by ticket `6b2dc497-188c-44f5-9106-bf35deecb7a1`.
- Hard-link cascade: `not-implemented`; planned by ticket `d8f76965-1ff3-4a0a-bb24-773b9637fae4`.

## Governing-rule requirement

This contract is governed by `.agents/instructions/spec-system.instructions.md` and its aligned-structure v2 requirement.

# Contract

- Client-side rendering consumes structured session views; runtime code does not rewrite instruction files.
- File-backed mutations flush before returning within the platform durability guarantees declared by the durable workflow child spec.
- Entity references use cross-store URNs and hard links; semantic search never silently auto-pins.
- `workspace_session_id` is durable across handoff; every execution creates a distinct `run_id` with predecessor linkage before finish.
- Pinned context and workflow state survive resume without transcript replay.
- Ticket nodes resolve current state live; session-only nodes may represent discovered actions and later be promoted to tickets.
- Handoff records persist before prompt rendering and always supply an exact resume command.
- Finish is explicit and requires all required nodes and validation gates to be satisfied.
- Finish is terminal: runtime mutation and new lineage reject afterward, while ordinary init is read-only and byte-stable.
- Runtime mutation, init/resume, and finish share an OS-held exclusive workspace lock that cannot be stolen based solely on age.
- Feedback usage/rating emission is optional through an adapter and cannot block context or workflow operations.

# Non-goals

- Replacing ticket-api as workflow authority.
- Copying ticket lifecycle state into session files.
- Gating core session functionality on the full feedback program.
- Semantic auto-pinning from vague text matches.
- Claiming stronger cross-platform replacement or power-loss durability than platform-specific evidence establishes.

# Acceptance Criteria

1. Durable context initializes and resumes idempotently under one workspace ID across distinct runs before finish; ordinary init after finish is byte-stable and resume rejects.
2. Pinned entities and workflow nodes/edges persist independently of transcript capture.
3. Agents can add discovered work during execution and promote temporary nodes to tickets.
4. Terminal and Mermaid views deterministically represent the same roadmap.
5. Handoff always persists and carries workspace ID, run lineage, blockers, validation state, and exact resume command.
6. Finish rejects incomplete required work, excludes concurrent mutation/init/resume, and accepts a validated complete workflow.
7. Existing capture artifacts remain backward compatible.

# Traceability

- Epic: `effba966-f0a8-4d7d-b289-b7feba826cf8`.
- Runtime context child: `709f067a-21b6-41b6-8879-3cacef4bacaf`.
- Durable workflow child: `c677182e-90da-4ac3-8b94-9e2e97c825cf`.
- Cascade child: `fda5c915-5c37-432f-acf1-8b20c6219fdc`.
- Selective-loading child: `a28a88db-2b41-49d2-897c-6dbcdb313255`.
- Remediation ticket: `6b1edff1-bc32-40c7-b3a9-fb1292b0213f`.
