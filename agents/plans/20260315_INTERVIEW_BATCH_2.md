---
tags: `#interview` `#expansion-loop` `#ExpansionCtx`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
batch: 2
topic: ExpansionCtx Loop Contract
status: ✅ answered
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

- **Answer:** `ExpansionCtx` should yield `BandState` values upward and leave
  committing to the caller. `ExpansionCtx` is transient working memory focused
  solely on finding the next expansion — it must not own `RootManager`. This
  preserves the transactional model: each yielded `BandState` represents one
  completed expansion unit; the caller commits it to the root before the next
  iteration begins. This also eliminates borrow conflicts (no shared mutable access
  to root inside the iterator) and makes `ExpansionCtx` independently testable
  without a live `RootManager`.

---

**Q7.** The proposed loop calls `insert_next_match(remaining)` at each cursor
position. `insert_next_match` takes a `Vec<Token>` (or anything `Searchable`).
Should the remaining slice be passed as a reference or moved? If the graph holds
interior mutability (`HypergraphRef = Arc<RwLock<...>>`), does passing a slice
reference conflict with the write lock that `insert_next_match` may need when
creating a new compound token?

- **Answer:** The query (the atom slice) should be **read-only**. The cursor path
  is mutably borrowed by `insert_next_match` to advance through the query and
  resolve any necessary graph mutations along the way. There is no conflict: the
  atom slice is owned by `ExpansionCtx` (not by the graph), so holding a shared
  reference to `atoms[cursor..]` does not conflict with taking a write lock on the
  `HypergraphRef` inside `insert_next_match`.

---

**Q8.** `insert_next_match` returns `Err(ErrorReason::SingleIndex(iwp))` when the
query has exactly one element. In the proposed loop, when `remaining.len() == 1`,
what should happen? Options:
  - (a) Append the single atom directly as a `BandState::new(atom)` without
        calling `insert_next_match`.
  - (b) Call `insert_next_match` anyway and handle `SingleIndex` as a special case.
  - (c) Check if the single atom has parents in the graph; if yes, run
        the postfix overlap check against it; if no, append directly.

- **Answer:** The query can contain compound tokens (not only atoms), so a
  single-element query is a single token — which by definition cannot expand
  further. Treat it as a non-expanded token and append it directly as
  `BandState::new(token)` without calling `insert_next_match`. This is option (a),
  generalised: the guard is `remaining.len() == 1`, regardless of whether the
  single element is an atom or a compound token.

---

**Q9.** After `ExpansionCtx` yields all `BandState` values and the cursor reaches
the end of `atoms`, the iterator returns `None`. At that point, `RootManager.root`
should represent the full known-atom block appended to whatever was in root before
the block started. Is there any state in `ExpansionCtx` that must be flushed on
exhaustion (e.g. a partially-built `BandState` that was not yet yielded)?

- **Answer:** Yes — padding tokens. Every pattern created from an overlap requires
  a **padding token at the beginning** to align the pattern to the full composite
  string of the new token being created. When the expansion loop ends, any overlap
  patterns that were started but have not yet had their leading padding token
  resolved must be filled. In other words: the beginning and end of every
  overlap-derived pattern must be padded so that all patterns on a given bundled
  token span exactly the same atom range (tight packing). This padding resolution
  is the flush step that must happen at exhaustion — or lazily on the final
  `commit_state` call — before `None` is returned.

---

**Q10.** The current `ExpansionCtx` stores a `BandState` that accumulates tokens
across multiple `Cap` operations (appending to the single band without committing).
In the proposed design, each cursor step yields one `BandState` immediately. Does
removing the accumulation change the semantics? Specifically: is a `BandState`
consisting of multiple tokens ever needed, or is one token per yield always
sufficient?

