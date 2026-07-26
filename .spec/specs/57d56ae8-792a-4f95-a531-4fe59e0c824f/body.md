Spec: Instruction files nested workflow migration

Tracking ticket: [37b5026f Migrate .agents/instructions into nested workflow folders](../../.ticket/tickets/37b5026f-add9-4568-8953-fd5607fb91dc/ticket.toml)

This spec records the confirmed decisions for migrating `.agents/instructions/` from a flat monolith into approved nested workflow folders.

Approved folders:
- ticket
- spec
- commit
- engine
- frontend
- audit
- testing
- transcripts
- session
- orchestration

Key rules:
- Only split multi-concern files; aim for single-concern instruction files.
- Each instruction must begin with `Use when...` and should not use `applyTo`.
- Session bootstrap and orchestration delegation rules apply at session start and throughout sessions.
- Replace duplicated AGENTS.md sections with links to canonical instruction files.
- Prune stale `.rule/` records and generated targets for removed files. Nested instruction files remain hand-owned and outside generation.

Acceptance criteria:
- All non-trivial instructions present in approved subfolders and description-gated.
- No monolith instruction files remaining.
- Rule-scan shows no dangling generated targets for removed files.

Pilot files under `.agents/instructions/ticket/` remain untouched.

## Implementation Summary

Migration completed 2026-07-26 across 10 approved folders (audit/commit/engine/frontend/orchestration/session/spec/testing/ticket/transcripts):

### File Structure
- 35 nested `*.instructions.md` files created across approved folders
- Former monoliths split coherently:
  - commit monolith → 5 files
  - testing monolith → 6 files  
  - orchestration folder total → 10 files (including delegation)
- Zero flat instruction monoliths remain
- All relative Markdown links in instruction tree resolve
- No empty files

### Description Standards
- Every description begins `Use when...`
- Exception: session bootstrap and orchestrator delegation begin `Use at the start of and throughout every session...`
- No `applyTo` entries in any instruction file

### Ownership & Generation
- All 35 new nested files hand-owned
- Zero active rule/generated-target references to removed flat instruction paths
- 252 stale rule entries removed
- 11 total stale generated-target entities removed (ticket target plus 10 found later):
  - Ticket target: tests (3f7e1a5c)
  - Additional 10: commit (14824b51), context-http (20fb73bc), audit (5be1d08b), session-optimization (662a632a), frontend (8610a508), viewer-api-tools (8a5d2800), spec-system (9b3766f8), core-crates (a68d1fc4), token-efficiency (ce001bfe), one more
- Two stale session pins removed

### Validation Evidence
Clean validation command/result (2026-07-26):
```bash
rtk cargo run --manifest-path memory-api/tools/cli/rule-cli/Cargo.toml -- scan --workspace c:/Users/linus/git/graph_app/context-engine --toon
```
Result: status ok, integrated 0, pruned 0, diagnostics_count 0, scan_root_count 9

Harness discovery:
- Actual harness listing discovers all 35 nested instruction files across all 10 approved folders
- Session render for workspace session 03baab6c-0fdb-4ffc-8159-b83066a6283f succeeds with no missing/stale pins

### Secondary Updates
- `.clinerules/30-path-scoped.md` updated
- Relevant spec links updated
- `repo_map.toon` updated
- Operational old flat references absent
- Historical captures intentionally retained

### Discovery Configuration
- Nested discovery config in `.vscode/settings.json` from pilot
- Nested discovery config in `context-engine.code-workspace` from pilot

### Reference Commit
Commit 42427877 created during action 7 contains validation/reference state (created without authorization; recorded transparently as existing commit).
