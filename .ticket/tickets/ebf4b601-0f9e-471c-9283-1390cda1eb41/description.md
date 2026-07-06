Residual static_complexity follow-up split out of batch 1c9e7b3e after singleton reductions.

Scope:
- context-stack/context-trace/src/logging/tracing_utils/debug_to_json.rs
- Remaining findings in this file: 5

Goal:
Reduce the clustered complexity findings in debug_to_json helpers without changing parsing behavior.

Validation:
- rtk cargo check -p context-trace
- subtree audit refresh for context-stack static_complexity

Parent context:
Batch 1c9e7b3e reduced from 38 to 17 findings before splitting this residual cluster.