Phase C. Establish repo-level self-referential artifact stores in `workflow-tools` (and confirm per-tool artifact stores in each tool repo) so work on the tools themselves is tracked with the tools themselves — e.g. tickets about improving the ticket system live in the ticket repo's own .ticket store; cross-tool work lives in the workflow-tools repo-level stores.

## Scope
- Initialize repo-level artifact stores (.ticket/.spec/.rule/.doc/.test/.log/.feedback) in workflow-tools for cross-tool concerns.
- Confirm each per-tool repo carries its own tool-scoped stores.
- Exploit cross-store/cross-workspace URN references so tasks can be localized per tool while still linking upward.

## Acceptance criteria
- workflow-tools has functioning repo-level artifact stores discoverable by the tools.
- Each tool repo has its own initialized stores.
- Cross-store references resolve across the nested hierarchy (validated with a sample reference).

## Dependencies
- Blocked by umbrella creation and per-tool extraction.
- Uses cross-store reference model (coordinates with default-store `671d4e47`).