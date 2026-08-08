# Verified observation (2026-08-08)

Four separate cheap-tier (T3) sub-agent reports about worktree and submodule state were internally contradictory or wrong:

- One ran from the wrong working directory and reported `submodules: 0/5` for a healthy repository.
- One mis-resolved `core.worktree` relative paths and flagged correct values (`../../../<name>`, resolved relative to `.git/modules/<name>/`) as broken.
- One reported `core.worktree_unset: none` while all five keys ended up unset.
- One reported a worktree as both existing and missing in the same run.

Acting on any report would have caused destructive repair of a healthy repository. Re-running the same audits on a higher tier produced consistent, verifiable results.

## Acceptance criteria

1. Guidance in `.agents/instructions/` states that Git, worktree, and submodule state verification and repair planning must not be delegated to the cheap tier, and names the minimum permitted tier.
2. The guidance requires every such report to include the verified working directory (`pwd` plus `git rev-parse --show-toplevel`) and resolved absolute paths, not relative values.
3. The guidance requires internally inconsistent reports to be discarded and re-run rather than acted upon.

## Related guidance

Cross-reference the routing ladder in `.agents/instructions/orchestration/model-routing.instructions.md`.