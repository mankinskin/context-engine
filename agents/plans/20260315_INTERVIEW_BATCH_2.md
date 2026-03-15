---
tags: `#interview` `#expansion-loop` `#ExpansionCtx`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
batch: 2
topic: ExpansionCtx Loop Contract
status: 🟡 awaiting-answers
---

# Interview — Batch 2: ExpansionCtx Loop Contract

> These questions establish the contract of the redesigned `ExpansionCtx` — what
> it owns, what it yields, and when it terminates.

---

**Q6.** In the proposed design, `ExpansionCtx` owns the `atoms: Pattern` slice for
the current known-atom block and a `cursor: usize`. Should `ExpansionCtx` own the
`RootManager`, or should it yield `BandState` values and let `BlockExpansionCtx`
(or `ReadCtx` directly) call `root.commit_state(state)` after each yield? What
are the trade-offs of each ownership model in terms of borrow conflicts and
testability?

- **Answer:**

---

**Q7.** The proposed loop calls `insert_next_match(remaining)` at each cursor
position. `insert_next_match` takes a `Vec<Token>` (or anything `Searchable`).
Should the remaining slice be passed as a reference or moved? If the graph holds
interior mutability (`HypergraphRef = Arc<RwLock<...>>`), does passing a slice
reference conflict with the write lock that `insert_next_match` may need when
creating a new compound token?

- **Answer:**

---

**Q8.** `insert_next_match` returns `Err(ErrorReason::SingleIndex(iwp))` when the
query has exactly one element. In the proposed loop, when `remaining.len() == 1`,
what should happen? Options:
  - (a) Append the single atom directly as a `BandState::new(atom)` without
        calling `insert_next_match`.
  - (b) Call `insert_next_match` anyway and handle `SingleIndex` as a special case.
  - (c) Check if the single atom has parents in the graph; if yes, run
        the postfix overlap check against it; if no, append directly.

- **Answer:**

---

**Q9.** After `ExpansionCtx` yields all `BandState` values and the cursor reaches
the end of `atoms`, the iterator returns `None`. At that point, `RootManager.root`
should represent the full known-atom block appended to whatever was in root before
the block started. Is there any state in `ExpansionCtx` that must be flushed on
exhaustion (e.g. a partially-built `BandState` that was not yet yielded)?

- **Answer:**

---

**Q10.** The current `ExpansionCtx` stores a `BandState` that accumulates tokens
across multiple `Cap` operations (appending to the single band without committing).
In the proposed design, each cursor step yields one `BandState` immediately. Does
removing the accumulation change the semantics? Specifically: is a `BandState`
consisting of multiple tokens ever needed, or is one token per yield always
sufficient?

- **Answer:**

---

## Research Notes
<!-- Filled in after answers received -->

## Plan Impact
<!-- Changes to main plan driven by this batch -->