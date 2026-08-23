<!-- aligned-structure:v2 -->

# Criterion Artifact Contract

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) currently defines `AcceptanceCriterion`; it is the replacement location for a provider-owned criterion artifact.

## Naming Conventions

Use `CriterionArtifact`; ids are `<registered-prefix>-<behavior>` and are unique
per root. This child owns `criterion-required-fields`, `criterion-single-owner`,
`criterion-root-unique`, `criterion-optional-validation`, `criterion-evidence-integrity`, and `criterion-naming-conventions-required`.

## Requester Input

> Code-first required fields: `code_refs` (target code location) becomes required for a code-facing component spec, and naming conventions are a required section.

## Reading Order

1. [fdb7645d Component Artifact Contract](.spec/specs/fdb7645d-eac5-4b82-88eb-94cb22f1b0b2/body.md) - owner provider.
2. [7498bed7 Evidence Reference Contract](.spec/specs/7498bed7-ac74-4484-b50e-8a9cf96d8431/body.md) - `validated_by` evidence provider.
3. [ad0685f5 Directed Contract Edge](.spec/specs/ad0685f5-cb35-4c61-b1dc-f69232521e25/body.md) - criterion-edge consumer.
4. [workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) - current criterion model.

## Responsibility

If implemented, dependents can rely on one provider-owned, separately named
acceptance obligation rather than copied consumer requirements.

## Interfaces And Dependencies

`CriterionArtifact` requires `id`, `spec_id`, `owner_component_id`, and
`statement`; `validated_by[]` is optional and references same-root evidence.

## Behavior

- `criterion-required-fields`: require identity, owner, and statement.
- `criterion-single-owner`: resolve exactly one same-root component.
- `criterion-root-unique`: require an id once per root.
- `criterion-optional-validation`: accept an empty `validated_by` list.
- `criterion-evidence-integrity`: resolve every named evidence id in that root.
- `criterion-naming-conventions-required`: require a documented owner-prefix naming convention for each code-facing criterion component.
- `criterion-prefix-registry`: require each component to resolve to exactly one `.spec/criterion-prefixes.toml` entry, with unique component ids and prefixes across all registered scan roots; every criterion must match that entry's `<prefix>-<behavior>` form.

## Boundaries And Failure Cases

Criteria do not copy provider claims into consumer contracts or require an
observation. Missing owner, duplicate id, cross-root owner, dangling evidence,
an unregistered/mismatched/duplicate/orphan prefix entry, or undocumented
code-facing naming convention is invalid. The target registry schema is:

```toml
[component_prefixes]
"<stable-component-id>" = "<prefix>"
```

This is specified-but-not-built: create the file only after materialized stable
component ids allow real entries, then migrate and rename affected criteria;
never auto-populate it.

## Provider/Consumer Contract

Consumes [fdb7645d Component Artifact Contract](.spec/specs/fdb7645d-eac5-4b82-88eb-94cb22f1b0b2/body.md) `component-criterion-ownership`; provides `criterion-single-owner` and `criterion-root-unique` to [ad0685f5 Directed Contract Edge](.spec/specs/ad0685f5-cb35-4c61-b1dc-f69232521e25/body.md), and `criterion-required-fields` to [83c0b9c4 Validation Observation Contract](.spec/specs/83c0b9c4-1617-4751-af23-57811060f0fb/body.md).

## Examples

The Health Check component owns `health-link-parity` with a statement requiring
one structured manifest link for each parsed `body.md` markdown link; an edge
may consume that id but may not restate its statement.

## Evidence

Position: `not-implemented`; [workflow-tools/spec/crates/spec-api/src/manifest.rs](workflow-tools/spec/crates/spec-api/src/manifest.rs) stores the retired `AcceptanceCriterion`. Planned manifest/store tests cover round-trip, duplicate id, foreign owner, and empty `validated_by`.

## Scope

Owns criterion shape and ownership only; observations and edge persistence are siblings.
