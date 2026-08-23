<!-- aligned-structure:v2 -->

# Rust Source Annotation Traceability Contract

## Target Code Location

[workflow-tools/spec/crates/spec-api/src/code_ref.rs](../../../workflow-tools/spec/crates/spec-api/src/code_ref.rs) defines the current `CodeRef` fallback; no annotation attribute or Rust-item resolver exists today.

## Naming Conventions

Use source-side `#[implements(component_id = "...")]` and `#[validates(criterion_id = "...")]`. The resolver emits `ResolvedImplementation` keyed by persisted immutable `component_id`, and `ResolvedValidation` keyed by provider-owned criterion artifact `criterion_id` and parsed Rust item identity.

## Requester Input

> Rust-level source traceability. Define item-level source annotation contracts for eventual attributes such as `#[implements(component_id = "...")]` and `#[validates(criterion_id = "...")]`.

## Reading Order

1. [f1b8f01a Component-Oriented Specification System](../f1b8f01a-c7da-4a71-97c5-39519a7d7f38/body.md) - composing parent.
2. [fdb7645d Component Artifact Contract](../fdb7645d-eac5-4b82-88eb-94cb22f1b0b2/body.md) - component identities.
3. [aebcbab4 Criterion Artifact Contract](../aebcbab4-2827-4ea1-8244-0a2e6277b571/body.md) - criterion identities.
4. [workflow-tools/spec/crates/spec-api/src/code_ref.rs](../../../workflow-tools/spec/crates/spec-api/src/code_ref.rs) - navigation fallback.

## Responsibility

If implemented, dependents can rely on parsed Rust items to declare their implemented component or validated criterion identities, with current source locations resolved from syntax rather than stale hand-maintained spans.

## Interfaces And Dependencies

The resolver validates every `component_id` against an existing persisted component identity, not its manifest UUID or slug, and every `criterion_id` against an existing provider-owned criterion artifact. Supported stable Rust items are structs, enums, traits, impl methods and associated functions, free functions, consts, and statics. A successful annotation result includes parsed item kind/name and current file/line range.

## Behavior

- `annotation-identity-resolution`: resolve `component_id` and provider-owned `criterion_id` before reporting a location.
- `annotation-current-location`: derive location from the parsed Rust item on each resolution pass.
- `annotation-authoritative-when-present`: use a valid annotation as the authoritative implementation or validation link; retain `CodeRef` as a navigation fallback when no annotation exists.
- `annotation-health`: report unknown ids, malformed attributes, duplicate conflicting declarations, unsupported item kinds, and unresolved source as health findings.

## Boundaries And Failure Cases

These attributes are specified-but-not-built and do not imply existing proc macros. Local variables are explicitly excluded from proc-macro coverage; any local discovery is a separate future source-model concern. An annotation neither creates an identity nor changes criterion ownership, and a `CodeRef` does not become authoritative when a valid annotation is present.

## Provider/Consumer Contract

Consumes component identities from [fdb7645d Component Artifact Contract](../fdb7645d-eac5-4b82-88eb-94cb22f1b0b2/body.md) and criterion identities from [aebcbab4 Criterion Artifact Contract](../aebcbab4-2827-4ea1-8244-0a2e6277b571/body.md); provides resolved source traceability to health and reviewer navigation. This is not a provider/consumer edge between Rust items.

## Examples

`#[implements(component_id = "worktree-control-cli")]` on a free `dispatch` function resolves to that parsed function's current span. `#[validates(criterion_id = "worktree-gitlink-containment")]` on an integration-test helper resolves only when the criterion exists. A local `let candidate` is outside this contract.

## Evidence

Position: `not-implemented`. Planned resolver fixtures cover each supported item kind, stale location movement, unknown identities, duplicate conflicts, and `CodeRef` fallback. Health must distinguish invalid annotation from absent annotation.

## Scope

Owns source annotation declaration and resolution semantics, not proc-macro implementation, local-variable discovery, or component/criterion creation.
