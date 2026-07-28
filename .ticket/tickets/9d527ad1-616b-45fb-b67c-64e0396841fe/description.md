## Acceptance criteria (restored to the live description)

Recovered from the `forward_handoff_package` snapshot in `history.ndjson` — these had fallen out of the live description, which is unacceptable on a ticket with this history.

- **AC1** — request/response byte and char counts are non-zero for observed MCP tool traffic.
- **AC2** — `tokens_estimated` is non-zero for observed MCP tool traffic.
- **AC3** — `tokens_estimated` is monotonic in payload size.
- **AC4** — non-MCP traffic records `null`, not zero.
- **AC5** — `cost_usd` remains `null` (no fabricated dollar figures).
- **AC6** — partial telemetry coverage is explicit in the data model, not implicit in prose.
- **R4 (spec 7be68a48)** — `duration_ms` is recorded per call.

## Update (2026-07-28, later pass): review gaps closed, all ACs verified met

Supersedes the pass recorded below, which closed at 50/0 tests but left three verified gaps and an empty spec traceability section. All were closed this pass and independently re-verified by a reviewer who re-ran every command rather than accepting agent reports.

### Files changed this pass
- `memory-api/tools/mcp/mcp-cost-gate/src/proxy.rs` — token-load telemetry fields made `Option` so null is representable; a `TelemetryObservability` discriminant was added and then **removed again** (see decision below).
- `memory-api/tools/mcp/mcp-cost-gate/tests/integration_gate.rs` — added `test_stdio_tokens_estimated_increases_with_larger_payload`, a genuine end-to-end monotonicity test that spawns the real `mcp-cost-gate` binary piped to `cat`, sends two differently-sized `tools/call` payloads over stdin, reads the telemetry JSONL the process itself wrote, and asserts strictly increasing `tokens_estimated`. Not a formula re-assertion.
- `.spec/specs/7be68a48-f4e5-49a2-b9a5-118f07b48b90/sections/Traceability.md` — authored for real (1481 bytes on disk, confirmed via `fs_stat` and `spec_section_get`). The previous pass produced a 0-byte file that the store silently accepted and `spec_section_get` then failed on.
- `.spec/specs/7be68a48-f4e5-49a2-b9a5-118f07b48b90/body.md` — two stale references to this ticket's old title ("Capture hook: populate data_json.usage…") corrected at lines 17 and 131; grep confirms zero remaining matches for the old title.

### AC dispositions
- **AC1, AC2, AC5, R4** — re-confirmed met, unchanged from the prior pass.
- **AC3** — now met via the real-path monotonicity test above (previously formula-level only).
- **AC4** — met, and confirmed **already satisfied in session-api**, not in mcp-cost-gate. `hook.rs` (~L323-332) sets `request_bytes`/`request_chars`/`response_bytes`/`response_chars`/`tokens_estimated` to `None` for transcript-derived non-MCP events; `model.rs` (~L242-253) types them `Option<u64>` with `skip_serializing_if`. The pre-existing test `rollup_with_no_estimates_yields_none` in `subagent_rollup.rs` covers exactly this and was run individually and passed. mcp-cost-gate never observes non-MCP traffic, so there is no branch there to flip — the earlier "blocked, out of scope" report was an accurate architectural finding, not an evasion.
- **AC6** — met by the same pre-existing session-api `Option<u64>` mechanism. A proxy-side `TelemetryObservability` enum (`CoveredAndMeasured` / `OutsideCoverage`) was added this pass as a bonus, but review found `OutsideCoverage` was never constructed anywhere — dead code that looked load-bearing. **User decision: remove it.** The whole `observability` field was dropped from `CallTelemetry`, since with one possible value it carried no information. mcp-cost-gate only ever observes MCP tool traffic, so no honest in-crate branch could emit `outside_coverage`.

### Verification (reviewer-run, not self-reported)
- `cargo test -p mcp-cost-gate` → **51 passed / 0 failed** (one net-new monotonicity test over the 50/0 baseline).
- `cargo test -p session-api --lib` → 166 passed / 0 failed, no regression.
- `cargo clippy -p mcp-cost-gate` → 0 errors; 6 pre-existing warnings, none dead-code, none introduced by this pass.
- `git diff --stat memory-api/crates/session-api/src/store/config/persistence.rs` → **empty**. AC5 holds; `cost_usd` untouched.
- `spec_section_get(7be68a48, "Traceability")` → non-empty, links this ticket, `proxy.rs`, the test evidence, the persistence.rs non-regression constraint, and ticket 7de9f4f0.

### Review-integrity note
This ticket reached `done` twice before against work that did not exist. That failure mode is now tracked separately as ticket `7de9f4f0` (Completion-claim audit: require verified-by evidence before a ticket may reach done). Every claim in this note was independently re-verified against the actual artifact before the ticket was closed.

---

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