<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=709f067a-21b6-41b6-8879-3cacef4bacaf slug=memory-api/session-api/runtime-session-context digest=dcbc78caddf6 -->

# Runtime session-context model (pinned entities, init/pin/unpin/view)

- slug: `memory-api/session-api/runtime-session-context`
- component: session-api
- scope: internal
- state: draft
- index_ref: `.spec/specs/709f067a-21b6-41b6-8879-3cacef4bacaf/spec.toml`

## Summary

<!-- aligned-structure:v2 -->

## Acceptance Criteria Excerpt

1. Fresh initialization creates a durable session manifest with session ID, schema version, timestamps, empty pins, and initial run lineage. 2. Resume preserves pins and adds a distinct linked run without reusing the outgoing run ID before finish; resume rejects after finish. 3.…

## Navigation

- Parent: [memory-api/session-api/dynamic-session-bootstrapping](../../README.md)
- Siblings: [memory-api/curation/entity-usage-and-feedback](../../entity-usage-and-feedback/71b81a55/README.md), [memory-api/session-api/cascade-context-gathering](../../cascade-context-gathering/fda5c915/README.md), [memory-api/session-api/durable-session-workflow](../../durable-session-workflow/c677182e/README.md), [memory-api/session-api/minimal-bootstrapper-selective-loading](../../minimal-bootstrapper-selective-loading/a28a88db/README.md), [session-api/execution-track](../../execution-track/7b277ba4/README.md)
- Children: _(none)_
