---
tags: `#interview` `#expansion-loop` `#insert_sequence` `#context-api`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
batch: 6
topic: insert_sequence Outer Loop in context-api
status: ✅ answered
---

# Interview — Batch 6: insert_sequence Outer Loop in context-api

> These questions establish how RC-1 is fixed and how the outer loop in
> `insert_sequence` relates to the inner loop in `ExpansionCtx`.

---

**Q26.** `insert_sequence` in `context-api` calls `insert_next_match` once. The
proposed fix adds an outer loop. Should this outer loop be:
  - (a) Inlined directly in `WorkspaceManager::insert_sequence` in
        `commands/insert.rs`.
  - (b) Extracted as a shared helper (e.g. `insert_full_sequence`) in
        `context-insert` so that both `insert_sequence` and `ReadCtx` can use it.
  - (c) Delegated entirely to `ReadCtx` — i.e. `insert_sequence` calls
        `ReadCtx::read_sequence` internally rather than maintaining its own loop.

What are the correctness and coupling trade-offs of each option?

- **Answer:** Option **(c)** — delegate entirely to `ReadCtx::read_sequence`.
  `ReadCtx::read_sequence` is the canonical entry point for consuming an entire
  sequence and creating all necessary tokens. All other references to the read
  algorithm — including `insert_sequence` in `context-api` — should be thin
  consumers of this call. No separate loop logic should be maintained in
  `context-api` or `context-insert`.

---

**Q27.** In the outer loop, when `insert_next_match` returns `NoExpansion { token,
width }`, the loop should advance by `width` and continue. But `insert_sequence`
is supposed to produce a *compound* root token that covers the entire input — not
just the first segment. After collecting all segment tokens `[T1, T2, ..., Tn]`,
the segments must be wrapped into a root. Should this wrapping call
`graph.insert_pattern(segments)` directly, or should it call `insert_next_match`
again with `[T1, T2, ..., Tn]` (where `Ti` are now compound tokens, not atoms)?

- **Answer:** There is no segment token list at the end. Segments are committed
  directly into the root as they are produced — each `BandState` is committed to
  the `RootManager` immediately after it is yielded (per PI-5). By the time the
  loop terminates, the root already represents the full sequence. No wrapping step
  is required.

---

**Q28.** `insert_sequence` currently checks `text.chars().count() < 2` and returns
`QueryTooShort`. The outer loop handles single-atom remainders differently (option
(a) or (b) from Q8). Does the `< 2` guard still apply to the full input, or only
to the `insert_next_match` call inside the loop? What happens for `"aa"` — two
identical characters, each being a single-character token?

- **Answer:** The `< 2` input guard should be **removed**. A single-token input
  is handled trivially: return the one token as the response with no graph updates.
  The loop naturally handles this without a pre-check. For `"aa"`, both `a` tokens
  are atoms and each is committed in sequence; the result is a two-atom root
  `[a, a]`.

---

**Q29.** `already_existed` in `InsertResult` should be `true` iff the entire
compound root token was already in the graph before this call. In the outer loop,
different segments may be `Created`, `Complete`, or `NoExpansion`. What is the
correct logic for computing `already_existed` from a sequence of mixed outcomes?
Options:
  - All segments `Complete` → `true`
  - Any segment `Created` → `false`
  - Other combinations?

- **Answer:** `already_existed` applies only to a **single outcome** — it is `true`
  when the outcome is not `Created`. During postfix expansion the relevant signal
  is `has_expanded`, which is `true` when the outcome is not `NoExpansion`. These
  two flags are orthogonal and should not be conflated into a single aggregate
  flag across the whole sequence.

---

**Q30.** `insert_sequence` marks the workspace dirty only when `!already_existed`.
If the outer loop creates some new compound tokens (e.g. `T1` is `Created`) but
the final root wrapping token already existed (because this exact sequence was
inserted before), is `already_existed` for the root `true` or `false`? What does
the workspace-dirty flag mean in this case?

- **Answer:** This case **cannot happen**. The algorithm always searches for the
  largest existing tokens, and any token found via `insert_next_match` has no
  parents that still match the surrounding query. The root wrapping step always
  produces a new token — one that did not exist before — because the combination
  of segment tokens has not been seen. Therefore `already_existed` for the root is
  always `false` when a wrapping step occurs, and the workspace is always marked
  dirty in that case.

---

## Research Notes

### R17 — `ReadCtx::read_sequence` is the single entry point (Q26)

A26 settles the RC-1 fix architecture decisively. `insert_sequence` in
`context-api` must call `ReadCtx::read_sequence`. The consequence is:

- The RC-1 fix is **not** a change to `context-api` logic at all — it is the
  completion of `ReadCtx::read_sequence` itself (the RC-2/RC-3 fix).
- Once `ReadCtx::read_sequence` correctly drives `ExpansionCtx` in a cursor loop,
  `insert_sequence` automatically inherits correct behaviour.
- The RC-1 / RC-2 / RC-3 labels collapse into a single fix: implement the
  `ExpansionCtx` cursor loop inside `ReadCtx`.

### R18 — No wrapping step: root is built incrementally (Q27)

A27 confirms the incremental commit model established in PI-5. The root is never
assembled from a collected list of segments — it is extended with each commit.
This means `RootManager` must correctly handle every possible transition:

- `None → first token` (fresh root)
- `token → [token, next_token]` (sequential append)
- `token → bundled_overlap_token` (overlap replace)
- `[..., last] → [..., new_wrapper]` (overlap extending the tail)

All four transitions must be exercised in tests before the algorithm work begins
(feeds into the dedicated root-update design session called for in Q22).

### R19 — `already_existed` and `has_expanded` are per-outcome, not aggregate (Q29)

A29 separates two orthogonal signals that the original plan conflated:

| Signal | Scope | Meaning |
|--------|-------|---------|
| `already_existed` | single outcome | `true` iff outcome ≠ `Created` |
| `has_expanded` | single outcome | `true` iff outcome ≠ `NoExpansion` |

The workspace-dirty flag is driven by whether the **root token** is new — which
per A30 is always the case when a multi-segment sequence is produced. Single-token
inputs return the existing token directly and do not mark dirty.

### R20 — Root wrapping always creates a new token (Q30)

A30 provides a structural proof: because `insert_next_match` always finds the
**largest** existing token at each cursor position, the found token has no
remaining parents that cover the broader query. The composed root is therefore
always a new token combination. The "already existed root" scenario is impossible
under the algorithm's invariants.

---

## Plan Impact

### PI-16 — RC-1 fix is subsumed by RC-2/RC-3 fix

Update the implementation plan:

- Remove `context-api/src/commands/insert.rs` as the site of the RC-1 fix.
- The RC-1 fix is the completion of `ReadCtx::read_sequence`. Once the
  `ExpansionCtx` cursor loop is correct, `insert_sequence` calls
  `ReadCtx::read_sequence` and inherits the correct behaviour automatically.
- `commands/insert.rs` change becomes: replace the `insert_next_match` call with
  a `ReadCtx::read_sequence` call. One-line change.

### PI-17 — Remove `< 2` guard from `insert_sequence`

Remove the `text.chars().count() < 2` early return. Replace with: if the input
resolves to a single token, return it directly with `already_existed = true` and
no graph writes.

### PI-18 — `already_existed` and `has_expanded` must be tracked separately

The `InsertOutcome` type (or its consumer) must expose both signals cleanly:
- `outcome.already_existed()` → `outcome != Created`
- `outcome.has_expanded()` → `outcome != NoExpansion`

Do not aggregate these into a sequence-level flag. Each step's outcome is
evaluated independently.