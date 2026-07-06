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

## Validation
Commands run:
- `cargo check -p context-insert -p context-read`
- `cargo check -p context-insert -p context-read --message-format=short | grep ...`
- `cargo test -p context-read`

Results:
- Scoped compile check passed.
- Scoped warning recount for the ticket file set is effectively **0** (no matching warnings remained in the targeted file set).
- `cargo test -p context-read` still fails in read/overlap behavior tests (`read_infix1`, `read_infix2`, `read_multiple_overlaps1`, `read_repeating_known1`, `sync_read_text2`, `repetition_aabbaabb`, `validate_mixed_pattern`, `complex_abcabababcaba`). Those are runtime behavior failures owned by the next stability batch [`f2d8f807`](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/f2d8f807-447e-41a2-80db-2fca03d5b9ee/ticket.toml) (`test_execution`), not by this dead_code cleanup slice. The deleted modules were proven unreferenced (`grep` found no remaining `ComplementBuilder` / `ExpansionLink` usages).

# Acceptance
- Scoped dead_code + unused_features findings resolved. ✓
- No broader compile regression introduced. ✓
- Test failures remain as pre-existing next-batch stability work, documented for handoff. ✓