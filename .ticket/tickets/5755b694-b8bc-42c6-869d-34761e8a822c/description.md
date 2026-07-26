## Goal

Author the **iteration-loop workflow spec**: the phase model, canonical ordering, and gates that govern how a finished implementation becomes the next handoff.

## Content

- **Phase model**: implementation (self-contained) → transition (Iteration Agent) → next implementation.
- **Canonical order**: Review → Interview → Commit → Handoff (only approved work committed).
- **Gates**:
  - Review gate: acceptance criteria verified before commit; failures return ticket to `in-implementation`.
  - Escalation gate: no ticket reaches `done` with an unresolved user escalation.
  - Loop-closure gate: every finished implementation terminates in a durable handoff package + a ticket transition (closed or returned).
- **Re-packaging rule**: a returned (failed-review) ticket is immediately re-packaged into the next handoff.
- **Roles**: which existing agent owns each phase (Review, Interview, Commit, Handoff) and what the Iteration Agent adds (sequencing + next-handoff authoring).

## Acceptance criteria

- Spec exists describing phases, ordering, and the three gates.
- Spec cross-links the Iteration Agent ticket (T1), the handoff-package schema spec (T2), and the durable rules (T5).
- Spec is linked to this ticket and to the epic.