<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=5de125ad-eb0c-4bcb-8e6d-175df1ba33a6 slug=rule-api/workspaces/nested-resolution digest=e8c84cf7d90e -->

# Nested Workspace Discovery and Target Resolution

- slug: `rule-api/workspaces/nested-resolution`
- component: rule-api
- scope: public
- state: draft
- index_ref: `memory-api/.spec/specs/5de125ad-eb0c-4bcb-8e6d-175df1ba33a6/spec.toml`

## Summary

Nested rule workspaces should extend the existing `rule-api` store and target model from "one store + one config" into an explicitly scanned workspace graph. The owning repo workspace remains the uni…

## Acceptance Criteria Excerpt

`memory-viewers/` can generate parent targets using rules from `memory-api/` and `viewer-api/` child workspaces after `rule scan` persists those child roots. A nested repo can generate its own targets without loading unrelated parent outputs. Before a scan persists child roots, …

## Navigation

- Parent: [rule-api/workspaces](../../README.md)
- Siblings: [rule-api/workspaces/memory-api-readme-generation](../../memory-api-readme-generation/3b96ec1c/README.md)
- Children: _(none)_
