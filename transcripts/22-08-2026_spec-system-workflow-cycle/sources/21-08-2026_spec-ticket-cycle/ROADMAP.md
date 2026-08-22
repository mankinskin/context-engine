# Roadmap — Closed-Loop Production-Workflow Cycle

## Relevant Artifact IDs

- Ticket: [5b50329b Ticket-depends-on-spec gating edge](../../.ticket/tickets/5b50329b-59f3-4a6f-a90e-cbacefdcce48) — the one oversized work package, tracked separately per the pipeline's ticket-creation exception.
- Instruction files to cross-link (not restate): [AGENTS.md](../../AGENTS.md), [workflow.instructions.md](../../.agents/instructions/ticket/workflow.instructions.md), [lifecycle.instructions.md](../../.agents/instructions/ticket/lifecycle.instructions.md), [phase-separation.instructions.md](../../.agents/instructions/orchestration/phase-separation.instructions.md), [escalation-gate.instructions.md](../../.agents/instructions/orchestration/escalation-gate.instructions.md).
- Deck: [.presentation/deck.toml](../../.presentation/deck.toml) (`id = "context-engine"`, the correct repo-wide deck).
- Existing presentation-system spec/epic to coordinate with, not duplicate: [2ccde9ee Presentation System](../../.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/spec.toml), [0ee95228 presentation epic](../../.ticket/tickets/0ee95228-475d-4706-a108-fd208f7c4098/ticket.toml), from [transcripts/20-08-2026_presentation-automation-planning/ROADMAP.md](../20-08-2026_presentation-automation-planning/ROADMAP.md).

## Active Blockers

None. Both open questions from the first review loop (which deck; whether the ticket-spec gating capability already exists) resolved from repository evidence — no human-judgment blocker remains.

## Validation Gates

- Task 1 (instruction file): manual read-through confirming no restated mechanics (each cycle step links out rather than duplicates prose).
- Task 2 (presentation slide): `npm run build` in `.presentation/` succeeds and the new slide renders (manual visual check, external browser, per `AGENTS.md`'s browser-verification rule if the deck is a rendered web surface).
- Task 3 (ticket): no validation here — the ticket's own acceptance criteria are defined when it is refined.
- Task 4 (test-evidence cross-link): manual read-through confirming the linked tool names (`mcp_test-mcp_record_execution`, `mcp_test-mcp_record_spec`) are current and accurate.

## Full Roadmap

1. **[Single-session] Write the closed-loop cycle instruction file.** New file (suggested path: `.agents/instructions/core-cycle.instructions.md`) stating the 7-step cycle from [01-document-closed-loop-cycle.md](01-document-closed-loop-cycle.md), folding in the test-evidence cross-link from [04-test-evidence-link.md](04-test-evidence-link.md). Add one cross-reference line from `AGENTS.md`. No code changes.
2. **[Single-session] Add the cycle to the `.presentation/` deck.** Per [02-presentation-deck-slide.md](02-presentation-deck-slide.md); depends on Task 1 existing so the slide can cite it, but is otherwise independent — can run in parallel with Task 1 once Task 1's content outline is settled. Coordinate with the existing [2ccde9ee Presentation System](../../.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/spec.toml) spec and its [0ee95228 epic](../../.ticket/tickets/0ee95228-475d-4706-a108-fd208f7c4098/ticket.toml) — add the slide as content within that system rather than as an ad hoc, untracked deck edit.
3. **[Ticket, already created] Ticket-depends-on-spec gating edge.** [5b50329b](../../.ticket/tickets/5b50329b-59f3-4a6f-a90e-cbacefdcce48) — per [03-ticket-spec-gating-edge.md](03-ticket-spec-gating-edge.md). Architecture-level `ticket-api` change; not implemented in this pass. No dependency on Tasks 1/2 — can be picked up independently, though its resolution should eventually be reflected back into Task 1's cycle description once the edge kind exists.

## Heads-Up Notes

- The mechanical pieces of this cycle (spec store, ticket store, `[[refs]]`, test-api's `spec_ids`/`ticket_ids` linkage) **already exist** — this dossier is primarily a naming/documentation/presentation exercise, plus one genuinely new architecture proposal (Task 3).
- `[[refs]]` (`kind = spec`) and an observed `spec_refs` field already let a ticket point at a spec today, but neither gates readiness — do not confuse this existing informational link with the gating relationship Task 3 proposes.
- The root `.presentation/` deck composes `workflow-tools`'s deck (`composes = ["workflow-tools"]`) — check whether the cycle content belongs at the composing level (repo-wide) or should also be echoed in a composed sub-deck; this dossier scopes it to the composing (root) deck only per the transcript's "our complete cycle" framing.
- A prior dossier at [transcripts/20-08-2026_presentation-automation-planning/](../20-08-2026_presentation-automation-planning/) already tracks presentation-system work via a real spec ([2ccde9ee](../../.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/spec.toml)) and epic ([0ee95228](../../.ticket/tickets/0ee95228-475d-4706-a108-fd208f7c4098/ticket.toml)) with two phase tickets already underway — Task 2 should extend that tracked system, not bypass it with an untracked direct deck edit.
