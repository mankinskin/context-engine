<!-- aligned-structure:v2 -->

# Component Artifact

## Responsibility And Interface

Represent one named participant in a specification root. The future manifest
interface requires `id`, `spec_id`, `title`, and `purpose`; it optionally carries
`context`, related spec/evidence IDs, and `code_refs`.

## Behavior And Contract

- `component-required-fields`: require the four identity and purpose fields.
- `component-optional-fields`: retain only the declared optional context links.
- `component-root-membership`: resolve the declared root.
- `component-criterion-ownership`: exclusively own zero or more criteria.
- Consume `root-artifact-namespace` without redefining root identity.

## Boundaries And Failure Cases

A component is not the manifest's existing `component` classification and does
not own consumer edges or another component's criteria. Missing root or required
fields is invalid; empty criteria are valid.

## Acceptance Evidence And Position

Add parsing and round-trip cases in `src/manifest/tests.rs` and `tests/schema_test.rs`.
Validate with focused manifest tests; current `SpecManifest` has no artifact type.
