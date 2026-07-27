<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=d702ed9e-f75c-4727-8f05-1b2b244ec74f slug=ticket-api/workflow/blocker-trees-and-recently-unblocked-ordering digest=9810fbce503d -->

# Blocker trees and recently-unblocked workflow ordering

- slug: `ticket-api/workflow/blocker-trees-and-recently-unblocked-ordering`
- component: ticket-api
- scope: public
- state: draft
- index_ref: `memory-api/.spec/specs/d702ed9e-f75c-4727-8f05-1b2b244ec74f/spec.toml`

## Summary

<!-- aligned-structure:v1 -->

## Acceptance Criteria Excerpt

`ticket blockers <id>` returns a nested upstream tree with all deep blockers and emphasizes frontier leaves. `ticket unblocked-by <id>` returns a nested downstream tree that preserves direct parent-child structure and exposes frontier leaves for quick follow-up work. Parent node…

## Navigation

- Parent: [ticket-api/workflow/graph-aware-best-next](../../README.md)
- Children: _(none)_
