---
tags: `#plan` `#context-insert` `#context-read` `#context-api` `#refactoring` `#insert_next_match` `#InsertOutcome`
summary: Rename `insert_or_get_complete` to `insert_next_match`, replace nested `Result<Result<R, R::Error>, ErrorReason>` with flat `Result<InsertOutcome, ErrorReason>` enum carrying `Response` in all variants.
status: ready-for-implementation
parent: 20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md
priority: top — design artifact for Context-Read UX Improvement project
decisions: D1, D5, D6, D7
---

# Plan: `insert_next_match` — Rename & Return Type Refactoring

**Date:** 2026-03-14
**Scope:** Medium (cross-crate API change, ~20 call sites, 4 crates touched)
**Crates:** `context-insert`, `context-api`, `context-read`, `context-search` (read-only dep)

---

## 1. Objective

Rename `insert_or_get_complete` to `insert_next_match` and replace its confusing nested `Result<Result<R, R::Error>, ErrorReason>` return type with a flat `Result<InsertOutcome, ErrorReason>` enum that distinguishes `Created`, `Complete`, and `NoExpansion` outcomes and carries the search `Response` in every variant.

---

## 2. Context

### Parent Plan

This is the **top-priority design artifact** for the [Context-Read UX Improvement](20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md) project (Phase 2: Design → Phase 3: Implement).

### Design Decisions Applied

| # | Decision | Effect |
|---|----------|--------|
| D1 | `insert_next_match` lives in `context-insert` as a rename of `insert_or_get_complete` | Rename, not new method |
| D5 | Flat 3-variant enum (`Created`, `Complete`, `NoExpansion`) — no nested Result | `InsertOutcome` type |
| D6 | All variants carry `Response` for caching/debugging | `response` field on each variant |
| D7 | Always returns `IndexWithPath` (no generics on the new method) | No `R: InsertResult` parameter |

### Why This Matters

The current API has three problems:
1. **Ambiguous semantics** — `Ok(Err(iwp))` means "already existed" for `IndexWithPath` but `Ok(Ok(token))` for `Token`. The `TryInitWith` encoding is confusing.
2. **Lost information** — the two "already existed" branches (`Complete` and `NoExpansion`) are conflated into the same `Err` variant, hiding whether the query was fully consumed.
3. **Missing `Response`** — callers cannot access the search `Response` (cache, events, cursor state) after insertion. Context-read needs this for expansion decisions.

---

## 3. Files Affected

### Core Changes (new code + signature changes)

| File | Lines | Change |
|------|-------|--------|
| `crates/context-insert/src/insert/outcome.rs` | NEW | New `InsertOutcome` enum definition |
| `crates/context-insert/src/insert/mod.rs` | L1-9, L15-36 | Add `pub mod outcome;`, add `insert_next_match` to `ToInsertCtx`, deprecate `insert_or_get_complete` |
| `crates/context-insert/src/insert/context.rs` | L62-67, L158-206 | Add `insert_next_match` impl on `InsertCtx`, add `insert_next_match_impl` |

### Production Call Site Updates

| File | Lines | Change |
|------|-------|--------|
| `crates/context-api/src/commands/insert.rs` | L73-97, L161-185 | Migrate `insert_first_match` and `insert_sequence` to `insert_next_match` |
| `crates/context-read/src/expansion/mod.rs` | L94-116 | Migrate `ExpansionCtx::new` to `insert_next_match` |

### Test Call Site Updates

| File | Call Count | Change |
|------|-----------|--------|
| `crates/context-insert/src/tests/cases/insert/context_read_scenarios.rs` | 2 | Migrate `insert_or_get_complete` → `insert_next_match` |
| `crates/context-insert/src/tests/cases/insert/expanded_overlap.rs` | 8 | Migrate all `insert_or_get_complete` calls |
| `crates/context-read/src/tests/cursor.rs` | 12 | Migrate all `insert_or_get_complete` calls |

### Documentation Updates

| File | Change |
|------|--------|
| `agents/plans/20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md` | Update references to `insert_or_get_complete` |
| `agents/plans/INDEX.md` | Add entry for this plan |

---

## 4. Execution Steps

### Step 1: Create `InsertOutcome` enum in `outcome.rs`

**File:** `crates/context-insert/src/insert/outcome.rs` (NEW)

```crates/context-insert/src/insert/outcome.rs#L1-84
use context_search::Response;
use context_trace::*;

/// Outcome of `insert_next_match` — always resolves to a single token + path.
///
/// Each variant carries the matched/created [`IndexWithPath`] and the search
/// [`Response`] for caching, debugging, and downstream visibility.
///
/// # Variants
///
/// - **`Created`** — A new token was inserted via the split+join pipeline.
///   The query extended beyond what was known in the graph.
/// - **`Complete`** — The query was fully consumed by an existing token.
///   No insertion was needed (idempotent match).
/// - **`NoExpansion`** — The search found an existing token at the start of
///   the query, but the query extends beyond it. No new token was created.
///   The caller should advance by the returned token's width and retry.
#[derive(Debug, Clone)]
pub enum InsertOutcome {
    /// Newly created via split+join pipeline.
    /// The query extended beyond what was known; a new token was inserted.
    Created {
        result: IndexWithPath,
        response: Response,
    },

    /// Full match already existed — the query was fully consumed by an
    /// existing token. No insertion was needed.
    Complete {
        result: IndexWithPath,
        response: Response,
    },

    /// No expansion: the search found an existing token at the start of
    /// the query, but the query extends beyond it. No new token was created.
    /// The caller should advance by the returned token's width and try again.
    NoExpansion {
        result: IndexWithPath,
        response: Response,
    },
}

impl InsertOutcome {
    /// The matched/created token + cursor path (all variants carry this).
    pub fn result(&self) -> &IndexWithPath {
        match self {
            InsertOutcome::Created { result, .. } => result,
            InsertOutcome::Complete { result, .. } => result,
            InsertOutcome::NoExpansion { result, .. } => result,
        }
    }

    /// Consume into the [`IndexWithPath`].
    pub fn into_result(self) -> IndexWithPath {
        match self {
            InsertOutcome::Created { result, .. } => result,
            InsertOutcome::Complete { result, .. } => result,
            InsertOutcome::NoExpansion { result, .. } => result,
        }
    }

    /// The token (shorthand for `result().index`).
    pub fn token(&self) -> Token {
        self.result().index
    }

    /// The search response (for caching, debugging, trace inspection).
    pub fn response(&self) -> &Response {
        match self {
            InsertOutcome::Created { response, .. } => response,
            InsertOutcome::Complete { response, .. } => response,
            InsertOutcome::NoExpansion { response, .. } => response,
        }
    }

    /// Whether a new token was created (split+join pipeline ran).
    pub fn is_expanded(&self) -> bool {
        matches!(self, InsertOutcome::Created { .. })
    }

    /// Whether the query was fully consumed by an existing token.
    pub fn is_complete(&self) -> bool {
        matches!(self, InsertOutcome::Complete { .. })
    }

    /// Whether no expansion occurred (starting token is the best match,
    /// but query extends beyond it).
    pub fn is_no_expansion(&self) -> bool {
        matches!(self, InsertOutcome::NoExpansion { .. })
    }
}
```

