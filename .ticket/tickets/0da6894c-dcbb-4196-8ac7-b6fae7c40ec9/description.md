Phase A design/contract. Define the canonical per-domain crate layout that every tool extraction must follow: a single domain crate (named after the domain, e.g. `ticket`) that unifies the domain API and all transports into one build target, with each transport exposed as a binary target of that crate.

## Decision (locked 2026-07-25, refined after review)
- Collapse the previous multi-crate transport split into ONE domain crate `{domain}`.
- The domain crate's library is the primary build target and the public domain handle.
- `{domain}-api` remains its own internal crate; the domain crate depends on it and re-exports its public surface. The domain crate lib is the internal API re-export plus transport-agnostic wiring.
- Each transport (CLI, MCP, HTTP, future) is a binary build target (`[[bin]]`) of the domain crate, sharing the crate lib. Transport-specific code lives in `src/bin/*` (or gated modules), not separate transport crates.
- Transport bins use the shared `transport-harness` crate (`dbe0e955`) so CLI/MCP/HTTP boilerplate is not duplicated across the 11 domain crates.
- Transport binaries are feature-gated. Consumers enable the features they need, such as `--features cli,mcp`; a slim consumer can build the lib only.
- Binary names preserve the current interface tool names (`{domain}-cli`, `{domain}-mcp`, `{domain}-http`).
- Frontends stay separate: the domain viewer (Dioxus/WASM) and any VS Code extension remain their own crates/packages, depending on the domain crate lib.

## Delivered Contract
- Specification: `workflow-tools/domain-crate-contract` (`5ee7f36a-2aea-4373-8c67-e6b26ae174bf`).
- Human-readable contract: `WORKFLOW_TOOLS_DOMAIN_CRATE_CONTRACT.md`.
- Compiling reference workspace: `workflow-tools-contract-reference`, with `example-api`, public `example`, and `transport-harness` crates.
- The reference manifest proves the `[lib]`, API re-export, empty default feature set, and feature-gated CLI/MCP/HTTP `[[bin]]` pattern.
- Per-tool tickets already reference this contract and the shared harness ticket.

## Validation
- Validation spec: `workflow-tools-domain-crate-contract`.
- Execution: `workflow-tools-domain-crate-contract-20260725` passed library-only build, all-feature build, workspace tests, formatting, and direct CLI/MCP/HTTP binary probes.

## Workflow State
- Implementation is complete and validated. Transition to `in-review` is blocked by ticket MCP store error `no schema for type 'task'` when calling `update_ticket` with `to_state=in-review`; the ticket remains `new` until the ticket transition schema is repaired.

## Dependencies
- Pairs with the shared `transport-harness` crate (`dbe0e955`).
- Gates every per-tool extraction ticket (they must follow this layout).
- Should be finalized before per-tool work begins.