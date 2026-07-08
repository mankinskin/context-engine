<!-- aligned-structure:v1 -->

# Summary

Add generated agent guidance for spec-system work so spec creation and updates consistently follow a clear workflow.

## Behavior Story

Add generated agent guidance for spec-system work so spec creation and updates consistently follow a clear workflow.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# Summary
Add generated agent guidance for spec-system work so spec creation and updates consistently follow a clear workflow.

## Acceptance Criteria
- A generated `.agents/instructions/spec-system.instructions.md` file exists with guidance for creating and updating specs.
- A generated `.agents/agents/spec.agent.md` file exists with a workflow for spec authoring and traceability.
- The guidance explicitly covers linking tests, tickets, and related specs.
- Rule targets and canonical rule entries generate the files successfully.
- Target generation/check completes without drift.