**Verification:** `cargo check -p context-insert` — should compile (no dependents yet).

---

### Step 2: Add `insert_next_match` to `InsertCtx` (parallel to `insert_or_get_complete`)

**File:** `crates/context-insert/src/insert/context.rs`

Add a new method `insert_next_match` and a new internal method `insert_next_match_impl` alongside the existing methods. The existing `insert_or_get_complete` and `insert_impl` remain untouched at this step.

**Add after L67** (after `insert_or_get_complete`):

```crates/context-insert/src/insert/context.rs#L68-73
    #[context_trace::instrument_sig(level = "info", skip(self))]
    pub fn insert_next_match(
        &mut self,
        searchable: impl Searchable<InsertTraversal> + Debug,
    ) -> Result<InsertOutcome, ErrorReason> {
        self.insert_next_match_impl(searchable).map_err(|err| err.reason)
    }
```

**Add after `insert_impl`** (after ~L206, as a new method on `InsertCtx<R>`):

```crates/context-insert/src/insert/context.rs#L208-264
    /// Core implementation for `insert_next_match`.
    ///
    /// Unlike `insert_impl`, this method:
    /// - Always returns `IndexWithPath` (no generics)
    /// - Distinguishes `Complete` vs `NoExpansion` (no `TryInitWith` encoding)
    /// - Carries the search `Response` in every variant
    fn insert_next_match_impl(
        &mut self,
        searchable: impl Searchable<InsertTraversal>,
    ) -> Result<InsertOutcome, ErrorState> {
        use crate::insert::outcome::InsertOutcome;

        match searchable.search(self.graph.clone()) {
            Ok(result) => {
                if result.is_entire_root() && result.query_exhausted() {
                    // Complete: query fully matched an existing token
                    let query_path = result.query_cursor().path().clone();
                    let root_token = result.root_token();
                    let response = result;

                    Ok(InsertOutcome::Complete {
                        result: IndexWithPath {
                            index: root_token,
                            path: query_path,
                        },
                        response,
                    })
                } else if result.is_entire_root() && !result.query_exhausted() {
                    // NoExpansion: found a token at start but query extends beyond
                    let root_token = result.root_token();
                    let query_path = result.query_cursor().path().clone();
                    let response = result;

                    Ok(InsertOutcome::NoExpansion {
                        result: IndexWithPath {
                            index: root_token,
                            path: query_path,
                        },
                        response,
                    })
                } else {
                    // Partial match — need to insert via split+join
                    // Clone the response BEFORE consuming it into InitInterval
                    let response = result.clone();
                    let extract = <PatternRangePath as ResultExtraction>::extract_from(&result);
                    let init = InitInterval::from(result);

                    // insert_init returns the new token
                    let new_token = self.insert_init(extract, init)?;

                    // Build the IndexWithPath from the new token + extracted path
                    let iwp: IndexWithPath = new_token.into();

                    Ok(InsertOutcome::Created {
                        result: iwp,
                        response,
                    })
                }
            },
            Err(err) => Err(err),
        }
    }
```

**Important:** The method is defined on `impl<R: InsertResult> InsertCtx<R>` so it can call `self.insert_init()`. However, `insert_next_match_impl` ignores the `R` generic — it always builds `IndexWithPath` directly. The `insert_init` call uses `R::Extract` and `R::build_with_extract`, but we handle this explicitly (see Step 4 for details).

**Also add the import at the top of context.rs** (after existing imports):

```crates/context-insert/src/insert/context.rs#L2-3
use crate::insert::outcome::InsertOutcome;
use crate::insert::result::ResultExtraction;
```

**Verification:** `cargo check -p context-insert` — new method compiles alongside existing one.

---

### Step 3: Add `insert_next_match` to `ToInsertCtx` trait

**File:** `crates/context-insert/src/insert/mod.rs`

**Add after `pub mod result;` (L9):**

```crates/context-insert/src/insert/mod.rs#L10
pub mod outcome;
```

**Add new trait method after `insert_or_get_complete` (after L36):**

```crates/context-insert/src/insert/mod.rs#L37-43
    fn insert_next_match(
        &self,
        searchable: impl Searchable<InsertTraversal>,
    ) -> Result<outcome::InsertOutcome, context_trace::graph::getters::ErrorReason> {
        self.insert_context().insert_next_match(searchable)
    }
```

**Add re-export at the top of mod.rs for convenience** (after `use context_search::*;` on L5):

```crates/context-insert/src/insert/mod.rs#L6
pub use outcome::InsertOutcome;
```

**Note:** The `insert_context()` call returns `InsertCtx<R>` where `R` comes from the trait's generic parameter. The `insert_next_match` method on `InsertCtx` is defined for all `R: InsertResult`, so this works regardless of whether the caller uses `ToInsertCtx<Token>` or `ToInsertCtx<IndexWithPath>`.

**Verification:** `cargo check -p context-insert` — trait change compiles.

---

### Step 4: Response Extraction Design — Getting `Response` Through to `InsertOutcome`

This is the central design challenge. The search returns a `Response` which is currently **moved** into `InitInterval::from(result)` in the `Created` branch. We need the `Response` in the `InsertOutcome::Created` variant too.

#### Current Flow (Created branch)

```/dev/null/current_flow.txt#L1-8
searchable.search(graph)          → Ok(result: Response)
ResultExtraction::extract_from(&result)  → extract: PatternRangePath   [borrows result]
InitInterval::from(result)        → init: InitInterval                 [MOVES result — consumes cache, root, end_bound]
self.insert_init(extract, init)   → new_token: R                      [split+join pipeline]
R::build_with_extract(new_token, extract) → final result
```

