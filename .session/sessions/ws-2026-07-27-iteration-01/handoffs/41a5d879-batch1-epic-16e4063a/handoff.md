# Implementation Handoff: Epic 16e4063a Batch 1

**Handoff ID:** 41a5d879-batch1-epic-16e4063a  
**Workspace Session:** ws-2026-07-27-iteration-01  
**Created:** 2026-07-27T21:47:30Z

## Objective

Land Batch 1 in ONE implementation session, committing at the end: flatten the session store layout (eliminating `.session/runtime/` in favor of `.session/sessions/<session_id>/`), add track fields (`track_id`, `anchor_ticket_id`, `parent_session_id`) to the session schema, and expose track query and rollup over MCP and CLI with parity.

## Target Tickets (9)

Implementation order (strict):

1. **7a4f9c3d** - REOPENED: Flatten session store layout (.session/runtime/ → .session/sessions/<id>/)
2. **4817a5cc** - REOPENED: Ensure handoffs persist under sessions/<id>/handoffs/
3. **41ed4585** - Move .session/runtime/ persistence to .session/sessions/<id>/
4. **0a45bedb** - Add .session/local/ for git-ignored pointers
5. **ab02e15a** - Add track_id, anchor_ticket_id, parent_session_id to session schema
6. **fd7737ec** - Implement track query over MCP
7. **938a7ae9** - Implement track query over CLI  
8. **185a00a2** - Implement track rollup over MCP
9. **9cd9440e** - Implement track rollup over CLI

## Target Files

- `memory-api/crates/session-api/src/store.rs`
- `memory-api/crates/session-api/src/model.rs`
- `memory-api/crates/session-api/src/persistence.rs`
- `memory-api/crates/session-api/src/store_persistence_types.rs`
- `memory-api/crates/session-api/src/subagent_rollup.rs`
- `memory-api/crates/session-api/src/tool_metrics.rs`
- `memory-api/tools/mcp/session-mcp/src/server.rs`
- `memory-api/tools/cli/session-cli/src/lib.rs`
- `.gitignore`
- `memory-api/.gitignore`

## Decisions (Locked)

1. No track entity, no new store, no track status field. Track = 3 nullable session schema fields.
2. Anchor ticket owns goal/DoD; spec owns requirements; NO goal text on session records.
3. Track completes when its anchor ticket closes.
4. Migration is fail-open; existing sessions load without error and report `track_id = null`.
5. Conformance gate belongs in the handoff WRITE path (`session_handoff`), fail closed — NOT in this batch (ticket 7bb007e9).
6. Lightweight sub-agent sessions = same schema + lazy artifacts — NOT in this batch (be1552ba).
7. Traceability injection — NOT in this batch (490f1cbc).
8. Sub-agent routing and board/WIP — NOT in this batch (c410ca60, 648a64a6).
9. Flatten layout: eliminate `.session/runtime/workspaces/` in favor of `.session/sessions/<session_id>/` owning `context.json`, `handoffs/`, and `finish.json` directly.
10. Keep `.session/local/` for pointers and locks (git-ignored); track durable artifacts under `sessions/<id>/`.
11. Handoffs persist as folders with both `handoff.json` and rendered `handoff.md`.

## Non-Goals

- Do not start tickets: 7bb007e9, be1552ba, c410ca60, 648a64a6, 490f1cbc, b43363e1
- Do not implement parallel sessions (25dd26cb, deferred)
- Do not create a track entity or a new store
- Do not add response post-processing
- Do not touch effba966's subtree (downgraded from hard prerequisite)
- Do not implement handoff conformance gate (7bb007e9)

## Context Anchors

- **Anchor ticket:** [16e4063a](.ticket/tickets/16e4063a-32c6-416c-a6fe-160df9f9edd0/ticket.toml) Track-scoped multi-session execution (epic)
- **Spec:** [7b277ba4](.spec/specs/7b277ba4-7deb-4f24-a3c4-88ea0d8e0a4f/spec.toml) Requirements specification
- **Source proposal:** `transcripts/27-07-2026_session-track-management/input.clean.md`
- **Prior art (do not redo):** 1fbf2d84, 0647a212, 76e831f2, 5755b694, d3af78d7
- **Draft docs:** `tmp/spec-iteration-loop.md`, `tmp/spec-handoff-package-schema.md`
- **Commits from planning:** 924d25c8, b2aee53a, 2c799b10, 647d68a6
- **Closed prerequisite:** 3eaceaae (done)
- **Review findings:**
  - 7a4f9c3d REOPENED: `runtime_paths_for_workspace` missing, `.session/runtime/` still exists
  - 4817a5cc REOPENED: AC1 handoffs not under `sessions/<id>/`, AC4 no `events.json` decision doc
- **Existing test files:** `memory-api/crates/session-api/src/store_tests.rs`, `memory-api/crates/session-api/src/hook/tests.rs`

## Risk Notes

Tickets 7a4f9c3d and 4817a5cc are REOPENED after review failures. Prior implementation attempted layout flatten but placed artifacts at `.session/runtime/workspaces/` instead of `.session/sessions/<session_id>/`. 

This handoff requires the implementer to complete the layout work correctly:
- Ensure `runtime_paths_for_workspace` exists and points to the new unified location
- Verify `.session/local/` is present for ignored pointers
- Confirm `.session/runtime/` is fully eliminated

## Validation Gates (All Required)

1. **cargo-test-session-api**: `cargo test -p session-api`
2. **cargo-test-session-mcp**: `cargo test -p session-mcp`
3. **memory-kernel-board-tests**: `cargo test -p memory-kernel --test board`
4. **session-lifecycle-layout-track**: NEW end-to-end test asserting check-in → handoff write → finish → reload with on-disk layout AND track fields
5. **track-query-mcp-cli-parity**: NEW MCP and CLI round-trip through track query surface
6. **migration-test-nullable-track**: NEW migration test proving pre-existing sessions load with `track_id = null`

## Open Escalations

None. This package is implementation-ready.

---

**Resume command:**  
```bash
session-cli resume --workspace-session-id ws-2026-07-27-iteration-01 --predecessor-run-id 2d487450-3c1b-45da-9b96-80eab42d05e0
```
