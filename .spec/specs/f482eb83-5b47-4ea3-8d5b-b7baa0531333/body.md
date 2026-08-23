<!-- aligned-structure:v2 -->

# Ticket Store Integration

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/ticket_ref.rs](../../../workflow-tools/spec/crates/spec-api/src/ticket_ref.rs) defines explicit target identity; [workflow-tools/spec/src/cli/commands/validate_links.rs](../../../workflow-tools/spec/src/cli/commands/validate_links.rs) validates current bidirectional ticket/spec references.

## Naming Conventions

Use `TicketRef` for a governing component-spec target, `SpecificationGate` for the ticket-side typed record, and `ticket-` criterion ids. This child owns `ticket-governing-spec`, `ticket-explicit-spec-target`, `ticket-spec-before-plan`, `ticket-bidirectional-governance`, `ticket-gate-lifecycle`, `ticket-governance-recovery`, and `edge-spec-consumes-ticket-gate`.

## Reading Order

1. [90e4fb79 Production Workflow Cycle](../90e4fb79-2c60-42a6-ab10-91d243693150/body.md) - sequencing provider.
2. [a608f774 Specification Root Contract](../a608f774-9f50-4de1-90e2-ffeefb0198b1/body.md) - governed root consumer.
3. [workflow-tools/spec/src/cli/commands/validate_links.rs](../../../workflow-tools/spec/src/cli/commands/validate_links.rs) - current link validation.

## Responsibility

If implemented, governed work can be planned only after it explicitly identifies
the specification that defines its goal and acceptance criteria.

## Interfaces And Dependencies

The governing component specification persists a distinct relation, for example:

```toml
[[governs_ticket]]
ticket_id = "<ticket-id>"
workspace = "<workspace>"
store_root = ".ticket"
```

The ticket store persists a distinct typed `SpecificationGate` record resolving
that specification. Neither side uses generic `related_specs` semantics.

This governing relation gates ticket goal and acceptance readiness only. It is
not component containment through `parent` and is not a provider/consumer edge.

## Behavior

- `ticket-governing-spec`: governed work references a governing spec as a readiness gate, not informational metadata.
- `ticket-explicit-spec-target`: identify the target without implicit resolution.
- `ticket-spec-before-plan`: create a ticket after, never instead of, its spec.
- `ticket-bidirectional-governance`: require the governing spec's `governs_ticket` relation and the ticket's typed gate record to resolve to each other.
- `ticket-gate-lifecycle`: ticket-gate evaluation queries test-api executions by `validation_spec_id`, then consumer-side filters each `execution.links.acceptance_criterion_ids` for the criterion id. No first-class criterion query or index, ticket id, governing-spec id, or `.test` store identity participates. With several matches, newest `executed_at` wins and deterministic id ordering resolves equal timestamps; absent matching execution is unsatisfied `pending`, `passed` satisfies, and `failed` or `blocked` revokes. A validation specification and criterion shared across tickets intentionally makes every matching gate observe the same execution outcome. Gate outcomes never transition ticket lifecycle state; the existing review-controlled workflow alone does so.
- `ticket-governance-recovery`: cross-store governance consumes the reusable shared operation journal prerequisite from [55d8f2eb Specification Store Contract](../55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md). It records planned and inverse writes, locks, collisions, and recovery state to expose resume or rollback; it never silently repairs ticket/spec divergence and does not require a global transaction.
- `edge-spec-consumes-ticket-gate`: Specification Root consumes all three ticket criteria.

## Boundaries And Failure Cases

This child neither authors requirements nor permits an ungoverned ticket to claim
governed work. The pair is not a `ComponentContractEdge`; legacy generic forms
are detect-and-report only and must never silently infer the typed relation.
Missing target identity, wrong store root, missing reverse typed gate,
pre-spec planning, or a cross-store interruption silently repaired without an
explicit resume/rollback choice is invalid. A recovered operation reports its
status and preserves the original divergent state until the selected recovery
action runs. The validation match predicate does not gain a first-class
criterion query/index, ticket, governing-spec, or test-store qualifier.

## Provider/Consumer Contract

Provides `ticket-governing-spec`, `ticket-explicit-spec-target`, and `ticket-spec-before-plan` to [a608f774 Specification Root Contract](../a608f774-9f50-4de1-90e2-ffeefb0198b1/body.md) through `edge-spec-consumes-ticket-gate`; consumes [90e4fb79 Production Workflow Cycle](../90e4fb79-2c60-42a6-ab10-91d243693150/body.md) sequencing.

## Examples

An implementation ticket records a `TicketRef` for its governing spec before
planning begins; `validate-links` rejects it if the referenced ticket does not
link back to that spec or its declared `.ticket` store is wrong.

A gate shared by two tickets queries executions by `validation_spec_id` and
consumer-side filters `execution.links.acceptance_criterion_ids` for its shared
criterion id. One newest `passed` execution satisfies both gates, and a newer
`failed` or `blocked` execution revokes both; equal timestamps use deterministic
id ordering. This shared outcome is intentional, and neither ticket changes
lifecycle state until the normal review-controlled workflow transitions it.

If writing the governing spec relation succeeds but the ticket-side gate write
is interrupted, the operation reports recoverable drift. Resume completes the
recorded counterpart write; rollback removes the recorded first write. Neither
store is silently changed merely because the mismatch is observed.

## Evidence

Position: `partial`; explicit `TicketRef` and bidirectional validation exist, while the workflow gate and journaled cross-store recovery are specified-but-not-built. Planned focused ticket/spec relationship, interruption, recoverable-drift, resume, and rollback tests under a later reviewed ticket; no implementation ticket is linked now.

## Scope

Owns governing ticket linkage only; it does not create tickets or modify this draft's state.
