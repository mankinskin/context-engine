Phase B. Extract the doc tool into its own `doc` repository (owner mankinskin), bundling: doc-api, doc transports (cli/mcp/http as present), and doc-viewer.

Follow the common per-tool extraction recipe (see parent tracker). Migrate doc-scoped artifacts via the cross-store move tooling.

## Acceptance criteria
- `doc` repo builds/tests independently; transports smoke pass; doc-viewer browser-verified (screenshot + Playwright).
- doc-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.