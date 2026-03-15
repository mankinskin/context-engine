---
tags: `#interview` `#expansion-loop` `#RootManager` `#commit`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
batch: 5
topic: RootManager and Commit Contract
status: ✅ answered
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

- **Answer:** The overlap logic in `append_collapsed` is **invalid** and should be
  removed entirely. Overlaps must be found by expanding postfixes of the current
  root inside `ReadCtx` / `ExpansionCtx`. Keeping a fallback overlap path in
  `append_collapsed` would mask bugs and create ambiguous double-detection. Once
  `ExpansionCtx` is correct, `append_collapsed` becomes a pure structural append
  with no overlap awareness.

---

**Q22.** `BandState::collapse()` calls `graph.insert_patterns(vec![primary_pat,
overlap_pat])` to write both decompositions into the graph as a new bundled token.
After `commit_state` calls `collapse`, the returned `Pattern` (a single-element
pattern containing the bundled token) is passed to `append_collapsed`. If the
bundled token's pattern is `[bundled]` (one element), which branch of
`append_collapsed` is entered: `append_token` or the multi-token path? Is this
always correct?

- **Answer:** The synchronisation between `BandState` creation and commitment to
  `RootManager` requires careful design. The options depend on the current root
  structure:
  - If the root is empty: the bundled overlap token becomes the root directly.
  - If the root is non-empty: either a new root is created containing the previous
    root and the overlap as children, **or** the last token in the root's pattern
    is replaced by a new wrapper that includes the overlap.
  This logic is non-trivial and must be designed precisely to handle all structural
  cases correctly. **A dedicated session file is needed** to work through the exact
  root update steps for every case that arises during reading.
  See: [`20260315_DESIGN_ROOT_UPDATE_DURING_READ.md`](../designs/20260315_DESIGN_ROOT_UPDATE_DURING_READ.md)

---

**Q23.** `RootManager::last_child_token` returns the rightmost child of the root's
first pattern. After committing a `WithOverlap` state that produces a bundled
overlap token, does `last_child_token` correctly return the last atom of the
overlap region (i.e. the end of the bundled token)? This value is used as the
anchor for the next `ExpandCtx` iteration.

- **Answer:** `last_child_token` should **not** return the rightmost atom of the
  bundled token. Instead, the anchor for the next iteration is the **exact overlap
  token of the `WithOverlap` state** — i.e. the overlap token `T2` itself (the
  largest leftmost match that was found). This overlap token is the shortest path
  to finding the next largest leftmost match, because it will have the shortest
  path to its own postfix expansions. The algorithm structure mirrors the start:
  first find the largest match at the beginning of the query, then explore postfixes
  of that match to find the next largest overlapping match — and that next match
  becomes the new anchor. The anchor must therefore be `T2` (the expansion result),
  not the rightmost atom of the bundled token.

---

**Q24.** `append_to_owned_pattern` modifies a pattern in-place and is used when
the root has exactly one child pattern and no parents (i.e. it is not shared with
anyone). After the expansion loop runs and the root has been extended, does the
root still satisfy the `can_extend` conditions (`child_patterns().len() == 1 &&
parents().is_empty()`)? Or does building overlap decompositions during the loop
add parents to intermediate tokens and invalidate the extend path?

- **Answer:** `can_extend` **can be invalidated** after each commit. Specifically,
  when a commit adds a second pattern to the root (e.g. an overlap decomposition
  that introduces an additional `child_pattern`), the condition
  `child_patterns().len() == 1` no longer holds and the in-place extend path is
  unavailable for subsequent commits. The implementation must check `can_extend`
  freshly at each step — it cannot be assumed to remain stable across multiple
  commits in the same expansion loop.

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

- **Answer:** To be determined — this must be validated as part of the dedicated
  root update design session. The `None` branch in `append_collapsed` may be
  correct as-is for the simple `BandState::Single` case, but whether it correctly
  initialises the root when the first committed state is a `WithOverlap` (bundled
  token) is not yet confirmed.
  See: [`20260315_DESIGN_ROOT_UPDATE_DURING_READ.md`](../designs/20260315_DESIGN_ROOT_UPDATE_DURING_READ.md)

