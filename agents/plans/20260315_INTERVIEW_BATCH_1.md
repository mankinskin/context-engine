---
tags: `#interview` `#expansion-loop` `#classification`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
batch: 1
topic: Classification Boundary
status: 🟡 awaiting-answers
---

# Interview — Batch 1: Classification Boundary

> These questions establish whether the new/known boundary is correctly located
> and what invariants must hold at the boundary for the rest of the algorithm to
> be correct.

---

**Q1.** The current design tags each atom as `New` or `Known` at resolution time
(the moment it is first seen in the input stream). A `New` atom is inserted into
the graph immediately and classified as new for the purposes of this read
operation. A `Known` atom was already in the graph before the read began.

Is this classification stable? That is: if the same character appears three times
in the input — e.g. `"aXa...a"` — is it guaranteed that the second and third
occurrences are always `Known` (never `New`)?

- **Answer:**

---

**Q2.** The classification controls whether `append_pattern` (no overlap search)
or `BlockExpansionCtx` (overlap search) is used for a run of atoms. The boundary
between an unknown-run and a known-run produces a `NextSegment`. Is it possible
for a single input string to produce more than two segments (i.e. `unknown →
known → unknown → known → ...`)? If yes, give an example and describe what the
correct processing order is.

- **Answer:**

---

**Q3.** A `New` atom is appended directly to the root without any overlap search.
This means if the input is `"Xab"` where `X` is new but `ab` is a known token,
the unknown run `[X]` is appended first, giving root=`X`. Then `[a, b]` enters
the expansion loop. The expansion loop would find `ab` and produce root=`[X, ab]`.
Is this the correct final structure, or should the root carry both
`[[X, a, b], [X, ab]]` as decompositions?

- **Answer:**

---

**Q4.** What happens at the boundary when the last token of the unknown run and
the first token of the known run together form an existing compound token? For
example: atom `X` is new, atom `a` is known, and `Xa` exists as a compound token.
The current code would produce root=`[X, ...]` then start the expansion loop at
`a`. Should the boundary itself be checked for cross-segment overlap?

- **Answer:**

---

**Q5.** `LazyAtomIter` resolves each character at consumption time, meaning the
`Known`/`New` status is computed during the SegmentIter traversal, not upfront.
If `SegmentIter` is consumed lazily (one segment at a time), can the `New`/`Known`
status of an atom depend on a prior segment having been processed? Specifically:
if `"ab"` is the input and `a` is created as `New` during segment 1, is `a` then
`Known` if it appears again in segment 2 (e.g. the full input is `"a...a"` split
across segment boundaries)?

- **Answer:**

---

## Research Notes
<!-- Filled in after answers received -->

## Plan Impact
<!-- Changes to main plan driven by this batch -->