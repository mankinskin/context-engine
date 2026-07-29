## Purpose

Define the **handoff package**: the required fields that make the next implementation session fully self-contained — zero discovery, zero user clarification. A handoff package is produced by the transition phase (Handoff Agent) and consumed by the next implementation phase (Implement Agent). It extends the existing `session_handoff` record schema and is supplied alongside required handoff-record parameters.

## Required fields

- **objective** — the single goal of the next implementation unit.
- **target_tickets** — ticket ids with current state and acceptance criteria inlined (not referenced-by-lookup).
- **target_files** — explicit workspace-relative paths expected to be touched.
- **decisions** — resolved design choices, so implementation does not re-decide.
- **non_goals** — explicit out-of-scope boundaries.
- **context_anchors** — prior findings, links, and ids needed so no search is required. Each anchor MUST carry store-qualified physical paths when referencing entities in nested stores (e.g., `memory-api/.ticket/tickets/<uuid>`, not just the root `.ticket/`). This ensures the next session can resolve cross-store references without discovery or guessing.
- **open_escalations** — must be empty for a package to be implementation-ready.

## Required handoff-record parameters

- **validation** — exact commands / checks that prove the unit done. This is a required parameter supplied alongside `SessionHandoffPackage` when creating the handoff record, not a field on the package struct.

## Optional fields

- **risk_notes** — known risks or fragile areas.
- **predecessor_handoff** — id of the handoff this one supersedes, for lineage. **Superseded**: per spec c737328d (Session merge and pickup), this field is removed. A handoff record instead becomes the provenance edge itself — target-less and claimable at creation, with `target_session_id` bound at pickup time (`SessionRecord.picked_up_handoff_ids` / `emitted_handoff_ids` carry the lineage that `predecessor_handoff` used to approximate). Do not add new usages of `predecessor_handoff`; see c737328d R1/R2/R7 for the replacement model.

## Enforcement

- `session_handoff` produces / validates a record that carries every required package field plus required handoff-record parameters. A package missing a required package field (or with a non-empty `open_escalations`) is rejected or flagged as not implementation-ready.
- The Implement Agent treats the package as its sole input. If a required field is missing at consume time, it escalates rather than searching or asking the user.

## Readiness rule

A handoff package is **implementation-ready** only when all required package fields are present and `open_escalations` is empty.

## Storage / durable home

The `session_handoff` record is the **source of truth** for a produced package and its required companion parameters. It is additionally **mirrored onto the target ticket** as a field/artifact so the next implementation session can load the package cold from the ticket store without a live session. The ticket mirror and the `session_handoff` record must stay consistent; the mirror carries at least `objective`, `target_tickets`, `target_files`, `validation`, and `open_escalations`. Whether this mirror becomes authoritative once handoffs are provenance edges is an open question tracked in spec c737328d, not decided here.

## Rendered markdown

- The rendered handoff markdown document (`handoff.md`) MUST include, in its `## Workflow` section, a fenced ```mermaid block containing a `flowchart TD` diagram of the handoff's workflow graph (nodes + edges from the workflow snapshot), in addition to the existing node/edge/not-done counts.
- The diagram is omitted only when the workflow graph has no nodes.
- The diagram is rendered from the handoff record's own workflow snapshot, not hand-authored, so it stays consistent with `handoff.json`.

## Related

- Iteration loop workflow spec (phases, ordering, gates).
- Phase-separation rule (implementation must not search or clarify).
- Loop-closure and escalation-gate rules.
- [c737328d Session merge and pickup: handoff-edge provenance graph and first-class tracks](../c737328d-a97e-4250-bf9a-390224ab57fd/spec.toml) — supersedes `predecessor_handoff` with the binary provenance edge model; owns the target-binding-at-pickup and unclaimed-handoff-backlog requirements.

## Related Tickets

- [d3af78d7 (existing linked ticket)]
- [fb14754e Carry verified physical repo paths in handoff packages](.ticket/tickets/fb14754e-2be8-40a5-a995-488842ba6367/ticket.toml)
- [d28afbc0 [session-api] Session merge and pickup: handoff-edge provenance graph and first-class tracks](.ticket/tickets/d28afbc0-9d16-4494-8ca5-4154f3ace9be/ticket.toml) — epic that removes `predecessor_handoff` per the superseded note above.
- [e4f84414 Render workflow mermaid graph in handoff markdown](memory-api/.ticket/tickets/e4f84414-ef2e-4012-9cfe-da08fe2c077c/ticket.toml) — adds the rendered mermaid workflow-graph requirement above.
