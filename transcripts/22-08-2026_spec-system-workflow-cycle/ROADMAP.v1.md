# Roadmap — Spec-System Redesign and Full Workflow-Cycle Implementation

## Outcome Summary

Implement, test, and document a new component-oriented specification system — components, measurable acceptance criteria, external evidence references, and directed consumer/provider contract edges (permitting cycles) — specified as dogmatically as possible using the *current* (old) spec tooling, then used to specify the full production workflow cycle (request → spec → tickets → tests → implementation → validated response → next iteration) and its adjacent tooling (tickets, docs, tests). The new system then becomes authoritative: the spec system's own specification migrates into it, the presentation deck gains a diagram of the full cycle, and agent guidance is updated to teach the new system and cycle. The main outcome is a better-specified, better-verified basis for defining goals on both new and existing products — not a documentation exercise.

This roadmap supersedes both source dossiers' own `ROADMAP.md` files as the current, most-refined artifact for this combined effort; see `sources/*/ROADMAP.md` for each dossier's original, narrower plan.

## Relevant Artifact IDs

See [ARTIFACTS.md](ARTIFACTS.md) for the full union with state and relevance. Load-bearing ids repeated here for convenience:

- Ticket-depends-on-spec gating edge — cited by both source dossiers as ticket `5b50329b`, but a dry-run probe of this roadmap found that id does not resolve in the current ticket store (see Active Blockers). Genuinely distinct from the contract-edge model below (ticket-level vs. component-level; see duplication-review verdict below).
- [2ccde9ee Presentation System spec](../../.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/spec.toml) — shared case-study anchor for the monolithic-shape problem; **no longer the migration target** (see Resolved Decisions).
- [0ee95228 presentation epic](../../.ticket/tickets/0ee95228-475d-4706-a108-fd208f7c4098/ticket.toml) — coordinate with, do not bypass, for the presentation-deck waypoint.
- [workflow-tools/spec/crates/spec-api/](../../workflow-tools/spec/crates/spec-api/) — current (old) spec tooling: `manifest.rs`, `schemas/specification.toml`, `store/sections.rs`, `store/hierarchy.rs`, `ticket_ref.rs`, `tests/schema_test.rs`.
- [duplication-reviews/22-08-2026_spec-system-workflow-cycle-merge/](../../duplication-reviews/22-08-2026_spec-system-workflow-cycle-merge/) — this merge's duplication-review workspace.

## Duplication-Review Consolidation Verdict

A scoped duplication review (workspace above) compared both source dossiers' key artifacts and resolved three open questions before this roadmap was compiled:

- Both dossiers describe overlapping views of "the spec system" (same Presentation System spec id, same test-api evidence-linking capability, same confirmed absence of ticket→spec gating) — consolidated once each into [ARTIFACTS.md](ARTIFACTS.md) instead of restated per-waypoint.
- The **directed contract edge** (Dossier B, component-to-component, within one spec) and the **ticket-depends-on-spec gating edge** (Dossier A, ticket-to-spec, ticket 5b50329b) are **genuinely distinct** and are kept as two separate waypoints below, not merged.
- Both source `ROADMAP.md` files independently referenced the same Presentation System spec id and the same `[[refs]]`-is-informational-only caution — this was intentional cross-dossier coordination, not accidental duplication, and is preserved as a single statement here.

## Active Blockers

- **Ticket 5b50329b does not resolve.** Both source dossiers cite `5b50329b Ticket-depends-on-spec gating edge` as already created during Dossier A's own roadmap compilation. A dry-run probe of this merged roadmap (`get_ticket 5b50329b`, `list_tickets` by title, `next_tickets` by title filter) found no matching ticket in any searched workspace. Treat Waypoint 10 as **not yet created** — whoever picks up Waypoint 10 must create the ticket first, then proceed. This does not block Waypoints 1-9, which do not depend on Waypoint 10.

The contract-ownership decision left open by Dossier B is resolved (see Resolved Decisions); that is not a blocker.

## Resolved Decisions

