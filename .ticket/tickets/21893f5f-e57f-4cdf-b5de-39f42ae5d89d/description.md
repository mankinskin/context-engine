Phase B. Extract the rule tool into its own `rule` repository (owner mankinskin), bundling: rule-api, rule-cli, rule-mcp, and any rule-http transport.

Follow the common per-tool extraction recipe (see parent tracker). Migrate rule-scoped artifacts via the cross-store move tooling. Note rule-targets generation coupling — coordinate with the migrate-off-generator work.

## Acceptance criteria
- `rule` repo builds/tests independently; transports smoke pass.
- rule-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.