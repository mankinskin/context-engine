---
tags: `#plan` `#context-read` `#algorithm` `#cursor` `#expansion` `#overlap`
summary: Complete the context-read crate to support iterative text indexing via search/insert orchestration with overlap detection and block expansion.
status: 📋
---

# Plan: Complete Context-Read Crate

**Date:** 2026-02-18
**Scope:** Large (multiple modules, algorithm redesign, 27 failing tests)
**Crate:** `context-read`

---

## Objective

Complete the context-read crate so it can **index any text into the hypergraph** by iteratively finding overlapping regions, inserting partial matches, and expanding a block token to represent the full input text. The core idea is **deduplication**: reuse existing graph structures to represent new input.

---

## Architecture Overview

### Crate Stack
```
context-read    ← We are here. Orchestrates the full reading algorithm.
  ↓ uses
context-insert  ← Insert partial matches (split-join). Working.
  ↓ uses
context-search  ← Find largest known prefix. Working.
  ↓ uses
context-trace   ← Foundational graph, paths, tracing. Working.
```

### Core Algorithm (Reading a Text)

```
Input text → atomize → segment(unknown, known) → for each segment:
  1. Append unknown atoms to root (trivial - just insert_pattern)
  2. Process known atoms via BlockExpansionCtx:
     a. Find largest prefix match in graph (insert_or_get_complete)
     b. Detect overlaps between last committed block and new match
     c. Commit block + overlap decompositions to root
     d. Advance cursor and repeat
  3. Final root token = deduplication-optimal representation of input
```

### Key Invariant
> Two nodes have a path between them **iff** one is a substring of the other.
>
> The graph stores only edges to closest neighbors (transitive reduction).

This means: after reading "abcabc", the graph contains both decompositions `[abc, abc]` AND any overlap-derived decompositions, with shared structure.

---

## Current State

### Test Results: 17 passing / 27 failing / 44 total

**Passing categories:**
- All purely linear reads (no repeats): `linear_read_abc`, `linear_read_single_char`, etc.
- Simple structural tests

**Failing categories:**
- **Any input with repeated substrings** (overlap detection not working):
  - `repetition_abab`, `repetition_abcabc`, `repetition_abcabcabc`, etc.
- **Multi-read scenarios** (incremental graph building):
  - `read_sequence1` ("hypergraph" then "hyper" then "graph")
  - `read_infix1` ("subdivision" then "visualization")
  - `read_multiple_overlaps1` ("abcde" then "bcdea" then "cdeab" etc.)
- **Known-segment processing** crashes or produces wrong structure

### Key Modules and Their Status

| Module | File(s) | Status | Notes |
|--------|---------|--------|-------|
| **SegmentIter** | `segment.rs` | ✅ Working | Splits input into unknown/known pairs |
| **ReadCtx** | `context/mod.rs` | ⚠️ Partial | Top-level iterator, calls read_segment |
| **RootManager** | `context/root.rs` | ⚠️ Partial | append_pattern/commit_state work, overlap logic fragile |
| **BlockExpansionCtx** | `expansion/block.rs` | ⚠️ Partial | Processing loop exists but produces wrong results |
| **ExpansionCtx** | `expansion/mod.rs` | ⚠️ Partial | Iterator for expansion ops, cursor advancement buggy |
| **CursorCtx** | `expansion/cursor.rs` | ❌ Minimal | Just a wrapper, no cursor advancement logic |
| **BandState** | `expansion/chain/mod.rs` | ⚠️ Partial | Single/WithOverlap enum, collapse logic exists |
| **ExpandCtx** | `expansion/chain/expand.rs` | ⚠️ Partial | Postfix iteration works, but insert call incorrect |
| **ComplementBuilder** | `complement.rs` | ⚠️ Incomplete | TODO: proper trace cache building |
| **OverlapStack** | `expansion/stack.rs` | ❌ Unused | Shell code with commented-out methods |
| **Bands/Policy** | `bands/` | ✅ Working | Prefix/postfix iterators functional |

---

## Root Cause Analysis

### Problem 1: Cursor Advancement Not Working

**Location:** `expansion/cursor.rs`, `expansion/mod.rs`

