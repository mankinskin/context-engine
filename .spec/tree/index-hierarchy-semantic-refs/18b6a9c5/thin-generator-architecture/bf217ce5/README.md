<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=bf217ce5-8890-4749-9a2d-deffb6d0f4dd slug=generated-context/thin-generator-architecture digest=d789ec0c4865 -->

# Domain-owned thin generator architecture for store indexes

- slug: `generated-context/thin-generator-architecture`
- component: memory-api
- scope: internal
- state: draft
- index_ref: `.spec/specs/bf217ce5-8890-4749-9a2d-deffb6d0f4dd/spec.toml`

## Summary

Define the architecture boundary for store-index generation so each domain owns a **thin** generator while `memory-api` exposes only reusable, domain-agnostic infrastructure. An implementer must be a…

## Acceptance Criteria Excerpt

The contract explicitly states `memory-api` is generic infrastructure only and does not own domain generators (C1). The split of responsibilities between `memory-api` and domain crates is enumerated (C1, C2). The required extension points are identified and shown to avoid any `m…

## Navigation

- Parent: [generated-context/index-hierarchy-semantic-refs](../../README.md)
- Siblings: [generated-context/benchmarking-profiling-plan](../../benchmarking-profiling-plan/c598ddb2/README.md), [generated-context/digest-input-contract](../../digest-input-contract/449fe68a/README.md), [generated-context/git-hook-automation](../../git-hook-automation/53c70cae/README.md), [generated-context/peek-lod-validation](../../peek-lod-validation/c4f7b0ae/README.md), [generated-context/rendering-pipeline-integration](../../rendering-pipeline-integration/9109f12a/README.md)
- Children: _(none)_
