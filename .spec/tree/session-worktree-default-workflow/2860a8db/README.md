<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=2860a8db-0c4e-4e94-984a-c10a72a67ffc slug=context-engine/session-worktree-default-workflow digest=db862d94c94d -->

# default worktree-backed session workflow

- slug: `context-engine/session-worktree-default-workflow`
- component: context-engine
- scope: internal
- state: context-engine
- index_ref: `.spec/specs/2860a8db-0c4e-4e94-984a-c10a72a67ffc/spec.toml`

## Summary

Make dedicated git worktrees the default workflow for new agent sessions in this repository so parallel implementation tracks do not share one staging area.

## Acceptance Criteria Excerpt

1. This spec defines the mandatory startup order: session check-in, authoritative worktree assignment, board check-in or file claims, implementation, then stop or handoff capture. 2. This spec defines the ownership boundary between `session-api`, the ticket draftboard, and repos…

## Navigation

- Parent: _(root)_
- Children: _(none)_
