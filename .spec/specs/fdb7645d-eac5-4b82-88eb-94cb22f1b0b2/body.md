<!-- aligned-structure:v2 -->

# Component Artifact Contract

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) owns the future component-artifact fields; [workflow-tools/spec/crates/spec-api/src/code_ref.rs](workflow-tools/spec/crates/spec-api/src/code_ref.rs) owns `CodeRef` serialization and range validation.

## Naming Conventions

Use `ComponentArtifact` for the future record, not `SpecManifest.component`.
Component criterion ids use the registered system-wide prefix; this child owns
`component-required-fields`, `component-optional-fields`,
`component-root-membership`, `component-criterion-ownership`,
`component-code-refs-required`, and `component-outward-edge-ownership`.

## Requester Input

> Code-first required fields: `code_refs` (target code location) becomes required for a code-facing component spec, and naming conventions are a required section.

## Reading Order

1. [.agents/instructions/spec/spec-system.instructions.md](.agents/instructions/spec/spec-system.instructions.md) - code-first child format.
2. [a608f774 Specification Root Contract](.spec/specs/a608f774-9f50-4de1-90e2-ffeefb0198b1/body.md) - root namespace provider.
3. [aebcbab4 Criterion Artifact Contract](.spec/specs/aebcbab4-2827-4ea1-8244-0a2e6277b571/body.md) - owned criterion consumer.
4. [workflow-tools/spec/crates/spec-api/src/code_ref.rs](workflow-tools/spec/crates/spec-api/src/code_ref.rs) - `CodeRef` contract.

## Responsibility

If implemented, dependents can name a code-facing participant that owns criteria
without conflating it with the root manifest classifier.

## Interfaces And Dependencies

`ComponentArtifact` requires `id`, `spec_id`, `title`, and `purpose`; optional
fields are `context`, related spec/evidence ids, `code_refs`, and provider-owned
outward contract edges. Provider identity is implicit in the owning manifest:

```toml
[[outward_contract_edges]]
name = "health reads persisted artifacts"
consumer_spec_id = "<consumer-spec-id>"
criterion_ids = ["<registered-prefix>-persists-artifacts"]
```

## Behavior

- `component-required-fields`: require the four identity and purpose fields.
- `component-optional-fields`: retain only declared optional context links.
- `component-root-membership`: resolve the declared root.
- `component-criterion-ownership`: exclusively own zero or more criteria.
- `component-code-refs-required`: a code-facing component has at least one valid `CodeRef`, and its body has a non-empty `## Naming Conventions` section.
- `component-outward-edge-ownership`: each provider persists its own `[[outward_contract_edges]]` rows; root aggregation is read-only and never mirrors an authoritative edge.

## Boundaries And Failure Cases

A component is not the root classifier and owns neither consumer edges nor another
component's criteria. Missing root, required field, code reference, naming
section, or registry entry is invalid for a code-facing component; empty owned
criteria are valid.

## Provider/Consumer Contract

Consumes [a608f774 Specification Root Contract](.spec/specs/a608f774-9f50-4de1-90e2-ffeefb0198b1/body.md) `root-artifact-namespace`; provides `component-criterion-ownership` to [aebcbab4 Criterion Artifact Contract](.spec/specs/aebcbab4-2827-4ea1-8244-0a2e6277b571/body.md) and `component-root-membership` to [ad0685f5 Directed Contract Edge](.spec/specs/ad0685f5-cb35-4c61-b1dc-f69232521e25/body.md).

## Examples

A `Health Check` component declares a `CodeRef` to `SpecStore::health`, names
the `health-` criterion namespace, and owns `health-link-parity`; the root's
`component = "spec-api"` remains unrelated classification metadata.

## Evidence

Position: `not-implemented`; current [workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) has no artifact type. Planned checks: manifest round-trip and invalid missing-`code_refs` cases in `cargo test -p spec-api`.

## Scope

Owns component shape and code-facing metadata requirements, not root persistence or health evaluation.
