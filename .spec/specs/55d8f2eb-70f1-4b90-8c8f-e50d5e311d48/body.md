<!-- aligned-structure:v2 -->

# Specification Store Contract

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/store.rs](workflow-tools/spec/crates/spec-api/src/store.rs) owns `SpecStore`; [workflow-tools/spec/crates/spec-api/src/store/hierarchy.rs](workflow-tools/spec/crates/spec-api/src/store/hierarchy.rs) and [workflow-tools/spec/crates/spec-api/src/store/sections.rs](workflow-tools/spec/crates/spec-api/src/store/sections.rs) own preserved child behavior; [workflow-tools/spec/crates/spec-api/src/store_index_render.rs](workflow-tools/spec/crates/spec-api/src/store_index_render.rs) renders catalog Markdown.

## Naming Conventions

Use `SpecStore` for persistence, `spec.toml` for structured contract data, and
`body.md` for human navigation. This child owns `store-persists-artifacts`,
`store-preserves-baselines`, `store-removes-retired-model`,
`store-parent-navigation-verification`, and `store-criterion-prefix-registry`.

## Requester Input

> Parent link-list + subcomponent graph requirement: the store-index renderer must be able to emit/verify the parent's child-link list and graph.

## Reading Order

1. [ad0685f5 Directed Contract Edge](.spec/specs/ad0685f5-cb35-4c61-b1dc-f69232521e25/body.md) - persisted edge provider.
2. [b4475214 Specification Health Check](.spec/specs/b4475214-e14e-4926-b853-b2553444e36f/body.md) - persistence consumer.
3. [workflow-tools/spec/crates/spec-api/src/store.rs](workflow-tools/spec/crates/spec-api/src/store.rs) - store boundary.
4. [workflow-tools/spec/crates/spec-api/src/store_index_render.rs](workflow-tools/spec/crates/spec-api/src/store_index_render.rs) - catalog renderer.

## Responsibility

If implemented, readers can round-trip each root's artifacts and contract edges
from `spec.toml`, while parent navigation remains mechanically verifiable.

## Interfaces And Dependencies

`SpecStore` persists `SpecManifest` and `body.md`; the index renderer receives
raw bodies and manifests to derive the existing hierarchy catalog. The `.spec`
index root owns the committed system-wide `.spec/criterion-prefixes.toml`
registry across all registered scan roots in the same `SpecStore`.

## Behavior

- `store-persists-artifacts`: round-trip components, criteria, evidence, typed edges, observations, and distinct `governs_ticket` relations with their root.
- `store-preserves-baselines`: preserve sections, hierarchy, and `TicketRef`.
- `store-removes-retired-model`: retire `contract_mode`, expected properties, mandatory evidence requirements, and fulfillment summaries.
- `store-parent-navigation-verification`: verify, but never generate or rewrite, each handwritten parent body's complete direct-child link list and `flowchart TD` graph against its structured hierarchy and component edges.
- `store-criterion-prefix-registry`: preserve the committed registry mapping each stable component id to one unique prefix across every registered scan root. Migration first populates the registry and renames affected criteria; it never infers entries automatically.

## Boundaries And Failure Cases

The store does not decide health policy or migration. Failed parse, missing root,
invalid persisted reference, child list omission, graph/body mismatch, missing
or duplicate registry entry, duplicate prefix, or malformed criterion id returns
an error; retained baselines must not silently change semantics. No manifest
schema-version field exists today; legacy generic forms are detect-and-report
only, following the ticket metadata precedent that detects `related_specs` when
`refs` is absent.

## Provider/Consumer Contract

Consumes [ad0685f5 Directed Contract Edge](.spec/specs/ad0685f5-cb35-4c61-b1dc-f69232521e25/body.md) `edge-persisted-typed-model`; provides `store-persists-artifacts` to [b4475214 Specification Health Check](.spec/specs/b4475214-e14e-4926-b853-b2553444e36f/body.md), [a608f774 Specification Root Contract](.spec/specs/a608f774-9f50-4de1-90e2-ffeefb0198b1/body.md), and the future CLI query child.

## Examples

When the root has 12 direct children, renderer verification rejects a body whose
Reading Order links only 11 children or whose Mermaid graph omits the CLI child;
the persisted `parent` fields are the comparison source.

## Evidence

Position: `partial`; [workflow-tools/spec/crates/spec-api/src/store.rs](workflow-tools/spec/crates/spec-api/src/store.rs) persists current manifests and [workflow-tools/spec/crates/spec-api/src/store_index_render.rs](workflow-tools/spec/crates/spec-api/src/store_index_render.rs) renders catalog trees, but neither models artifacts nor checks authored maps. Planned `cargo test -p spec-api` store, hierarchy, and renderer tests.

## Scope

Owns persistence and index rendering; health policy and CLI formatting are sibling responsibilities.
