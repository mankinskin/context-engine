---
tags: `#interview` `#expansion-loop` `#performance` `#streaming`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
batch: 7
topic: Performance and Streaming
status: ✅ answered
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

- **Answer:** `ExpansionCtx` should use a lazy iterator for easier stream plumbing.
  Pre-materialising the known segment into a `Vec` before entering `ExpansionCtx`
  would require consuming the lazy iterator twice (or buffering eagerly), which
  defeats the purpose of the lazy path. Instead, `ExpansionCtx` should accept a
  lazy atom iterator and materialise only as far ahead as each `insert_next_match`
  call requires.

---

**Q32.** `postfix_iter()` on a token with many parents (high-frequency compound
token in a large corpus) can yield many postfixes. In the worst case, for every
cursor step we call `postfix_iter()` and scan all postfixes. What is the expected
worst-case postfix count for a token of width `w` in a graph of `N` vertices? Is
an early-termination heuristic (e.g. stop after the first postfix whose width ≥
`remaining.len()`) correct and safe?

- **Answer:** The scenario motivating early termination **cannot occur**. Postfixes
  are subtokens of a previously expanded token — they were produced by the graph's
  own expansion machinery and therefore must fit within the query that produced
  them. A postfix that exceeds `remaining.len()` would mean the query was shorter
  than the token it came from, which is a structural invariant violation.
  The only correct early-termination condition is when the query is **fully
  consumed** (no more context remains in `remaining`), at which point postfix
  iteration and overlap search can be skipped entirely. No width-based heuristic
  is needed.

---

**Q33.** `insert_next_match` acquires write locks on the graph (via
`HypergraphRef = Arc<RwLock<...>>`). In the outer loop for `insert_sequence`,
multiple `insert_next_match` calls are made sequentially. Each call acquires and
releases the lock. Is there any advantage to batching the atom creation step
separately (all atoms resolved and inserted first) before the outer loop begins,
to avoid interleaving atom-creation writes with compound-token writes?

- **Answer:** No. Batching atom creation as a separate pre-pass is not beneficial
  because: (1) new atoms do not have parents yet and therefore cannot participate
  in the overlap expansion loop — they are invisible to `find_overlap`; (2) a
  pre-pass would require reading the entire sequence twice, or buffering it fully,
  which conflicts with the lazy streaming model (Q31). Interleaving atom creation
  with compound-token writes is safe because new atoms are inert with respect to
  overlap detection.

---

**Q34.** The `obs1` / `obs2` tests in `skill3_exploration.rs` document current
broken behaviour (`already_existed=true` and `width=1` for multi-char inserts)
and are green today as regression guards. After the outer loop fix, these tests
will assert false things and must be retired. Conversely, all `skill3_exp_*` tests
currently `#[ignore = "RC-1"]` should become green. Is there any `skill3_exp_*`
test that would remain failing after RC-1 is fixed (i.e. one that also requires
the RC-2/RC-3 inner loop fix)?

- **Answer:** Possibly. Some `skill3_exp_*` tests may require the full
  `ExpansionCtx` cursor loop fix (RC-2/RC-3) and not just the outer loop fix
  (RC-1). Rather than trying to classify them upfront, we will accept failing tests
  for now and review the entire test suite in a dedicated future session once the
  algorithm implementation is complete.

---

**Q35.** The `context-read` test suite does not compile due to 247 errors (stale
imports: `ToInsertCtx`, `PatternIndex`, `ReadRequest`, `HypergraphRef`, missing
`use` paths). These errors pre-exist the current work. Should fixing the test
compilation be a prerequisite step (done before writing any algorithm code), so
that the green/red test count is known before and after the algorithm change? Or
can the algorithm be implemented first with the test compilation fix deferred?

- **Answer:** The `context-read` tests should compile — fixing compilation is a
  prerequisite — but test **failures** are allowed. The goal is to have a
  compilable test suite so that we have a clear signal of what was passing and
  failing before and after each algorithm change. The 247 compile errors are stale
  import issues unrelated to the algorithm logic, so they can be fixed in a
  focused cleanup commit before algorithm work begins without risk of introducing
  logic regressions.

---

## Research Notes

### R21 — Lazy iterator is the correct abstraction for `ExpansionCtx` (Q31)

A31 settles the streaming design: `ExpansionCtx` must accept a lazy atom iterator,
not a pre-materialised slice. The practical implication is that `ExpansionCtx`'s
internal query window (`remaining`) must be built by consuming the lazy iterator
incrementally — as many atoms as each `insert_next_match` call requests, no more.

This means the `atoms: Pattern` field in the proposed `ExpansionCtx` struct (which
assumed a pre-materialised slice) must become a generic over a lazy iterator type,
or `ExpansionCtx` must maintain an internal buffer that it fills on demand. The
buffer approach is simpler: `ExpansionCtx` holds a `Vec<Token>` that grows as the
lazy source is consumed, and `remaining = &self.buffer[self.cursor..]` is always
valid because the buffer is never truncated.

**Implication for the struct definition:** replace `atoms: Pattern` with a pair:
```
buffer: Vec<Token>,     // materialised so far
source: impl Iterator<Item = NewAtomIndex>,  // lazy remainder
```
and add a `ensure_materialised(n: usize)` helper that pulls from `source` until
`buffer.len() >= cursor + n`.

