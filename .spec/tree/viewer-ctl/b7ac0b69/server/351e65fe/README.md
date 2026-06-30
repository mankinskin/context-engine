<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=351e65fe-0629-4a0f-9c19-27dabb36b72f slug=viewer-ctl/lifecycle/server digest=3c98c4fa4961 -->

# server lifecycle

- slug: `viewer-ctl/lifecycle/server`
- component: viewer-ctl
- scope: internal
- state: draft
- index_ref: `viewer-api/.spec/specs/351e65fe-0629-4a0f-9c19-27dabb36b72f/spec.toml`

## Summary

Servers are long-running Rust binaries that bind a TCP port. viewer-ctl owns

## Acceptance Criteria Excerpt

`start` never silently kills a server it did not just launch unless that server is bound to the configured port. `start` works on a clean machine: a missing binary triggers exactly one recovery `install`, no infinite loop. `stop` exits 0 when nothing is listening on the port. On…

## Navigation

- Parent: [viewer-ctl](../../README.md)
- Siblings: [viewer-ctl/cli](../../cli/4dafde12/README.md), [viewer-ctl/config](../../config/3fa36e9e/README.md), [viewer-ctl/install-layout](../../install-layout/afe17aef/README.md), [viewer-ctl/lifecycle/extension](../../extension/b568bb7a/README.md), [viewer-ctl/lifecycle/frontend](../../frontend/c23166c7/README.md), [viewer-ctl/lifecycle/task](../../task/01f7eae8/README.md), [viewer-ctl/process-management](../../process-management/86bb3a01/README.md)
- Children: _(none)_
