# Duplication Report — Intent-Refinement / Transcription / Ticket-Refinement Clusters

**Scope**: 14 files across three related guidance clusters (narrowed scope, not the full `.agents/` corpus) — see [pair-ledger.md](pair-ledger.md) for the file index and full batch/pair table, and [duplicate-passages.md](duplicate-passages.md) for every marked-section finding.

**Coverage**: 91/91 pairs evaluated (`14 × 13 / 2 = 91`), 19 batches, 4 phases. Every pair has a verdict; every pair has at least one finding row.

## Verdict Counts

| Verdict | Count |
|---|---:|
| exact duplicate | 3 |
| near-duplicate | 24 |
| thematic overlap | 35 |
| no overlap | 29 |
| **Total pairs** | **91** |

## Top Duplicated Ideas (ranked by occurrence)

### 1. The transcript denoise/restructure/verify pipeline is restated, not referenced, by its consumers (~19 occurrences across 5 files)

`audio-transcript.instructions.md` is the canonical three-stage pipeline (Stage 1 Denoise → Stage 2 Restructure → Stage 3 Verify), its constraint list (preserve identifiers, resolve self-corrections to final value, translate-then-verify non-English content), its `transcripts/DD-MM-YYYY_<slug>/` + `input.md`/`input-N.md`/`merged.clean.md` naming convention, and its Output Requirements. Three consumer files restate this content near-verbatim instead of pointing at it cleanly:

