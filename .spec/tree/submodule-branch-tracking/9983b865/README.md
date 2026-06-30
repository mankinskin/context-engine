<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=9983b865-5082-437a-945a-05c26a56c113 slug=context-engine/submodule-branch-tracking digest=9314e36b132f -->

# submodule branch tracking workflow

- slug: `context-engine/submodule-branch-tracking`
- component: context-engine
- scope: internal
- state: active
- index_ref: `.spec/specs/9983b865-5082-437a-945a-05c26a56c113/spec.toml`

## Summary

Top-level submodules in the repository currently land in detached HEAD state, which makes local commits easy to create without advancing the intended `main` branch in each submodule.

## Acceptance Criteria Excerpt

`.gitmodules` records the intended `main` branch for maintained top-level submodules Repository guidance includes a standard command that switches maintained submodules to local branches tracking `origin/main` Focused validation demonstrates the changed configuration and the int…

## Navigation

- Parent: _(root)_
- Children: _(none)_
