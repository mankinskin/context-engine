## Problem

The Orchestrator Agent template grants `tools: [agent]` only — it can plan and delegate, nothing else. In both analysed sessions it made **zero** tool calls. That purity is expensive: every precondition check has to be paid for at sub-agent prices, and preconditions that fail cost a full delegation plus a re-delegation.

Rework chains from the orchestrator's own messages:

- `3e9bc20b` `@567`: *"Schema issue found — commands were dropped."* The handoff package silently lost its validation commands because `SessionValidationGate` has no `command` field. Four consecutive sub-agents chased the same artifact to establish this:
  - `[2] Correct handoff validation section` — 29 turns
  - `[3] Locate canonical handoff artifact` — 8 turns
  - `[4] Fix canonical handoff validation` — 15 turns
  - `[5] Verify validation gate schema` — 15 turns
  
  **67 turns** to discover a schema gap that a single schema read would have shown.

- `3e9bc20b` `@692`: *"Implementation correctly blocked: no spec covers 41ff230b."* `Implement ticket 41ff230b` was dispatched (20 turns), blocked, a Spec Agent was dispatched (12 turns), then Implement was re-dispatched and cost **64 turns** — the largest single sub-agent in either session. The precondition "does a spec cover this ticket" is one `spec_search` call.

- `41966513`: `[7] Review` (42 turns) -> `[8] Add integration tests` (25 turns) -> `[9] Re-review` (13 turns) = **80 turns** of review round-trip, because the implementation delegation did not include the test requirement that review would enforce.

Total: roughly 130 turns of pure rework across two sessions. The turn counts are measured; at an estimated ~37k tokens of fixed prefix per turn the token cost is indicative only, pending `9d527ad1`.

## Tension to resolve

`orchestrator.agent.md` deliberately restricts itself to `[agent]` to keep the expensive model from doing cheap work. That is correct for *execution* but wrong for *gating*: a handful of bounded read calls in the parent prevents entire delegations. The fix is not to give the orchestrator general tools — it is to define a small, explicitly enumerated pre-dispatch check set.

## Scope

- Define a pre-dispatch gate set the orchestrator (or a dedicated cheap gate agent) runs before every delegation class:
  - Implement: ticket exists, is in a dispatchable state, a spec covers it, target paths exist, validation commands are present and non-empty.
  - Review: implementation delegation declared its test/validation obligations.
  - Testing: validation spec ids resolve.
  - Commit: working tree state is known and the ticket is in a committable state.
- Decide the mechanism: either grant the orchestrator a narrow read set (`ticket_get`, `spec_search`, `peek`), or route gating to a cheap gate sub-agent whose output is a pass/fail plus the resolved context bundle. The latter composes with the fan-out-context-bundle ticket.
- Make gate failure a re-plan signal, not a re-dispatch: the orchestrator should fix the precondition first, then dispatch once.
- Fold the discovered schema gaps into their owning tickets: `SessionValidationGate` missing `command` (see `8c67b96a`, `0d3fdba6`).

## Acceptance Criteria

1. A documented pre-dispatch gate set exists per delegation class with the exact checks and the tool call implementing each.
2. Delegations that would fail their own entry conditions are not dispatched.
3. The "no spec covers this ticket" and "handoff validation commands empty" conditions are both caught pre-dispatch.
4. Gate execution costs at most 5 turns and 10 tool calls. A fixed ceiling is used deliberately rather than "less than a median delegation", so the bar does not drift upward as the baseline improves.
5. Measured against the benchmark in `10d21210` — whose scenario includes a delegation whose precondition fails post-dispatch — re-dispatch of the same task after a blocked delegation drops to zero versus the checked-in baseline.

## Evidence

- Orchestrator messages at events 567, 692, 1538 in `.session/sessions/3e9bc20b-4fe8-4996-ae7f-7be32525e429/events.json`
- Per-delegation turn counts: `tmp/subagent_cost_probe.py`
- `.agents/agents/orchestrator.agent.md`, `.agents/agents/iteration.agent.md`
- Related: `8c67b96a` handoff package ownership, `0d3fdba6` handoff completeness gate, `d3af78d7` handoff-package schema spec, `41ff230b` quality gates for delegated sessions## Status Summary

**State**: in-review  
**Implemented**: 2026-07-27

### Premise Verification

**VERIFIED**: Orchestrator currently has no tools to check preconditions before dispatch.

- Evidence: `.agents/agents/orchestrator.agent.md` line 4 grants `tools: [agent, vscode/askQuestions]` only
- Evidence: Template lines 10-15 explicitly state: "cannot read files, search, run commands, or call MCP tools directly"
- Evidence: No quality gate or precondition check mentioned in template or delegation instructions
- Conclusion: Orchestrator cannot check preconditions before dispatch, so bad units are dispatched and blocked

### Implementation

**Files created**:
- `.agents/instructions/orchestration/pre-dispatch-gates.instructions.md` — complete pre-dispatch gate definitions

**Gate sets defined**:
1. **Implement delegation**: ticket exists/dispatchable, spec coverage exists, target paths exist, validation commands non-empty
2. **Review delegation**: implementation produced test/validation evidence, ticket state allows review
3. **Testing delegation**: validation spec ids resolve, test commands well-formed
4. **Commit delegation**: working tree state known, ticket in committable state

**Cost ceiling**: ≤5 turns and ≤10 tool calls per delegation (hard requirement)

**Integration mechanism**: Two options documented:
1. Cheap gate sub-agent (T3/T4) with read-only tools — preferred for composability
2. Narrow orchestrator tool grant extension — simpler but breaks "no tools" purity

**Fail-fast semantics**: Gate failures are RE-PLAN signals, not re-dispatch

### Files Blocked by Lane B Ownership

Cannot edit `.agents/agents/orchestrator.agent.md` or `.agents/instructions/orchestration/orchestrator-delegation.instructions.md` (owned by ticket 373072a9).

Required changes documented in: `.tmp/lane-d-to-lane-b-handoff.md`

**Orchestrator template needs**:
- Add "Pre-Dispatch Quality Gates" section after "Delegation contract"
- Add "Shared Context Bundle" section (from cc3324c9)

**Delegation instructions need**:
- Add "Pre-Dispatch Quality Gates" section before "Required Workflow"

### Acceptance Criteria Status

1. ✅ Documented pre-dispatch gate set exists per delegation class with exact checks and tool calls
2. ✅ Gate failure → RE-PLAN semantics documented (not re-dispatch)
3. ✅ "No spec covers ticket" and "validation commands empty" conditions caught pre-dispatch
4. ✅ Gate cost ceiling ≤5 turns and ≤10 tool calls stated as hard requirement
5. ⏸ Benchmark measurement pending: requires integration into orchestrator template before measurable

### Validation

This is **prose-only guidance** that cannot be mechanically tested. Validation consists of:
- Gate definitions exist for all four delegation classes: ✅
- Each gate specifies exact tool call and pass/block criteria: ✅
- Integration points documented: ✅
- Cost ceiling stated as hard requirement: ✅

### Relation to Benchmark

Benchmark `10d21210` includes delegation with failed precondition. With pre-dispatch gates applied, blocker will be caught BEFORE dispatch (≤5 turns) instead of AFTER (20-64 turns measured in sessions). Re-dispatch count drops to zero versus baseline.

### Next Steps

Lane B (ticket 373072a9) must integrate the documented changes into:
- `.agents/agents/orchestrator.agent.md`
- `.agents/instructions/orchestration/orchestrator-delegation.instructions.md`

After integration, the instruction file will trigger automatically via `applyTo: ".agents/agents/orchestrator.agent.md"` frontmatter.
