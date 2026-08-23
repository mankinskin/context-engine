<!-- aligned-structure:v2 -->

# Criterion Template Contract

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/manifest.rs](../../../workflow-tools/spec/crates/spec-api/src/manifest.rs) is the future persisted artifact boundary; no template type exists there today.

## Naming Conventions

Use `CriterionTemplate`, `CriterionTemplateVersion`, `CriterionTemplateBinding`, and `template-<family>-v<version>` identities. A binding record carries `template_id`, `template_version`, `bindings`, `owner_component_id`, and concrete `criterion_id`. Instantiated provider-owned artifacts retain the owner prefix and use deterministic ids such as `<owner-prefix>-<binding>-<behavior>`.

## Requester Input

> Reusable criterion templates. Model generic parameterized criterion templates that can be instantiated across component graphs, initially for the `-api`, `-cli`, `-mcp`, `-http`, `-viewer` family.

## Reading Order

1. [f1b8f01a Component-Oriented Specification System](../f1b8f01a-c7da-4a71-97c5-39519a7d7f38/body.md) - composing parent and shared invariants.
2. [aebcbab4 Criterion Artifact Contract](../aebcbab4-2827-4ea1-8244-0a2e6277b571/body.md) - instantiated criterion ownership.
3. [ad0685f5 Directed Contract Edge](../ad0685f5-cb35-4c61-b1dc-f69232521e25/body.md) - consumer/provider edges after expansion.

## Responsibility

If implemented, component graphs can reuse a versioned generic criterion definition while every resulting criterion remains an explicitly identified, provider-owned criterion of one component.

## Interfaces And Dependencies

A template has stable identity, version, typed parameter names, a criterion-id recipe, and behavior/measurement recipes. An instantiation binds one version and complete parameters to one concrete provider-owned `CriterionArtifact`, preserving matching provenance fields on that artifact. Initial families cover `-api`, `-cli`, `-mcp`, `-http`, and `-viewer`; they are templates, not components or edges.

Template expansion produces the same ordinary `CriterionArtifact` shape for a
parent composition criterion as for a component-internal criterion; templates
do not introduce a composition-specific artifact class.

## Behavior

- `template-definition`: reject duplicate parameter names and ambiguous id recipes.
- `template-deterministic-expansion`: identical template version, bindings, and owner produce identical criterion ids and statements.
- `template-owner-materialization`: expansion creates concrete criterion artifacts owned by the bound `owner_component_id`, never by the template or parent.
- `template-collision-handling`: collision with a non-identical existing criterion fails; identical repeated expansion is idempotent.
- `template-version-migration`: a version is immutable; a migration declares old-to-new bindings and reports criteria requiring review rather than silently rewriting them.

## Boundaries And Failure Cases

Templates neither create components nor encode provider/consumer dependencies. Missing bindings, unknown template/version, invalid generated ids, owner outside the composed hierarchy, mismatched artifact provenance, or a collision with differing content is invalid. Template upgrade is explicit migration, not implicit latest-version selection.

## Provider/Consumer Contract

Provides template expansion semantics to [aebcbab4 Criterion Artifact Contract](../aebcbab4-2827-4ea1-8244-0a2e6277b571/body.md); expanded criteria may then be consumed through [ad0685f5 Directed Contract Edge](../ad0685f5-cb35-4c61-b1dc-f69232521e25/body.md). No template itself is a provider endpoint.

## Examples

`template-interface-parity-v1` binds `api = spec-api`, `cli = spec-cli`, and `operation = get`. It deterministically creates `spec-api-get-shared-api` and `spec-cli-get-delegates-api`; `spec-cli` consumes the former only after the concrete components exist. The `spec-api`, `spec-cli`, and `spec-mcp` component specs are specified-but-not-built examples, not present components.

## Evidence

Position: `not-implemented`. Planned `cargo test -p spec-api` cases cover deterministic expansion, idempotence, collision rejection, version migration, and the initial family matrix. A future health check resolves every template/version/binding and generated owner criterion.

## Scope

Owns reusable criterion definition and instantiation only; hierarchy composition, component identity, and edge persistence remain sibling contracts.
