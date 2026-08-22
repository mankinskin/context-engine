---
description: "Merge two or more completed prompt-ingestion dossiers under transcripts/ that turned out to be thematically related into one consolidated, fully standalone dossier. Archives the originals outside the merged dossier unchanged, then uses a duplication review to consolidate shared/overlapping content while keeping every unique passage — the result must be resolvable without ever opening a source."
name: "merge-dossiers"
argument-hint: "<two or more dossier folder paths or slugs to merge, and optionally the merged dossier's slug>"
agent: "agent"
---

# Merge Dossiers

Fold two or more independently-refined dossiers into one, when they turned out to cover the same theme or effort.

Follow [dossier-merge.instructions.md](../instructions/orchestration/dossier-merge.instructions.md) for every substantive rule — folder layout, the duplication-review-and-consolidate mechanics, and the constraints. This prompt only sequences the dispatch.

## Workflow

1. **Resolve the source dossiers** named in the argument text. Validate each per [dossier-merge.instructions.md's Required Procedure step 1](../instructions/orchestration/dossier-merge.instructions.md#required-procedure).
2. **Create the merged dossier folder**, then archive the sources outside it, per steps 2-3 of the same procedure.
3. **Run the duplication pass** scoped to the archived sources' key artifacts, per step 4.
4. **Consolidate** the merged `ARTIFACTS.md`, `ROADMAP.md`, and `README.md` so every source's content is fully absorbed and nothing references a source back, per step 5.
5. **Dry-run the merged roadmap** before reporting it ready, per step 6.

## Constraints

- Do not create a ticket, create or edit a spec, or change store/workflow state beyond the same ticket-creation exception `prompt-ingestion.instructions.md` allows during roadmap adjustment.
- Do not edit a source dossier's files in place — they move into the `transcripts/_merged-sources/` archive unchanged.
- The merged dossier must be completely resolvable standalone — no merged artifact may name, link, or otherwise reference a source dossier or the archive path.
- If fewer than two valid source dossiers are given, say so and stop rather than manufacturing a merge of one.

## Response

- source dossiers archived and their new `transcripts/_merged-sources/<name>/` paths
- merged dossier folder path
- duplication review workspace location and its shared-vs-unique verdicts
- concepts consolidated into the merged `ARTIFACTS.md`/`ROADMAP.md`/`README.md`
- merged `ROADMAP.md`'s outcome summary and waypoint count
- confirmation the merged dossier is standalone (no reference to any source dossier remains in its content)
- explicit reminder that no ticket/spec/implementation was created — that is the next, separate step
