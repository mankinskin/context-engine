<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=3b96ec1c-4e99-48f4-86e5-a36ba24b827a slug=rule-api/workspaces/memory-api-readme-generation digest=2119b92204da -->

# memory-api Rule Workspace and README Generation

- slug: `rule-api/workspaces/memory-api-readme-generation`
- component: rule-api
- scope: public
- state: draft
- index_ref: `memory-api/.spec/specs/3b96ec1c-4e99-48f4-86e5-a36ba24b827a/spec.toml`

## Summary

`memory-api` needs its own repo-local rule workspace so the repo README and local usage guides are authored next to the crates and tools they describe. The local target config should stay manageable …

## Acceptance Criteria Excerpt

`memory-api/` contains a repo-local rule workspace and `rule-targets.yaml`. The README target is defined locally and renders to `memory-api/README.md`. The local target config can be expressed as a file/folder tree with outputs grouped by root files, CLI tools, MCP tools, and HT…

## Navigation

- Parent: [rule-api/workspaces](../../README.md)
- Siblings: [rule-api/workspaces/nested-resolution](../../nested-resolution/5de125ad/README.md)
- Children: _(none)_
