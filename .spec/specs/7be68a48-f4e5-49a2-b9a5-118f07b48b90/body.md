# Summary

Define pre- and post-delegation quality gates and the data collection architecture that captures session, tool-call, and delegated-session data needed to measure how often delegated sessions produce satisfactory work, per model.

# Problem

The orchestration system delegates work to cheaper models (see spec [a4d61b8c Model cost routing](../.spec/specs/a4d61b8c-df1c-454d-ab56-4bce5706eb15/spec.toml)) but has no recorded evaluation of whether the delegated work meets quality standards. Without quality gates before and after each delegated session, and without structured data linking sessions → tool calls → delegated sessions → validation outcomes, the system cannot measure a per-model satisfactory-work rate or optimize the delegation policy toward the cheapest model that meets standards.

# Scope

- **Quality gate schema**: define what a quality gate is, when it is evaluated (pre- and post-delegation), and where gate outcomes are recorded.
- **Data collection points** across existing infrastructure:
  - `session-api` for session and delegated-session records (including per-turn token/cost/model attribution when available).
  - `test-api` for validation/quality-gate evidence.
  - existing session tool-metrics for tool-call data.
- **Data model requirements** that support computing a per-model satisfactory-work rate downstream (see ticket [8ad2581e Delegation quality/cost metric](.ticket/tickets/8ad2581e-d9c0-4d24-b913-2b5ee77b2eeb/ticket.toml)).
- **Dependency on token/cost capture**: the spec requires that token, cost, and model attribution flow through the normal `session-api` data path once ticket [9d527ad1 Per-tool-call token-load telemetry via mcp-cost-gate (proxy observes payloads, not usage)](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml) lands. No parallel data path or workaround for null fields.

# Non-Goals

- Fixing the capture hook that populates `data_json.usage` (that is ticket [9d527ad1](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml)).
- Changing model routing or cost policy (that is spec [a4d61b8c Model cost routing](../.spec/specs/a4d61b8c-df1c-454d-ab56-4bce5706eb15/spec.toml)).
- Building a new data store (reuse `session-api`, `test-api`, existing tool-metrics).
- Computing the delegation quality/cost metric itself (that is ticket [8ad2581e](.ticket/tickets/8ad2581e-d9c0-4d24-b913-2b5ee77b2eeb/ticket.toml)).

# Requirements

## R1: Quality Gate Definition

A **quality gate** is a recorded evaluation point with:
- **When**: evaluated before delegation (pre-gate) and after delegation completes (post-gate).
- **What**: a structured check (e.g., precondition validation, acceptance criteria, test suite pass/fail) that produces a pass/fail/blocked outcome.
- **Where recorded**: as a `test-api` validation execution linked to the delegated session id and the owning session id.

Pre-gate evaluates whether the delegated unit is well-formed (prompt clarity, required context present, acceptance criteria testable).
Post-gate evaluates whether the delegated unit's output satisfies acceptance criteria (tests pass, validation spec outcomes recorded, no unresolved blockers).

## R2: Data Collection Architecture

Capture must span three layers, reusing existing infrastructure:

1. **session-api**:
   - Session record: top-level session with per-turn token/cost/model attribution fields (`SessionTurnEventMeta.input_tokens`, `output_tokens`, `cache_read_tokens`, `cache_write_tokens`, `cost_usd`, `model_id`).
   - Delegated-session record: each `runSubagent` spawn captured as a nested session with its own per-turn attribution.
   - Sub-agent rollup: queryable aggregation of tokens, cost, wall time, and outcome per delegated session (see ticket [6549b6a7](.ticket/tickets/6549b6a7-8957-4df0-ada5-8fefb49c015c/ticket.toml)).

2. **test-api**:
   - Validation execution records for pre- and post-gate outcomes, linked to the delegated session id via `session_id` field.
   - Validation spec records defining the gate checks (e.g., `val-session-api-lib-suite`, `val-session-api-build`).

3. **tool-metrics** (existing session tool-metrics):
   - Per-tool-call counts, durations, and outcomes aggregated per session.
   - Used to attribute tool-call overhead and failure rates to specific delegated sessions.

## R3: Data Model for Satisfactory-Work Rate

