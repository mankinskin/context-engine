<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=53c70cae-731b-41b5-bd1a-1de9a98eb36f slug=generated-context/git-hook-automation digest=25999655f5e0 -->

# Git hook automation for store-index regeneration

- slug: `generated-context/git-hook-automation`
- component: memory-api
- scope: internal
- state: draft
- index_ref: `.spec/specs/53c70cae-731b-41b5-bd1a-1de9a98eb36f/spec.toml`

## Summary

Define the repository-local git-hook automation contract for store-index regeneration so generator tickets have one concrete execution surface instead of each hand-waving at "pre-commit/post-commit h…

## Acceptance Criteria Excerpt

`.githooks/pre-commit` has (or will have, per H6) an explicit repository-local branch for store-index generation, scaffolded as a guarded no-op until generators exist. The trigger matrix names staged-path patterns and generated outputs per domain (H2). Git hooks are clearly dist…

## Navigation

- Parent: [generated-context/index-hierarchy-semantic-refs](../../README.md)
- Siblings: [generated-context/benchmarking-profiling-plan](../../benchmarking-profiling-plan/c598ddb2/README.md), [generated-context/digest-input-contract](../../digest-input-contract/449fe68a/README.md), [generated-context/peek-lod-validation](../../peek-lod-validation/c4f7b0ae/README.md), [generated-context/rendering-pipeline-integration](../../rendering-pipeline-integration/9109f12a/README.md), [generated-context/thin-generator-architecture](../../thin-generator-architecture/bf217ce5/README.md)
- Children: _(none)_
