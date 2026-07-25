Phase A foundation. Provide a shared transport-harness library crate that factors out the common cli / mcp / http scaffolding (argument parsing, MCP server setup, HTTP router/error mapping, output formatting) so each domain crate's transport binary targets reuse it instead of duplicating boilerplate across all 11 domain crates.

## Motivation
The single-domain-crate decision (`0da6894c`) collapses cli/mcp/http into each domain crate as `[[bin]]` targets. Without a shared harness, transport scaffolding would be duplicated 11×. The harness keeps the transports DRY while each domain crate supplies only its domain-specific command/handler wiring.

## Scope
- Extract the reusable transport scaffolding into a `transport-harness` shared crate (default placement: the `memory-kernel` repo as a sibling crate; final placement is a minor open sub-decision — may become its own shared repo if it grows).
- Define the harness API each domain crate's bins call to register commands/handlers for cli, mcp, and http.
- Ensure the harness is transport-feature-aware so domain crates can feature-gate which transports build.

## Acceptance criteria
- `transport-harness` builds/tests independently and is consumable by domain crates.
- A reference domain crate wires all three transports (cli/mcp/http) through the harness with no duplicated scaffolding.
- Harness supports feature-gated transport selection.
- Documented in the domain-crate contract (`0da6894c`) as the required scaffolding dependency.

## Dependencies
- Builds on `memory-kernel` extraction; gates the per-tool extraction tickets (their bins depend on the harness).

## Implementation

- Added `crates/transport-harness` to the standalone `memory-kernel` workspace.
- Added empty-default `cli`, `mcp`, and `http` features. The crate owns shared
	output, tracing, transport-aware errors, CLI parsing/dispatch, MCP stdio
	serving, HTTP listener startup, and structured HTTP error responses.
- Domain code retains command dispatch, MCP `ServerHandler`, and HTTP `Router`
	registration through feature-scoped harness re-exports.
- Redirected the reference domain from its demonstration crate to the sibling
	production harness and removed the local demonstration manifest/source.

## Validation

Validation execution `transport-harness-reference-20260725` passed:

- `cargo test -p transport-harness --all-features`: 5 passed.
- `cargo test --workspace --all-features` in `memory-kernel`: 155 passed, 1 ignored.
- `cargo clippy --workspace --all-targets --all-features`: 0 errors and 24 inherited warnings in untouched kernel source.
- Reference default and all-feature tests/builds passed; strict clippy had no findings.
- CLI emitted `example-cli`; MCP returned a valid protocol `2025-03-26` initialize response; HTTP `/health` returned `example-http`.
- External Edge at 1920x1080 rendered `example-http`; screenshot saved at `workflow-tools-contract-reference/target/transport-harness-http-health.png`. The only console error was an unsolicited `/favicon.ico` 404.

Administrative state transition remains subject to the known `task` schema
defect (`store error: no schema for type 'task'`).

## Review outcome (2026-07-25 review-only pass)

Criterion-level status after the follow-up review:

- Feature-gated transport selection (default = [], independently selectable CLI/MCP/HTTP): ACCEPTED (unchanged; not re-reviewed).
- Independent build/consumption (9451f439): follow-up APPROVED (Pass). Submodule at `memory-kernel/`, reference consumes harness via a git branch-pinned dependency, submodule is development-only.
- Canonical contract ownership (f10f52e4): follow-up APPROVED (Pass). New `.spec` store in memory-kernel owns the canonical spec; validation evidence rooted in memory-kernel; context-engine references without duplicating. Depends on 9451f439.
- Realistic reference wiring (2cc7680c): follow-up NEEDS CHANGES. A design precursor (60114a17) must complete and be accepted first; durable proof moves into memory-kernel integration tests; one op across all three transports; assert success output AND harness error envelope + HTTP status mapping. Depends on 60114a17, 9451f439, f10f52e4.

Parent remains BLOCKED and not accepted until: 9451f439 and f10f52e4 are implemented, and 2cc7680c's design precursor (60114a17) is accepted and 2cc7680c implemented. No implementation was performed during this review-only pass.

Implementation order (recorded): 9451f439 → f10f52e4 → 60114a17 (design) → 2cc7680c (implementation) → parent acceptance.