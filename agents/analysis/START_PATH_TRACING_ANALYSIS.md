# Start Path Tracing Analysis

## Overview

This document analyzes how start_path tracing works during parent exploration in the find_ancestor search flow, and identifies where the trace cache update bug occurs.

## Graph Structure (Env2)

```
atoms: c, d, e, f, g, h, i

patterns (hierarchical):
  cd = [c, d]                    (pattern c_d_id)
  hi = [h, i]                    (pattern h_i_id)
  efg = [e, f, g]                (pattern e_f_g_id)
  cdefg = [cd, efg]              (pattern cd_efg_id)
  efghi = [efg, hi]              (pattern efg_hi_id)
  cdefghi = [cdefg, hi] OR [cd, efghi]  (two patterns: cdefghi_ids[0], cdefghi_ids[1])
```

## Query Flow

**Query:** `[d, e, f, g, h]` → finds ancestor `cdefghi`

### Search Progression

1. **Initial match**: Starts at `d` (position 0)
2. **Matches in `cd`**: `d` (pos 1) matches
3. **Enters `cdefg`**: Continues matching through `e, f, g`
4. **Reaches boundary**: At position 4, child exhausted, needs parent exploration
5. **Parent exploration**: Finds new root `cdefghi`

## Start Path Tracing Flow

### When Parent Exploration is Triggered

File: `crates/context-search/src/match/root_cursor/advance.rs:359`

```rust
pub(crate) fn create_parent_exploration_state(&self) -> MatchResult {
    let checkpoint = self.state.query.checkpoint();
    let checkpoint_child = self.state.child.checkpoint();

    // Use checkpoint_child path as it represents the matched state
    let mut path = checkpoint_child.child_state.path.clone();
    let entry_pos = checkpoint_child.child_state.entry_pos;
    let exit_pos = checkpoint_child.child_state.exit_pos;

    // Simplify path to remove redundant segments
    path.child_path_mut::<Start, _>().simplify(&self.trav);
    // ...
}
```

**Key Point**: The `path` here is an `IndexRangePath` which contains:
- `root`: The current root we're matching in (e.g., `cdefg`)
- `start`: RolePath from entry point upward (bottom-up)
- `end`: RolePath from exit point downward (top-down)

### Trace Execution

File: `crates/context-trace/src/trace/traceable/mod.rs:148-171`

```rust
impl IntoRootCommand<Start> for RangeCommand {
    type RootCommand = PostfixRootCommand;
    fn root_command<G: HasGraph>(
        &self,
        ctx: &mut TraceCtx<G>,
    ) -> Self::RootCommand {
        let first = self.path.role_leaf_token_location::<Start>().unwrap();
        let start_index = *ctx.trav.graph().expect_child_at(first);
        let initial_prev = UpKey {
            index: start_index,
            pos: start_index.width().0.into(),
        };
        let mut sub_path_prev =
            self.path.trace_start_sub_path(ctx, initial_prev);  // Line 161
        // Update position to match entry_pos after tracing sub-path
        sub_path_prev.pos = self.entry_pos;  // Line 163 ← THE BUG

        let root_entry = self.path.role_root_child_location::<Start>();
        PostfixRootCommand {
            root_entry,
            prev: sub_path_prev,
            entry_pos: self.entry_pos,
        }
    }
}
```

### The Bug

**Line 161**: `trace_start_sub_path` traces the path stored in `path.start`
- In our case: `d` (pos 1) → `cd` (pos 1)
- Returns: `UpKey { index: "cd", pos: UpPosition(AtomPosition(1)) }`

**Line 163**: Position is overwritten with `entry_pos` (= 4)
- Changes to: `UpKey { index: "cd", pos: UpPosition(AtomPosition(4)) }`

**PostfixRootCommand**: Uses this as `prev`
- Creates edge from `cd` at position 4 to `cdefghi` at position 4
- **SKIPS** the intermediate parent `cdefg`!

### What Should Happen

