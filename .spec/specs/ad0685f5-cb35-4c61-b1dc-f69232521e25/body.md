<!-- aligned-structure:v2 -->

# Directed Contract Edge

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) owns typed manifest extras, and [workflow-tools/spec/crates/spec-api/schemas/specification.toml](workflow-tools/spec/crates/spec-api/schemas/specification.toml) owns declared edge rules.

## Naming Conventions

Use provider-owned `outward_contract_edges { name, consumer_spec_id, criterion_ids }`; provider identity is implicit in the owning manifest. Edge ids use `edge-<consumer>-consumes-<provider>`; this child owns `edge-required-fields`, `edge-nonempty-provider-criteria`, `edge-provider-ownership`, `edge-distinct-endpoints`, `edge-cycles-allowed`, `edge-unique-claim`, `edge-consumer-does-not-copy`, and `edge-persisted-typed-model`.

## Requester Input

> Stored typed component edges: the store must persist directed `(consumer, provider, provider_criterion_ids[], name)` edges in the toml, not just markdown.

## Reading Order

1. [fdb7645d Component Artifact Contract](.spec/specs/fdb7645d-eac5-4b82-88eb-94cb22f1b0b2/body.md) - endpoint provider.
2. [aebcbab4 Criterion Artifact Contract](.spec/specs/aebcbab4-2827-4ea1-8244-0a2e6277b571/body.md) - provider-criterion provider.
3. [55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) - TOML persistence provider.
4. [workflow-tools/spec/crates/spec-api/schemas/specification.toml](workflow-tools/spec/crates/spec-api/schemas/specification.toml) - current generic edge rules.

## Responsibility

If implemented, consumers can persist one directed dependency on named
provider-owned criteria and tooling can read the same data without parsing prose.

## Interfaces And Dependencies

Each provider-owned row has nonempty `criterion_ids[]`, `consumer_spec_id`, and
`name`; its owning manifest supplies provider identity. Its serialized TOML is
authoritative; root aggregation is read-only and parent navigation remains
handwritten.

## Behavior

- `edge-required-fields`, `edge-nonempty-provider-criteria`, and `edge-provider-ownership` validate shape and provider ownership.
- `edge-distinct-endpoints` rejects self dependencies; `edge-cycles-allowed` permits multi-component cycles.
- `edge-unique-claim` rejects duplicate `(consumer, provider, criterion)` claims.
- `edge-consumer-does-not-copy` preserves provider ownership.
- `edge-persisted-typed-model`: store provider-owned `[[outward_contract_edges]]` rows in `spec.toml`, not only Markdown.

## Boundaries And Failure Cases

An edge is neither a copied criterion, hierarchy edge, nor a governing
specification's `governs_ticket` relation. A self edge, empty list, foreign
criterion, missing endpoint, duplicate claim, mirrored root authority, or
prose-only edge is invalid.

## Provider/Consumer Contract

Consumes [fdb7645d Component Artifact Contract](.spec/specs/fdb7645d-eac5-4b82-88eb-94cb22f1b0b2/body.md) `component-root-membership` and [aebcbab4 Criterion Artifact Contract](.spec/specs/aebcbab4-2827-4ea1-8244-0a2e6277b571/body.md) `criterion-single-owner`, `criterion-root-unique`; provides persisted edge data to [55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) and [b4475214 Specification Health Check](.spec/specs/b4475214-e14e-4926-b853-b2553444e36f/body.md).

## Examples

`{ consumer: "health", provider: "spec-store", provider_criterion_ids: ["store-persists-artifacts"], name: "health reads persisted artifacts" }` is valid TOML data and is mirrored by `Health --> Store` in the parent graph.

## Evidence

Position: `not-implemented`; the current schema declares only `depends_on`, `linked`, and `parent_of`. Planned store tests cover a two-component cycle, TOML round-trip, and every rejected case.

## Scope

Owns directed component-edge shape; it does not own hierarchy or health policy.
