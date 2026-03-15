---
tags: `#design` `#RootManager` `#ExpansionCtx` `#commit` `#overlap` `#reading`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
created-from: 20260315_INTERVIEW_BATCH_5.md#Q22
status: 📋 draft
---

# Design: Root Update Steps During Reading

> **Purpose:** Precisely and completely define every root update step that occurs
> during `ReadCtx::read_sequence`, so that `commit_state` / `append_collapsed` /
> `RootManager` can be correctly implemented for all structural cases.
>
> This document was created because Q22 in Batch 5 revealed that the interaction
> between `BandState::collapse()`, `commit_state`, and `RootManager` is subtle
> enough to warrant a dedicated design session before implementation begins.

---

## Table of Contents

1. [Invariants](#invariants)
2. [Root States](#root-states)
3. [Update Operations](#update-operations)
4. [Case Catalogue](#case-catalogue)
   - [Case A: First token, root is None](#case-a-first-token-root-is-none)
   - [Case B: Sequential append, no overlap](#case-b-sequential-append-no-overlap)
   - [Case C: Overlap — root is atomic, anchor equals overlap start](#case-c-overlap--root-is-atomic-anchor-equals-overlap-start)
   - [Case D: Overlap — root is compound, last child is the overlap anchor](#case-d-overlap--root-is-compound-last-child-is-the-overlap-anchor)
   - [Case E: Overlap — bundled token replaces last child of root pattern](#case-e-overlap--bundled-token-replaces-last-child-of-root-pattern)
   - [Case F: can_extend invalidated mid-loop](#case-f-can_extend-invalidated-mid-loop)
5. [Anchor Tracking](#anchor-tracking)
6. [Algorithm Sketch](#algorithm-sketch)
7. [Open Questions](#open-questions)

---

## Invariants

These invariants must hold after every `commit_state` call, regardless of which
case is entered:

1. **Coverage:** `root` covers `atoms[0..cursor]` exactly — no gap, no
   double-cover.
2. **Tight packing:** every pattern stored on every token in the root's subtree
   uses the tightest available child tokens. No raw atom sequences where a
   compound token exists.
3. **Downward reachability:** for any atom at position `i`, there is a path from
   `root` downward through child patterns to the atom token at position `i`.
4. **No redundant decompositions:** only decompositions that are not derivable
   from another already-stored decomposition are written. (Confirmed by Batch 1,
   Q3: `[X, a, b]` is not stored when `[X, ab]` is already present.)
5. **Single source of overlap detection:** overlaps are found exclusively by
   `ExpansionCtx` via postfix expansion. `append_collapsed` must not contain
   independent overlap logic (Batch 5, Q21).
6. **Commit-before-next:** every `BandState` is committed to the root before the
   next `ExpansionCtx::next()` call (Batch 2, PI-5). The graph is always
   up-to-date at the start of each `next()`.

---

## Root States

At any point during reading, the root `Option<Token>` can be in one of these
structural states:

| State | Description | `can_extend`? |
|-------|-------------|---------------|
| `None` | No tokens committed yet | N/A — first commit creates root |
| `Atom` | Root is a single atom token (width 1, no parents) | Yes |
| `Compound-extendable` | Root is a compound token with exactly one child pattern and no parents | Yes |
| `Compound-shared` | Root has >1 child patterns or has parents (overlap was committed) | No — must wrap |

`can_extend` = `child_patterns().len() == 1 && parents().is_empty()`

**Key insight from Batch 5 Q24:** committing a `WithOverlap` state writes two
patterns onto the bundled token. If the root *is* that bundled token, the root
transitions from `Compound-extendable` to `Compound-shared` at the moment of
commit, because it now has two child patterns. All subsequent commits that try to
extend the root must use the wrap path, not the extend-in-place path.

---

## Update Operations

There are three primitive update operations on `RootManager`:

### Op-1: `set_root(token)`
Used when `root` is `None`. Sets root to `token` directly.
- Pre-condition: `root == None`
- Post-condition: `root == Some(token)`, coverage = `token.width`

### Op-2: `extend_root(token)`
Used when `root` is `Compound-extendable`. Appends `token` to the root's single
child pattern in-place.
- Pre-condition: `root.is_some() && can_extend(root)`
- Post-condition: root's single pattern now ends with `token`; coverage increased
  by `token.width`
- **Invalidated** once a second pattern is added to root (after a `WithOverlap`
  commit that targets root itself — see Case C).

### Op-3: `wrap_root(token)`
Used when `root` is `Compound-shared` (or `Atom` when the new token bundles the
root). Creates a new compound token whose first pattern is `[root, token]`.
- Pre-condition: `root.is_some() && !can_extend(root)`
- Post-condition: a new wrapping token is set as root; coverage increased by
  `token.width`

---

## Case Catalogue

### Case A: First token, root is None

**Trigger:** `commit_state(BandState::Single { token })` when `root == None`.

**Steps:**
1. Call `Op-1: set_root(token)`.
2. Set anchor = `token`.
3. Cursor is already at `token.width` (set by `ExpansionCtx`).

**Root after:** `Some(token)` — state depends on `token`'s graph structure.

---

### Case B: Sequential append, no overlap

**Trigger:** `commit_state(BandState::Single { token })` when `root.is_some()`.

**Steps:**
1. If `can_extend(root)`: call `Op-2: extend_root(token)`.
2. Else: call `Op-3: wrap_root(token)`.
3. Set anchor = `token`.

**Root after:** root coverage extended by `token.width`. `can_extend` status
depends on whether Op-2 or Op-3 was used and whether `token` itself has parents.

---

### Case C: Overlap — root is atomic, anchor equals overlap start

**Trigger:** `commit_state(BandState::WithOverlap { ... })` and `root` is a
single atom that is identical to the first token of the overlap region.

This is the case where the very first token read becomes the left side of an
overlap. Example: reading `"ab"` on a graph that already contains token `ab`.
- `root = a` (atom, set by the unknown segment or first expansion step)
- overlap found: `a` overlaps with `ab` via postfix `a`

**Steps:**
1. `BandState::collapse()` produces bundled token `B` with two patterns:
   - primary: `[a, b_suffix]`
   - overlap: `[a_prefix(?), ab]`  ← padding may be empty if `a` is width 1
2. Call `Op-1 / Op-3` to set/replace root with `B`.
3. Set anchor = the overlap token (i.e. `ab`, the expansion result — see
   Anchor Tracking below).

**Note:** if the root atom has width 1 and the overlap postfix is also width 1
(i.e. they are the same single atom), the "prefix complement" is empty. The
overlap pattern is simply `[ab]` (one token covering the full bundled width).
Padding logic must handle the zero-width prefix case.

---

### Case D: Overlap — root is compound, last child is the overlap anchor

**Trigger:** `commit_state(BandState::WithOverlap { ... })` and `root` is a
compound token whose **last child** is the token that produced the overlap.

Example: root = `[X, ab]`, last child = `ab`, overlap found between `ab` and
next token `bc` via postfix `b`.

**Steps:**
1. `BandState::collapse()` produces bundled token `B` covering `[ab, bc]` with:
   - primary: `[ab, c_suffix]`
   - overlap: `[a_prefix, bc]`
2. The root's last child (`ab`) is **replaced** by `B` in the root pattern:
   - root's pattern becomes `[X, B]`
3. If `can_extend(root)`: update the pattern in-place (Op-2 variant — replace
   last element rather than append).
4. Else: this requires creating a new root token with pattern `[X, B]`.
5. Set anchor = overlap token (i.e. `bc`, the expansion result).

**Key design question:** Does `RootManager` expose a "replace last child" primitive,
or is this done by collapsing the whole root and rebuilding? Replacing in-place
is only valid when `can_extend` holds. If not, a new wrapping token must be created
from `[..., B]` where `...` is all but the last child of root's first pattern.

---

### Case E: Overlap — bundled token replaces last child of root pattern

This is the general form of Case D, extended to handle the case where the root
is already `Compound-shared` (has multiple patterns or parents).

**Steps:**
1. Produce bundled token `B` via `collapse()` as in Case D.
2. Since `!can_extend(root)`, extract the root's first pattern minus its last
   child: `prefix_of_root_pattern = root.first_pattern()[..last]`.
3. Create a new root token with pattern `[prefix_of_root_pattern..., B]`.
   This new token is set as the root via `Op-1`.
4. Set anchor = overlap token.

---

### Case F: can_extend invalidated mid-loop

**Trigger:** After a `WithOverlap` commit, the root transitions to
`Compound-shared` (two patterns written to the bundled token). The *next*
`commit_state` call can no longer use `extend_root`; it must use `wrap_root`.

**Steps:** same as Case B, but Op-2 is unavailable. Op-3 is used.

This means the loop must re-check `can_extend(root)` on every `commit_state` call.
It cannot be cached from a previous step.

---

## Anchor Tracking

The anchor is the token that `ExpansionCtx` uses to search for the next overlap
via `find_overlap`. After each `commit_state`, the anchor is updated as follows:

| Committed state | New anchor |
|-----------------|-----------|
| `Single { token }` | `token` |
| `WithOverlap { primary, overlap, link }` | The **expansion result** token — i.e. the token returned by `insert_next_match` that qualified as an overlap (the right-side token `T2`, not the bundled token `B`). |

**Rationale (from Batch 5, Q23):** The overlap token (expansion result, `T2`) is
the largest and leftmost match found in the current query window. It serves as the
next anchor because its postfixes are the most likely candidates for the *next*
overlap — they represent the shortest path to the next largest leftmost match.
The cursor is positioned at `T2`'s largest postfix start, so the next
`insert_next_match` call starts from inside `T2`'s range.

**Not the bundled token `B`:** `B` is a new composite; its postfixes are not yet
meaningful in the graph (it was just created). Using `B` as anchor would always
yield an empty postfix set and skip the next overlap check.

**Not the rightmost atom of `B`:** this was the assumption in the original plan,
but it is incorrect (confirmed by Batch 5, Q23).

---

## Algorithm Sketch

This integrates all cases into a single loop pseudocode:

```
ReadCtx::read_sequence(input):
  for each NextSegment { unknown, known } from segment_iter(input):

    // Phase 1: unknown atoms — direct append, no overlap (PI-1 invariant)
    for atom in unknown:
      root.commit_state(BandState::Single { token: atom })

    // Phase 2: known atoms — expansion loop
    anchor = root.last_anchor()   // None if root is None
    cursor = 0
    atoms = known

    loop:
      if cursor >= atoms.len(): break

      remaining = atoms[cursor..]

      // Fast-path: single remaining token
      if remaining.len() == 1:
        root.commit_state(BandState::Single { token: remaining[0] })
        cursor += 1
        anchor = remaining[0]
        break

      // Fast-path: atom (no true postfixes)
      if remaining[0].is_atom() && anchor.map_or(true, |a| a.is_atom()):
        let token = remaining[0]
        root.commit_state(BandState::Single { token })
        cursor += 1
        anchor = token
        continue

      outcome = insert_next_match(graph, remaining)
      token = outcome.token()

      match find_overlap(anchor, token, remaining[token.width..]):
        Some((postfix, expansion, next_cursor)):
          band = BandState::with_overlap(token, postfix, expansion)
                           .with_complements_resolved(graph)
          root.commit_state(band)
          anchor = expansion   // ← expansion result is the new anchor (not bundled)
          cursor = next_cursor

        None:
          root.commit_state(BandState::Single { token })
          anchor = token
          cursor += token.width

      debug_assert!(cursor advanced)
```

`commit_state` dispatches to the correct case (A–F) based on:
- `root == None` → Case A
- `BandState::Single` + `root.is_some()` → Case B or F
- `BandState::WithOverlap` + root atom → Case C
- `BandState::WithOverlap` + root compound, `can_extend` → Case D
- `BandState::WithOverlap` + root compound, `!can_extend` → Case E

---

## Open Questions

> Fill in as the design is reviewed and implementation proceeds.

**OQ-1.** In Case D / Case E, extracting `prefix_of_root_pattern[..last]` and
building a new root token from it — does this require `graph.insert_pattern` or
can the root token be mutated in-place? Mutation is only valid if the root has no
parents and a single child pattern (`can_extend`). If `!can_extend`, a new token
must be created. Confirm whether an in-place replace-last-child operation exists
on `RootManager`.

**OQ-2.** In Case C, when the overlap prefix complement has zero width (atom root
of width 1 is the full postfix), the overlap pattern collapses to a single token.
Does `BandState::collapse()` handle the zero-width prefix case, or does it require
both complement slots to be non-empty?

**OQ-3.** Is `last_anchor()` on `RootManager` a new method that needs to be added,
or does it correspond to an existing method? The current `last_child_token` returns
the rightmost child — this is not the same as the anchor (which is the expansion
result from the last `WithOverlap` commit, per the Anchor Tracking section above).
A separate `anchor` field on `RootManager` (or carried through `ExpansionCtx`)
may be needed.

**OQ-4.** After Case D/E replaces the last child of the root pattern with bundled
token `B`, the old last-child token (e.g. `ab` in Case D's example) still exists
in the graph as a standalone token. It is no longer the root's last child. Does
any other structure hold a reference to it that needs updating, or is the graph's
reference-by-index model self-consistent here?

**OQ-5.** The algorithm sketch shows `anchor` being tracked separately from
`ExpansionCtx`'s `self.anchor`. Should `RootManager` own the anchor (since it
reflects committed state) or should `ExpansionCtx` own it (since it drives the
next iteration)? If owned by `ExpansionCtx`, `commit_state` must return the new
anchor to the caller. If owned by `RootManager`, `ExpansionCtx::next` must query
the root for the anchor before each step.