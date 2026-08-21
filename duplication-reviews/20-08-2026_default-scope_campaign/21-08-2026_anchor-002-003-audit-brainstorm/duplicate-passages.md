# Duplicate Passages — anchor slice [2,3] (`audit.agent.md`, `brainstorm.agent.md`)

Per [Coverage and Efficiency Rules](../../../.agents/instructions/orchestration/duplication-review.instructions.md#coverage-and-efficiency-rules), every `exact duplicate` and `near-duplicate` finding is quoted below in full. The 205 `thematic overlap` and 48 `no overlap` findings (one per pair, largely the anchor-template sections — frontmatter, persona line, Scope/Constraints/Required Workflow/Output Format headings — recurring against each target's equivalent sections) are not individually transcribed here for size; their per-pair verdicts are recorded in [pair-ledger.md](pair-ledger.md) and their marked line ranges are preserved in the raw batch-worker returns this run consumed. This omission does not affect verdict accuracy since verdict = most severe classification, already captured per pair.

## Exact duplicates (5 findings, all anchor `F_3` = `brainstorm.agent.md`)

| # | Anchor | Target | Anchor lines | Target lines | Excerpt |
|---|---|---|---|---|---|
| 1 | brainstorm.agent.md | agents/teacher.agent.md | 6-7 | 6-7 | `user-invocable: true` |
| 2 | brainstorm.agent.md | agents/testing.agent.md | 6 | 6 | `user-invocable: true` |
| 3 | brainstorm.agent.md | agents/ticket-refinement.agent.md | 6 | 6 | `user-invocable: true` |
| 4 | brainstorm.agent.md | agents/transcription.agent.md | 6 | 6 | `user-invocable: true` |
| 5 | brainstorm.agent.md | agents/writing.agent.md | 6 | 6 | `user-invocable: true` |

**Caveat:** this is a single shared YAML frontmatter boolean field (`user-invocable: true`), not a duplicated prose rule — every `.agent.md` template carries this field independently. Flagged here per the worker's literal-match instruction, but excluded from the synthesis phase as non-substantive (see [duplication-report.md](duplication-report.md)).

## Near-duplicates (21 findings)

| # | Anchor | Target | Anchor lines | Target lines | Excerpt |
|---|---|---|---|---|---|
| 1 | audit.agent.md | instructions/orchestration/model-routing.instructions.md | 2 | 126 | "Use for honest repository audits, findings-first reviews, and automated validation triage." |
| 2 | audit.agent.md | instructions/orchestration/question-quality.instructions.md | 24-29 | 41-44 | "Read the affected code and nearby tests directly; do not rely only on summaries." |
| 3 | audit.agent.md | prompts/audit.prompt.md | 26 | 29 | "findings first, ordered by severity" |
| 4 | audit.agent.md | prompts/audit.prompt.md | 33-36 | 16-22 | "1. Treat the slash-command text as an optional audit scope." |
| 5 | audit.agent.md | prompts/audit.prompt.md | 40-45 | 26-31 | "Return: - scope audited - tools and validation used - findings first, ordered by severity - residual risks or coverage gaps - recommended follow-up work, if any" |
| 6 | audit.agent.md | prompts/reviews.prompt.md | 27 | 57 | "Read the affected code and nearby tests directly; do not rely only on summaries." |
| 7 | brainstorm.agent.md | agents/mission-planning.agent.md | 32-44 | 21-26 | "Do not edit code, create tickets, update specifications, or choose a final direction for the user." |
| 8 | brainstorm.agent.md | agents/online-research.agent.md | 32-44 | 30-34 | "Do not edit code, create tickets, update specifications, or choose a final direction for the user." |
| 9 | brainstorm.agent.md | agents/orchestrator.agent.md | 32-44 | 37-44 | "Do not edit code, create tickets, update specifications, or choose a final direction for the user." |
| 10 | brainstorm.agent.md | agents/roast.agent.md | 32-44 | 32-43 | "Do not edit code, create tickets, update specifications, or choose a final direction for the user." |
| 11 | brainstorm.agent.md | instructions/engine/workflow-tool-extraction.instructions.md | 42-44 | 13 | "When existing tickets or specifications constrain a direction, name the constraint and explain whether the direction extends, conflicts with, or is orthogonal to the recorded work." |
| 12 | brainstorm.agent.md | instructions/orchestration/model-prices.instructions.md | 40-41 | 12 | "Do not edit code, create tickets, update specifications, or choose a final direction for the user." |
| 13 | brainstorm.agent.md | instructions/orchestration/model-routing.instructions.md | 7 | 133 | `model: "Claude Sonnet 5"` |
| 14 | brainstorm.agent.md | prompts/audit.prompt.md | 49-50 | 17 | "Inspect only the closest relevant tickets, specifications, and repository surfaces needed to avoid contradicted or duplicate ideas." |
| 15 | brainstorm.agent.md | prompts/debug-test.prompt.md | 48-50 | 19-21 | "Inspect only the closest relevant tickets, specifications, and repository surfaces needed to avoid contradicted or duplicate ideas." |
| 16 | brainstorm.agent.md | prompts/interview.prompt.md | 48-59 | 16-29 | "Inspect only the closest relevant tickets, specifications, and repository surfaces needed to avoid contradicted or duplicate ideas." |
| 17 | brainstorm.agent.md | prompts/research.prompt.md | 48-59 | 15-24 | "Inspect only the closest relevant tickets, specifications, and repository surfaces needed to avoid contradicted or duplicate ideas." |
| 18 | brainstorm.agent.md | prompts/reviews.prompt.md | 48-59 | 48-66 | "Inspect only the closest relevant tickets, specifications, and repository surfaces needed to avoid contradicted or duplicate ideas." |
| 19 | brainstorm.agent.md | prompts/user-training.prompt.md | 49-50 | 18 | "Inspect only the closest relevant tickets, specifications, and repository surfaces needed to avoid contradicted or duplicate ideas." |
| 20 | brainstorm.agent.md | prompts/transform-transcript.prompt.md | 40-41 | 57 | "Do not edit code, create tickets, update specifications, or choose a final direction for the user." |
| 21 | brainstorm.agent.md | prompts/iteration.prompt.md | 16-20 / 32-44 | 16-26 / 26 | "Accept a challenge or opportunity, the current context, constraints, target audience, and a requested breadth." / "Do not edit code, create tickets, update specifications, or choose a final direction for the user." |

Additional near-duplicate occurrences of the same "Inspect only the closest relevant tickets…duplicate ideas" sentence and the "Do not edit code, create tickets…" sentence were reported for `prompts/memory-setup.prompt.md` (37/57), `prompts/refine-ingest.prompt.md` (37/61), `prompts/research.prompt.md` (61-69/34-41, a related but distinct clause), and `prompts/model-prices.instructions.md`/`model-routing.instructions.md` — see clustering in [duplication-report.md](duplication-report.md).
