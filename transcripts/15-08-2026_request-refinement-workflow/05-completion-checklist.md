# Completion Checklist

## Traceability Matrix

| Requirement from `input.clean.md` | Where it is addressed | Status |
| --- | --- | --- |
| Use the precedent transcript as the model for an end-to-end request-to-solution workflow. | [prompt-ingestion.instructions.md](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md) — "Worked Example (Canonical Reference)" section cites the precedent directly. | Pass |
| Step 1: a cheap agent cleans up the raw input. | Same file — Stage 1 ("Denoise"), reusing [transcription.agent.md](../../.agents/agents/transcription.agent.md) unmodified. | Pass |
| Step 2: the cleaned input is reviewed for structure. | Same file — Stage 2 ("Review gate"), producing a verdict + findings table + scope decision. | Pass |
| Step 3: research further improves the input and produces new files. | Same file — Stage 3 ("Research-informed restructure") and Stage 4 ("Traceability checklist"), producing numbered documents. | Pass |
| Always run this before a raw prompt kicks off tickets or other complex workflows. | [AGENTS.md](../../AGENTS.md) Task Routing — new first row routes raw/unstructured requests to `/refine-ingest` before the ticket/spec rows. | Pass |
| Without going ahead and implementing anything. | Pipeline instructions file — "Decision Boundary" section: the dossier never creates tickets/specs/code; that is a separate later step. | Pass |
| The workflow could be applied directly to this very request. | This dossier ([README.md](README.md), `input.md`, `input.clean.md`, `REVIEW.md`, this checklist) is that applied instance. | Pass |

## Deterministic Artifact Checks

| Check | Expected result | Status |
| --- | --- | --- |
| Raw source saved verbatim | [input.md](input.md) exists, unedited German text | Pass |
| Cleaned artifact exists | [input.clean.md](input.clean.md) exists, English, structured | Pass |
| Review exists with a verdict | [REVIEW.md](REVIEW.md) states "Approved as scoped, with two required refinements" | Pass |
| Reusable process artifact exists outside this dossier | [prompt-ingestion.instructions.md](../../.agents/instructions/orchestration/prompt-ingestion.instructions.md) created | Pass |
| Reusable slash-command exists | [refine-ingest.prompt.md](../../.agents/prompts/refine-ingest.prompt.md) created | Pass |
| Routing entry point updated | [AGENTS.md](../../AGENTS.md) Task Routing table has the new first row | Pass |
| No ticket, spec, or code change was made by this dossier | No `ticket`/`spec` CLI or MCP mutation call was issued during this session; no files outside `transcripts/`, `.agents/instructions/`, `.agents/prompts/`, and `AGENTS.md` were touched | Pass |

## Open Questions for a Follow-Up Session

1. Should `/refine-ingest` become a **mandatory** pre-step enforced by tooling (e.g., a hook that blocks `/tickets` on an unscoped raw prompt), or does it remain a convention the orchestrator applies by judgment? This dossier assumes the latter — no enforcement mechanism was built.
2. Should the Stage 2 review gate be a distinct new lightweight agent, or is directly reusing `review.agent.md`'s contract (as this dossier assumes) sufficient once a second worked example exists?
