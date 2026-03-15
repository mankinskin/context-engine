---
tags: `#interview` `#expansion-loop` `#cursor` `#NoExpansion`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
batch: 4
topic: Cursor Advancement and NoExpansion Handling
status: 🔴 blocked-by-batch-3
---

# Interview — Batch 4: Cursor Advancement and NoExpansion Handling

> These questions establish how `NoExpansion` is handled at each level and how the
> cursor stays in sync with the root.

---

**Q16.** `insert_next_match` returns `NoExpansion { token, width }` meaning: the
search found token `T` at the start of the query, but the query extends beyond
`T.width`. No new compound token was created. The cursor should advance by
`T.width` regardless of whether an overlap is later found. Is this always safe?
Consider the case where `T` is a compound token of width 3, the remaining query
has width 5, and a postfix of `T` of width 2 overlaps the next 2 atoms. Should
the cursor advance by 3 (full `T.width`) or by 1 (the non-overlapping prefix of
`T`)?

- **Answer:**

---

**Q17.** In the single-level loop (RC-1 fix in `insert_sequence`), `NoExpansion`
means "advance by `token.width` and call again." In the expansion loop (RC-2/RC-3
fix in `ExpansionCtx`), `NoExpansion` triggers the postfix descent. Are these two
uses of `NoExpansion` consistent, or do they require different handling? Should
the outer `insert_sequence` loop also descend into postfixes, or is postfix descent
only needed inside `ExpansionCtx`?

- **Answer:**

---

**Q18.** When `insert_next_match` returns `Complete { token }` (the entire
remaining query is consumed by an existing token), the cursor advances by
`token.width` and the block is done. But `token.width` equals `remaining.len()`
in this case. Is it possible for `Complete` to be returned with `token.width <
remaining.len()`? If so, what does that mean for the cursor?

- **Answer:**

---

**Q19.** After a `Created { token }` outcome — a new compound token was just
inserted via the split+join pipeline — the cursor advances by `token.width`. The
newly created token now exists in the graph. On the next `insert_next_match` call,
could the newly created token itself become the first result, effectively
re-entering a `Complete` or `NoExpansion` path with the same token? Is idempotence
guaranteed here?

- **Answer:**

---

**Q20.** The `ExpandCtx::try_new` guard — which returns `None` if
`anchor.postfix_iter()` yields nothing — currently short-circuits the entire
expansion for fresh atoms. In the proposed design, an anchor with no postfixes
(i.e. a fresh atom or a newly-created compound with no parents yet) means "no
overlap possible, advance normally." Should `find_overlap` be a no-op (return
`None`) when `anchor.postfix_iter()` is empty, or should there be a separate
fast-path that skips the postfix iterator entirely for performance?

- **Answer:**

---

## Research Notes
<!-- Filled in after answers received -->

## Plan Impact
<!-- Changes to main plan driven by this batch -->