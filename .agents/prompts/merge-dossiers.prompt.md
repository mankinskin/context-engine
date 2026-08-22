---
description: "Merge two or more completed prompt-ingestion dossiers under transcripts/ that turned out to be thematically related into one consolidated dossier. Moves the originals into the merged dossier's sources/ folder unchanged, then uses a duplication review to consolidate shared/overlapping content while keeping every unique passage."
name: "merge-dossiers"
argument-hint: "<two or more dossier folder paths or slugs to merge, and optionally the merged dossier's slug>"
agent: "agent"
---

# Merge Dossiers

Fold two or more independently-refined dossiers into one, when they turned out to cover the same theme or effort.

Follow [dossier-merge.instructions.md](../instructions/orchestration/dossier-merge.instructions.md) for every substantive rule — folder layout, the duplication-review-and-consolidate mechanics, and the constraints. This prompt only sequences the dispatch.

## Workflow

1. **Resolve the source dossiers** named in the argument text. Validate each per [dossier-merge.instructions.md's Required Procedure step 1](../instructions/orchestration/dossier-merge.instructions.md#required-procedure).
2. **Create the merged dossier folder** and relocate the sources into it, per steps 2-3 of the same procedure.
3. **Run the duplication pass** scoped to the moved sources' key artifacts, per step 4.
4. **Consolidate** the merged `ARTIFACTS.md`, `ROADMAP.md`, and `README.md`, per step 5.
5. **Dry-run the merged roadmap** before reporting it ready, per step 6.

## Constraints

- Do not create a ticket, create or edit a spec, or change store/workflow state beyond the same ticket-creation exception `prompt-ingestion.instructions.md` allows during roadmap adjustment.
- Do not edit a source dossier's files in place — they move into `sources/` unchanged.
- If fewer than two valid source dossiers are given, say so and stop rather than manufacturing a merge of one.

## Response

- source dossiers moved and their new `sources/<name>/` paths
- merged dossier folder path
- duplication review workspace location and its shared-vs-unique verdicts
- concepts consolidated into the merged `ARTIFACTS.md`/`ROADMAP.md`/`README.md`
- merged `ROADMAP.md`'s outcome summary and waypoint count
- explicit reminder that no ticket/spec/implementation was created — that is the next, separate step
