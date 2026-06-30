<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=0386c4d0-15c4-4561-a33f-63b881c852c5 slug=ticket-api/workflow/unblocked-by-discovery digest=dec79d8bcd09 -->

# CLI reverse-dependency unlock and blocker follow-up discovery

- slug: `ticket-api/workflow/unblocked-by-discovery`
- component: ticket-cli
- scope: public
- state: draft
- index_ref: `memory-api/.spec/specs/0386c4d0-15c4-4561-a33f-63b881c852c5/spec.toml`

## Summary

The ticket CLI needs first-class reverse-dependency workflow support: `ticket unblocked-by <id>` should show which dependents a prerequisite unlocks or still affects, and `ticket next <id>` should sh…

## Acceptance Criteria Excerpt

A dependent ticket blocked only by the queried ticket is returned. A dependent ticket with at least one other unresolved blocker is excluded from the actionable `items` list and surfaced in `still_blocked_items` instead. A transitive dependent is returned only after all blockers…

## Navigation

- Parent: [memory-api/workspace](../../README.md)
- Siblings: [ticket-api/workflow/best-next-ordering](../../best-next-ordering/ec22fe34/README.md), [ticket-api/workspaces/ancestor-dependency-visibility](../../ancestor-dependency-visibility/0b1888f2/README.md)
- Children: _(none)_
