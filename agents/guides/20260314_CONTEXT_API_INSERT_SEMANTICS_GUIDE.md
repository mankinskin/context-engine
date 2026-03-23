---
tags: `#guide` `#context-api` `#context-insert` `#insertion` `#duplicate-vertex` `#split-join` `#debugging` `#api`
summary: Context-API insertion semantics, the duplicate vertex bug caused by edge partition merging, and the boundary-alignment fix in the split-join pipeline
---

# Context-API Insert Semantics Guide

**How insertion flows from `context-api` through `context-insert`, the duplicate vertex
bug, and the boundary-alignment fix in the split-join pipeline.**

---

## Overview

The `context-api` crate exposes three insertion entry points that are **thin forwards**
to `context-insert`'s algorithmic pipeline (search → split → join). No short-circuits,
no `ReadCtx`, no `insert_pattern` fallbacks are applied — the API layer intentionally
surfaces the exact semantics of the engine.

This guide covers:

1. How the API layer delegates to `context-insert`.
2. The `insert_or_get_complete` return-value contract (new vs. existing).
3. The duplicate/stray vertex bug that arose from the join pipeline.
4. The boundary-alignment fix in `populate_edge_partitions` / `add_root_pattern`.
5. Debugging and testing strategies.

---

## API Entry Points

All three methods live in `crates/context-api/src/commands/insert.rs`.

| Method | Input | Creates atoms? | Min length |
|--------|-------|----------------|------------|
| `insert_first_match` | `Vec<TokenRef>` (resolved to `Token`) | No | 2 |
| `insert_sequence` | `&str` (text) | Yes (auto) | 2 chars |
| `insert_sequences` | `HashSet<String>` | Yes (auto) | 2 chars each |

### Delegation Pattern

Every method follows the same skeleton:

```rust
// 1. Resolve / create atom tokens
let tokens: Vec<Token> = …;

// 2. Forward directly to context-insert
let result = <_ as ToInsertCtx<IndexWithPath>>::insert_or_get_complete(
    &graph_ref,
    tokens,
);

// 3. Map the inner Ok/Err to (Token, already_existed)
let (token, already_existed) = match result {
    Ok(iwp)  => (iwp.index, false),   // newly created
    Err(iwp) => (iwp.index, true),    // already existed
};

// 4. Mark workspace dirty only when new
if !already_existed { ws.mark_dirty(); }
```

**Key point:** `insert_or_get_complete` returns `Result<IndexWithPath, IndexWithPath>`.
`Ok` means the token was newly created via the split+join pipeline. `Err` means a
full match already existed (the `Err` carries the existing token — it is *not* a
failure).

### Important Behavioural Nuance

When the search phase finds `is_entire_root() && !query_exhausted()`, the engine
returns the matched root (a *prefix* token) rather than continuing to build a new
compound token. This means callers may receive a token whose width is shorter than
the query. Use `already_existed` and `token.width` to detect this case.

---

## The Duplicate Vertex Bug

### Symptom

Inserting a short subsequence into a graph that already contains a longer sequence
creates an **unexpected extra vertex** for the remaining suffix.

**Reproduction:** Given atoms `a..i` and an existing compound token `abcdefghi`,
inserting `abc` produced:

- The expected `abc` token ✅
- An unwanted `defghi` compound vertex ❌

### Root Cause

The split+join pipeline works in three stages:

1. **Search** — find where the new pattern overlaps existing tokens.
2. **Split** — partition the root token's children at the overlap boundary.
3. **Join** — reassemble the root's pattern with the new token spliced in.

During the **join** phase, `PartitionMergeIter::populate_edge_partitions()` must
handle the children that fall *outside* the operating range (the prefix/postfix
"edge" partitions). Previously it **always** called `merge_token_only()` on these
edge partitions, which created a new compound vertex even when the split boundary
aligned perfectly with the root's original child-token boundaries.

