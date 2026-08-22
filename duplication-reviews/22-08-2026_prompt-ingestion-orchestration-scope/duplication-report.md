# Duplication Report — prompt-ingestion / orchestration fixed 7-file scope (22-08-2026)

## Scope

Fixed 7-file comparison scope (no other `.agents/` or [AGENTS.md](../../AGENTS.md) files reviewed):

- F1: [.agents/agents/mission-planning.agent.md](../../.agents/agents/mission-planning.agent.md)
- F2: [.agents/agents/orchestrator.agent.md](../../.agents/agents/orchestrator.agent.md)
- F3: [.agents/instructions/orchestration/intent-refinement.instructions.md](../../.agents/instructions/orchestration/intent-refinement.instructions.md)
- F4: [.agents/instructions/orchestration/prompt-ingestion.instructions.md](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md)
- F5: [.agents/instructions/orchestration/roadmap-execution.instructions.md](../../.agents/instructions/orchestration/roadmap-execution.instructions.md)
- F6: [.agents/prompts/execute-roadmap.prompt.md](../../.agents/prompts/execute-roadmap.prompt.md)
- F7: [.agents/prompts/refine-ingest.prompt.md](../../.agents/prompts/refine-ingest.prompt.md)

Pairs evaluated: 21 of 21 (7×6/2), all closed — see [pair-ledger.md](pair-ledger.md).

## Verdict counts

