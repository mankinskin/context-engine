<!-- aligned-structure:v2 -->

# Ticket Store Integration

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/ticket_ref.rs](workflow-tools/spec/crates/spec-api/src/ticket_ref.rs) defines explicit target identity; [workflow-tools/spec/src/cli/commands/validate_links.rs](workflow-tools/spec/src/cli/commands/validate_links.rs) validates current bidirectional ticket/spec references.

## Naming Conventions

Use `TicketRef` for a governing target and `ticket-` criterion ids. This child owns `ticket-governing-spec`, `ticket-explicit-spec-target`, `ticket-spec-before-plan`, and `edge-spec-consumes-ticket-gate`.

## Reading Order

1. [90e4fb79 Production Workflow Cycle](.spec/specs/90e4fb79-2c60-42a6-ab10-91d243693150/body.md) - sequencing provider.
2. [a608f774 Specification Root Contract](.spec/specs/a608f774-9f50-4de1-90e2-ffeefb0198b1/body.md) - governed root consumer.
3. [workflow-tools/spec/src/cli/commands/validate_links.rs](workflow-tools/spec/src/cli/commands/validate_links.rs) - current link validation.

## Responsibility

If implemented, governed work can be planned only after it explicitly identifies
the specification that defines its goal and acceptance criteria.

## Interfaces And Dependencies

`TicketRef` carries ticket UUID, workspace, and repo-root-relative store root;
the reverse ticket `related_specs` link must resolve to the same root.

## Behavior

- `ticket-governing-spec`: governed work references a governing spec as a readiness gate, not informational metadata.
- `ticket-explicit-spec-target`: identify the target without implicit resolution.
- `ticket-spec-before-plan`: create a ticket after, never instead of, its spec.
- `edge-spec-consumes-ticket-gate`: Specification Root consumes all three ticket criteria.

## Boundaries And Failure Cases

This child neither authors requirements nor permits an ungoverned ticket to claim
governed work. Missing target identity, wrong store root, missing reverse link,
or pre-spec planning is invalid.

## Provider/Consumer Contract

Provides `ticket-governing-spec`, `ticket-explicit-spec-target`, and `ticket-spec-before-plan` to [a608f774 Specification Root Contract](.spec/specs/a608f774-9f50-4de1-90e2-ffeefb0198b1/body.md) through `edge-spec-consumes-ticket-gate`; consumes [90e4fb79 Production Workflow Cycle](.spec/specs/90e4fb79-2c60-42a6-ab10-91d243693150/body.md) sequencing.

## Examples

An implementation ticket records a `TicketRef` for its governing spec before
planning begins; `validate-links` rejects it if the referenced ticket does not
link back to that spec or its declared `.ticket` store is wrong.

## Evidence

Position: `partial`; explicit `TicketRef` and bidirectional validation exist, while the workflow gate remains draft. Planned focused ticket/spec relationship tests under a later reviewed ticket; no implementation ticket is linked now.

## Scope

Owns governing ticket linkage only; it does not create tickets or modify this draft's state.
