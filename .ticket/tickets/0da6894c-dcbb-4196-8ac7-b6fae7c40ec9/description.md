Phase A design/contract. Define the canonical per-domain crate layout that every tool extraction must follow: a single domain crate (named after the domain, e.g. `ticket`) that unifies the domain api and all of its transports into one build target, with each transport exposed as a binary target of that crate.

## Decision (locked 2026-07-25, refined after review)
- Collapse the previous multi-crate transport split into ONE domain crate `{domain}`.
- The domain crate's library is the primary build target and the public domain handle.
- **api relationship (reviewed):** `{domain}-api` is KEPT as its own internal crate; the domain crate depends on it and re-exports its public surface. The domain crate lib = internal-api-crate re-export + transport-agnostic wiring. (Not absorbed into modules.)
- Each transport (cli, mcp, http, future) is a binary build target (`[[bin]]`) of the domain crate, sharing the crate lib. Transport-specific code lives in `src/bin/*` (or gated modules), not separate transport crates.
- **transport scaffolding (reviewed):** transport bins are built on the shared `transport-harness` crate (`dbe0e955`) so cli/mcp/http boilerplate is not duplicated across the 11 domain crates.
- **bin gating (reviewed):** transport binaries are FEATURE-GATED. Consumers enable the features for the transports they need (e.g. `--features cli,mcp`); a slim consumer can build the lib only.
- Binary names preserve the current interface tool names (`{domain}-cli`, `{domain}-mcp`, `{domain}-http`).
- Frontends stay separate: the domain viewer (Dioxus/WASM) and any vscode extension remain their own crates/packages, depending on the domain crate lib.

## Rationale
- One primary build target per domain simplifies dependency declaration for `workflow-tools` and target projects.
- Internal api crate keeps a clean api boundary and independent testability while the domain crate is the single consumable handle.
- Shared transport-harness + feature-gated bins keep transports DRY and let consumers build only what they invoke.

## Scope
- Author the crate-layout contract (manifest shape: `[lib]` + feature-gated `[[bin]]` targets, feature matrix, module layout) and a reference skeleton.
- Define the api re-export rule (domain crate re-exports the internal `{domain}-api` crate) and transport-binary naming rule.
- Define how bins consume the `transport-harness`.
- Define the workspace/dependency declaration `workflow-tools` and context-engine use to consume a domain crate (lib + selected transport bins).
- Update the per-tool extraction recipe and every per-tool ticket to target this structure.

## Acceptance criteria
- Committed crate-layout contract + reference skeleton compiling with lib + feature-gated transport bins over the transport-harness.
- Internal `{domain}-api` crate re-exported from the domain crate lib; binary names preserved.
- Per-tool parent tracker and all 11 per-tool tickets updated to reference this contract.
- workflow-tools dependency-declaration pattern documented against the single domain crate with feature-selected bins.

## Dependencies
- Pairs with the shared `transport-harness` crate (`dbe0e955`).
- Gates every per-tool extraction ticket (they must follow this layout).
- Should be finalized before per-tool work begins.