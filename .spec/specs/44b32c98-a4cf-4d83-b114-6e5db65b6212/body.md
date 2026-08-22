<!-- aligned-structure:v2 -->

# Implementation

## Responsibility And Interface

Execute one planned ticket with its validation approach and produce review-ready
code, documentation, and evidence. Consume Tests' three criteria and complete
handoff; use `.test/` for records and `target/test-logs/` for full failure logs;
provide the three criteria consumed by Validated response.

## Behavior And Contract

- `implementation-planned-scope`: changes only approved scope and non-goals.
- `implementation-review-evidence`: exposes changed paths, docs, and validation verdict.
- `implementation-escalation`: stops when handoff lacks criteria, owning paths, or context.

## Boundaries And Failure Cases

`.agents/instructions/orchestration/phase-separation.instructions.md` forbids
broad search or inline clarification during execution. Incomplete/ambiguous
handoffs escalate via `escalation-gate.instructions.md`; required failing tests
cannot be reported as complete, and unrelated failures cannot expand scope.

## Acceptance Evidence And Position

Run required ticket commands, inspect `target/test-logs/`, and record applicable
outcomes before review. Verify ticket spec/doc/evidence pointers. This draft has
no run and no `validated_by`; phase separation and escalation are implemented rules.
