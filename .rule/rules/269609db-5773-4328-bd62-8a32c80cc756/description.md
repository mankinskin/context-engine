## Acceptance criteria

- This spec no longer treats a separate wrapper documentation command path as the target surface.
- `doc-api` owns documentation-validation metadata and `doc-cli` is defined as the primary CLI surface.
- Generated-guidance checks and manual documentation verification are captured in native workflow metadata.
- Unsupported or partial documentation coverage is explicit in the doc-owned model.
- Any existing wrapper implementation is explicitly described as migration context rather than target architecture.