The schema must support this downstream query (ticket [8ad2581e](.ticket/tickets/8ad2581e-d9c0-4d24-b913-2b5ee77b2eeb/ticket.toml)):

For a given model M and time window W:
- Count: delegated sessions using model M
- Count: delegated sessions where post-gate outcome = passed
- Count: delegated sessions where post-gate outcome = failed or blocked
- Aggregate: total token count, total cost USD, median wall time
- Rate: satisfactory-work-rate = (passed post-gates) / (total delegated sessions)

Required fields per delegated session record:
- `session_id` (delegated session)
- `parent_session_id` (owning session)
- `model_id` (acting model for the delegated work)
- `cost_usd` (aggregated across the delegated session)
- `outcome` (derived from post-gate validation execution)

Pre-gate and post-gate validation executions link via `session_id` to the delegated session.

## R4: Token-Load Coverage — Partial by Design with Measurement Limitations

**Measurement limitations**: Token usage and model identity telemetry **do not exist** at two critical boundaries:
1. The upstream Microsoft Copilot raw transcript format (`C:/Users/linus/AppData/Roaming/Code/User/workspaceStorage/<workspace-id>/GitHub.copilot-chat/transcripts/*.jsonl`) contains no `usage` object, no token fields (under any spelling), and no `model` field. The capture source is Microsoft's closed-source `GitHub.copilot` extension; this repository cannot modify or extend it.
2. MCP `tools/call` results carry no `usage` field; no repo MCP server emits one; `mcp-cost-gate`'s `proxy.rs` passes non-`tools/list` responses through untouched. Token counts are therefore not observable at the proxy.

**Coverage scope**: Token-load data will be **estimated** only from **MCP tool traffic** via the existing `mcp-cost-gate` proxy, which observes serialized request and response payloads. This coverage is **structurally partial** — it observes tool calls, not whole conversation turns.

**Measurement method**: The proxy records per-call: `timestamp`, `caller_model` (when supplied), tool name, `grant_id`, gate decision, request/response byte and character counts, `duration_ms`, and an **estimated token load** derived via a fixed char-per-token divisor (chars/4) over the serialized payloads. This is a rough empirical signal proportional to real token consumption; exact tokenization is explicitly out of scope. The estimate is named and typed distinctly (e.g., `tokens_estimated`) so it can never be read as an observed count.

**No dollar-cost computation**: Tools do not have a dollar cost. A tool call's only cost effect is the tokens it adds to the consuming agent's context, and the dollar cost arises there, in that agent's model billing. Therefore ticket [9d527ad1](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml) measures **token load**, never a dollar figure. No price-table lookup occurs at this boundary. The `cost_usd` field remains `null` in `session-api` records at this boundary.

**Honest-absence constraint** (normative): Absent **dollar cost** remains `null` in `session-api` records rather than being estimated, fabricated, or defaulting to zero. This constraint is enforced throughout quality-gate consumers (e.g., ticket [8ad2581e](.ticket/tickets/8ad2581e-d9c0-4d24-b913-2b5ee77b2eeb/ticket.toml)) and remains in force independently of ticket [9d527ad1](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml). Token-load estimation is explicitly exempt from this constraint; the estimate is clearly named and the partial-coverage boundary is recorded in the data so "no MCP traffic" is distinguishable from "telemetry unavailable".

**Known limitation**: Full-fidelity per-turn token/cost/model telemetry remains unavailable pending either (a) Microsoft emitting usage in the raw transcript format, or (b) a future repo-local VS Code extension built on the `vscode.lm` API. Both paths are outside this spec's scope and the implementation scope of ticket [41ff230b](.ticket/tickets/41ff230b-cedf-4ec3-86cf-9b48a89b8325/ticket.toml).

# Acceptance Criteria

1. **AC1 (Quality gates defined)**: Pre- and post-delegation quality gates are defined as structured checks with pass/fail/blocked outcomes, evaluated before and after delegated sessions.

2. **AC2 (Gates recorded)**: Quality gate outcomes are recorded as `test-api` validation executions linked to the delegated session id and owning session id.

3. **AC3 (Session data captured)**: `session-api` records session and delegated-session data with per-turn token/cost/model attribution fields (populated once ticket [9d527ad1](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml) lands).

4. **AC4 (Tool-call data captured)**: Tool-call metrics are aggregated per session via existing tool-metrics infrastructure and queryable per delegated session.

