<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=d702ed9e-f75c-4727-8f05-1b2b244ec74f slug=ticket-api/workflow/blocker-trees-and-recently-unblocked-ordering digest=719dff81e5e4 -->

# Blocker trees and recently-unblocked workflow ordering

- slug: `ticket-api/workflow/blocker-trees-and-recently-unblocked-ordering`
- component: ticket-api
- scope: public
- state: draft
- index_ref: `memory-api/.spec/specs/d702ed9e-f75c-4727-8f05-1b2b244ec74f/spec.toml`

## Summary

The current workflow surface has two strong but separate pieces:

## Acceptance Criteria Excerpt

`ticket blockers <id>` returns a nested upstream tree with all deep blockers and emphasizes frontier leaves. `ticket unblocked-by <id>` returns a nested downstream tree that preserves direct parent-child structure and exposes frontier leaves for quick follow-up work. Parent node…

## Navigation

- Parent: [ticket-api/workflow/graph-aware-best-next](../../README.md)
- Children: _(none)_
