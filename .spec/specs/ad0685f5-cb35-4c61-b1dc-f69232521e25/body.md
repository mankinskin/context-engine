<!-- aligned-structure:v2 -->

# Directed Contract Edge

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/manifest.rs](../../../workflow-tools/spec/crates/spec-api/src/manifest.rs) owns typed manifest extras, and [workflow-tools/spec/crates/spec-api/schemas/specification.toml](../../../workflow-tools/spec/crates/spec-api/schemas/specification.toml) owns declared edge rules.

## Naming Conventions

Use provider-owned `outward_contract_edges { edge_id, name, consumer_component_id, provider_component_id, criterion_ids }`; both endpoints are immutable `component_id` values and the provider equals the owning component. Edge ids use `edge-<consumer>-consumes-<provider>`; this child owns `edge-required-fields`, `edge-nonempty-provider-criteria`, `edge-provider-ownership`, `edge-distinct-endpoints`, `edge-cycles-allowed`, `edge-unique-claim`, `edge-consumer-does-not-copy`, and `edge-persisted-typed-model`.

## Requester Input

> Stored typed component edges: the store must persist directed `(consumer, provider, provider_criterion_ids[], name)` edges in the toml, not just markdown.

## Reading Order

1. [fdb7645d Component Specification Contract](../fdb7645d-eac5-4b82-88eb-94cb22f1b0b2/body.md) - endpoint provider.
2. [aebcbab4 Criterion Artifact Contract](../aebcbab4-2827-4ea1-8244-0a2e6277b571/body.md) - provider-criterion provider.
3. [55d8f2eb Specification Store Contract](../55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) - TOML persistence provider.
4. [workflow-tools/spec/crates/spec-api/schemas/specification.toml](../../../workflow-tools/spec/crates/spec-api/schemas/specification.toml) - current generic edge rules.

## Responsibility

If implemented, consumers can persist one directed dependency on named
provider-owned criteria and tooling can read the same data without parsing prose.

## Interfaces And Dependencies

Each provider component's row has nonempty `criterion_ids[]`, `edge_id`,
`consumer_component_id`, `provider_component_id`, and `name`; both endpoint
values are `component_id`s, never manifest UUIDs, parents, or containing-spec
identifiers. Its serialized TOML is authoritative; hierarchy composition and
parent navigation remain separate.

## Behavior

- `edge-required-fields`, `edge-nonempty-provider-criteria`, and `edge-provider-ownership` validate shape, `component_id` endpoints, and provider ownership.
- `edge-distinct-endpoints` rejects self dependencies; `edge-cycles-allowed` permits multi-component cycles.
- `edge-unique-claim` rejects duplicate `(consumer, provider, criterion)` claims.
- `edge-consumer-does-not-copy` preserves provider ownership.
- `edge-persisted-typed-model`: store provider-owned `[[outward_contract_edges]]` rows in `spec.toml`, not only Markdown, and validate them separately from `parent` composition.

## Boundaries And Failure Cases

An edge is neither a copied criterion, hierarchy composition edge, criterion
template expansion, nor a governing
specification's `governs_ticket` relation. A self edge, empty list, foreign
criterion, missing or non-component-id endpoint, provider not equal to the
owning component, separate containing
`spec_id`, duplicate claim, mirrored parent authority, or prose-only edge is
invalid.

## Provider/Consumer Contract

Consumes [fdb7645d Component Specification Contract](../fdb7645d-eac5-4b82-88eb-94cb22f1b0b2/body.md) `component-root-membership` and [aebcbab4 Criterion Artifact Contract](../aebcbab4-2827-4ea1-8244-0a2e6277b571/body.md) `criterion-single-owner`, `criterion-root-unique`; provides persisted edge data to [55d8f2eb Specification Store Contract](../55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) and [b4475214 Specification Health Check](../b4475214-e14e-4926-b853-b2553444e36f/body.md).

## Examples

The provider component `spec-store` persists `{ consumer_component_id:
"spec-health", provider_component_id: "spec-store", criterion_ids:
["spec-store-persists-artifacts"], name: "health reads persisted artifacts" }`;
both values are component ids and the dependency
is mirrored by `Health --> Store` in the parent graph.

## Evidence

Position: `not-implemented`; the current schema declares only `depends_on`, `linked`, and `parent_of`. Planned store tests cover a two-component cycle, TOML round-trip, and every rejected case.

## Scope

Owns directed component-edge shape; it does not own hierarchy or health policy.