- **Contract ownership** (from Dossier B): each component declares only its own outward-facing contract (owned acceptance criteria); a consuming component's edge references the provider's owned criteria rather than restating them.
- **Migration target** (superseding Dossier B's original Presentation-System-first pilot): per this merge's directing instructions, the first migration target is **the spec system's own specification** — the new system must be able to describe itself before any other spec migrates into it. The Presentation System spec remains the illustrative case study for the monolithic-shape problem and a candidate for a *later* migration, not the first pilot.

## Validation Gates

- `cargo test -p spec-api --test schema_test` — must keep passing as the schema gains new artifact kinds.
- `./target/debug/spec.exe health --all` — structural health check across all specs, including the new component/criterion/evidence/contract-edge specs.
- A representative two-component cycle example (each serving and consuming the other) assigns every criterion, provider obligation, and consumer claim exactly once under the adopted ownership rule.
- User review sign-off recorded (e.g. via a Review Agent pass or explicit approval in the owning ticket/spec) before Waypoint 5's specs are treated as implementation-ready.
- `npm run build`/`npm run dev` in `.presentation/` succeeds and the new cycle diagram renders — manual visual check in an external fullscreen browser per `AGENTS.md`'s browser-verification rule.
- Post-migration: `./target/debug/spec.exe get <spec-system-spec-id> --json` and `spec.exe health --all` both pass against the migrated spec-system spec.
- Standard ticket quality gates (`.agents/instructions/ticket/lifecycle.instructions.md`'s Review Gate) for every ticket created in Waypoints 3, 4, 6, and 7.

## Roadmap Waypoints

1. **[Single-session] Document the closed-loop cycle as a named instruction file.** New file (suggested: `.agents/instructions/core-cycle.instructions.md`) stating the 7-step cycle (request → spec → tickets → tests → implementation → validated response → next iteration), folding in the test-evidence cross-link (test-api already links `spec_ids`/`ticket_ids`/`acceptance_criterion_ids`). One cross-reference line added to `AGENTS.md`. No code changes. *(From Dossier A waypoints 1 and 4, merged — the test-evidence note was always meant to live inside this file, not stand alone.)*
2. **[Single-session] Finalize the target artifact contract model.** Confirm the component/criterion/evidence-reference/directed-contract-edge shapes and the resolved ownership decision above cover the current spec-api manifest fields without contradiction. Validation: the two-component cycle check. *(From Dossier B waypoint 1.)*
3. **[Ticket] Specify the full workflow cycle's components in the current spec system.** Using the current (old) spec tooling as dogmatically as the model from Waypoint 2 allows, author spec entities for each cycle component named in Waypoint 1 (request, spec, ticket, test, implementation, validated response, next iteration), including their contract edges to one another. Depends on Waypoints 1-2.
4. **[Ticket] Specify the new spec system itself, and adjacent tooling, in the current spec system.** Author specs for: the new spec system's own components (component, criterion, evidence-reference, contract-edge artifacts, per Waypoint 2's model), and specs for adjacent tooling — tickets, docs, and tests — as components with their own contract edges into/out of the spec system. Depends on Waypoint 2; can run in parallel with Waypoint 3.
5. **[Review gate, not a ticket] Review the specs from Waypoints 3-4 with the user until satisfactory.** Iterate specs until the user confirms readiness; do not proceed to Waypoint 6 while an open question remains. Depends on Waypoints 3-4.
6. **[Ticket, likely an epic with sub-tickets] Create implementation tickets for the new spec system and adjacent tooling.** Covers both tests and production code for the new spec system (component/criterion/evidence/contract-edge storage, schema, and API surface) and any adjacent-tooling changes the Waypoint 4 specs require. Size and split per `AGENTS.md`'s Task Routing once the reviewed specs (Waypoint 5) are final. Depends on Waypoint 5.
7. **[Ticket] Migrate the spec system's own specification to the new format.** Dogfood the newly implemented system (Waypoint 6) by migrating its own governing spec into the new component/criterion/evidence/contract-edge shape; preserve the legacy spec until the migrated result passes health and traceability validation. Depends on Waypoint 6.
8. **[Single-session] Update the presentation deck with a diagram of the full workflow cycle.** Add the cycle diagram to the root `.presentation/` (`id = "context-engine"`) deck, coordinating with the existing [2ccde9ee Presentation System](../../.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/spec.toml) spec and [0ee95228 epic](../../.ticket/tickets/0ee95228-475d-4706-a108-fd208f7c4098/ticket.toml) rather than bypassing them. Depends on Waypoint 3 (the cycle's component shape must be settled first); does not need to wait for Waypoints 6-7.
9. **[Single-session] Update agent guidance files for the new spec system and full cycle.** Extend `AGENTS.md` and the relevant `.agents/instructions/` files to teach agents how to author and consume specs under the new system, and to reference the full cycle from Waypoint 1. Depends on Waypoint 6 (guidance should describe the real, implemented system).
10. **[Ticket, not yet created — see Active Blockers] Ticket-depends-on-spec gating edge.** Genuinely distinct architecture-level `ticket-api` change (see Duplication-Review Consolidation Verdict above): a ticket can depend on/fulfill a spec as a gating relationship, not just an informational `[[refs]]` pointer. Create the ticket when this waypoint is picked up — the id `5b50329b` both source dossiers cite does not resolve in the current ticket store. No dependency on the other waypoints; can be picked up independently at any point.

## Heads-up Notes

- The mechanical pieces most of this roadmap builds on (spec store, ticket store, `[[refs]]`, test-api's `spec_ids`/`ticket_ids` linkage) **already exist** — Waypoints 1-2 are naming/modeling, not new capability; the new capability starts at Waypoint 6.
- `[[refs]]` (`kind = spec`) and the observed `spec_refs` field already let a ticket point at a spec today, but neither gates readiness — do not confuse this with Waypoint 10's proposed gating edge.
- The Presentation System spec ([2ccde9ee](../../.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/spec.toml)) is the illustrative case study throughout, but the first real migration target is the spec system's own specification (Waypoint 7) — do not default back to Dossier B's original Presentation-System-first plan.
- The root `.presentation/` deck composes `workflow-tools`'s deck (`composes = ["workflow-tools"]`); Waypoint 8 scopes the diagram to the composing (root) deck, matching the original transcript's "our complete cycle" framing.
- A prior dossier at [transcripts/20-08-2026_presentation-automation-planning/](../20-08-2026_presentation-automation-planning/) already tracks presentation-system work via [2ccde9ee](../../.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/spec.toml) and [0ee95228](../../.ticket/tickets/0ee95228-475d-4706-a108-fd208f7c4098/ticket.toml) — Waypoint 8 extends that tracked system rather than editing the deck ad hoc.
- Waypoints 3, 4, 6, and 7 are deliberately not decomposed further inline — each spans multiple sessions and/or ambiguous internal sub-dependencies; ticket creation at pickup time is where that decomposition happens, per the ticket-creation exception this pipeline allows during roadmap compilation.
- This dossier (including this merge) does not itself create the Waypoint 3/4/6/7 tickets, and does not create or edit any spec. It compiles the route; `/tickets` and `/spec` (and, for Waypoint 7, `/spec` again against the new system) are the separate, later steps that execute it.
