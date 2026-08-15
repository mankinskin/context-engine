# Explainer Agent: Review and Implementation Plan

## Source and Status

This plan derives from [input.clean.md](input.clean.md). The source describes a
human-in-the-loop agent that explains a task before performing the task, lets a
human evaluate the result, and uses the feedback to improve future explanations
and identify frequent interactions.

**Review verdict: not implementation-ready yet.** The product direction is
clear, but the source does not define the execution boundary, the meaning of
"learn", the feedback store, or measurable success criteria. The decisions in
the [Open Decisions](#open-decisions) section must be resolved before creating
an implementation ticket or specification.

## Confirmed Requirements

The Explainer Agent must:

1. Explain the intended task and approach before performing any mutating work.
2. Keep a human in the control loop, with an explicit opportunity to approve,
   reject, or change the proposed approach.
3. Let the human directly evaluate the quality of both the system behavior and
   the explanation.
4. Capture feedback so later Explainer Agent runs can improve the explanation
   quality and reveal commonly used controls or interaction patterns.
5. Preserve the human's authority to stop work before an action is executed.

## Scope and Non-Goals

### In Scope

- A new agent template at `.agents/agents/explainer.agent.md`.
- A repeatable explain -> approve -> execute -> evaluate workflow.
- Durable feedback capture linked to the task, ticket, or specification when an
  entity exists.
- A reviewable summary of what was proposed, approved, executed, and evaluated.
- Aggregated interaction evidence that can inform future template revisions.

### Out of Scope for the First Version

- Training or fine-tuning a foundation model from user feedback.
- Silent execution after an explanation without an explicit approval.
- Autonomous changes to the Explainer Agent's permissions, instructions, or
  model routing.
- Inferring sensitive user preferences from unrelated conversation data.
- Replacing the specialized Implement, Interview, Review, or Testing Agent
  templates.

## Product Contract

### Workflow

1. **Receive a task.** The Explainer Agent establishes the goal, relevant
   repository entities, and allowed scope.
2. **Explain.** Before any mutation, the Explainer Agent presents a concise
   explanation containing the goal, proposed steps, files or entities expected
   to change, risks, validation, and assumptions.
3. **Obtain a human decision.** The human chooses one of the following:
   `approve`, `revise`, `narrow`, `decline`, or `delegate`.
4. **Execute only after approval.** The Explainer Agent performs the approved
   scope, announces material deviations, and returns to approval when a change
   would exceed the approved plan.
5. **Evaluate.** The human rates the explanation and result, and can submit a
   concise free-text finding.
6. **Record evidence.** The Explainer Agent records the approved plan, execution
   evidence, feedback, and interaction category in the repository's durable
   stores.
7. **Improve future explanations.** A later template-maintenance process uses
   aggregated feedback to update instructions. A run must never rewrite its own
   template or behavior automatically.

### Explanation Requirements

Every pre-execution explanation must state:

- the requested outcome in plain language;
- known constraints and assumptions;
- the proposed sequence of actions;
- the intended files, tickets, specifications, or services;
- the validation method and the expected evidence;
- risks, non-goals, and the human decision required before execution.

The explanation must distinguish facts found in the repository from inferences
and recommendations. The explanation must not claim that a command, test, or
change succeeded before evidence confirms success.

### Human Control Requirements

- No mutating tool call may run before an explicit `approve` decision covering
  the described scope.
- A `revise` or `narrow` decision produces a new explanation and requires a new
  approval.
- A `decline` decision ends the execution path without mutation.
- A material deviation from the approved plan pauses execution and asks for a
  new decision.
- The recorded evaluation must identify the specific run and, where applicable,
  its ticket or specification anchor.

### Learning Requirements

For version one, "learn" means **collecting and reviewing durable feedback**,
not updating model weights or modifying agent instructions during a run.

Each evaluated run should capture:

- explanation rating;
- execution-result rating;
- free-text feedback, when supplied;
- interaction category, such as `approve`, `revise`, `narrow`, `decline`, or
  `delegate`;
- task type and the linked ticket/specification identifiers, when available.

Periodic analysis may propose template changes from the recorded evidence. A
human must review and approve every such template change through the ordinary
ticket, specification, review, and commit workflow.

## Proposed Template Design

Create `.agents/agents/explainer.agent.md` using the repository's existing agent
template format: YAML frontmatter followed by a bounded role contract.

### Frontmatter Requirements

- `name`: `Explainer Agent`
- `description`: states the explain-before-execute and human-approval contract.
- `argument-hint`: requests the task plus an optional existing ticket or spec.
- `user-invocable`: `true`
- `model`: select a model from the canonical routing ladder after the execution
  scope is decided.
- `tools`: include only the tools needed for the approved execution model. The
  final list must not grant mutation tools until the approval gate has been
  designed and tested.

### Required Contract Sections

1. **Purpose**: turn a task into an understandable, human-approved execution
   unit.
2. **Scope**: state which task classes the template may execute and which tasks
   must be delegated to existing specialized agents.
3. **Pre-execution explanation**: require the explanation fields listed above.
4. **Approval gate**: require a `vscode/askQuestions` decision before every
   mutation; list the exact legal decisions and their effects.
5. **Execution boundary**: prohibit scope expansion and require renewed
   approval for material deviations.
6. **Evaluation and evidence**: require a post-execution rating and feedback
   capture through the repository feedback store.
7. **Learning boundary**: prohibit autonomous self-modification; permit only
   evidence-backed recommendations for a later human-reviewed template update.
8. **Return contract**: require a concise result containing the approved plan,
   execution evidence, evaluation, recorded feedback reference, and remaining
   risks.

### Template Skeleton

```markdown
---
name: "Explainer Agent"
description: "Explain a bounded task, obtain human approval, execute only the approved scope, and record evaluation evidence."
tools: [TBD after approval-gate design]
argument-hint: "Task to explain and execute, with an optional ticket or spec anchor."
user-invocable: true
model: "Claude Sonnet 5"
---

You are the Explainer Agent. Turn a task into a clear, human-approved execution
unit. Never perform a mutating action until the human has explicitly approved
the presented scope.

## Pre-execution Explanation

Present the outcome, constraints, assumptions, proposed actions, affected
entities, validation, risks, non-goals, and the specific approval requested.

## Approval Gate

Ask the human to approve, revise, narrow, decline, or delegate. Record the
decision. Only `approve` permits execution within the stated boundary.

## Execution and Re-approval

Execute only the approved scope. Stop and request new approval before a
material deviation or an additional mutation.

## Evaluation and Evidence

Ask the human to rate the explanation and execution result. Record feedback
with the run's task and ticket/spec anchor. Do not change this template or its
permissions from feedback during a run.
```

The model value above is a provisional default. The final model and tools must
follow the canonical model-routing and tool-grant rules once the execution
boundary is decided.

## Implementation Plan

### Phase 0: Resolve Product Decisions

1. Define the first-version task class: read-only research, a narrow code edit,
   or delegated execution through existing agents.
2. Define whether one approval covers one command, one plan, or one ticket
   slice.
3. Define the allowed feedback audience and retention policy.
4. Define success thresholds for explanation quality and human control.

**Exit criterion:** every entry in [Open Decisions](#open-decisions) has an
owner and a recorded answer, or is explicitly deferred from version one.

### Phase 1: Establish Durable Product Scope

1. Search existing ticket and specification stores for overlapping human-loop,
   feedback, agent-template, and execution-gating work.
2. Create or update one tracking ticket and a linked specification.
3. Record the approved contract, acceptance criteria, privacy boundary, and
   validation plan.

**Exit criterion:** the tracking ticket is implementation-ready and the
specification defines the complete version-one behavior.

### Phase 2: Author the Agent Template

1. Add `.agents/agents/explainer.agent.md` from the approved template design.
2. Select the narrowest tool grant that can implement the approved task class.
3. Include an explicit approval gate before every mutation.
4. Define the expected result and evidence record formats.
5. Regenerate repository-managed agent artifacts if the template is mirrored.

**Exit criterion:** the template follows the repository's frontmatter and role
contract conventions and has no capability beyond the approved scope.

### Phase 3: Add Feedback and Evidence Wiring

1. Select the canonical feedback target format for agent-run evaluations.
2. Record explanation and execution ratings separately.
3. Capture the interaction category and linked entity identifiers.
4. Provide a query or report that groups feedback by task type and interaction
   category without exposing sensitive content unnecessarily.

**Exit criterion:** a completed trial run can be read back from durable storage
and connected to its task or entity anchor.

### Phase 4: Validate Human Control

1. Test that a declined task produces no mutation.
2. Test that `revise` and `narrow` require a new explanation and approval.
3. Test that an unplanned mutation pauses for re-approval.
4. Test that feedback is recorded and can be read back.
5. Run the narrowest existing agent-template, store, and integration checks.

**Exit criterion:** every acceptance criterion below has executable evidence or
a documented manual validation result.

### Phase 5: Pilot and Review

1. Run a small set of representative tasks with a human reviewer.
2. Collect explanation and result evaluations for every pilot run.
3. Review recurring revisions, declines, and feedback themes.
4. Propose only evidence-backed changes to the template through the normal
   review workflow.

**Exit criterion:** the human reviewer approves the pilot outcome and accepts
the template's boundaries.

## Acceptance Criteria

1. The Explainer Agent presents a complete pre-execution explanation before any
   mutating action.
2. A human can approve, revise, narrow, decline, or delegate the proposed task.
3. No mutation occurs after a decline or before approval.
4. A material execution deviation blocks further mutation until renewed human
   approval is recorded.
5. Every completed run records explanation feedback, execution feedback, and
   the interaction category in durable storage.
6. Recorded feedback can be retrieved for the run and linked to a ticket or
   specification when an anchor exists.
7. The Explainer Agent does not modify its own template, permissions, or routing
   based on runtime feedback.
8. A template-maintenance review can identify frequent interaction patterns
   from aggregated evidence without treating those patterns as automatic policy
   changes.

## Validation Matrix

| Criterion | Validation |
| --- | --- |
| Explain before mutation | Capture the transcript and assert the approval question precedes the first mutating tool call. |
| Approval boundary | Run approve, revise, narrow, decline, and delegate scenarios. |
| Re-approval | Introduce a planned scope change and assert that execution pauses. |
| Feedback persistence | Write ratings and notes, then read the stored feedback back by target. |
| Anchor linkage | Verify a ticket/specification-linked run resolves the same identifiers on read-back. |
| No self-modification | Review the template contract and test that feedback cannot edit the template in the same run. |
| Pattern analysis | Query multiple trial records and confirm the report groups interaction categories accurately. |

## Open Decisions

| Decision | Options | Recommended default | Why the decision matters |
| --- | --- | --- | --- |
| Version-one task scope | Read-only; narrow direct edits; delegated execution | Delegated execution | Reuses specialized agents and limits new permissions. |
| Approval granularity | Per command; per bounded plan; per ticket | Per bounded plan | Keeps interaction usable while retaining a clear scope boundary. |
| Definition of material deviation | New file; new entity; new command class; changed outcome | Any new mutation target or changed outcome | Produces an objective re-approval rule. |
| Meaning of learning | Runtime self-modification; feedback analysis; model training | Feedback analysis only | Allows improvement without uncontrolled behavior changes. |
| Feedback visibility | Private to run; team-visible; anonymized aggregate | Private run feedback plus approved aggregates | Balances traceability with privacy. |
| Pilot success threshold | Qualitative review; rating threshold; task-completion rate | Set before pilot | Prevents interpreting feedback after the fact. |

## Review Summary

The transcript establishes a valuable interaction model: explanation is part of
the product, not merely narration before execution. The critical design rule is
that feedback produces evidence for a later, human-reviewed improvement cycle;
feedback must not grant the Explainer Agent authority to change its own behavior
or execute beyond the approved plan.