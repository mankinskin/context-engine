---
description: "Run the full duplication review pipeline end to end: pairwise-compare the instruction corpus, classify findings, and produce the duplication report."
name: "review-duplication"
argument-hint: "[scope: directory, file subset, or omit for the full .agents/instructions/** corpus]"
agent: "Duplication Review Agent"
---

# Duplication Review Pipeline

Run a complete, end-to-end duplication review of the instruction corpus and hand back a finished report.

Follow [duplication-review.instructions.md](../instructions/orchestration/duplication-review.instructions.md) for the workspace layout, classification scheme, coverage rules, and two-phase workflow.

## Workflow

1. Treat the slash-command text as the review scope; default to the full `.agents/instructions/**` corpus when no scope is given.
2. Create the dedicated review workspace folder and seed `pair-ledger.md` with every unordered in-scope pair.
3. Run the pairwise phase to full coverage: classify every pair and collect verbatim `exact duplicate`/`near-duplicate` passages into `duplicate-passages.md`.
4. Run the synthesis phase only after every pair is classified: cluster ideas recurring across three or more files and rank them.
5. Write `duplication-report.md` with the ranked ideas and every occurrence.
6. Do not edit, condense, or delete anything under `.agents/instructions/**` — this pipeline only produces the report.

## Response

Return:
- workspace folder path and `duplication-report.md` link
- files compared and total pairs evaluated
- counts by classification
- top duplicated ideas table: idea, occurrence count, classification, linked occurrences
- handoff note pointing to Simplify Agent for any consolidation
