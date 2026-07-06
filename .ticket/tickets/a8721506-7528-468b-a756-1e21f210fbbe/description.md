Residual static_complexity follow-up split out of batch 1c9e7b3e after singleton reductions.

Scope:
- context-stack/context-trace/src/graph/search_path.rs
- Remaining findings in this file: 2

Goal:
Reduce the paired complexity findings in search_path transition/application helpers while preserving path graph semantics.

Validation:
- rtk cargo check -p context-trace
- subtree audit refresh for context-stack static_complexity

Parent context:
Batch 1c9e7b3e reduced from 38 to 17 findings before splitting this residual cluster.