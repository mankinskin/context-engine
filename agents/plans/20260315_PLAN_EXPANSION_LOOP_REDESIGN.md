---
tags: `#plan` `#context-read` `#context-insert` `#algorithm` `#expansion` `#overlap` `#refactoring`
summary: Redesign the ExpansionCtx inner loop so it drives insert_next_match in a cursor-advancing loop over known-atom segments, correctly handles the new/known classification boundary, and collects a tight set of decomposition patterns from all detected overlaps.
status: 📋 interview
phase: 1-interview
parent: 20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md
related: 20260315_PLAN_INTEGRATION_TEST_REMEDIATION.md, 20260314_PLAN_INSERT_NEXT_MATCH.md, 20260314_PLAN_APPEND_TO_PATTERN_FIX.md
priority: top — this is the core algorithm fix that unblocks RC-1, RC-2, RC-3 and all 20 ignored integration tests
---

# Plan: Expansion Loop Redesign — `insert_next_match` Cursor Loop inside `ExpansionCtx`

**Date:** 2026-03-15
**Scope:** Medium (algorithm change in `context-read`; call-site ripple into `context-api`)
**Crates:** `context-read`, `context-api`
**Test baseline:** 44 pass / 9 fail / 22 ignored in `cli_integration`; `context-read` crate does not compile its test suite (247 errors — stale imports, not logic failures)

---

## Table of Contents

