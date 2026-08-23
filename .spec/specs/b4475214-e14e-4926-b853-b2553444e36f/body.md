<!-- aligned-structure:v2 -->

# Specification Health Check

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) owns `SpecManifest::health_issues`; [workflow-tools/spec/crates/spec-api/src/store.rs](workflow-tools/spec/crates/spec-api/src/store.rs) owns `SpecStore::health` and `SpecStore::health_all`; [workflow-tools/spec/crates/spec-api/src/store/hierarchy.rs](workflow-tools/spec/crates/spec-api/src/store/hierarchy.rs) traverses parents; [workflow-tools/spec/src/cli/commands/query.rs](workflow-tools/spec/src/cli/commands/query.rs) exposes `spec health`.

## Naming Conventions

Use `health-` criterion ids and stable finding categories such as `link_parity`,
`missing_parent`, `orphan`, `parent_cycle`, and `missing_examples`. This child
owns `health-validates-references`, `health-allows-unvalidated-criteria`,
`health-no-fulfillment-gate`, `health-hierarchy-integrity`, `health-link-parity`, and `health-examples-section`.

## Requester Input

> `spec health` must parse markdown links from `body.md` and confirm they match the structured links in the toml, failing on drift.

## Reading Order

1. [55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) - persisted artifacts and parent navigation provider.
2. [ad0685f5 Directed Contract Edge](.spec/specs/ad0685f5-cb35-4c61-b1dc-f69232521e25/body.md) - structured-edge provider.
3. [89360ad7 Validation Store Evidence Integration](.spec/specs/89360ad7-d638-49e7-85ba-21839fa99851/body.md) - enforcement consumer.
4. [workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) - existing manifest health.
5. [workflow-tools/spec/crates/spec-api/src/store.rs](workflow-tools/spec/crates/spec-api/src/store.rs) - aggregate health.

## Responsibility

If implemented, `spec health` reports every structural problem needed to trust
the persisted contract and its authored Markdown navigation, without asserting fulfillment.

## Interfaces And Dependencies

Health consumes `SpecManifest`, `body.md`, hierarchy records, structured links,
and persisted component edges; it returns `SpecHealthReport` findings through
the CLI.

## Behavior

- `health-validates-references`: check required fields, root membership, ownership, uniqueness, and artifact references.
- `health-allows-unvalidated-criteria` and `health-no-fulfillment-gate`: accept absent validation and never require satisfied evidence.
- `health-hierarchy-integrity`: reject missing parents, orphans, and parent cycles.
- `health-link-parity`: parse `body.md` Markdown links and fail when the resolved spec, code, ticket, document, or component-edge links differ from structured TOML links.
- `health-examples-section`: require each spec to have a non-empty `## Examples` section.

## Boundaries And Failure Cases

Health reports structural findings, not fulfillment. Invalid Markdown, unknown
link target, unrepresented TOML link, duplicate body link semantics, missing
examples, missing parent, orphan, or cycle must be a finding. Current hierarchy
traversal and health code do not implement these checks.

## Provider/Consumer Contract

Consumes [55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md) `store-persists-artifacts` and [ad0685f5 Directed Contract Edge](.spec/specs/ad0685f5-cb35-4c61-b1dc-f69232521e25/body.md) `edge-persisted-typed-model`; provides all health criteria to [89360ad7 Validation Store Evidence Integration](.spec/specs/89360ad7-d638-49e7-85ba-21839fa99851/body.md) for hook enforcement.

## Examples

If `body.md` links `[55d8f2eb Specification Store Contract](.spec/specs/55d8f2eb-70f1-4b90-8c8f-e50d5e311d48/body.md)` but the TOML omits its structured provider edge, `spec health` returns `link_parity`; if a child has no `## Examples` content, it returns `missing_examples`.

## Evidence

Position: `partial`; existing `health_issues` checks field presence and generic dangling `depends_on`, while `health_all` aggregates reports. Planned tests cover TOML/body drift, missing and extra links, every hierarchy defect, and empty Examples; command: `./target/debug/spec.exe --workspace . health --all`. Current baseline is exactly three unrelated `9f0b9e30` findings.

## Scope

Owns health findings and validation behavior; it does not write manifests, render catalog files, or create tickets.
