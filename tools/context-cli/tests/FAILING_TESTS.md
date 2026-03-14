# Failing Tests Tracker

> Auto-generated after running: `cargo test -p context-cli --test cli_integration`
> Last updated: 2026-03-14
> Skill 3 exploration run added: 8 new green tests, 10 new ignored tests (RC-1 / RC-3)

## Summary

| Status | Count |
|--------|-------|
| ✅ Passing | 31 |
| ❌ Failing | 9 |
| ⏭️ Ignored | 10 |
| **Total** | **50** |

## Root Causes

| ID | Root Cause | Crate | Affected Tests |
|----|-----------|-------|----------------|
| RC-1 | `insert_sequence` calls `insert_next_match` only **once** — first atom matched via `NoExpansion`, compound tokens never created; `already_existed=true` and `width=1` for every multi-char insert | `context-api` | `read_known_pattern`, `read_produces_decomposition_tree`, `read_text_output`, `dedup_insert_then_read`, `file_read_basic` + all `skill3_exp_*` ignored tests |
| RC-2 | `ReadCtx::read_sequence` returns `None` after prior insert (graph state interaction) | `context-read` | `read_sequence_after_insert`, `dedup_read_sequence_finds_existing`, `dedup_multiple_reads_same_result` |
| RC-3 | Repeat/overlap handling broken — cursor advancement and `append_to_pattern` issues | `context-read` | `edge_repeated_single_char`, `skill3_exp_m_repeated_char_known_failing` |

## Failure Details

### RC-1 Mechanism (identified via Skill 3 exploration)

`insert_sequence` converts text to atom tokens, then calls `insert_next_match([atom₀, atom₁, …])` **once**.
On a fresh graph the first call matches only `atom₀` (the first character) and returns `NoExpansion` —
meaning "I matched a complete token at position 0 but the query extends beyond it; advance the cursor."
Because `already_existed = !outcome.is_expanded()` and `NoExpansion` is not `Created`, the function
returns `already_existed=true` and `token.width=1` (the first atom) for **every** multi-character insert.

**Fix needed:** Add an outer loop to `WorkspaceManager::insert_sequence` that repeatedly calls
`insert_next_match` with the remaining atoms, collecting segment tokens, until the full input is consumed.
The collected segments are then wrapped into a root token.  The outer loop already exists in `ReadCtx`.

**Unaffected today:** atom auto-creation, atom deduplication, graph validation, `read <index>`, `show`,
`stats`, and the read pipeline (`read <text>`).

---

### ❌ `integration::basic_read_tests::read_known_pattern`

- **Root Cause:** RC-1 (insert_sequence missing outer loop — returns first atom only)
- **Expected:** Insert "abc", read root → text is "abc" with children
- **Actual:** `read_as_text` returns "a" (only first character)
- **Fix Plan:** Pre-existing `context-insert` / `context-read` algorithm issues — tracked in parent plan Phase 3 fixes

### ❌ `integration::basic_read_tests::read_produces_decomposition_tree`

- **Root Cause:** RC-1 (insert_sequence missing outer loop — returns first atom only)
- **Expected:** Insert "hello", read root → text is "hello" with width 5
- **Actual:** `read_pattern` returns text "h" with width 1
- **Fix Plan:** Same as RC-1

### ❌ `integration::basic_read_tests::read_text_output`

- **Root Cause:** RC-1 (insert_sequence missing outer loop — returns first atom only)
- **Expected:** Insert "hello", `read_as_text` → "hello"
- **Actual:** Returns "h"
- **Fix Plan:** Same as RC-1

### ❌ `integration::basic_read_tests::read_sequence_after_insert`

- **Root Cause:** RC-2 (`ReadCtx::read_sequence` returns None after insert)
- **Expected:** Insert "abc", then `read_sequence("abc")` returns the existing structure
- **Actual:** `read_sequence` returns `None`, resulting in `InternalError`
- **Fix Plan:** Investigate interaction between insert and read graph state. May be related to `ReadCtx` atom resolution after graph has been modified by insert.

### ❌ `integration::dedup_tests::dedup_insert_then_read`

- **Root Cause:** RC-1 (insert_sequence missing outer loop — returns first atom only)
- **Expected:** Insert "hello", `read_as_text` → "hello"
- **Actual:** Returns truncated text
- **Fix Plan:** Same as RC-1

### ❌ `integration::dedup_tests::dedup_read_sequence_finds_existing`

- **Root Cause:** RC-2 (`ReadCtx::read_sequence` returns None after insert)
- **Expected:** Insert "hello", then `read_sequence("hello")` returns root with width 5
- **Actual:** `read_sequence` returns `None`, resulting in `InternalError`
- **Fix Plan:** Same as RC-2

### ❌ `integration::dedup_tests::dedup_multiple_reads_same_result`

- **Root Cause:** RC-2 (`ReadCtx::read_sequence` returns None after insert)
- **Expected:** Insert "test", then two `read_sequence("test")` calls return identical results
- **Actual:** First `read_sequence` fails with `InternalError`
- **Fix Plan:** Same as RC-2

### ❌ `integration::edge_case_tests::edge_repeated_single_char`

- **Root Cause:** RC-3 (repeat/overlap handling broken)
- **Expected:** `read_sequence("aaaa")` returns text "aaaa" with width 4
- **Actual:** Panics or returns incorrect result due to cursor advancement issues with repeated patterns
- **Fix Plan:** `PLAN_APPEND_TO_PATTERN_FIX` (partially done) + cursor advancement fix in `context-read`

### ❌ `integration::file_input_tests::file_read_basic`

