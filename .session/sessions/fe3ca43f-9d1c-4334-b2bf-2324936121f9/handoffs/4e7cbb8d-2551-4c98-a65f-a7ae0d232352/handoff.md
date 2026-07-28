# Handoff: 4e7cbb8d-2551-4c98-a65f-a7ae0d232352

## Summary
- **Workspace Session**: `fe3ca43f-9d1c-4334-b2bf-2324936121f9`
- **Outgoing Run**: `096955dd-b238-42d8-93e3-5f5cbf27dab4`
- **Created**: 2026-07-28T00:13:10.413642700Z
- **Objective**: Remove the legacy runtime-path fallback shims (ticket 7fabc77a), then untrack the 76 committed events.json files (ticket 580019e8), clearing the way for the remaining Batch 1 chain starting at ab02e15a.
- **Implementation Ready**: True

## Resume Command
```bash
session-cli resume --workspace-session-id fe3ca43f-9d1c-4334-b2bf-2324936121f9 --predecessor-run-id 096955dd-b238-42d8-93e3-5f5cbf27dab4
```

## Target Tickets
- `7fabc77a-7704-448b-aef8-1f3e22dd18dd`
- `580019e8-b253-42ee-80fc-990f6d26baf6`

## Target Files
- `memory-api/crates/session-api/src/store/config/persistence.rs`
- `memory-api/crates/session-api/src/store/config/worktree_runtime.rs`
- `memory-api/crates/session-api/tests/gitignore_policy.rs`
- `.gitignore`
- `memory-api/.gitignore`

## Decisions
- Legacy runtime-path back-compat removed rather than retained as a permanent shim; removal is fail-closed and stale legacy trees on other machines are abandoned, not migrated.
- Decision records live in the spec, not in standalone ADR files; memory-api/docs/decisions/ was rejected and deleted.
- 4817a5cc AC1 closed on git check-ignore evidence; the commit landed this iteration serves as the end-to-end proof that durable artifacts are tracked.
- Untracking the 76 committed events.json files (~99MB) stays scoped to ticket 580019e8, which now depends_on 4817a5cc.
- Only the active track's own session directory is committed; other agents' session captures stay untracked.

## Non-Goals
- Do not rewrite git history; 580019e8 is explicitly git rm --cached only.
- Do not reintroduce legacy runtime/ fallback reads or migrate stale legacy trees.
- Do not commit .github/mcp.json or session-capture directories belonging to other agents.
- Do not create standalone ADR files; record decisions as spec requirements.
- Do not expand into the remaining Batch 1 chain (fd7737ec, 938a7ae9, 185a00a2, 9cd9440e) in the same unit.

## Context Anchors
- Session store layout is now canonical and pinned in spec 7b277ba4-7deb-4f24-a3c4-88ea0d8e0a4f as R11 (on-disk layout) and R12 (git-tracking policy). Read those two requirements before touching layout or .gitignore.
- Durable session artifacts live at .session/sessions/<workspace_session_id>/ containing context.json, handoffs/<handoff_id>/{handoff.json,handoff.md}, finish.json, runs/<run_id>/. Machine-local state lives at .session/local/ via local_root() in memory-api/crates/session-api/src/store/config/persistence.rs.
- .session/runtime/ was deleted after byte-and-hash verification of all 65 files. Zero writers remain. Legacy fallback shims legacy_runtime_paths_for_workspace and legacy_active_workspace_session_path still exist but are slated for removal by ticket 7fabc77a.
- Back-compat for the legacy runtime/workspaces/ tree was DROPPED by explicit user decision during the 2026-07-28 iteration review. 7a4f9c3d AC3, 0a45bedb AC5, and spec R11 were all amended to remove the fail-open obligation. Do not reintroduce it.
- The events.json git-tracking decision is authoritative in spec R12 only. Working notes at memory-api/.ticket/tickets/4817a5cc-5e91-4280-b7ed-aed296a480b3/decision-events-tracking.md are non-authoritative. memory-api/docs/decisions/ was rejected as an ADR location and deleted.
- Committed this iteration: memory-api 5f1dfa8, 9972b62, 069ed01; superproject 09e89d94. Both trees clean apart from deliberately excluded local drift.
- .github/mcp.json carries an unrelated local fs-mcp entry, deliberately left uncommitted. Do not sweep it into a commit.
- Session-capture directories under .session/sessions/ from other agents are deliberately left untracked. Only this track's own session dir was committed.
- Validation baseline at handoff time: cargo build --workspace clean (6 pre-existing unrelated warnings); 333/333 tests across session-api, session-mcp, session-cli, ticket-api.

## Risk Notes
Removing the legacy fallback is fail-closed: any worktree or machine still holding a .session/runtime/ tree will lose access to those sessions. This was accepted by the user. 580019e8 touches ~99MB across 76 files with git rm --cached; verify .gitignore rules from spec R12 are in effect BEFORE untracking, or the files will simply be re-added on the next commit.

## Workflow
- **Nodes**: 0
- **Edges**: 0
- **Not Done**: 0
