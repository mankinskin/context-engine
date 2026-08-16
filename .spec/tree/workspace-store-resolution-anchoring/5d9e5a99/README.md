<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=5d9e5a99-74b5-4f4e-9e6f-0a6cc3741ddf slug=memory-api/workspace-store-resolution-anchoring digest=6a3f33ae520c -->

# Workspace store resolution anchoring

- slug: `memory-api/workspace-store-resolution-anchoring`
- component: memory-api
- scope: component
- state: draft
- index_ref: `.spec/specs/5d9e5a99-74b5-4f4e-9e6f-0a6cc3741ddf/spec.toml`

## Summary

`ticket-api` and `session-api` resolve a store from one authoritative checkout

## Acceptance Criteria Excerpt

1. A root-invoked ticket or session write resolves to the repository-root store even when at least fifteen `.worktrees/*` directories each contain a nested store. 2. From a repository-root invocation, no nested `.worktrees/*/.ticket` or `.worktrees/*/.session` path is considered…

## Navigation

- Parent: _(root)_
- Children: _(none)_
