# Status: TODO

# Decision Shortlist And Next Steps

## Shortlist

### Primary recommendation: Filesystem + SQLite + Tantivy + Event Log

Why:
- Best balance of transactional safety, query expressiveness, and human-readable artifact storage.
- Matches your requirement to keep JSON/TOML/binary files side-by-side in ticket folders.
- Supports CLI/HTTP naturally via existing command adapter pattern.

### Secondary recommendation: Filesystem + redb + Tantivy + Event Log

Why:
- Strong all-Rust path.
- Good when you want to avoid C dependencies and stay embedded-native.

Cost:
- More custom query/index logic than SQL path.

## Proposed Phased Plan

1. Phase 0: Design contracts
- Ticket manifest schema, folder schema, event schema.

2. Phase 1: Minimal working backend
- Implement ticket create/update/save with validation and atomic writes.
- Add dependency edges and basic query listing.

3. Phase 2: History and rollback
- Event log, snapshots, and rollback command set.

4. Phase 3: Search and highlighting
- Tantivy indexing, snippets, and filters.

5. Phase 4: Advanced references and visualization
- Cross-ticket graph views and validation overlays.

## Evaluation Criteria

- Correctness under crash/failure
- Migration ergonomics
- Query power for dependencies/workflow views
- Search quality/latency
- Operational simplicity (single-machine default)
- Extensibility for future distributed sync

## Risks

- Dual-write complexity (filesystem + database)
- Schema/version drift
- Search index staleness if updates are async
- Locking/concurrency edge cases

## TODO

- TODO: Build proof-of-concept benchmark suite.
- TODO: Select initial backend between SQLite and redb.
- TODO: Write ADR documenting source-of-truth and transaction semantics.
- TODO: Define acceptance tests for crash-safety and recovery.
