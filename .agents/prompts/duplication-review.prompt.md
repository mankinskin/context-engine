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

1. Treat the slash-command text as the review scope, resolved per [Scope Resolution](../instructions/orchestration/duplication-review.instructions.md#scope-resolution).
2. Create the dedicated review workspace folder, list every in-scope file, sort it, and build anchor-fixed batches (each with one fixed anchor file and a capped set of targets) per Anchor-Fixed Batching.
3. Run [Phased Dispatch](../instructions/orchestration/duplication-review.instructions.md#phased-dispatch): dispatch each phase's batches in parallel to Duplication Batch Worker Agent on a cheap T3 model (`GPT-5 mini` by default), waiting for the phase to fully return before starting the next one.
4. Merge each phase's returned findings into `pair-ledger.md` and `duplicate-passages.md` as it completes, per Phased Dispatch.
5. Run the [synthesis phase](../instructions/orchestration/duplication-review.instructions.md#two-phase-workflow) yourself only after every pair has a verdict.
6. Write `duplication-report.md` with the ranked ideas and every occurrence.
7. Do not edit, condense, or delete anything under `.agents/` or `AGENTS.md` (per [duplication-review.instructions.md Purpose](../instructions/orchestration/duplication-review.instructions.md#purpose)) — this pipeline only produces the report.

## Response

Follow the [Reporting Contract](../instructions/orchestration/duplication-review.instructions.md#reporting-contract): return the workspace folder and report link, files/batch/phase/pair counts, counts by verdict, the top duplicated ideas table, and a handoff note to Simplify Agent.
