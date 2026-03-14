# Failing Tests Tracker

> Auto-generated after running: `cargo test -p context-cli --test cli_integration`
> Last updated: 2026-03-14

## Summary

| Status | Count |
|--------|-------|
| ✅ Passing | 23 |
| ❌ Failing | 9 |
| ⏭️ Ignored | 0 |
| **Total** | **32** |

## Root Causes

| ID | Root Cause | Crate | Affected Tests |
|----|-----------|-------|----------------|
| RC-1 | `insert_sequence` returns truncated text (only first char) via `read_as_text` / `read_pattern` | `context-api` + `context-insert` | `read_known_pattern`, `read_produces_decomposition_tree`, `read_text_output`, `dedup_insert_then_read`, `file_read_basic` |
| RC-2 | `ReadCtx::read_sequence` returns `None` after prior insert (graph state interaction) | `context-read` | `read_sequence_after_insert`, `dedup_read_sequence_finds_existing`, `dedup_multiple_reads_same_result` |
| RC-3 | Repeat/overlap handling broken — cursor advancement and `append_to_pattern` issues | `context-read` | `edge_repeated_single_char` |

## Failure Details

### ❌ `integration::basic_read_tests::read_known_pattern`

- **Root Cause:** RC-1 (insert returns truncated text)
- **Expected:** Insert "abc", read root → text is "abc" with children
- **Actual:** `read_as_text` returns "a" (only first character)
- **Fix Plan:** Pre-existing `context-insert` / `context-read` algorithm issues — tracked in parent plan Phase 3 fixes

### ❌ `integration::basic_read_tests::read_produces_decomposition_tree`

- **Root Cause:** RC-1 (insert returns truncated text)
- **Expected:** Insert "hello", read root → text is "hello" with width 5
- **Actual:** `read_pattern` returns text "h" with width 1
- **Fix Plan:** Same as RC-1

### ❌ `integration::basic_read_tests::read_text_output`

- **Root Cause:** RC-1 (insert returns truncated text)
- **Expected:** Insert "hello", `read_as_text` → "hello"
- **Actual:** Returns "h"
- **Fix Plan:** Same as RC-1

### ❌ `integration::basic_read_tests::read_sequence_after_insert`

- **Root Cause:** RC-2 (`ReadCtx::read_sequence` returns None after insert)
- **Expected:** Insert "abc", then `read_sequence("abc")` returns the existing structure
- **Actual:** `read_sequence` returns `None`, resulting in `InternalError`
- **Fix Plan:** Investigate interaction between insert and read graph state. May be related to `ReadCtx` atom resolution after graph has been modified by insert.

### ❌ `integration::dedup_tests::dedup_insert_then_read`

- **Root Cause:** RC-1 (insert returns truncated text)
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

- **Root Cause:** RC-1 (truncated text due to read algorithm)
- **Expected:** Read file containing "hello world" → text is "hello world"
- **Actual:** Returns "helo wrd" (deduplication of repeated chars produces wrong leaf text)
- **Fix Plan:** Same as RC-1 — the `collect_leaf_text` function follows the first child pattern which may not contain all expected children after `insert_sequence` modifies the graph

## Passing Tests (23)

### Category 1: Atom Management (6/6 ✅)
- `atom_create_basic` ✅
- `atom_create_unicode` ✅
- `atom_create_duplicate` ✅
- `atom_list_all` ✅
- `atom_get_by_index` ✅
- `atom_auto_create_on_insert` ✅

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
- All error-handling edge cases pass — the API layer correctly propagates errors
- Read operations on fresh sequences (no prior insert) work correctly
- Failures cluster around two patterns:
  1. **Post-insert reads** — after `insert_sequence`, reading the same text back produces truncated or incorrect results
  2. **`ReadCtx` after insert** — `read_sequence` returns `None` when the graph already contains structure from a prior insert
- These are pre-existing bugs in `context-read` and `context-insert`, not regressions from the CLI/API changes