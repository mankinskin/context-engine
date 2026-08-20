---
description: "Use when running or resuming a structured duplication review of the .agents/instructions/** guidance corpus: pairwise comparison, similarity classification, and the duplication report."
---

## Purpose

A duplication review finds every duplicated or semantically similar passage across an instruction corpus and reports it. It never rewrites, condenses, or deletes the corpus itself — that is Simplify Agent's job, using this review's report as input.

## Workspace

Create one dedicated review folder per run, sibling to `transcripts/`:

```
duplication-reviews/<DD-MM-YYYY>_<scope-slug>/
  pair-ledger.md          # one row per unordered file pair: classification + status
  duplicate-passages.md   # verbatim exact/near-duplicate passages, tagged with both sources
  duplication-report.md   # final deliverable
```

Never write review artifacts into `.agents/instructions/**` itself.

## Classification

Classify every compared pair as exactly one of:

- `exact duplicate` — near-identical text in both files.
- `near-duplicate` — the same statement, reworded or reordered.
- `thematic overlap` — the same underlying rule or idea, expressed with different wording, structure, or example.
- `no overlap` — no meaningful similarity.

## Coverage and Efficiency Rules

- Read each in-scope file's full content once and hold it for every subsequent comparison; never re-read the same file per pair.
- Seed `pair-ledger.md` with every unordered pair up front so coverage is verifiable and a review can resume mid-run.
- Compare same-directory pairs first (highest expected overlap), then cross-directory pairs — this ordering does not exempt any pair from the ledger.
- Every pair must reach a classification before the review is complete; an unclassified pair blocks completion rather than being silently skipped.
- Quote real text with real file paths and line ranges for every `exact duplicate` or `near-duplicate` finding; never paraphrase a duplicate into existence.

## Parallel Batch Dispatch

Pairwise comparison is local semantic matching — it does not need a strong model, so run it as a fan-out of cheap-tier subagents instead of doing it inline. Follow [model-routing.instructions.md](model-routing.instructions.md) for the ladder and the context-isolation contract; the orchestrating agent stays on its own model and never performs the pairwise comparisons itself.

1. **Partition** the in-scope file list into ordered batches `B1..Bk` (batch size chosen so every file's content plus the comparison set fits the dispatched model's context window — see the T3 sizing guidance in [model-routing.instructions.md](model-routing.instructions.md)).
2. **Assign each pair to exactly one batch** so no pair is skipped or double-covered: batch `Bi`'s subagent owns every pair between a file in `Bi` and a file in `Bi` itself or any later batch `Bj` (`j >= i`). It never re-covers a pair whose other file belongs to an earlier batch — that pair already belongs to the earlier batch.
3. **Dispatch one `runSubagent` call per batch**, in parallel, with `agentName: "Duplication Batch Worker Agent"` and `model` set to a T3 model (`GPT-5 mini` by default; step up to `GPT-5.6 Luna` only if the batch's combined file content does not fit 400k tokens). Each dispatch prompt must be fully self-contained per the context-isolation checklist: the full text (or exact repository-relative paths to read) of every file in `Bi` plus every file in batches `>= i`, the Classification categories above verbatim, and the exact required return shape: one row per pair with `file A | file B | classification | verbatim passage (if exact/near-duplicate) | line ranges`.
4. **Collect** every subagent's returned rows and merge them into `pair-ledger.md` and `duplicate-passages.md` — this merge is the orchestrator's job, never delegated to the batch worker.
5. If a batch worker returns an incomplete or malformed set of rows (fewer rows than its assigned pair count), re-dispatch only that batch once before escalating.

## Two-Phase Workflow

1. **Pairwise phase**: partition and dispatch parallel batch subagents per Parallel Batch Dispatch above, then merge their returned classifications and verbatim passages into `pair-ledger.md` and `duplicate-passages.md`.
2. **Synthesis phase** (only after every pair in the ledger is classified): re-evaluate `duplicate-passages.md` as a whole — cluster entries that express the same underlying idea across three or more files, simplify each cluster to one representative statement, and drop clusters that turn out to be coincidental phrasing rather than a real duplicated rule. Rank the surviving clusters by occurrence count into `duplication-report.md`, listing every occurrence (file + line range) per idea. Run the synthesis phase on the orchestrator's own model — it needs the cross-batch judgement a cheap model should not be trusted with.

## Reporting Contract

- Render every file reference as a clickable link with repository-relative path and line range per the Clickable Reference Policy in [AGENTS.md](../../AGENTS.md).
- Report counts by classification and confirm the pairs-evaluated count matches `pair-ledger.md`'s row count.
- Close with a handoff note pointing to Simplify Agent for any consolidation of the reported duplicates; do not apply consolidation in this workflow.
