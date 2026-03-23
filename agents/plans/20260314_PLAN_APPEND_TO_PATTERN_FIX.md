---
tags: `#plan` `#context-trace` `#context-read` `#bug-fix` `#refactor` `#safety`
summary: Split `append_to_pattern` into `extend_root_pattern` (safe, creates new vertex) and `append_to_owned_pattern` (in-place, debug-asserted), deprecate the original, update 3 RootManager call sites.
status: 📋
phase: 2-design
parent: 20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md
decision: D3 (Q3, Q17)
priority: 2 (second design artifact — foundation fix, independent of other plans)
---

# Plan: Split `append_to_pattern` into Safe + Owned Variants

**Date:** 2026-03-14
**Scope:** Small–Medium (2 files, ~120 lines new code, ~30 lines migration)
**Crates:** `context-trace`, `context-read`

---

## Table of Contents

1. [Objective](#objective)
2. [Context](#context)
3. [Files Affected](#files-affected)
4. [Analysis](#analysis)
5. [Execution Steps](#execution-steps)
6. [`extend_root_pattern` Implementation Design](#extend_root_pattern-implementation-design)
7. [Migration Guide](#migration-guide)
8. [Validation](#validation)
9. [Risks & Mitigations](#risks--mitigations)

---

## Objective

Split the destructively-mutating `append_to_pattern` into two explicit functions — `extend_root_pattern` (safe, non-mutating) and `append_to_owned_pattern` (in-place, invariant-checked) — to eliminate a class of silent vertex-corruption bugs.

---

## Context

This plan is the **second-priority design artifact** for the [Context-Read UX Improvement](20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md) project (Phase 2: Design). It implements **Decision D3** from interview questions Q3 and Q17:

> **D3:** Split `append_to_pattern` → `extend_root_pattern` (safe) + `append_to_owned_pattern` (in-place)

The fix is **independent** of the other Phase 2 plans (`PLAN_INSERT_NEXT_MATCH`, `PLAN_CLI_READ_UX`, etc.) and can be implemented in parallel as a foundation fix in Phase 3a.

---

## Files Affected

| File | Change Type | Description |
|------|-------------|-------------|
| `crates/context-trace/src/graph/insert/parents.rs` | **Modified** | Add `extend_root_pattern`, add `append_to_owned_pattern`, deprecate `append_to_pattern` |
| `crates/context-read/src/context/root.rs` | **Modified** | Update 3 call sites in `RootManager` to use `append_to_owned_pattern` |

---

## Analysis

### Current Implementation

The function `append_to_pattern` lives at `crates/context-trace/src/graph/insert/parents.rs` (L92–138) on `Hypergraph<G>`:

```rust
pub fn append_to_pattern(
    &self,
    parent: impl ToToken,
    pattern_id: PatternId,
    new: impl IntoIterator<Item = impl ToToken>,
) -> Token
```

It performs **4 in-place mutations**:

| # | Mutation | Target | Risk |
|---|----------|--------|------|
| 1 | `node.get_parent_mut(...).width += width` | Each existing child's parent-entry `.width` | Corrupts width for ALL patterns referencing this child |
| 2 | `pattern.extend(new.iter())` | Parent vertex's child pattern | Changes pattern globally |
| 3 | `*vertex.width_mut() += width.0` | Parent vertex's width | All tokens pointing to this vertex see wrong width |
| 4 | `node.add_parent(ChildLocation::new(...))` | Each new token's vertex | Adds back-pointer (this one is correct) |

### Why In-Place Mutation is Dangerous: Concrete Example

Consider a graph where vertex `V5` has:
- Pattern 0: `[V1, V2]` (width = 4)
- Pattern 1: `[V3, V4]` (width = 4)
- Parents: `{ V10 → ChildLocation { pattern_id: 0, offset: 2 } }`

And separately, vertex `V8` has:
- Pattern 0: `[V1, V2]` (width = 4)
- Parents: `{}` (no parents, one pattern — passes the guard)

Now a caller does `append_to_pattern(V8, pid_0, [V6])` where `V6.width = 2`:

**Mutation 1** bumps `V1`'s parent-entry for `V8` by +2 and `V2`'s parent-entry for `V8` by +2. This is correct for `V8`.

But **`V1` and `V2` are shared** — they also appear in `V5`'s Pattern 0. If any code later queries `V1.parents()` expecting to find the original width for `V5`, the width for `V8`'s entry is now 6 instead of 4. The parent entries are per-parent so this specific case is safe.

**However**, the real danger is **Mutation 3**: `V8`'s vertex width changes from 4 to 6. Any existing `Token { vertex_index: V8, width: 4 }` held elsewhere is now **stale** — the vertex says 6 but the token says 4. The function returns a *new* `Token` with the updated width, but any previously-created tokens referencing `V8` are silently wrong.

More critically, if `V8` *did* have parents (violating the implicit contract), those parents' pattern entries would still reference `V8` with the old width, creating an inconsistency in the graph structure. The function **does not check** for this — it relies entirely on callers to enforce the invariant.

### Current Call Sites (All in `RootManager`)

All 3 call sites are in `crates/context-read/src/context/root.rs`:

**1. `append_pattern`** (L40–43):
```rust
let vertex = (*root).vertex(&self.graph);
*root = if vertex.child_patterns().len() == 1
    && vertex.parents().is_empty()
{
    let (&pid, _) = vertex.expect_any_child_pattern();
    self.graph.append_to_pattern(*root, pid, new)
} ...
```

**2. `append_token`** (L73–78):
```rust
let vertex = (*root).vertex(&self.graph);
*root = if token.vertex_index() != root.vertex_index()
    && vertex.child_patterns().len() == 1
    && vertex.parents().is_empty()
{
    let (&pid, _) = vertex.expect_any_child_pattern();
    self.graph.append_to_pattern(*root, pid, token)
} ...
```

**3. `append_collapsed`** (L228–233):
```rust
let can_extend = vertex.child_patterns().len() == 1
    && vertex.parents().is_empty()
    && !append_pattern
        .iter()
        .any(|t| t.vertex_index() == root.vertex_index());
// ...
if can_extend {
    let (&pid, _) = vertex.expect_any_child_pattern();
    self.graph.append_to_pattern(root, pid, append_pattern)
} ...
```

All three guard with `parents().is_empty() && child_patterns().len() == 1`, and all three assign the result back to `*root` / `self.root`. The invariant is enforced by callers — the function itself is a footgun.

---

## Execution Steps

### Step 1: Add `extend_root_pattern` to `Hypergraph` in `parents.rs`

**File:** `crates/context-trace/src/graph/insert/parents.rs`
**Location:** After the existing `append_to_pattern` impl block (after L138)

This is the **safe variant** that creates a new vertex instead of mutating the parent in-place. See [Implementation Design](#extend_root_pattern-implementation-design) for full details.

```rust
impl<G: GraphKind> Hypergraph<G> {
    /// Extend a pattern by creating a NEW vertex containing the existing
    /// pattern's children followed by the new tokens.
    ///
    /// This is the **safe** variant — it never mutates the parent vertex.
    /// The original vertex remains unchanged, so any other patterns or
    /// tokens referencing it are unaffected.
    ///
    /// # Returns
    /// A new `Token` for the newly-created vertex. The caller must update
    /// any root references to point to this new token.
    pub fn extend_root_pattern(
        &self,
        parent: impl ToToken,
        pattern_id: PatternId,
        new: impl IntoIterator<Item = impl ToToken>,
    ) -> Token {
        let new: Vec<_> = new.into_iter().map(|c| c.to_token()).collect();
        if new.is_empty() {
            return parent.to_token();
        }

        // Read the existing pattern from the parent vertex (non-mutating)
        let existing_pattern = self
            .with_vertex(parent.vertex_index(), |vertex| {
                vertex.expect_child_pattern(&pattern_id).clone()
            })
            .expect("Parent vertex should exist");

        // Build new_pattern = existing_pattern ++ new_tokens
        let mut combined: Vec<Token> = existing_pattern;
        combined.extend(new.iter());

        // Insert as a brand-new vertex via insert_pattern
        self.insert_pattern(combined)
    }
}
```

- [ ] Add the function
- [ ] Add doc comment explaining safety properties
- [ ] Verify: `cargo check -p context-trace`

### Step 2: Add `append_to_owned_pattern` to `Hypergraph` in `parents.rs`

**File:** `crates/context-trace/src/graph/insert/parents.rs`
**Location:** In the same `#[allow(dead_code)]` impl block, adjacent to the deprecated `append_to_pattern`

This is the **in-place variant** — identical to the current `append_to_pattern` body, but with a `debug_assert!` enforcing the safety invariant.

```rust
impl<G: GraphKind> Hypergraph<G> {
    /// Append tokens to an existing pattern **in-place**.
    ///
    /// # Safety Invariant
    /// The parent vertex MUST be "owned" — no parents, exactly one child
    /// pattern. This is enforced via `debug_assert!` in debug builds.
    ///
    /// If the vertex has parents or multiple patterns, use
    /// [`extend_root_pattern`] instead, which creates a new vertex.
    ///
    /// # Panics (debug builds)
    /// Panics if the parent vertex has parents or more than one child pattern.
    pub fn append_to_owned_pattern(
        &self,
        parent: impl ToToken,
        pattern_id: PatternId,
        new: impl IntoIterator<Item = impl ToToken>,
    ) -> Token {
        let new: Vec<_> = new.into_iter().map(|c| c.to_token()).collect();
        if new.is_empty() {
            return parent.to_token();
        }

        // Debug-assert the safety invariant
        #[cfg(debug_assertions)]
        {
            self.with_vertex(parent.vertex_index(), |vertex| {
                debug_assert!(
                    vertex.parents().is_empty(),
                    "append_to_owned_pattern called on vertex {:?} which has {} parents. \
                     Use extend_root_pattern for shared vertices.",
                    parent.vertex_index(),
                    vertex.parents().len()
                );
                debug_assert!(
                    vertex.child_patterns().len() == 1,
                    "append_to_owned_pattern called on vertex {:?} which has {} child patterns. \
                     Use extend_root_pattern for multi-pattern vertices.",
                    parent.vertex_index(),
                    vertex.child_patterns().len()
                );
            })
            .expect("Parent vertex should exist");
        }

        let width = pattern_width(&new);

        // Get pattern and update child parents
        let pattern = self
            .with_vertex(parent.vertex_index(), |vertex| {
                vertex.expect_child_pattern(&pattern_id).clone()
            })
            .expect("Parent vertex should exist");

        for c in pattern.into_iter().collect::<HashSet<_>>() {
            self.with_vertex_mut(c.vertex_index(), |node| {
                node.get_parent_mut(parent.vertex_index()).unwrap().width +=
                    width;
            })
            .expect("Child vertex should exist");
        }

        // Update parent vertex
        let (offset, final_width) = self
            .with_vertex_mut(parent.vertex_index(), |vertex| {
                let pattern = vertex.expect_child_pattern_mut(&pattern_id);
                let offset = pattern.len();
                pattern.extend(new.iter());
                *vertex.width_mut() += width.0;
                (offset, vertex.width())
            })
            .expect("Parent vertex should exist");

        let parent = Token::new(parent.vertex_index(), final_width);
        self.add_pattern_parent(parent, new, pattern_id, offset);
        parent
    }
}
```

- [ ] Add the function
- [ ] Add doc comment with safety invariant
- [ ] Add `debug_assert!` for `parents().is_empty()` and `child_patterns().len() == 1`
- [ ] Verify: `cargo check -p context-trace`

### Step 3: Deprecate `append_to_pattern`

**File:** `crates/context-trace/src/graph/insert/parents.rs`
**Location:** The existing `append_to_pattern` function (L92)

Add a `#[deprecated]` attribute to the existing function:

```rust
#[deprecated(
    note = "Use `extend_root_pattern` (safe, creates new vertex) or \
            `append_to_owned_pattern` (in-place, requires owned vertex) instead."
)]
pub fn append_to_pattern(
    &self,
    parent: impl ToToken,
    pattern_id: PatternId,
    new: impl IntoIterator<Item = impl ToToken>,
) -> Token {
    // Body unchanged — delegate to in-place variant for backward compat
    self.append_to_owned_pattern(parent, pattern_id, new)
}
```

The body is replaced with a delegation to `append_to_owned_pattern` so the `debug_assert!` is also active through the deprecated path.

- [ ] Add `#[deprecated]` attribute
- [ ] Replace body with delegation to `append_to_owned_pattern`
- [ ] Verify: `cargo check -p context-trace` (expect deprecation warnings, no errors)

### Step 4: Update `RootManager::append_pattern` call site

**File:** `crates/context-read/src/context/root.rs`
**Location:** L43

The caller already guards with `child_patterns().len() == 1 && parents().is_empty()`, so `append_to_owned_pattern` is correct.

```rust
// Before:
self.graph.append_to_pattern(*root, pid, new)

// After:
self.graph.append_to_owned_pattern(*root, pid, new)
```

- [ ] Replace `append_to_pattern` with `append_to_owned_pattern`
- [ ] Verify: `cargo check -p context-read` (no deprecation warning at this site)

### Step 5: Update `RootManager::append_token` call site

**File:** `crates/context-read/src/context/root.rs`
**Location:** L78

Same guard pattern — `append_to_owned_pattern` is safe here.

```rust
// Before:
self.graph.append_to_pattern(*root, pid, token)

// After:
self.graph.append_to_owned_pattern(*root, pid, token)
```

- [ ] Replace `append_to_pattern` with `append_to_owned_pattern`
- [ ] Verify: `cargo check -p context-read` (no deprecation warning at this site)

### Step 6: Update `RootManager::append_collapsed` call site

**File:** `crates/context-read/src/context/root.rs`
**Location:** L233

Same guard pattern — `append_to_owned_pattern` is safe here. The additional guard `!append_pattern.iter().any(|t| t.vertex_index() == root.vertex_index())` prevents self-referential patterns but isn't part of the ownership invariant.

```rust
// Before:
self.graph.append_to_pattern(root, pid, append_pattern)

// After:
self.graph.append_to_owned_pattern(root, pid, append_pattern)
```

- [ ] Replace `append_to_pattern` with `append_to_owned_pattern`
- [ ] Verify: `cargo check -p context-read` (no deprecation warning at this site)

### Step 7: Add unit tests for both new functions

**File:** `crates/context-trace/src/graph/insert/parents.rs` (or a new test module)

Two test categories:

#### Test 7a: `extend_root_pattern` does NOT mutate the original vertex

```rust
#[test]
fn extend_root_pattern_preserves_original_vertex() {
    let graph = Hypergraph::default();

    // Create atoms: a, b, c
    let a = graph.insert_atom('a');
    let b = graph.insert_atom('b');
    let c = graph.insert_atom('c');

    // Create pattern [a, b] → vertex "ab"
    let ab = graph.insert_pattern(vec![a, b]);
    let ab_width_before = ab.width();
    let (ab_pid, _) = graph
        .with_vertex(ab.vertex_index(), |v| {
            let (&pid, pat) = v.expect_any_child_pattern();
            (pid, pat.clone())
        })
        .unwrap();

    // Extend: create new vertex [a, b, c] without mutating "ab"
    let abc = graph.extend_root_pattern(ab, ab_pid, vec![c]);

    // Original vertex "ab" is unchanged
    let ab_width_after = graph
        .with_vertex(ab.vertex_index(), |v| v.width())
        .unwrap();
    assert_eq!(ab_width_before, ab_width_after, "Original vertex width must not change");

    // New vertex "abc" is different
    assert_ne!(ab.vertex_index(), abc.vertex_index(), "Must create a new vertex");
    assert_eq!(abc.width().0, ab_width_before.0 + c.width().0, "New vertex has combined width");

    // New vertex has pattern [a, b, c]
    let abc_pattern = graph
        .with_vertex(abc.vertex_index(), |v| {
            let (_, pat) = v.expect_any_child_pattern();
            pat.clone()
        })
        .unwrap();
    assert_eq!(abc_pattern.len(), 3);
    assert_eq!(abc_pattern[0].vertex_index(), a.vertex_index());
    assert_eq!(abc_pattern[1].vertex_index(), b.vertex_index());
    assert_eq!(abc_pattern[2].vertex_index(), c.vertex_index());
}
```

#### Test 7b: `append_to_owned_pattern` panics if vertex has parents (debug mode)

```rust
#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "which has")]
fn append_to_owned_pattern_panics_on_shared_vertex() {
    let graph = Hypergraph::default();

    // Create atoms: a, b, c, d
    let a = graph.insert_atom('a');
    let b = graph.insert_atom('b');
    let c = graph.insert_atom('c');
    let d = graph.insert_atom('d');

    // Create [a, b] → "ab" (has no parents yet)
    let ab = graph.insert_pattern(vec![a, b]);

    // Create [ab, c] → "abc" — this gives "ab" a parent!
    let _abc = graph.insert_pattern(vec![ab, c]);

    // Now "ab" has parents. Attempting append_to_owned_pattern should panic.
    let (pid, _) = graph
        .with_vertex(ab.vertex_index(), |v| {
            let (&pid, pat) = v.expect_any_child_pattern();
            (pid, pat.clone())
        })
        .unwrap();

    // This should panic because ab has parents
    let _ = graph.append_to_owned_pattern(ab, pid, vec![d]);
}
```

#### Test 7c: `append_to_owned_pattern` succeeds on truly owned vertex

```rust
#[test]
fn append_to_owned_pattern_succeeds_on_owned_vertex() {
    let graph = Hypergraph::default();

    let a = graph.insert_atom('a');
    let b = graph.insert_atom('b');
    let c = graph.insert_atom('c');

    // Create [a, b] → "ab" with no parents (freshly created root)
    let ab = graph.insert_pattern(vec![a, b]);

    let (pid, _) = graph
        .with_vertex(ab.vertex_index(), |v| {
            let (&pid, pat) = v.expect_any_child_pattern();
            (pid, pat.clone())
        })
        .unwrap();

    // Append c in-place — vertex is owned (no parents, one pattern)
    let abc = graph.append_to_owned_pattern(ab, pid, vec![c]);

    // Same vertex index (mutated in place)
    assert_eq!(ab.vertex_index(), abc.vertex_index(), "In-place mutation keeps same vertex");

    // Width updated
    assert_eq!(abc.width().0, a.width().0 + b.width().0 + c.width().0);
}
```

#### Test 7d: `extend_root_pattern` with empty new tokens is a no-op

```rust
#[test]
fn extend_root_pattern_empty_new_returns_parent() {
    let graph = Hypergraph::default();

    let a = graph.insert_atom('a');
    let b = graph.insert_atom('b');
    let ab = graph.insert_pattern(vec![a, b]);

    let (pid, _) = graph
        .with_vertex(ab.vertex_index(), |v| {
            let (&pid, pat) = v.expect_any_child_pattern();
            (pid, pat.clone())
        })
        .unwrap();

    let result = graph.extend_root_pattern(ab, pid, Vec::<Token>::new());
    assert_eq!(result.vertex_index(), ab.vertex_index(), "Empty extend returns parent unchanged");
}
```

- [ ] Add all 4 tests
- [ ] Verify: `cargo test -p context-trace` — all pass
- [ ] Verify: `cargo test -p context-trace` with `--release` — Test 7b is skipped (cfg debug_assertions), others pass

---

## `extend_root_pattern` Implementation Design

The new safe function follows a **read-then-create** pattern — no mutation of the source vertex.

### Algorithm

```
extend_root_pattern(parent, pattern_id, new) -> Token:
    1. new_tokens = collect(new)
    2. if new_tokens is empty → return parent.to_token()
    3. existing_pattern = parent.vertex.child_patterns[pattern_id].clone()   // READ ONLY
    4. combined = existing_pattern ++ new_tokens
    5. result = self.insert_pattern(combined)                                // NEW VERTEX
    6. return result
```

### Key Properties

| Property | Value |
|----------|-------|
| **Mutates parent vertex?** | ❌ No |
| **Mutates existing children?** | ❌ No (no width bumps on existing child parent-entries) |
| **Creates new vertex?** | ✅ Yes — via `insert_pattern` |
| **New vertex has parents?** | ❌ No (freshly created, no one references it yet) |
| **Return type** | `Token` — new vertex's token with correct width |
| **Caller responsibility** | Must update their root reference (`*root = result`) |

### Why `insert_pattern` is Sufficient

`insert_pattern` (in `crates/context-trace/src/graph/insert/pattern.rs` L57–64) already:
1. Computes the combined width from all child tokens
2. Allocates a new `VertexIndex`
3. Creates `VertexData` with the pattern
4. Adds parent back-pointers from each child to the new vertex

This is exactly what we need — the new vertex will correctly reference `[a, b, c]` as children, and each of `a`, `b`, `c` will have a parent-entry pointing to the new vertex. The **original** parent vertex (`[a, b]`) is untouched — its children (`a`, `b`) retain their existing parent-entries pointing to it.

### Deduplication Behavior

`insert_pattern` may **deduplicate** — if a vertex with pattern `[a, b, c]` already exists, it returns the existing vertex's token rather than creating a duplicate. This is correct behavior: the graph maintains the invariant that each unique pattern exists at most once.

---

## Migration Guide

### Before / After for Each Call Site

#### Call Site 1: `RootManager::append_pattern` (root.rs L40–43)

**Before:**
```rust
*root = if vertex.child_patterns().len() == 1
    && vertex.parents().is_empty()
{
    let (&pid, _) = vertex.expect_any_child_pattern();
    self.graph.append_to_pattern(*root, pid, new)
} else {
    let new = new.into_pattern();
    self.graph.insert_pattern([&[*root], new.as_slice()].concat())
};
```

**After:**
```rust
*root = if vertex.child_patterns().len() == 1
    && vertex.parents().is_empty()
{
    let (&pid, _) = vertex.expect_any_child_pattern();
    self.graph.append_to_owned_pattern(*root, pid, new)
} else {
    let new = new.into_pattern();
    self.graph.insert_pattern([&[*root], new.as_slice()].concat())
};
```

**Rationale:** Guards already verify ownership invariant. Using `append_to_owned_pattern` adds the `debug_assert!` as a safety net. The in-place mutation is an optimization (avoids creating a new vertex for the root being built up).

---

#### Call Site 2: `RootManager::append_token` (root.rs L73–78)

**Before:**
```rust
*root = if token.vertex_index() != root.vertex_index()
    && vertex.child_patterns().len() == 1
    && vertex.parents().is_empty()
{
    let (&pid, _) = vertex.expect_any_child_pattern();
    self.graph.append_to_pattern(*root, pid, token)
} else {
    self.graph.insert_pattern(vec![*root, token])
};
```

**After:**
```rust
*root = if token.vertex_index() != root.vertex_index()
    && vertex.child_patterns().len() == 1
    && vertex.parents().is_empty()
{
    let (&pid, _) = vertex.expect_any_child_pattern();
    self.graph.append_to_owned_pattern(*root, pid, token)
} else {
    self.graph.insert_pattern(vec![*root, token])
};
```

**Rationale:** Same ownership guard. The additional `token.vertex_index() != root.vertex_index()` guard prevents self-referential patterns but is orthogonal to the ownership invariant.

---

#### Call Site 3: `RootManager::append_collapsed` (root.rs L228–233)

**Before:**
```rust
self.root = Some(if can_extend {
    debug!("Extending root in place");
    let (&pid, _) = vertex.expect_any_child_pattern();
    self.graph.append_to_pattern(root, pid, append_pattern)
} else {
    debug!("Creating new combined root");
    let combined: Vec<Token> = std::iter::once(root)
        .chain(append_pattern.iter().cloned())
        .collect();
    self.graph.insert_pattern(combined)
});
```

**After:**
```rust
self.root = Some(if can_extend {
    debug!("Extending root in place");
    let (&pid, _) = vertex.expect_any_child_pattern();
    self.graph.append_to_owned_pattern(root, pid, append_pattern)
} else {
    debug!("Creating new combined root");
    let combined: Vec<Token> = std::iter::once(root)
        .chain(append_pattern.iter().cloned())
        .collect();
    self.graph.insert_pattern(combined)
});
```

**Rationale:** `can_extend` already checks `child_patterns().len() == 1 && parents().is_empty()`. The additional `!append_pattern.iter().any(|t| t.vertex_index() == root.vertex_index())` prevents cycles but is orthogonal to ownership.

---

### Future Migration: Switching to `extend_root_pattern`

All three call sites *could* use `extend_root_pattern` instead for maximum safety. The trade-off:

| Variant | Safety | Performance | Vertex Identity |
|---------|--------|-------------|-----------------|
| `append_to_owned_pattern` | Safe when guarded (debug-asserted) | No allocation, in-place mutation | Same `VertexIndex` |
| `extend_root_pattern` | Always safe | Allocates new vertex | **Different** `VertexIndex` |

Since all three call sites assign the result back to `*root` / `self.root`, switching to `extend_root_pattern` would work correctly — the root reference is always updated. However, the in-place variant avoids vertex churn during the read algorithm's tight loop, so we use `append_to_owned_pattern` for the initial migration. The safe variant is available for any future call site where ownership cannot be guaranteed.

---

## Validation

### Automated Checks

```bash
# Step 1: Type-check both crates
cargo check -p context-trace
cargo check -p context-read

# Step 2: Run context-trace tests (includes new unit tests)
cargo test -p context-trace

# Step 3: Run context-read tests (validates call site migration)
cargo test -p context-read

# Step 4: Verify no unexpected deprecation warnings remain
cargo check -p context-read 2>&1 | grep -i "deprecat"
# Expected: no output (all call sites migrated)

# Step 5: Full workspace check (no other crates use append_to_pattern)
cargo check --workspace 2>&1 | grep "append_to_pattern"
# Expected: only the #[deprecated] definition itself, no call sites
```

### Manual Checks

- [ ] `extend_root_pattern` doc comment mentions safety properties
- [ ] `append_to_owned_pattern` doc comment warns about invariant
- [ ] `#[deprecated]` note text points to both new functions
- [ ] All 4 unit tests have descriptive names and comments
- [ ] No `#[allow(deprecated)]` added at any call site (all migrated)

---

## Risks & Mitigations

| # | Risk | Likelihood | Impact | Mitigation |
|---|------|-----------|--------|------------|
| R1 | `extend_root_pattern` creates a **new** `VertexIndex` — callers must update their root reference | Low | High | All 3 current call sites already do `*root = result` / `self.root = Some(result)`. This is safe. The risk is for **future** callers who might forget. The doc comment explicitly warns about this. |
| R2 | `debug_assert!` in `append_to_owned_pattern` won't fire in release builds | Med | Low | The invariant is a correctness check, not a safety check. Release builds skip it for performance. The guards at call sites provide the real protection. The assert catches bugs during development/testing. |
| R3 | `insert_pattern` deduplication may return an **existing** vertex if the combined pattern already exists | Low | Low | This is correct behavior — the graph's deduplication invariant is maintained. The caller still gets a valid token. |
| R4 | Performance regression from `extend_root_pattern` allocating new vertices | N/A (not used yet) | Low | We use `append_to_owned_pattern` (in-place) for all current call sites. `extend_root_pattern` exists for future callers where safety > performance. |
| R5 | Other crates may call `append_to_pattern` directly | Low | Low | The `#[deprecated]` attribute produces compiler warnings. A workspace-wide `cargo check` in validation Step 5 confirms no other call sites exist. |

---

## Notes

### Design Decisions Captured

- **Why not just add the assert to `append_to_pattern`?** — The function name doesn't communicate the ownership requirement. Two explicit functions with clear names (`owned` vs `root`) make the contract visible at the call site without reading docs.
- **Why keep the in-place variant at all?** — Performance. The read algorithm calls this in a tight loop while building up the root token. Creating a new vertex for every appended token would cause unnecessary vertex churn. The ownership invariant is easy to verify at the call site.
- **Why `extend_root_pattern` uses `insert_pattern` instead of manual vertex construction?** — `insert_pattern` already handles width calculation, vertex allocation, and parent back-pointer setup. Reimplementing this would be error-prone and violate DRY. The only "cost" is that `insert_pattern` may deduplicate, which is actually desirable.

### Relationship to Other Plans

- **`PLAN_INSERT_NEXT_MATCH`**: Independent. That plan modifies `context-insert`; this plan modifies `context-trace` and `context-read`. No conflicts.
- **`PLAN_CLI_READ_UX`**: Depends on this plan being complete (the read pipeline must be correct before exposing it in the CLI).
- **`PLAN_INTEGRATION_TESTS`**: Tests in that plan will exercise the corrected code paths.

### Questions Resolved

All questions were resolved in interview rounds Q3 and Q17 of the parent plan. No open questions remain.