<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=a969562b-6920-4fa6-a757-d317e7d442df slug=rule-system/single-target-reverse-sync digest=41dcc80134eb -->

# rule-cli single-target reverse-sync from generated artifacts

- slug: `rule-system/single-target-reverse-sync`
- component: rule-cli
- scope: internal
- state: draft
- index_ref: `.spec/specs/a969562b-6920-4fa6-a757-d317e7d442df/spec.toml`

## Summary

Add a reverse-sync workflow for rule-generated artifacts so edits in a generated file can be written back to the originating rule bodies by canonical rule id.

## Acceptance Criteria Excerpt

1. `sync-rules --file` rejects non-generated files with a clear guard error. 2. `sync-rules --file` rejects spec-doc artifacts with a clear unsupported error. 3. `sync-rules --file` updates only existing rules referenced by entry ids. 4. Unknown entry ids fail with an orphan-id …

## Navigation

- Parent: _(root)_
- Children: _(none)_
