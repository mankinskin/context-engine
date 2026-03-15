---
tags: `#interview` `#expansion-loop` `#insert_sequence` `#context-api`
plan: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
batch: 6
topic: insert_sequence Outer Loop in context-api
status: 🟡 awaiting-answers
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

- **Answer:**

---

**Q27.** In the outer loop, when `insert_next_match` returns `NoExpansion { token,
width }`, the loop should advance by `width` and continue. But `insert_sequence`
is supposed to produce a *compound* root token that covers the entire input — not
just the first segment. After collecting all segment tokens `[T1, T2, ..., Tn]`,
the segments must be wrapped into a root. Should this wrapping call
`graph.insert_pattern(segments)` directly, or should it call `insert_next_match`
again with `[T1, T2, ..., Tn]` (where `Ti` are now compound tokens, not atoms)?

- **Answer:**

---

**Q28.** `insert_sequence` currently checks `text.chars().count() < 2` and returns
`QueryTooShort`. The outer loop handles single-atom remainders differently (option
(a) or (b) from Q8). Does the `< 2` guard still apply to the full input, or only
to the `insert_next_match` call inside the loop? What happens for `"aa"` — two
identical characters, each being a single-character token?

- **Answer:**

---

**Q29.** `already_existed` in `InsertResult` should be `true` iff the entire
compound root token was already in the graph before this call. In the outer loop,
different segments may be `Created`, `Complete`, or `NoExpansion`. What is the
correct logic for computing `already_existed` from a sequence of mixed outcomes?
Options:
  - All segments `Complete` → `true`
  - Any segment `Created` → `false`
  - Other combinations?

- **Answer:**

---

**Q30.** `insert_sequence` marks the workspace dirty only when `!already_existed`.
If the outer loop creates some new compound tokens (e.g. `T1` is `Created`) but
the final root wrapping token already existed (because this exact sequence was
inserted before), is `already_existed` for the root `true` or `false`? What does
the workspace-dirty flag mean in this case?

- **Answer:**

---

## Research Notes
<!-- Filled in after answers received -->

## Plan Impact
<!-- Changes to main plan driven by this batch -->