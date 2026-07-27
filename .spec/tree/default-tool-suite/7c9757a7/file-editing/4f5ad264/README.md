<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=4f5ad264-8e8d-4681-9551-4ec14b73c3b1 slug=agent-tooling/file-editing digest=2e6f1532ced5 -->

# file editing: context-anchored differential patching surface

- slug: `agent-tooling/file-editing`
- component: agent-tooling
- scope: internal
- state: agent-tooling
- index_ref: `.spec/specs/4f5ad264-8e8d-4681-9551-4ec14b73c3b1/spec.toml`

## Summary

Specify a token-bounded, context-anchored file editing surface so agents --

## Acceptance Criteria Excerpt

1. A `*-api` crate owns the anchoring, matching, and patch-application behavior with transport-independent request and response types and one error model. 2. Replacement is anchored by surrounding context text, not by line number, so an edit stays valid when unrelated parts of t…

## Navigation

- Parent: [agent-tooling/default-tool-suite](../../README.md)
- Siblings: [agent-tooling/compact-terminal](../../compact-terminal/63c60c9d/README.md), [agent-tooling/filesystem-operations](../../filesystem-operations/58a1d32c/README.md), [agent-tooling/repo-wide-search](../../repo-wide-search/af9ebba9/README.md)
- Children: _(none)_
