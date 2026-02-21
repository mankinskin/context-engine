---
tags: `#analysis` `#context-trace` `#context-search` `#debugging` `#testing`
summary: The `prefix1` test fails because top-down cache entries are recorded at position 5 instead of position 4. The `range1` test passes with correct pos...
---

# Analysis: PrefixEnd Top-Down Cache Position Bug

**Date:** 2025-11-27  
**Status:** Root cause identified, ready for implementation

## Problem Summary

The `prefix1` test fails because top-down cache entries are recorded at position 5 instead of position 4. The `range1` test passes with correct positions at 3. Both tests have similar structure but create different `PathCoverage` variants during query exhaustion.

### Test Details

**prefix1 test:**
- Query: `[a, bc, d, e]` (4 tokens, width 5 atoms)
- Expected top-down cache position: **4** (where we entered the end path traversal)
- Actual top-down cache position: **5** (one past, using end_pos)
- PathCoverage created: `PrefixEnd` (start_path empty, end_path non-empty)

**range1 test:**
- Query: `[bc, d, e]` (3 tokens, width 4 atoms)
- Expected/actual top-down cache position: **3** (correct!)
- PathCoverage created: `RangeEnd` (both start_path and end_path non-empty)

## Root Cause

### The Issue

`PrefixEnd` lacks a `root_pos` field, so `PrefixCommand::trace()` uses `end_pos` for top-down edges:

```rust
// PrefixCommand::trace() - WRONG
let exit_key = DownKey {
    pos: self.end_pos.into(),  // ❌ Uses end_pos (5)
    index: root_exit.parent,
};
let target = DownKey {
    index: *ctx.trav.graph().expect_child_at(root_exit),
    pos: exit_key.pos,  // ❌ Also end_pos (5)
};
```

Compare to `RangeCommand::trace()` - CORRECT:

```rust
// RangeCommand::trace() - CORRECT
let exit_key = DownKey {
    pos: self.root_pos.0.into(),  // ✓ Uses root_pos (3)
    index: root_exit.parent,
};
let target = DownKey {
    pos: self.root_pos.0.into(),  // ✓ Also root_pos (3)
    index: *ctx.trav.graph().expect_child_at(root_exit),
};
```

### Why PrefixEnd Doesn't Have root_pos

Looking at `PrefixEnd` struct:

```rust
pub struct PrefixEnd {
    pub(crate) path: IndexEndPath,
    pub(crate) target: DownKey,
    pub(crate) end_pos: AtomPosition,
    // ❌ Missing: root_pos field
}
```

Compare to `RangeEnd`:

```rust
pub struct RangeEnd {
    pub(crate) path: IndexRangePath,
    pub(crate) target: DownKey,
    pub(crate) root_pos: AtomPosition,  // ✓ Has root_pos
    pub(crate) end_pos: AtomPosition,
}
```

### Call Chain Analysis

Both tests follow the same call chain:

1. **Query exhaustion** in `advance_query()` (advance.rs:251)
   - Logs show: `root_pos=4, checkpoint.atom_position=5, end_pos=5` (prefix1)
   - Logs show: `root_pos=3, checkpoint.atom_position=4, end_pos=4` (range1)

2. **PathCoverage creation** in `from_range_path()` (end/mod.rs:82)
   - prefix1: `start_at_border=true, start_path_empty=true, end_at_border=true, end_path_empty=false`
     → Creates `PrefixEnd` with only `end_pos=5`
   - range1: `start_at_border=true, start_path_empty=false, end_at_border=true, end_path_empty=false`
     → Creates `RangeEnd` with `root_pos=3` and `end_pos=4`

3. **Trace command execution**
   - prefix1: `PrefixCommand::trace()` uses `end_pos=5` for TD edges ❌
   - range1: `RangeCommand::trace()` uses `root_pos=3` for TD edges ✓

### Why This Matters

Top-down cache positions must indicate **where the parent starts** in the trace, not where the child ends. This enables:
- Consistent cache lookups across different traversal paths
- Incremental tracing that knows which parent position led to which child
- Proper understanding of "we were at position X in the parent when we entered this child"

Using `end_pos` (which is `checkpoint.atom_position` = one past the last matched atom) is semantically wrong. We need `root_pos` (the position where we entered the matched root/parent).

## Execution Plan

### Step 1: Add root_pos to PrefixEnd

**File:** `crates/context-search/src/state/end/prefix.rs`

