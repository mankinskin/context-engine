## Purpose

Define the **handoff package**: the required fields that make the next implementation session fully self-contained — zero discovery, zero user clarification. A handoff package is produced by the transition phase (Handoff Agent) and consumed by the next implementation phase (Implement Agent). It extends the existing `session_handoff` record schema and is supplied alongside required handoff-record parameters.

## Required fields

- **objective** — the single goal of the next implementation unit.
- **higher_level_objective** — required prose explaining why the current implementation unit matters now; it captures rationale that no graph edge can express.
- **upward_context** — a required ordered ancestor chain from the higher-level program context to the current leaf work. Each entry MUST carry an entity URN, a human-readable title, and a role such as epic, phase, or parent.
- **target_tickets** — structured ticket entries carrying at least the ticket id and the author's why this ticket belongs in the handoff. Each entry MUST retain the ticket's current state and acceptance criteria inlined (not referenced-by-lookup).
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
- When `objective` is non-empty and `open_escalations` is empty, absence of `higher_level_objective` or `upward_context` MUST reject handoff creation; when either derived-readiness condition is false, the same absence MUST produce a warning and the handoff MUST persist.
- Existing persisted `handoff.json` records that encode `target_tickets` as plain ticket-id strings MUST remain deserializable after structured ticket entries are introduced.
- The Implement Agent treats the package as its sole input. If a required field is missing at consume time, it escalates rather than searching or asking the user.

## Readiness rule

A handoff package's derived **implementation-ready** status is true only when `objective` is non-empty and `open_escalations` is empty. For a package with derived implementation-ready status, `higher_level_objective` and `upward_context` are mandatory creation-time requirements; their absence rejects creation. For a package without derived implementation-ready status, their absence emits warnings and does not prevent persistence.

## Storage / durable home

The `session_handoff` record is the **source of truth** for a produced package and its required companion parameters. It is additionally **mirrored onto the target ticket** as a field/artifact so the next implementation session can load the package cold from the ticket store without a live session. The ticket mirror and the `session_handoff` record must stay consistent; the mirror carries at least `objective`, `target_tickets`, `target_files`, `validation`, and `open_escalations`. Whether this mirror becomes authoritative once handoffs are provenance edges is an open question tracked in spec c737328d, not decided here.

## Rendered markdown

- The rendered handoff markdown document (`handoff.md`) MUST open with the package's `higher_level_objective`, stated before the implementation-unit details.
- The rendered handoff markdown document (`handoff.md`) MUST include an upward-context breadcrumb that presents the ordered ancestor chain from epic to phase to leaf work, including each ancestor's role, human-readable title, and clickable entity reference.
- The rendered handoff markdown document (`handoff.md`) MUST include a per-ticket table. Each row MUST contain a clickable ticket reference with a real title, ticket-store-resolved what-it-does narrative, and the author-supplied why the ticket belongs in the handoff.
- Ticket titles and what-it-does narratives in the per-ticket table are resolved from the ticket store at render time; the why text is authored in the handoff record.
- Every ticket reference in the rendered form MUST follow the repository Clickable Reference Policy: a forward-slash repo-root-relative markdown link with link text `{short-id} {title}`, never a backtick-wrapped bare ticket id. When a ticket cannot be resolved, rendering MUST degrade gracefully to its id and any cached title without panicking or failing handoff creation.
- The high-level goal, upward-context breadcrumb, and ticket table are rendered from the handoff record's own package data and ticket-store resolution, not hand-authored, so they stay consistent with `handoff.json` and survive regeneration.
- The rendered handoff markdown document (`handoff.md`) MUST include, in its `## Workflow` section, a fenced ```mermaid block containing a `flowchart TD` diagram of the handoff's workflow graph (nodes + edges from the workflow snapshot), in addition to the existing node/edge/not-done counts.
- The diagram is omitted only when the workflow graph has no nodes.
- The diagram is rendered from the handoff record's own workflow snapshot, not hand-authored, so it stays consistent with `handoff.json`.

## Concise summaries and validation (amendment)

The following clarifications and requirements are added to the Handoff Package Schema to support implementation-ready handoffs that carry durable, rendered, concise summaries for target tickets and sessions:

1. Target-ticket and session summaries MAY be author-supplied and MUST have generated defaults. An author can override the generated defaults at creation time.

2. The schema enforces no arbitrary character limits on these summaries; authoring guidance requires concision in intent but not by enforced truncation.

3. The `target_ticket.summary` and `session_summary` fields are REQUIRED only when the handoff's derived `implementation-ready` status is true. For exploratory/non-implementation handoffs the existing permissive behavior remains: summaries are optional and may be empty.

4. Each `target_ticket` entry in the package MUST include, inline, the ticket's durable `current_state` and `acceptance_criteria` (copied at creation time), plus the concise `summary`, the `title/reference` (short-id + title), and an author-provided `why` explaining why the ticket is in-scope.

5. A `session_summary` field is durable and persisted on the handoff record and MUST be rendered in the produced `handoff.md`. The session summary covers completed work, validation results, and remaining risks/blockers.

6. Rendered markdown MUST render the persisted concise `target_ticket.summary` and `session_summary` as-is; rendering MUST NOT fetch or inline a full ticket objective/body text. Ticket-store lookups at render time are limited to resolving `title`/`reference` for the clickable links only.

7. Legacy stored handoff formats and legacy `target_tickets` encodings (for example a plain list of ticket ids) MUST remain readable and deserializable by the schema migration paths.

8. Validation requirements are updated to include checks that cover:
	- JSON round-trip serialization/deserialization for the record and package fields,
	- legacy compatibility (old encodings still parse),
	- presence and rendering of the concise summaries when required,
	- absence of full objective/body text in rendered summary areas,
	- rejection of creation when implementation-ready flags are set but required summary fields are missing,
	- permissive exploratory behavior when not implementation-ready,
	- rendering the `session_summary` in `handoff.md` and ensuring it reflects persisted content and validation results.

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
- [25b5f3e7 [session-api][handoff] Make upward context and ticket narrative reproducible in handoff markdown](.ticket/tickets/25b5f3e7-cace-4822-a955-bc2e3202be77/ticket.toml)
- [742dbc65 [session-api][handoff] Model and enforce upward context for implementation-ready handoffs](.ticket/tickets/742dbc65-a100-4278-9274-7d99a3e2afc4/ticket.toml)
- [ba8f5528 [session-api][handoff] Render resolved ticket narrative and upward context in handoff markdown](.ticket/tickets/ba8f5528-5af3-4de2-8904-442a4691854a/ticket.toml)
