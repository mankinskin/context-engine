# Root Join Refactoring Progress

**Date:** 2025-01-06  
**Status:** In Progress - 7/10 tests passing (3 insert tests failing)

## Summary

Implemented new `root.rs` module for root node joining with support for Prefix, Postfix, and Infix modes. The implementation correctly handles split children and creates wrapper partitions for prefix/postfix cases.

## What Works

### Passing Tests (3 insert tests + 4 other tests)
- `insert_postfix1` - Split child at target start, creates `bcd = [b, cd]`, wrapper `abcd = [[a, bcd], [ab, cd]]`
- `insert_prefix1` - Split child at target end, creates `hel = [he, l]`, wrapper `held = [[hel, d], [he, ld]]`
- `insert_infix2` - Simple infix without inner structure requirements

### Implementation Highlights
1. **Postfix Join** (`join_postfix_root`):
   - Identifies target children from offset to end
   - Detects split children via `ctx.ctx.splits` cache
   - Creates: inner partition (cd), target (bcd), wrapper (abcd)
   - Updates root with wrapper, returns target

2. **Prefix Join** (`join_prefix_root`):
   - Mirrors postfix but from start to offset
   - Uses split LEFT for target (not right)
   - Same pattern: inner, target, wrapper

3. **Infix Join** (`join_infix_root`):
   - Handles both left and right boundaries
   - Uses split left/right halves appropriately
   - Includes remainder tokens in root replacement

## What Doesn't Work

### Failing Tests (3)
1. **`insert_infix1`** - Expects `aby = [ab, y]` but gets `aby = [a, b, y]`
   - Root cause: Consecutive atoms `a, b` need to be joined into `ab` first
   - Fix needed: Inner structure preservation before final join

2. **`insert_pattern1`** - "Single index pattern" error
   - Root cause: Wrapper becomes the only child of root (not allowed)
   - Fix needed: Special case when wrapper spans entire root

3. **`insert_pattern2`** - Similar to pattern1

## Key Insights

1. **Splits are Pre-computed**: The `ctx.ctx.splits` cache already contains all split information from inner node processing

2. **Wrapper vs Target**: 
   - Target = the actual insertion result (returned)
   - Wrapper = contains alternative patterns (used to update root)

3. **Width Consistency**: When replacing in root, the replacement tokens must have the same total width as the replaced range

## Files Changed

- `crates/context-insert/src/join/context/node/root.rs` (NEW, ~500 lines)
  - `join_root_partitions()` - entry point
  - `join_postfix_root()` - postfix handling
  - `join_prefix_root()` - prefix handling  
  - `join_infix_root()` - infix handling

- `crates/context-insert/src/join/context/node/context.rs`
  - `join_root_partitions()` now delegates to `root.rs`
  - Legacy functions marked `#[allow(dead_code)]`

- `crates/context-insert/src/join/context/node/mod.rs`
  - Added `pub mod root;`

## Next Steps

1. **Fix inner structure preservation** for `insert_infix1`:
   - Need to join consecutive atoms before creating target
   - May need to use existing `JoinPartition` infrastructure

2. **Handle wrapper-equals-root** for `insert_pattern1/2`:
   - Special case when the wrapper is the entire root
   - May need to modify the root itself rather than creating a child

3. **Clean up legacy code** after all tests pass

## Test Commands

```bash
# Run all insert tests
cargo test -p context-insert "insert_" -- --nocapture

# Run specific test with tracing
LOG_STDOUT=1 LOG_FILTER=trace cargo test -p context-insert insert_postfix1 -- --nocapture
```
