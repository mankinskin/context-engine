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

## Two-Phase Workflow

1. **Pairwise phase**: work through `pair-ledger.md`, classifying each pair and appending verbatim passages for `exact duplicate`/`near-duplicate` findings to `duplicate-passages.md`.
2. **Synthesis phase** (only after every pair in the ledger is classified): re-evaluate `duplicate-passages.md` as a whole — cluster entries that express the same underlying idea across three or more files, simplify each cluster to one representative statement, and drop clusters that turn out to be coincidental phrasing rather than a real duplicated rule. Rank the surviving clusters by occurrence count into `duplication-report.md`, listing every occurrence (file + line range) per idea.

## Reporting Contract

- Render every file reference as a clickable link with repository-relative path and line range per the Clickable Reference Policy in [AGENTS.md](../../AGENTS.md).
- Report counts by classification and confirm the pairs-evaluated count matches `pair-ledger.md`'s row count.
- Close with a handoff note pointing to Simplify Agent for any consolidation of the reported duplicates; do not apply consolidation in this workflow.
