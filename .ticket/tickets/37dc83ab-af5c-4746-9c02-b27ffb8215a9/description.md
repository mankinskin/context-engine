## Problem

Full-text search across the spec store (and any store backed by the same Tantivy index path) is non-functional. `spec scan --force` panics inside Tantivy and incremental scans silently fail to populate a searchable index.

### Reproduction

```bash
./target/debug/spec.exe scan --index-root "$(pwd)/.spec" --force
```

Panics with:

```
thread '...' panicked at tantivy-0.22.1/src/fastfield/writer.rs:137:
index out of bounds: the len is 5 but the index is 5
```

An incremental (non-force) scan succeeds and reports `integrated: 436, pruned: 0`, but search still returns zero results:

```bash
./target/debug/spec.exe search "thin generator"   # -> count: 0
./target/debug/spec.exe search "memory"            # -> count: 0
./target/debug/spec.exe search "rendering"         # -> count: 0
```

The same zero-result behavior reproduces through the MCP `spec_search` interface.

## Impact

- Full-text spec search returns `count: 0` for every query.
- Writes, `spec_get` by id/slug, and `spec scan` (incremental) still work — specs persist to disk and SQLite, only the Tantivy search index is broken.
- This forces all spec discovery to go through id/slug lookups, defeating the discovery-before-creating workflow.

## Scope

- Root cause is the `fastfield/writer.rs` index-out-of-bounds in Tantivy 0.22.1.
- Affects spec-api search; verify whether ticket-api / other Tantivy-backed stores share the failure.

## Candidate fixes

- Upgrade Tantivy past 0.22.1 to a release where the fastfield writer panic is fixed.
- If upgrade is blocked, identify the schema/fastfield shape that triggers the len==index overflow and apply a workaround (e.g. adjust the offending fast field definition).

## Acceptance Criteria

- `spec scan --force` completes without panicking.
- `spec search "<term>"` returns non-zero results for terms known to exist in the store.
- MCP `spec_search` returns matching specs.
- Regression coverage confirms full-text search stays functional after a force reindex.

## Non-goals

- Does not change the spec/ticket data model or digest contract.
- Does not alter the store-index generation track (memory-index).