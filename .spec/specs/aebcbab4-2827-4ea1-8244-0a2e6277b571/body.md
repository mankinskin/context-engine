<!-- aligned-structure:v2 -->

# Criterion Artifact

## Responsibility And Interface

State one provider-owned acceptance obligation. The replacement for
`AcceptanceCriterion` requires `id`, `spec_id`, `owner_component_id`, and
`statement`; `validated_by[]` is optional.

## Behavior And Contract

- `criterion-required-fields`: require its identity, owner, and statement.
- `criterion-single-owner`: resolve exactly one same-root component.
- `criterion-root-unique`: make criterion IDs unique per root.
- `criterion-optional-validation`: accept no `validated_by` entries.
- `criterion-evidence-integrity`: resolve each named evidence ID in that root.
- Consume Component ownership and the root namespace.

## Boundaries And Failure Cases

Criteria do not copy provider claims into consumer contracts or require an
observation. Missing owner, duplicate ID, cross-root owner, or dangling evidence
reference is invalid.

## Acceptance Evidence And Position

Replace today's `expected_property_ids` and `required_evidence_ids` behavior in
`src/manifest.rs`; prove round-trips and invalid references in manifest/store tests.
