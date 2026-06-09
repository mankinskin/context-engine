## Problem

Each tool domain stores structured data in its own local store (`.ticket/`, `.spec/`, `.rule/`, `.audit/`, and related workspace-local metadata). Agents and humans still have to inspect raw TOML and markdown files directly, which is verbose and expensive. The repository needs committed index artifacts that make those stores easier to navigate and consume.

The earlier version of this track was too memory-api-centric. `memory-api` is a generic backend library and should not absorb specialized ticket/spec/rule/audit/workspace generator logic. Each domain should own a thin generator and reuse only shared generic infrastructure from `memory-api`.

The track also had three planning gaps that would have made early generator work premature:
- no explicit integration plan with the existing `rule-api` rendering/generation pipeline
- no explicit `peek-cli` / level-of-detail validation story for efficient agent consumption
- no dedicated benchmarking and profiling plan to keep commit-time regeneration fast

## Goal

Generate lightweight, committed store-index artifacts for the memory-api tool domains while preserving separation of concerns:
- each domain owns its own thin generator in its local crate or CLI surface
- `memory-api` provides only shared generic infrastructure such as schema types, digest/sidecar helpers, validation utilities, and reusable rendering support
- generated outputs are designed for both human browsing and efficient agent consumption

## Scope

This track covers the planning and implementation work needed to generate committed index artifacts for the tool domains first under review: ticket, spec, rule, audit, and workspace summaries.

The architecture boundary is explicit:
- domain crates own domain-specific loading, normalization, grouping, and thin generation
- `memory-api` remains generic and does not learn specialized domain semantics
- shared rendering and generation infrastructure must align with the existing repository rendering pipeline rather than creating a parallel one

## Track decisions so far

- Shared schema and sidecar foundations already exist in `0dba399a` and `e7a0ee3c`.
- Generator tickets must not advance until the planning blockers for hook automation, domain digest inputs, generator layering, rendering-pipeline integration, efficient `peek-cli` consumption, and performance budgeting are reviewed.
- Generated outputs are committed to git and should support stable diffs and digest-stable regeneration on unchanged inputs.

## Child tickets

Foundational blockers:
- `0dba399a` Define IndexEntry schema and serde contract
- `e7a0ee3c` IndexEntry TOON sidecar format and validator
- `52dfd793` Define git hook automation for store-index regeneration
- `7f7fe4a8` Define domain digest input contract for generated index entries
- `94c56f3d` Define domain-owned thin generator architecture for store indexes
- `db667eed` Define shared rendering pipeline integration for generated indexes
- `d3a95908` Define peek-cli and level-of-detail validation for generated indexes
- `98bc6b1c` Define benchmarking and profiling plan for store-index generation

Generator / implementation slices under this tracker:
- `c5e9bb39` Ticket store index generator with git hook integration
- `b9757ba7` Spec store hierarchy generator
- `9336a096` Rule store catalog generator
- `855a1e5d` Audit store status summary generator
- `a72e3aca` Test store catalog generator (still gated on test-api + log-api bootstrap)
- `c2409055` Memory workspace DAG indexing

## Review gate

Before any generator ticket is moved forward again, review should confirm that the six planning blockers above capture:
- domain-owned generator placement instead of memory-api specialization
- rendering-pipeline integration with existing generator infrastructure
- efficient `peek-cli` / bounded-read consumption and optional LOD handling
- explicit performance budgets and profiling evidence for commit-time execution