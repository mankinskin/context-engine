Phase B. Extract the spec tool into its own `spec` repository (owner mankinskin), bundling: spec-api, spec-cli, spec-mcp, spec-http (if present), and spec-viewer.

Follow the common per-tool extraction recipe (see parent tracker). Migrate spec-scoped artifacts via the cross-store move tooling.

## Acceptance criteria
- `spec` repo builds/tests independently; transports smoke pass; spec-viewer browser-verified (screenshot + Playwright).
- spec-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.