- [.agents/agents/transcription.agent.md](../../.agents/agents/transcription.agent.md) restates the Input Modes paragraph ([L17-28](../../.agents/agents/transcription.agent.md#L17-L28)), all three stage descriptions ([L44-46](../../.agents/agents/transcription.agent.md#L44-L46)), the constraints list ([L53-57](../../.agents/agents/transcription.agent.md#L53-L57)), the output requirements ([L62-67](../../.agents/agents/transcription.agent.md#L62-L67)), and the merge/fold rules ([L29,49](../../.agents/agents/transcription.agent.md#L29)) — all near-duplicates of the corresponding [audio-transcript.instructions.md](../../.agents/instructions/transcripts/audio-transcript.instructions.md) sections.
- [.agents/prompts/transform-transcript.prompt.md](../../.agents/prompts/transform-transcript.prompt.md) restates the same three stages ([L27-34](../../.agents/prompts/transform-transcript.prompt.md#L27-L34)) and delivery/report fields ([L47-50](../../.agents/prompts/transform-transcript.prompt.md#L47-L50)) — near-identical to both `audio-transcript.instructions.md` and `transcription.agent.md`, and contains two **exact-duplicate** verbatim sentences shared with `transcription.agent.md`: "Run the three-stage pipeline as distinct passes. Do not collapse them." ([transcription.agent.md#L41](../../.agents/agents/transcription.agent.md#L41) / [transform-transcript.prompt.md#L27](../../.agents/prompts/transform-transcript.prompt.md#L27)).
- [.agents/prompts/refine-ingest.prompt.md](../../.agents/prompts/refine-ingest.prompt.md) restates the dated-folder/`input.md` convention ([L18-19](../../.agents/prompts/refine-ingest.prompt.md#L18-L19)) and contains the **exact-duplicate** heading "**Stage 1 — Denoise.**" shared with `transcription.agent.md` ([L21](../../.agents/prompts/refine-ingest.prompt.md#L21) / [transcription.agent.md#L44](../../.agents/agents/transcription.agent.md#L44)).
- [.agents/instructions/orchestration/prompt-ingestion.instructions.md](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md) restates the naming convention and "same three-stage denoise/restructure/verify pipeline" language ([L18](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md#L18)) even though it also correctly states the stage is "delegate[d] entirely" — the delegation sentence and the restated details coexist in the same paragraph.

**Assessment**: this is the cluster the user specifically flagged, and the review confirms it — the "delegation" from `transcription.agent.md`/`transform-transcript.prompt.md` to `audio-transcript.instructions.md` is not clean; large chunks of stage/constraint/output text are copy-derived rather than referenced.

### 2. `intent-refinement.instructions.md` ↔ `prompt-ingestion.instructions.md` — two verbatim exact duplicates despite an explicit "owns" split

These two files state they have a clean ownership boundary (`prompt-ingestion.instructions.md` "owns the dossier folder layout and the six-stage sequence"; `intent-refinement.instructions.md` "owns the recurring technique"), yet two full sentences are copy-pasted verbatim between them:

- "Output: `REVIEW.md` with an `Approved as scoped` verdict and a scope decision." — [intent-refinement.instructions.md#L23](../../.agents/instructions/orchestration/intent-refinement.instructions.md#L23) / [prompt-ingestion.instructions.md#L20](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md#L20).
- "This loop replaces a separate traceability-checklist stage: its job is to make sure `ROADMAP.md` ships with zero open questions — every one gets an interview, not a checklist entry." — [intent-refinement.instructions.md#L24](../../.agents/instructions/orchestration/intent-refinement.instructions.md#L24) / [prompt-ingestion.instructions.md#L22](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md#L22).

**Assessment**: given the deliberate owns/references design of these two files, this is the highest-value exact-duplicate finding to fix — one file should state the fact once and the other should cross-reference it.

### 3. The "do not implement code changes unless explicitly asked" guardrail (6 occurrences)

Restated as a near-duplicate boilerplate constraint across nearly every agent/prompt in scope: [mission-planning.agent.md#L23](../../.agents/agents/mission-planning.agent.md#L23), [ticket-refinement.agent.md#L24](../../.agents/agents/ticket-refinement.agent.md#L24), [transcription.agent.md#L58](../../.agents/agents/transcription.agent.md#L58), [refine-ingest.prompt.md#L31-32](../../.agents/prompts/refine-ingest.prompt.md#L31-L32), [tickets.prompt.md#L35](../../.agents/prompts/tickets.prompt.md#L35), [transform-transcript.prompt.md#L57](../../.agents/prompts/transform-transcript.prompt.md#L57).

**Assessment**: likely acceptable as a per-agent guardrail restatement (each agent needs its own explicit constraint), but a shared short reference (e.g. to a common constraints note) would remove six near-identical sentences.

### 4. "Search/discover existing tickets before creating" — `workflow.instructions.md`'s Discovery Before Creating section is the canonical source, correctly cross-referenced by anchor text in most places but restated as prose in others

- [ticket-refinement.agent.md#L36](../../.agents/agents/ticket-refinement.agent.md#L36) and [tickets.prompt.md#L22](../../.agents/prompts/tickets.prompt.md#L22) both cite `workflow.instructions.md#discovery-before-creating` verbatim — clean cross-reference, not a duplication problem.
- [workflow.instructions.md#L106](../../.agents/instructions/ticket/workflow.instructions.md#L106) itself ("Always search for existing tickets before creating new ones...") is restated as prose (not just referenced) in [board.instructions.md#L7-8](../../.agents/instructions/ticket/board.instructions.md#L7-L8) via the check-in/check-out framing, and that same board sentence is independently restated in both [workflow.instructions.md#L18-19](../../.agents/instructions/ticket/workflow.instructions.md#L18-L19) (orientation step) and [ticket-next.prompt.md#L16](../../.agents/prompts/ticket-next.prompt.md#L16) ("Inspect the draftboard...").

**Assessment**: minor — three files each restate "check the draftboard first" in their own words; low risk, low priority.

### 5. `ticket-next.prompt.md` ↔ `tickets.prompt.md` — near-identical structure

Both slash-command prompts share a near-identical reference list ([ticket-next.prompt.md#L12](../../.agents/prompts/ticket-next.prompt.md#L12) / [tickets.prompt.md#L13](../../.agents/prompts/tickets.prompt.md#L13)) and a near-identical "Return"/"Response" bullet shape citing the Clickable Reference Policy ([ticket-next.prompt.md#L30-36](../../.agents/prompts/ticket-next.prompt.md#L30-L36) / [tickets.prompt.md#L37-42](../../.agents/prompts/tickets.prompt.md#L37-L42)).

**Assessment**: expected boilerplate for sibling slash-commands; low priority.

## No-Overlap Pairs of Note

The three ticket-instructions engine/lifecycle/board files ([board.instructions.md](../../.agents/instructions/ticket/board.instructions.md), [engine.instructions.md](../../.agents/instructions/ticket/engine.instructions.md), [lifecycle.instructions.md](../../.agents/instructions/ticket/lifecycle.instructions.md)) are largely orthogonal to the transcription/intent-refinement cluster (29 of 91 pairs are `no overlap`, concentrated on cross-cluster pairs between ticket-engine internals and the transcript pipeline) — confirming these clusters are only lightly coupled outside the specific ticket-workflow-boilerplate findings above.

## Handoff

This review reports findings only; it does not rewrite, condense, or delete any file. For consolidation of the exact-duplicate and near-duplicate findings above (especially #1 and #2), route this report to **Simplify Agent**.
