<!-- aligned-structure:v2 -->

# Component-Oriented Specification System

## Motivation

Define the reviewed target artifact model for specifications without conflating
component contracts, evidence, or adjacent-store responsibilities in one file.

## Dependent Expectation

If implemented, dependents can rely on root-scoped component contracts,
provider-owned criteria, explicit evidence, and directed dependencies.

## Shared Invariants

- A provider exclusively owns its criteria and outward obligations.
- A consumer owns only an edge that names provider criteria; it never copies them.
- An unvalidated criterion is complete; health is structural, not fulfillment gating.
- `component` remains root classification, distinct from the Component artifact.

## Component Relationship Map

| Consumer child | Provider child | Criteria consumed |
| --- | --- | --- |
| Component Artifact | Specification Root | `root-artifact-namespace` |
| Criterion Artifact | Component Artifact; Specification Root | `component-criterion-ownership`; `root-artifact-namespace` |
| Evidence Reference | Specification Root; Document Store | `root-artifact-namespace`; `document-stable-target`, `document-resolution-result` |
| Directed Contract Edge | Component Artifact; Criterion Artifact | `component-root-membership`, `component-criterion-ownership`; `criterion-single-owner`, `criterion-root-unique` |
| Validation Observation | Criterion Artifact; Evidence Reference; Validation Store | `criterion-required-fields`; `evidence-required-fields`; `validation-criterion-link`, `validation-observation-source`, `validation-best-effort` |
| Specification Root | Specification Store; Ticket Store | `store-persists-artifacts`, `store-preserves-baselines`; `ticket-governing-spec`, `ticket-explicit-spec-target`, `ticket-spec-before-plan` |
| Health Check | Specification Store | `store-persists-artifacts` |

## Positions And Evidence

`workflow-tools/spec/crates/spec-api/src/manifest.rs` currently implements the
retired model; this draft defines its replacement. The children name focused
schema, manifest, store, and CLI evidence. `90e4fb79-2c60-42a6-ab10-91d243693150`
is the related production-workflow neighbor. Governing rule:
`.agents/instructions/spec/spec-system.instructions.md`.

## Scope

This root owns only the cross-component invariant and relationship map. Its
eleven direct children own acceptance behavior, boundaries, and evidence.