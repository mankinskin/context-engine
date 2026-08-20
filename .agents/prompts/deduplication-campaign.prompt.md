---
description: "Run a full multi-slice deduplication campaign over a large guidance corpus: repeatedly review and consolidate one anchor slice at a time until every pair in scope is covered."
name: "deduplication-campaign"
argument-hint: "[scope: directory, file subset, or omit for the full .agents/ tree plus AGENTS.md]"
agent: "Deduplication Campaign Agent"
---

# Deduplication Campaign

Drive a large-corpus deduplication campaign to completion, one anchor slice at a time.

This pipeline only loops [Duplication Cleanup Agent](../agents/duplication-cleanup.agent.md) — see [duplication-review.instructions.md](../instructions/orchestration/duplication-review.instructions.md#anchor-subset-scope-for-large-corpora), [duplication-consolidation.instructions.md](../instructions/orchestration/duplication-consolidation.instructions.md), and [duplication-cleanup.agent.md](../agents/duplication-cleanup.agent.md) for every substantive rule; nothing here repeats them.

## Workflow

1. Treat the slash-command text as the comparison scope; default to the full `.agents/` tree plus `AGENTS.md` when none is given.
2. Check `campaign-coverage.md` for this scope; if it already shows the campaign complete, report that directly without dispatching anything.
3. Otherwise, dispatch Duplication Cleanup Agent for one anchor slice, wait for its full return, and re-check the ledger before dispatching the next slice — sequentially, never in parallel.
4. Stop after the campaign closes or after this run's `SLICES_PER_INVOCATION` cap is reached, whichever comes first.
5. Report every slice dispatched this run together as one combined campaign summary.

## Response

Return:
- comparison scope and campaign folder
- per-slice table (anchor range, workspace folder, verdict counts, concepts consolidated, files touched)
- campaign totals: pairs resolved / pairs remaining
- whether the campaign is fully complete, or an instruction to re-invoke `/deduplication-campaign` with the same scope to continue
- reminder that committing is Commit Agent's job
