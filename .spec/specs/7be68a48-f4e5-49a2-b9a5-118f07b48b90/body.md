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
- **Dependency on token/cost capture**: the spec requires that token, cost, and model attribution flow through the normal `session-api` data path once ticket [9d527ad1 Capture hook: populate data_json.usage](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml) lands. No parallel data path or workaround for null fields.

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

## R4: Token/Cost Capture Dependency

The backend infrastructure for per-turn token/cost/model attribution is **architecturally complete** as of ticket [6549b6a7](.ticket/tickets/6549b6a7-8957-4df0-ada5-8fefb49c015c/ticket.toml) (done).

However, all token/cost fields are **null on disk** in real sessions because the VS Code Copilot capture hook does not populate `data_json.usage`.

**This spec requires** that once ticket [9d527ad1](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml) lands, token/cost/model data flows through the normal `session-api` extraction path (`hook.rs` L251-290 reads from `data_json.usage` and populates `event_meta` fields).

No parallel data path, no workaround for the null fields: the quality-gate consumer (ticket [8ad2581e](.ticket/tickets/8ad2581e-d9c0-4d24-b913-2b5ee77b2eeb/ticket.toml)) reads from `session-api` queryable surfaces (`subagent_rollups`, per-turn `event_meta`) once the capture source supplies the data.

# Acceptance Criteria

1. **AC1 (Quality gates defined)**: Pre- and post-delegation quality gates are defined as structured checks with pass/fail/blocked outcomes, evaluated before and after delegated sessions.

2. **AC2 (Gates recorded)**: Quality gate outcomes are recorded as `test-api` validation executions linked to the delegated session id and owning session id.

3. **AC3 (Session data captured)**: `session-api` records session and delegated-session data with per-turn token/cost/model attribution fields (populated once ticket [9d527ad1](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml) lands).

4. **AC4 (Tool-call data captured)**: Tool-call metrics are aggregated per session via existing tool-metrics infrastructure and queryable per delegated session.

5. **AC5 (Schema supports per-model rate)**: The data schema supports querying for a given model: count of delegated sessions, count of passed/failed post-gates, aggregated token/cost/time, and computing a satisfactory-work rate.

6. **AC6 (No parallel data path)**: Token/cost/model data flows through the normal `session-api` path (`data_json.usage` → `hook.rs` extraction → `event_meta` fields); no workaround or shim for null fields before ticket [9d527ad1](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml) lands.

# Traceability / Evidence

**Tickets**:
- [41ff230b Quality gates and session/tool-call data collection for delegated sessions](.ticket/tickets/41ff230b-cedf-4ec3-86cf-9b48a89b8325/ticket.toml) — this spec's implementing ticket.
- [6549b6a7 Session store: record per-turn/per-sub-agent token and cost with model attribution](.ticket/tickets/6549b6a7-8957-4df0-ada5-8fefb49c015c/ticket.toml) — backend infrastructure (done, but token/cost fields null on disk).
- [9d527ad1 Capture hook: populate data_json.usage so token/cost/model telemetry is non-zero](.ticket/tickets/9d527ad1-616b-45fb-b67c-64e0396841fe/ticket.toml) — blocker for non-null token/cost data.
- [8ad2581e Delegation quality/cost metric and self-optimization loop](.ticket/tickets/8ad2581e-d9c0-4d24-b913-2b5ee77b2eeb/ticket.toml) — downstream consumer of this spec's data model.

**Validation specs**:
- `val-session-api-lib-suite` — session-api library test suite (`.test/default/specs/val-session-api-lib-suite.json`).
- `val-session-api-build` — session-api build validation (`.test/default/specs/val-session-api-build.json`).

**Related specs**:
- [a4d61b8c Model cost awareness and tiered model routing](../.spec/specs/a4d61b8c-df1c-454d-ab56-4bce5706eb15/spec.toml) — adjacent model routing/cost policy (out of scope for this spec).

# Open Questions

None. All acceptance criteria in ticket [41ff230b](.ticket/tickets/41ff230b-cedf-4ec3-86cf-9b48a89b8325/ticket.toml) are specifiable from the documented sources.
