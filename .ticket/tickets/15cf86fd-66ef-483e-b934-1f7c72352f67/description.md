Residual static_complexity follow-up split out of batch 1c9e7b3e after singleton reductions.

Scope:
- context-stack/tools/cli/context-cli/src/output.rs
- Remaining findings in this file: 2

Goal:
Reduce the paired complexity findings in CLI output helpers while preserving current human-readable formatting.

Validation:
- rtk cargo check -p context-cli
- subtree audit refresh for context-stack static_complexity

Parent context:
Batch 1c9e7b3e reduced from 38 to 17 findings before splitting this residual cluster.