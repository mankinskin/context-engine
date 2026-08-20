---
description: "Run the full duplication consolidation pipeline: group a completed duplication review's findings into concepts, compile authoritative snippets and reference replacements, and mechanically apply the changeset."
name: "duplication-consolidation"
argument-hint: "[path to a completed duplication-reviews/<...>/ folder, or omit for the most recent]"
agent: "Duplication Consolidation Agent"
---

# Duplication Consolidation Pipeline

Consolidate a completed duplication review's findings into single authoritative sources, with every other occurrence replaced by a reference, and apply the changeset.

Follow [duplication-consolidation.instructions.md](../instructions/orchestration/duplication-consolidation.instructions.md) for candidate selection, concept grouping, authoritative-location priority, and mechanical execution.

## Workflow

1. Treat the slash-command text as the review workspace folder path, resolved per [Input Contract](../instructions/orchestration/duplication-consolidation.instructions.md#input-contract).
2. Confirm the review is complete (`pair-ledger.md` has no unclassified pairs) before consolidating.
3. Group `exact duplicate`/`near-duplicate` findings into concepts, folding in below-threshold findings the report's clusters missed.
4. For each concept, select the authoritative location and compile its snippet plus every replacement, per [Authoritative Location Selection](../instructions/orchestration/duplication-consolidation.instructions.md#authoritative-location-selection) and [Snippet/Replacement Compilation](../instructions/orchestration/duplication-consolidation.instructions.md#snippet-compilation).
5. Apply the full compiled changeset per [Mechanical Execution](../instructions/orchestration/duplication-consolidation.instructions.md#mechanical-execution).
6. Defer every `thematic overlap` finding to Simplify Agent instead of consolidating it.
7. Do not commit — that stays with Commit Agent.

## Response

Return:
- review workspace folder consolidated
- per-concept table: concept, authoritative location, occurrences replaced, verdict
- edits applied and files touched
- deferred thematic-overlap candidates
- concepts skipped (single occurrence after grouping)
