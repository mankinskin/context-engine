---
description: "Pre-dispatch quality gates for orchestrator delegation. Use before spawning any sub-agent to catch precondition failures cheaply. Covers per-delegation-class gate sets, tool calls implementing each check, and fail-fast semantics."
applyTo: ".agents/agents/orchestrator.agent.md"
---

## Purpose

Quality gates that run AFTER dispatch cost full delegation loops when preconditions fail. This instruction defines pre-dispatch gate sets that catch bad units while they are still cheap — before spawning the sub-agent.

## When to Apply

Run pre-dispatch gates for EVERY delegation, regardless of delegation class. Each class has its own gate set tailored to its common precondition failures.

## Gate Execution Model

**Mechanism**: The orchestrator cannot run gates directly (it has no tools). Two options:

1. **Cheap gate sub-agent** (preferred for composability with context bundle sharing): spawn a dedicated gate agent on a T3 model with a narrow read-only tool grant. The gate agent returns `pass` with the resolved context bundle, or `block` with the exact reason.
2. **Orchestrator tool grant extension**: grant the orchestrator a narrow read-only tool set (`ticket_get`, `spec_search`, `peek_skeleton`, `list_dir`). This breaks the "no direct tools" purity but may be cheaper than a gate sub-agent for simple checks.

**Fail-fast semantics**: A gate failure is a RE-PLAN signal, not a re-dispatch. The orchestrator MUST address the precondition (e.g., create the missing spec, update the ticket state, fix the handoff package) BEFORE dispatching the blocked unit.

**Cost ceiling**: Gate execution for any single delegation MUST cost at most 5 turns and 10 tool calls. This is a HARD ceiling, not a "less than median delegation" target — the bar does not drift upward as delegation quality improves.

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

Before EVERY delegation, run the pre-dispatch gate set for that delegation class. See `.agents/instructions/orchestration/pre-dispatch-gates.instructions.md` for the complete gate definitions.

**Gate mechanism**: Spawn a cheap gate sub-agent (T3 model) with read-only tool access, or use a narrow orchestrator tool grant if available. The gate agent returns `pass` with the resolved context bundle, or `block` with the exact blocker.

**On gate failure**: RE-PLAN, do not re-dispatch. Address the precondition (create spec, update ticket state, fix handoff) BEFORE dispatching the blocked unit.

**Cost ceiling**: Gates for any single delegation MUST cost ≤5 turns and ≤10 tool calls.
```

## Integration with Delegation Instructions

**Required change to `.agents/instructions/orchestration/orchestrator-delegation.instructions.md`** (DOCUMENT ONLY — Lane B will apply):

Add before the "Required Workflow" section:

```markdown
## Pre-Dispatch Quality Gates

Run pre-dispatch gates for EVERY delegation. Each delegation class (Implement, Review, Testing, Commit) has its own gate set. See `.agents/instructions/orchestration/pre-dispatch-gates.instructions.md` for complete definitions.

Gate failures are RE-PLAN signals: fix the precondition BEFORE dispatch, never re-dispatch a blocked unit and hope it works.
```

## Validation

This is prose-only guidance that cannot be mechanically tested. The acceptance check is:
1. Gate definitions exist for all four delegation classes
2. Each gate specifies the exact tool call and pass/block criteria
3. Integration points with orchestrator template and delegation instructions are documented
4. Cost ceiling (≤5 turns, ≤10 tool calls) is stated as a hard requirement

## Relation to Benchmark

Benchmark ticket `10d21210` includes a scenario where a delegation's precondition fails post-dispatch. With pre-dispatch gates applied, the blocker will be caught BEFORE dispatch, eliminating the re-dispatch entirely. The gate cost (≤5 turns) is far cheaper than a full delegation loop (20-64 turns in the measured sessions).

## Schema Gaps Discovered

The ticket description references schema gaps discovered during rework chains:
- `SessionValidationGate` missing `command` field (see tickets `8c67b96a`, `0d3fdba6`)

These gaps belong in their owning ticket/spec scopes, not in this gate definition. If a gate discovers a new schema gap, record it in the relevant ticket rather than expanding this gate file.
