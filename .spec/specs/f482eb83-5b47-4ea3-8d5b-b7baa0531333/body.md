<!-- aligned-structure:v2 -->

# Ticket Store Integration

## Responsibility And Interface

Plan governed implementation work only after a specification exists. Ticket
references identify the target explicitly, matching the `TicketRef` workspace
and store-root rule in `workflow-tools/spec/crates/spec-api/src/ticket_ref.rs`.

## Behavior And Contract

- `ticket-governing-spec`: governed work references a governing spec as a
	readiness gate, not informational metadata.
- `ticket-explicit-spec-target`: identify the target without implicit resolution.
- `ticket-spec-before-plan`: create a ticket after, never instead of, its spec.
- `edge-spec-consumes-ticket-gate`: Specification Root consumes all three criteria.

## Boundaries And Failure Cases

This child neither authors requirements nor permits an ungoverned ticket to
claim governed work. Missing target identity or pre-spec planning is invalid.

## Acceptance Evidence And Position

Add focused ticket/spec relationship tests under the later reviewed ticket;
the related workflow-cycle root `90e4fb79-2c60-42a6-ab10-91d243693150` supplies
the implemented sequencing rule. No implementation ticket exists yet.
