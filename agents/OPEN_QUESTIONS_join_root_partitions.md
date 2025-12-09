# Open Questions: join_root_partitions Implementation

**Date:** 2025-12-09  
**Related Investigation:** `agents/SUMMARY_insert_postfix1_investigation.md`  
**Related Plan:** `agents/plans/PLAN_fix_insert_postfix1_wrapper.md`  
**Status:** Questions for Implementation

## Context

The `join_root_partitions` method needs to be modified to create wrapper vertices at pattern-entry level instead of atom level. Based on our investigation, we understand the general approach but have several open questions about the implementation details.

## Core Algorithm Questions

### 1. Wrapper Range Extraction

**Q1.1:** How exactly do we extract the wrapper range from the `delta` field?
- The `delta` field is `PatternSubDeltas = HashMap<PatternId, usize>`
- Does each pattern in the root have its own `delta` entry?
- How do we determine which pattern_id to use when there are multiple patterns?

**Q1.2:** For Postfix mode, the wrapper range should be "from delta entry to end of pattern"
- Do we use `delta` from the postfix partition or the prefix partition?
- What if both partitions have `delta` information?

**Q1.3:** For Prefix mode, the wrapper range should be "from start of pattern to delta entry"
- Similar question: which partition's `delta` do we use?
- Is the delta value the inclusive or exclusive end of the range?

### 2. Perfect Split Detection

**Q2.1:** How do we detect if a split is "perfect"?
- Is this indicated by the `perfect` field in `JoinedPartition`?
- Does `perfect.is_some()` mean we have a perfect split?
- What information does the `perfect` field contain?

**Q2.2:** How do we determine if two perfect splits are in "different patterns"?
- Do we check the `pattern_id` from `delta` for each partition?
- What data structure tells us which pattern each split belongs to?

**Q2.3:** When we have a single perfect split, how do we "replace partitions in the existing pattern"?
- Is there an existing method that handles pattern replacement?
- Do we modify the pattern in-place or create a new pattern?

### 3. Split Partition Extraction

**Q3.1:** How do we "extract the splits from all entry/exit children"?
- What methods exist to get split partitions for a child token?
- Are the split partitions already stored somewhere, or do we need to compute them?

**Q3.2:** What is the structure of split partitions?
- Is it just two tokens (prefix and postfix parts)?
- How do we access the partition boundaries?

**Q3.3:** When building patterns from split partitions, what order should they be in?
- For the `ababcd` example, we create `[ab, cd]` and `[a, bcd]`
- How do we determine which pattern to create first?
- Does the order matter?

### 4. Wrapper Creation Details

**Q4.1:** When creating two wrappers for perfect splits in different patterns:
- How do we find "the last vertex that is intersected by the end split"?
- How do we find "the index that is intersected by the start split"?
- Are there existing methods for finding intersected vertices?

**Q4.2:** How do we create the "overlap vertex of these two wrappers"?
- Is there an existing method like `find_or_create_overlap(wrapper1, wrapper2)`?
- Does this involve checking if a pattern already exists?
- Should we use `insert_pattern` or a different method?

**Q4.3:** For the wrapper patterns themselves:
- Do we always create exactly two patterns per wrapper?
- In the `ababcd` example: `[ab, cd]` and `[a, bcd]` - why two patterns?
- Is one pattern using "full entry tokens" and the other using "complement tokens"?

### 5. Root Pattern Replacement

**Q5.1:** How do we replace the wrapper range in the root pattern?
- Is there a method like `replace_pattern_range(start, end, new_token)`?
- Do we need to create a new pattern entirely, or modify the existing one?

**Q5.2:** When replacing, do we need to update other data structures?
- Do we need to update any indices or references?
- Are there any caching structures that need invalidation?

**Q5.3:** What happens to the original entries that are replaced?
- Do they remain in the graph or get garbage collected?
- Should we keep them for other patterns that might reference them?

## Implementation Strategy Questions

### 6. Existing Functions Research

