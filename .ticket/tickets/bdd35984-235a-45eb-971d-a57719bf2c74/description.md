Phase B. Extract the feedback tool into its own `feedback` repository (owner mankinskin), bundling: feedback-api, feedback-cli, feedback-mcp.

Follow the common per-tool extraction recipe (see parent tracker). Migrate feedback-scoped artifacts via the cross-store move tooling. Coordinate with the feedback-api design/curation tickets still in review.

## Acceptance criteria
- `feedback` repo builds/tests independently; transports smoke pass.
- feedback-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.