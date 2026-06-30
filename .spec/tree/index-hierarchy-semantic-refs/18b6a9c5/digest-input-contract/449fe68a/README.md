<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=449fe68a-541c-4804-bbfd-476af783f80c slug=generated-context/digest-input-contract digest=9438d05156ed -->

# Domain digest input contract for generated index entries

- slug: `generated-context/digest-input-contract`
- component: memory-api
- scope: internal
- state: draft
- index_ref: `.spec/specs/449fe68a-541c-4804-bbfd-476af783f80c/spec.toml`

## Summary

Define the domain-level digest input contract for generated memory-index entries so every generator derives a stable `IndexEntry` payload before calling `seal()`. Given identical source inputs, every…

## Acceptance Criteria Excerpt

A per-domain normalization contract exists for ticket, spec, rule, audit, and workspace generators. The contract names the exact source fields and normalization rules used before `compute_digest()` / `seal()`. Stable-ID rules are documented for synthetic entries (audit root, wor…

## Navigation

- Parent: [generated-context/index-hierarchy-semantic-refs](../../README.md)
- Siblings: [generated-context/benchmarking-profiling-plan](../../benchmarking-profiling-plan/c598ddb2/README.md), [generated-context/git-hook-automation](../../git-hook-automation/53c70cae/README.md), [generated-context/peek-lod-validation](../../peek-lod-validation/c4f7b0ae/README.md), [generated-context/rendering-pipeline-integration](../../rendering-pipeline-integration/9109f12a/README.md), [generated-context/thin-generator-architecture](../../thin-generator-architecture/bf217ce5/README.md)
- Children: _(none)_