---

## Research Notes

### R17 — `append_collapsed` overlap logic must be deleted (Q21)

The A21 answer is unambiguous: the overlap detection in `append_collapsed` is
architecturally invalid in the redesigned system. It must be deleted — not
disabled, not guarded, not preserved as fallback. After deletion, `append_collapsed`
handles only structural append cases:

1. `root == None` → create root from pattern.
2. `root` is non-empty, `can_extend` → extend in-place.
3. `root` is non-empty, `!can_extend` → create new compound root wrapping old root
   and new pattern.

No overlap logic appears in any of these branches. If an overlap is incorrectly
passed through (i.e. `ExpansionCtx` failed to detect it), it will be appended as
a sequential token — a correctness bug that is detectable by tests, not silently
papered over.

### R18 — Anchor is the overlap token T2, not the bundled root's last atom (Q23)

This is a significant clarification of the iteration model. The anchor chain is:

```
Step 0: anchor = first insert_next_match result (largest match at query start)
Step 1: explore postfixes of anchor → find T2 (largest overlapping match)
        → commit WithOverlap(anchor, T2)
        → new anchor = T2
Step 2: explore postfixes of T2 → find T3 (next overlapping match)
        → commit WithOverlap(T2, T3)
        → new anchor = T3
...
```

The anchor is always the **last overlap token found** (`T2`, `T3`, …), not the
rightmost atom of the committed bundled token. This mirrors the algorithm's
starting condition: the first anchor is the largest match at position 0, which is
exactly how the loop begins. Each subsequent anchor is the expansion result of
the previous overlap check.

**Implication for `ExpansionCtx`:** the `anchor` field must be updated to the
overlap token (`T2`) after a `WithOverlap` commit, not to the bundled token. The
current plan pseudo-code sets `self.anchor = Some(token)` where `token` is `T1`
— this must change to `self.anchor = Some(expansion)` where `expansion` is `T2`.

### R19 — `can_extend` is volatile; must be checked per-commit (Q24)

Because each overlap commit may add a second pattern to the root (making `can_extend`
false), the root extension path must be re-evaluated at every `commit_state` call.
This is already the natural behaviour of `can_extend` as a computed predicate, but
the implementation must not cache the result across loop iterations.

### R20 — Root update design session required (Q22, Q25)

Both Q22 and Q25 point to the same gap: the exact sequence of root mutations
during reading — covering all combinations of root state × incoming `BandState`
variant — has not been fully specified. This must be worked out before implementation
to avoid iterating on broken code. A dedicated design document is the correct
vehicle.

---

## Plan Impact

### PI-16 — Delete `append_collapsed` overlap logic

Remove the two overlap branches from `RootManager::append_collapsed`. Add a
`// REMOVED: overlap detection — handled entirely by ExpansionCtx` comment at
the deletion site to explain the intentional absence. Add a test that verifies
`append_collapsed` does not modify the root when called with a pattern that would
previously have triggered the overlap branch.

### PI-17 — Fix anchor update in `ExpansionCtx::next`

In the `WithOverlap` arm of `ExpansionCtx::next`, change:

```rust
self.anchor = Some(token);   // WRONG: token is T1
```

to:

```rust
self.anchor = Some(expansion);  // CORRECT: expansion is T2, the next overlap anchor
```

Update the `ExpansionCtx` pseudo-code in the Proposed Architecture section of
the main plan accordingly.

### PI-18 — `can_extend` must be checked per-commit, never cached

Add an explicit note in the `RootManager` implementation: `can_extend` is a
live predicate that must be evaluated at each `commit_state` call. Document that
overlap commits invalidate `can_extend` by adding a second pattern.

### PI-19 — Create root update design session (prerequisite)

Create `agents/designs/20260315_DESIGN_ROOT_UPDATE_DURING_READ.md` before
proceeding to RC-2/RC-3 implementation. This document must specify the exact
root mutation for every combination of:
- Root state: `None` / single-token / multi-token / `can_extend` / `!can_extend`
- Incoming `BandState`: `Single` / `WithOverlap`

This is a **hard prerequisite** — implementation must not proceed without it.