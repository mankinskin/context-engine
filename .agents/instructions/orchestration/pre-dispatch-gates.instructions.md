---
description: "Use at the start of and throughout every session, immediately before spawning any sub-agent: pre-dispatch quality gates for delegation. Covers per-delegation-class gate sets, tool calls implementing each check, and fail-fast semantics."
applyTo: "**"
---

## Purpose

Quality gates that run AFTER dispatch cost full delegation loops when preconditions fail. This instruction defines pre-dispatch gate sets that catch bad units while they are still cheap — before spawning the sub-agent.

## When to Apply

Run pre-dispatch gates for EVERY delegation, regardless of delegation class. Each class has its own gate set tailored to its common precondition failures.

## Gate Execution Model (MANDATED — no either/or)

**Mechanism (binding, single choice)**: The orchestrator cannot run gates directly (it has no tools), so a **cheap gate sub-agent MUST be dispatched before every delegation**. There is no fallback "orchestrator tool grant" option — that alternative is rejected: it would break the orchestrator's structural no-direct-tools constraint for marginal savings on simple checks, and it does not compose with context-bundle sharing the way a gate sub-agent does. The gate agent is the workspace **Explore Agent** template (`.agents/agents/explore.agent.md`), formally designated as the pre-dispatch gate agent — see "Acting as the Pre-Dispatch Gate" in that template. It runs on the T3 floor model (`"GPT-5 mini (copilot)"`).

**Gate contract (explicit input/output)**:

- **Receives**: the delegation class (Implement/Review/Testing/Commit), the candidate ticket/spec ids or handoff package draft, and the specific gate set below for that class. Nothing else — the gate agent is context-isolated like any sub-agent.
- **Must return exactly one of**:
  - `{pass: true, bundle: {...}}` — the resolved context bundle (ticket, specs, paths, validation commands per the gate set's "Output" line below), ready to hand to the real delegation's sub-agent unmodified.
  - `{pass: false, blocker: "<single exact reason>"}` — one concrete, actionable blocker (not a list, not a hedge). Example: `"ticket fb14754e is in state 'blocked', not dispatchable"`, not "there might be an issue with the ticket."

**Fail-fast semantics (binding)**: `pass: false` means the delegation is **NOT dispatched**, full stop. The orchestrator MUST do exactly one of:

1. **Resolve** the precondition itself (create the missing spec, update the ticket state, fix the handoff package), then re-run the gate once, or
2. **Escalate** to the user if resolution requires a decision outside the orchestrator's authority (see [escalation-gate.instructions.md](escalation-gate.instructions.md)).

Re-dispatching the same blocked unit without resolving the blocker is the exact failure mode this ticket exists to close (`redispatch_count` in the AC5 benchmark below) and is never acceptable.

**Cost ceiling (structurally enforced in the gate contract, not just asserted)**: The gate agent's own template caps it at **≤5 turns and ≤10 tool calls** per invocation (see the "Hard Ceiling" clause in explore.agent.md's gate section). This is a HARD ceiling enforced by the dispatched template's own contract, not a target that drifts upward as delegation quality improves. If the gate agent cannot reach a verdict within the ceiling, it MUST return `{pass: false, blocker: "gate exceeded its 5-turn/10-tool-call ceiling before reaching a verdict"}` rather than continue investigating.

## Per-Delegation-Class Gate Sets

### Implement Delegation

**Purpose**: Verify the implementation unit is dispatchable and has the context it needs.

**Gates** (all must pass):

1. **Ticket exists and is dispatchable**
   - Tool: `ticket_get <id>`
   - Block if: ticket does not exist, or `state ∉ {new, ready, in-implementation}`
   - Pass: ticket path, current state, title

2. **Spec coverage exists**
   - Tool: `spec_search <ticket-title>` or `spec_list --where ticket_ids=<id>`
   - Block if: no spec references this ticket
   - Pass: matching spec id(s)

3. **Target paths exist**
   - Tool: `peek_skeleton <path>` or `list_dir <dir>`
   - Block if: declared target file/directory does not exist in workspace
   - Pass: confirmed path set

4. **Validation commands are present and non-empty**
   - Tool: read handoff package or session validation gates
   - Block if: validation section is empty, missing, or contains only placeholder text
   - Pass: exact command list to run

**Output**: `{pass: true, ticket: <resolved-ticket>, specs: [<spec-ids>], paths: [<confirmed-paths>], validation_cmds: [<commands>]}` OR `{pass: false, blocker: "<exact-reason>"}`

### Review Delegation

**Purpose**: Verify the implementation produced the evidence that review will check.

**Gates**:

1. **Implementation delegation declared test/validation obligations**
   - Tool: read prior implement sub-agent's return or handoff package
   - Block if: no test evidence, no validation results, no "done" criteria declared
   - Pass: test result pointers, validation command output references

2. **Ticket state allows review**
   - Tool: `ticket_get <id>`
   - Block if: `state ∉ {in-review, done}`
   - Pass: current state

**Output**: `{pass: true, evidence: [<test-pointers>], ticket_state: <state>}` OR `{pass: false, blocker: "<exact-reason>"}`

