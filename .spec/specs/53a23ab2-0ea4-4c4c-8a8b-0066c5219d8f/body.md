<!-- aligned-structure:v2 -->

## Motivation

Workflow domains expose CLI, MCP, and HTTP binary targets. A shared harness prevents those 11 domains from each reimplementing argument parsing, server startup, HTTP error mapping, and output mechanics.

## Dependent expectation

If this spec is implemented, a domain crate can select CLI, MCP, and HTTP features and use `transport-harness` for common startup and dispatch mechanics while retaining only domain-specific wiring.

## Guards

- A standalone harness test suite verifies feature-gated CLI/MCP/HTTP entry points.
- The domain-crate reference workspace compiles and runs all three gated binaries through the production harness.

## Positions

- `../memory-kernel/crates/transport-harness/Cargo.toml`: implemented - production crate with empty defaults and independent CLI/MCP/HTTP features.
- `../memory-kernel/crates/transport-harness/src/lib.rs`: implemented - shared output, error normalization, CLI dispatch, MCP stdio startup, and HTTP startup/error mapping.
- `workflow-tools-contract-reference/crates/example/Cargo.toml`: implemented - each domain feature forwards to the sibling production harness.
- `workflow-tools-contract-reference/crates/example/src/bin/`: implemented - domain command, handler, and router registration use production harness entry points.
- `WORKFLOW_TOOLS_DOMAIN_CRATE_CONTRACT.md`: implemented - production placement and feature forwarding are documented.

## Validation

Validation `transport-harness-reference-20260725` passed on 2026-07-25. The
harness all-feature suite passed 5 tests; the full memory-kernel workspace
passed 155 tests with 1 ignored; reference default/all-feature tests and builds
passed; strict reference clippy was clean. CLI, MCP initialization, and live
HTTP health behavior were probed. External Edge at 1920x1080 rendered the HTTP
response and produced `workflow-tools-contract-reference/target/transport-harness-http-health.png`.

Kernel clippy completed with no errors and 24 inherited warnings in untouched
legacy source, matching the pre-existing strict-clippy blocker.

## Governing-rule requirement

This spec is introduced under the repository ticket/spec workflow in `AGENTS.md` and `.agents/instructions/spec-system.instructions.md`. The durable session’s pinned-rule renderer is blocked by stale rule IDs; this operational defect does not remove the contract.

## Non-goals

- Domain command schemas, MCP tools, and HTTP routes are not standardized by this crate.
- Frontend viewers and VS Code extensions remain outside the harness.