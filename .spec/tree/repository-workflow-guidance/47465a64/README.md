<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=47465a64-0c5f-4ddc-8d38-018048090af2 slug=context-engine/repository-workflow-guidance digest=cb5e00b8ff6d -->

# repository workflow guidance

- slug: `context-engine/repository-workflow-guidance`
- component: context-engine
- scope: internal
- state: context-engine
- index_ref: `.spec/specs/47465a64-0c5f-4ddc-8d38-018048090af2/spec.toml`

## Summary

Repository guidance is partly rule-generated today, but nested workspaces still carry hand-written agent files and the parent workspace duplicates child target definitions directly in its own `rule-t…

## Acceptance Criteria Excerpt

A parent `rule-targets` config can import child workspace target configs with relative paths A parent `rule-targets` config can import a directory of child config fragments with relative paths Loading a config merges local and imported targets deterministically and rejects dupli…

## Navigation

- Parent: _(root)_
- Children: _(none)_
