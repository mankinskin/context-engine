---
description: "Use when merging two or more completed prompt-ingestion dossiers (transcripts/DD-MM-YYYY_<slug>/ folders) that turned out to be thematically related into one consolidated dossier."
applyTo: "**/*.md"
---

## Purpose

A prompt-ingestion dossier (see [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md)) is refined independently per raw prompt. Sometimes several already-refined dossiers turn out to cover the same theme or effort after the fact — refined from separate transcripts, but pointing at one session's worth of work. This file governs folding them into a single dossier without losing any source or silently duplicating shared content.

## When to Merge

Two or more dossiers under `transcripts/` are thematically related enough that a single `ROADMAP.md` and artifact set should govern them going forward, rather than tracking overlapping waypoints across separate folders.

## Required Procedure

1. **Resolve source dossiers.** Validate each named folder is a completed dossier — it has `README.md` and `ROADMAP.md`. Treat an incomplete dossier (mid-pipeline, no `ROADMAP.md` yet) as a blocker: finish [refine-ingest.prompt.md](../../prompts/refine-ingest.prompt.md) on it first, or exclude it.
2. **Create the merged dossier folder.** `transcripts/DD-MM-YYYY_<merged-slug>/`, dated today, named for the combined theme.
3. **Relocate originals unchanged.** `git mv` each source dossier folder in full into `sources/<original-folder-name>/` inside the merged folder. Never edit a source file in place — `sources/` is the immutable historical record the merge draws from, not a workspace to rewrite.
4. **Duplication pass.** Run [duplication-review.prompt.md](../../prompts/duplication-review.prompt.md) with its scope narrowed to the moved sources' `ARTIFACTS.md`/`ROADMAP.md`/`README.md`/work-package files — an explicit file subset is a valid narrowed scope per [duplication-review.instructions.md's Scope Resolution](duplication-review.instructions.md#scope-resolution). Its report is the shared-vs-unique map the merge draws from; do not consolidate from eyeballing the sources instead.
5. **Consolidate the merged artifacts**, applying [duplication-consolidation.instructions.md](duplication-consolidation.instructions.md)'s Concept Grouping, Authoritative Location, and Snippet Compilation mechanics (only the mechanics — this step is dossier-specific, not a corpus-wide consolidation run):
   - **`ARTIFACTS.md`**: union of rows across sources, deduplicated by artifact id/path.
   - **`ROADMAP.md`**: one outcome summary describing the combined effort (not a concatenation of the sources' summaries); the union of active blockers, validation gates, and heads-up notes with duplicates collapsed to one entry each; one ordered waypoint list combining every source's waypoints, collapsing an exact- or near-duplicate waypoint pair to a single entry and keeping both only when the review found genuine thematic overlap describing two distinct waypoints.
   - **`README.md`**: index pointing at the merged `ROADMAP.md` as the entry point, and at `sources/` for the full original per-dossier history.
6. **Dry-run the merged roadmap.** Apply [prompt-ingestion.instructions.md's Roadmap Improvement Loop](prompt-ingestion.instructions.md#roadmap-improvement-loop) to the merged `ROADMAP.md` before treating it as ready — merging can introduce the same ordering and dependency defects a single dossier's drafting pass can.
7. **Version later revisions.** If the merged dossier itself is revised further, follow [prompt-ingestion.instructions.md's versioned-supersession pattern](prompt-ingestion.instructions.md#roadmap-compilation-and-versioning) rather than overwriting the merge in place.

## Constraints

- Same read-only/ticket-creation boundary as [prompt-ingestion.instructions.md's Decision Boundary](prompt-ingestion.instructions.md#decision-boundary): the merge does not create or edit a spec, and ticket creation follows the same exception (a merged waypoint too large for one session becomes a ticket, not an inline block).
- Never overwrite or delete a source dossier's content — it is moved, not edited, and stays inspectable under `sources/`.
- A merged artifact must not restate a shared statement in more than one place; keep the authoritative-location-plus-reference pattern from [duplication-consolidation.instructions.md](duplication-consolidation.instructions.md) rather than letting the merge reintroduce the duplication the review just found.

## Reporting

Return: source dossiers moved and their new `sources/` paths, the merged dossier's folder path, the duplication review's workspace location, concepts consolidated (shared vs. unique), and the final merged artifact paths ending with `ROADMAP.md`.
