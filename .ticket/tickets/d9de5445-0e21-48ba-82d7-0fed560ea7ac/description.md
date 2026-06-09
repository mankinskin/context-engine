## Problem

The current memory-index track risks pulling specialized generator logic into `memory-api`, even though `memory-api` is a generic backend library and should not know about ticket, spec, rule, audit, or workspace-specific rendering semantics. If domain-specific generators live centrally in `memory-api`, separation of concerns degrades, reusable domain infrastructure is bypassed, and future generators become harder to place correctly.

## Goal

Define the generator layering contract so each domain owns a thin generator in its own crate or CLI surface, while `memory-api` only provides shared generic infrastructure such as index schemas, sidecar support, common rendering helpers, and validation utilities.

## Scope

- Define which responsibilities belong in `memory-api` versus the domain crates (`ticket-api` / `ticket-cli`, `spec-api` / `spec-cli`, `rule-api` / `rule-cli`, `audit-api` / `audit-cli`, and workspace-level tooling).
- Define the reusable generic interfaces, helper traits, or builder utilities that `memory-api` may expose without importing domain-specific concepts.
- Define the thin-generator shape each domain should implement locally.
- Update the generator tickets so their implementation surface is the owning domain crate or CLI, not centralized specialized code inside `memory-api`.
- Clarify how shared validation and sidecar generation are reused from `memory-api` without coupling domain parsing and rendering into the generic library.

## Acceptance Criteria

- The track has a written layering decision separating generic `memory-api` infrastructure from domain-owned generator code.
- The owning implementation surface is named for ticket, spec, rule, audit, and workspace generator tickets.
- `memory-api` is limited to generic index infrastructure and does not acquire specialized domain knowledge.
- Generator tickets are updated so implementers do not need to guess where the code belongs.

## Non-goals

- Does not implement the generators.
- Does not change the generic `IndexEntry` / sidecar schema.
- Does not define domain digest normalization details beyond ownership boundaries.

## Resolved direction carried into this ticket

- Shared index infrastructure can live in `memory-api`, but domain-specific generation must live in the owning domain crates.