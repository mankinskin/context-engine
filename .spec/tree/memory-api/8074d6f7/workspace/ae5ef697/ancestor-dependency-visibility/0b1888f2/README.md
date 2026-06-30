<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=0b1888f2-7e59-45fb-95d8-1bf14ff7747f slug=ticket-api/workspaces/ancestor-dependency-visibility digest=910e8ac31f63 -->

# Ancestor Workspace Ticket References for Child-Workspace Dependencies

- slug: `ticket-api/workspaces/ancestor-dependency-visibility`
- component: ticket-api
- scope: public
- state: draft
- index_ref: `memory-api/.spec/specs/0b1888f2-7e59-45fb-95d8-1bf14ff7747f/spec.toml`

## Summary

Child ticket workspaces need a way to surface ancestor-owned ticket entries when those parent entries participate directly in dependency relationships with child-owned tickets.

## Acceptance Criteria Excerpt

A child workspace can resolve dependency endpoints owned by an ancestor workspace without dropping the relationship. Mixed local and ancestor dependency results preserve explicit workspace ownership per returned ticket reference. Dependency and graph consumers can render parent-…

## Navigation

- Parent: [memory-api/workspace](../../README.md)
- Siblings: [ticket-api/workflow/best-next-ordering](../../best-next-ordering/ec22fe34/README.md), [ticket-api/workflow/unblocked-by-discovery](../../unblocked-by-discovery/0386c4d0/README.md)
- Children: _(none)_