1. [Objective](#objective)
2. [Context](#context)
3. [Current Architecture (as-is)](#current-architecture-as-is)
4. [Root Cause Analysis](#root-cause-analysis)
5. [Proposed Architecture (to-be)](#proposed-architecture-to-be)
6. [Interview Batches](#interview-batches)
7. [Files Affected](#files-affected)
8. [Risks](#risks)
9. [Related Documents](#related-documents)
10. [Notes](#notes)

---

## Objective

Fix the read/insert pipeline so that:

1. `ReadCtx` + `ExpansionCtx` correctly process a known-atom block by driving
   `insert_next_match` in a cursor-advancing loop, finding overlaps via postfix
   descent, and collecting a complete set of tight decomposition patterns.
2. `insert_sequence` in `context-api` reuses the same mechanism so that multi-
   character insertions produce a correct compound root token (not just the first
   atom).
3. The new/known classification is preserved as the correct boundary control —
   new atoms are concatenated directly without overlap search; known atoms go
   through the expansion loop.

---

## Context

### Architecture layers involved

```
ReadCtx (context-read)
  └─ SegmentIter          — splits input into (unknown-run, known-run) pairs
       └─ RootManager     — accumulates the growing root token
            ├─ append_pattern    — handles unknown atoms (direct concatenation)
            └─ BlockExpansionCtx — handles known atoms (overlap search)
                 └─ ExpansionCtx — drives postfix iteration
                      └─ ExpandCtx  — per-postfix: calls insert() to test expansion
```

### What the algorithm is supposed to do

The read algorithm processes input left-to-right. At each position it asks:
*"what is the largest token already in the graph that matches the input starting
here?"* If found via search, it records that token and advances the cursor by its
width. If the next call from the new cursor position also finds an existing token
whose **prefix overlaps** with the **postfix** of the previously found token, an
overlap is detected and both decompositions are recorded into the graph.

The new/known classification is the efficiency gate: a **new** atom (one that did
not exist before this read began) cannot be part of any existing compound token
and therefore cannot be part of any overlap. Processing it is just concatenation.
A **known** atom (pre-existing) may already be a child of compound tokens, so
overlap search is warranted.

### Key types

| Type | Location | Role |
|------|----------|------|
| `NewAtomIndex` | `context-trace` | `New(idx)` or `Known(idx)` — produced by lazy/eager atom resolution |
| `SegmentIter<I>` | `context-read/segment.rs` | Partitions `NewAtomIndex` stream into `NextSegment { unknown, known }` |
| `NextSegment` | `context-read/segment.rs` | One `(unknown: Pattern, known: Pattern)` pair |
| `RootManager` | `context-read/context/root.rs` | Owns the accumulating root `Option<Token>`; provides `append_pattern`, `append_token`, `commit_state` |
| `BlockExpansionCtx` | `context-read/expansion/block.rs` | Wraps `ExpansionCtx`; calls `process()` then `finish()` |
| `ExpansionCtx` | `context-read/expansion/mod.rs` | Owns `CursorCtx` + `BandState`; `Iterator` yielding `BandState` |
| `ExpandCtx<'_>` | `context-read/expansion/chain/expand.rs` | Per-step: iterates postfixes of anchor token, calls `insert()` per postfix |
| `BandState` | `context-read/expansion/chain/mod.rs` | `Single { band }` or `WithOverlap { primary, overlap, link }` |
| `Band` | `context-read/expansion/chain/band.rs` | Pattern + start/end bounds |
| `InsertOutcome` | `context-insert/insert/outcome.rs` | `Created`, `Complete`, `NoExpansion` — each carries `IndexWithPath + Response` |
| `insert_next_match` | `context-insert/insert/mod.rs` | Single-step: find or create largest-match token at query start |

---

## Current Architecture (as-is)

### Segment loop in ReadCtx

```
ReadCtx::read_sequence()
  for each NextSegment { unknown, known } from SegmentIter:
    root.append_pattern(unknown)   ← direct concat, no search
    BlockExpansionCtx::new(root, known).process()
      while ExpansionCtx::next() -> Some(state):
        root.commit_state(state)
```

### ExpansionCtx initialisation

On construction `ExpansionCtx::new` checks if a `band` (root's last token) is
provided. If not, it calls `insert_next_match` once to find the first token from
the cursor and builds a `BandState::Single` from it. The `cursor` is the known
pattern as a `PatternRangePath`.

### ExpandCtx iteration (per ExpansionCtx::next step)

`ExpandCtx::try_new` takes the anchor token (last token in `BandState`'s primary
band) and calls `anchor.postfix_iter()`. If the anchor has no parents in the graph
(e.g. it is a fresh atom with no compound ancestors), `postfix_iter()` returns
nothing and `ExpandCtx::try_new` returns `None`.

For each postfix `P` of the anchor (largest first):
- Calls `insert(&cursor)` — tests whether the cursor can expand from this postfix
- If `Ok(expansion)` → `ChainOp::Expansion` (overlap found)
- If `Err(_)` → `ChainOp::Cap` (no expansion from this postfix)

`ExpansionCtx::next` picks the first `Expansion` or a qualifying `Cap` and calls
`apply_op`, producing a new `BandState`.

### The problem

`ExpandCtx` calls `insert(&cursor)` with the **same static cursor for every
postfix**. The cursor is not advanced between postfix probes. After finding an
overlap and committing via `commit_state`, the cursor is also not advanced past
the consumed atoms. `BlockExpansionCtx::process` loops over `ExpansionCtx::next`
but `ExpansionCtx` has no mechanism to advance the cursor through the known-atom
block: it handles exactly one overlap event and then exhausts.

Consequence: on the first call to a fresh graph for `"aaaa"`, atom `a` has no
parents, `postfix_iter()` is empty, `ExpandCtx::try_new` returns `None`,
`ExpansionCtx::next` returns `None` immediately, and the loop exits with the root
still holding only the first appended atom (width=1 from `append_pattern`).

For `insert_sequence("hello")` in `context-api`, the call sequence does not go
through `ReadCtx` at all — it calls `insert_next_match` once with the full atom
slice, gets `NoExpansion { token=h, width=1 }`, and returns `h` as the root.

---

## Root Cause Analysis

There are two distinct missing loops:

### RC-1 — `insert_sequence` has no outer loop

`WorkspaceManager::insert_sequence` calls `insert_next_match([a₀…aₙ])` once.
On a fresh graph `insert_next_match` returns `NoExpansion { token=a₀, width=1 }`.
The function returns `a₀` as the root. Fix: add a cursor-advancing `while` loop
that calls `insert_next_match` with the remaining slice and collects segment tokens
until `cursor == atoms.len()`, then wraps the segments into a compound root.

### RC-2/RC-3 — `ExpansionCtx` has no inner cursor loop

Within a known-atom block, `ExpansionCtx` does not call `insert_next_match` in a
loop over the atoms. After one overlap event (or zero if no postfixes exist), the
expansion stops. The cursor inside `CursorCtx` is a `PatternRangePath` and is
never advanced in the current code after an overlap is committed.

Fix: `ExpansionCtx` must drive a cursor loop:
1. Call `insert_next_match(remaining_known_atoms)` from current cursor position.
2. If `Created` or `Complete` → record the token, advance cursor by `token.width`.
3. If `NoExpansion { token, width }` → the token matches at the cursor but the
   block is longer. Descend postfixes of `token` to check for overlap with the
   next part of the block. If overlap found, record both decompositions and advance
   cursor by `token.width - postfix.width` (up to the overlap start). If no
   overlap, advance cursor by `token.width`.
4. Repeat until cursor reaches the end of the known-atom block.

---

## Proposed Architecture (to-be)

```
ReadCtx::read_sequence()
  for each NextSegment { unknown, known } from SegmentIter:
    root.append_pattern(unknown)          ← unchanged: direct concat
    if !known.is_empty():
      expansion_states = ExpansionCtx::new(graph, known, root.last_token())
                           .collect::<Vec<BandState>>()
      for state in expansion_states:
        root.commit_state(state)
```

### ExpansionCtx (to-be)

```
struct ExpansionCtx {
    graph:   HypergraphRef,
    atoms:   Pattern,          // the full known-atom block for this segment
    cursor:  usize,            // atom-index position within `atoms`
    anchor:  Option<Token>,    // last committed token (for postfix overlap search)
    pending: Option<BandState> // buffered state awaiting commit
}

impl Iterator for ExpansionCtx {
    type Item = BandState;

    fn next(&mut self) -> Option<BandState> {
        if self.cursor >= self.atoms.len() { return None; }

        let remaining = &self.atoms[self.cursor..];

        // Single atom: direct advance, no insert_next_match (would error)
        if remaining.len() == 1 {
            let token = remaining[0];
            self.cursor += 1;
            self.anchor = Some(token);
            return Some(BandState::new(token));
        }

        let outcome = insert_next_match(graph, remaining)?;

        match outcome {
            Complete { token } | Created { token } => {
                self.cursor += token.width;
                self.anchor = Some(token);
                Some(BandState::new(token))
            }

            NoExpansion { token } => {
                // token is the best match starting at cursor (width = token.width)
                // Check postfixes of token for overlap with remaining[token.width..]
                let overlap = self.find_overlap(token, &remaining[token.width..]);
                self.cursor += token.width;  // always advance past token
                self.anchor = Some(token);
                match overlap {
                    Some((postfix, expansion)) =>
                        Some(BandState::with_overlap(token, postfix, expansion)),
                    None =>
                        Some(BandState::new(token))
                }
            }
        }
    }
}
```

`find_overlap(anchor, next_remaining)`:
- Iterates postfixes of `anchor`, largest first.
- For each postfix `P`, calls `insert_next_match(next_remaining)` — if the result
  token's width equals `P.width`, the overlap region is `P`.
- Returns `Some((postfix, expansion_token))` on first match, `None` if exhausted.

### insert_sequence outer loop (context-api, to-be)

```rust
let mut cursor = 0;
let mut segments: Vec<Token> = Vec::new();

while cursor < atoms.len() {
    let remaining = &atoms[cursor..];
    if remaining.len() == 1 {
        segments.push(remaining[0]);
        cursor += 1;
        continue;
    }
    let outcome = insert_next_match(&graph_ref, remaining.to_vec())?;
    let width = outcome.token().width.0;
    assert!(width > 0, "insert_next_match returned zero-width token");
    segments.push(outcome.token());
    cursor += width;
}

let root = if segments.len() == 1 {
    segments[0]
} else {
    graph_ref.insert_pattern(segments)
};
```

---

## Interview Batches

Interview questions are split into separate files for focused iteration. Each
batch is answered independently; answers feed back into the plan via **Plan
Impact** sections in each batch file.

| Batch | File | Topic | Status |
|-------|------|-------|--------|
| 1 | [20260315_INTERVIEW_BATCH_1.md](20260315_INTERVIEW_BATCH_1.md) | Classification Boundary | 🟡 awaiting-answers |
| 2 | [20260315_INTERVIEW_BATCH_2.md](20260315_INTERVIEW_BATCH_2.md) | ExpansionCtx Loop Contract | 🔴 blocked-by-batch-1 |
| 3 | [20260315_INTERVIEW_BATCH_3.md](20260315_INTERVIEW_BATCH_3.md) | Overlap Collection and Decomposition Output | 🔴 blocked-by-batch-2 |
| 4 | [20260315_INTERVIEW_BATCH_4.md](20260315_INTERVIEW_BATCH_4.md) | Cursor Advancement and NoExpansion Handling | 🔴 blocked-by-batch-3 |
| 5 | [20260315_INTERVIEW_BATCH_5.md](20260315_INTERVIEW_BATCH_5.md) | RootManager and Commit Contract | 🔴 blocked-by-batch-4 |
| 6 | [20260315_INTERVIEW_BATCH_6.md](20260315_INTERVIEW_BATCH_6.md) | insert_sequence Outer Loop in context-api | 🔴 blocked-by-batch-4 |
| 7 | [20260315_INTERVIEW_BATCH_7.md](20260315_INTERVIEW_BATCH_7.md) | Performance and Streaming | 🔴 blocked-by-batch-6 |

---

## Files Affected

> Preliminary — final list after interview answers are incorporated.

| File | Change | RC |
|------|--------|----|
| `crates/context-read/src/expansion/mod.rs` | Redesign `ExpansionCtx` iterator — add `atoms`, `cursor`, `anchor` fields; implement `insert_next_match` loop with postfix overlap search | RC-2/RC-3 |
| `crates/context-read/src/expansion/block.rs` | Simplify `BlockExpansionCtx` — may reduce to thin wrapper or dissolve | RC-2/RC-3 |
| `crates/context-read/src/expansion/chain/mod.rs` | `BandState::new` / `BandState::with_overlap` constructors; `collapse` may need adjustment | RC-2/RC-3 |
| `crates/context-read/src/expansion/chain/expand.rs` | `ExpandCtx` may be repurposed as the per-step postfix probe helper (no longer an `Iterator` driving the outer loop) | RC-2/RC-3 |
| `crates/context-read/src/context/root.rs` | `append_collapsed` overlap logic — may be removed if overlaps are fully committed via `BandState::WithOverlap` | RC-2/RC-3 |
| `crates/context-read/src/context/mod.rs` | `ReadCtx::read_segment` — simplify to delegate to new `ExpansionCtx` | RC-2/RC-3 |
| `crates/context-api/src/commands/insert.rs` | `insert_sequence` — add outer cursor-advancing loop | RC-1 |
| `crates/context-read/src/tests/cursor.rs` | Fix compilation (stale imports); update `TODO` tests to assert correct behaviour | all |
| `crates/context-read/src/tests/linear.rs` | Should pass without changes once RC-1 fixed | RC-1 |
| `crates/context-read/src/tests/overlapping.rs` | Should pass after RC-2/RC-3 fixed | RC-2/RC-3 |
| `tools/context-cli/tests/FAILING_TESTS.md` | Update after each fix round | all |

---

## Risks

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|-----------|
| Zero-width token from `insert_next_match` causes infinite loop | Medium | Critical | Add `assert!(width > 0)` guard immediately inside the loop |
| `BandState::collapse` produces wrong patterns when called once-per-token | Medium | High | Add unit tests for `collapse` with single-token `Single` and two-token `WithOverlap` before touching the loop |
| `append_collapsed` overlap logic in `RootManager` fires redundantly alongside `ExpansionCtx` overlap, double-committing | Medium | High | Answer Q21 (Batch 5) before touching `RootManager`; disable the `append_collapsed` overlap path if `ExpansionCtx` takes over |
| RC-2 (`read_sequence` returns `None` after insert) survives even after RC-1 fix | Medium | Medium | Re-run RC-2 tests immediately after RC-1 fix; if still failing, trace `ReadCtx::new` construction after prior `insert_sequence` call |
| Fixing `ExpansionCtx` loop breaks the 44 currently-passing integration tests | Low | High | Run full suite after every sub-step; do not merge until 44 still pass |
| `context-read` test compilation fix introduces unexpected test failures | Low | Medium | Fix compilation first in a separate commit; confirm 0 new regressions before algorithm work begins |

---

## Related Documents

| Document | Relationship |
|----------|-------------|
| [`20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md`](20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md) | Grandparent plan — RC-1/RC-2/RC-3 are Phase 3 items |
| [`20260315_PLAN_INTEGRATION_TEST_REMEDIATION.md`](20260315_PLAN_INTEGRATION_TEST_REMEDIATION.md) | Fix round structure — Round 1 (RC-1), Round 2 (RC-2), Round 3 (RC-3) |
| [`20260314_PLAN_INSERT_NEXT_MATCH.md`](20260314_PLAN_INSERT_NEXT_MATCH.md) | ✅ Complete — `InsertOutcome` enum available; `insert_next_match` API stable |
| [`20260314_PLAN_APPEND_TO_PATTERN_FIX.md`](20260314_PLAN_APPEND_TO_PATTERN_FIX.md) | ✅ Complete — `extend_root_pattern` / `append_to_owned_pattern` available |
| [`20260315_PLAN_NGRAMS_ORACLE_VALIDATION.md`](20260315_PLAN_NGRAMS_ORACLE_VALIDATION.md) | Oracle tests are the long-form acceptance criteria for the fixed algorithm |
| `tools/context-cli/tests/FAILING_TESTS.md` | Live failure tracker |

---

## Notes

### Deviations from Plan
<!-- Track changes made during execution -->
-

### Plan Impacts from Interview

#### From Batch 1 (Classification Boundary)

- **PI-1** — No cross-boundary overlap guard needed. A `New` atom cannot be part
  of any pre-existing compound token, so the unknown→known boundary is always a
  clean cut. Add a safety comment at that boundary in the implementation.
- **PI-2** — Confirm the `read_sequence` segment loop is unbounded (handles
  arbitrary many `NextSegment` values). Add `"abac"` as a required test case.
- **PI-3** — `SegmentIter` may be redundant in the lazy path (per-atom dispatch
  could replace it). Defer to a post-fix-round cleanup plan; no action during
  this redesign.
- **PI-4** — Tight-packing rule (Q3) means complement tokens in overlap
  decompositions must be resolved via `insert_next_match` (best existing match),
  not constructed from raw atom slices. This feeds into Batch 3 (Q12/Q13).

### Interview Progress

| Batch | Status | Answer file |
|-------|--------|-------------|
| 1 — Classification Boundary | ✅ answered | [BATCH_1](20260315_INTERVIEW_BATCH_1.md) |
| 2 — ExpansionCtx Loop Contract | 🟡 awaiting-answers | [BATCH_2](20260315_INTERVIEW_BATCH_2.md) |
| 3 — Overlap Collection and Decomposition Output | 🔴 blocked-by-batch-2 | [BATCH_3](20260315_INTERVIEW_BATCH_3.md) |
| 4 — Cursor Advancement and NoExpansion Handling | 🔴 blocked-by-batch-3 | [BATCH_4](20260315_INTERVIEW_BATCH_4.md) |
| 5 — RootManager and Commit Contract | 🔴 blocked-by-batch-4 | [BATCH_5](20260315_INTERVIEW_BATCH_5.md) |
| 6 — insert_sequence Outer Loop in context-api | 🔴 blocked-by-batch-4 | [BATCH_6](20260315_INTERVIEW_BATCH_6.md) |
| 7 — Performance and Streaming | 🔴 blocked-by-batch-6 | [BATCH_7](20260315_INTERVIEW_BATCH_7.md) |

### Lessons Learned
<!-- Post-execution -->
-