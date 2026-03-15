---
tags: `#design` `#RootManager` `#ExpansionCtx` `#commit` `#overlap` `#reading`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
created-from: 20260315_INTERVIEW_BATCH_5.md#Q22
status: ✅ resolved
---

# Design: Root Update Steps During Reading

> **Purpose:** Precisely and completely define every root update step that occurs
> during `ReadCtx::read_sequence`, so that `commit_state` / `append_collapsed` /
> `RootManager` can be correctly implemented for all structural cases.
>
> This document was created because Q22 in Batch 5 revealed that the interaction
> between `BandState::collapse()`, `commit_state`, and `RootManager` is subtle
> enough to warrant a dedicated design session before implementation begins.
>
> All five open questions (OQ-1 through OQ-5) have been answered and are resolved
> below. The document is now implementation-ready.

---

## Table of Contents

1. [Invariants](#invariants)
2. [Root States](#root-states)
3. [Update Operations](#update-operations)
4. [Postfix Iteration Model](#postfix-iteration-model)
5. [Case Catalogue](#case-catalogue)
   - [Case A: First token, root is None](#case-a-first-token-root-is-none)
   - [Case B: Sequential append, no overlap](#case-b-sequential-append-no-overlap)
   - [Case C: Overlap — root is the anchor token directly](#case-c-overlap--root-is-the-anchor-token-directly)
   - [Case D: Overlap — root is compound, can_extend, last child is anchor](#case-d-overlap--root-is-compound-can_extend-last-child-is-anchor)
   - [Case E: Overlap — root is compound, cannot extend, last child is anchor](#case-e-overlap--root-is-compound-cannot-extend-last-child-is-anchor)
   - [Case F: can_extend invalidated mid-loop — sequential append after overlap](#case-f-can_extend-invalidated-mid-loop--sequential-append-after-overlap)
6. [Anchor Tracking](#anchor-tracking)
7. [RootManager API](#rootmanager-api)
8. [Algorithm Sketch](#algorithm-sketch)
9. [Resolved Open Questions](#resolved-open-questions)

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
   This is preserved across Case D/E replacements because the bundled token `B`
   is an expansion that wraps the old last-child token — `B` covers everything
   the old token covered plus the overlap region. The old token is not deleted
   from the graph; it simply ceases to be the root's direct last child, but
   remains reachable through `B`'s own child patterns.
4. **No redundant decompositions:** only decompositions that are not derivable
   from another already-stored decomposition are written. The flat atom sequence
   is never stored alongside a tighter compound decomposition.
5. **Single source of overlap detection:** overlaps are found exclusively by
   `ExpansionCtx` via true postfix expansion. `append_collapsed` / `RootManager`
   must not contain independent overlap logic.
6. **Commit-before-next:** every `BandState` is committed to the root before the
   next `ExpansionCtx::next()` call. The graph is always up-to-date at the start
   of each `next()`.
7. **Anchor validity:** the anchor stored in `RootManager` is always either `None`
   (root not yet initialised) or the last token committed as a sequential token
   or the last overlap expansion result (`T2`). It is never the bundled token `B`,
   and never a stale value from a previous segment.

---

## Root States

At any point during reading, the root `Option<Token>` can be in one of these
structural states:

| State | Description | `can_extend`? |
|-------|-------------|---------------|
| `None` | No tokens committed yet | N/A — first commit creates root |
| `Atom` | Root is a single atom token (width 1, no parents) | Yes |
| `Compound-extendable` | Root is a compound token with exactly one child pattern and no parents | Yes |
| `Compound-shared` | Root has >1 child patterns or has parents | No — must wrap |

`can_extend` = `root.child_patterns().len() == 1 && root.parents().is_empty()`

**Key dynamic:** committing a `WithOverlap` state writes two patterns onto the
bundled token `B`. If `B` becomes the root (Case C) or is embedded as the root's
last child (Cases D/E), the root may immediately become `Compound-shared` if `B`
itself has two patterns. All subsequent commits in the same loop must re-check
`can_extend` — it must never be cached across iterations.

---

## Update Operations

There are four primitive operations on `RootManager`. The first three are the
same as before; Op-4 is new and required by Cases D/E.

### Op-1: `set_root(token)`
Used when `root` is `None`. Sets root to `token` directly.
- Pre-condition: `root == None`
- Post-condition: `root == Some(token)`, `anchor == token`

### Op-2: `extend_root(token)`
Used when `root` is `Compound-extendable`. Appends `token` to the root's single
child pattern in-place. No new graph token is created.
- Pre-condition: `root.is_some() && can_extend(root)`
- Post-condition: root's single pattern ends with `token`; coverage increased by
  `token.width`; `anchor == token`

### Op-3: `wrap_root(token)`
Used when `root` is `Compound-shared` or `Atom` and the next token does not
subsume the root. Creates a new compound token whose first pattern is
`[root, token]` via `graph.insert_pattern`.
- Pre-condition: `root.is_some() && !can_extend(root)`
- Post-condition: new wrapping token is set as root; coverage increased by
  `token.width`; `anchor == token`

### Op-4: `replace_last_child(bundled: Token)`
Used in Cases D/E when an overlap produces a bundled token `B` that subsumes the
root's current last child. The last child of the root's first pattern is replaced
by `B`.

Two sub-cases:

**Op-4a — in-place replace (`can_extend`):**
- Pre-condition: `can_extend(root)` — root has exactly one child pattern and no
  parents, so its pattern `Vec` is unshared and can be mutated.
- Steps: pop the last element of root's single child pattern; push `B`.
- Post-condition: root's pattern ends with `B`; coverage extended by
  `B.width - old_last_child.width`; `anchor == expansion_token (T2)`
- No new graph token is created for the root itself.

**Op-4b — rebuild root (`!can_extend`):**
- Pre-condition: `!can_extend(root)` — root has multiple patterns or has parents.
- Steps: take root's **first** child pattern, drop its last element, append `B`,
  call `graph.insert_pattern(rebuilt_pattern)` to create a new root token `R`.
  Set root to `R`.
- Post-condition: new root `R` covers the same atoms as old root plus `B`'s
  extension; `anchor == expansion_token (T2)`
- A new graph token is created for `R`.

**Invariant preserved by both sub-cases (OQ-4):** the old last-child token is not
deleted from the graph. `B` wraps it — `B` was created as an expansion token that
contains the old last-child in one of its own child patterns. Downward reachability
is therefore maintained: `root → B → old_last_child → ...` is always a valid path.
No back-reference surgery is needed on the old last-child token.

---

## Postfix Iteration Model

**Resolved from OQ-2:** a postfix is always a *true* postfix — it is strictly
shorter than the token it comes from. The anchor token itself is never a postfix
of itself. Therefore:

- An **atom** (single-character token) has no true postfixes and can never be an
  overlap anchor for postfix descent. The atom fast-path in `ExpansionCtx::next`
  (PI-14) correctly skips `find_overlap` entirely for atoms.
- A **newly-created compound token** with no parents has no postfixes either
  (the graph has not yet linked it as a child of anything). The fast-path via
  `postfix_iter()` returning empty is sufficient here; no separate guard is needed.
- When the root is a single token (before any extension), postfix iteration is
  not started. The first step of the expansion loop calls `insert_next_match` to
  find the first token, and only subsequent steps use the anchor for postfix search.

**Conceptual model for postfix iteration:** postfix iteration implements the rule
*"reduce the constraints on the right-side context by removing a prefix of the
anchor token."* Each smaller postfix is increasingly likely to be compatible with
the right-side context because it imposes fewer atoms on what must follow it. We
search largest-first and stop at the first qualifying expansion (result.width >
postfix.width).

This means:
- We never need a width guard comparing postfix width to `remaining.len()` — the
  postfix is structurally a subtoken of a previously expanded token and is
  guaranteed to fit within the query.
- The only early exit from postfix iteration is `remaining.is_empty()`.

---

## Case Catalogue

### Case A: First token, root is None

**Trigger:** `commit_state(BandState::Single { token })` when `root == None`.

**Steps:**
1. `Op-1: set_root(token)` — root becomes `Some(token)`.
2. `anchor = token` (stored in `RootManager`).

**Root after:** `Some(token)`. `can_extend` depends on `token`'s graph structure
(atom → yes; compound with one pattern and no parents → yes; otherwise → no).

---

### Case B: Sequential append, no overlap

**Trigger:** `commit_state(BandState::Single { token })` when `root.is_some()`.

**Steps:**
1. If `can_extend(root)`: `Op-2: extend_root(token)`.
2. Else: `Op-3: wrap_root(token)`.
3. `anchor = token`.

**Root after:** coverage extended by `token.width`.

---

### Case C: Overlap — root is the anchor token directly

**Trigger:** `commit_state(BandState::WithOverlap { anchor: T1, expansion: T2, ... })`
and `root == Some(T1)` — the root token *is* the left-side overlap token.

This occurs when the very first token in the known segment immediately participates
in an overlap (e.g. the graph already contains a compound token that starts with
the same postfix).

**Steps:**
1. `BandState::collapse(graph)` produces bundled token `B` covering `T1` + the
   non-overlapping suffix of `T2`, with two child patterns:
   - primary: `[T1, suffix_of_T2]`
   - overlap: `[T2]` — when T1's postfix equals all of T1 (a true postfix that
     is a strict subtoken), the leading complement is absent; the overlap pattern
     is just `[T2]` padded to cover the full width of `B` via `T2`'s own structure.
   All complement/padding tokens are resolved inside `find_overlap` before this
   call (PI-9). `collapse` only assembles already-resolved tokens.
2. `Op-1: set_root(B)` — `B` replaces the root entirely.
3. `anchor = T2` (the expansion result, not `B`).

**Root after:** `Some(B)`. Because `B` has two child patterns, `can_extend(B)`
is `false` — root is immediately `Compound-shared`. All subsequent appends use
Op-3 or Op-4b.

---

### Case D: Overlap — root is compound, `can_extend`, last child is anchor

**Trigger:** `commit_state(BandState::WithOverlap { anchor: T1, expansion: T2, ... })`
and `root.is_some()` and `can_extend(root)` and `root.last_child() == T1`.

Example: root = compound token `R` with single pattern `[X, T1]`, `can_extend`
holds. Overlap found: `T1` overlaps `T2` via true postfix `P`.

**Steps:**
1. `BandState::collapse(graph)` produces bundled token `B` (wrapping `T1 + T2`
   overlap region) with two patterns. All complement tokens already resolved.
2. `Op-4a: replace_last_child(B)` — root's single pattern becomes `[X, B]`
   in-place. Root token `R` itself is not recreated; only its pattern `Vec` is
   mutated. This is valid because `can_extend` guarantees the pattern is unshared.
3. `anchor = T2`.

**Root after:** root is still `R`, but `R`'s pattern is now `[X, B]`. Because `B`
has two child patterns, `can_extend(R)` is now re-evaluated: `R` still has only
one child pattern (`[X, B]`) and still has no parents — so `can_extend(R)` remains
`true` after this step, provided `R` has not acquired parents elsewhere.

---

### Case E: Overlap — root is compound, `!can_extend`, last child is anchor

**Trigger:** same as Case D but `!can_extend(root)`.

**Steps:**
1. `BandState::collapse(graph)` produces bundled token `B` as in Case D.
2. Extract root's **first** child pattern: `pat = root.first_child_pattern()`.
3. Build rebuilt pattern: `rebuilt = pat[..pat.len()-1] + [B]` (all children of
   the first pattern except the last, with `B` appended).
4. `graph.insert_pattern(rebuilt)` → new root token `R2`.
5. `Op-1: set_root(R2)`.
6. `anchor = T2`.

**Root after:** `Some(R2)`. `can_extend(R2)` = `R2.child_patterns().len() == 1 &&
R2.parents().is_empty()` — since `R2` was just created with one pattern and no
parents, it is initially extendable. However `R2` will become shared if its next
commit is another overlap.

**Reachability note:** old root token is not deleted. If any other structure held
a reference to the old root, it remains valid. The reading algorithm does not
patch up old root references — it only ever moves the `RootManager.root` pointer
forward.

---

### Case F: can_extend invalidated mid-loop — sequential append after overlap

**Trigger:** `commit_state(BandState::Single { token })` when `root.is_some()` and
`!can_extend(root)` — this occurs naturally after Case C or E produced a root that
immediately has `can_extend == false`, and the next step is a plain sequential token.

**Steps:** same as Case B, branch 2 — `Op-3: wrap_root(token)`.

This case is not structurally distinct from Case B; it is listed separately only
to make clear that `can_extend` must be checked at every `commit_state` call and
the loop never assumes it is stable between iterations.

---

## Anchor Tracking

**Resolved from OQ-3 and OQ-5:** the anchor is owned by `RootManager` as a
dedicated field `anchor: Option<Token>`. It is updated inside every `commit_state`
call. `ExpansionCtx::next` reads the anchor from `RootManager` at the start of
each step (via a method such as `root.anchor()`).

This ownership model is correct because the anchor reflects *committed* state —
it must be consistent with what has actually been written to the root and to the
graph. Keeping it in `ExpansionCtx` would create a risk of the anchor drifting out
of sync if a commit is interleaved with a non-`next()` call path.

| Committed state | New `RootManager.anchor` |
|-----------------|--------------------------|
| `BandState::Single { token }` | `Some(token)` |
| `BandState::WithOverlap { ..., expansion: T2 }` | `Some(T2)` — the expansion result, never the bundled token `B` |

**Why T2, not B:** `B` is freshly created and has no parents yet in the graph.
Its postfixes are empty — iterating them would always yield nothing, silently
skipping every subsequent overlap check. `T2` was found by `insert_next_match` as
the largest leftmost match in the current query window and may already have
parents (it is a known token). Its postfixes are the correct candidates for the
next overlap: they represent progressively looser right-side constraints, exactly
mirroring how the algorithm began with the first `insert_next_match` call.

**Initialisation:** at the start of the known-segment loop, `ExpansionCtx` reads
`root.anchor()` to set its initial anchor. If root is `None` (first read), the
anchor is `None` and the first step uses `insert_next_match` without a postfix
check. If root already has a value (from a prior unknown segment), the anchor is
the last token committed there.

---

## RootManager API

The following methods must exist on `RootManager` after the redesign. Methods
marked **new** do not currently exist and must be added. Methods marked
**modified** exist but need updating. Methods marked **delete** must be removed.

| Method | Status | Notes |
|--------|--------|-------|
| `set_root(token)` | new (or rename of existing init path) | Op-1 |
| `extend_root(token)` | new (or rename of `append_to_owned_pattern`) | Op-2 |
| `wrap_root(token)` | new (or rename of multi-token `append_collapsed` path) | Op-3 |
| `replace_last_child(bundled: Token)` | **new** | Op-4a/4b; dispatches on `can_extend` internally |
| `commit_state(state: BandState)` | modified | Dispatches to Op-1 through Op-4; updates `anchor` |
| `anchor() -> Option<Token>` | **new** | Read-only accessor for `ExpansionCtx` |
| `can_extend() -> bool` | existing (confirm) | Must not be cached by callers |
| `append_collapsed` overlap branches | **delete** | Replaced entirely by `ExpansionCtx` |

---

## Algorithm Sketch

Revised to incorporate all resolved open questions:

```
RootManager {
    root:   Option<Token>,
    anchor: Option<Token>,   // OQ-3/OQ-5: owned here, not in ExpansionCtx
}

impl RootManager {
    fn commit_state(&mut self, state: BandState, graph: &mut Graph) {
        match state {
            BandState::Single { token } => {
                match self.root {
                    None       => self.set_root(token),         // Case A
                    Some(_) if self.can_extend()
                               => self.extend_root(token),      // Case B
                    Some(_)    => self.wrap_root(token, graph), // Case B / Case F
                }
                self.anchor = Some(token);
            }

            BandState::WithOverlap { t1, t2, bundled: B, .. } => {
                // collapse() already called; B exists in graph with two patterns.
                match self.root {
                    None => {
                        self.set_root(B);                       // Case C (root was None)
                    }
                    Some(root) if root == t1 => {
                        self.set_root(B);                       // Case C (root IS t1)
                    }
                    Some(_) => {
                        // Cases D / E: replace last child of root with B
                        self.replace_last_child(B, graph);      // Op-4a or Op-4b
                    }
                }
                self.anchor = Some(t2);  // OQ-5: anchor = T2, not B
            }
        }
    }

    fn replace_last_child(&mut self, bundled: Token, graph: &mut Graph) {
        if self.can_extend() {
            // Op-4a: mutate root's single pattern in-place
            root_pattern.pop();
            root_pattern.push(bundled);
            // root token identity unchanged
        } else {
            // Op-4b: rebuild root from first pattern
            let mut pat = self.root.first_child_pattern().clone();
            pat.pop();
            pat.push(bundled);
            let new_root = graph.insert_pattern(pat);
            self.root = Some(new_root);
        }
    }
}
```

`ExpansionCtx::next` (revised anchor read):

```
fn next(&mut self) -> Option<BandState> {
    if self.ensure_remaining() == 0 { return None; }

    let remaining = self.remaining_slice();
    let anchor    = self.root.anchor();   // OQ-3: read from RootManager

    // Fast-path: atom anchor has no true postfixes (OQ-2)
    // Fast-path: single remaining token
    if remaining.len() == 1 || anchor.map_or(false, |a| a.is_atom()) {
        let token = remaining[0];
        self.cursor += 1;
        return Some(BandState::Single { token });
        // commit_state will update anchor
    }

    let outcome = insert_next_match(graph, remaining);
    let t1 = outcome.token();

    match find_overlap(anchor, t1, &remaining[t1.width..], graph) {
        Some((postfix, t2, next_cursor)) => {
            let band = BandState::with_overlap(t1, postfix, t2)
                           .resolve_complements(graph); // PI-9
            self.cursor = next_cursor;
            Some(band)
            // commit_state will set anchor = t2
        }
        None => {
            self.cursor += t1.width;
            Some(BandState::Single { token: t1 })
            // commit_state will set anchor = t1
        }
    }
}
```

---

## Resolved Open Questions

### OQ-1 — In-place mutation vs. new token for root replacement (Cases D/E)

**Answer:** In-place mutation of the root's pattern `Vec` is valid **only** when
`can_extend(root)` — the root has exactly one child pattern and no parents, so
the pattern is unshared. This is Op-4a. When `!can_extend`, a new token must be
created from the rebuilt pattern via `graph.insert_pattern`. This is Op-4b. The
`replace_last_child` primitive on `RootManager` dispatches between the two
sub-cases internally. Callers do not need to check `can_extend` before calling
`replace_last_child` — the dispatch is encapsulated.

**Status:** ✅ Resolved. Op-4 added to the Update Operations section.

---

### OQ-2 — Zero-width prefix complement in Case C

**Answer:** This case **cannot occur**. A postfix is always a *true* postfix —
strictly shorter than the token it comes from. An atom (width 1) has no true
postfixes. Therefore the root being a single atom means postfix iteration is never
started in the first place (the atom fast-path in `ExpansionCtx::next` skips
`find_overlap` entirely). The "zero-width prefix complement" scenario cannot arise
because the anchor atom is never passed to `find_overlap`.

**Consequence:** Case C in the catalogue is reached only when the root is a
compound token whose last child (the anchor) has at least one true postfix. The
bundled token `B` produced by `collapse()` always has a non-empty leading
complement on the overlap pattern (because the postfix used is strictly shorter
than `T1`, leaving at least one atom as the prefix complement).

`BandState::collapse()` does not need to handle the zero-width complement case.

**Status:** ✅ Resolved. Postfix Iteration Model section added to document this.

---

### OQ-3 — `last_anchor()` method and where it lives

**Answer:** The anchor is a new dedicated field `anchor: Option<Token>` on
`RootManager`. It is updated inside `commit_state` — no separate method call is
needed by callers. `ExpansionCtx::next` reads it via a new `root.anchor()` method.
The existing `last_child_token()` method returns the rightmost child of the root's
first pattern, which is a different concept and must not be confused with the
anchor. `last_child_token()` may remain for other uses but is not the anchor.

**Status:** ✅ Resolved. `anchor()` added to RootManager API table.

---

### OQ-4 — Stale reference to old last-child token after Case D/E

**Answer:** No stale-reference problem exists. The old last-child token (e.g. `ab`)
continues to live in the graph as a valid standalone token. The bundled token `B`
was created as an expansion that wraps `ab` — `B`'s own child patterns reference
`ab`. The reachability invariant is therefore maintained: `root → B → ab → ...`
is always a valid downward path. No back-reference surgery is needed. The graph's
reference-by-index model is self-consistent: indices are stable, and the old
token's index is preserved inside `B`.

**Status:** ✅ Resolved. Invariant 3 updated and Op-4 notes explain this.

---

### OQ-5 — Anchor ownership: `RootManager` vs. `ExpansionCtx`

**Answer:** The anchor is owned by `RootManager`. It is the committed anchor —
the last token that was successfully integrated into the root structure. Because
it reflects committed graph state, it belongs with the committed state manager
(`RootManager`), not with the working-memory iterator (`ExpansionCtx`).
`ExpansionCtx::next` queries it at the start of each step; `commit_state` updates
it at the end of each step. This eliminates any risk of the anchor drifting out
of sync with the root between `next()` and `commit_state()` calls.

**Status:** ✅ Resolved. RootManager API and Anchor Tracking sections updated.