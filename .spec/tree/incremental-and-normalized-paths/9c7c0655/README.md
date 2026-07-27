<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=9c7c0655-436c-4cd8-a3d3-2d893f1d865c slug=rule-cli/sync-targets/incremental-and-normalized-paths digest=f36e45812b21 -->

# rule-cli sync-targets: incremental work and normalized path output

- slug: `rule-cli/sync-targets/incremental-and-normalized-paths`
- component: rule-cli
- state: draft
- index_ref: `memory-api/.spec/specs/9c7c0655-436c-4cd8-a3d3-2d893f1d865c/spec.toml`

## Summary

<!-- aligned-structure:v1 -->

## Acceptance Criteria Excerpt

1. Path normalization: every path field in `sync-targets` output (`config`, `generated[].output`, `removed[].output`) uses `/` separators on all hosts. A unit test asserts no `\\` appears in any emitted path on Windows-style inputs. `generate-target` and MCP `generate` payloads …

## Navigation

- Parent: _(root)_
- Children: _(none)_
