<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=6e63979a-f29b-4c6f-a4b7-5264fd9c29d4 slug=memory-api/cli/toon-format-support digest=a1ba6b40accd -->

# Add TOON format support across the memory-api CLI suite

- slug: `memory-api/cli/toon-format-support`
- component: memory-api
- scope: internal
- state: draft
- index_ref: `.spec/specs/6e63979a-f29b-4c6f-a4b7-5264fd9c29d4/spec.toml`

## Summary

Add a compact TOON machine-readable format alongside existing JSON output across the memory-api CLI suite.

## Acceptance Criteria Excerpt

1. Each CLI supports `--toon` output for successful command payloads and formatted errors. 2. `--json` behavior remains backward compatible. 3. The existing structured file-based input path for spec field maps accepts either JSON or TOON. 4. Focused tests cover TOON output and T…

## Navigation

- Parent: _(root)_
- Children: _(none)_
