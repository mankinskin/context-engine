# Presentation Automation Planning Dossier

## Reading Order

1. [input.md](input.md) — verbatim German source transcript.
2. [input.clean.md](input.clean.md) — English denoise and refined-track context.
3. [REVIEW.md](REVIEW.md) — Stage 2 scope gate.
4. [ARTIFACTS.md](ARTIFACTS.md) — Stage 3 frozen repository evidence.
5. [01-conceptual-input-contract.md](01-conceptual-input-contract.md) — first implementation-sized package.
6. [02-projection-extractors.md](02-projection-extractors.md) — second implementation-sized package.
7. [03-deck-generation-validation.md](03-deck-generation-validation.md) — third implementation-sized package.
8. [04-completion-checklist.md](04-completion-checklist.md) — Stage 5 traceability and open questions.
9. [ROADMAP.md](ROADMAP.md) — current entry point for the first executing session.

## Scope

This dossier scopes specification-derived conceptual decks for a live human
audience. It establishes safe generation contracts, separate Git/Cargo
projections, declarative workflow illustrations, and static deck validation.
It does not implement product code in this dossier.

## Decision Boundary

Stages 1 through 5 were read-only research and scoping passes. Stage 6 created
three ticket-backed work packages because they are multi-session dependencies;
it did not edit a specification. `ROADMAP.md` is a sequencing artifact, not an
implementation or a replacement specification. An executing session must use
the existing governing specification and the linked tickets before changing
product behavior.
