<!-- aligned-structure:v2 -->

# Ticket Store Integration

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/ticket_ref.rs](workflow-tools/spec/crates/spec-api/src/ticket_ref.rs) defines explicit target identity; [workflow-tools/spec/src/cli/commands/validate_links.rs](workflow-tools/spec/src/cli/commands/validate_links.rs) validates current bidirectional ticket/spec references.

## Naming Conventions

Use `TicketRef` for a governing target, `SpecificationGate` for the ticket-side typed record, and `ticket-` criterion ids. This child owns `ticket-governing-spec`, `ticket-explicit-spec-target`, `ticket-spec-before-plan`, `ticket-bidirectional-governance`, and `edge-spec-consumes-ticket-gate`.

## Reading Order

1. [90e4fb79 Production Workflow Cycle](.spec/specs/90e4fb79-2c60-42a6-ab10-91d243693150/body.md) - sequencing provider.
2. [a608f774 Specification Root Contract](.spec/specs/a608f774-9f50-4de1-90e2-ffeefb0198b1/body.md) - governed root consumer.
3. [workflow-tools/spec/src/cli/commands/validate_links.rs](workflow-tools/spec/src/cli/commands/validate_links.rs) - current link validation.

## Responsibility

If implemented, governed work can be planned only after it explicitly identifies
the specification that defines its goal and acceptance criteria.

## Interfaces And Dependencies

The governing specification persists a distinct relation, for example:

```toml
[[governs_ticket]]
ticket_id = "<ticket-id>"
workspace = "<workspace>"
store_root = ".ticket"
```

The ticket store persists a distinct typed `SpecificationGate` record resolving
that specification. Neither side uses generic `related_specs` semantics.

## Behavior

- `ticket-governing-spec`: governed work references a governing spec as a readiness gate, not informational metadata.
- `ticket-explicit-spec-target`: identify the target without implicit resolution.
- `ticket-spec-before-plan`: create a ticket after, never instead of, its spec.
- `ticket-bidirectional-governance`: require the governing spec's `governs_ticket` relation and the ticket's typed gate record to resolve to each other.
- `edge-spec-consumes-ticket-gate`: Specification Root consumes all three ticket criteria.

## Boundaries And Failure Cases

This child neither authors requirements nor permits an ungoverned ticket to claim
governed work. The pair is not a `ComponentContractEdge`; legacy generic forms
are detect-and-report only and must never silently infer the typed relation.
Missing target identity, wrong store root, missing reverse typed gate, or
pre-spec planning is invalid.

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

## Open Decisions

G1 (owning component: `f482eb83-5b47-4ea3-8d5b-b7baa0531333`): must a gate outcome auto-transition the governed ticket? Options: no automatic transition; passing gate closes ticket; explicit state mapping. Recommended pending answer: no automatic transition; passed authorizes the normal review-controlled transition, while failed and blocked remain unsatisfied.

G2 (owning components: `f482eb83-5b47-4ea3-8d5b-b7baa0531333` and [83c0b9c4 Validation Observation Contract](.spec/specs/83c0b9c4-1617-4751-af23-57811060f0fb/body.md)): when validation reruns, which execution is authoritative? Options: the gate explicitly selects an execution; latest wins; first pass remains. Recommended pending answer: an explicit execution pointer, updated on rerun while test-api retains full history.
