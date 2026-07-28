## Implementing Ticket

- [9d527ad1 Per-tool-call token-load telemetry via mcp-cost-gate (proxy observes payloads, not usage)](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml) — implements R4 (token-load coverage) and AC3/AC6 of this spec.

## Implementation

- [memory-api/tools/mcp/mcp-cost-gate/src/proxy.rs](memory-api/tools/mcp/mcp-cost-gate/src/proxy.rs) — the `CallTelemetry` struct records request/response byte and char counts and the derived `tokens_estimated` field for each MCP tool call the proxy observes.

## Validation Evidence

- `cargo test -p mcp-cost-gate` — baseline 50 passed / 0 failed.
- [memory-api/tools/mcp/mcp-cost-gate/tests/integration_gate.rs](memory-api/tools/mcp/mcp-cost-gate/tests/integration_gate.rs) — integration coverage for the gate/proxy behavior.

## Non-Regression Constraint

- [memory-api/crates/session-api/src/store/config/persistence.rs](memory-api/crates/session-api/src/store/config/persistence.rs) must remain untouched by this ticket's work: `cost_usd` stays `null` in `session-api` records, per the honest-absence constraint in R4.

## Related Ticket

- [7de9f4f0 Completion-claim audit: require verified-by evidence before a ticket may reach done](.ticket/tickets/7de9f4f0-0189-40c7-ac0a-0669e2aab57c/ticket.toml) — exists because two prior `done` revisions in ticket 9d527ad1's `history.ndjson` were recorded against work that did not exist; this audit ticket tracks the review-integrity follow-up.