The bottom-up path from the query position to the new root should be:
```
d (pos 1) → cd (pos 1) → cdefg (pos 4) → cdefghi (pos 4)
         ^              ^                ^
         |              |                |
      traced by     MISSING!         traced by
    start_sub_path                 PostfixRootCommand
```

The intermediate parent `cdefg` should be added to the cache during this traversal.

## Where Start Path Tracing is Used

### 1. Parent Exploration (find_ancestor)

File: `crates/context-search/src/search/mod.rs:240-243`

```rust
RootEndResult::Inconclusive(need_parent_cursor) => {
    // Root boundary reached - need parent exploration
    let checkpoint_state = need_parent_cursor
        .create_parent_exploration_state();
    // ...
}
```

When root cursor exhausted but query continues, creates a `MatchResult` with path coverage that will be traced at the end.

### 2. Final State Tracing

File: `crates/context-search/src/search/mod.rs:168-169`

```rust
let trace_ctx = &mut self.matches.trace_ctx;
end.trace(trace_ctx);
```

The final matched state (including parent exploration state) is traced, which calls:
- `MatchResult::trace` → `PathCoverage::trace` → `RangeEnd::trace` → `RangeCommand::trace`

### 3. Trace Cache Population

File: `crates/context-trace/src/trace/traceable/mod.rs:219-237`

```rust
impl Traceable for RangeCommand {
    fn trace<G: HasGraph>(
        self,
        ctx: &mut TraceCtx<G>,
    ) {
        let exit_key =
            IntoRootCommand::<Range>::root_command(&self, ctx).trace_root(ctx);
        self.path.trace_end_sub_path(ctx, exit_key);
    }
}
```

This:
1. Traces start_sub_path (bottom-up from query position)
2. Traces root entry (from last matched parent to new root) ← **BUG HERE**
3. Traces end_sub_path (top-down from root)

## Expected Cache Entries

For query `[d, e, f, g, h]` finding `cdefghi`:

| Vertex | Position | Direction | Child | Pattern | Sub-Index | Status |
|--------|----------|-----------|-------|---------|-----------|--------|
| d | - | - | - | - | - | ✓ Found |
| cd | 1 | BU | d | c_d_id | 1 | ✓ Found |
| **cdefg** | **1** | **BU** | **cd** | **cd_efg_id** | **0** | **❌ MISSING** |
| h | - | - | - | - | - | ✓ Found |
| hi | 4 | TD | h | h_i_id | 0 | ✓ Found |
| cdefghi | 4 | BU | cdefg | cdefghi_ids[0] | 0 | ✓ Found |
| cdefghi | 4 | TD | hi | cdefghi_ids[0] | 1 | ✓ Found |

**Note**: `efghi` was initially in the expected cache but confirmed as incorrect by @mankinskin. It should NOT be in the cache because the top-down exploration from the root doesn't traverse through `efghi` - it goes directly to `hi`.

## Root Cause Summary

**Problem**: When creating `PostfixRootCommand` during parent exploration, the code overwrites the position of the last traced vertex with the entry position of the new root. This creates a direct edge that skips intermediate parents.

**Location**: `crates/context-trace/src/trace/traceable/mod.rs:163`

**Solution**: Need to trace the full bottom-up path from the last matched parent through ALL intermediate parents to the new root, not just update the position.

## Potential Fix Approaches

### Option 1: Extend start_path to include intermediate parents

Modify `create_parent_exploration_state` to build a complete start_path that includes all parents from the last match up to the new root.

### Option 2: Trace intermediate parents separately

After tracing the sub_path, traverse upward from the result through all parents until reaching the new root, adding each to the cache.

### Option 3: Don't change position after tracing

Instead of changing `sub_path_prev.pos` to `entry_pos`, trace the remaining path from the sub_path result to the entry position, capturing all intermediate vertices.

## Next Steps

1. Determine the best approach for the fix
2. Implement the fix to ensure `cdefg` is added to the cache
3. Run the test to verify it passes
4. Check for any other similar issues in the codebase
