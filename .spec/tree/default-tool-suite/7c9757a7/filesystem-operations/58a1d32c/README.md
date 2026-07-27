<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=58a1d32c-2643-455c-bf3b-e0ccf0eecd9f slug=agent-tooling/filesystem-operations digest=eaf15a0f25cc -->

# filesystem operations: bounded listing, stat, and conflict-aware mutation

- slug: `agent-tooling/filesystem-operations`
- component: agent-tooling
- scope: internal
- state: agent-tooling
- index_ref: `.spec/specs/58a1d32c-2643-455c-bf3b-e0ccf0eecd9f/spec.toml`

## Summary

Specify a token-bounded filesystem surface -- list, stat, move, rename, copy,

## Acceptance Criteria Excerpt

1. A `*-api` crate owns listing, stat, and mutation behavior with transport-independent request and response types and one error model. 2. Listing is bounded by default with explicit depth and entry caps, and a truncated result is flagged as truncated with the total count where …

## Navigation

- Parent: [agent-tooling/default-tool-suite](../../README.md)
- Siblings: [agent-tooling/compact-terminal](../../compact-terminal/63c60c9d/README.md), [agent-tooling/file-editing](../../file-editing/4f5ad264/README.md), [agent-tooling/repo-wide-search](../../repo-wide-search/af9ebba9/README.md)
- Children: _(none)_
