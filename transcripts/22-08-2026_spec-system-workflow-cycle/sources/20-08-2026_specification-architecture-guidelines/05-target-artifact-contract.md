# 05 - Target Artifact Contract (Priority 1)

Source: `transcripts/20-08-2026_specification-architecture-guidelines/merged.clean.md`.

This document defines the semantics from Priority 1 of `03-migration-pilot-roadmap.md`. It is a model proposal for review, not an implementation and not a change to any existing spec.

## Component Artifact

A component artifact records:

- `id` - stable identifier.
- `title` - short name.
- `purpose` - one short, human-readable statement of the component's core task.
- `context` - brief note on where/how the component is used.
- `owns_criteria` - acceptance criteria this component is responsible for satisfying.
- `related_specs` - links to other specifications describing related components.
- `related_evidence` - links to specification-external artifacts (tests, tickets, docs).

## Acceptance Criterion Artifact

- `id` - stable identifier, referenced by contract edges.
- `statement` - short, measurable claim.
- `validated_by` - one or more test or validation-execution references.
- `owning_component` - the component whose fulfillment this criterion measures.

## External Evidence Reference

- `target_kind` - `test | ticket | doc | other`.
- `target_ref` - concrete identifier or path.
- `relation` - short note on why this evidence grounds the component.

## Contract Edge (directed)

- `from` - consuming/reading component.
- `to` - serving/writing component.
- `criteria` - acceptance criteria on `to` that must be satisfied for `from` to treat the contract as fulfilled.

Edges are directed but the graph is not required to be acyclic: two components may hold a `from -> to` edge and a separate `to -> from` edge, each with its own criteria, when they mutually serve and consume each other.

## Open Ownership Decision (still unresolved)

The merged source repeats one unresolved question: should a component declare only the expectations (edges) it has of the components it consumes, or should it also separately declare the contract it offers outward to any consumer?

**Recommended default, pending confirmation:** each component declares only its own outward-facing contract (its owned acceptance criteria), and every consuming component declares its edges by referencing that contract. This avoids duplicate or conflicting declarations of the same criteria from both sides, at the cost of requiring a consumer to look up the provider's criteria rather than restating its expectation locally.

This recommendation is not adopted — it is carried into `02-existing-capability-and-decision.md`'s open decision and must be confirmed by the requester before Priority 2 (component mapping) proceeds.

## Validation Method

Same as `02-existing-capability-and-decision.md`: exercise the model against a representative cycle (two components that both serve and consume each other) and confirm every criterion, provider obligation, and consumer claim is assigned exactly once under the recommended ownership rule.