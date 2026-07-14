<!-- aligned-structure:v2 -->

# Motivation

The doc/log viewer applications are owned with the other viewer applications in `memory-viewers`, while the shared peek API and its CLI/MCP transports belong with the API and transport crates in `memory-api`. Their physical source locations now reflect those ownership boundaries without changing package identities or behavior.

Execution is tracked by [6ded2540 Relocate viewer and peek crates into owning repositories](../../../.ticket/tickets/6ded2540-206b-4ffd-bdb6-23459a16ab1d/ticket.toml).

# Dependent expectation

Dependents can rely on the Cargo packages `doc-viewer`, `log-viewer`, `doc-viewer-dioxus`, `log-viewer-dioxus`, `peek-api`, `peek-cli`, `peek-mcp`, and `compact-terminal-mcp` retaining their names and behavior while resolving from their canonical `memory-viewers` or `memory-api` source repositories. The context-engine root remains the aggregate Cargo workspace and managed-viewer coordination root.

# Guards

- `val-workspace-relocated-crates`: Cargo metadata plus focused checks/tests resolve and execute all moved packages from canonical paths.
- `val-workspace-relocated-viewers-browser`: managed doc-viewer and log-viewer prepare/start and Chromium browser checks pass from canonical paths.
- `val-workspace-relocated-active-refs`: bounded search confirms active build, configuration, validation, documentation, and current specification references no longer target removed source paths; historical ticket/revision evidence is excluded.

# Positions

- `deprecated`: source positions under `tools/viewer`, `tools/peek-api`, `tools/cli/peek-cli`, and `tools/mcp/{peek-mcp,compact-terminal-mcp}` have been removed with no compatibility shims.
- `implemented`: `memory-viewers/doc-viewer/` and `memory-viewers/log-viewer/` are the canonical viewer positions.
- `implemented`: `memory-api/crates/peek-api/`, `memory-api/tools/cli/peek-cli/`, `memory-api/tools/mcp/peek-mcp/`, and `memory-api/tools/mcp/compact-terminal-mcp/` are the canonical peek positions.
- `implemented`: `viewer-ctl.toml`, VS Code integration, install validation, hooks, workflows, current docs/rules/specs, and package path dependencies resolve the canonical locations.

# Validation evidence

- `exec-workspace-relocated-crates-20260714`: Cargo metadata succeeded; the final six-package native regression passed 96 tests across 12 suites; both Dioxus frontends compiled for WASM; both TypeScript frontends built; log Vitest passed 18 tests.
- `exec-workspace-relocated-viewers-browser-20260714`: fresh release binaries deployed through `viewer-ctl`, resolved repository paths correctly, and rendered nonblank in external Chromium at 1280x800 and 390x844 with screenshots under `target/tmp/viewer-relocation/`.
- `exec-workspace-relocated-active-refs-20260714`: exact active-reference scan found no operational references to removed source paths; immutable historical records remain unchanged.

# Governing-rule requirement

This contract was introduced by `ce://default/rule/84fa9769-cff9-4d89-9068-88474584b4b3` (ticket/spec routing) and governed during implementation by `ce://default/rule/397b0447-135e-4d35-ad05-bcc69047d2c0` (viewer quality gates) and `ce://default/rule/dada11d6-d36e-464f-92cf-f0a50e3d7aec` (file ownership and escalation).

# Acceptance criteria

- The complete six source trees exist only at their canonical destinations; no compatibility shims remain at old paths.
- Cargo package and binary names, ports, MCP names, and runtime behavior are unchanged.
- Root workspace members and every active relative path dependency resolve across submodule boundaries.
- Active operational references are updated, while immutable historical ticket descriptions and revision logs remain unchanged.
- Focused native, WASM/frontend, managed-viewer, and browser validations provide linked evidence.

# Non-goals

- Implementation refactors or user-facing changes.
- Package publication or version changes.
- Rewriting historical evidence solely for path cosmetics.
