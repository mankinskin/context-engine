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