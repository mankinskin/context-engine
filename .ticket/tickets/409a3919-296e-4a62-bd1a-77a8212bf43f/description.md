Residual static_complexity follow-up split out of batch 1c9e7b3e after singleton reductions.

Scope:
- context-stack/context-insert/src/split/cache/vertex.rs
- Remaining findings in this file: 4

Goal:
Reduce the clustered complexity findings in split/cache/vertex.rs while preserving cache partition logic.

Validation:
- rtk cargo check -p context-insert
- subtree audit refresh for context-stack static_complexity

Parent context:
Batch 1c9e7b3e reduced from 38 to 17 findings before splitting this residual cluster.