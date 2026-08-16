<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=8c880efc-7083-4e1d-bf06-96b8254be913 slug=memory-api/session-api/dynamic-session-bootstrapping digest=e537e75f58d0 -->

# Dynamic session bootstrapping and just-in-time context routing

- slug: `memory-api/session-api/dynamic-session-bootstrapping`
- component: session-api
- scope: internal
- state: draft
- index_ref: `.spec/specs/8c880efc-7083-4e1d-bf06-96b8254be913/spec.toml`

## Summary

<!-- aligned-structure:v2 -->

## Acceptance Criteria Excerpt

1. A session initializes and resumes idempotently under the Copilot UUID across distinct runs before finish; ordinary init after finish is byte-stable and resume rejects. 2. Pinned entities and workflow nodes/edges persist independently of transcript capture. 3. Agents can add d…

## Navigation

- Parent: _(root)_
- Children: [memory-api/curation/entity-usage-and-feedback](entity-usage-and-feedback/71b81a55/README.md), [memory-api/session-api/cascade-context-gathering](cascade-context-gathering/fda5c915/README.md), [memory-api/session-api/durable-session-workflow](durable-session-workflow/c677182e/README.md), [memory-api/session-api/minimal-bootstrapper-selective-loading](minimal-bootstrapper-selective-loading/a28a88db/README.md), [memory-api/session-api/runtime-session-context](runtime-session-context/709f067a/README.md), [session-api/execution-track](execution-track/7b277ba4/README.md)
