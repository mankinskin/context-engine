# Handoff: e35e7d76-efc7-4fef-830b-df32f362c17e

## Summary
- **Workspace Session**: `fe3ca43f-9d1c-4334-b2bf-2324936121f9`
- **Outgoing Run**: `096955dd-b238-42d8-93e3-5f5cbf27dab4`
- **Created**: 2026-07-28T08:09:11.373625700+00:00
- **Objective**: Remove the legacy runtime-path fallback shims and add a guard that fails loudly on any write under .session/runtime/ (ticket 7fabc77a), then untrack the 76 committed events.json files (ticket 580019e8), unblocking the Batch 1 chain at ab02e15a.
- **Implementation Ready**: true

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
- Legacy runtime-path back-compat is removed, not retained as a permanent shim. Removal is fail-closed: stale legacy trees on other worktrees or machines are abandoned, not migrated.
- Decision records live in the spec as numbered requirements, not in standalone ADR files. memory-api/docs/decisions/ was proposed, rejected, and deleted.
- 4817a5cc AC1 was closed on git check-ignore evidence; the commits landed this iteration serve as the end-to-end proof that durable artifacts are actually tracked.
- Untracking the 76 committed events.json files (~99MB) is scoped to 580019e8 alone and is git rm --cached only, never a history rewrite.
- Only the active track's own session directory is committed; other agents' session captures stay untracked.
- Sub-agent reports are not trusted without verification. Two sub-agents in the prior iteration reported completed work that had not happened (a fabricated ticket written straight to disk under a made-up UUID, and a file deletion that never occurred). Verify every claimed store mutation against the store and every claimed deletion against the filesystem.

## Non-Goals
- Do not rewrite git history. 580019e8 is git rm --cached only.
- Do not reintroduce legacy runtime/ fallback reads and do not migrate stale legacy trees found on other machines.
- Do not commit .github/mcp.json, any events.json, or session-capture directories belonging to other agents.
- Do not create standalone ADR or decision files; record decisions as numbered spec requirements.
- Do not start the remaining Batch 1 chain (ab02e15a, fd7737ec, 938a7ae9, 185a00a2, 9cd9440e) in this unit; it is the unit after.
- Do not reopen 7a4f9c3d, 41ed4585, 4817a5cc, or 0a45bedb. Their criteria were reviewed, amended by user decision, and closed.

## Context Anchors
- Canonical layout is pinned in spec 7b277ba4-7deb-4f24-a3c4-88ea0d8e0a4f: R11 = on-disk layout, R12 = git-tracking policy. Read both before touching layout, paths, or .gitignore. R12 is the ONLY authoritative record of the events.json decision.
- Durable artifacts: .session/sessions/<workspace_session_id>/{context.json, handoffs/<handoff_id>/{handoff.json,handoff.md}, finish.json, runs/<run_id>/}. Machine-local state: .session/local/, resolved via local_root() in memory-api/crates/session-api/src/store/config/persistence.rs.
- Fallback shims to delete: legacy_runtime_paths_for_workspace and legacy_active_workspace_session_path in persistence.rs (~L173-190), plus the NotFound fallback branch in read_runtime_context in memory-api/crates/session-api/src/store/config/worktree_runtime.rs (~L286-303). These are the only remaining references to the legacy tree.
- Back-compat was DROPPED by explicit user decision on 2026-07-28. 7a4f9c3d AC3, 0a45bedb AC5, and spec R11 were all amended to remove the fail-open obligation. Do not reintroduce it and do not treat its absence as a bug.
- STALE-BINARY INCIDENT (resolved): on 2026-07-28 a ~/.cargo/bin/session-mcp.exe predating the flatten commit wrote a handoff to the legacy .session/runtime/ path, recreating a deleted tree. Root cause was the installed binary, not the source; a fresh build against a scratch store produced the correct sessions/<id>/handoffs/<id>/ layout. The binary has since been reinstalled and the server restarted. Residual .session/runtime/ files from that incident may still be on disk and are ticket 7fabc77a's to sweep.
- Lesson from that incident, worth encoding in code: nothing failed loudly when a legacy path was written. Add an assertion or test that panics/fails on any write under .session/runtime/ so install-staleness drift is detected instead of silently tolerated.
- Committed at handoff time: memory-api 5f1dfa8, 9972b62, 069ed01, 71ef212; superproject 09e89d94, 61905716, f8fee8d2, 80080c39. Tickets 7a4f9c3d, 41ed4585, 4817a5cc, 0a45bedb are all done.
- Deliberately left dirty, do not sweep into a commit: .github/mcp.json (unrelated local fs-mcp entry) and every .session/sessions/<id>/ directory belonging to another agent. Only the active track's own session dir gets committed.
- Sequencing constraint: 7fabc77a depends_on 7a4f9c3d (done, so it is unblocked now). 580019e8 depends_on 4817a5cc (done, also unblocked) but its gitignore rules must be verified in effect BEFORE untracking, or the files return on the next commit.
- Baseline to beat: cargo build --workspace clean with 6 pre-existing unrelated warnings; 333/333 tests across session-api, session-mcp, session-cli, ticket-api.

## Risk Notes
Fallback removal is fail-closed by design: any worktree or machine still holding a .session/runtime/ tree loses access to those sessions. Accepted by the user; do not soften it. For 580019e8, run the gitignore-policy gate FIRST — untracking ~99MB across 76 files without the R12 rules in effect simply re-adds them on the next commit. The stale-binary incident showed that installed binaries under ~/.cargo/bin can silently diverge from the worktree; after changing session-api path logic, reinstall and restart session-mcp before trusting any MCP-observed on-disk behavior, and verify against a fresh build rather than the running server.

## Workflow
- **Nodes**: 0
- **Edges**: 0
- **Not Done**: 0

## Validation
- `vt-events-json-untracked`: - (required)
- `vt-session-core-suites`: - (required)
- `vt-session-gitignore-policy`: - (required)
- `vt-session-no-legacy-runtime-path`: - (required)
