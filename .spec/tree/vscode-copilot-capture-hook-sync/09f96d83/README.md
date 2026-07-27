<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=09f96d83-4795-4f19-9259-64ad0d452387 slug=context-engine/session-api/vscode-copilot-capture-hook-sync digest=cbff8dcb64e2 -->

# VS Code Copilot capture-hook session sync

- slug: `context-engine/session-api/vscode-copilot-capture-hook-sync`
- component: session-api
- scope: internal
- state: in-review
- index_ref: `.spec/specs/09f96d83-4795-4f19-9259-64ad0d452387/spec.toml`

## Summary

<!-- aligned-structure:v1 -->

## Acceptance Criteria Excerpt

1. Hook commands invoke the capture executable directly via cargo using the renamed binary `copilot-capture-hook`. 2. The executable supports stdin hook mode (`--from-hook-stdin`) and captures periodically without transcript rewrite errors during normal sync. 3. Transcript inges…

## Navigation

- Parent: _(root)_
- Children: _(none)_
