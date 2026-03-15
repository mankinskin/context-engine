---
tags: `#interview` `#expansion-loop` `#overlap` `#decomposition`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
batch: 3
topic: Overlap Collection and Decomposition Output
status: ✅ answered
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

- **Answer:** An overlap can **never** be the same width as the postfix it expanded
  from — if `insert_next_match` does not expand the postfix (i.e. the result width
  equals the postfix width without producing a new larger token), that postfix must
  be skipped as "not expanded." The search continues to the next smaller postfix.
  Only a postfix for which `insert_next_match` returns a token whose width is
  **greater** than the postfix width (i.e. the postfix was genuinely expanded by
  the following atoms) represents a real overlap. Stop at the first such match
  (largest qualifying postfix).

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

- **Answer:** Both `prefix_complement` and `suffix_complement` can be **multi-token
  patterns** that require wrapping into a new token. They are not guaranteed to
  exist as single tokens. Resolution is via `insert_next_match` on the relevant
  atom sub-slice; if no single token covers the range, a new compound token must
  be created (wrapping the multi-token pattern).

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

- **Answer:** Neither is guaranteed to exist. Both may be multi-token patterns that
  must be **created on-the-fly** during overlap resolution. A recursive
  `insert_next_match` call on the relevant atom sub-slice is sufficient to find or
  create the tightest covering token for each complement range. Full recursive
  insert (not just lookup) may be triggered if no token covering that sub-range
  yet exists.

---

