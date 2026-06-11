## Goal

Provide one canonical roadmap tracker for the memory-index work so implementation proceeds in a single explicit order instead of a loose set of related tickets.

This roadmap assumes the architecture boundary already reviewed in this track:
- `memory-api` stays a generic backend library
- each domain owns a thin generator in its own crate or CLI surface
- shared rendering, digest, validation, and hook infrastructure are reused rather than reimplemented per domain

## Roadmap order

### Phase 0 — Foundations already completed
1. `0dba399a` Define IndexEntry schema and serde contract
2. `e7a0ee3c` IndexEntry TOON sidecar format and validator

These are complete and remain the base prerequisites for every remaining slice.

### Phase 1 — Planning and architecture contracts
3. `94c56f3d` Define domain-owned thin generator architecture for store indexes
4. `db667eed` Define shared rendering pipeline integration for generated indexes
5. `7f7fe4a8` Define domain digest input contract for generated index entries
6. `d3a95908` Define peek-cli and level-of-detail validation for generated indexes
7. `98bc6b1c` Define benchmarking and profiling plan for store-index generation
8. `52dfd793` Define git hook automation for store-index regeneration

Phase 1 must finish before any new generator implementation resumes. The purpose is to lock down placement, rendering, digest stability, bounded-consumption validation, and commit-time performance constraints first.

### Phase 2 — First implementation reference path
9. `9336a096` Rule store catalog generator

This is the first implementation slice because it most directly exercises the rendering-pipeline decision and gives the track one reference generator for committed file rendering.

### Phase 3 — First hook-backed domain generator
10. `c5e9bb39` Ticket store index generator with git hook integration

The ticket generator follows the rule generator so the track can prove hook-backed regeneration on a core store after the rendering reference path exists.

### Phase 4 — Extend the pattern to another summary domain
11. `855a1e5d` Audit store status summary generator

Audit follows the ticket generator so the same shared pattern is exercised on another non-hierarchical domain before the track moves into hierarchy-heavy output.

### Phase 5 — Hierarchical generator after the simpler reference slices
12. `b9757ba7` Spec store hierarchy generator

Spec comes after rule, ticket, and audit because it adds the most demanding traversal and markdown-link structure.

### Phase 6 — Workspace graphing after local store patterns are proven
13. `c2409055` Memory workspace DAG indexing

Workspace DAG indexing comes after the local domain generator pattern is proven, so it can build on stable workspace-local outputs rather than forcing earlier generators to guess the final workspace shape.

### Phase 7 — Externally gated final slice
14. `a72e3aca` Test store catalog generator

The test generator remains last because it is additionally gated on the external bootstrap tickets for test-api and log-api evidence identities and because it benefits from the earlier validation and workspace decisions already being proven.

## Execution rule

Only the earliest unfinished roadmap ticket should move into implementation. Later slices stay parked until every earlier roadmap step is done or explicitly re-sequenced here.

## Dependency policy

The roadmap order is encoded directly in the ticket graph:
- the planning chain is serialized from architecture through hook automation
- the implementation chain is serialized from rule to ticket to audit to spec to workspace to test
- the tracker depends on the entire ordered sequence

## Acceptance criteria for this tracker

- The roadmap order is explicit in the tracker description
- The graph encodes that order through `depends_on` edges between roadmap steps
- Generator work cannot advance ahead of unresolved planning or earlier implementation tickets
- The roadmap remains the single source of truth for the implementation sequence

## Non-goals

- Does not add new generator features beyond sequencing the existing ticket set
- Does not replace per-ticket implementation details or acceptance criteria
- Does not force closure of already-completed foundation tickets