### R22 — Postfix width heuristic is unnecessary; only query-exhaustion matters (Q32)

A32 confirms that the postfix set is structurally bounded by the query — postfixes
are derived from tokens that were themselves found by searching the query, so they
cannot exceed the query width. The only valid early exit from `postfix_iter()` is
when `remaining.is_empty()` (no more context to overlap into). No width comparison
against `remaining.len()` is needed.

This simplifies `find_overlap`: the loop body never needs to check
`postfix.width > remaining.len()` before calling `insert_next_match`. The
structural invariant guarantees this case cannot arise.

### R23 — Atom pre-pass would break lazy streaming and provides no benefit (Q33)

A33 closes the atom-batching question permanently. The reasoning is tight:
new atoms are inert for overlap purposes (no parents → `find_overlap` returns
`None` immediately via the atom fast-path, PI-14), so separating them into a
pre-pass achieves nothing for correctness. And doing so would require either a
full eager materialisation pass or a two-pass design, both of which conflict with
the lazy streaming requirement established in A31. The current interleaved model
(atom creation and compound-token writes in the same pass) is both correct and
optimal for the streaming case.

### R24 — Test suite strategy: compile first, tolerate failures (Q34, Q35)

The two answers together define the test strategy for the implementation phase:

1. **Step 0 (prerequisite):** Fix the 247 `context-read` compile errors in a
   dedicated cleanup commit. Do not change any algorithm logic in this commit.
   Confirm the suite compiles and record the baseline pass/fail counts.
2. **Step 1:** Implement the `ExpansionCtx` cursor loop (RC-2/RC-3), which
   subsumes RC-1 via the `ReadCtx::read_sequence` delegation.
3. **Step 2:** Retire `obs1`/`obs2` tests (they assert broken behaviour).
4. **Step 3:** Run the full suite; accept remaining failures; schedule a test
   review session to classify any still-failing `skill3_exp_*` tests.

This avoids both the "flying blind" problem (no compile → no signal) and the
"blocked by failing tests" problem (failures are expected during active development
of a core algorithm).

---

## Plan Impact

### PI-26 — Replace `atoms: Pattern` with a lazy-buffered source in `ExpansionCtx`

Update the `ExpansionCtx` struct definition in the Proposed Architecture section:

```rust
struct ExpansionCtx<I: Iterator<Item = NewAtomIndex>> {
    graph:   HypergraphRef,
    buffer:  Vec<Token>,    // atoms materialised so far from source
    source:  I,             // lazy remainder of the known-atom stream
    cursor:  usize,         // position within buffer
    anchor:  Option<Token>, // last overlap token (T2) or last sequential token
}
```

Add `ensure_materialised(n)` as a private helper that drains from `source` until
`buffer.len() >= self.cursor + n`, stopping at source exhaustion. All uses of
`&self.atoms[self.cursor..]` become `self.ensure_materialised(k); &self.buffer[self.cursor..]`.

### PI-27 — Remove width-comparison guard from `find_overlap`

No `postfix.width > remaining.len()` check is needed inside `find_overlap`. The
postfix structural invariant guarantees postfixes fit within the query. Document
this explicitly with a comment:

```rust
// No width guard needed: postfixes are subtokens of a previously expanded
// token and are structurally guaranteed to fit within `remaining`.
// The only early exit is remaining.is_empty() (checked before calling find_overlap).
```

### PI-28 — No atom pre-pass; interleaved writes are correct and optimal

Explicitly document in the implementation notes that atom creation and
compound-token writes are intentionally interleaved in a single pass. Add a
comment at the loop entry point:

```rust
// Atom creation and compound expansion are interleaved intentionally.
// New atoms are inert for overlap detection (no parents yet) and separating
// them into a pre-pass would require full eager materialisation, conflicting
// with the lazy streaming model.
```

### PI-29 — Implementation phase task order (incorporating all batches)

The definitive task order for the implementation phase, derived from all seven
interview batches:

1. **T0** Fix `context-read` test compilation (247 stale import errors). No logic
   changes. Record baseline pass/fail counts.
2. **T1** Review and resolve open questions OQ-1 through OQ-5 in
   [`20260315_DESIGN_ROOT_UPDATE_STEPS.md`](../designs/20260315_DESIGN_ROOT_UPDATE_STEPS.md).
3. **T2** Review `BandState::collapse()` (PI-11). Rewrite only if it cannot
   accommodate one-token-per-yield bands with externally-resolved complements.
4. **T3** Delete `append_collapsed` overlap logic (PI-21). Add comment at deletion
   site. Confirm no regressions.
5. **T4** Implement `ExpansionCtx` cursor loop with lazy-buffered source (PI-26),
   atom fast-path (PI-14), correct overlap predicate (PI-8), dynamic cursor
   advance (PI-10), `T2` anchor update (PI-22), complement resolution inside
   `find_overlap` (PI-9), `debug_assert` on cursor advance (PI-15).
6. **T5** Wire `insert_sequence` → `ReadCtx::read_sequence` (PI-16). Remove `< 2`
   guard (PI-18). One-line change in `commands/insert.rs`.
7. **T6** Retire `obs1`/`obs2` tests. Run full suite. Record new pass/fail counts.
8. **T7** Schedule test review session for any still-failing `skill3_exp_*` tests.