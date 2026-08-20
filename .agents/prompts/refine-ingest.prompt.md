---
description: "Run the prompt-ingestion pipeline on a raw, unstructured prompt (transcript, dictation, or rambling ask) before it becomes tickets, a spec, or an implementation session. Produces a bounded dossier under transcripts/, never tickets or code."
name: "refine-ingest"
argument-hint: "<the raw prompt text, or a path to an existing transcript file/folder>"
agent: "agent"
---

# Refine Ingest

Turn the raw prompt typed after `/refine-ingest` into a bounded, evidence-backed dossier before any ticket, spec, or implementation work starts on it.

Follow [intent-refinement.instructions.md](../instructions/orchestration/intent-refinement.instructions.md) for Stages 1-2 and [prompt-ingestion.instructions.md](../instructions/orchestration/prompt-ingestion.instructions.md) for Stages 3-6, the worked example, and the decision boundary. This prompt only sequences the dispatch.

## Workflow

1. **Resolve the input.** If the argument is a path to an existing transcript file or folder, read it in place. Otherwise treat the argument as raw text: create `transcripts/DD-MM-YYYY_<short-kebab-slug>/` (append `-HHMMSS` only if that date+slug folder already exists today) and write the raw text verbatim to `input.md`.
2. **Stage 1 — Denoise.** Dispatch the [Transcription Agent](../agents/transcription.agent.md) on that folder/file. Do not let it restructure beyond what the instruction file specifies. Confirm `input.clean.md` exists before continuing.
3. **Stage 2 — Review gate.** Critique the cleaned prompt: verdict, findings table with severity and required improvement, existing-capability check against the repository, and an explicit scope decision (what the dossier will and will not cover). Write `REVIEW.md`. If the verdict is `Changes requested`, the scope decision — not the raw ask — is what Stage 3 restructures against. If the underlying mission goal itself is unclear rather than just the wording, dispatch the [Mission Planning Agent](../agents/mission-planning.agent.md) instead of guessing the scope decision.
4. **Stage 3 — Research-informed restructure.** Dispatch [Research Agent](../agents/research.agent.md) (or [Structured Research Agent](../agents/structured-research.agent.md) for a thesis that needs adversarial testing) to check each reviewed concern against real repository capability and rewrite the bounded scope as numbered documents (`01-...md`, `02-...md`, ...), each work package carrying an outcome, a non-goal, and a validation method.
5. **Stage 4 — Traceability checklist.** Write a final `NN-completion-checklist.md`: a table mapping every raw-transcript requirement to the dossier location that addresses it, a deterministic check that every expected artifact exists and is non-empty, and an explicit open-questions list for anything genuinely unresolved.
6. **Index.** Write `README.md` linking every artifact in reading order, stating scope, and stating the decision boundary: this dossier does not create tickets, does not edit a spec, and does not change workflow/store state. A separate later step (`/tickets` or `/spec`) consumes the roadmap after the requester picks which items to act on.

## Constraints

- Do not create a ticket, create or edit a spec, or change any store/workflow state during this pipeline.
- Do not implement any code change described in the prompt.
- Do not skip a stage or merge two stages into one document — each stage's artifact must be independently inspectable.
- If the prompt is already a bounded, single-file ask with clear acceptance criteria, say so and skip the pipeline rather than manufacturing ceremony.

## Response

- resolved input mode (existing path vs newly created `transcripts/DD-MM-YYYY_<slug>/`)
- dossier file paths created, in reading order
- Stage 2 verdict and the scope decision it produced
- the prioritized roadmap items from Stage 3, each with its non-goal
- open questions from the completion checklist
- explicit reminder that no ticket/spec/implementation was created — that is the next, separate step