`CursorCtx` is just a wrapper around `(HypergraphRef, PatternRangePath)` with no logic to advance the cursor after a successful match/insert. The `ExpansionCtx` iterator updates `self.cursor.cursor` when applying an expansion, but:
- The cursor is only updated on `ChainOp::Expansion`, not on initial insert
- There's no mechanism to advance past the consumed prefix after `insert_or_get_complete`
- The `PatternRangePath` cursor doesn't track how much of the input has been consumed

**Impact:** After the first match, the cursor doesn't move forward, causing repeated matching of the same prefix or incorrect behavior.

### Problem 2: insert_or_get_complete Usage in ExpansionCtx

**Location:** `expansion/mod.rs:97-110` (ExpansionCtx::new)

The `new()` method calls `insert_or_get_complete` on the full cursor pattern, which:
1. Searches for the largest known prefix
2. Inserts a new token if partial match
3. Returns the result... but the cursor isn't properly advanced to account for what was matched

The returned `IndexWithPath` contains a `path` that should indicate the cursor state after the match, but it's not being used to advance the `PatternRangePath` cursor correctly.

### Problem 3: ExpandCtx Insert on Full Cursor

**Location:** `expansion/chain/expand.rs:57-62`

```rust
let result = match ToInsertCtx::<IndexWithPath>::insert(
    &self.ctx.graph,
    self.ctx.cursor.cursor.clone(),  // ← inserts FULL remaining cursor, not just overlap portion
)
```

This inserts the **entire remaining cursor pattern**, not the portion that overlaps with the postfix. The insert should be operating on the specific overlapping subsequence, not the full remaining input.

### Problem 4: BandState Collapse and Complement Logic

**Location:** `expansion/chain/mod.rs:170-220`, `complement.rs`

The collapse logic for `BandState::WithOverlap` builds prefix/postfix complements and inserts both decompositions. However:
- `ComplementBuilder` uses a minimal `TraceCache` (documented TODO)
- The complement boundary calculations may be incorrect
- `build_postfix_complement` has a TODO for partial token extraction

### Problem 5: Commit Flow in BlockExpansionCtx

**Location:** `expansion/block.rs:68-73`

```rust
while let Some(state) = self.ctx.next() {
    self.root.commit_state(state);
}
```

The loop commits each yielded state, but after committing, the next iteration should search against the **updated graph** (which now contains the newly inserted structure). Currently, the `ExpansionCtx` iterator may not see newly committed tokens because:
- The cursor isn't properly advanced
- The graph state for subsequent searches may be stale (though interior mutability should handle this)

---

## Proposed Work Items

### WI-1: Redesign CursorCtx with Proper Advancement

**Priority:** Critical
**Files:** `expansion/cursor.rs`, `expansion/mod.rs`

The cursor must track:
- The **full input pattern** (all known tokens to process)
- The **current atom position** (how many atoms have been consumed)
- A method to **advance** past a consumed prefix of width N

```rust
// Proposed CursorCtx design
pub(crate) struct CursorCtx {
    pub(crate) graph: HypergraphRef,
    pub(crate) full_pattern: Pattern,      // The complete known input
    pub(crate) position: AtomPosition,     // Current atom offset consumed
    pub(crate) cursor: PatternRangePath,   // Current range path for search
}

impl CursorCtx {
    /// Advance the cursor past `width` atoms.
    /// Updates `position` and rebuilds `cursor` from remaining tokens.
    pub(crate) fn advance(&mut self, width: TokenWidth) { ... }
    
    /// Check if cursor has been fully consumed.
    pub(crate) fn is_exhausted(&self) -> bool { ... }
    
    /// Get remaining pattern as a searchable slice.
    pub(crate) fn remaining(&self) -> &[Token] { ... }
}
```

**Questions:**
- Q1: Should `CursorCtx` rebuild its `PatternRangePath` on each advance, or should it work at the `Pattern` (token slice) level directly?
- Q2: Does the `PatternRangePath` need to be a range path, or can we simplify to just a `&[Token]` for the searchable interface?

### WI-2: Fix Insert-Then-Advance Flow in ExpansionCtx

**Priority:** Critical
**Files:** `expansion/mod.rs`, `expansion/block.rs`

The `ExpansionCtx::new()` and the iterator's `next()` must follow this flow:

