<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=36fd7849-65eb-405e-8cc5-70440f0cb7c2 slug=memory-api/session-api/hook-ingestion-read-query digest=cce5d8f774b1 -->

# session-api hook ingestion and read query

- slug: `memory-api/session-api/hook-ingestion-read-query`
- component: session-api
- scope: internal
- state: draft
- index_ref: `memory-api/.spec/specs/36fd7849-65eb-405e-8cc5-70440f0cb7c2/spec.toml`

## Summary

Extend `session-api` so repeated Copilot hook captures preserve transcript history as an append-only log and expose a first read/query API over the persisted store.

## Acceptance Criteria Excerpt

1. Persisting a later capture for the same session never removes or replaces earlier transcript turns; only the new suffix is added. 2. `session-api` can read a persisted session back into a `SessionRecord`. 3. `session-api` can query stored sessions by simple metadata and trans…

## Navigation

- Parent: _(root)_
- Children: _(none)_
