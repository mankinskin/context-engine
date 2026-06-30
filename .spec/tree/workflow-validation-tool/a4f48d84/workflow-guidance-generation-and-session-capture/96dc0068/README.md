<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=96dc0068-d05d-4e61-b785-144272119fa9 slug=context-engine/workflow-guidance-generation-and-session-capture digest=d6853009e5a2 -->

# workflow guidance generation and session capture scaffolding

- slug: `context-engine/workflow-guidance-generation-and-session-capture`
- component: context-engine
- scope: internal
- state: context-engine
- index_ref: `.spec/specs/96dc0068-d05d-4e61-b785-144272119fa9/spec.toml`

## Summary

Generate the requested workflow prompt and agent files from canonical rule-api entries, then seed a bounded first session-api scaffold for storing Copilot chat sessions in the memory-api store.

## Acceptance Criteria Excerpt

1. The requested prompts and agents are generated from canonical rule entries and checked in as generated files. 2. Root rule-target config covers the new prompt and agent outputs with minimal additional structure. 3. Focused validation proves the rule-target wiring and generate…

## Navigation

- Parent: [context-engine/workflow-validation-tool](../../README.md)
- Children: [context-engine/handoff-workflow-prompts](handoff-workflow-prompts/9e04ff58/README.md)
