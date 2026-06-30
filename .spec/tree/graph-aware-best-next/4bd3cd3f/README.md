<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=4bd3cd3f-5851-4d9e-b499-978cb7b53275 slug=ticket-api/workflow/graph-aware-best-next digest=b97571f234d9 -->

# Graph-aware best-next ranking and dependency convergence

- slug: `ticket-api/workflow/graph-aware-best-next`
- component: ticket-api
- scope: public
- state: draft
- index_ref: `memory-api/.spec/specs/4bd3cd3f-5851-4d9e-b499-978cb7b53275/spec.toml`

## Summary

The current best-next contract is deterministic but shallow: default next discovery ranks only dependency-satisfied candidates by candidate workflow state, priority, immediate dependees, and recency.…

## Acceptance Criteria Excerpt

Given an eligible `new` or `ready` prerequisite that blocks an `in-implementation` or `in-review` dependent, default next ranks that prerequisite ahead of otherwise similar candidates that do not unblock more advanced work. CLI and MCP next surfaces produce the same ordering and…

## Navigation

- Parent: _(root)_
- Children: [ticket-api/workflow/blocker-trees-and-recently-unblocked-ordering](blocker-trees-and-recently-unblocked-ordering/d702ed9e/README.md)
