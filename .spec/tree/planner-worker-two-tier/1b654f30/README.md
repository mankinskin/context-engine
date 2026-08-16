<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=1b654f30-d1a4-4cb4-ab2e-8355dfe5a758 slug=agent-orchestration/planner-worker-two-tier digest=31f51192944a -->

# Two-tier Planner/Worker model routing architecture

- slug: `agent-orchestration/planner-worker-two-tier`
- component: agent-orchestration
- scope: internal
- state: agent-orchestration
- index_ref: `.spec/specs/1b654f30-d1a4-4cb4-ab2e-8355dfe5a758/spec.toml`

## Summary

Flattened two-tier orchestration: **Planner/Architect** (frontier model, plans once) and **Worker** (fast/cheap model, executes exactly one isolated step), replacing the multi-hop chain (T0 orchestra…

## Acceptance Criteria Excerpt

1. **Boundary table is complete and traceable, not merely present.** Every row in the Worker capability boundary table either states a mechanically enforceable field-level constraint (`target_path`, `allowed_tools`, `return_contract.shape`) or links the shipped instruction file …

## Navigation

- Parent: _(root)_
- Children: _(none)_
