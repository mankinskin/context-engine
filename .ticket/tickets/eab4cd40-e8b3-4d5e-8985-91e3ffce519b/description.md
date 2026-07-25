Phase B. Extract the test tool into its own `test` repository (owner mankinskin), bundling: test-api, test-cli, test-mcp, and any test-http transport.

Follow the common per-tool extraction recipe (see parent tracker). Migrate test-scoped artifacts via the cross-store move tooling.

## Acceptance criteria
- `test` repo builds/tests independently; transports smoke pass.
- test-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.