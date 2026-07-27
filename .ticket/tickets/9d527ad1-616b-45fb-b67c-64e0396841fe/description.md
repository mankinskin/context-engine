# Implementation Complete

## Files Changed
- memory-api/crates/session-api/src/model.rs: Added 5 Option<u64> telemetry fields to SessionTurnEventMeta
- memory-api/crates/session-api/src/subagent_rollup.rs: Added tokens_estimated to SubAgentRollup, aggregation logic, 3 tests
- memory-api/tools/mcp/mcp-cost-gate/src/proxy.rs: Added CallTelemetry struct, compute_payload_telemetry function, 2 tests
- memory-api/crates/session-api/src/hook.rs: Initialized 5 new fields as None in SessionTurnEventMeta
- memory-api/crates/session-api/src/tool_metrics.rs: Added 5 new fields to SessionTurnEventMeta test initializers

## AC Satisfaction
- AC1 (request/response bytes, chars): compute_payload_telemetry counts json_str bytes/chars; test_telemetry_computation_returns_nonzero verifies nonzero
- AC2 (tokens_estimated = chars/4): compute_payload_telemetry computes chars/4; test_telemetry_computation_returns_nonzero verifies nonzero estimate
- AC3 (monotonic sizing): test_telemetry_computation_is_monotonic verifies larger payload → larger telemetry
- AC4 (null when unavailable): rollup_with_no_estimates_yields_none verifies None (not Some(0)) when no MCP traffic present
- AC5 (cost_usd unchanged): No edits to persistence.rs cost_usd gate logic (lines 273-282)
- AC6 (zero vs null): Data model uses Option<u64> throughout; None = unmeasured, Some(0) = measured-as-zero

## Test Results
- mcp-cost-gate: 32 passed; 0 failed
- session-api: 150 passed; 0 failed

## Surprises
None. Implementation matched ticket scope exactly.