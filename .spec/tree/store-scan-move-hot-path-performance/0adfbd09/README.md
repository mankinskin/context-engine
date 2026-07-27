<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=0adfbd09-15c7-46ee-be24-03da0564833d slug=memory-api/store-scan-move-hot-path-performance digest=2124204fc96a -->

# Store scan and move hot-path performance for ticket-backed indexing

- slug: `memory-api/store-scan-move-hot-path-performance`
- component: memory-api
- scope: internal
- state: draft
- index_ref: `.spec/specs/0adfbd09-15c7-46ee-be24-03da0564833d/spec.toml`

## Summary

<!-- aligned-structure:v1 -->

## Acceptance Criteria Excerpt

1. A non-reindex scan avoids reprocessing unchanged ticket entries when the metadata/search index is already healthy. 2. Bulk scan integration does not pay one Tantivy commit and merge wait per ticket document. 3. Ticket move execution avoids forcing full source and target store…

## Navigation

- Parent: _(root)_
- Children: _(none)_
