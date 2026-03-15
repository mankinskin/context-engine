---
tags: `#interview` `#expansion-loop` `#RootManager` `#commit`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
batch: 5
topic: RootManager and Commit Contract
status: 🔴 blocked-by-batch-4
---

# Interview — Batch 5: RootManager and Commit Contract

> These questions establish how `RootManager` must change (if at all) to support
> the new iteration model.

---

**Q21.** `RootManager::append_collapsed` currently contains logic for two overlap
cases: "cursor-level overlap" (root is atomic, equals first token of append) and
"compound overlap" (root's last child equals first child of append's first token).
In the proposed design, overlaps are detected and committed as `WithOverlap`
states by `ExpansionCtx`, not by `append_collapsed`. Should `append_collapsed`
lose its overlap logic entirely, or should it remain as a fallback for cases
that `ExpansionCtx` does not yet handle?

- **Answer:**

---

**Q22.** `BandState::collapse()` calls `graph.insert_patterns(vec![primary_pat,
overlap_pat])` to write both decompositions into the graph as a new bundled token.
After `commit_state` calls `collapse`, the returned `Pattern` (a single-element
pattern containing the bundled token) is passed to `append_collapsed`. If the
bundled token's pattern is `[bundled]` (one element), which branch of
`append_collapsed` is entered: `append_token` or the multi-token path? Is this
always correct?

- **Answer:**

---

**Q23.** `RootManager::last_child_token` returns the rightmost child of the root's
first pattern. After committing a `WithOverlap` state that produces a bundled
overlap token, does `last_child_token` correctly return the last atom of the
overlap region (i.e. the end of the bundled token)? This value is used as the
anchor for the next `ExpandCtx` iteration.

- **Answer:**

---

**Q24.** `append_to_owned_pattern` modifies a pattern in-place and is used when
the root has exactly one child pattern and no parents (i.e. it is not shared with
anyone). After the expansion loop runs and the root has been extended, does the
root still satisfy the `can_extend` conditions (`child_patterns().len() == 1 &&
parents().is_empty()`)? Or does building overlap decompositions during the loop
add parents to intermediate tokens and invalidate the extend path?

- **Answer:**

---

**Q25.** When `ReadCtx` processes a string that is read for the second time (all
atoms are `Known`, no unknown segment), `SegmentIter` yields a single `NextSegment
{ unknown: [], known: [all_atoms] }`. The `append_pattern([])` call is a no-op.
The entire input goes into `BlockExpansionCtx`. The root starts as `None`. At the
first `insert_next_match` call with a `Complete` result, the expansion loop yields
`BandState::new(complete_token)` and `commit_state` sets root to `complete_token`.
Is the `root=None` initial state handled correctly in `commit_state` /
`append_collapsed`? (Currently `append_collapsed` has an explicit `None` branch
that creates a fresh root from the first pattern.)

- **Answer:**

---

## Research Notes
<!-- Filled in after answers received -->

## Plan Impact
<!-- Changes to main plan driven by this batch -->