`add_root_pattern()` then used that compound vertex as a single child in the new
root pattern, e.g. `[abc, defghi]` instead of `[abc, d, e, f, g, h, i]`.

In atom-only graphs (where every child is width-1), the boundary **always** aligns
with an existing child boundary, so the compound edge vertex was never necessary.

### The Fix (Hybrid Boundary Check)

Two new helper methods on `MergeCtx` detect whether the split boundary aligns with
original child-token boundaries:

```rust
fn try_original_prefix_tokens(&self, boundary_width: usize) -> Option<Vec<Token>>
fn try_original_postfix_tokens(&self, boundary_width: usize) -> Option<Vec<Token>>
```

Each walks the root's first child pattern, accumulating widths. If the cumulative
width exactly equals `boundary_width` at a child boundary, it returns the original
tokens (`Some`). Otherwise (boundary falls *inside* a compound child) it returns
`None`.

**`populate_edge_partitions` now branches:**

```text
if boundary aligns (Some):
    skip merge_token_only — no new vertex created
    add_root_pattern will splice original child tokens
else (None):
    call merge_token_only — compound edge vertex needed
    add_root_pattern uses that single compound token
```

**`add_root_pattern` mirrors the same logic** for each edge (prefix/postfix/infix).
When original tokens are available it splices them; otherwise it falls back to the
compound token from the range map.

### Result After Fix

Inserting `abc` into `abcdefghi` now produces:

```text
root pattern: [abc, d, e, f, g, h, i]
```

No `defghi` vertex is created. Complex graphs with nested compound children still
get the compound edge vertex when needed (the boundary falls inside a compound
child, so `try_original_*_tokens` returns `None`).

---

## Affected Files

| File | Role |
|------|------|
| `crates/context-api/src/commands/insert.rs` | API entry points — thin forward to `context-insert` |
| `crates/context-insert/src/join/context/node/merge/iter.rs` | `populate_edge_partitions`, `add_root_pattern`, `try_original_prefix_tokens`, `try_original_postfix_tokens` |
| `crates/context-insert/src/join/context/node/merge/partition.rs` | `merge_token_only` — still used when boundary is inside compound child |
| `crates/context-insert/src/join/context/frontier.rs` | `FrontierSplitIterator` — orchestrates the join walk |
| `crates/context-insert/src/insert/context.rs` | `InsertCtx::insert_init` — creates `FrontierSplitIterator` |

---

## Debugging Insertion Issues

### Enable Tracing

```bash
LOG_STDOUT=1 LOG_FILTER=trace cargo test -p context-insert <test_name> -- --nocapture
```

Then inspect `target/test-logs/<test_name>.log`. Key events to look for:

| Log message | Meaning |
|-------------|---------|
| `"Skipping prefix edge — boundary aligns with child tokens"` | No extra vertex for prefix |
| `"Skipping postfix edge — boundary aligns with child tokens"` | No extra vertex for postfix |
| `"Merging prefix edge — boundary inside compound child"` | Compound edge vertex created (expected for nested graphs) |
| `"Using original prefix tokens"` / `"Using original postfix tokens"` | `add_root_pattern` splicing originals |
| `"Root pattern already exists, skipping"` | Idempotent — pattern was already present |
| `"Skipping add_root_pattern - pattern was already modified by replace_in_pattern"` | Perfect replacement, no new pattern needed |

### Checking for Unwanted Vertices

After an insertion, count the vertices. In the CLI:

```bash
context-cli stats <workspace>
```

Or programmatically:

```rust
let stats = mgr.get_statistics("ws")?;
println!("vertices={} atoms={} patterns={}",
    stats.vertex_count, stats.atom_count, stats.pattern_count);
```

If `vertex_count` is higher than expected, inspect the graph snapshot:

```rust
let snap = mgr.get_snapshot("ws")?;
for node in &snap.nodes {
    println!("{}: label={:?} width={}", node.index, node.label, node.width);
}
```

Look for compound vertices whose label matches a suffix/prefix of an existing
token — these are the "stray" vertices the bug used to create.

