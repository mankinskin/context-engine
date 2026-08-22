<!-- aligned-structure:v2 -->

# Specification Root

## Responsibility And Interface

Preserve root identity and provide the namespace that contains components,
criteria, evidence references, edges, and observations. It extends the current
`SpecManifest` in `workflow-tools/spec/crates/spec-api/src/manifest.rs`.

## Behavior And Contract

- `root-surviving-fields`: retain `id`, lifecycle, `title`, `slug`, `type`,
  `state`, `scope`, `parent`, `code_refs`, sections, hierarchy, and TicketRef.
- `root-component-classification`: retain the manifest `component` classifier,
  distinct from a new component artifact.
- `root-artifact-namespace`: scope every new artifact to exactly one root.
- Consume Specification Store persistence and Ticket Store's governing-spec gate.

## Boundaries And Failure Cases

The root does not own a participant's criteria or evidence state. Missing root
identity or cross-root artifacts are invalid; it does not implement migration.

## Acceptance Evidence And Position

Extend `tests/schema_test.rs` and `src/manifest/tests.rs`, then run
`cargo test -p spec-api --test schema_test` and `spec.exe --workspace . get
f1b8f01a-c7da-4a71-97c5-39519a7d7f38 --json`. Current code is pre-target.
