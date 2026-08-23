<!-- aligned-structure:v2 -->

# Specification Root Contract

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) defines `SpecManifest`, the root persisted record; [workflow-tools/spec/crates/spec-api/schemas/specification.toml](workflow-tools/spec/crates/spec-api/schemas/specification.toml) defines its lifecycle and generic edge schema.

## Naming Conventions

`SpecManifest` is the root record; `component` remains its classification field,
not a Component Artifact. Root-owned criterion ids use `root-`; this child owns
`root-surviving-fields`, `root-component-classification`, and `root-artifact-namespace`.

## Reading Order

1. [.agents/instructions/spec/spec-system.instructions.md](.agents/instructions/spec/spec-system.instructions.md) - root/child authoring rule.
2. [55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) - persistence provider.
3. [f482eb83 Ticket Store Integration](.spec/specs/f482eb83-5b47-4ea3-8d5b-b7baa0531333/body.md) - governed-work provider.
4. [workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) - current manifest shape.

## Responsibility

If implemented, dependents can rely on one root namespace for components,
criteria, evidence references, directed edges, and observations.

## Interfaces And Dependencies

The root extends `SpecManifest`; [55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) persists it and [f482eb83 Ticket Store Integration](.spec/specs/f482eb83-5b47-4ea3-8d5b-b7baa0531333/body.md) gates governed work on it.

## Behavior

- `root-surviving-fields`: retain `id`, lifecycle, `title`, `slug`, `type`, `state`, `scope`, `parent`, `code_refs`, sections, hierarchy, `TicketRef`, and distinct `governs_ticket` relations.
- `root-component-classification`: preserve the manifest classifier independently of a Component Artifact.
- `root-artifact-namespace`: place every new artifact in exactly one root.

## Boundaries And Failure Cases

The root neither owns a participant criterion nor evidence state. A
`governs_ticket` relation is not a `ComponentContractEdge` or an informational
`related_tickets`/`related_specs` link. Missing root identity, cross-root
artifacts, or a missing typed ticket-side gate are invalid; legacy generic
forms are detect-and-report only and never infer a semantic equivalent.

## Provider/Consumer Contract

[fdb7645d Component Artifact Contract](.spec/specs/fdb7645d-eac5-4b82-88eb-94cb22f1b0b2/body.md) consumes `root-artifact-namespace`; [55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) provides persistence; [f482eb83 Ticket Store Integration](.spec/specs/f482eb83-5b47-4ea3-8d5b-b7baa0531333/body.md) provides `ticket-governing-spec`.

## Examples

A root with id `f1b8f01a...` retains `component = "spec-system"`; its
Component Artifact and Directed Contract Edge records each carry that root id,
while neither replaces the root's classifier or parent relationship.

## Evidence

Position: `partial`; [workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) stores the surviving root fields but not the target artifacts. Planned checks: `cargo test -p spec-api --test schema_test` and `./target/debug/spec.exe --workspace . get f1b8f01a-c7da-4a71-97c5-39519a7d7f38 --json`.

## Scope

Owns root identity and namespace only; artifact shape and persistence belong to sibling children.
