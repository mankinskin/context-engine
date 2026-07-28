## Update (2026-07-28): duration_ms + emission path implemented

The prior "Implementation Complete" note above was inaccurate: it claimed AC1/AC2/AC3 satisfaction from `compute_payload_telemetry` alone, but nothing ever called it in production code — `CallTelemetry` was defined and unit-tested in isolation with no emission path, and `duration_ms` (required by spec 7be68a48 R4's "Measurement method") did not exist on the struct at all.

### Files changed this pass
- memory-api/tools/mcp/mcp-cost-gate/src/proxy.rs: added `duration_ms: u64` to `CallTelemetry`; added `PendingCall`/`PendingCalls` (JSON-RPC id correlation); `handle_client_message` and `handle_server_message` now return `(action, Option<CallTelemetry>)` and emit telemetry for every `tools/call` (allow/reject/delegate/missing-model), with response counts recorded as `0` (not omitted) when nothing was forwarded.
- memory-api/tools/mcp/mcp-cost-gate/src/main.rs: wired a shared `PendingCalls`, added `COST_GATE_TELEMETRY_LOG` (optional path, matches the existing `COST_GATE_*` env-var convention) and an `emit_telemetry` helper that appends each `CallTelemetry` as a JSONL line.
- memory-api/tools/mcp/mcp-cost-gate/tests/integration_gate.rs: updated call sites for the new signature; added `test_stdio_telemetry_recorded_for_allowed_call` (spawns the real binary, verifies a non-zero `tokens_estimated` and a `duration_ms` field for a real intercepted `tools/call`).

### New tests
- `proxy::tests::allowed_call_emits_nonzero_tokens_estimated_on_response`
- `proxy::tests::duration_ms_is_populated_for_forwarded_calls`
- `proxy::tests::refused_call_records_zero_duration_and_response_counts`
- `test_stdio_telemetry_recorded_for_allowed_call` (integration_gate.rs)

### Verification
`cargo test -p mcp-cost-gate`: 50 passed; 0 failed (35 unit + 15 integration).

`cost_usd` in `crates/session-api/src/store/config/persistence.rs` (lines ~267-282) remains untouched: still gated on `(Some(model_id), Some(input_tokens), Some(output_tokens))`, still `Option<f64>`, still `None` unless all three are present. Not edited by this pass.

### Contradiction found
This ticket was already in state `done` with a description claiming full implementation, but the emission path and `duration_ms` did not exist in the code. The ticket/spec requirements win per the task's instructions — this pass supplies the missing emission mechanism and field.