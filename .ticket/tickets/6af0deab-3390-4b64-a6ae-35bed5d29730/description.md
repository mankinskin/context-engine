Phase B. Extract the peek tool into its own `peek` repository (owner mankinskin), bundling: peek-api, peek-cli, peek-mcp, and the related compact-terminal-mcp.

Follow the common per-tool extraction recipe (see parent tracker). Migrate peek-scoped artifacts via the cross-store move tooling.

## Acceptance criteria
- `peek` repo builds/tests independently; transports smoke pass.
- peek-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.