- **Root Cause:** RC-1 (insert_sequence missing outer loop)
- **Expected:** Read file containing "hello world" → text is "hello world"
- **Actual:** Returns "helo wrd" (deduplication of repeated chars produces wrong leaf text — only first atoms are recorded per insert call)
- **Fix Plan:** Same as RC-1 — once the outer loop is added, full compound tokens will be created and `collect_leaf_text` will traverse the correct child patterns

## Passing Tests (23)

### Category 1: Atom Management (6/6 ✅)
- `atom_create_basic` ✅
- `atom_create_unicode` ✅
- `atom_create_duplicate` ✅
- `atom_list_all` ✅
- `atom_get_by_index` ✅
- `atom_auto_create_on_insert` ✅

---

## Ignored Tests (10)

### Skill 3 Exploration — RC-1 blocked (9 tests)

These tests document the **intended** behaviour of `insert_next_match` and will
become green once the outer loop is added to `insert_sequence`:

| Test | Scenario |
|------|----------|
| `skill3_exp_a_fresh_insert_is_created` | First insert → `already_existed=false`, `width=5` |
| `skill3_exp_b_second_insert_is_complete` | Second insert → same index, `already_existed=true` |
| `skill3_exp_c_compound_from_known_pieces` | Insert "hel"+"lo"+"hello" → "hello" Created from parts |
| `skill3_exp_e_known_prefix_then_new_suffix` | Insert "hello" then "helloworld" — NoExpansion then Created |
| `skill3_exp_f_both_pieces_known_before_compound` | Insert "hello"+"world"+"helloworld" — compound from both pieces |
| `skill3_exp_g_idempotent_multiple_complete_calls` | 4× same text — first Created, rest Complete |
| `skill3_exp_i_minimum_length_insert` | 2-char insert → Created, width=2 |
| `skill3_exp_k_dungeon_analogy_compound` | Dungeon scenario with "dungeon run" and "dungeon dungeon" |
| `skill3_exp_l_bulk_created_then_complete` | 5 sequences Created then Complete |

### Skill 3 Exploration — RC-3 blocked (1 test)

| Test | Scenario |
|------|----------|
| `skill3_exp_m_repeated_char_known_failing` | "aaaa" → Created, width=4, 1 atom |

---

## Passing Tests (31)

### Category 0: Skill 3 Exploration — Green Today (8/8 ✅)
- `skill3_exp_d_atom_deduplication` ✅ — h,e,l,o = 4 atoms after "hel"+"lo"+"hello"
- `skill3_exp_h_graph_valid_after_completion_sequence` ✅ — validate after many inserts
- `skill3_exp_j_single_char_is_too_short` ✅ — single char → QueryTooShort error
- `skill3_obs1_multi_char_insert_reports_already_existed_true_today` ✅ — RC-1 current state
- `skill3_obs2_same_text_twice_same_atom_index_today` ✅ — RC-1 current state
- `skill3_obs3_atoms_correctly_auto_created_and_deduped` ✅ — atom creation works
- `skill3_obs4_cross_word_atom_sharing` ✅ — "hello"+"world" = 7 atoms
- `skill3_obs5_graph_valid_under_rc1_conditions` ✅ — graph stays valid under RC-1

### Category 2: Basic Read (3/8 — 5 failing)
- `read_single_atom` ✅
- `read_sequence_text` ✅
- `read_sequence_single_char` ✅
- `read_empty_sequence_returns_error` ✅
- `read_sequence_after_insert` ❌ (RC-2)
- `read_known_pattern` ❌ (RC-1)
- `read_produces_decomposition_tree` ❌ (RC-1)
- `read_text_output` ❌ (RC-1)

### Category 3: Deduplication (5/8 — 3 failing)
- `dedup_exact_match` ✅
- `dedup_shared_prefix` ✅
- `dedup_no_duplicate_vertices` ✅
- `dedup_atoms_not_duplicated` ✅
- `dedup_graph_valid_after_inserts` ✅
- `dedup_insert_then_read` ❌ (RC-1)
- `dedup_read_sequence_finds_existing` ❌ (RC-2)
- `dedup_multiple_reads_same_result` ❌ (RC-2)

### Category 4: File Input (3/4 — 1 failing)
- `file_read_unicode` ✅
- `file_read_empty` ✅
- `file_read_nonexistent` ✅
- `file_read_basic` ❌ (RC-1)

### Category 5: REPL Integration
- ⏭️ Deferred — requires CLI binary process spawning

### Category 6: Edge Cases (5/6 — 1 failing)
- `error_read_no_workspace` ✅
- `error_read_invalid_index` ✅
- `error_read_closed_workspace` ✅
- `edge_single_char` ✅
- `edge_two_chars` ✅
- `edge_repeated_single_char` ❌ (RC-3)

## Notes

- All 6 atom tests pass — atom management is stable
- All 8 skill3 exploration green tests pass — atom dedup, graph validity, and error guards are solid
- All error-handling edge cases pass — the API layer correctly propagates errors
- Read operations on fresh sequences (no prior insert) work correctly
- Failures cluster around two patterns:
  1. **RC-1 (missing outer loop)** — `insert_sequence` calls `insert_next_match` only once; every
     multi-char insert returns `NoExpansion` for the first atom (`already_existed=true`, `width=1`)
  2. **RC-2 (`ReadCtx` after insert)** — `read_sequence` returns `None` when the graph already
     contains structure from a prior insert
- These are pre-existing bugs in `context-read` and `context-api`, not regressions from the 3b/3e changes
- Skill 3 exploration tests (Section 0) provide a precise characterisation of RC-1 and serve as
  the acceptance criteria for the fix: all `skill3_exp_*` tests must turn green