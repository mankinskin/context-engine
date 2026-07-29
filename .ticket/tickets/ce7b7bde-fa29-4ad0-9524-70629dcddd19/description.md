## Problem

Three independent post-mortem agents (review, roast, session forensics) converged on one recurrence mechanism: **proxy evidence was accepted in place of outcome evidence**, repeatedly, for months.

Evidence:

- Ticket 84c7757d was marked `done` by **code-existence tracing** — "all 3 acceptance criteria traced to tool_metrics.rs, hook.rs, hook/tool_execution.rs" — while the file those criteria describe was `{"tools":{}}`.
- The validation record [.test/default/specs/val-session-api-lib-suite.json](.test/default/specs/val-session-api-lib-suite.json) is a per-module unit-test count ("148 tests passed: … tool_metrics::tests (10) …") with no artifact read-back. All ten of those tests hand-construct `role: Tool` turns the producer never emits.
- Session `d4868a37` correctly diagnosed "four independent breaks", created tickets 4aa13ba7 and 574560bf, then pivoted in the same session to unrelated review work. Both tickets sat `new` ever since while ~6 later sessions worked adjacent to them.

User decision (2026-07-29): add an enforced rule plus a mandatory e2e validation spec.

## Deliverables

1. **Instruction rule** under `.agents/instructions/` (testing scope): an acceptance criterion whose wording asserts data is *captured / collected / populated / persisted / recorded* must be verified by **reading the produced file or record**. Citing source code, a test name, or a passing-test count does not satisfy such an AC. The reviewer records the artifact path and the observed value.
2. **Validation spec** `val-session-api-tool-metrics-e2e` recorded in `.test`: drives a producer-shaped transcript through the real `copilot-capture-hook` binary and asserts the resulting `tool-metrics.json` has a non-empty `tools` map. Would have failed on every session prior to `7df14ea`.
3. **Corollary rule**: when a session's own diagnosis produces a new blocking ticket, that ticket becomes the next unit of work in the session unless the user explicitly defers it — directly targeting the `d4868a37` pivot.

## Acceptance criteria

- AC1 — The instruction file exists, is discoverable through the normal instruction-loading path, and names the forbidden evidence forms explicitly.
- AC2 — `val-session-api-tool-metrics-e2e` exists in `.test` with a recorded execution against the current code, verified by reading the execution record.
- AC3 — Deliberately reverting `compute_session_summary_with_events` to the turn-only computation makes the validation spec **fail**. This is the proof the gate is real; record the observed failure.
- AC4 — The corollary rule is added to the relevant orchestration instruction file and cross-references this ticket for the evidence.