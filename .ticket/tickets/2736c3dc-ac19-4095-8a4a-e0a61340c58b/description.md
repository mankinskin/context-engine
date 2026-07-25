Phase B. Extract the log tool into its own `log` repository (owner mankinskin), bundling: log-api, log transports (cli/mcp as present), and log-viewer.

Follow the common per-tool extraction recipe (see parent tracker). Migrate log-scoped artifacts via the cross-store move tooling. Coordinate with in-flight log-viewer Dioxus port tickets.

## Acceptance criteria
- `log` repo builds/tests independently; transports smoke pass; log-viewer browser-verified (screenshot + Playwright).
- log-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.