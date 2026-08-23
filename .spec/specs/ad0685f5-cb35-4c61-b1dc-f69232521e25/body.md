<!-- aligned-structure:v2 -->

# Directed Contract Edge

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) owns typed manifest extras, and [workflow-tools/spec/crates/spec-api/schemas/specification.toml](workflow-tools/spec/crates/spec-api/schemas/specification.toml) owns declared edge rules.

## Naming Conventions

Use provider-owned `outward_contract_edges { name, consumer_spec_id, criterion_ids }`; the owning provider component spec's `id` is the provider endpoint and `consumer_spec_id` is the consumer component spec's id. Edge ids use `edge-<consumer>-consumes-<provider>`; this child owns `edge-required-fields`, `edge-nonempty-provider-criteria`, `edge-provider-ownership`, `edge-distinct-endpoints`, `edge-cycles-allowed`, `edge-unique-claim`, `edge-consumer-does-not-copy`, and `edge-persisted-typed-model`.

## Requester Input

> Stored typed component edges: the store must persist directed `(consumer, provider, provider_criterion_ids[], name)` edges in the toml, not just markdown.

## Reading Order

1. [fdb7645d Component Specification Contract](.spec/specs/fdb7645d-eac5-4b82-88eb-94cb22f1b0b2/body.md) - endpoint provider.
2. [aebcbab4 Criterion Artifact Contract](.spec/specs/aebcbab4-2827-4ea1-8244-0a2e6277b571/body.md) - provider-criterion provider.
3. [55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) - TOML persistence provider.
4. [workflow-tools/spec/crates/spec-api/schemas/specification.toml](workflow-tools/spec/crates/spec-api/schemas/specification.toml) - current generic edge rules.

## Responsibility

If implemented, consumers can persist one directed dependency on named
provider-owned criteria and tooling can read the same data without parsing prose.

## Interfaces And Dependencies

Each provider component spec's row has nonempty `criterion_ids[]`,
`consumer_spec_id`, and `name`; its owning `spec.toml` supplies the provider
component spec id. Both endpoint values are component spec ids, never a parent
or containing-spec identifier. Its serialized TOML is authoritative; hierarchy
composition and parent navigation remain handwritten.

## Behavior

- `edge-required-fields`, `edge-nonempty-provider-criteria`, and `edge-provider-ownership` validate shape, component-spec endpoints, and provider ownership.
- `edge-distinct-endpoints` rejects self dependencies; `edge-cycles-allowed` permits multi-component cycles.
- `edge-unique-claim` rejects duplicate `(consumer, provider, criterion)` claims.
- `edge-consumer-does-not-copy` preserves provider ownership.
- `edge-persisted-typed-model`: store provider-owned `[[outward_contract_edges]]` rows in `spec.toml`, not only Markdown.

## Boundaries And Failure Cases

An edge is neither a copied criterion, hierarchy edge, nor a governing
specification's `governs_ticket` relation. A self edge, empty list, foreign
criterion, missing or non-component-spec endpoint, separate containing
`spec_id`, duplicate claim, mirrored parent authority, or prose-only edge is
invalid.

## Provider/Consumer Contract

Consumes [fdb7645d Component Specification Contract](.spec/specs/fdb7645d-eac5-4b82-88eb-94cb22f1b0b2/body.md) `component-root-membership` and [aebcbab4 Criterion Artifact Contract](.spec/specs/aebcbab4-2827-4ea1-8244-0a2e6277b571/body.md) `criterion-single-owner`, `criterion-root-unique`; provides persisted edge data to [55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) and [b4475214 Specification Health Check](.spec/specs/b4475214-e14e-4926-b853-b2553444e36f/body.md).

## Examples

The provider component spec `spec-store-id` persists `{ consumer_spec_id:
"health-id", criterion_ids: ["store-persists-artifacts"], name: "health reads
persisted artifacts" }`; both values are component spec ids and the dependency
is mirrored by `Health --> Store` in the parent graph.

## Evidence

Position: `not-implemented`; the current schema declares only `depends_on`, `linked`, and `parent_of`. Planned store tests cover a two-component cycle, TOML round-trip, and every rejected case.

## Scope

Owns directed component-edge shape; it does not own hierarchy or health policy.
