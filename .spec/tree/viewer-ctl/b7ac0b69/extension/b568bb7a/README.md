<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=b568bb7a-6726-46ac-bb78-fbc1858da4b8 slug=viewer-ctl/lifecycle/extension digest=e477c0547013 -->

# extension lifecycle

- slug: `viewer-ctl/lifecycle/extension`
- component: viewer-ctl
- scope: internal
- state: draft
- index_ref: `viewer-api/.spec/specs/b568bb7a-6726-46ac-bb78-fbc1858da4b8/spec.toml`

## Summary

VS Code extensions are TypeScript projects compiled to `out/`. viewer-ctl

## Acceptance Criteria Excerpt

A first-time install on a machine without `vsce` errors out cleanly with the missing-binary message instead of partial state. A subsequent install always uses the fast path so iteration stays sub-second for the common case (`out/` only). The user is reminded after both paths tha…

## Navigation

- Parent: [viewer-ctl](../../README.md)
- Siblings: [viewer-ctl/cli](../../cli/4dafde12/README.md), [viewer-ctl/config](../../config/3fa36e9e/README.md), [viewer-ctl/install-layout](../../install-layout/afe17aef/README.md), [viewer-ctl/lifecycle/frontend](../../frontend/c23166c7/README.md), [viewer-ctl/lifecycle/server](../../server/351e65fe/README.md), [viewer-ctl/lifecycle/task](../../task/01f7eae8/README.md), [viewer-ctl/process-management](../../process-management/86bb3a01/README.md)
- Children: _(none)_
