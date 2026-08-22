# Specification Structure and Semantics: Refinement Dossier

## Reading Order

1. `input.md` - raw German request (original architecture guidance).
2. `input.clean.md` - verified English denoise of the original guidance.
3. `input-2.md` - raw German follow-up (Presentation System case-study pointer, folded in).
4. `input-2.clean.md` - verified English denoise of the follow-up.
5. `merged.clean.md` - the two transcripts folded into one current source; read this instead of re-reading `input.clean.md` and `input-2.clean.md` separately.
6. `REVIEW.md` - review verdict, findings, and scope decision.
7. `ARTIFACTS.md` - bounded inventory of existing evidence.
8. `01-case-study-and-target.md` - Presentation System diagnosis and desired model.
9. `02-existing-capability-and-decision.md` - verified reusable primitives and the (now-resolved, see `ROADMAP.md`) contract-ownership decision.
10. `03-migration-pilot-roadmap.md` - prioritized future work packages (superseded by `ROADMAP.md`'s waypoint ordering).
11. `04-completion-checklist.md` - traceability, deterministic artifact check, and the open questions `ROADMAP.md` resolves.
12. `05-target-artifact-contract.md` - Priority 1 deliverable: the component, criterion, evidence-reference, and contract-edge model.
13. `ROADMAP.md` - entry point: outcome summary, resolved ownership decision, validation gates, and ordered waypoints.

## Scope

This dossier evaluates the Presentation System specification as an example of the current monolithic specification shape. It derives a bounded path toward a component-oriented model with measurable criteria, evidence references, and directed contracts.

## Decision Boundary

This dossier does not create tickets, edit specifications, implement code, or change workflow/store state beyond the ticket-creation exception `ROADMAP.md` names for its oversized migration waypoint. It is a research-and-scoping artifact only. A later, separate `/tickets` or `/spec` step must consume `ROADMAP.md`.