```
1. Take remaining cursor pattern
2. Call insert_or_get_complete(remaining) → gets largest prefix token
3. Advance cursor past the matched prefix (by its width)
4. Use the matched token as the new "anchor" for overlap detection
5. Check postfixes of anchor for overlaps with new remaining pattern
6. If overlap found → yield BandState::WithOverlap
7. If no overlap → append anchor to band, loop back to step 1
```

Currently, step 3 is missing, causing the algorithm to stall.

### WI-3: Fix ExpandCtx to Insert Only Overlap Portion

**Priority:** High
**Files:** `expansion/chain/expand.rs`

The `ExpandCtx` iterator should not insert the full cursor. It should:
1. Get the postfix token of the current anchor
2. Check if the postfix's children match the beginning of the remaining cursor
3. If yes: search for a combined token `[postfix_children... + remaining]` or verify the overlap
4. The "expansion" is the resulting token that covers both the postfix and the next portion

**Questions:**
- Q3: Should the overlap check use `insert_or_get_complete` on a synthesized pattern `[overlap_region + remaining]`, or should it use ancestor search to find if such a combined token already exists?
- Q4: What exactly should be considered an "overlap"? Is it: the postfix of the anchor token matches the prefix of the remaining cursor pattern? (i.e., the last child of the anchor == the first atom(s) of the remaining cursor)

### WI-4: Fix ComplementBuilder

**Priority:** Medium
**Files:** `complement.rs`

The `TODO` in `build_complement_trace_cache` needs to be resolved. The complement builder extracts the "other side" of an overlap to create both decompositions. It needs:
1. A proper `TraceCache` built by searching the root up to the split point
2. Use `insert_init` with the correct `InitInterval` to extract the complement token

Since `insert_init` already handles this (it takes a root, cache, and end_bound), the fix may be straightforward: just build a proper trace cache via search.

### WI-5: Ensure Graph Mutations Are Visible for Subsequent Iterations

**Priority:** High
**Files:** `expansion/block.rs`, `context/root.rs`

After each block commit, the newly inserted tokens must be visible to subsequent search/insert calls. With interior mutability (`Arc<RwLock<Hypergraph>>`), this should work automatically. But we need to verify:
1. `commit_state` actually writes to the graph (not just to local state)
2. Subsequent `insert_or_get_complete` calls see newly inserted patterns
3. The `BandState::collapse` method properly inserts bundled tokens

### WI-6: Handle Edge Cases in RootManager::commit_state

**Priority:** Medium
**Files:** `context/root.rs`

The `append_collapsed` method has complex overlap detection logic (compound overlap, cursor-level overlap). This needs testing and possibly simplification:
- Atomic root that equals append[0] → creates decompositions
- Compound overlap: root's last child equals append[0]'s first child
- Standard case: just extend or create new pattern

**Questions:**
- Q5: Is the compound overlap detection in `append_collapsed` still needed, or does the `BandState::WithOverlap` path handle all overlap cases before reaching `commit_state`?
- Q6: Should `commit_state` be simplified to always just append (since overlaps are already captured in the BandState)?

### WI-7: Implement Test-Driven Iteration

**Priority:** Ongoing
**Files:** All test files

Work through tests in increasing complexity:
1. ✅ Linear cases (passing)
2. `repetition_abab` (simplest repeat)
3. `repetition_abcabc` (repeat with 3-atom unit)
4. `read_repeating_known1` ("xyyxy")
5. `read_sequence1` ("hypergraph" then "hyper" then "graph")
6. `read_multiple_overlaps1` (rotating overlaps)
7. `repetition_abcabcabc` (triple repeat with nested overlap)

---

## Key Code References