```rust
pub struct PrefixEnd {
    pub(crate) path: IndexEndPath,
    pub(crate) target: DownKey,
    pub(crate) root_pos: AtomPosition,  // NEW
    pub(crate) end_pos: AtomPosition,
}
```

**Updates needed:**
- Update `CompactFormat` implementations to show `root_pos`
- Update `From<&PrefixEnd> for PrefixCommand` to pass `root_pos`

### Step 2: Update PrefixCommand to use root_pos

**File:** `crates/context-trace/src/trace/command.rs`

```rust
pub struct PrefixCommand {
    pub path: IndexEndPath,
    pub add_edges: bool,
    pub root_pos: AtomPosition,  // NEW
    pub end_pos: AtomPosition,
}
```

**Update trace implementation:**

```rust
impl Traceable for PrefixCommand {
    fn trace<G: HasGraph>(self, ctx: &mut TraceCtx<G>) {
        let root_exit = self.path.role_root_child_location::<End>();
        
        // Use root_pos for TD edges (like RangeCommand does)
        let exit_key = DownKey {
            pos: self.root_pos.into(),  // CHANGED from end_pos
            index: root_exit.parent,
        };
        let target = DownKey {
            index: *ctx.trav.graph().expect_child_at(root_exit),
            pos: exit_key.pos,  // Now uses root_pos
        };
        
        let new = NewTraceEdge::<TopDown> {
            target,
            prev: exit_key,
            location: root_exit,
        };
        ctx.cache.add_state(new, self.add_edges);

        TraceRole::<End>::trace_sub_path(
            ctx,
            &self.path,
            target,
            self.add_edges,
        );
    }
}
```

### Step 3: Update PrefixEnd creation

**File:** `crates/context-search/src/state/end/mod.rs`

In `from_range_path()`, when creating `PrefixEnd`:

```rust
(true, true, false, _) | (true, true, true, false) =>
    PathCoverage::Prefix(PrefixEnd {
        path: path.into(),
        target,
        root_pos,  // ADDED - pass through root_pos
        end_pos,
    }),
```

### Step 4: Update PathCoverage::root_key()

**File:** `crates/context-search/src/state/end/mod.rs`

Update the `RootKey` implementation to return `root_pos` for `PrefixEnd`:

```rust
impl RootKey for PathCoverage {
    fn root_key(&self) -> UpKey {
        UpKey::new(
            self.root_parent(),
            match self {
                PathCoverage::Range(s) => s.root_pos.into(),
                PathCoverage::Postfix(p) => p.root_pos.into(),
                PathCoverage::Prefix(p) => p.root_pos.into(),  // CHANGED from 0.into()
                PathCoverage::EntireRoot(_) => 0.into(),
            },
        )
    }
}
```

### Step 5: Update PrefixEnd conversion

**File:** `crates/context-search/src/state/end/prefix.rs`

Update the `From` implementation:

```rust
impl From<&PrefixEnd> for PrefixCommand {
    fn from(value: &PrefixEnd) -> Self {
        PrefixCommand {
            add_edges: true,
            path: value.path.clone(),
            root_pos: value.root_pos,  // ADDED
            end_pos: value.end_pos,
        }
    }
}
```

### Step 6: Update display implementations

Update `CompactFormat` implementations for both `PrefixEnd` and `PrefixCommand` to show `root_pos`.

## Expected Outcome

After these changes:
- prefix1 test: top-down cache entries at position **4** ✓
- range1 test: still at position **3** ✓ (no change, already correct)
- All other tests: should continue to pass (PrefixEnd/PrefixCommand rare in test suite)

## Files to Modify

1. `crates/context-search/src/state/end/prefix.rs` - Add root_pos field, update From impl
2. `crates/context-trace/src/trace/command.rs` - Add root_pos to PrefixCommand, update trace()
3. `crates/context-search/src/state/end/mod.rs` - Pass root_pos when creating PrefixEnd, update root_key()

## Validation

```bash
KEEP_LOGS=1 cargo test -p context-search prefix1 -- --nocapture
KEEP_LOGS=1 cargo test -p context-search range1 -- --nocapture
cargo test -p context-search
```

Expected: All tests pass with correct cache positions.

## Notes

- This mirrors the fix already applied to `RangeCommand` (see comments in command.rs:218-219)
- The pattern is: **top-down edges use root_pos, bottom-up edges use root_pos (for the parent)**
- `end_pos` should only be used for the final cursor position, not for cache edge positions
