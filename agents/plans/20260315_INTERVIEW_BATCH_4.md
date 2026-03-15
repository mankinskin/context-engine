---
tags: `#interview` `#expansion-loop` `#cursor` `#NoExpansion`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
batch: 4
topic: Cursor Advancement and NoExpansion Handling
status: ✅ answered
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

- **Answer:** When no postfix of `T` can be expanded (i.e. `find_overlap` returns
  `None`), the cursor advances by the full `T.width`. The full advance is safe
  precisely because no overlap was found — the entire token is non-overlapping and
  can be committed as a clean sequential segment. When an overlap *is* found, the
  cursor does **not** advance by `T.width`; instead it advances to the start of
  `T2`'s largest postfix (per PI-10 from Batch 3).

---

**Q17.** In the single-level loop (RC-1 fix in `insert_sequence`), `NoExpansion`
means "advance by `token.width` and call again." In the expansion loop (RC-2/RC-3
fix in `ExpansionCtx`), `NoExpansion` triggers the postfix descent. Are these two
uses of `NoExpansion` consistent, or do they require different handling? Should
the outer `insert_sequence` loop also descend into postfixes, or is postfix descent
only needed inside `ExpansionCtx`?

- **Answer:** `insert_sequence` may be a **duplicated read endpoint**. There should
  be a single outer loop that calls `insert_next_match` repeatedly and uses
  `ExpansionCtx` to handle postfix expansion. The two-level split (RC-1 in
  `context-api`, RC-2/RC-3 in `context-read`) should converge: `insert_sequence`
  should delegate to the same loop mechanism that `ReadCtx` uses, rather than
  maintaining its own `NoExpansion`-advances-only loop. Postfix descent belongs
  to `ExpansionCtx`; the outer loop is just the cursor-advancing shell.

---

**Q18.** When `insert_next_match` returns `Complete { token }` (the entire
remaining query is consumed by an existing token), the cursor advances by
`token.width` and the block is done. But `token.width` equals `remaining.len()`
in this case. Is it possible for `Complete` to be returned with `token.width <
remaining.len()`? If so, what does that mean for the cursor?

- **Answer:** This may depend on context, but `Complete { token }` does **not**
  necessarily guarantee that `token.width == remaining.len()`. It is possible for
  `Complete` to be returned with `token.width < remaining.len()`. In that case the
  cursor advances by `token.width` only — the remaining atoms after position
  `cursor + token.width` are not yet covered and the loop must continue from the
  new cursor position. `Complete` should be treated the same as `NoExpansion` for
  cursor advancement purposes: advance by `token.width`, then check for overlap
  and continue.

---

**Q19.** After a `Created { token }` outcome — a new compound token was just
inserted via the split+join pipeline — the cursor advances by `token.width`. The
newly created token now exists in the graph. On the next `insert_next_match` call,
could the newly created token itself become the first result, effectively
re-entering a `Complete` or `NoExpansion` path with the same token? Is idempotence
guaranteed here?

- **Answer:** Yes, the same token can be found again. Whenever `insert_next_match`
  finds an expansion (including creating a new token), the cursor advances only to
  the **start of the postfix expansion of the overlap token** — not past the full
  token. This is how multiple overlapping tokens continue to be found: the overlap
  region stays in the next query window. Idempotence is guaranteed by two
  properties: (1) the query is a finite atom sequence and the cursor strictly
  advances on every step, and (2) composite patterns expand their component
  sub-patterns, so a second encounter of the same token takes the `Complete` /
  `NoExpansion` path rather than re-triggering `Created`.

---

**Q20.** The `ExpandCtx::try_new` guard — which returns `None` if
`anchor.postfix_iter()` yields nothing — currently short-circuits the entire
expansion for fresh atoms. In the proposed design, an anchor with no postfixes
(i.e. a fresh atom or a newly-created compound with no parents yet) means "no
overlap possible, advance normally." Should `find_overlap` be a no-op (return
`None`) when `anchor.postfix_iter()` is empty, or should there be a separate
fast-path that skips the postfix iterator entirely for performance?

- **Answer:** There should be explicit **fast-paths** for:
  1. **Known/unknown segment boundaries** — the boundary is always a clean cut
     (PI-1); no postfix check is needed when transitioning between segments.
  2. **Atoms** (single-character tokens) — atoms do not have true postfixes in the
     graph; a fast-path that skips `postfix_iter()` entirely avoids a guaranteed
     empty traversal.
  These fast-paths are performance optimisations but also serve as clear
  documentation of the structural invariants. `find_overlap` itself remains a
  no-op when the anchor has no parents, but the caller (`ExpansionCtx::next`)
  should skip calling `find_overlap` entirely in these cases.

---

## Research Notes

### R13 — `insert_sequence` and `ReadCtx` should share one loop (Q17)

A17 identifies a structural convergence opportunity. The current two-loop design:

```
insert_sequence  →  insert_next_match (once, no postfix descent)
ReadCtx          →  ExpansionCtx (postfix descent, no outer advance)
```

should become a single unified loop:

```
outer loop       →  insert_next_match (advances cursor)
                 →  ExpansionCtx (postfix descent on each NoExpansion)
```

`insert_sequence` should be a thin call-site that invokes this shared loop, not a
separate implementation. This resolves the RC-1/RC-2/RC-3 split artificially —
they are all symptoms of the same missing loop.

**Implication for the files-affected table:** `context-api/src/commands/insert.rs`
may shrink significantly if it delegates to a shared helper in `context-insert` or
`context-read`. The RC-1 fix and the RC-2/RC-3 fix are then a single change, not
two independent changes.

### R14 — `Complete` and `NoExpansion` have the same cursor semantics (Q18)

A18 reveals that `Complete` does not guarantee full query consumption. Both
`Complete` and `NoExpansion` advance the cursor by `token.width` and then check
for overlap. The only difference is the token's provenance:

- `Complete` — token already existed and was found by search
- `NoExpansion` — token was the best partial match; query extends beyond it
- `Created` — token was newly inserted

For cursor advancement and overlap checking, all three outcomes follow the same
rule: advance by `token.width` (or to the overlap's next_cursor), then continue.
The `Complete` / `Created` / `NoExpansion` distinction matters only for
`already_existed` tracking (Batch 6) and for deciding whether complement tokens
need to be created.

### R15 — Idempotence via finite query + composite expansion (Q19)

A19 confirms that re-encountering the same token is safe and expected. The
termination proof is:
1. The query is finite (bounded by `atoms.len()`).
2. Every step either advances the cursor by at least 1, or produces an overlap
   that advances the cursor into the overlap region (which is also strictly
   forward).
3. A `Created` token, when re-encountered, takes the `Complete` or `NoExpansion`
   path — its composite patterns are already expanded and no new `Created` outcome
   is produced for the same range.

No infinite loop is possible as long as the cursor-advance invariant holds: every
`next()` call must strictly increase `self.cursor`.

### R16 — Fast-paths are both performance and documentation (Q20)

Two explicit fast-paths should be coded:

1. **Atom fast-path:** if `remaining[0]` is an atom (single-char token with no
   parents), skip `find_overlap` entirely and yield `BandState::new(token)` with
   `cursor += 1`.
2. **Segment-boundary fast-path:** already guaranteed by PI-1 (unknown→known
   boundary is a hard cut). No code change needed — the boundary is respected by
   the segment loop structure. But the atom fast-path inside `ExpansionCtx` is
   new code.

These fast-paths also act as structural guards: if a future refactor accidentally
routes atoms through `find_overlap`, the fast-path assertion will catch it.

---

## Plan Impact

### PI-12 — Unify RC-1 and RC-2/RC-3 into a single loop

The RC-1 fix (`insert_sequence` outer loop) and the RC-2/RC-3 fix (`ExpansionCtx`
inner loop) should be implemented as **one shared mechanism**, not two independent
changes. Proposed structure:

- A shared `insert_sequence_loop(graph, atoms) -> Vec<BandState>` helper (in
  `context-insert` or `context-read`) drives `insert_next_match` + `ExpansionCtx`
  postfix descent.
- `insert_sequence` in `context-api` calls this helper.
- `ReadCtx::read_segment` also calls this helper for the known-atom block.

Update the Files Affected table: `context-api/src/commands/insert.rs` becomes a
thin wrapper; the loop logic lives in one place only.

### PI-13 — `Complete` treated same as `NoExpansion` for cursor advancement

Update the `ExpansionCtx::next` pseudo-code: the `Complete { token }` arm must
advance the cursor by `token.width` and then check for overlap, identical to
`NoExpansion`. The `Complete` vs `NoExpansion` distinction is preserved only for
`already_existed` bookkeeping (Batch 6).

### PI-14 — Add atom fast-path inside `ExpansionCtx::next`

Add an explicit check before calling `find_overlap`:

```rust
// Fast-path: atoms have no true postfixes — skip overlap search entirely.
if token.is_atom() {
    self.cursor += 1;
    self.anchor = Some(token);
    return Some(BandState::new(token));
}
```

This replaces the current `remaining.len() == 1` guard with a semantically
stronger check: it fires for any single-element atom regardless of query length,
not only at the terminal position.

### PI-15 — Cursor-advance invariant must be asserted in the loop

Add a debug assertion inside `ExpansionCtx::next` to catch any regression:

```rust
let cursor_before = self.cursor;
// ... compute next state ...
debug_assert!(self.cursor > cursor_before,
    "ExpansionCtx::next must strictly advance the cursor");
```
```

Now update the main plan's progress table, add Batch 4 impacts, and unblock Batches 5 and 6 in parallel: