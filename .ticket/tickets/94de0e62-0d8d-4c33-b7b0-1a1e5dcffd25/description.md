Phase B. Extract the session tool into its own `session` repository (owner mankinskin), bundling: session-api, session-cli, session-mcp.

Follow the common per-tool extraction recipe (see parent tracker). Migrate session-scoped artifacts via the cross-store move tooling. Coordinate with session identity/optimization tickets still in review.

## Acceptance criteria
- `session` repo builds/tests independently; transports smoke pass.
- session-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.