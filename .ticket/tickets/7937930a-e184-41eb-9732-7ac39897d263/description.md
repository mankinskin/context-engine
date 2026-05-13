# Problem

The current `crane-cli` mapping model requires a non-empty destination path. That is sufficient for direct source-to-destination transplants, but it does not yet support collapsing a filtered subtree directly to branch root for review or import flows that need that shape.

# Scope

Extend `crane-cli` so selected source trees can be rewritten into branch root when that migration shape is required.

The work should cover:

- the CLI mapping syntax or flag model for branch-root rewrites
- path-transform behavior for files, renames, copies, and deletes when the destination root is empty
- error handling for ambiguous or overlapping mappings
- focused tests for the branch-root rewrite path
- confirmation that the feature keeps the fast-export -> transform -> fast-import streaming model intact

# Acceptance Criteria

- `crane-cli` can express a branch-root rewrite without ad hoc post-processing.
- The transform layer handles branch-root file operations correctly.
- Focused tests cover at least one successful branch-root import path.
- Existing non-root mapping behavior remains unchanged.
- The CLI help and documentation explain when branch-root rewrite mode should be used.
