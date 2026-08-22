---
description: "Run the prompt-ingestion pipeline on a raw, unstructured prompt (transcript, dictation, or rambling ask) before it becomes tickets, a spec, or an implementation session. Extends the transcript-transformation pipeline through research, two informed review/interview loops, and roadmap compilation into a fully refined, zero-open-question dossier under transcripts/, never tickets or code."
name: "refine-ingest"
argument-hint: "<the raw prompt text, or a path to an existing transcript file/folder>"
agent: "agent"
---

# Refine Ingest

Turn the raw prompt typed after `/refine-ingest` into a bounded, evidence-backed, fully refined dossier before any ticket, spec, or implementation work starts on it — the raw spell elevated into the mechanical steps that execute the requester's actual intent, with nothing added and nothing lost.

Follow [intent-refinement.instructions.md](../instructions/orchestration/intent-refinement.instructions.md) for the informed review + interview loop (Stages 3 and 5) and [prompt-ingestion.instructions.md](../instructions/orchestration/prompt-ingestion.instructions.md) for the full six-stage sequence and the decision boundary. This prompt only sequences the dispatch.

## Workflow

1. **Check for an in-progress dossier from this session first.** Follow [prompt-ingestion.instructions.md](../instructions/orchestration/prompt-ingestion.instructions.md)'s "Resuming an In-Progress Dossier" section: scan this conversation for a dossier path a prior `/refine-ingest` call already created or resumed this session, or check `session_runtime_view` for a path pinned under relation `intent-ingestion-dossier`. Treat the new input as a continuation only when the dossier is from this session **and** the new ask is thematically continuous with it (a refinement, addition, or correction), not an unrelated request. This is the standard mode of iteration, not an edge case: reviewing the dossier and re-triggering `/refine-ingest` with an additional transcript is how a requester refines a request across turns.
2. **Resolve the input**, following the naming convention in [audio-transcript.instructions.md's Scope](../instructions/transcripts/audio-transcript.instructions.md#scope) and [Multi-Transcript Composition](../instructions/transcripts/audio-transcript.instructions.md#multi-transcript-composition) sections.
   - **Continuation:** write the new raw text verbatim to the next `input-N.md` (`input-2.md`, `input-3.md`, ...) inside the existing dossier folder — never a new dated folder, never an `addendum` file.
   - **New request:** if the argument is a path to an existing transcript file or folder, read it in place. Otherwise treat the argument as raw text: create `transcripts/DD-MM-YYYY_<short-kebab-slug>/` (append `-HHMMSS` only if that date+slug folder already exists today) and write the raw text verbatim to `input.md`.
   - Either way, pin the resolved folder path via `session_runtime_pin` (relation `intent-ingestion-dossier`) so a later call in this session can find it.
3. **Stage 1 — Denoise.** Dispatch the [Transcription Agent](../agents/transcription.agent.md) on the new file (`input.md` for a new dossier, the latest `input-N.md` for a continuation), producing the matching `input-N.clean.md`. On a continuation, update `merged.clean.md` from the full set of clean parts so it reflects the combined intent rather than only the newest fragment. Confirm the clean artifact(s) exist before continuing.
4. **Stage 2 — Research and artifact inventory.** Dispatch a read-only [Explore Agent](../agents/explore.agent.md) or [Research Agent](../agents/research.agent.md) pass to gather every existing artifact relevant to the cleaned prompt — tickets, specs, docs, prior dossiers, code/config paths. Write `ARTIFACTS.md`. On a continuation, add new rows and update stale ones in place rather than starting a new file.
5. **Stage 3 — First informed review + interview loop.** Critique the cleaned prompt **against `ARTIFACTS.md`**, not the raw words alone: verdict, findings table with severity and required improvement, an explicit scope decision. Write `REVIEW.md`. If a finding is a genuine ambiguity the research cannot resolve, dispatch the [Mission Planning Agent](../agents/mission-planning.agent.md) (mission-goal-level gap) or the [Interview Agent](../agents/interview.agent.md) (narrower requirement gap) — handing over the research already gathered — and repeat the critique until the verdict is `Approved as scoped`. On a continuation, version the outgoing `REVIEW.md` (`REVIEW.v1.md`, ...) before writing the refined one against the combined intent.
6. **Stage 4 — Fully informed dossier creation or restructure.** With the scope decision and `ARTIFACTS.md` both in hand, dispatch [Research Agent](../agents/research.agent.md) (or [Structured Research Agent](../agents/structured-research.agent.md) for a thesis that needs adversarial testing) to produce, in one informed pass: the numbered work-package documents (`01-...md`, `02-...md`, ...; each with an outcome, a non-goal, and a validation method), a draft `ROADMAP.md`, and a draft `README.md` index.
7. **Stage 5 — Second informed review + interview loop.** Critique the drafted dossier and `ROADMAP.md` for anything newly ambiguous or low-confidence that drafting surfaced. Interview via Mission Planning/Interview Agent to close each one — do not log an open question and move on. This replaces a separate traceability-checklist stage: coverage already lives in `ARTIFACTS.md` and `ROADMAP.md`.
8. **Stage 6 — Adjustments and roadmap compilation.** Apply the second loop's resolved answers, then dry-run `ROADMAP.md` per [prompt-ingestion.instructions.md](../instructions/orchestration/prompt-ingestion.instructions.md)'s "Roadmap Improvement Loop" and refine it until no new blocker surfaces. `ROADMAP.md` must ship with zero open questions — do not end the pipeline while one remains. On a continuation, version the prior `ROADMAP.md` (`ROADMAP.v1.md`, ...) before writing the refined one.
9. **Index.** Finalize `README.md` linking every artifact in reading order, with `ROADMAP.md` as the entry point, stating scope and the decision boundary: this dossier does not create tickets, does not edit a spec, and does not change workflow/store state beyond the ticket exception `ROADMAP.md` compilation is allowed. A separate later step (`/tickets` or `/spec`) consumes the roadmap after the requester picks what to act on. On a continuation, update `README.md` in place rather than duplicating it.

## Constraints

- Do not create a ticket, create or edit a spec, or change any store/workflow state during this pipeline (except the ticket-creation exception in Stage 6).
- Do not implement any code change described in the prompt.
- Do not skip a stage or merge two stages into one document — each stage's artifact must be independently inspectable.
- If the prompt is already a bounded, single-file ask with clear acceptance criteria, say so and skip the pipeline rather than manufacturing ceremony.
- Do not create a second dossier folder for a follow-up ask that continues the same in-flight request within this session — resume the existing one per [prompt-ingestion.instructions.md](../instructions/orchestration/prompt-ingestion.instructions.md)'s dossier-resumption rule instead.
- Never critique or interview from the raw prompt's words alone — both review + interview loops must be grounded in `ARTIFACTS.md` or the drafted dossier, per [intent-refinement.instructions.md](../instructions/orchestration/intent-refinement.instructions.md).
- Do not end the pipeline with an open question still unresolved, and do not end it without a `ROADMAP.md` — it is the main deliverable an implementing session reads first, and it must ship with zero open questions.

## Response

- whether this run continued an existing session dossier or created a new one, and the folder path either way
- resolved input mode (existing path, continuation `input-N.md`, or newly created `transcripts/DD-MM-YYYY_<slug>/`)
- dossier file paths created or updated, in reading order, ending with `ROADMAP.md`
- both loops' verdicts: Stage 3's scope decision and Stage 5's confirmation that no open question remains
- `ROADMAP.md`'s outcome summary
- the prioritized waypoints from `ROADMAP.md`, each with its non-goal
- explicit reminder that no ticket/spec/implementation was created — that is the next, separate step