The problem: `InitInterval::from(result)` on L201 of `context.rs` **moves** the `Response`, consuming it. After this line, `result` is gone.

#### Solution: Clone `Response` Before the Move

`Response` derives `Clone` (confirmed: `#[derive(Debug, Clone, Eq)]` at `crates/context-search/src/state/response.rs` L17). The clone cost is acceptable because:

1. **`Response` is small-ish** — it contains a `TraceCache` (HashMap of visited entries), a `MatchResult` (enum + cursor), and a `Vec<GraphOpEvent>`. In practice these are modest-sized for typical queries.
2. **`Created` is the rare branch** — most calls hit `Complete` or `NoExpansion`, which don't need a clone (they move `result` directly into the outcome).
3. **The alternative (restructuring `InitInterval`)** would require deep changes to the split pipeline and is far more risky.

#### New Flow (Created branch)

```/dev/null/new_flow.txt#L1-9
searchable.search(graph)          → Ok(result: Response)
let response = result.clone();    ← CLONE for InsertOutcome           [cost: one clone on Created path only]
ResultExtraction::extract_from(&result)  → extract: PatternRangePath   [borrows result]
InitInterval::from(result)        → init: InitInterval                 [moves result — fine, we have the clone]
self.insert_init(extract, init)   → new_token: Token                   [split+join pipeline]
IndexWithPath::build_with_extract(new_token, extract) → iwp
InsertOutcome::Created { result: iwp, response }                       [response is the clone]
```

#### Why Not Restructure `InitInterval`?

`InitInterval::from(Response)` extracts `cache`, `root_token()`, and `cursor_position()` from the `Response`. We could change this to borrow, but:
- `InitInterval` takes ownership of `cache: TraceCache` (used by the split pipeline)
- Changing this would require the split pipeline to borrow the cache, which cascades through `SplitTraceStatesCtx`, `SplitCacheCtx`, and `IntervalGraph::try_from_init`
- This is a large, risky refactor for minimal benefit — a single clone is simpler and safer

#### Complete/NoExpansion Branches

These branches don't call `insert_init`, so `result` is available directly:

```/dev/null/complete_branch.txt#L1-7
// Complete branch — result is moved into the outcome, no clone needed
let query_path = result.query_cursor().path().clone();   // clone the path (small)
let root_token = result.root_token();                     // copy (Token is Copy)
let response = result;                                    // MOVE — no clone needed

InsertOutcome::Complete { result: IndexWithPath { index: root_token, path: query_path }, response }
```

**Wait — there's a subtlety.** We call `result.query_cursor().path().clone()` and `result.root_token()` which borrow `result`, then we need to move `result` into `response`. This works because the borrows produce owned values (`PatternRangePath` and `Token`) before the move. Let's verify the exact sequence:

1. `result.query_cursor()` → `&PatternCursor<Matched>` (borrow)
2. `.path()` → `&PatternRangePath` (borrow)
3. `.clone()` → `PatternRangePath` (owned) — borrow released
4. `result.root_token()` → `Token` (Copy) — borrow released
5. `let response = result;` → move — OK, no outstanding borrows

This is safe. No clone needed for `Complete` or `NoExpansion`.

#### `insert_init` Generic Issue

The current `insert_init` is generic over `R: InsertResult`. In `insert_next_match_impl`, we always want `IndexWithPath`. Since the method is defined on `InsertCtx<R>`, and `insert_init` takes `R::Extract`, we need to handle this carefully.

**Option A (chosen): Call `insert_init` through the generic path, then convert.**

Since `insert_next_match_impl` is on `impl<R: InsertResult> InsertCtx<R>`, the `R` is already bound. For the `Created` branch:

```/dev/null/generic_handling.txt#L1-12
// R might be Token or IndexWithPath
let extract = <R::Extract as ResultExtraction>::extract_from(&result);
let init = InitInterval::from(result);  // moves result

// insert_init returns Result<R, ErrorState>
let new_token: R = self.insert_init(extract, init)?;

// Convert R → Token (both Token and IndexWithPath impl Into<Token>)
let token: Token = new_token.into();

// For the path, we already extracted it via ResultExtraction
// But we need the full IndexWithPath...
```

**Problem:** If `R = Token`, we lose the path. We need to extract the path separately.

**Better approach:** Extract the path from the response *before* the move, then build `IndexWithPath` manually after `insert_init`:

```/dev/null/better_approach.txt#L1-11
let response = result.clone();
let query_path = result.query_cursor().path().clone();
let extract = <R::Extract as ResultExtraction>::extract_from(&result);
let init = InitInterval::from(result);

let new_token: R = self.insert_init(extract, init)?;
let token: Token = new_token.into();

Ok(InsertOutcome::Created {
    result: IndexWithPath { index: token, path: query_path },
    response,
})
```

This works because:
- `query_path` is cloned before the move
- `new_token.into()` gives us the `Token` regardless of `R`
- The `IndexWithPath` is built from the original query path + new token

**Verification:** The `query_path` from the search response represents the query cursor's path at the point of the best match. The `token` from `insert_init` is the newly created token. Together they form a valid `IndexWithPath`.

---

### Step 5: Update `context-api` production call sites

**File:** `crates/context-api/src/commands/insert.rs`

#### 5a: `insert_first_match` (L73-97)

**Before** (L73-97):

```crates/context-api/src/commands/insert.rs#L73-97
        // Delegate directly to context-insert's insert_or_get_complete
        let result = <_ as ToInsertCtx<context_trace::IndexWithPath>>::insert_or_get_complete(
            &graph_ref,
            tokens,
        )
        .map_err(|e| {
            InsertError::InternalError(format!(
                "insert_or_get_complete failed: {e:?}"
            ))
        })?;

        let (token, already_existed) = match result {
            Ok(iwp) => {
                // Newly created via split+join
                debug!(token = ?iwp.index, "insert_first_match: newly inserted");
                (iwp.index, false)
            },
            Err(iwp) => {
                // Already existed — full match found
                debug!(token = ?iwp.index, "insert_first_match: already existed");
                (iwp.index, true)
            },
        };
```

**After:**

