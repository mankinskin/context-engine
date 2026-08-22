# 03 — Ticket-Depends-On-Spec Gating Edge (Ticket Recommendation, Not Implemented Here)

## Outcome

A tracked ticket exists proposing that a ticket can depend on/fulfill a spec as a **gating** relationship — analogous to today's ticket-to-ticket `depends_on` — instead of specs being only an informational `[[refs]]`/`spec_refs` pointer.

## Description

The transcript's core architectural claim: "tickets should depend on specifications, and tickets should be able to close, fulfill, or implement specifications, instead of the current model where one ticket depends on another ticket and the first ticket must be closed to unblock the next one." Research (`ARTIFACTS.md`) confirms this gating relationship does not exist today: `[[refs]]` (`kind = spec`) and the observed `spec_refs` field are informational only and do not block ticket actionability or `next_tickets` selection the way `depends_on` does.

This is an architecture-level change to `ticket-api`'s edge/gating model (schema, `next_tickets` readiness logic, possibly a new edge kind such as `fulfills_spec` or a `blocked_by_spec_state` semantic) — too large and too risky to implement inline in this dossier, per the pipeline's own decision boundary (no code changes; oversized work becomes a ticket, not an inline task).

## Non-Goal

This work package does not implement the change. It does not decide the exact edge-kind name, schema shape, or gating semantics — those are the ticket's own design questions, to be resolved when that ticket is picked up (likely via `ticket-refinement.agent.md`'s own evidence-grounded refinement loop).

## Validation Method

N/A for this dossier — the validation gate here is that the recommended ticket, once created, is linked from `ROADMAP.md`'s artifact list. The ticket's own acceptance criteria and tests are defined when it is refined.
