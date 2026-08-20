---
description: "Run the full duplication cleanup pipeline end to end on a scope: review it for duplicates, then consolidate the findings into authoritative snippets with reference-only replacements."
name: "duplication-cleanup"
argument-hint: "[scope: directory, file subset, or omit for the full .agents/ tree plus AGENTS.md]"
agent: "Duplication Cleanup Agent"
---

# Duplication Cleanup Pipeline

Run the complete end-to-end duplication cleanup on a scope: review it for duplicates, then consolidate the findings.

This pipeline only sequences [Duplication Review Agent](../agents/duplication-review.agent.md) and [Duplication Consolidation Agent](../agents/duplication-consolidation.agent.md) — see [duplication-review.instructions.md](../instructions/orchestration/duplication-review.instructions.md) and [duplication-consolidation.instructions.md](../instructions/orchestration/duplication-consolidation.instructions.md) for every substantive rule; nothing here repeats them.

## Workflow

1. Treat the slash-command text as the review scope; default per Duplication Review Agent's own Scope Resolution when none is given.
2. Run the review stage to completion.
3. Run the consolidation stage against exactly the review stage's resulting workspace folder.
4. Report both stages together.

## Response

Return:
- review stage summary (scope, workspace folder, counts)
- consolidation stage summary (concepts, authoritative locations, edits applied, deferred candidates)
- reminder that committing is Commit Agent's job
