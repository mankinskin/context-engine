---
description: "Use before turning a raw, unstructured user request into tickets, a spec, or any other complex downstream workflow. Covers the denoise -> review-gate -> research-informed restructure -> traceability-checklist pipeline that must run first, worked example, and the decision boundary that keeps this stage from silently becoming implementation."
applyTo: "**/*.md"
---

## Purpose

A raw request — a rambling transcript, a dictated prompt, a stream-of-consciousness ask — must not be handed directly to `tickets.prompt.md`, `spec.prompt.md`, or an implementation session. Structure and scope are extracted first, cheaply, in a bounded pipeline, and only the resulting dossier is used to seed tickets/specs. This closes the gap the raw-request path otherwise leaves open: unbounded scope, no verification lens, and no evidence that the eventual tickets actually cover what the requester said.

## Worked Example (Canonical Reference)

[transcripts/15-08-2026_verification-first-workflow/](../../../transcripts/15-08-2026_verification-first-workflow/) is the reference instance of this pipeline, produced before this instruction file existed. Read it once to see the pattern in full:

- [input.md](../../../transcripts/15-08-2026_verification-first-workflow/input.md) — the raw transcript, saved verbatim.
- [input.clean.md](../../../transcripts/15-08-2026_verification-first-workflow/input.clean.md) — Stage 1 output: denoised, restructured, still just the requester's own intent.
- [REVIEW.md](../../../transcripts/15-08-2026_verification-first-workflow/REVIEW.md) — Stage 2 output: a verdict (`Changes requested`), a research-findings table, a findings/severity table, and a bounded scope decision.
- [01-research-inventory.md](../../../transcripts/15-08-2026_verification-first-workflow/01-research-inventory.md) through [03-improved-one-session-plan.md](../../../transcripts/15-08-2026_verification-first-workflow/03-improved-one-session-plan.md) — Stage 3 output: the research-informed restructure, turning the reviewed scope into work packages with outcomes, non-goals, and validation methods.
- [04-completion-checklist.md](../../../transcripts/15-08-2026_verification-first-workflow/04-completion-checklist.md) — Stage 4 output: a requirement-to-artifact traceability matrix, deterministic artifact checks, and an explicit open-questions list.
- [README.md](../../../transcripts/15-08-2026_verification-first-workflow/README.md) — the index: reading order, scope, and the decision boundary that stops the dossier from drifting into implementation.

## The Four Stages

Run each stage as a distinct pass; do not collapse them. Each stage has one job and one exit artifact.

1. **Denoise (cheap).** Dispatch the [Transcription Agent](../../agents/transcription.agent.md) per [audio-transcript.instructions.md](../transcripts/audio-transcript.instructions.md), even when the source was typed rather than spoken — the same denoise/restructure/verify pipeline removes filler, resolves self-corrections, and translates non-English input without adding or dropping content. Output: `input.md` (raw, verbatim) and `input.clean.md` (denoised) in a dated `transcripts/DD-MM-YYYY_<slug>/` folder.
2. **Review gate.** Critique the cleaned request the way [review.agent.md](../../agents/review.agent.md) critiques an implementation: is the ask bounded, is it verifiable, does it conflate distinct concerns, what existing repository capability already answers part of it? Produce a verdict (`Changes requested` or `Approved as scoped`), a findings table with severity and required improvement per finding, and an explicit scope decision listing what the eventual dossier will and will not contain. Output: `REVIEW.md`.
3. **Research-informed restructure.** Only after the review gate bounds the scope, dispatch [Research Agent](../../agents/research.agent.md) or [Structured Research Agent](../../agents/structured-research.agent.md) (dialectic pass, when the first answer needs adversarial testing) to check each reviewed concern against actual repository capability, then rewrite the bounded scope as concrete, independently actionable work packages — each with an outcome, a non-goal, and a validation method. Output: one or more numbered documents (`01-...md`, `02-...md`, ...).
4. **Traceability checklist.** Close the loop: map every requirement from the raw transcript to the dossier location that addresses it, run deterministic checks that the expected artifacts exist and are non-empty, and list genuinely open questions rather than silently resolving them. Output: a final numbered `NN-completion-checklist.md` and a `README.md` index stating reading order, scope, and the decision boundary below.

## Decision Boundary

The dossier produced by this pipeline is a bounded research-and-scoping artifact, not an implementation. State this explicitly in the dossier's `README.md`:

- The pipeline may read source, docs, tickets, and specs; it does not mutate them.
- The pipeline does not create tickets, does not create or edit a spec, and does not change any workflow or store state.
- Turning a roadmap item from Stage 3 into a ticket happens in a **separate**, later step — `tickets.prompt.md` or `spec.prompt.md` — consuming the dossier's roadmap as its input, after the requester (or reviewer) picks which items to act on.

This mirrors [escalation-gate.instructions.md](escalation-gate.instructions.md) and [phase-separation.instructions.md](phase-separation.instructions.md): discovery/interview/review happen before implementation, and this pipeline is exactly that discovery phase for a raw request.

## When to Run This Pipeline

Run it before `tickets.prompt.md`, `spec.prompt.md`, or any multi-file implementation session whenever the incoming request is:

- a raw transcript, dictation, or stream-of-consciousness prompt rather than an already-scoped ask,
- broad enough that "just start implementing" would produce an unbounded session (compare the "Feature or refactor" and "Unfamiliar module" rows in `AGENTS.md`'s Task Routing table),
- ambiguous about whether it is one request or several interleaved concerns.

Skip it for an already-bounded, single-file fix or an ask that already names its acceptance criteria — running the full pipeline on a two-line, unambiguous request is pure overhead.

## Cost Note

Stage 1 (denoise) runs on the cheap tier per `transcription.agent.md`'s own `model:` declaration. Stage 2 (review) and Stage 3 (research) are judgement-bearing and route per the tier ladder in [model-routing.instructions.md](model-routing.instructions.md) — do not run the whole pipeline on the orchestrator-tier model when the denoise pass alone is mechanical.
