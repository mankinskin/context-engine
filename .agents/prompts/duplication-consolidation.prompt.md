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

1. Treat the slash-command text as the review workspace folder path; default to the most recently modified folder under `duplication-reviews/` when none is given.
2. Confirm the review is complete (`pair-ledger.md` has no unclassified pairs) before consolidating.
3. Group `exact duplicate`/`near-duplicate` findings into concepts, folding in below-threshold findings the report's clusters missed.
4. For each concept, pick the authoritative location by priority (`AGENTS.md` > `.instructions.md` > `SKILL.md`, never a template while one of those exists), compile its snippet, and compile every other occurrence's reference-only replacement.
5. Apply the full compiled changeset file by file, bottom-to-top within each file, only after every concept is compiled.
6. Defer every `thematic overlap` finding to Simplify Agent instead of consolidating it.
7. Do not commit — that stays with Commit Agent.

## Response

Return:
- review workspace folder consolidated
- per-concept table: concept, authoritative location, occurrences replaced, verdict
- edits applied and files touched
- deferred thematic-overlap candidates
- concepts skipped (single occurrence after grouping)
