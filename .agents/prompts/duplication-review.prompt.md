---
description: "Run the full duplication review pipeline end to end: pairwise-compare the agent guidance corpus, classify findings, and produce the duplication report."
name: "duplication-review"
argument-hint: "[scope: directory, file subset, or omit for the full .agents/ tree plus AGENTS.md]"
agent: "Duplication Review Agent"
---

# Duplication Review Pipeline

Run a complete, end-to-end duplication review of the agent guidance corpus and hand back a finished report.

Follow [duplication-review.instructions.md](../instructions/orchestration/duplication-review.instructions.md) for the workspace layout, Scope Resolution, classification scheme, the per-pair comparison procedure, anchor-fixed batching, and phased dispatch.

## Workflow

1. Treat the slash-command text as the review scope; default to every file under `.agents/` plus the repository-root `AGENTS.md` when no scope is given, per Scope Resolution.
2. Create the dedicated review workspace folder, list every in-scope file, sort it, and build anchor-fixed batches (each with one fixed anchor file and a capped set of targets) per Anchor-Fixed Batching.
3. Group the batches into sequential phases of bounded width and dispatch each phase's batches in parallel, targeting Duplication Batch Worker Agent on a cheap T3 model (`GPT-5 mini` by default), waiting for the phase to fully return before starting the next one.
4. Merge every batch's returned marked-section findings into `pair-ledger.md` (pair verdicts) and `duplicate-passages.md` (findings) as each phase completes.
5. Run the synthesis phase yourself only after every pair has a verdict: cluster ideas recurring across three or more files and rank them.
6. Write `duplication-report.md` with the ranked ideas and every occurrence.
7. Do not edit, condense, or delete anything under `.agents/` or `AGENTS.md` — this pipeline only produces the report.

## Response

Return:
- workspace folder path and `duplication-report.md` link
- files compared, batch count, phase count, and total pairs evaluated
- counts by verdict
- top duplicated ideas table: idea, occurrence count, classification, linked occurrences
- handoff note pointing to Simplify Agent for any consolidation
