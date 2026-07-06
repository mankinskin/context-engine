# Goal
Triage `dead_code` compiler warnings in the `context-stack` submodule (split off from parent `9347c9f8` mechanical pass).

# Scope (host `cargo check --workspace` counts)
- ~9 `dead_code` warnings + 1 `unused_features` warning.
- Files:
  - `context-stack/context-insert/src/lib.rs` (dead_code 1 + `#![feature(slice_index_methods)]` unused_features 1)
  - `context-stack/context-read/src/complement.rs` (2: `ComplementBuilder` struct + `build`/`build_trace_cache_stub` methods)
  - `context-stack/context-read/src/expansion/chain/mod.rs` (4)
  - `context-stack/context-read/src/expansion/chain/link.rs` (1)
  - `context-stack/context-read/src/expansion/link.rs` (1)

# Approach
- Per item, decide: delete truly-dead code, or annotate intentional scaffolding with `#[allow(dead_code)]` + rationale comment.
- Do NOT remove code that is macro-consumed or WIP without confirming.

# Validation
- `cargo check -p context-insert -p context-read` clean of dead_code/unused_features for touched files.
- `cargo test -p context-read` passes.

# Notes
- Submodule commit: changes land in `context-stack`, then update pointer in root repo.
- Mechanical warnings (unused_mut/imports/variables) already resolved by parent `9347c9f8`.