### Reproducing the Original Bug

The minimal reproduction is:

1. Create atoms `a` through `i`.
2. Insert `abcdefghi` (creates a root compound vertex).
3. Insert `abc`.
4. **Before fix:** a `defghi` vertex appears. **After fix:** no extra vertex.

```rust
#[test]
fn insert_subsequence_no_stray_vertex() {
    let _tracing = init_test_tracing!(&graph);
    let mut mgr = setup();
    mgr.create_workspace("test").unwrap();

    // Insert long sequence first
    let r1 = mgr.insert_sequence("test", "abcdefghi").unwrap();
    assert!(!r1.already_existed);

    let before = mgr.get_statistics("test").unwrap().vertex_count;

    // Insert subsequence
    let r2 = mgr.insert_sequence("test", "abc").unwrap();
    assert!(!r2.already_existed);

    let after = mgr.get_statistics("test").unwrap().vertex_count;

    // Only the `abc` vertex should be new — no `defghi` vertex
    assert_eq!(after, before + 1, "expected exactly 1 new vertex (abc)");
}
```

---

## API Design Decisions

### Why Thin Forwarding (No ReadCtx / No Fallbacks)

Previous iterations of the API used `ReadCtx` and `insert_pattern` fallbacks to
work around algorithmic issues. This masked bugs and made it impossible to tell
whether the engine or the API was responsible for incorrect behaviour. The current
design:

- **Exposes exact `context-insert` semantics** so algorithm bugs surface immediately.
- **Keeps the API layer trivially auditable** (< 100 lines per method).
- **Makes test failures actionable** — if a test fails, the fix belongs in
  `context-insert`, not in API workarounds.

### Possible Future: Dual Modes

If end-users need a more forgiving interface, a future "convenience mode" could
reintroduce higher-level orchestration (ReadCtx + guarded `insert_pattern`
fallback) as an explicit opt-in. The strict/algorithmic mode would remain the
default.

---

## Common Mistakes

| Mistake | Symptom | Fix |
|---------|---------|-----|
| Treating `Err(iwp)` from `insert_or_get_complete` as failure | Logic skips valid existing tokens | `Err` means "already existed" — map to `already_existed = true` |
| Expecting returned token width == query length | Assertion failures on prefix matches | Check `is_entire_root && !query_exhausted` — engine may return a prefix token |
| Inserting single-char text via `insert_sequence` | `InsertError::QueryTooShort` | Minimum is 2 characters; use `add_atom` for single chars |
| Modifying graph between search and insert | Inconsistent state / panics | The search → split → join pipeline must run atomically on a consistent graph |
| Not marking workspace dirty after insert | Changes lost on save | The API handles this automatically; only relevant if bypassing the API |

---

## Related Documentation

- **Insert algorithm:** `agents/guides/20251203_CONTEXT_INSERT_GUIDE.md`
- **Search ↔ Insert interop:** `agents/guides/20251204_CONTEXT_INSERT_SEARCH_INTEROP.md`
- **Cheat sheet (types & patterns):** `agents/CHEAT_SHEET.md`
- **context-insert architecture:** `crates/context-insert/HIGH_LEVEL_GUIDE.md`
- **context-api README:** `crates/context-api/README.md`

---

## Summary

| Topic | Key Takeaway |
|-------|-------------|
| API design | Thin forward to `context-insert` — no masking, no fallbacks |
| `insert_or_get_complete` | `Ok` = new, `Err` = existed (not a failure) |
| Duplicate vertex bug | Caused by `merge_token_only` creating compound edge vertices when split boundary aligned with atom boundaries |
| Fix | `try_original_prefix_tokens` / `try_original_postfix_tokens` detect alignment; skip compound creation; splice original children in `add_root_pattern` |
| When compound edges are still needed | Boundary falls inside a compound child (nested graphs) |
| Debugging | Enable tracing, look for "Skipping … edge" vs. "Merging … edge" log lines |