```/dev/null/insert_first_match_after.rs#L1-22
        // Delegate directly to context-insert's insert_next_match
        let outcome = graph_ref.insert_next_match(tokens)
            .map_err(|e| {
                InsertError::InternalError(format!(
                    "insert_next_match failed: {e:?}"
                ))
            })?;

        let already_existed = !outcome.is_expanded();
        let token = outcome.token();

        match &outcome {
            context_insert::InsertOutcome::Created { .. } => {
                debug!(token = ?token, "insert_first_match: newly inserted");
            },
            context_insert::InsertOutcome::Complete { .. } => {
                debug!(token = ?token, "insert_first_match: already existed (complete)");
            },
            context_insert::InsertOutcome::NoExpansion { .. } => {
                debug!(token = ?token, "insert_first_match: already existed (no expansion)");
            },
        }
```

**Key changes:**
- No more turbofish `<_ as ToInsertCtx<context_trace::IndexWithPath>>` — `insert_next_match` is not generic
- No more nested `Ok`/`Err` matching — flat enum match
- `already_existed` is now `!outcome.is_expanded()` (both `Complete` and `NoExpansion` mean "already existed")

Also update the import at the top of the file (add after L19):

```/dev/null/insert_import.rs#L1
use context_insert::InsertOutcome;
```

#### 5b: `insert_sequence` (L161-185)

**Before** (L161-185):

```crates/context-api/src/commands/insert.rs#L161-185
        // Delegate directly to context-insert's insert_or_get_complete
        let result = <_ as ToInsertCtx<context_trace::IndexWithPath>>::insert_or_get_complete(
            &graph_ref,
            tokens,
        )
        .map_err(|e| {
            InsertError::InternalError(format!(
                "insert_or_get_complete failed: {e:?}"
            ))
        })?;

        let (token, already_existed) = match result {
            Ok(iwp) => {
                debug!(token = ?iwp.index, "insert_sequence: newly inserted");
                (iwp.index, false)
            },
            Err(iwp) => {
                debug!(token = ?iwp.index, "insert_sequence: already existed");
                (iwp.index, true)
            },
        };
```

**After:**

```/dev/null/insert_sequence_after.rs#L1-22
        // Delegate directly to context-insert's insert_next_match
        let outcome = graph_ref.insert_next_match(tokens)
            .map_err(|e| {
                InsertError::InternalError(format!(
                    "insert_next_match failed: {e:?}"
                ))
            })?;

        let already_existed = !outcome.is_expanded();
        let token = outcome.token();

        match &outcome {
            InsertOutcome::Created { .. } => {
                debug!(token = ?token, "insert_sequence: newly inserted");
            },
            InsertOutcome::Complete { .. } => {
                debug!(token = ?token, "insert_sequence: already existed (complete)");
            },
            InsertOutcome::NoExpansion { .. } => {
                debug!(token = ?token, "insert_sequence: already existed (no expansion)");
            },
        }
```

**Note:** `insert_sequences` (L223-233) calls `insert_sequence` in a loop — no changes needed there.

**Verification:** `cargo check -p context-api` and `cargo test -p context-api`

---

### Step 6: Update `context-read` production call site

**File:** `crates/context-read/src/expansion/mod.rs` (L94-116)

**Before** (L94-116):

```crates/context-read/src/expansion/mod.rs#L94-116
        } else {
            // No root - use insert_or_get_complete to find longest prefix match
            let result: Result<Result<IndexWithPath, _>, _> =
                graph.insert_or_get_complete(cursor.clone());

            let IndexWithPath {
                index: first,
                path: cursor,
            } = match result {
                Ok(Ok(found)) => found,
                Ok(Err(found)) => found,
                Err(ErrorReason::SingleIndex(c)) => *c,
                Err(_) => {
                    // No match - use first cursor token
                    let first = cursor.path_root()[0];
                    debug!(first_index = ?first, "No match, using first cursor token");
                    return Self {
                        state: BandState::new(first),
                        cursor: CursorCtx::new(graph, cursor),
                    };
                },
            };

            debug!(first_index = ?first, "ExpansionCtx initialized with insert_or_get_complete result");
```

**After:**

```/dev/null/expansion_after.rs#L1-26
        } else {
            // No root - use insert_next_match to find longest prefix match
            let result = graph.insert_next_match(cursor.clone());

            let IndexWithPath {
                index: first,
                path: cursor,
            } = match result {
                Ok(outcome) => {
                    debug!(
                        outcome_variant = if outcome.is_expanded() { "Created" }
                            else if outcome.is_complete() { "Complete" }
                            else { "NoExpansion" },
                        token = ?outcome.token(),
                        "insert_next_match result"
                    );
                    outcome.into_result()
                },
                Err(ErrorReason::SingleIndex(c)) => *c,
                Err(_) => {
                    // No match - use first cursor token
                    let first = cursor.path_root()[0];
                    debug!(first_index = ?first, "No match, using first cursor token");
                    return Self {
                        state: BandState::new(first),
                        cursor: CursorCtx::new(graph, cursor),
                    };
                },
            };

            debug!(first_index = ?first, "ExpansionCtx initialized with insert_next_match result");
```

