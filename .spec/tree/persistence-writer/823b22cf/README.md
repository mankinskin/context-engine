<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=823b22cf-c0dc-46c6-a03d-00cdd3c4c83a slug=memory-api/session-api/persistence-writer digest=d51215a4ea38 -->

# session-api persistence writer

- slug: `memory-api/session-api/persistence-writer`
- component: session-api
- scope: internal
- state: draft
- index_ref: `memory-api/.spec/specs/823b22cf-c0dc-46c6-a03d-00cdd3c4c83a/spec.toml`

## Summary

Persist `session-api` capture requests into a deterministic filesystem layout that can become the first memory-api-backed session store.

## Acceptance Criteria Excerpt

1. `session-api` can persist a capture request into the planned filesystem layout. 2. The write path creates the session directory and writes stable JSON files for metadata and transcript content. 3. Error handling distinguishes serialization and filesystem failures. 4. Focused …

## Navigation

- Parent: _(root)_
- Children: _(none)_
