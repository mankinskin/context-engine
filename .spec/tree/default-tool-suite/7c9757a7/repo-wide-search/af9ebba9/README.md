<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=af9ebba9-6de4-4290-ab4a-319c432ded4c slug=agent-tooling/repo-wide-search digest=f044522d71e8 -->

# repo-wide search: bounded, capped, counts-first repository search

- slug: `agent-tooling/repo-wide-search`
- component: agent-tooling
- scope: internal
- state: agent-tooling
- index_ref: `.spec/specs/af9ebba9-6de4-4290-ab4a-319c432ded4c/spec.toml`

## Summary

Specify a repo-root-scoped, token-bounded search surface so agents locate code

## Acceptance Criteria Excerpt

1. A `*-api` crate owns traversal, matching, capping, and truncation reporting with transport-independent request and response types and one error model. 2. Search is scoped to the repository root by default and accepts a narrower subtree scope. 3. A counts-only mode returns per…

## Navigation

- Parent: [agent-tooling/default-tool-suite](../../README.md)
- Siblings: [agent-tooling/compact-terminal](../../compact-terminal/63c60c9d/README.md), [agent-tooling/file-editing](../../file-editing/4f5ad264/README.md), [agent-tooling/filesystem-operations](../../filesystem-operations/58a1d32c/README.md)
- Children: _(none)_