**Q6.1:** What existing functions handle pattern-entry operations?
- The maintainer mentioned "there are existing functions for most of what you want to do"
- Which modules should we look in? (`pattern_info.rs`, `join/`, `split/`?)
- Are there helper methods in `NodeJoinCtx` or other contexts?

**Q6.2:** What methods exist for:
- Getting child tokens at specific entry indices?
- Finding patterns that already exist?
- Creating new patterns from token sequences?
- Replacing ranges in existing patterns?

### 7. Testing and Validation

**Q7.1:** For the `ababcd` test case specifically:
- How do we verify that `abcd` is created with the correct patterns?
- Should we check both `[ab, cd]` and `[a, bcd]` patterns exist?
- How do we verify it's found as EntireRoot, not just Postfix?

**Q7.2:** What edge cases should we test?
- Single-entry patterns?
- Patterns where all splits are perfect?
- Patterns where no splits are perfect?
- Empty partitions?

### 8. Mode-Specific Questions

**Q8.1:** Prefix Mode:
- The implementation was attempted but failed for `insert_prefix1`
- What is fundamentally different about the prefix case?
- Should the wrapper range calculation be symmetric to postfix?

**Q8.2:** Infix Mode:
- How do we handle the case with two split points (start and end)?
- Do we need to join prefix, infix, and postfix partitions?
- Is the infix partition always in the middle?

**Q8.3:** Do all three modes need the same two-wrapper approach for non-perfect splits?
- Or is this specific to certain configurations?
- Can we unify the logic across modes?

## Data Structure Questions

### 9. JoinedPartition Structure

**Q9.1:** What fields are available in `JoinedPartition<Pre>` and `JoinedPartition<Post>`?
- We know about `delta` and `perfect`
- What other fields might be useful?
- Is there an `index` field for the partition token?

**Q9.2:** How do we access the pattern from a `JoinedPartition`?
- Is there a method like `partition.get_pattern()`?
- Do we need to go through a context object?

### 10. Pattern and Token Access

**Q10.1:** Given a token, how do we:
- Get all its patterns?
- Get a specific pattern by ID?
- Get the children tokens in a pattern?
- Get the width (number of atoms) of a token?

**Q10.2:** How do we navigate the parent-child relationships?
- Given a root token and an entry index, how do we get the child token?
- How do we determine which entries contain a specific atom position?

## Performance and Correctness Questions

### 11. Algorithm Complexity

**Q11.1:** What is the expected complexity of the wrapper creation?
- Should it be O(n) where n is the pattern length?
- Are there any operations that might be expensive?

**Q11.2:** Should we cache any intermediate results?
- Split partition information?
- Pattern lookups?
- Entry index mappings?

### 12. Correctness Verification

**Q12.1:** How do we ensure we don't create duplicate vertices?
- Should we always check if a pattern exists before creating?
- Is there a risk of creating the same wrapper multiple times?

**Q12.2:** How do we ensure the surrounding context is truly unchanged?
- Should we add assertions to verify this?
- How do we check that entries outside the wrapper range are unmodified?

## Next Steps

To answer these questions, we should:

1. **Read the source code** for:
   - `JoinedPartition` struct definition
   - `NodeJoinCtx` methods
   - Existing pattern manipulation functions
   - The current `join_root_partitions` implementation

2. **Trace through the failing test** (`insert_postfix1`) to understand:
   - What data is available at each step
   - What the actual vs expected behavior is
   - Where exactly the algorithm diverges

3. **Look for similar code** that already does pattern-entry-level operations
   - Other join or split methods
   - Pattern replacement logic elsewhere in the codebase

4. **Clarify with maintainer** the highest priority questions that block implementation

## Priority Questions

If we can only get answers to a few questions, these are the most critical:

1. **Q1.1, Q1.2, Q1.3**: How to extract wrapper range from delta (blocking for all modes)
2. **Q2.1, Q2.2**: How to detect and handle perfect splits (blocking for correctness)
3. **Q3.1**: How to get split partitions from children (blocking for wrapper creation)
4. **Q6.1, Q6.2**: What existing functions to use (blocking to avoid reinventing the wheel)
5. **Q9.1, Q10.1**: What data is available in the structures (blocking for implementation)