**Q14.** The `BandState::WithOverlap` variant currently carries:
```rust
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

- **Answer:** The existing `collapse()` logic should produce the correct two-pattern
  output provided the `primary` and `overlap` bands are correctly populated
  (including padding tokens at both ends). However, `collapse()` may benefit from
  a review and possible rewrite now that the one-yield-per-step model is in place
  and the padding responsibility is explicit. It should not be changed blindly —
  review first, rewrite only if the existing logic cannot accommodate the new band
  shape.

---

**Q15.** After yielding a `WithOverlap` state and calling `root.commit_state`, the
root contains the bundled overlap token. The cursor then advances past `T1`. The
next `insert_next_match` call starts at the position after `T1`'s start (since the
overlap consumed `T1.width - P.width` atoms before the overlap start). Is the
cursor advance `T1.width` (full token) or `T1.width - P.width` (only the
non-overlapping prefix of T1)? Describe the invariant: at every cursor position,
what does `atoms[cursor]` represent relative to the root token's current coverage?

- **Answer:** The cursor does **not** advance past `T1`. Instead it advances to the
  position of the **largest postfix of `T2`** — which may itself still overlap with
  `T1`. This means the cursor moves to `T2.start + (T2.width - largest_postfix_of_T2.width)`,
  keeping the overlap region in view for the next iteration.

  **Invariant:** at every cursor position, `atoms[cursor]` is the first atom that
  has not yet been covered by the root token's current committed coverage. The root
  always covers `atoms[0..cursor]` exactly — no gap, no double-cover. After
  committing a `WithOverlap` state, the cursor sits at the start of the next
  unresolved region, which may be inside `T2`'s range if `T2` itself has further
  overlap candidates.

---

## Research Notes

### R9 — Overlap condition is expansion, not width-equality (Q11)

The Q11 answer fundamentally corrects the overlap detection predicate stated in
the original plan. The original plan said:

> "the first postfix `P` for which `insert_next_match(next_remaining)` returns a
> token of the **same width** as `P` is the overlap"

This is **wrong**. The correct condition is:

> A postfix `P` produces an overlap iff `insert_next_match(next_remaining)` returns
> a token whose width is **greater than** `P.width` — i.e. the postfix was
> genuinely expanded by the following atoms, meaning the result token is a compound
> that includes `P` plus some continuation.

If the result width equals `P.width`, the query found the postfix token itself
without expansion — this is not an overlap, skip to the next smaller postfix.

This has an immediate impact on `find_overlap` in the proposed `ExpansionCtx`:
the match condition must be `result.width > postfix.width`, not `result.width == postfix.width`.

### R10 — Complement tokens require on-the-fly recursive insert (Q12, Q13)

Neither `prefix_complement` nor `suffix_complement` (nor `prefix_of_T1` /
`suffix_of_T2`) are guaranteed to exist. The implication is:

1. Overlap resolution is not a pure read operation — it may trigger graph writes
   (new compound tokens) for complement and padding ranges.
2. These writes must happen **before** `BandState::collapse()` is called, because
   `collapse()` calls `graph.insert_patterns` which needs the child tokens to
   already exist as valid graph indices.
3. The recursive `insert_next_match` calls for complements must therefore happen
   inside `find_overlap` (or in the band construction step), not inside `collapse`.
4. Since each complement creation is itself a graph write, and the transactional
   model (PI-5) requires the graph to be up-to-date before the next step, these
   complement creations must complete within the same `ExpansionCtx::next()` call
   that detects the overlap.

### R11 — Cursor advances into T2's overlap region, not past T1 (Q15)

The A15 answer reveals that the cursor advance rule is **more subtle than stated
in the original plan**. The original plan proposed:

> "advance cursor by `token.width`" (past T1)

The correct rule is: after an overlap between `T1` and `T2` is committed, the
cursor advances to `T2.start + (T2.width - largest_postfix_of_T2.width)` — i.e.
to the start of `T2`'s largest postfix. This keeps that postfix in the next
iteration's query window so it can be checked for further overlaps with the atoms
following `T2`.

This means the cursor advance is **not** a fixed `token.width` step. It depends
on the overlap structure detected. The loop must compute the cursor position
dynamically from the committed `BandState`'s coverage end, not from a simple
`cursor += token.width` formula.

### R12 — `collapse()` review is required before algorithm work begins

A14 recommends reviewing `collapse()` before proceeding. Given that:
- Band shapes have changed (one-yield-per-step, explicit padding)
- Complement tokens may be multi-token patterns now resolved externally
- The overlap condition predicate has changed (R9)

A collapse review is a **prerequisite** to the RC-2/RC-3 implementation, not an
optional cleanup. Schedule it as the first sub-task of the implementation phase.

---

## Plan Impact

### PI-8 — Fix overlap detection predicate in `find_overlap`

Replace `result.width == postfix.width` with `result.width > postfix.width` in
the `find_overlap` helper. The original plan's stated condition was incorrect.
Update the Proposed Architecture section of the main plan and add a code comment:

```rust
// Overlap condition: insert_next_match expanded the postfix (result > postfix width).
// If result.width == postfix.width the postfix was found verbatim — not an overlap.
// Skip and try the next smaller postfix.
```

### PI-9 — Complement creation happens inside `find_overlap`, before `collapse`

`find_overlap` must resolve complement tokens (via recursive `insert_next_match`)
as part of overlap detection, not deferred to `collapse()`. By the time
`BandState::collapse()` is called, all child tokens in both the `primary` and
`overlap` bands must already exist in the graph. Add this as a stated invariant
in the implementation notes.

### PI-10 — Cursor advance is dynamic, not `+= token.width`

The main plan's proposed `ExpansionCtx::next` pseudo-code uses `self.cursor +=
token.width` unconditionally. This must change: after an overlap, the cursor
advances to the start of `T2`'s largest postfix (inside `T2`'s range), not past
`T1`. The cursor update formula must be derived from the committed `BandState`'s
resolved coverage. Update the pseudo-code in the Proposed Architecture section.

### PI-11 — `collapse()` review is a prerequisite sub-task

Add to the implementation plan's task ordering:
1. Review and optionally rewrite `BandState::collapse()` — confirm it handles
   one-token-per-yield bands with explicit external padding resolution.
2. Only then proceed to `ExpansionCtx` cursor loop implementation.