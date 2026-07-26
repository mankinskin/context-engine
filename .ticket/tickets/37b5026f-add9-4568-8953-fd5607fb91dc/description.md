Tracking ticket for migrating .agents/instructions/ into nested workflow folders.

Linked spec: [57d56ae8 Instruction files nested workflow migration](.spec/specs/57d56ae8-792a-4f95-a531-4fe59e0c824f/spec.toml)

Scope:
- Move instruction files from the flat monolith into approved nested folders under `.agents/instructions/`.
- Keep pilot files under `.agents/instructions/ticket/` untouched.

Goals/Decisions captured in linked spec:
- Hybrid taxonomy and approved folder names: ticket/spec/commit/engine/frontend/audit/testing/transcripts/session/orchestration
- Split only large multi-concern files; prefer single-concern files.
- Every instruction file must start with a precise `Use when...` description and remove `applyTo` entries.
- Session bootstrap and orchestration delegation instructions are applied at session start and throughout sessions.
- Deduplicate AGENTS.md content by replacing copies with links to canonical instructions.
- Prune stale `.rule/` records + generated targets for removed files; keep nested files hand-owned (outside generation).

## Implementation Complete

Migration completed 2026-07-26:
- 35 nested `*.instructions.md` files across approved folders
- Former monoliths split coherently (commit 5, testing 6, orchestration 10)
- All descriptions begin `Use when...` (session bootstrap and orchestrator delegation use session-wide variant)
- No `applyTo` in any instruction file
- All relative Markdown links resolve
- 252 stale rule entries removed
- 11 stale generated-target entities removed
- Two stale session pins removed

### Validation Evidence

Clean rule scan (2026-07-26):
```bash
rtk cargo run --manifest-path memory-api/tools/cli/rule-cli/Cargo.toml -- scan --workspace c:/Users/linus/git/graph_app/context-engine --toon
```
Result: status ok, integrated 0, pruned 0, diagnostics_count 0, scan_root_count 9

Harness verification:
- All 35 nested instruction files discovered across 10 approved folders
- Session render succeeds with no missing/stale pins
- Secondary generated references updated: `.clinerules/30-path-scoped.md`, relevant spec link, `repo_map.toon`

### Acceptance Criteria Status

✅ No multi-workflow monoliths remain
✅ All non-trivial instructions live in approved subfolders with precise description gating
✅ AGENTS.md duplication removed (replaced by links)
✅ Rule store scan clean; removed generated targets pruned

Reference commit 42427877 contains validation state (created during action 7; recorded transparently).

Ready for review.