- **Answer:** One token per yield is always sufficient. We must commit after each
  expansion to guarantee that the latest graph structure is available for the next
  expansion step — a later `insert_next_match` call may depend on tokens created
  by the previous commit. Accumulating multiple expansions before committing would
  mean operating on a stale graph view and could miss valid expansions. The
  accumulation in the current code is therefore not a semantic requirement but an
  artefact of the missing cursor loop; it can be removed.

---

## Research Notes

### R5 — Transactional commit model is load-bearing (Q10)

The A10 answer establishes a critical ordering invariant:

> **Each `BandState` must be committed to the root before the next
> `insert_next_match` call is made.**

This is not just a style preference — it is a correctness requirement. If a
`Created` outcome produces a new compound token at step N, the graph must reflect
that token before step N+1 runs, otherwise step N+1's postfix search may miss
valid overlap candidates that include the newly-created token. The one-yield-per-
step model is therefore mandatory, not optional.

### R6 — Padding tokens are the flush obligation (Q9)

The A9 answer identifies a non-trivial flush requirement: every overlap-derived
pattern must carry a **leading padding token** to align it to the full width of
the bundled token. Concretely, if the bundled token spans atoms `[0..5]` and the
overlap pattern only covers `[2..5]`, atoms `[0..1]` must be represented by a
padding token prepended to that pattern.

This flush is tied to the `BandState::collapse()` call (Batch 5, Q22). It implies
that `collapse()` — or the code that populates the `overlap: Band` — must resolve
padding tokens at both ends before calling `graph.insert_patterns`. The padding
token for a given range is found by calling `insert_next_match` on that atom
sub-slice (which returns the tightest existing token covering it), consistent with
PI-4 from Batch 1.

### R7 — Single-element guard applies to compound tokens too (Q8)

A8 generalises the single-element guard beyond atoms: any single-token remainder
— atom or compound — cannot expand and is appended directly. This simplifies the
loop's terminal case: no special atom-vs-compound branch is needed. The guard is
purely `remaining.len() == 1`.

### R8 — Cursor path mutability is inside `insert_next_match` (Q7)

The atom slice is owned by `ExpansionCtx` and borrowed read-only by the query.
All mutable state (cursor advancement, graph writes) lives inside
`insert_next_match`. This means `ExpansionCtx::next` can hold `&self.atoms[self.cursor..]`
as a shared slice without any lifetime conflict with the `HypergraphRef` write
lock. The borrow checker should be satisfied without unsafe code or cloning the
slice.

---

## Plan Impact

### PI-5 — Commit before each next step (mandatory ordering)

The proposed `ExpansionCtx` pseudo-code in the main plan shows:

```rust
expansion_states = ExpansionCtx::new(...).collect::<Vec<BandState>>()
for state in expansion_states:
    root.commit_state(state)
```

This **must change**. Collecting all states into a `Vec` before any commit is
incorrect — step N+1 may depend on the graph state produced by committing step N.
The correct pattern is:

```rust
let mut ctx = ExpansionCtx::new(graph, known, root.last_token());
while let Some(state) = ctx.next() {
    root.commit_state(state);   // commit before next ctx.next() call
}
```

Update the Proposed Architecture section of the main plan accordingly.

### PI-6 — Padding token resolution is part of `collapse()` / band construction

The flush obligation identified in R6 means `BandState::collapse()` (or the band
construction logic that feeds it) must:
1. Determine the full atom range of the bundled token.
2. For each overlap-derived pattern, prepend a padding token covering any leading
   atoms not already covered by the overlap band.
3. Similarly append a padding token for any trailing atoms.

Padding tokens are resolved via `insert_next_match` on the relevant atom sub-
slice. This feeds directly into Batch 3 (Q12, Q13) — complement tokens and
padding tokens are the same concept approached from different angles.

### PI-7 — Remove `collect()` anti-pattern from plan pseudo-code

Update the main plan's Proposed Architecture section to replace the `.collect()`
pattern with the interleaved commit loop (PI-5). Add a warning comment in the
implementation plan:

```
// DO NOT collect() ExpansionCtx before committing.
// Each state must be committed before the next state is computed.
```