### Testing Delegation

**Purpose**: Verify validation spec ids resolve and commands are executable.

**Gates**:

1. **Validation spec ids resolve**
   - Tool: `test_get_spec <id>` (when test-mcp is available)
   - Block if: spec id does not resolve or is not linked to the target ticket
   - Pass: resolved spec with command

2. **Test commands are well-formed**
   - Tool: parse command for obvious syntax errors (missing binary, malformed args)
   - Block if: command is obviously broken (e.g., `cargo test -p <missing-package>`)
   - Pass: command ready to run

**Output**: `{pass: true, specs: [<resolved-specs>], commands: [<validated-commands>]}` OR `{pass: false, blocker: "<exact-reason>"}`

### Commit Delegation

**Purpose**: Verify working tree state is known and ticket is committable.

**Gates**:

1. **Working tree state is known**
   - Tool: `git status --short` (via terminal tool or committed handoff state)
   - Block if: unknown dirty state, untracked files that should be staged, or conflicts
   - Pass: clean or staged state

2. **Ticket is in committable state**
   - Tool: `ticket_get <id>`
   - Block if: `state ∉ {in-review, done}`
   - Pass: ticket state

**Output**: `{pass: true, git_state: "<clean|staged>", ticket_state: <state>}` OR `{pass: false, blocker: "<exact-reason>"}`

## Integration with Orchestrator Template

**Required change to `.agents/agents/orchestrator.agent.md`** (DOCUMENT ONLY — Lane B will apply):

Add after the "Delegation contract" section:

```markdown
## Pre-Dispatch Quality Gates

Before EVERY delegation, dispatch the **Explore Agent** template as the pre-dispatch gate for that delegation class (mandated mechanism — no orchestrator tool-grant alternative). See `.agents/instructions/orchestration/pre-dispatch-gates.instructions.md` for the complete gate definitions.

**Gate mechanism**: Spawn `.agents/agents/explore.agent.md` on `"GPT-5 mini (copilot)"`. It returns `{pass: true, bundle: {...}}` with the resolved context bundle, or `{pass: false, blocker: "<exact reason>"}`.

**On gate failure**: the delegation is NOT dispatched. Resolve the precondition (create spec, update ticket state, fix handoff) or escalate, then re-run the gate — never re-dispatch a blocked unit without resolving the blocker.

**Cost ceiling**: the gate template's own contract caps it at ≤5 turns and ≤10 tool calls; exceeding it forces a `pass: false` verdict rather than continued investigation.
```

## Integration with Delegation Instructions

**Required change to `.agents/instructions/orchestration/orchestrator-delegation.instructions.md`** (DOCUMENT ONLY — Lane B will apply):

Add before the "Required Workflow" section:

```markdown
## Pre-Dispatch Quality Gates

Run pre-dispatch gates for EVERY delegation by dispatching the Explore Agent template (`.agents/agents/explore.agent.md`, `"GPT-5 mini (copilot)"`) as the mandated gate mechanism. Each delegation class (Implement, Review, Testing, Commit) has its own gate set. See `.agents/instructions/orchestration/pre-dispatch-gates.instructions.md` for complete definitions.

Gate failures (`pass: false`) mean the delegation is NOT dispatched: fix the precondition or escalate BEFORE dispatch, never re-dispatch a blocked unit and hope it works.
```

## Validation

This is prose-only guidance that cannot be mechanically tested. The acceptance check is:
1. Gate definitions exist for all four delegation classes
2. Each gate specifies the exact tool call and pass/block criteria
3. The gate mechanism is a single mandated choice (cheap gate sub-agent via the Explore Agent template) with no unresolved either/or
4. Fail-fast semantics are stated: `pass: false` blocks dispatch until resolved or escalated
5. Integration points with orchestrator template and delegation instructions are documented
6. Cost ceiling (≤5 turns, ≤10 tool calls) is stated as a hard requirement enforced in the gate's own contract

## Relation to Benchmark

Benchmark ticket `10d21210` (now DONE) publishes the combined-baseline `redispatch_count` in [.benchmark/10d21210/README.md](../../../.benchmark/10d21210/README.md)'s thresholds table: **baseline 10 → target 0**, measured as `runSubagent` dispatches sharing `(agent_name, description)` with an earlier dispatch whose span recorded a failure. With pre-dispatch gates applied via the mandated gate mechanism above, the blocker is caught BEFORE dispatch, which is the mechanism this threshold measures — a post-change session replayed through the same harness is expected to show `redispatch_count = 0`. Actual measurement of a post-change run is not owed by this ticket; only the evidence path is cited here. The gate cost (≤5 turns) is far cheaper than a full delegation loop (20-64 turns in the measured sessions).

## Schema Gaps Discovered

The ticket description references schema gaps discovered during rework chains:
- `SessionValidationGate` missing `command` field (see tickets `8c67b96a`, `0d3fdba6`)

These gaps belong in their owning ticket/spec scopes, not in this gate definition. If a gate discovers a new schema gap, record it in the relevant ticket rather than expanding this gate file.
