<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=ec22fe34-2d24-4dc5-a067-85121bed3655 slug=ticket-api/workflow/best-next-ordering digest=0f96e1409baa -->

# Cross-interface best-next ordering

- slug: `ticket-api/workflow/best-next-ordering`
- component: ticket-api
- scope: public
- state: draft
- index_ref: `memory-api/.spec/specs/ec22fe34-2d24-4dc5-a067-85121bed3655/spec.toml`

## Summary

Best-next-ticket discovery must remain consistent anywhere the repository surfaces candidate work.

## Acceptance Criteria Excerpt

`ticket next` returns higher-`dependee_count` tickets ahead of lower-`dependee_count` tickets when state and priority are equal, even when the lower-`dependee_count` ticket is newer. `ticket board show` recommends higher-`dependee_count` tickets ahead of lower-`dependee_count` t…

## Navigation

- Parent: [memory-api/workspace](../../README.md)
- Siblings: [ticket-api/workflow/unblocked-by-discovery](../../unblocked-by-discovery/0386c4d0/README.md), [ticket-api/workspaces/ancestor-dependency-visibility](../../ancestor-dependency-visibility/0b1888f2/README.md)
- Children: _(none)_
