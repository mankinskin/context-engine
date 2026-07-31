## Review Outcome

**Verdict:** PASS with AC7 waived.

**Decision date:** 2026-07-27 (user-approved waiver).

**Gating logic coverage:** Unit tests (`evaluate_with_rollup`, `graded_budget_boundary_with_rollup`, `expensive_measured_tool_is_refused`) and proxy integration tests (`unmeasured_tool_fail_open`) provide comprehensive coverage of fail-open and graded-cost behavior for both measured and unmeasured tools.

**AC7 deferral:** Live end-to-end verification of expensive-model MPC calls is deferred to [56be2eaa Integration test harness for live expensive-model gating verification](memory-api/tickets/56be2eaa-6b32-4291-857a-b2c1aa24f273/ticket.toml). Reason: mcp-cost-gate is a stdio proxy with no standalone CLI verdict; verification requires an MCP server restart and an expensive-tier caller.

**Known finding tracked separately:** Pre-existing bidirectional substring over-match in `tool_cost()` (tool names matching both directions between request and rollup keys) is tracked in [9c29d697 Reduce false-positive tool name matches in cost gate](memory-api/tickets/9c29d697-f782-4737-aea1-645abf75cfb9/ticket.toml).