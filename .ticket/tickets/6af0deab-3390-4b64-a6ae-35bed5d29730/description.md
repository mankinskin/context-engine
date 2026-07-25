Phase B. Extract the peek tool into its own `peek` repository (owner mankinskin), built as a single `peek` domain crate per contract `0da6894c`: the crate lib re-exports the internal `peek-api` crate and exposes transports as FEATURE-GATED binary targets (`peek-cli`, `peek-mcp`, and `compact-terminal-mcp`) built on the shared `transport-harness` (`dbe0e955`).

Follow the parent-tracker recipe (`858c5286`); migrate peek-scoped artifacts via the cross-store move tooling.

## Acceptance criteria
- `peek` builds independently: domain crate lib (primary) re-exporting internal `peek-api` + feature-gated transport bins (names preserved incl. compact-terminal-mcp) over the harness.
- transport bin smoke pass.
- peek-scoped artifacts migrated with reference integrity.
- Registered as a workflow-tools dependency.