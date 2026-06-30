<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=5b404022-6a67-4395-90e0-1e4282fd83b4 slug=audit-api/workspace-graph-health-and-board-check-in-validation digest=b18a6a7b70ee -->

# audit-api: workspace graph health and board check-in validation

- slug: `audit-api/workspace-graph-health-and-board-check-in-validation`
- component: audit-api
- state: audit-api
- index_ref: `.spec/specs/5b404022-6a67-4395-90e0-1e4282fd83b4/spec.toml`

## Summary

Current topology checks can detect orphan tickets and planned convergence risks, but they do not enforce whether dependency requirements are defined, whether required dependency evidence is passing, …

## Acceptance Criteria Excerpt

Audit and ticket health contracts account for dependency evidence status, not topology alone. Board check-in warning behavior is specified for missing or unsatisfied dependency requirements. The audit surface degrades gracefully when test-store data is unavailable. Severity mapp…

## Navigation

- Parent: _(root)_
- Children: _(none)_
