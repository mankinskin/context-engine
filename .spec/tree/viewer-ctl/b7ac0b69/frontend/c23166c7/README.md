<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=c23166c7-2315-4b89-9160-cde7df3086e6 slug=viewer-ctl/lifecycle/frontend digest=0e9fd5f55a4e -->

# frontend lifecycle

- slug: `viewer-ctl/lifecycle/frontend`
- component: viewer-ctl
- scope: internal
- state: draft
- index_ref: `viewer-api/.spec/specs/c23166c7-2315-4b89-9160-cde7df3086e6/spec.toml`

## Summary

Frontends are static-asset bundles produced by `trunk` (Dioxus/WASM) or

## Acceptance Criteria Excerpt

A frontend with no `prebuild` array installs cleanly on a fresh checkout provided its build dependencies are present (npm/trunk). A failing prebuild or build step preserves enough child output in `viewer-ctl`'s own error text for non-interactive callers to diagnose missing prere…

## Navigation

- Parent: [viewer-ctl](../../README.md)
- Siblings: [viewer-ctl/cli](../../cli/4dafde12/README.md), [viewer-ctl/config](../../config/3fa36e9e/README.md), [viewer-ctl/install-layout](../../install-layout/afe17aef/README.md), [viewer-ctl/lifecycle/extension](../../extension/b568bb7a/README.md), [viewer-ctl/lifecycle/server](../../server/351e65fe/README.md), [viewer-ctl/lifecycle/task](../../task/01f7eae8/README.md), [viewer-ctl/process-management](../../process-management/86bb3a01/README.md)
- Children: _(none)_
