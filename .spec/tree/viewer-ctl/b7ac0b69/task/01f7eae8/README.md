<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=01f7eae8-555d-46e2-bb54-0e0bf2b2da90 slug=viewer-ctl/lifecycle/task digest=c31e96f5a042 -->

# tasks

- slug: `viewer-ctl/lifecycle/task`
- component: viewer-ctl
- scope: internal
- state: draft
- index_ref: `viewer-api/.spec/specs/01f7eae8-555d-46e2-bb54-0e0bf2b2da90/spec.toml`

## Summary

A task is an ordered list of shell command invocations. Tasks are the

## Acceptance Criteria Excerpt

A task with zero steps prints a header and a `done.` line. An `allow_failure = false` step that exits non-zero stops the task and exits viewer-ctl with status 1. An `allow_failure = true` step that exits non-zero produces a warning and the task continues with the next step.

## Navigation

- Parent: [viewer-ctl](../../README.md)
- Siblings: [viewer-ctl/cli](../../cli/4dafde12/README.md), [viewer-ctl/config](../../config/3fa36e9e/README.md), [viewer-ctl/install-layout](../../install-layout/afe17aef/README.md), [viewer-ctl/lifecycle/extension](../../extension/b568bb7a/README.md), [viewer-ctl/lifecycle/frontend](../../frontend/c23166c7/README.md), [viewer-ctl/lifecycle/server](../../server/351e65fe/README.md), [viewer-ctl/process-management](../../process-management/86bb3a01/README.md)
- Children: _(none)_
