Phase B. Extract the ticket tool into its own `ticket` repository (owner mankinskin), bundling all ticket components: ticket-api, ticket-cli, ticket-mcp, ticket-http, ticket-viewer, and ticket-vscode(+ticket-vscode-core).

Follow the common per-tool extraction recipe (see parent tracker): assemble crates+transports+viewer+vscode, declare memory-kernel/viewer-api/memory-fixtures as deps, preserve history, migrate ticket-scoped artifacts via the cross-store move tooling, verify build/test/transports/viewer, register in workflow-tools.

## Acceptance criteria
- `ticket` repo builds/tests independently; cli/mcp/http smoke pass; ticket-viewer browser-verified (screenshot + Playwright).
- ticket-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.