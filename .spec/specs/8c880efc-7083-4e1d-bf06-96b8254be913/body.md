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
- Structured handoff/resume and finish: `partial` at `memory-api/crates/session-api/src/store.rs`; delivered by ticket `0647a212-9d2e-4943-9627-f854ce3f14c4` and remediated by ticket `6b1edff1-bc32-40c7-b3a9-fb1292b0213f`. Persisted `SessionHandoffRecord` (`memory-api/crates/session-api/src/model.rs`) omits AC5 `blockers` and resolved validation `outcome`, and carries no structured narrative, so knowledge transfer falls back to pasted prose; remediated by ticket `6431985e-e729-426b-9f91-66ad4b1c6fe6`.
- Handoff completeness against session-created/owned entities: `not-implemented`; planned by ticket `0d3fdba6-45e6-4129-84f7-d98324c9519d`.
- Resume-time pin resolution and dead-pin GC: `not-implemented`; `render_instructions` currently hard-fails on an unresolved pin; planned by ticket `e731d333-7c11-4f9d-aec5-277c84be3796`.
- Compact handoff serialization and snapshot schema hygiene: `not-implemented`; records re-serialize the full workflow per handoff and double-nest `workflow.workflow`; planned by ticket `96f9ffaa-6514-480a-afbd-b345cc206863`.
- Event-capture de-duplication: `not-implemented`; `data_json`/`raw_event_json` store the same payload twice; planned by ticket `67d7c279-6661-461b-8204-7a1bd7e028c5`.
- Session store layout and identity: `not-implemented`; runtime threads live under `runtime/workspaces/<workspace_session_id>/` decoupled from captured `sessions/<session_id>/` with no join field; flattening + identity unification planned by tracker `0a45bedb-6dfe-466e-893f-fddfd225f1f6` (identity `fc86f42d-3fc0-4f07-911a-525098248dcf`, layout `7a4f9c3d-bf5f-4849-93c7-b8c2706dac61`).
- Handoff artifact persistence (folders + markdown + git): `not-implemented`; handoffs are single JSON files under a git-ignored `runtime/` tree, so no markdown form persists and research loops cannot read them from git; folder form with `handoff.json` + `handoff.md` planned by ticket `41ed4585-5b1e-4681-96e8-4883ed140c18` and git-tracking policy by ticket `4817a5cc-5e91-4280-b7ed-aed296a480b3`.
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
- Durable runtime artifacts (context, handoffs, finish) are owned by the session under `.session/sessions/<session_id>/` and git-tracked for feedback/research loops; handoffs persist in folders with both JSON and rendered markdown; only machine-local pointers and locks stay git-ignored.
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
5. Handoff always persists and carries workspace ID, run lineage, blockers, resolved validation outcome, a structured narrative (summary, remaining work, decisions), session-created/owned entities, and exact resume command. (Current implementation persists only workspace ID, run lineage, pins, workflow, required-gate ids, and resume command; blockers, resolved outcome, and narrative are unmet — tracked by ticket `6431985e-e729-426b-9f91-66ad4b1c6fe6`.)
6. Finish rejects incomplete required work, excludes concurrent mutation/init/resume, and accepts a validated complete workflow.
7. Existing capture artifacts remain backward compatible.
8. Runtime context, handoffs, and finish are owned directly by `sessions/<session_id>/` (no `runtime/workspaces/` nesting) and joined to captured transcripts by a `session_id`; handoffs persist as folders with `handoff.json` + `handoff.md`; durable artifacts are git-tracked while local pointers/locks are ignored; legacy layouts still load. (Planned by flattening tracker `0a45bedb-6dfe-466e-893f-fddfd225f1f6`; gates further handoff/schema work.)

# Traceability

- Epic: `effba966-f0a8-4d7d-b289-b7feba826cf8`.
- Runtime context child: `709f067a-21b6-41b6-8879-3cacef4bacaf`.
- Durable workflow child: `c677182e-90da-4ac3-8b94-9e2e97c825cf`.
- Cascade child: `fda5c915-5c37-432f-acf1-8b20c6219fdc`.
- Selective-loading child: `a28a88db-2b41-49d2-897c-6dbcdb313255`.
- Remediation ticket: `6b1edff1-bc32-40c7-b3a9-fb1292b0213f`.
- Handoff payload remediation (AC5 blockers/outcome/narrative): `6431985e-e729-426b-9f91-66ad4b1c6fe6`.
- Handoff completeness gate: `0d3fdba6-45e6-4129-84f7-d98324c9519d`.
- Resume-time pin resolution and dead-pin GC: `e731d333-7c11-4f9d-aec5-277c84be3796`.
- Delta handoff serialization and snapshot cleanup: `96f9ffaa-6514-480a-afbd-b345cc206863`.
- Event-capture de-duplication: `67d7c279-6661-461b-8204-7a1bd7e028c5`.
- Store flatten + identity + git-tracking tracker (gates further work): `0a45bedb-6dfe-466e-893f-fddfd225f1f6`.
  - Identity unification (session_id join): `fc86f42d-3fc0-4f07-911a-525098248dcf`.
  - Layout flatten + local pointers/locks: `7a4f9c3d-bf5f-4849-93c7-b8c2706dac61`.
  - Handoff folders + rendered markdown: `41ed4585-5b1e-4681-96e8-4883ed140c18`.
  - Git-tracking policy: `4817a5cc-5e91-4280-b7ed-aed296a480b3`.
