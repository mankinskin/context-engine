<!-- aligned-structure:v2 -->

# Request

## Responsibility And Interface

Turn a user ask or raw transcript into a request/dossier that records the
intended outcome, constraints, and unresolved questions. Inputs are chat,
transcripts, or Next iteration follow-up; the output is handed to Specification,
not silently converted into a ticket or implementation plan.

## Behavior And Contract

- `request-outcome`: records the desired observable result, not an inferred fix.
- `request-open-questions`: lists unresolved decisions or explicitly states none.
- Specification consumes those two criteria without copying them.

## Boundaries And Failure Cases

Request capture may research and interview but cannot claim durable acceptance
criteria or executable slices. Ambiguous outcome, scope, or decisions remain
open for discovery rather than guessed. An Implement Agent that receives this
incomplete handoff escalates under
`.agents/instructions/orchestration/escalation-gate.instructions.md`.

## Acceptance Evidence And Position

Review the dossier for an outcome plus questions/`none`, then inspect the
Specification child with `spec.exe --workspace . get d6b4d989-f9ac-428a-9dbc-68400006fc96 --json`.
No executable `validated_by` exists. `core-cycle.instructions.md` defines the
request/dossier handoff; `spec-system.instructions.md` is the governing rule.
