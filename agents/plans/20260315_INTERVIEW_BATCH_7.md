---
tags: `#interview` `#expansion-loop` `#performance` `#streaming`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
batch: 7
topic: Performance and Streaming
status: 🔴 blocked-by-batch-6
---

# Interview — Batch 7: Performance and Streaming

> These questions identify performance trade-offs and constraints from the streaming
> (`LazyAtomIter`) architecture.

---

**Q31.** In the eager path (`ReadCtx::new`), all atoms are resolved upfront and
`SegmentIter` classifies them all before any `ExpansionCtx` runs. In the lazy path
(`ReadCtx::from_chars`), atoms are resolved on demand. The proposed `ExpansionCtx`
inner loop needs to call `insert_next_match` with `atoms[cursor..]`. In the lazy
path, `atoms[cursor..]` may not yet be materialised. Should the lazy path
pre-materialise the current known-segment into a `Vec<Token>` before entering
`ExpansionCtx`, or should `ExpansionCtx` be able to work with a lazy iterator?

- **Answer:**

---

**Q32.** `postfix_iter()` on a token with many parents (high-frequency compound
token in a large corpus) can yield many postfixes. In the worst case, for every
cursor step we call `postfix_iter()` and scan all postfixes. What is the expected
worst-case postfix count for a token of width `w` in a graph of `N` vertices? Is
an early-termination heuristic (e.g. stop after the first postfix whose width ≥
`remaining.len()`) correct and safe?

- **Answer:**

---

**Q33.** `insert_next_match` acquires write locks on the graph (via
`HypergraphRef = Arc<RwLock<...>>`). In the outer loop for `insert_sequence`,
multiple `insert_next_match` calls are made sequentially. Each call acquires and
releases the lock. Is there any advantage to batching the atom creation step
separately (all atoms resolved and inserted first) before the outer loop begins,
to avoid interleaving atom-creation writes with compound-token writes?

- **Answer:**

---

**Q34.** The `obs1` / `obs2` tests in `skill3_exploration.rs` document current
broken behaviour (`already_existed=true` and `width=1` for multi-char inserts)
and are green today as regression guards. After the outer loop fix, these tests
will assert false things and must be retired. Conversely, all `skill3_exp_*` tests
currently `#[ignore = "RC-1"]` should become green. Is there any `skill3_exp_*`
test that would remain failing after RC-1 is fixed (i.e. one that also requires
the RC-2/RC-3 inner loop fix)?

- **Answer:**

---

**Q35.** The `context-read` test suite does not compile due to 247 errors (stale
imports: `ToInsertCtx`, `PatternIndex`, `ReadRequest`, `HypergraphRef`, missing
`use` paths). These errors pre-exist the current work. Should fixing the test
compilation be a prerequisite step (done before writing any algorithm code), so
that the green/red test count is known before and after the algorithm change? Or
can the algorithm be implemented first with the test compilation fix deferred?

- **Answer:**

---

## Research Notes
<!-- Filled in after answers received -->

## Plan Impact
<!-- Changes to main plan driven by this batch -->