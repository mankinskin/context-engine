## Problem

The current memory-index track still reads as if generator logic can live centrally inside `memory-api`. That violates separation of concerns: `memory-api` is the generic backend library and must not accumulate specialized ticket/spec/rule/audit/workspace generation logic. If generators are implemented there, domain crates lose ownership of their own projections and shared infrastructure becomes harder to reuse cleanly.

## Goal

Define the architecture boundary for store-index generation so each domain crate owns a thin generator while `memory-api` exposes only reusable generic infrastructure.

## Scope

- Define which responsibilities stay in `memory-api` as generic infrastructure: shared schema types, digest helpers, sidecar codecs, generic rendering helpers, validation utilities, and common test fixtures.
- Define which responsibilities move to domain crates or CLIs: source loading, domain normalization, grouping, domain-specific rendering choices, and generator entrypoints.
- Define the minimal shared traits, helper APIs, or extension points needed so domain crates can implement thin generators without duplicating infrastructure.
- Define how generated agent-hook/workspace index surfaces are requested from each domain generator without teaching `memory-api` about specialized domains.
- Update the generator tickets so their implementation target is the owning domain crate or CLI, not `memory-api`.

## Acceptance Criteria

- The track explicitly states that `memory-api` is generic infrastructure only and does not own domain-specific generators.
- The required extension points between `memory-api` and each domain generator are identified.
- Ticket, spec, rule, audit, and workspace generator tickets point to thin domain-owned generators built on shared infrastructure.
- The boundary is precise enough that an implementer can place new code without guessing which crate owns it.

## Non-goals

- Does not implement any generator.
- Does not redefine `IndexEntry` or TOON sidecar formats.
- Does not decide git hook triggers or performance budgets.