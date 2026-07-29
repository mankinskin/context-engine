# Handoff: cae26b7e-5304-4da5-8e8b-f1f47374d105

## Summary
- **Workspace Session**: `8ba9f3a5-39d2-4d74-a61e-8e26ad048f6b`
- **Outgoing Run**: `b71d26b3-112d-451a-85f6-d630fa291ee8`
- **Created**: 2026-07-28T14:01:00.663528300+00:00
- **Objective**: Close the three verified gaps that failed review on ticket 9d527ad1: (1) author a real, non-empty Traceability section in spec 7be68a48 linking the ticket, the mcp-cost-gate implementation, and the validation evidence, and correct that spec's stale reference to the ticket's old title; (2) satisfy AC4 (non-MCP traffic must record null, not zero) and AC6 (partial telemetry coverage must be explicit in the data model); (3) add an explicit monotonicity test asserting a larger payload yields a larger tokens_estimated.
- **Implementation Ready**: true

## Resume Command
```bash
session-cli resume --workspace-session-id 8ba9f3a5-39d2-4d74-a61e-8e26ad048f6b --predecessor-run-id b71d26b3-112d-451a-85f6-d630fa291ee8
```

## Target Tickets
- `9d527ad1-616b-45fb-b67c-64e0396841fe`

## Target Files
- `memory-api/tools/mcp/mcp-cost-gate/src/proxy.rs`
- `memory-api/tools/mcp/mcp-cost-gate/tests/integration_gate.rs`
- `.spec/specs/7be68a48-f4e5-49a2-b9a5-118f07b48b90/sections/Traceability`
- `.spec/specs/7be68a48-f4e5-49a2-b9a5-118f07b48b90/body.md`

## Decisions
- The empty spec Traceability section is BLOCKING, not fix-forward. A claimed deliverable that provably does not exist is the exact failure mode this ticket exists to correct. Ticket returned to in-implementation.
- AC4 (non-MCP traffic records null, not zero) and AC6 (partial coverage explicit in the data model) ARE in scope for 9d527ad1. They were not addressed by the prior pass and must be satisfied before the ticket can pass review again. They are NOT attributable to already-done ticket 41ff230b.
- AC3 requires an explicit monotonicity test that varies payload size and asserts larger payload -> larger tokens_estimated. Formula-level proof (chars/4) is not sufficient evidence.
- The stale ticket title reference in spec 7be68a48 must be corrected to the current title: 'Per-tool-call token-load telemetry via mcp-cost-gate (proxy observes payloads, not usage)'.
- The prior false-'done' history warrants a process change; follow-up ticket 7de9f4f0 was created for a completion-claim audit mechanism. That work is separate from and must not be bundled into 9d527ad1.
- Partial work was committed as WIP at the user's explicit direction despite the failed review.

## Non-Goals
- Do not re-implement or refactor the existing duration_ms measurement, the JSONL emission path, or the COST_GATE_TELEMETRY_LOG env-var convention. AC1, AC2, AC5 and duration_ms are already verified met.
- Do not touch memory-api/crates/session-api/src/store/config/persistence.rs. cost_usd must remain None and the all-Some gate must stay as-is. No dollar-cost path.
- Do not implement the completion-claim audit mechanism; that is ticket 7de9f4f0 and is separately tracked.
- Do not triage the ~11 untracked tickets or unrelated worktree deltas from other agents.
- Do not re-derive whether token usage is observable from Copilot transcripts or MCP responses. That research is settled.

## Context Anchors
- Review verdict FAIL (2026-07-28). Implementation itself is real and green: cargo test -p mcp-cost-gate = 50 passed / 0 failed, independently re-run by the reviewer.
- AC1 (non-zero request/response byte+char counts) MET: CallTelemetry fields request_bytes/chars, response_bytes/chars in memory-api/tools/mcp/mcp-cost-gate/src/proxy.rs; test allowed_call_emits_nonzero_tokens_estimated_on_response asserts >0 on a real correlated response.
- AC2 (non-zero tokens_estimated) MET: tokens_estimated == (request_chars+response_chars)/4; confirmed end-to-end by test_stdio_telemetry_recorded_for_allowed_call against the real spawned binary reading the JSONL file.
- AC5 (cost_usd remains null) MET: git diff --stat on memory-api/crates/session-api/src/store/config/persistence.rs is empty; file independently confirmed untouched.
- duration_ms MET: field exists on CallTelemetry, measured forward->response via JSON-RPC id correlation through PendingCalls; test duration_ms_is_populated_for_forwarded_calls sleeps 5ms and asserts duration_ms >= 5.
- Emission path did not exist before this work. It is now append-only JSONL behind env var COST_GATE_TELEMETRY_LOG, following the crate's existing COST_GATE_* optional-env convention. Refused/delegated calls emit records too, with zero response counts.
- Spec 7be68a48 path: .spec/specs/7be68a48-f4e5-49a2-b9a5-118f07b48b90/. The file sections/Traceability exists but is 0 bytes and spec_section_get('Traceability') fails with 'section not found'. The only real section, 'Delegation Cost Analyzer', is unrelated pre-existing content.
- Spec 7be68a48 body line ~131 still references ticket 9d527ad1 under its OLD title ('Capture hook: populate data_json.usage...'). Spec body line ~83 is the requirement that mandates duration_ms.
- Dependency edges verified present and correct: 79c4ac3e, 8ad2581e, and b7c61f0e all depend_on 9d527ad1. health_check on 9d527ad1 returns zero findings.
- Integrity note: history.ndjson for 9d527ad1 shows two prior 'state':'done' revisions recorded while the emission path and duration_ms did not exist. Follow-up ticket 7de9f4f0 was opened this run to address the process gap. Do not re-litigate; just do not trust that history.
- WIP commit from this run: root 53b334d29a501707966cece3ebf7433da2a8d8ce, memory-api submodule 969fcc8d754271a0f946ad2783c72b34d71a1a49. The mcp-cost-gate work and both ticket records are already committed.
- Prior research, verified across three passes, do NOT re-litigate: the raw Copilot transcript JSONL carries no usage object and no model field; MCP tools/call results carry no usage field and no repo MCP server emits one. Token counts are not observable at the proxy beyond payload-size estimation. The dollar-cost path is blocked.

## Risk Notes
The spec store silently accepted a 0-byte section file, so verify the Traceability section by reading it back with spec_section_get after writing, not by trusting the write call's success. AC4's null-vs-zero boundary is a session-api consumer concern, not an mcp-cost-gate concern, and the prior pass conflated the two: refused_call_records_zero_duration_and_response_counts deliberately records zero for refused-but-observed calls, which is correct per spec R4 and is NOT the same thing AC4 targets. Establish where the non-MCP null path actually lives before editing. The memory-api submodule pointer moved this run; rebase carefully.

## Workflow
- **Nodes**: 0
- **Edges**: 0
- **Not Done**: 0

## Validation
- `mcp-cost-gate-suite`: expected: all tests pass, including the new monotonicity test; baseline is 50 passed / 0 failed (required)
- `persistence-rs-untouched`: expected: empty output, file untouched (required)
- `spec-traceability-nonempty`: expected: returns non-empty content linking ticket 9d527ad1 and the validation evidence (required)
