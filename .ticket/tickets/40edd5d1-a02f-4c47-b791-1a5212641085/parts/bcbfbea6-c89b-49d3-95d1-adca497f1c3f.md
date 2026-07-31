# Goal
Triage `dead_code` compiler warnings in the `context-stack` submodule (split off from parent `9347c9f8` mechanical pass).

# Result
Resolved the full scoped warning set for this ticket:
- `context-stack/context-insert/src/lib.rs` — removed unused `#![feature(slice_index_methods)]`
- `context-stack/context-read/src/complement.rs` — deleted unreferenced dead module
- `context-stack/context-read/src/expansion/link.rs` — deleted unreferenced dead module
- `context-stack/context-read/src/expansion/mod.rs` / `lib.rs` — removed module wiring for deleted dead paths
- `context-stack/context-read/src/expansion/chain/mod.rs` — added focused `#[allow(dead_code)]` on intentional Pass C3 overlap-chain scaffolding (`anchor_token`, `end_bound`, `has_overlap`, `single_token`, `append`, `set_overlap`, `OverlapChain`, `push`, `into_chain`)
- `context-stack/context-read/src/expansion/chain/link.rs` — added focused `#[allow(dead_code)]` on `BandCapLink` pending Pass C3 cap support

## Decision log
- Deleted only code proven unreferenced (`ComplementBuilder` / `ExpansionLink` module path).
- Kept chain-oriented overlap scaffolding and silenced it narrowly per policy (a), because docs/comments in the file explicitly mark it as deferred Pass C3 work.