**Key changes:**
- No more type annotation `Result<Result<IndexWithPath, _>, _>` — the return type is `Result<InsertOutcome, ErrorReason>`
- No more `Ok(Ok(found))` / `Ok(Err(found))` — single `Ok(outcome)` with `outcome.into_result()`
- `ErrorReason::SingleIndex` handling remains unchanged (it's in the `Err` branch of the outer Result)
- The `response` from the outcome is available for future use (e.g., caching in expansion pipeline)

**Also add import** (after existing `use context_insert::*;` on L24 — already covers `InsertOutcome` via re-export):

No additional import needed — `InsertOutcome` is re-exported from `context_insert` via the `pub use outcome::InsertOutcome;` added in Step 3.

**Verification:** `cargo check -p context-read` and `cargo test -p context-read`

---

### Step 7: Update test call sites in `context-insert` tests

#### 7a: `context_read_scenarios.rs` (2 tests)

**File:** `crates/context-insert/src/tests/cases/insert/context_read_scenarios.rs`

**Test: `integration_partial_match_no_checkpoint`** (L41-58)

**Before:**

```crates/context-insert/src/tests/cases/insert/context_read_scenarios.rs#L49-58
    // This mimics what context-read does: insert_or_get_complete
    // Should handle gracefully without panicking
    let result: Result<Result<IndexWithPath, _>, _> =
        graph.insert_or_get_complete(query);

    // The result should either be:
    // - Ok(Ok(_)) if insertion succeeded
    // - Ok(Err(_)) if pattern already exists
    // - Err(_) if validation failed
    // It should NOT panic
    assert!(
        result.is_ok() || result.is_err(),
        "insert_or_get_complete should return a result, not panic"
    );
```

**After:**

```/dev/null/test_partial_match_after.rs#L1-10
    // This mimics what context-read does: insert_next_match
    // Should handle gracefully without panicking
    let result = graph.insert_next_match(query);

    // The result should either be:
    // - Ok(InsertOutcome::Created { .. }) if insertion succeeded
    // - Ok(InsertOutcome::Complete { .. }) if pattern already exists
    // - Ok(InsertOutcome::NoExpansion { .. }) if partial match with no expansion
    // - Err(_) if validation failed
    // It should NOT panic
    assert!(
        result.is_ok() || result.is_err(),
        "insert_next_match should return a result, not panic"
    );
```

**Test: `triple_repeat_pattern_scenario`** (L90-106)

**Before:**

```crates/context-insert/src/tests/cases/insert/context_read_scenarios.rs#L97-106
    // insert_or_get_complete should handle this without panic
    let result: Result<Result<IndexWithPath, _>, _> =
        graph.insert_or_get_complete(query);

    // Verify no panic occurred
    match result {
        Ok(Ok(_)) => { /* Pattern already exists - fine */ },
        Ok(Err(_)) => { /* Insertion completed - fine */ },
        Err(_) => { /* Error returned - fine, as long as no panic */ },
    }
```

**After:**

```/dev/null/test_triple_repeat_after.rs#L1-10
    // insert_next_match should handle this without panic
    let result = graph.insert_next_match(query);

    // Verify no panic occurred
    match result {
        Ok(InsertOutcome::Created { .. }) => { /* New token created - fine */ },
        Ok(InsertOutcome::Complete { .. }) => { /* Pattern already exists - fine */ },
        Ok(InsertOutcome::NoExpansion { .. }) => { /* Partial match - fine */ },
        Err(_) => { /* Error returned - fine, as long as no panic */ },
    }
```

#### 7b: `expanded_overlap.rs` (8 tests)

**File:** `crates/context-insert/src/tests/cases/insert/expanded_overlap.rs`

All 8 tests follow the same pattern. Here's the migration template:

**Before pattern** (repeated 8 times with variations):

```/dev/null/expanded_overlap_before.rs#L1-10
    let result: Result<Result<IndexWithPath, _>, _> =
        graph.insert_or_get_complete(query.clone());

    assert!(result.is_ok(), "insert_or_get_complete should succeed");
    let inner = result.unwrap();

    let IndexWithPath { index, path } = match inner {
        Ok(found) => found,
        Err(found) => found,
    };
```

**After pattern:**

```/dev/null/expanded_overlap_after.rs#L1-7
    let result = graph.insert_next_match(query.clone());

    assert!(result.is_ok(), "insert_next_match should succeed");
    let outcome = result.unwrap();

    let IndexWithPath { index, path } = outcome.into_result();
```

**Specific tests to update:**
1. `insert_postfix_bc_of_abc` (L53-68) — same pattern
2. `insert_postfix_bcd_extends_abc` (L82-96) — same pattern
3. `insert_single_atom_postfix` (L109-120) — same pattern
4. `insert_finds_existing_overlap_pattern` (L133-147) — same pattern
5. `insert_finds_postfix_of_compound_token` (L163-177) — same pattern
6. `cursor_position_after_postfix_match` (L195-209) — same pattern
7. `cursor_tracks_overlap_consumption` (L225-239) — same pattern + conditional check
8. `insert_no_overlap_path` (L256-267) — same pattern

**Special case: `insert_single_atom_is_postfix`** (L344-366):

**Before:**

```crates/context-insert/src/tests/cases/insert/expanded_overlap.rs#L351-366
    let result: Result<Result<IndexWithPath, _>, _> =
        graph.insert_or_get_complete(query.clone());

    // Single atom query might return error (SingleIndex) or succeed
    // Just verify no panic
    match result {
        Ok(Ok(IndexWithPath { index, .. })) => {
            assert_eq!(index, c, "Should return c atom");
        },
        Ok(Err(IndexWithPath { index, .. })) => {
            assert_eq!(index, c, "Should return c atom");
        },
        Err(e) => {
            // SingleIndex error is acceptable for single-atom queries
            // This is expected behavior per the search algorithm
        },
    }
```

**After:**

```/dev/null/single_atom_postfix_after.rs#L1-13
    let result = graph.insert_next_match(query.clone());

    // Single atom query might return error (SingleIndex) or succeed
    // Just verify no panic
    match result {
        Ok(outcome) => {
            assert_eq!(outcome.token(), c, "Should return c atom");
        },
        Err(ErrorReason::SingleIndex(_)) => {
            // SingleIndex error is acceptable for single-atom queries
            // This is expected behavior per the search algorithm
        },
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
```

**Verification:** `cargo test -p context-insert -- expanded_overlap` and `cargo test -p context-insert -- context_read_scenarios`

---

### Step 8: Update test call sites in `context-read` tests

**File:** `crates/context-read/src/tests/cursor.rs` (12 call sites)

All tests follow similar patterns. Here are the migration templates:

#### Pattern A: Simple success check (8 occurrences)

**Before** (e.g., `cursor_single_token_exhausts` L138-142):

```crates/context-read/src/tests/cursor.rs#L138-142
    let result: Result<Result<IndexWithPath, _>, _> =
        graph.insert_or_get_complete(query.clone());

    assert!(result.is_ok(), "insert_or_get_complete should succeed");
    let inner = result.unwrap();
```

...followed by:

```/dev/null/pattern_a_inner.rs#L1-4
    let IndexWithPath { index, path } = match inner {
        Ok(found) => found,
        Err(found) => found,
    };
```

**After:**

```/dev/null/pattern_a_after.rs#L1-5
    let result = graph.insert_next_match(query.clone());

    assert!(result.is_ok(), "insert_next_match should succeed");
    let outcome = result.unwrap();
    let IndexWithPath { index, path } = outcome.into_result();
```

**Applies to tests:**
- `cursor_single_token_exhausts` (L138)
- `cursor_two_tokens_first_match` (L166)
- `cursor_atoms_finds_pattern` (L194)
- `cursor_atoms_uses_existing_pattern` (L346)

#### Pattern B: Result without assertion (3 occurrences)

**Before** (e.g., `cursor_repeated_atoms_aa` L227-228):

```crates/context-read/src/tests/cursor.rs#L227-228
    let result: Result<Result<IndexWithPath, _>, _> =
        graph.insert_or_get_complete(query);
```

**After:**

```/dev/null/pattern_b_after.rs#L1
    let result = graph.insert_next_match(query);
```

Then update downstream matching accordingly (each test has its own assertions).

**Applies to tests:**
- `cursor_repeated_atoms_aa` (L227)
- `cursor_repeated_atoms_aaa` (L253)
- `cursor_repeated_atoms_aaaa` (L280)

#### Pattern C: `cursor_insert_advance_flow` (L466-495)

**Before:**

```crates/context-read/src/tests/cursor.rs#L473-488
    // Step 1: First insert_or_get_complete
    let result1: Result<Result<IndexWithPath, _>, _> =
        graph.insert_or_get_complete(original_query.to_vec());

    assert!(result1.is_ok(), "First insert should succeed");
    let inner1 = result1.unwrap();

    let first_match = match inner1 {
        Ok(found) => found,
        Err(found) => found,
    };
```

**After:**

```/dev/null/advance_flow_after.rs#L1-6
    // Step 1: First insert_next_match
    let result1 = graph.insert_next_match(original_query.to_vec());

    assert!(result1.is_ok(), "First insert should succeed");
    let first_match = result1.unwrap().into_result();
```

Also update comment on L491-494:

```/dev/null/advance_flow_comment.rs#L1-4
    // The actual cursor advancement implementation will need to:
    // 1. Track remaining = original_query[first_width..]
    // 2. Call insert_next_match on remaining
    // 3. Repeat until exhausted
```

#### Pattern D: Error handling tests

**`cursor_empty_query`** (L503-515):

**Before:**

```crates/context-read/src/tests/cursor.rs#L509-510
    let result: Result<Result<IndexWithPath, _>, _> =
        graph.insert_or_get_complete(query);
```

**After:**

```/dev/null/empty_query_after.rs#L1
    let result = graph.insert_next_match(query);
```

**`cursor_single_atom`** (L520-544):

**Before:**

```crates/context-read/src/tests/cursor.rs#L526-527
    let result: Result<Result<IndexWithPath, _>, ErrorReason> =
        graph.insert_or_get_complete(query);
```

**After:**

```/dev/null/single_atom_after.rs#L1
    let result = graph.insert_next_match(query);
```

The downstream `match` on `result` already handles `Err(ErrorReason::SingleIndex(..))` — no change needed there since `insert_next_match` returns the same `ErrorReason` variants.

**Verification:** `cargo test -p context-read -- cursor`

---

### Step 9: Deprecate `insert_or_get_complete`

**File:** `crates/context-insert/src/insert/mod.rs`

Add `#[deprecated]` attribute to the trait method:

**Before** (L31-36):

```crates/context-insert/src/insert/mod.rs#L31-36
    fn insert_or_get_complete(
        &self,
        searchable: impl Searchable<InsertTraversal>,
    ) -> Result<Result<R, R::Error>, ErrorReason> {
        self.insert_context().insert_or_get_complete(searchable)
    }
```

**After:**

```/dev/null/deprecated_trait.rs#L1-8
    #[deprecated(
        since = "0.2.0",
        note = "Use `insert_next_match` instead. Returns `Result<InsertOutcome, ErrorReason>` — a flat enum replacing the confusing nested Result."
    )]
    fn insert_or_get_complete(
        &self,
        searchable: impl Searchable<InsertTraversal>,
    ) -> Result<Result<R, R::Error>, ErrorReason> {
        self.insert_context().insert_or_get_complete(searchable)
    }
```

**File:** `crates/context-insert/src/insert/context.rs`

Add `#[deprecated]` attribute to the impl method:

**Before** (L62-67):

```crates/context-insert/src/insert/context.rs#L62-67
    pub(crate) fn insert_or_get_complete(
        &mut self,
        searchable: impl Searchable<InsertTraversal> + Debug,
    ) -> Result<Result<R, R::Error>, ErrorReason> {
        self.insert_impl(searchable).map_err(|err| err.reason)
    }
```

**After:**

```/dev/null/deprecated_impl.rs#L1-9
    #[deprecated(
        since = "0.2.0",
        note = "Use `insert_next_match` instead."
    )]
    pub(crate) fn insert_or_get_complete(
        &mut self,
        searchable: impl Searchable<InsertTraversal> + Debug,
    ) -> Result<Result<R, R::Error>, ErrorReason> {
        self.insert_impl(searchable).map_err(|err| err.reason)
    }
```

**Note:** After Steps 5-8 are complete, there should be **zero** remaining callers of `insert_or_get_complete`. The deprecation is a safety net for any code outside the main repo (or future code that accidentally uses the old API). Add `#[allow(deprecated)]` on the old method body itself to suppress self-deprecation warnings.

**Verification:** `cargo check --workspace` — should compile with zero deprecation warnings (all callers migrated).

---

### Step 10: Update documentation references in `agents/` directory

**File:** `agents/plans/20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md`

No code changes — update prose references:
- L332-335: Mark the "CURRENT" signature block as `DEPRECATED`
- L341-344: Mark the "NEW" signature block as `IMPLEMENTED`
- Add a note in the "Phase Plan Overview" section (L1091+) marking this sub-task as complete

**File:** `agents/plans/INDEX.md`

Add entry:

```/dev/null/index_entry.md#L1
| 20260314_PLAN_INSERT_NEXT_MATCH | `insert_next_match` rename & `InsertOutcome` enum | ready-for-implementation | context-insert, context-api, context-read |
```

**Verification:** Review documentation reads correctly.

---

## 5. Response Extraction Design

### Problem Statement

The search `Response` must be available in all three `InsertOutcome` variants. The `Complete` and `NoExpansion` branches are straightforward — the `Response` is simply moved into the variant. The `Created` branch is the challenge because `InitInterval::from(result)` consumes the `Response`.

### Solution: Clone-Before-Move

```/dev/null/response_extraction_diagram.txt#L1-27
┌─────────────────────────────────────────────────────────────────┐
│  searchable.search(graph) → Ok(result: Response)                │
│                                                                 │
│  ┌─ Complete branch ──────────────────────────────────────────┐ │
│  │  query_path = result.query_cursor().path().clone()  [small]│ │
│  │  root_token = result.root_token()                   [Copy] │ │
│  │  response = result                                  [MOVE] │ │
│  │  → InsertOutcome::Complete { result, response }            │ │
│  │  Cost: 0 clones                                            │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                 │
│  ┌─ NoExpansion branch ───────────────────────────────────────┐ │
│  │  (identical to Complete — MOVE, no clone)                  │ │
│  │  Cost: 0 clones                                            │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                 │
│  ┌─ Created branch ───────────────────────────────────────────┐ │
│  │  response = result.clone()            [CLONE — one-time]   │ │
│  │  query_path = result.query_cursor().path().clone()         │ │
│  │  extract = ResultExtraction::extract_from(&result)         │ │
│  │  init = InitInterval::from(result)    [MOVE — consumed]    │ │
│  │  new_token = self.insert_init(extract, init)?              │ │
│  │  token = new_token.into()                                  │ │
│  │  → InsertOutcome::Created { result: iwp, response }        │ │
│  │  Cost: 1 clone of Response                                 │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Clone Cost Analysis

`Response` contains:
- `cache: TraceCache` — a `HashMap<VertexIndex, TraceCacheEntry>`. Typical size: 5-50 entries for normal queries.
- `end: MatchResult` — small struct (enum + cursor + path).
- `events: Vec<GraphOpEvent>` — visualization events. Typically 10-100 events.

The clone happens **only on the Created path** (split+join), which is already the most expensive operation (it modifies the graph). The clone cost is negligible compared to the split+join pipeline.

### Why Not Other Approaches

| Approach | Why rejected |
|----------|-------------|
| Make `InitInterval` borrow `&Response` | Cascades through split pipeline (`SplitTraceStatesCtx`, `SplitCacheCtx`), requiring lifetime parameters everywhere. Too invasive. |
| Extract fields manually instead of `From<Response>` | Would duplicate the logic in `InitInterval::from()` and diverge. Fragile. |
| Store `Response` in `InsertCtx` as side-channel | Breaks the functional style. Would need `&mut self` threading. Error-prone. |
| Use `Rc<Response>` or `Arc<Response>` | Over-engineered for a one-time clone. Adds indirection. |

---

## 6. Migration Guide

### Pattern 1: Production code checking `already_existed`

**Before:**

```/dev/null/migration_pattern1_before.rs#L1-13
let result = <_ as ToInsertCtx<IndexWithPath>>::insert_or_get_complete(
    &graph_ref,
    tokens,
).map_err(|e| InsertError::InternalError(format!("{e:?}")))?;

let (token, already_existed) = match result {
    Ok(iwp) => (iwp.index, false),   // newly created
    Err(iwp) => (iwp.index, true),   // already existed
};
```

**After:**

```/dev/null/migration_pattern1_after.rs#L1-6
let outcome = graph_ref.insert_next_match(tokens)
    .map_err(|e| InsertError::InternalError(format!("{e:?}")))?;

let token = outcome.token();
let already_existed = !outcome.is_expanded();
```

### Pattern 2: Context-read extracting `IndexWithPath` regardless of variant

**Before:**

```/dev/null/migration_pattern2_before.rs#L1-8
let result: Result<Result<IndexWithPath, _>, _> =
    graph.insert_or_get_complete(cursor.clone());

let IndexWithPath { index, path } = match result {
    Ok(Ok(found)) => found,
    Ok(Err(found)) => found,
    Err(ErrorReason::SingleIndex(c)) => *c,
    Err(_) => { /* fallback */ },
};
```

**After:**

```/dev/null/migration_pattern2_after.rs#L1-6
let result = graph.insert_next_match(cursor.clone());

let IndexWithPath { index, path } = match result {
    Ok(outcome) => outcome.into_result(),
    Err(ErrorReason::SingleIndex(c)) => *c,
    Err(_) => { /* fallback */ },
};
```

### Pattern 3: Tests checking Ok(Ok/Err) destructuring

**Before:**

```/dev/null/migration_pattern3_before.rs#L1-7
let result: Result<Result<IndexWithPath, _>, _> =
    graph.insert_or_get_complete(query.clone());
assert!(result.is_ok());
let inner = result.unwrap();
let IndexWithPath { index, path } = match inner {
    Ok(found) => found,
    Err(found) => found,
};
```

**After:**

```/dev/null/migration_pattern3_after.rs#L1-4
let result = graph.insert_next_match(query.clone());
assert!(result.is_ok());
let IndexWithPath { index, path } = result.unwrap().into_result();
```

### Pattern 4: Tests that distinguish Created vs Complete

**Before** (not possible — both `Complete` and `NoExpansion` were `Err`):

```/dev/null/migration_pattern4_before.rs#L1-2
// Could NOT distinguish Complete from NoExpansion
// Both returned Ok(Err(iwp))
```

**After:**

```/dev/null/migration_pattern4_after.rs#L1-5
let outcome = graph.insert_next_match(query).unwrap();
match outcome {
    InsertOutcome::Created { result, response } => { /* split+join happened */ },
    InsertOutcome::Complete { result, response } => { /* full match, query exhausted */ },
    InsertOutcome::NoExpansion { result, response } => { /* partial match, more query remains */ },
}
```

### Pattern 5: Error handling (SingleIndex) — unchanged

**Before and After (no change needed):**

```/dev/null/migration_pattern5.rs#L1-5
match result {
    Err(ErrorReason::SingleIndex(idx_path)) => {
        // Single-element query — handle as before
    },
    _ => { /* ... */ },
}
```

The `ErrorReason` type is unchanged. Only the `Ok` variant's inner type changes.

---

## 7. Validation

### Build Checks

```/dev/null/validation_commands.sh#L1-6
# Step-by-step verification (run after each step)
cargo check -p context-insert        # Steps 1-4
cargo check -p context-api            # Step 5
cargo check -p context-read           # Step 6
cargo check --workspace               # Step 9 (deprecation, full build)
cargo check --workspace 2>&1 | grep -i "deprecated"  # Should be empty
```

### Test Commands

```/dev/null/test_commands.sh#L1-12
# Unit tests per crate
cargo test -p context-insert                              # All insert tests
cargo test -p context-insert -- expanded_overlap           # Step 7b specifically
cargo test -p context-insert -- context_read_scenarios     # Step 7a specifically
cargo test -p context-read -- cursor                       # Step 8 specifically
cargo test -p context-api                                  # Step 5 specifically

# Full test suite
cargo test --workspace                                     # Everything

# Verify no remaining references to old API (outside deprecated definition)
grep -rn "insert_or_get_complete" crates/ --include="*.rs" | grep -v "#\[deprecated" | grep -v "/// " | grep -v "//!"
# Expected: only the deprecated method definitions themselves + allow(deprecated) attributes
```

### Manual Verification Checklist

- [ ] `InsertOutcome` has all three variants with `result` and `response` fields
- [ ] `insert_next_match` compiles on `InsertCtx` and `ToInsertCtx`
- [ ] `context-api` `insert_first_match` uses `insert_next_match` (no turbofish)
- [ ] `context-api` `insert_sequence` uses `insert_next_match` (no turbofish)
- [ ] `context-read` `ExpansionCtx::new` uses `insert_next_match`
- [ ] All 12 cursor tests pass
- [ ] All 8 expanded_overlap tests pass
- [ ] All 2 context_read_scenarios tests pass
- [ ] `insert_or_get_complete` is marked `#[deprecated]`
- [ ] Zero deprecation warnings in `cargo check --workspace`
- [ ] `InsertOutcome` is re-exported from `context_insert` crate root

---

## 8. Risks & Mitigations

| # | Risk | Likelihood | Impact | Mitigation |
|---|------|-----------|--------|------------|
| R1 | `Response::clone()` is expensive for large graphs | Low | Low | Clone only on `Created` path (already expensive). Benchmark if concerned. |
| R2 | `insert_init` generic `R` parameter conflicts with always-`IndexWithPath` return | Medium | Medium | Extract path before `insert_init`, convert token with `.into()` after. Tested in Step 2. |
| R3 | Test assertions break because `Ok(Ok(..))` / `Ok(Err(..))` patterns change | High (certain) | Low | Systematic find-replace in Steps 7-8. Each test is mechanically migrated. |
| R4 | Downstream crates outside this repo call `insert_or_get_complete` | Low | Medium | `#[deprecated]` attribute with clear migration message. Old method still works. |
| R5 | `Response` fields are accessed by value (moved out) in tests | Medium | Low | `InsertOutcome` provides `response()` (borrow) and accessor methods. Tests can destructure if needed. |
| R6 | `NoExpansion` semantics misunderstood — callers treat it as error | Medium | Medium | Clear doc comments on enum + `is_no_expansion()` helper. Migration guide Pattern 1 shows `already_existed = !outcome.is_expanded()`. |
| R7 | Thread-local insert visualization state not reset properly on new path | Low | Low | `insert_next_match_impl` delegates to same `insert_init` which already calls `reset_step_counter()`. No change needed. |

---

## Appendix A: Complete `insert_next_match_impl` Implementation

This is the full method body for reference, incorporating all design decisions from Section 5:

```/dev/null/insert_next_match_impl_full.rs#L1-56
fn insert_next_match_impl(
    &mut self,
    searchable: impl Searchable<InsertTraversal>,
) -> Result<InsertOutcome, ErrorState> {
    match searchable.search(self.graph.clone()) {
        Ok(result) => {
            if result.is_entire_root() && result.query_exhausted() {
                // ── Complete ──
                // Query fully matched an existing token. No insertion needed.
                let query_path = result.query_cursor().path().clone();
                let root_token = result.root_token();
                let response = result; // MOVE — no clone

                Ok(InsertOutcome::Complete {
                    result: IndexWithPath {
                        index: root_token,
                        path: query_path,
                    },
                    response,
                })
            } else if result.is_entire_root() && !result.query_exhausted() {
                // ── NoExpansion ──
                // Found a complete token at start, but query extends beyond.
                let root_token = result.root_token();
                let query_path = result.query_cursor().path().clone();
                let response = result; // MOVE — no clone

                Ok(InsertOutcome::NoExpansion {
                    result: IndexWithPath {
                        index: root_token,
                        path: query_path,
                    },
                    response,
                })
            } else {
                // ── Created ──
                // Partial match — need to insert via split+join.
                let response = result.clone(); // CLONE — needed because InitInterval consumes result
                let query_path = result.query_cursor().path().clone();
                let extract = <R::Extract as ResultExtraction>::extract_from(&result);
                let init = InitInterval::from(result); // MOVE — consumes result

                let new_token: R = self.insert_init(extract, init)?;
                let token: Token = new_token.into();

                Ok(InsertOutcome::Created {
                    result: IndexWithPath {
                        index: token,
                        path: query_path,
                    },
                    response,
                })
            }
        },
        Err(err) => Err(err),
    }
}
```

---

## Appendix B: Mapping Table — Old API → New API

| Old expression | New expression | Notes |
|---------------|---------------|-------|
| `graph.insert_or_get_complete(q)` | `graph.insert_next_match(q)` | Direct rename |
| `Result<Result<R, R::Error>, ErrorReason>` | `Result<InsertOutcome, ErrorReason>` | Flat enum |
| `Ok(Ok(iwp))` — newly created | `Ok(InsertOutcome::Created { result, response })` | Was `Ok` for `Token`, confusing |
| `Ok(Err(iwp))` — already existed (complete) | `Ok(InsertOutcome::Complete { result, response })` | Was `Err` for `IndexWithPath` |
| `Ok(Err(iwp))` — already existed (no expansion) | `Ok(InsertOutcome::NoExpansion { result, response })` | **NEW distinction** — previously conflated with Complete |
| `Err(reason)` — hard error | `Err(reason)` — unchanged | Same `ErrorReason` type |
| `<_ as ToInsertCtx<IndexWithPath>>::insert_or_get_complete(&g, q)` | `g.insert_next_match(q)` | No turbofish needed |
| `match inner { Ok(f) => f, Err(f) => f }` | `outcome.into_result()` | One-liner replacement |
| N/A (not possible) | `outcome.response()` | **NEW** — access search response |
| N/A (not possible) | `outcome.is_no_expansion()` | **NEW** — distinguish from Complete |