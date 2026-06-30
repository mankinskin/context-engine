<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=86bb3a01-ef29-4c7b-905b-9582a0d75f40 slug=viewer-ctl/process-management digest=3336598cb686 -->

# process management

- slug: `viewer-ctl/process-management`
- component: viewer-ctl
- scope: internal
- state: draft
- index_ref: `viewer-api/.spec/specs/86bb3a01-ef29-4c7b-905b-9582a0d75f40/spec.toml`

## Summary

viewer-ctl needs to find and terminate processes that occupy a TCP port.

## Acceptance Criteria Excerpt

`pids_on_port` returns an empty vector — never an error — when no tool on the platform can identify the listener. `pids_on_port` works on a Windows install with non-English UI language (PowerShell path is preferred precisely for this reason). `kill_process` is monotonic: the fun…

## Navigation

- Parent: [viewer-ctl](../../README.md)
- Siblings: [viewer-ctl/cli](../../cli/4dafde12/README.md), [viewer-ctl/config](../../config/3fa36e9e/README.md), [viewer-ctl/install-layout](../../install-layout/afe17aef/README.md), [viewer-ctl/lifecycle/extension](../../extension/b568bb7a/README.md), [viewer-ctl/lifecycle/frontend](../../frontend/c23166c7/README.md), [viewer-ctl/lifecycle/server](../../server/351e65fe/README.md), [viewer-ctl/lifecycle/task](../../task/01f7eae8/README.md)
- Children: _(none)_
