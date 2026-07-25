Phase B. Extract the audit tool into its own `audit` repository (owner mankinskin), bundling: audit-api, audit-cli, audit-mcp, plus the .audit.toml config contract.

Follow the common per-tool extraction recipe (see parent tracker). Migrate audit-scoped artifacts via the cross-store move tooling.

## Acceptance criteria
- `audit` repo builds/tests independently; transports smoke pass.
- audit-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.