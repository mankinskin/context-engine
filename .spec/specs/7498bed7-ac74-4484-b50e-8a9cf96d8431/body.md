<!-- aligned-structure:v2 -->

# Evidence Reference Contract

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/ticket_ref.rs](workflow-tools/spec/crates/spec-api/src/ticket_ref.rs) defines the implemented explicit workspace/store identity baseline; [workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) is the future evidence-reference field owner.

## Naming Conventions

Use `EvidenceReference`; ids use `evidence-<target-kind>-<name>`. This child owns `evidence-required-fields`, `evidence-optional-locator`, `evidence-explicit-store-target`, and `evidence-external-state`.

## Reading Order

1. [a608f774 Specification Root Contract](.spec/specs/a608f774-9f50-4de1-90e2-ffeefb0198b1/body.md) - root provider.
2. [73817390 Document Store Evidence Integration](.spec/specs/73817390-7e6a-427a-a644-626718d9f25d/body.md) - document-target provider.
3. [83c0b9c4 Validation Observation Contract](.spec/specs/83c0b9c4-1617-4751-af23-57811060f0fb/body.md) - evidence consumer.
4. [workflow-tools/spec/crates/spec-api/src/ticket_ref.rs](workflow-tools/spec/crates/spec-api/src/ticket_ref.rs) - explicit-store reference model.

## Responsibility

If implemented, dependents can link review material by stable identity without
embedding its fulfillment state.

## Interfaces And Dependencies

`EvidenceReference` requires `id`, `spec_id`, `target_kind`, `target_ref`, and
`relation`; `locator` is optional. Store-backed targets carry explicit workspace
and store-root identity. Document references consume the future Document API
typed target and outcome contract from [73817390 Document Store Evidence Integration](.spec/specs/73817390-7e6a-427a-a644-626718d9f25d/body.md); they do not define an adapter grammar or free-form path semantics.

## Behavior

- `evidence-required-fields`: require the five target fields.
- `evidence-optional-locator`: preserve an exact supplied target location.
- `evidence-explicit-store-target`: forbid implicit store resolution.
- `evidence-external-state`: link material without mandatory fulfillment state.
- `evidence-document-outcome-consumer`: once the specified-but-not-built Document API exists, consume only its `Resolved`, `Missing`, and `Unsupported` typed outcomes.

## Boundaries And Failure Cases

Evidence is neither an observation nor success claim. Missing identity,
unresolvable declared store metadata, or cross-root reference is invalid.
Document `Missing` and `Unsupported` resolution outcomes are not success and
cannot establish a successful observation; an unavailable document is not
itself a health fulfillment gate. Until the Document API repository/index and
typed grammar are implemented, this layer cannot claim to resolve documents and
must not add a spec-adapter grammar or free-form path interpretation.

## Provider/Consumer Contract

Consumes [a608f774 Specification Root Contract](.spec/specs/a608f774-9f50-4de1-90e2-ffeefb0198b1/body.md) `root-artifact-namespace` and [73817390 Document Store Evidence Integration](.spec/specs/73817390-7e6a-427a-a644-626718d9f25d/body.md) `document-stable-target`; provides `evidence-required-fields` to [83c0b9c4 Validation Observation Contract](.spec/specs/83c0b9c4-1617-4751-af23-57811060f0fb/body.md).

## Examples

`evidence-document-health-template` records `target_kind = "document"`, a
Document API typed target, and an optional locator. Once the provider exists,
an absent target yields `Missing` rather than a claimed successful observation.

## Evidence

Position: `partial`; `TicketRef` serializes and resolves explicit store roots,
but an EvidenceReference artifact and its Document API prerequisite are
specified-but-not-built. Planned TicketRef-style serialization and same-root
reference tests follow Document API `Resolved`/`Missing`/`Unsupported` contract tests.

## Scope

Owns evidence identity, not document retrieval or validation outcome.
