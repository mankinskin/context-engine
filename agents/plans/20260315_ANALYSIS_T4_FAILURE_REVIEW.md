---
tags: `#analysis` `#context-read` `#algorithm` `#expansion` `#overlap` `#testing`
summary: Detailed analysis of the 11 remaining context-read unit test failures at T4 completion checkpoint. Classifies each failure, identifies whether the issue is in the test expectation or the implementation, and proposes a resolution path.
status: 📋 ready-for-review
parent: 20260315_PLAN_EXPANSION_LOOP_REDESIGN.md
---

# T4 Failure Review — Remaining 11 `context-read` Unit Test Failures

**Date:** 2026-03-15
**Scope:** Analysis only — no code changes proposed here
**Test suite:** `crates/context-read` unit tests
**Baseline:** 64 pass / 11 fail / 0 ignored

---

## Table of Contents

1. [How to Read This Document](#how-to-read-this-document)
2. [Failure Taxonomy](#failure-taxonomy)
3. [Group A — Missing second decomposition pattern (overlap not committed)](#group-a--missing-second-decomposition-pattern)
4. [Group B — Spurious intermediate tokens created during overlap commit](#group-b--spurious-intermediate-tokens)
5. [Group C — Test expectation questionable / possibly wrong](#group-c--test-expectation-questionable)
6. [Group D — Second-read graph state pollution](#group-d--second-read-graph-state-pollution)
7. [Resolution Road Map](#resolution-road-map)
8. [Open Questions](#open-questions)

---

## How to Read This Document

Each failure entry records:

- **Actual output** — what the code produces today.
- **Expected output** — what the test asserts.
- **Failure mechanism** — the precise assertion that fires and why.
- **Verdict** — one of:
  - `IMPL BUG` — the implementation is wrong; test expectation is correct.
  - `TEST WRONG` — the test expectation is wrong; implementation may be correct.
  - `TEST WRONG (ngrams oracle)` — ngrams oracle produces a different result; requires investigation of whether the oracle or the test is authoritative for that case.
  - `AMBIGUOUS` — both sides need review before a verdict can be issued.

---

## Failure Taxonomy

After examining all 11 failures, they fall into four distinct groups:

| Group | Root Symptom | Failure Mechanism | Tests |
|-------|-------------|-------------------|-------|
| A | Missing second overlap pattern on a token | `assert_patterns!` fires because ngrams expects `[[X, Y], [Z, W]]` but implementation stores only `[[X, Y]]` | `validate_three_repeated`, `validate_triple_repeat`, `repetition_abcabcabc`, `repetition_xyzxyzxyz`, `read_infix1`, `read_infix2`, `read_multiple_overlaps1`, `complex_abcabababcaba` |
| B | Spurious intermediate compound token present | `assert_indices!` fires via `expect_complete` → `expect_entire_root` panic: "Expected EntireRoot path. Got PathCoverage::Postfix" — meaning a shorter token exists that is a postfix of a longer one | `repetition_aabbaabb` |
| C | ngrams oracle expects flat atom pattern; context-read builds a structured one | `validate_mixed_pattern`: ngrams produces `aabb → [[a,a,b,b]]`; context-read produces `aabb → [[aab,b]]` | `validate_mixed_pattern` |
| D | Second read pollutes first-read token structure | `assert_not_indices!` / `expect_complete` fires because a token that should not exist yet (`held`) is created as a side-effect of the second read | `sync_read_text2` |

---

## Group A — Missing Second Decomposition Pattern

### Overview

The canonical overlap commit should produce a token with **two** child patterns:
the *sequential* decomposition (primary band) and the *overlap* decomposition
(overlap band). The current `build_overlap_state` / `collapse()` path creates the
bundled token but only one of the two patterns is being stored correctly. As a
result, every token that should have been bundled with a `WithOverlap` state is
present in the graph with only one pattern — the overlap band pattern or the
primary band pattern, but not both.

### Failure: `validate_three_repeated` — input `"aaa"`

**Actual (context-read):**
```
"aaa" → { ["aa", "a"] }
```

**Expected (ngrams oracle):**
```
"aaa" → { ["a", "aa"], ["aa", "a"] }
```

**Verdict:** `IMPL BUG`

The pattern `["aa", "a"]` is present — that is the sequential/primary decomposition
(`aa` found first, then `a`). The pattern `["a", "aa"]` is absent — that is the
overlap decomposition where `a` is the left context and `aa` is the expansion
result starting at position 1. The `WithOverlap` state for `"aaa"` is being either
not generated at all, or generated but only the primary band is recorded after
`collapse()`.

The ngrams result is definitively correct for `"aaa"`: both `a+aa` and `aa+a` are
valid tightest decompositions (they use the largest available token at each
starting position). The implementation must produce both.

---

### Failure: `validate_triple_repeat` — input `"ababab"`

**Actual (context-read):**
```
"ababab" → { ["abab", "ab"] }
```

**Expected (ngrams oracle):**
```
"ababab" → { ["ab", "abab"], ["abab", "ab"] }
```

**Verdict:** `IMPL BUG`

Identical structure to `"aaa"`. The primary decomposition `["abab", "ab"]` is
stored. The overlap decomposition `["ab", "abab"]` is missing.

---

### Failure: `repetition_abcabcabc` — input `"abcabcabc"`

**Actual:**
```
root → { Pattern(["abcabc"(4), "abc"(3)]) }
```

**Expected:**
```
root → { Pattern(["abc"(3), "abcabc"(4)]), Pattern(["abcabc"(4), "abc"(3)]) }
```

**Verdict:** `IMPL BUG`

The `["abcabc", "abc"]` sequential pattern is present. The `["abc", "abcabc"]`
overlap pattern is absent. Same root cause as Group A above.

---

### Failure: `repetition_xyzxyzxyz` — input `"xyzxyzxyz"`

**Actual:** (by analogy — same structure, different atoms)
```
root → { Pattern(["xyzxyz", "xyz"]) }
```

**Expected:**
```
root → { Pattern(["xyz", "xyzxyz"]), Pattern(["xyzxyz", "xyz"]) }
```

**Verdict:** `IMPL BUG` — identical to `repetition_abcabcabc`.

---

### Failure: `read_infix1` — `"subdivision"` then `"visualization"`

**Failure message:**
```
su: Expected EntireRoot path. Got PathCoverage::Postfix
```

The panic fires at the `assert_indices!(graph, su)` call inside `read_infix1`.
`assert_indices!` calls `find_ancestor("su")` then `.expect_complete("su")`. The
`expect_complete` calls `expect_entire_root` which panics if the search result
path coverage is `Postfix` rather than `EntireRoot`.

A `Postfix` coverage means the search for `"su"` matched only the trailing portion
of some larger token, not the entirety of a standalone `"su"` token. This implies
`"su"` does not exist as an independent vertex — or it exists embedded inside
`"visu"` or `"subdivision"` without being promoted to its own root-level token.

**Expected:** After reading `"visualization"` following `"subdivision"`, the token
`su` should exist as a standalone vertex because `sub` (from `subdivision`) ends
with `su` and `visu` (from `visualization`) ends with `su`, creating the overlap
that materialises `su` as a distinct token.

**Verdict:** `IMPL BUG` — the overlap detection / `collapse()` failed to materialise
`su` as a standalone token when processing the second sequence. The missing overlap
pattern for a lower-level token cascades into `su` not being registered.

The full expected state after both reads is documented in the test:
```
su => [[s, u]],
vi => [[v, i]],
vis => [[vi, s]],
visu => [[vis, u], [vi, su]],
ion => [[i, o, n]],
visualization => [[visu, a, l, i, z, a, t, ion]],
subdivision => [[su, b, d, i, vis, ion]]
```

Both `visu` and `subdivision` require `su` as a child. The fact that `assert_indices!`
fails on `su` means `collapse()` is not inserting the complement/overlap tokens
into the graph as standalone addressable vertices.

---

### Failure: `read_infix2` — `"subvisu"` then `"visub"`

**Failure message:**
```
vi: Expected EntireRoot path. Got PathCoverage::Prefix
```

Same mechanism as `read_infix1` but for `vi`. After reading `"visub"` (which
overlaps with `"subvisu"` through the shared `visu` / `sub` components), `vi`
should exist as a standalone vertex. The `Prefix` coverage variant here means
the search matched only the leading portion of some larger token — `vi` is a
prefix of `visu` but not yet materialised as its own root.

**Verdict:** `IMPL BUG` — same root cause as `read_infix1`.

---

### Failure: `read_multiple_overlaps1` — progressive 5-sequence overlap build

**Failure message:**
```
ab: Expected EntireRoot path. Got PathCoverage::Postfix
```

The test reads five sequences (`abcde`, `bcdea`, `cdeab`, `deabc`, `eabcd`) then
`abcdeabcde`. The panic fires at the `assert_indices!(graph, de, dea, bc, deab, abc)`
line (when looking up `ab`). This means `ab` has not been materialised as a
standalone vertex by the time the fourth sequence `deabc` is processed, even though
`deabc` requires `ab` to exist (expected: `deabc => [[de, abc], [dea, bc], [deab, c]]`
and `abc => [[ab, c], [a, bc]]`).

**Verdict:** `IMPL BUG` — same root cause: `collapse()` for the overlap producing
`abc` did not insert `ab` as an independent vertex. The pattern `abc → [[ab, c]]`
requires `ab` to exist; `ab` is the prefix complement of the overlap that produces
`abc`, and complement tokens must be fully resolved before `collapse()` is called
(PI-9).

---

### Failure: `complex_abcabababcaba` — input `"abcabababcaba"`

**Failure message:**
```
aba: Expected EntireRoot path. Got PathCoverage::Postfix
```

`aba` should be created as part of the overlap `abcaba → { [ab, caba], [abc, aba] }`.
The fact it returns `Postfix` coverage means `aba` is present only as a sub-token
of something larger, not as a root-level vertex. Same root cause as all Group A
failures: complement tokens are not being committed as standalone vertices.

**Verdict:** `IMPL BUG`

---

### Group A Summary — Root Cause

All eight Group A failures share a single root cause with two aspects:

**A1 — `WithOverlap` state produces only one pattern, not two.**
After `collapse()`, the bundled token is inserted via `graph.insert_patterns(vec![primary, overlap])`.
The two-pattern insert should create a vertex with both decompositions. If the
patterns are equivalent (e.g. due to a complement being empty or zero-width), or
if `insert_patterns` deduplicates them, one pattern disappears. However the
ngrams oracle confirms they should be distinct.

**A2 — Complement tokens are not materialised as standalone vertices.**
`build_prefix_complement` / `build_postfix_complement` in `chain/mod.rs` call
`graph.insert_init` or build the complement from the band's token list, but the
resulting token may not be independently searchable by `find_ancestor` if it is
only registered as a child pattern element without its own top-level entry. PI-9
states: *"all complement tokens must exist in the graph as valid graph indices
before `collapse()` is called"* — if they are created inside `collapse()` rather
than before, the ordering guarantee is violated and the anchor for the next step
may be stale.

---

## Group B — Spurious Intermediate Tokens

### Failure: `repetition_aabbaabb` — input `"aabbaabb"`

**Expected tokens to exist:** `aa`, `bb`, `aabb` (via `assert_indices!`)
**Expected tokens NOT to exist:** (none explicitly negated, but `aab` must not be
an independent token with a `Postfix` path coverage for `bb`)

**Failure message:**
```
bb: Expected EntireRoot path. Got PathCoverage::Postfix:
    parent: "aabb"(4)
        parent: "aab"(3)
```

`assert_indices!(graph, aa, bb, aabb)` is looking up `bb`. The search finds `bb`
only as a postfix of `"aab"` inside `"aabb"` — meaning `bb` as a standalone vertex
does not exist yet, but `aab` does exist.

The intermediate token `aab` (= `[aa, b]`) should not exist at this stage — or if
it does, `bb` must also exist as its own vertex. The graph currently contains:
```
"aab"(3): { ["aa", "b"] }
"aabb"(4): { ["aab", "b"] }
```

The correct graph should be:
```
"aa"   → [[a, a]]
"bb"   → [[b, b]]
"aabb" → [[aa, bb]]
root   → [[aabb, aabb]]
```

**Verdict:** `IMPL BUG`

The algorithm is processing `"aabbaabb"` and building `aab` and then `aabb` as
`[aab, b]` instead of recognising that `aa` and `bb` should be materialised first
and `aabb = [aa, bb]`. This is a cursor-advancement and overlap-detection ordering
issue: when reading `"aabb"` for the first time, the correct behaviour is to read
`aa` (width 2, from atom fast-path since `a` is known after the first segment),
then `bb` (width 2), then commit `aabb = [aa, bb]`. Instead the algorithm is
reading `aab` (width 3) on the first step — `aab` is a valid three-atom token but
it should not exist because the correct tightest decomposition never has a compound
with an `[aa, b]` structure for `"aabb"`.

The likely cause: `insert_next_match` on the remaining `[a, a, b, b]` slice returns
`aab` (width 3) as the longest match when `aa` already exists as a parent of `aab`
from the first half of `"aabbaabb"`. This is a greedy-match issue where an earlier
committed overlap token bleeds into the next step's anchor context. The `insert_next_match`
result for `[a,a,b,b]` with `aa` already in the graph should return `aa` (width 2)
if `aab` has not been created yet — but `aab` gets created as a side-effect of the
overlap processing of the first `aabb`.

This may indicate that the first `aabb` is being committed incorrectly (creating
`aab` as an intermediate when it should not), or that the second `aabb` read
encounters a polluted graph state from the first.

---

## Group C — Test Expectation Questionable

### Failure: `validate_mixed_pattern` — input `"aabb"`

**Actual (context-read):**
```
"aa"   → { ["a", "a"] }
"aab"  → { ["aa", "b"] }
"aabb" → { ["aab", "b"] }
```

**Expected (ngrams oracle):**
```
"aabb" → { ["a", "a", "b", "b"] }
```

(ngrams does not produce `aa`, `aab` at all.)

**Verdict:** `TEST WRONG (ngrams oracle)` — **the ngrams oracle result is
incorrect for this input, not the context-read output.**

The ngrams algorithm is described as a naive reference implementation. For `"aabb"`
it produces only the raw atom sequence `[a, a, b, b]` because `"aabb"` has no
repeated substrings of length ≥ 2 that appear more than once in the input. The
ngrams algorithm is a sliding-window n-gram counter — it only creates compound
tokens for substrings that appear at least twice. Since `aa`, `ab`, `bb`, `aab`,
`abb` each appear exactly once in `"aabb"`, ngrams does not create any compound
token.

However, context-read's contract is different: it creates `aa` because `a` is a
known atom after the first `a` is inserted, and `[a, a]` is the tightest
decomposition of the two-atom sequence at that point. Similarly `aab` and `aabb`
are built incrementally.

**The correct expected output for context-read on `"aabb"` is:**
```
"aa"   → { ["a", "a"] }
"aab"  → { ["aa", "b"] }   ← OR: is this a spurious intermediate?
"aabb" → { ["aab", "b"] }  ← OR: should it be { ["aa", "bb"] }?
```

This is ambiguous. Two sub-questions arise:

1. **Should `bb` be materialised when reading `"aabb"`?**  `b` is a known atom
   after the first `b` is inserted. When the cursor reaches the second `b`, `bb`
   should be the largest match starting there (since `b` is known but no `bb` or
   `bbb` token exists yet). So the second pass through `b, b` should yield `bb →
   [[b,b]]` and the full result should be `aabb → [[aa, bb]]`. The fact that the
   implementation produces `aab → [[aa, b]]` instead suggests the same Group B
   cursor-advancement bug applies here.

2. **Is the ngrams oracle comparison valid for `"aabb"`?** No — the ngrams oracle
   only records tokens that appear multiple times. The ngrams validation test's
   contract (`ngrams_patterns.is_subset(cr_patterns)`) does hold: the atom-only
   pattern `["a","a","b","b"]` should be a subset of context-read's patterns for
   `aabb`... but context-read does not produce that flat atom pattern at all
   because `aa` and `aab` get created first.

**Resolution:** This test should be **skipped or rewritten** to use a context-read-
specific oracle rather than the ngrams oracle. The ngrams oracle is not a valid
reference for inputs with no repeated substrings. The failing assertion
(`"aabb" missing patterns: [["a","a","b","b"]]`) is asking context-read to store
a flat atom-level pattern on `aabb`, which is not part of context-read's contract
(it always uses the tightest compound decomposition available).

**Note:** The Group B and Group C failures are related — if the Group B cursor bug
is fixed and `"aabb"` correctly produces `aabb → [[aa, bb]]`, the ngrams oracle
comparison still fails because ngrams expects the flat `[a,a,b,b]` pattern which
context-read will never produce. The ngrams oracle is simply not applicable to
inputs with no repeated substrings.

---

## Group D — Second-Read Graph State Pollution

### Failure: `sync_read_text2` — `"heldld"` then `"hell"`

**Test structure:**
1. Read `"heldld"` → asserts `ld → [[l,d]]`, `heldld → [[h,e,ld,ld]]`; asserts
   `held`, `he`, `hel` do NOT exist yet.
2. Read `"hell"` → asserts `he`, `hel`, `held` now exist.

**Failure message:**
```
Expected incomplete or error for held, but got complete match
```

This fires at line 47, which is the `assert_not_indices!(graph, held, he, hel)`
assertion after the first read. The `assert_not_indices!` macro (not shown in detail,
but symmetric to `assert_indices!`) calls `find_ancestor` on `"held"` and expects
an `Err` or incomplete result. Instead `"held"` already exists after reading
`"heldld"`.

**Actual behaviour:** Reading `"heldld"` creates `held` as a side-effect, even
though the test asserts it should not exist until `"hell"` is read.

**Expected behaviour:** `"heldld"` = `h + e + ld + ld`. The `ld` pattern is shared
(atom `l` followed by atom `d`). The tightest decomposition is `[h, e, ld, ld]` —
no `held` token should be created during this read. `held` should only appear when
`"hell"` is read and its overlap with `heldld` triggers the `[hel, d]` and `[he, ld]`
decompositions.

**Verdict:** `IMPL BUG`

The algorithm is creating `held` as a side-effect of reading `"heldld"`. Possible
causes:
1. The overlap detection in `ExpansionCtx` / `find_overlap` is finding a spurious
   overlap between `ld` (anchor after committing `[h, e, ld]`) and the second `ld`
   via some path that creates `held` as a complement token.
2. `build_overlap_state` or `build_prefix_complement` is calling
   `insert_next_match` on a sub-slice that includes `h,e,l` → creating `hel` and
   then `held` prematurely.
3. The `RootManager::replace_last_child` Op-4b path is creating a new root pattern
   that wraps the first `ld` together with preceding atoms, accidentally materialising
   `held`.

This failure is distinct from Group A (which is about missing patterns) — here the
problem is an *extra* token created too early, which breaks the `assert_not_indices!`
guard that enforces the invariant "unknown atoms cannot be part of pre-existing
compound tokens."

---

## Resolution Road Map

### Step R0 — Fix the ngrams oracle test for `validate_mixed_pattern` (Group C)

**Action:** Add a guard to `validate_graphs_equivalent` that skips the ngrams
comparison when the input has no repeated substrings (i.e., every n-gram of length
≥ 2 appears exactly once). Alternatively, mark `validate_mixed_pattern` as
`#[ignore = "ngrams oracle not applicable to inputs with no repeated substrings"]`
with a comment explaining the contract difference.

**Impact:** 1 test removed from the failing list. No implementation change needed.

**Risk:** Low.

---

### Step R1 — Understand and fix `repetition_aabbaabb` / Group B (spurious `aab`)

**Before fixing:** Add a targeted debug test for just `"aabb"` that prints the
exact sequence of `BandState` values yielded by `ExpansionCtx` and what
`insert_next_match` returns at each cursor position. Specifically:

- What does `insert_next_match([a, a, b, b])` return? It should return `aa` (width 2)
  if `aa` is already in the graph and no `aab` or `aabd` token exists yet.
- If it returns `aab` (width 3), trace why — has `aab` been created already as a
  side-effect of the first half of `"aabbaabb"`, and if so, is that creation correct?

**Hypothesis:** The first half `"aabb"` creates `aa → [[a,a]]` then tries to find
the next token at `[b, b]`. At this point `b` is known (just inserted) but `bb`
does not exist. `insert_next_match([b, b])` should return `NoExpansion {token=b, width=1}`.
Then the cursor advances by 1 to `[b]` (single atom fast-path), commits `bb` ?—
actually no, the fast-path only commits the single token, it does not create `bb`.
`bb` would only be created if there's a second scan over `[b, b]` from the start.

This suggests the **test expectation itself may need review**: does reading `"aabbaabb"`
on a fresh graph actually produce `bb` as a standalone token? Or does it only produce
`aa` and `aabb = [aa, b, b]`? The ngrams oracle would be the ground truth here.
Running `ngrams("aabbaabb")` would clarify whether `bb` and `aabb = [[aa,bb]]` are
part of the correct output.

**Action:** Run the ngrams oracle on `"aabbaabb"` (add a `ngrams_inspect_aabbaabb`
test similar to `ngrams_inspect_abcabababcaba`) and compare. If ngrams does not
produce `bb` or `aabb → [[aa,bb]]`, then the `repetition_aabbaabb` test expectations
are wrong.

---

### Step R2 — Root-cause the Group A missing-pattern failures

**The missing pattern is always the overlap band, never the primary band.** This
strongly suggests that `build_overlap_state` is constructing a `BandState::WithOverlap`
that has the bands reversed, or that `collapse()` is only inserting one of the two
patterns into the bundled token.

**Action:** Add a unit test for `BandState::collapse()` in isolation:
- Construct a `WithOverlap` state manually with known primary and overlap bands.
- Call `collapse()`.
- Assert that the resulting bundled token has exactly two child patterns matching
  the inputs.

If `collapse()` is correct in isolation, the bug is in `build_overlap_state`
producing wrong band contents. If `collapse()` is wrong in isolation, fix it there.

The specific area to inspect is `graph.insert_patterns(vec![primary_vec, overlap_vec])`
in `BandState::collapse()` — if the two vecs are identical (due to wrong complement
construction), `insert_patterns` may deduplicate them into a single pattern.

---

### Step R3 — Root-cause `sync_read_text2` / Group D early `held` creation

**Action:** Add logging to `find_overlap` and `build_overlap_state` that prints
every `insert_next_match` call made during the read of `"heldld"`. Identify the
call that creates `held`.

**Hypothesis to test:** `build_prefix_complement` in `chain/mod.rs` calls
`graph.insert_init` which may create `held` as a side effect of building the
complement for the `[ld, ld]` overlap (if a spurious overlap is detected between
the `e,l,d` anchor and the second `l,d` segment).

---

### Step R4 — Verify Group A fix against all 8 tests simultaneously

Once R2 is resolved, run all 11 tests. If R0 and R2 are fixed:
- Group C: 1 test removed (validate_mixed_pattern).
- Group A: 8 tests should pass.
- Group B: `repetition_aabbaabb` — depends on R1 verdict.
- Group D: `sync_read_text2` — depends on R3 verdict.

---

## Open Questions

| # | Question | Blocks |
|---|----------|--------|
| OQ-T4-1 | Does the ngrams oracle on `"aabbaabb"` produce `bb → [[b,b]]` and `aabb → [[aa,bb]]`? If not, the `repetition_aabbaabb` test expectations are wrong and must be corrected before any implementation work. | R1 |
| OQ-T4-2 | Does `BandState::collapse()` correctly call `graph.insert_patterns` with two distinct non-empty vecs? Or does the complement construction produce equal vecs? | R2 |
| OQ-T4-3 | Is `build_prefix_complement` in `chain/mod.rs` calling `insert_next_match` or `insert_init` in a way that creates tokens outside the current segment's atom range? | R3 |
| OQ-T4-4 | For the `sync_read_text2` Group D failure: does `"heldld"` intentionally produce `held` as a side-effect in the correct algorithm, or is it a bug? Cross-check with the ngrams oracle on `"heldld"`. | R3 |
| OQ-T4-5 | The `validate_mixed_pattern` failure reveals that context-read and ngrams have different contracts for inputs with no repeated substrings. Should the ngrams validation test suite be restricted to inputs where the ngrams oracle is applicable (inputs where at least one n-gram of length ≥ 2 appears more than once)? | R0 |