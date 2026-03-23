---
tags: `#interview` `#expansion-loop` `#classification`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
batch: 1
topic: Classification Boundary
status: ✅ answered
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

- **Answer:** Yes. Once an atom is created (first occurrence → `New`), all
  subsequent occurrences in the same read are `Known`. The classification is
  stable within a read operation.

---

**Q2.** The classification controls whether `append_pattern` (no overlap search)
or `BlockExpansionCtx` (overlap search) is used for a run of atoms. The boundary
between an unknown-run and a known-run produces a `NextSegment`. Is it possible
for a single input string to produce more than two segments (i.e. `unknown →
known → unknown → known → ...`)? If yes, give an example and describe what the
correct processing order is.

- **Answer:** Yes. Example: `"abac"` where `b` and `c` are new characters but `a`
  is already known. This produces segments:
  `known[a] → unknown[b] → known[a] → unknown[c]`.
  Correct processing order is strictly left-to-right: process each segment in
  sequence, committing the root after each one before starting the next.

---

**Q3.** A `New` atom is appended directly to the root without any overlap search.
This means if the input is `"Xab"` where `X` is new but `ab` is a known token,
the unknown run `[X]` is appended first, giving root=`X`. Then `[a, b]` enters
the expansion loop. The expansion loop would find `ab` and produce root=`[X, ab]`.
Is this the correct final structure, or should the root carry both
`[[X, a, b], [X, ab]]` as decompositions?

- **Answer:** `[X, ab]` is the correct and complete final structure. The
  simplistic all-atoms decomposition `[X, a, b]` must **not** be included.
  Rationale: the graph only needs downward reachability — `ab` already provides a
  path to `a` and `b` in their respective positions. Adding `[X, a, b]` as a
  second decomposition would be redundant and inconsistent with the tight-packing
  invariant.

---

**Q4.** What happens at the boundary when the last token of the unknown run and
the first token of the known run together form an existing compound token? For
example: atom `X` is new, atom `a` is known, and `Xa` exists as a compound token.
The current code would produce root=`[X, ...]` then start the expansion loop at
`a`. Should the boundary itself be checked for cross-segment overlap?

- **Answer:** This case is **impossible by construction**. Because `X` is a `New`
  atom — it did not exist in the graph before this read began — no compound token
  containing `X` can exist yet. Therefore `Xa` cannot be a pre-existing token, and
  no cross-segment overlap check is needed at the unknown→known boundary. The
  boundary is safe to treat as a hard cut.

---

**Q5.** `LazyAtomIter` resolves each character at consumption time, meaning the
`Known`/`New` status is computed during the SegmentIter traversal, not upfront.
If `SegmentIter` is consumed lazily (one segment at a time), can the `New`/`Known`
status of an atom depend on a prior segment having been processed? Specifically:
if `"ab"` is the input and `a` is created as `New` during segment 1, is `a` then
`Known` if it appears again in segment 2 (e.g. the full input is `"a...a"` split
across segment boundaries)?

- **Answer:** Yes — if `a` was created as `New` in an earlier segment, it is
  `Known` in all subsequent segments of the same read. This is consistent with Q1.
  Additionally: it is questionable whether `SegmentIter` is really necessary in
  the lazy path. The `Known`/`New` classification already prevents unnecessary
  postfix iterations at lazy segment boundaries, so the segmentation structure may
  be redundant overhead in the lazy case.

---

## Research Notes

### R1 — Cross-segment overlap is a non-issue (Q4)

The Q4 impossibility proof is a **hard invariant** the implementation can rely on
without adding a guard:

> A `New` atom, by definition, was not in the graph before this read. Therefore it
> cannot appear in any pre-existing compound token. Therefore no compound token
> crossing the unknown→known boundary can exist. Therefore no overlap search is
> required at the boundary.

This means `append_pattern` for the unknown run followed by a clean `ExpansionCtx`
start for the known run is **always correct** — no look-behind into the unknown
run is ever needed from inside `ExpansionCtx`.

### R2 — Multi-segment inputs are the norm (Q2)

`"abac"` is a minimal example. In real corpora, alternating known/unknown runs
are expected for any text that mixes high-frequency (known) and rare/new
characters. The segment loop in `ReadCtx::read_sequence` must handle an arbitrary
number of `NextSegment` values, not just one or two. This is already the design
intent but must be confirmed in the implementation — the `while let Some(segment)`
loop must not have an early-exit that fires after the first known segment.

### R3 — `SegmentIter` redundancy in the lazy path (Q5)

The A5 answer raises a design question: if `LazyAtomIter` can already classify
each atom as it is consumed, the segment-boundary abstraction (`SegmentIter`) may
be adding complexity without adding correctness in the lazy path. A possible
simplification: in the lazy path, process atoms one-at-a-time inside a single
loop that calls `append_pattern` for `New` atoms and `ExpansionCtx` step for
`Known` atoms, without grouping into segment pairs first.

This is **not a blocker** for the current redesign — `SegmentIter` is preserved
as-is. But it is a candidate for a follow-up simplification once the algorithm is
correct.

### R4 — Tight-packing confirmed (Q3)

The rejection of the `[X, a, b]` decomposition in A3 confirms the tight-packing
rule: **only the tightest (most-compressed) decomposition is stored**. The
algorithm must never emit a pattern that can be derived from another already-stored
pattern by expanding one child. This has a direct implication for Batch 3 (Q12,
Q13): complement tokens in overlap decompositions must also be the tightest
available tokens for their atom ranges, not raw atom sequences.

---

## Plan Impact

### PI-1 — Remove any cross-boundary overlap check guard

The main plan's proposed architecture does not include a cross-segment overlap
check, which is correct per R1. No change needed — but add a comment in the
implementation at the unknown→known boundary transition:

```rust
// Safety: no cross-segment overlap is possible here.
// A New atom cannot be part of any pre-existing compound token,
// so the unknown→known boundary is always a clean cut.
```

### PI-2 — Confirm unbounded segment loop in `read_sequence`

The `ReadCtx::read_sequence` pseudo-code in the plan's Proposed Architecture
section shows a single `for each NextSegment` loop. Verify the implementation
iterates all segments, not just the first. Flag as a required assertion in the
test plan: input `"abac"` (known/unknown/known/unknown) must produce the correct
4-atom root.

### PI-3 — Note `SegmentIter` simplification as a future follow-up

Add to the plan's **Notes** section: in the lazy path, `SegmentIter` may be
replaceable with a per-atom dispatch loop. Defer to a post-fix-round cleanup
plan; do not attempt during this redesign.

### PI-4 — Tight-packing rule drives complement token resolution (feeds Batch 3)

Q12/Q13 in Batch 3 ask whether complement tokens must be created on-the-fly. A3
confirms that tight packing requires using the most-compressed existing token for
each range. This means complement tokens (`prefix_complement`, `suffix_complement`)
must be looked up via `insert_next_match` (which finds the best existing match),
not constructed from raw atom slices. Add this as a constraint in the Batch 3
research phase.
```

Now let me update the main plan file's Notes section and the Batch 2 status: