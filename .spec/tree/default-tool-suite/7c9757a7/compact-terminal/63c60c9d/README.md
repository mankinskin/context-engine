<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=63c60c9d-adbe-4ddb-8c1d-6156610d0753 slug=agent-tooling/compact-terminal digest=66d0840b98b0 -->

# compact-terminal: bounded command execution with spill-and-peek

- slug: `agent-tooling/compact-terminal`
- component: agent-tooling
- scope: internal
- state: agent-tooling
- index_ref: `.spec/specs/63c60c9d-adbe-4ddb-8c1d-6156610d0753/spec.toml`

## Summary

Specify the token-bounded command-execution surface of the default agent tool

## Acceptance Criteria Excerpt

1. A `compact-terminal-api` crate owns the execution, inline/spill decision, and spill-reading behavior, with transport-independent request and response types. 2. `compact-terminal-mcp` delegates to that crate and keeps its current `run` and `read_spill` tool names and response …

## Navigation

- Parent: [agent-tooling/default-tool-suite](../../README.md)
- Siblings: [agent-tooling/file-editing](../../file-editing/4f5ad264/README.md), [agent-tooling/filesystem-operations](../../filesystem-operations/58a1d32c/README.md), [agent-tooling/repo-wide-search](../../repo-wide-search/af9ebba9/README.md)
- Children: _(none)_
