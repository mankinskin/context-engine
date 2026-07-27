# Validation and Traceability

## Review Status

**Specification reviewed on 2026-07-27.**

Related implementation ticket: [9185d8f2 Fail-open empirical tool-metrics gating](../../../memory-api/tickets/9185d8f2-1080-46b1-84da-485f9ad839f6/ticket.toml)

### Acceptance Criteria Summary

- **AC1–AC6**: ✅ All passed. Hardcoded constants removed, fail-open behavior verified, unit tests passing (26/26).
- **AC7** (live expensive-model MPC verification): ⏸️ **Waived** by user decision on 2026-07-27.
  - Reason: mcp-cost-gate is a stdio proxy with no standalone CLI for live verification; requires MCP server restart and expensive-tier caller.
  - Deferred to: [56be2eaa Integration test harness for live expensive-model gating verification](../../../memory-api/tickets/56be2eaa-6b32-4291-857a-b2c1aa24f273/ticket.toml)

### Gating Logic Coverage

Empirical fail-open and graded-cost behavior is covered by:
- Unit tests: `evaluate_with_rollup`, `graded_budget_boundary_with_rollup`, `expensive_measured_tool_is_refused`
- Proxy integration tests: `unmeasured_tool_fail_open`

### Known Findings

**Substring over-match in `tool_cost()`**: Bidirectional substring matching between requested tool name and rollup keys (pre-existing, not introduced by this change) is tracked in [9c29d697 Reduce false-positive tool name matches in cost gate](../../../memory-api/tickets/9c29d697-f782-4737-aea1-645abf75cfb9/ticket.toml).

## Related Tickets

- **Blocking deferred work**: [56be2eaa Integration test harness for live expensive-model gating verification](../../../memory-api/tickets/56be2eaa-6b32-4291-857a-b2c1aa24f273/ticket.toml)
- **Related finding**: [9c29d697 Reduce false-positive tool name matches in cost gate](../../../memory-api/tickets/9c29d697-f782-4737-aea1-645abf75cfb9/ticket.toml)
