<!-- aligned-structure:v2 -->

# Specification Root Contract

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) defines `SpecManifest`, the root persisted record; [workflow-tools/spec/crates/spec-api/schemas/specification.toml](workflow-tools/spec/crates/spec-api/schemas/specification.toml) defines its lifecycle and generic edge schema.

## Naming Conventions

`SpecManifest` is one component specification; `component` remains its
classification field, not a nested record. A parent component spec
composes child component specs through `parent`; every outward-contract endpoint
is a component spec `id`. This child owns `root-surviving-fields`,
`root-component-classification`, and `root-component-composition`.

## Reading Order

1. [.agents/instructions/spec/spec-system.instructions.md](.agents/instructions/spec/spec-system.instructions.md) - root/child authoring rule.
2. [55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) - persistence provider and specified-but-not-built shared operation journal prerequisite.
3. [f482eb83 Ticket Store Integration](.spec/specs/f482eb83-5b47-4ea3-8d5b-b7baa0531333/body.md) - governed-work consumer of that shared journal prerequisite.
4. [workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) - current manifest shape.

## Responsibility

If implemented, dependents can rely on each component being represented by one
independently addressable spec, with parent specs composing child specs through
the existing hierarchy rather than enclosing component records.

## Interfaces And Dependencies

Each component extends `SpecManifest`; a parent component spec composes direct
child component specs through their `parent` field. [55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) persists each component spec and specifies the shared operation journal prerequisite consumed by [f482eb83 Ticket Store Integration](.spec/specs/f482eb83-5b47-4ea3-8d5b-b7baa0531333/body.md), which gates governed work on its governing component spec.

## Behavior

- `root-surviving-fields`: retain `id`, lifecycle, `title`, `slug`, `type`, `state`, `scope`, `parent`, `code_refs`, sections, hierarchy, `TicketRef`, and distinct `governs_ticket` relations.
- `root-component-classification`: preserve the manifest classifier independently of component-spec identity.
- `root-component-composition`: represent each component as one spec; a parent composes child component specs through `parent` and no separate containing `spec_id` exists for a component.

## Boundaries And Failure Cases

This contract neither owns a participant criterion nor evidence state. A
`governs_ticket` relation is not a `ComponentContractEdge` or an informational
`related_tickets`/`related_specs` link. Missing component spec identity,
an endpoint that is not a component spec id, a separate containing `spec_id`,
or a missing typed ticket-side gate is invalid; legacy generic forms are
detect-and-report only and never infer a semantic equivalent.

## Provider/Consumer Contract

[fdb7645d Component Specification Contract](.spec/specs/fdb7645d-eac5-4b82-88eb-94cb22f1b0b2/body.md) consumes `root-component-composition`; [55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) provides persistence; [f482eb83 Ticket Store Integration](.spec/specs/f482eb83-5b47-4ea3-8d5b-b7baa0531333/body.md) provides `ticket-governing-spec`.

## Examples

The `f1b8f01a...` parent component spec retains `component = "spec-system"`.
Its Health Check child is a separate component spec whose `parent` is
`f1b8f01a...`; a provider edge stores that child spec id as its endpoint, while
neither spec replaces the other's classifier or hierarchy relationship.

## Evidence

Position: `partial`; [workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) stores the surviving root fields but not the target artifacts. Planned checks: `cargo test -p spec-api --test schema_test` and `./target/debug/spec.exe --workspace . get f1b8f01a-c7da-4a71-97c5-39519a7d7f38 --json`.

## Scope

Owns component-spec identity and hierarchy composition only; criterion shape,
edge shape, and persistence belong to sibling children.