### Entry Points
- [ReadCtx::read_segment](crates/context-read/src/context/mod.rs#L72-L95) - Main per-segment processing
- [BlockExpansionCtx::process](crates/context-read/src/expansion/block.rs#L61-L75) - Block expansion loop
- [ExpansionCtx::next](crates/context-read/src/expansion/mod.rs#L42-L60) - Expansion iterator

### Cursor & State
- [CursorCtx](crates/context-read/src/expansion/cursor.rs) - Minimal cursor wrapper (needs redesign)
- [BandState](crates/context-read/src/expansion/chain/mod.rs#L13-L30) - Single/WithOverlap state machine
- [Band](crates/context-read/src/expansion/chain/band.rs) - Pattern with positional bounds

### Expansion & Overlap
- [ExpandCtx](crates/context-read/src/expansion/chain/expand.rs) - Postfix iteration for overlaps
- [OverlapLink](crates/context-read/src/expansion/chain/link.rs#L17-L32) - Overlap path tracking
- [ExpansionLink](crates/context-read/src/expansion/link.rs) - Prefix/postfix path link

### Insert Interface
- [ToInsertCtx::insert_or_get_complete](crates/context-insert/src/insert/mod.rs#L31-L35) - Find or insert token
- [InitInterval](crates/context-insert/src/interval/init.rs#L12-L16) - Search→insert conversion
- [InsertCtx::insert_init](crates/context-insert/src/insert/context.rs#L70-L80) - Insert from init interval

### Commit & Root
- [RootManager::commit_state](crates/context-read/src/context/root.rs#L99-L115) - Collapse and append
- [RootManager::append_collapsed](crates/context-read/src/context/root.rs#L120-L217) - Complex append with overlap detection
- [BandState::collapse](crates/context-read/src/expansion/chain/mod.rs#L152-L216) - Build decompositions

### Complement
- [ComplementBuilder::build](crates/context-read/src/complement.rs#L14-L46) - Extract complement token
- [build_prefix_complement](crates/context-read/src/expansion/chain/mod.rs#L222-L260) - Prefix complement for collapse
- [build_postfix_complement](crates/context-read/src/expansion/chain/mod.rs#L262-L335) - Postfix complement for collapse

### Tests (in order of complexity)
- [tests/linear.rs](crates/context-read/src/tests/linear.rs) - No-repeat tests (passing)
- [tests/overlapping.rs](crates/context-read/src/tests/overlapping.rs) - Triple repeat tests (failing)
- [tests/read/mod.rs](crates/context-read/src/tests/read/mod.rs) - Complex multi-read tests (failing)

---

## Walkthrough: Expected Algorithm for "ababab"

This is the simplest failing case that exercises the full algorithm.

### Input
```
Text: "ababab"
Atoms: a, b (both new on first read)
```

### Step 1: Segmentation
```
SegmentIter produces:
  NextSegment { unknown: [a, b], known: [a, b, a, b] }
```
Atoms `a` and `b` are new → first two are unknown. The remaining four are known (atoms exist after inserting unknowns).

### Step 2: Process Unknown
```
RootManager::append_pattern([a, b])
  → insert_pattern([a, b]) = "ab"
  → root = "ab"
```

### Step 3: BlockExpansionCtx with known=[a,b,a,b]

**3a. Initialize ExpansionCtx**
- Cursor = PatternRangePath over [a, b, a, b]
- BandState starts with initial token from insert_or_get_complete([a,b,a,b])
- insert_or_get_complete finds "ab" as largest prefix → returns "ab"
- **Advance cursor past 2 atoms** → remaining = [a, b]
- BandState = Single { band: [ab], start_bound: 0, end_bound: 2 }

**3b. Check for overlap (ExpandCtx)**
- anchor_token = "ab"
- Postfixes of "ab": "b" (last child of [a, b])
- Does "b" expand into remaining [a, b]? No, remaining starts with "a", not "b"
- No overlap found

**3c. Cursor not exhausted → search again**
- insert_or_get_complete([a, b]) → finds "ab" again
- **Advance cursor past 2 atoms** → remaining = [] (exhausted)
- Append "ab" to band: BandState = Single { band: [ab, ab], start_bound: 0, end_bound: 4 }

**3d. Cursor exhausted → commit**
- Collapse Single band → pattern = [ab, ab]
- RootManager: append_collapsed([ab, ab])
  - root was "ab", now combine: [ab, ab, ab] or recognize optimization
  - Actually: root = "ab", append = [ab, ab]
  - Since root has 1 child pattern, extend: [ab] → [ab, ab, ab]
  - **OR** create "abab" from [ab, ab], then root = [ab, abab]

**Wait** — this doesn't produce the expected result! The expected result is:
```
ab => [[a, b]]
abab => [[ab, ab]]
ababab => [[ab, abab], [abab, ab]]
```

The overlap `[abab, ab]` / `[ab, abab]` comes from recognizing that "ab" at the boundary overlaps. This requires:
1. After committing [ab, ab] = "abab" to the root
2. Detecting that the last "ab" in "abab" is the same as the first "ab" following it
3. Creating both decompositions

**This is where the algorithm breaks down.** The overlap must be detected between the band's last token and the next match. Currently the overlap detection via ExpandCtx looks at postfixes of the anchor, but the anchor and the next match are the **same token** ("ab"), so the overlap is at the token level, not the postfix level.

### Revised Understanding

The overlap is not between a postfix of the anchor and the remaining pattern. It's between:
- The **full anchor token** (last committed token) and the **first token of the next match**
- These two tokens share boundary atoms: anchor's suffix == next match's prefix

For "ababab":
- First band block: [ab, ab] → committed as "abab"  
- But we should also detect: "abab" overlaps with "ab" because "abab" ends with "ab" and the next block starts with "ab"
- This gives: "ababab" = [abab, ab] AND [ab, abab]

The overlap detection should happen at the **root level** when appending blocks, not just at the per-token postfix level.

---

## Open Questions

> These should be resolved before or during implementation.

1. **Q1:** Should `CursorCtx` use `PatternRangePath` or just `&[Token]` for the searchable interface?
2. **Q2:** How should cursor advancement work with `insert_or_get_complete`'s returned path?
3. **Q3:** Should overlap detection use `insert_or_get_complete` or ancestor search?
4. **Q4:** Precise definition of "overlap": is it the anchor's postfix matching remaining's prefix, or is it the anchor token itself matching the next committed token?
5. **Q5:** Is `RootManager::append_collapsed` overlap detection redundant with `BandState::WithOverlap`?
6. **Q6:** Should `commit_state` be simplified since overlap info is in BandState?
7. **Q7:** For the "ababab" case, should overlap be detected during expansion iteration, or during the final commit to root?
8. **Q8:** When we have `insert_or_get_complete` returning a token, how do we know if the cursor should advance by the **search match width** or by the **token width**? (They might differ if the search found a higher-level token.)
9. **Q9:** The `BandState::collapse` creates a bundled token with `insert_patterns` (two decompositions). After this bundled token is committed to root, will subsequent searches correctly find it? Does the graph invariant guarantee this?

---

## Execution Strategy

### Phase 1: Fix Cursor Advancement (WI-1 + WI-2)
Get the basic loop working: search → advance → search → advance. Target: `repetition_abab` and `repetition_abcabc` tests.

### Phase 2: Fix Overlap Detection (WI-3 + WI-4)
Get overlap detection working for the simplest case. Target: `read_repeating_known1` ("xyyxy").

### Phase 3: Fix Commit Flow (WI-5 + WI-6)
Ensure committed blocks are visible to subsequent iterations. Target: `read_sequence1`, `read_infix1`.

### Phase 4: Complex Overlaps (WI-7)
Multi-read scenarios with rotating overlaps. Target: `read_multiple_overlaps1`, `repetition_abcabcabc`.

---

## Related Documents

- [20260206_CONTEXT_READ_STATE_ANALYSIS.md](agents/analysis/20260206_CONTEXT_READ_STATE_ANALYSIS.md) - Previous analysis
- [20260210_BLOCK_ITERATION_ALGORITHM.md](agents/analysis/20260210_BLOCK_ITERATION_ALGORITHM.md) - "ababab" walkthrough
- [20260207_BLOCK_ITER_OVERLAP_EXPANSION.md](agents/guides/20260207_BLOCK_ITER_OVERLAP_EXPANSION.md) - Segment/block guide
- [20260211_BANDCHAIN_OVERLAP_LINKS_GUIDE.md](agents/guides/20260211_BANDCHAIN_OVERLAP_LINKS_GUIDE.md) - BandChain guide
- [20260205_CONTEXT_INSERT_EDGE_CASES.md](agents/analysis/20260205_CONTEXT_INSERT_EDGE_CASES.md) - Edge case analysis
- [context-insert HIGH_LEVEL_GUIDE.md](crates/context-insert/HIGH_LEVEL_GUIDE.md) - Insert architecture
- [context-search HIGH_LEVEL_GUIDE.md](crates/context-search/HIGH_LEVEL_GUIDE.md) - Search architecture
