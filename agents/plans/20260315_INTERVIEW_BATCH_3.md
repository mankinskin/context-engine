---
tags: `#interview` `#expansion-loop` `#overlap` `#decomposition`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
batch: 3
topic: Overlap Collection and Decomposition Output
status: 🔴 blocked-by-batch-2
---

# Interview — Batch 3: Overlap Collection and Decomposition Output

> These questions establish how overlaps are detected, what "tight packing" means
> precisely, and what patterns are written into the graph.

---

**Q11.** When a `NoExpansion { token, width }` outcome is returned and we descend
postfixes of `token` to find overlap: the postfix iterator yields postfixes
largest-first. The first postfix `P` for which `insert_next_match(next_remaining)`
returns a token of the same width as `P` is the overlap. Is it correct to stop at
the first (largest) matching postfix, or should smaller postfixes also be checked?
What does the ngrams reference algorithm do?

- **Answer:**

---

**Q12.** When an overlap between `token T1` and the next segment `T2` is found via
postfix `P`:
- `T1` decomposes as `[prefix_complement, P]` where `prefix_complement` covers
  `T1.width - P.width` atoms.
- `T2` decomposes as `[P, suffix_complement]` where `suffix_complement` covers
  `T2.width - P.width` atoms.
- The bundled overlap token covers `T1.width + T2.width - P.width` atoms.

Is `prefix_complement` always a single existing token, or can it be a pattern of
multiple tokens that must be wrapped? Similarly for `suffix_complement`.

- **Answer:**

---

**Q13.** "Tight packing" of decompositions means that in each pattern on a bundled
token, every atom position is covered by exactly one child token — no gaps and no
double-cover. Given `T1` (width 3) overlapping `T2` (width 3) via postfix `P`
(width 1), the two decompositions of the bundled token (width 5) should be:
- `[T1, suffix_of_T2]` where `suffix_of_T2` has width 2
- `[prefix_of_T1, T2]` where `prefix_of_T1` has width 2

Are `prefix_of_T1` and `suffix_of_T2` guaranteed to already exist as tokens in the
graph at the time this overlap is being processed? Or must they be created
on-the-fly? If created on-the-fly, is a recursive insert needed?

- **Answer:**

---

**Q14.** The `BandState::WithOverlap` variant currently carries:
```context-engine/agents/plans/20260315_INTERVIEW_BATCH_3.md#L1-1
WithOverlap {
    primary: Band,   // sequential decomposition tokens
    overlap: Band,   // [complement, expansion] decomposition
    link:    OverlapLink,
}
```
In the proposed design where `ExpansionCtx` yields one `BandState` per cursor
step, a `WithOverlap` state covers the atoms from `overlap_start` to
`overlap_end`. Does `BandState::collapse()` need to change, or does the existing
collapse logic produce the correct two-pattern output as long as the `primary` and
`overlap` bands are correctly populated?

- **Answer:**

---

**Q15.** After yielding a `WithOverlap` state and calling `root.commit_state`, the
root contains the bundled overlap token. The cursor then advances past `T1`. The
next `insert_next_match` call starts at the position after `T1`'s start (since the
overlap consumed `T1.width - P.width` atoms before the overlap start). Is the
cursor advance `T1.width` (full token) or `T1.width - P.width` (only the
non-overlapping prefix of T1)? Describe the invariant: at every cursor position,
what does `atoms[cursor]` represent relative to the root token's current coverage?

- **Answer:**

---

## Research Notes
<!-- Filled in after answers received -->

## Plan Impact
<!-- Changes to main plan driven by this batch -->