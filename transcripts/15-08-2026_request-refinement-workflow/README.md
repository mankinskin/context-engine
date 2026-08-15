# Request-Refinement Workflow Dossier

## Purpose

This dossier applies the request-refinement pipeline to its own originating
request: formalize, as a reusable repository process, the denoise -> review ->
research-informed-restructure -> traceability-checklist pattern already
demonstrated in [transcripts/15-08-2026_verification-first-workflow/](../15-08-2026_verification-first-workflow/).

## Reading Order

1. [input.md](input.md) — the raw request, saved verbatim.
2. [input.clean.md](input.clean.md) — Stage 1: denoised, structured, English.
3. [REVIEW.md](REVIEW.md) — Stage 2: verdict, existing-capability check, findings, scope decision.
4. [05-completion-checklist.md](05-completion-checklist.md) — Stage 4: traceability matrix, deterministic checks, open questions.

## Stage 3 Output (Lives Outside This Dossier)

The research-informed restructure step of this run produced durable
repository process artifacts rather than more dossier documents, because the
deliverable the request asked for **is** the reusable process itself:

- [request-refinement-pipeline.instructions.md](../../.agents/instructions/orchestration/request-refinement-pipeline.instructions.md) — the canonical 4-stage process, citing the precedent transcript as its worked example.
- [refine-request.prompt.md](../../.agents/prompts/refine-request.prompt.md) — the `/refine-request` slash command that sequences the four stages.
- [AGENTS.md](../../AGENTS.md) Task Routing — the new first row that sends a raw/unstructured request through `/refine-request` before `/tickets` or `/spec`.

## Scope

This dossier and the artifacts it produced are process/documentation changes
only. No ticket was created, no spec was created or edited, and no product
code or workflow/store state was changed.

## Decision Boundary

Using the new `/refine-request` pipeline to turn any of its own open
questions into tickets is a separate, later step, to be taken only when a
user or reviewer decides to act on them.
