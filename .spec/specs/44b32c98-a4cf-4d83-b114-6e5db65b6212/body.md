<!-- aligned-structure:v2 -->

# Production Workflow: Implementation

## Target Code Location

[.agents/instructions/orchestration/escalation-gate.instructions.md](.agents/instructions/orchestration/escalation-gate.instructions.md) owns deficient-handoff escalation; [AGENTS.md](AGENTS.md) owns validation, logs, and commit requirements.

## Naming Conventions

Use `target/test-logs/` for complete failure logs and `.test/` executions for
validation records. This component owns `implementation-planned-scope`, `implementation-review-evidence`, and `implementation-escalation`.

## Requester Input

> Work every ticket created under Waypoint 6 through the standard ticket lifecycle to `done`.

## Reading Order

1. [.agents/instructions/orchestration/escalation-gate.instructions.md](.agents/instructions/orchestration/escalation-gate.instructions.md) — incomplete-handoff owner.
2. [18c1b04d Production Workflow: Tests](.spec/specs/18c1b04d-d23f-4047-9793-5c2af0ee04c1/body.md) — validation provider.
3. [0013fe78 Production Workflow: Validated Response](.spec/specs/0013fe78-9279-4bb2-8707-e86b6a3dd3b8/body.md) — review-evidence consumer.

## Responsibility

If implemented, Validated Response can rely on an approved-scope change with
its changed paths, documentation, validation verdict, and escalations exposed.

## Interfaces And Dependencies

Consumes all `tests-*` criteria and a complete ticket/spec handoff. Produces
review material and test evidence for the response stage.

## Behavior

- `implementation-planned-scope` changes only approved scope and non-goals.
- `implementation-review-evidence` reports paths, documentation, and validation outcome.
- `implementation-escalation` stops when criteria, owners, or context are missing.

## Boundaries And Failure Cases

An incomplete or ambiguous handoff escalates; it does not trigger invented requirements.
Required failing checks cannot be called complete, and unrelated failures do not expand scope.

## Provider/Consumer Contract

Consumes all `tests-*` criteria from [18c1b04d Production Workflow: Tests](.spec/specs/18c1b04d-d23f-4047-9793-5c2af0ee04c1/body.md); provides all three `implementation-*` criteria to [0013fe78 Production Workflow: Validated Response](.spec/specs/0013fe78-9279-4bb2-8707-e86b6a3dd3b8/body.md).

## Examples

After a planned change, inspect `target/test-logs/`, record the command result in
`.test/`, and provide changed paths plus any remaining limitation to review.

## Evidence

Position: `implemented` governing guidance. Required ticket commands and recorded
outcomes are evidence; this draft has no associated implementation run.

## Scope

Owns execution of approved work, not request clarification or user judgment.
