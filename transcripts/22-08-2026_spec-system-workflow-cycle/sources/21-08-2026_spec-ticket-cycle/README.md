# README — Closed-Loop Production-Workflow Cycle Dossier

## Reading Order

1. [input.md](input.md) / [input.clean.md](input.clean.md) — raw and cleaned first-pass transcript (the three-level workflow description: free-text-only, tickets-only, spec-derived-tickets).
2. [input-2.md](input-2.md) / [input-2.clean.md](input-2.clean.md) — raw and cleaned follow-up transcript (the framing/prioritization addendum: this is the core production cycle, must be documented and shown in the deck).
3. [merged.clean.md](merged.clean.md) — the combined, coherent statement of intent both inputs describe together.
4. [ARTIFACTS.md](ARTIFACTS.md) — existing repository artifacts relevant to this dossier.
5. [REVIEW.md](REVIEW.md) — first informed review + interview loop: verdict, critique, scope decision.
6. [01-document-closed-loop-cycle.md](01-document-closed-loop-cycle.md), [02-presentation-deck-slide.md](02-presentation-deck-slide.md), [03-ticket-spec-gating-edge.md](03-ticket-spec-gating-edge.md), [04-test-evidence-link.md](04-test-evidence-link.md) — the four work packages.
7. [ROADMAP.md](ROADMAP.md) — the entry point for whoever picks this up next: artifact ids, blockers (none), validation gates, the ordered task list, and heads-up notes.

## Scope

This dossier documents a closed-loop production-workflow cycle (request → spec → tickets → tests → implementation → validated response → next iteration) as a named, citable principle, scopes adding it to the repository's presentation deck, and flags one genuine architecture gap (ticket-to-spec gating) as a separate ticket rather than implementing it here.

## Decision Boundary

- This dossier is read-only with respect to code: no implementation was written.
- One ticket was created during roadmap compilation, per the pipeline's own ticket-creation exception for oversized work: [5b50329b Ticket-depends-on-spec gating edge](../../.ticket/tickets/5b50329b-59f3-4a6f-a90e-cbacefdcce48).
- No spec was created or edited in this pass. `ROADMAP.md` is a scoping/sequencing artifact, not a spec — a separate, later `/spec` step (likely extending the existing [2ccde9ee Presentation System](../../.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/spec.toml) spec for Task 2, and a new spec if Task 1's cycle principle needs one) picks this roadmap up.
- No open question remains: both ambiguities raised in the first review loop (target deck; whether the ticket-spec gating capability already exists) were resolved by research evidence, and the second review loop's one new finding (coordinate with the existing presentation epic) was folded directly into `ROADMAP.md` Task 2 and its heads-up notes.