| Verdict | Count |
|---|---|
| exact duplicate | 0 |
| near-duplicate | 8 |
| thematic overlap | 10 |
| legitimate restatement (excluded from duplicate ranking by caller's rule) | 3 |
| **Total** | **21** |

The 3 `legitimate restatement` pairs (F3↔F7, F4↔F7, F5↔F6) are prompt-file-vs-its-own-governing-instructions-file pairs, explicitly carved out by the caller as intentional dispatch-sequencer restatement, not duplication.

## Top duplicated ideas (synthesis, 3+ file clusters)

| # | Idea | Occurrences | Classification | Linked occurrences |
|---|---|---|---|---|
| 1 | Roadmap-walk procedure (read `ROADMAP.md` + dossier together; walk waypoints in dependency order; delegate large/ticket-backed waypoints as one isolated unit; validate before advancing) is written out in full in **two different instructions files** rather than once | F2, F4, F5 (F6 is F5's own legitimate restatement, excluded) | near-duplicate (F4↔F5 is the substantive pair; F2↔F5/F2↔F6 are a short pointer, thematic) | [prompt-ingestion.instructions.md#L47-L90](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md#L47-L90), [roadmap-execution.instructions.md#L6-L16](../../.agents/instructions/orchestration/roadmap-execution.instructions.md#L6-L16), [orchestrator.agent.md#L48-L52](../../.agents/agents/orchestrator.agent.md#L48-L52) |
| 2 | Interview-dispatch grounding rule ("hand the dispatched interview agent the research already gathered; interview only what evidence cannot resolve") restated across an agent template and both pipeline instructions files | F1, F3, F4 | near-duplicate | [mission-planning.agent.md#L9-L13](../../.agents/agents/mission-planning.agent.md#L9-L13), [intent-refinement.instructions.md#L12-L17](../../.agents/instructions/orchestration/intent-refinement.instructions.md#L12-L17), [prompt-ingestion.instructions.md#L20-L22](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md#L20-L22) |
| 3 | Stage-3 / Stage-5 review+interview loop folding its result into `REVIEW.md` / closing the open question against `ROADMAP.md` | F1, F3, F4, F7 (F7 is legitimate restatement of F3/F4, not counted as duplicate) | near-duplicate (F1↔F3, F1↔F4); legitimate restatement (F3↔F7, F4↔F7) | [mission-planning.agent.md#L28-L35](../../.agents/agents/mission-planning.agent.md#L28-L35), [intent-refinement.instructions.md#L19-L22](../../.agents/instructions/orchestration/intent-refinement.instructions.md#L19-L22), [prompt-ingestion.instructions.md#L20](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md#L20) |
| 4 (dropped) | "`ROADMAP.md` must ship with zero open questions / escalate drift instead of improvising" | F1, F3, F5, F6, F7 | thematic overlap only, no verbatim text shared | dropped — this is the repo's single shared *subject* (the roadmap artifact's no-open-questions invariant), independently phrased in every file that touches `ROADMAP.md`; not a duplicated rule statement, coincidental by topic rather than copy-paste. |
| 5 (dropped) | Prompt-file structural boilerplate: YAML frontmatter fields, `## Constraints` section, `## Response` section | F6, F7 (repo-wide `*.prompt.md` convention, only 2 files in this scope) | thematic overlap | dropped — shared authoring template across all prompt files in the repo, not a duplicated rule; below the 3-file cluster threshold in this scope and structural rather than substantive. |

Clusters 4 and 5 are reported for completeness but dropped from the ranked duplicate set per the synthesis phase's "coincidental phrasing" rule.

## All 21 pairs (verdict)

| Pair | Verdict |
|---|---|
| [mission-planning.agent.md](../../.agents/agents/mission-planning.agent.md) ↔ [orchestrator.agent.md](../../.agents/agents/orchestrator.agent.md) | thematic overlap |
| [mission-planning.agent.md](../../.agents/agents/mission-planning.agent.md) ↔ [intent-refinement.instructions.md](../../.agents/instructions/orchestration/intent-refinement.instructions.md) | near-duplicate |
| [mission-planning.agent.md](../../.agents/agents/mission-planning.agent.md) ↔ [prompt-ingestion.instructions.md](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md) | near-duplicate |
| [mission-planning.agent.md](../../.agents/agents/mission-planning.agent.md) ↔ [roadmap-execution.instructions.md](../../.agents/instructions/orchestration/roadmap-execution.instructions.md) | thematic overlap |
| [mission-planning.agent.md](../../.agents/agents/mission-planning.agent.md) ↔ [execute-roadmap.prompt.md](../../.agents/prompts/execute-roadmap.prompt.md) | thematic overlap |
| [mission-planning.agent.md](../../.agents/agents/mission-planning.agent.md) ↔ [refine-ingest.prompt.md](../../.agents/prompts/refine-ingest.prompt.md) | near-duplicate |
| [orchestrator.agent.md](../../.agents/agents/orchestrator.agent.md) ↔ [intent-refinement.instructions.md](../../.agents/instructions/orchestration/intent-refinement.instructions.md) | thematic overlap |
| [orchestrator.agent.md](../../.agents/agents/orchestrator.agent.md) ↔ [prompt-ingestion.instructions.md](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md) | thematic overlap |
| [orchestrator.agent.md](../../.agents/agents/orchestrator.agent.md) ↔ [roadmap-execution.instructions.md](../../.agents/instructions/orchestration/roadmap-execution.instructions.md) | near-duplicate |
| [orchestrator.agent.md](../../.agents/agents/orchestrator.agent.md) ↔ [execute-roadmap.prompt.md](../../.agents/prompts/execute-roadmap.prompt.md) | near-duplicate |
| [orchestrator.agent.md](../../.agents/agents/orchestrator.agent.md) ↔ [refine-ingest.prompt.md](../../.agents/prompts/refine-ingest.prompt.md) | thematic overlap |
| [intent-refinement.instructions.md](../../.agents/instructions/orchestration/intent-refinement.instructions.md) ↔ [prompt-ingestion.instructions.md](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md) | near-duplicate |
| [intent-refinement.instructions.md](../../.agents/instructions/orchestration/intent-refinement.instructions.md) ↔ [roadmap-execution.instructions.md](../../.agents/instructions/orchestration/roadmap-execution.instructions.md) | thematic overlap |
| [intent-refinement.instructions.md](../../.agents/instructions/orchestration/intent-refinement.instructions.md) ↔ [execute-roadmap.prompt.md](../../.agents/prompts/execute-roadmap.prompt.md) | thematic overlap |
| [intent-refinement.instructions.md](../../.agents/instructions/orchestration/intent-refinement.instructions.md) ↔ [refine-ingest.prompt.md](../../.agents/prompts/refine-ingest.prompt.md) | legitimate restatement |
| [prompt-ingestion.instructions.md](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md) ↔ [roadmap-execution.instructions.md](../../.agents/instructions/orchestration/roadmap-execution.instructions.md) | near-duplicate |
| [prompt-ingestion.instructions.md](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md) ↔ [execute-roadmap.prompt.md](../../.agents/prompts/execute-roadmap.prompt.md) | near-duplicate |
| [prompt-ingestion.instructions.md](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md) ↔ [refine-ingest.prompt.md](../../.agents/prompts/refine-ingest.prompt.md) | legitimate restatement |
| [roadmap-execution.instructions.md](../../.agents/instructions/orchestration/roadmap-execution.instructions.md) ↔ [execute-roadmap.prompt.md](../../.agents/prompts/execute-roadmap.prompt.md) | legitimate restatement |
| [roadmap-execution.instructions.md](../../.agents/instructions/orchestration/roadmap-execution.instructions.md) ↔ [refine-ingest.prompt.md](../../.agents/prompts/refine-ingest.prompt.md) | thematic overlap |
| [execute-roadmap.prompt.md](../../.agents/prompts/execute-roadmap.prompt.md) ↔ [refine-ingest.prompt.md](../../.agents/prompts/refine-ingest.prompt.md) | thematic overlap |

## Handoff

This review only reports findings; it does not rewrite, condense, or delete anything in the corpus. The most actionable finding for consolidation is **Cluster 1** (F4 vs F5): [prompt-ingestion.instructions.md](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md) restates the full roadmap-walk procedure that [roadmap-execution.instructions.md](../../.agents/instructions/orchestration/roadmap-execution.instructions.md) already owns authoritatively. Route this report to Simplify Agent for any consolidation decision (e.g. having F4 reference F5 instead of restating its procedure) — no such change is made here.
