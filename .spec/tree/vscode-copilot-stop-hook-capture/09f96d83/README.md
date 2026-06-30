<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=09f96d83-4795-4f19-9259-64ad0d452387 slug=context-engine/session-api/vscode-copilot-stop-hook-capture digest=8c5db5a1d295 -->

# VS Code Copilot stop-hook session capture

- slug: `context-engine/session-api/vscode-copilot-stop-hook-capture`
- component: session-api
- scope: internal
- state: draft
- index_ref: `.spec/specs/09f96d83-4795-4f19-9259-64ad0d452387/spec.toml`

## Summary

Wire the repository's VS Code GitHub Copilot hook configuration to persist chat sessions through `session-api` after each agent response stops.

## Acceptance Criteria Excerpt

1. Workspace settings point Copilot at `.github/hooks/docs-validation.json` for this checkout. 2. `.github/hooks/docs-validation.json` includes a `Stop` hook command in addition to the current `PostToolUse` commands. 3. The `Stop` hook command reads the hook input and transcript…

## Navigation

- Parent: _(root)_
- Children: _(none)_