5. **AC5 (Schema supports per-model rate)**: The data schema supports querying for a given model: count of delegated sessions, count of passed/failed post-gates, aggregated token/cost/time, and computing a satisfactory-work rate.

6. **AC6 (No parallel data path)**: Token/cost/model data flows through the normal `session-api` path (`data_json.usage` → `hook.rs` extraction → `event_meta` fields); no workaround or shim for null fields before ticket [9d527ad1](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml) lands.

## Pre-Dispatch Gate Execution

**Requirement:** Quality gates MUST run before delegation dispatch, not after the delegated session completes. Pre-dispatch gates validate that the handoff package is implementation-ready (objective clear, context_anchors present, open_escalations empty) before expensive sub-agent tokens are spent.

**Contract:** The orchestration layer evaluates pre-gates and blocks dispatch when gates fail. Post-gates remain for outcome validation after the sub-agent returns, but pre-gates prevent wasted work when the package is malformed or incomplete.

**Related ticket:** [46d8b25d Move quality gates before dispatch](.ticket/tickets/46d8b25d-e80c-4170-9601-1c26a7a0bcb8/ticket.toml)

## Synthetic Benchmark Session

**Requirement:** A synthetic benchmark session with a checked-in baseline serves as the verification substrate for measuring delegation cost and quality improvements. The benchmark defines a reproducible task, expected delegation points, and acceptance criteria, enabling consistent before/after comparisons.

**Contract:** The benchmark session artifact (transcript, handoff packages, validation outcomes) is version-controlled. Changes to delegation policy, tool grants, or model routing can be verified against this baseline to confirm cost reduction without quality regression.

**Related ticket:** [10d21210 Define a synthetic benchmark session](.ticket/tickets/10d21210-7168-4ed4-8e99-f6fb0e6e08db/ticket.toml)

# Traceability / Evidence

**Epic:**
- [79c4ac3e Delegation cost](.ticket/tickets/79c4ac3e-fd53-48bf-babb-43d27555c4bd/ticket.toml)

**Tickets**:
- [41ff230b Quality gates and session/tool-call data collection for delegated sessions](.ticket/tickets/41ff230b-cedf-4ec3-86cf-9b48a89b8325/ticket.toml) — this spec's implementing ticket.
- [46d8b25d Move quality gates before dispatch](.ticket/tickets/46d8b25d-e80c-4170-9601-1c26a7a0bcb8/ticket.toml) — pre-dispatch gate execution.
- [10d21210 Define a synthetic benchmark session](.ticket/tickets/10d21210-7168-4ed4-8e99-f6fb0e6e08db/ticket.toml) — benchmark verification substrate.
- [6549b6a7 Session store: record per-turn/per-sub-agent token and cost with model attribution](.ticket/tickets/6549b6a7-8957-4df0-ada5-8fefb49c015c/ticket.toml) — backend infrastructure (done, but token/cost fields null on disk).
- [9d527ad1 Per-tool-call token-load telemetry via mcp-cost-gate (proxy observes payloads, not usage)](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml) — blocker for non-null token/cost data.
- [8ad2581e Delegation quality/cost metric and self-optimization loop](.ticket/tickets/8ad2581e-d9c0-4d24-b913-2b5ee77b2eeb/ticket.toml) — downstream consumer of this spec's data model.

**Validation specs**:
- `val-session-api-lib-suite` — session-api library test suite (`.test/default/specs/val-session-api-lib-suite.json`).
- `val-session-api-build` — session-api build validation (`.test/default/specs/val-session-api-build.json`).

**Related specs**:
- [a4d61b8c Model cost awareness and tiered model routing](../.spec/specs/a4d61b8c-df1c-454d-ab56-4bce5706eb15/spec.toml) — adjacent model routing/cost policy (out of scope for this spec).
- [b71658f1 Iteration Loop Workflow](../.spec/specs/b71658f1-8de2-444a-9be1-64b1d8ecce70/spec.toml) — adjacent iteration loop and phase-separation rules.

# Open Questions

None. All acceptance criteria in ticket [41ff230b](.ticket/tickets/41ff230b-cedf-4ec3-86cf-9b48a89b8325/ticket.toml) are specifiable from the documented sources.
