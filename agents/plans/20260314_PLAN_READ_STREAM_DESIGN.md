---
tags: `#plan` `#context-read` `#stream` `#iterator` `#lazy` `#async` `#design`
summary: Stream/iterator design for the read pipeline — lazy atom resolution, `from_reader` adapter, `ReadSequenceIter`, future async Stream pattern, and multi-band debug_assert.
status: design
parent: 20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md
depends_on: PLAN_INSERT_NEXT_MATCH
date: 2026-03-14
---

# Plan: Read Pipeline Stream/Iterator Design

**Date:** 2026-03-14
**Scope:** Medium (single crate, iterator redesign, Cargo.toml cleanup)
**Crate:** `context-read` (touches `context-trace` for atom resolution)

---

## Table of Contents

1. [Objective](#objective)
2. [Context](#context)
3. [Files Affected](#files-affected)
4. [Analysis](#analysis)
5. [Execution Steps](#execution-steps)
6. [Lazy Atom Resolution Design](#lazy-atom-resolution-design)
7. [Future Async Stream Pattern](#future-async-stream-pattern)
8. [Validation](#validation)
9. [Risks & Mitigations](#risks--mitigations)

---

## Objective

Design the lazy atom resolution and stream consumer pattern for the read pipeline. The current implementation eagerly resolves all characters to `NewAtomIndex` values upfront (`Vec<NewAtomIndex>`). This plan converts the pipeline to a **lazy, one-pass stream consumer** where each character is resolved to an atom at consumption time, supports `impl Read` input sources, and documents the future async `Stream` wrapper pattern without implementing it.

**Design decisions referenced:**
- **D9:** One-pass stream consumer, sync iterator core, future async wrapper
- **D10:** Lazy atom resolution (char status resolved at consumption time)
- **D8:** BandState as-is + `debug_assert` for multi-band invariant

---

## Context

### Parent Plan

This plan is a sub-plan of [`20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md`](20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md) — the multi-phase context-read UX improvement plan. It covers the stream/iterator architecture track.

### Dependency

Depends on **PLAN_INSERT_NEXT_MATCH** — the `insert_next_match` API rename/enhancement in `context-insert`. The expansion loop (`ExpandCtx`) delegates to this method. The stream design wraps the expansion pipeline but does not change its internals. This plan can proceed in parallel for steps 1–5 and step 7; step 6 (async sketch) and step 7 (`debug_assert`) are independent.

### Current Architecture

The read pipeline is a **synchronous iterator chain** where all atoms are resolved eagerly before iteration begins:

```
                        ┌─────────────────────────────────────────────────────┐
                        │                    CURRENT FLOW                     │
                        └─────────────────────────────────────────────────────┘

  "hello"                    Vec<NewAtomIndex>                   SegmentIter
 ─────────►  new_atom_indices()  ─────────►  SegmentIter::new()  ─────────►
  (string)   [EAGER: resolves ALL         (wraps Vec into           yields
              chars in one pass]           Peekable<IntoIter>)    NextSegment
                                                                  { unknown,
                                                                    known }

ReadCtx::read_sequence()
  → for each NextSegment:
    ├─ RootManager::append_pattern(unknown)
    └─ BlockExpansionCtx::new(root, known)
         → ExpansionCtx::new(graph, cursor, band)
              → graph.insert_or_get_complete(cursor)  // bootstraps first token
         → BlockExpansionCtx::process()
              → ExpansionCtx::next() [loop]
                   ├─ ExpandCtx → PostfixIterator (BandExpandingIterator)
                   │    → ChainOp::Expansion or ChainOp::Cap
                   ├─ apply_op(Cap) → BandState::append()
                   └─ apply_op(Expansion) → ComplementBuilder::build()
                        → BandState::set_overlap() → commit
```

### Key Components

| Component | File | Role |
|-----------|------|------|
| `SegmentIter` | `segment.rs` | Partitions atom stream into alternating unknown/known chunks via `Peekable<IntoIter<NewAtomIndex>>` |
| `ToNewAtomIndices` | `segment.rs` | Trait for converting input (chars, patterns) to `NewAtomIndices` eagerly |
| `ReadCtx` | `context/mod.rs` | Top-level orchestrator, owns `RootManager` + `SegmentIter`, implements `Iterator<Item = ()>` |
| `BlockExpansionCtx` | `expansion/block.rs` | Bridge between segment processing and expansion loop, takes `RootManager` temporarily |
| `ExpansionCtx` | `expansion/mod.rs` | Core expansion loop, owns `CursorCtx` + `BandState`, yields `BandState` items |
| `ExpandCtx` | `expansion/chain/expand.rs` | Postfix expansion iterator, yields `ChainOp` (Expansion / Cap) |
| `BandState` | `expansion/chain/mod.rs` | Two-state machine: `Single { band }` → `WithOverlap { primary, overlap, link }` |
| `BandExpandingIterator` | `bands/mod.rs` | Breadth-first expanding iterator over decomposition tree, sorted by descending width |
| `RootManager` | `context/root.rs` | Tracks running root token, handles append/commit with overlap detection |
| `CursorCtx` | `expansion/cursor.rs` | Holds `HypergraphRef` + `PatternRangePath` cursor position |
| `ReadRequest` | `request.rs` | Data-oriented request wrapper (text or pattern input) |

### Atom Resolution (Current)

In `context-trace/src/graph/insert/atom.rs`:
```rust
pub fn new_atom_indices(&self, sequence: impl IntoIterator<Item = G::Atom>) -> NewAtomIndices {
    sequence.into_iter()
        .map(Atom::Element)
        .map(|t| match self.get_atom_index(t) {
            Ok(i) => NewAtomIndex::Known(i),
            Err(_) => {
                let i = self.insert_atom(t);
                NewAtomIndex::New(i.index)
            },
        })
        .collect()  // <── EAGER: collects ALL into Vec
}
```

This is called in `ReadCtx::new()`:
```rust
let new_indices = seq.to_new_atom_indices(&graph);  // eagerly resolves all atoms
Self {
    segments: SegmentIter::new(new_indices),  // wraps Vec into Peekable<IntoIter>
    root: Some(RootManager::new(graph)),
}
```

### Async Dependencies (Aspirational)

`context-read/Cargo.toml` declares these async dependencies — **none are used in source code**:

| Dependency | Version | Used? |
|-----------|---------|-------|
| `tokio` | ^1 (sync, rt, macros, time) | ❌ No |
| `tokio-stream` | ^0.1 (sync, time, io-util) | ❌ No |
| `async-std` | 1.12 | ❌ No |
| `futures` | ^0.3 | ❌ No |
| `async-trait` | ^0.1 | ❌ No |
| `async-recursion` | 1 | ❌ No |
| `pin-project-lite` | ^0.2 | ❌ No |

---

## Files Affected

### Primary Changes

| File | Change |
|------|--------|
| `crates/context-read/src/segment.rs` | Replace `SegmentIter` internals: `Peekable<IntoIter<NewAtomIndex>>` → generic `Peekable<I>` over lazy iterator |
| `crates/context-read/src/context/mod.rs` | Add `from_reader()` constructor; refactor `ReadCtx::new()` to use lazy resolution; add `ReadSequenceIter` wrapper |
| `crates/context-read/src/context/has_read_context.rs` | Update `HasReadCtx` trait to support `from_reader` |
| `crates/context-read/src/request.rs` | Add `RequestInput::Reader` variant or `from_reader` method |
| `crates/context-read/src/lib.rs` | Re-export `ReadSequenceIter` if public |
| `crates/context-read/Cargo.toml` | Remove unused async deps (`async-std`, `futures`, `async-trait`, `async-recursion`, `pin-project-lite`); keep `tokio` + `tokio-stream` behind feature flag |
| `crates/context-read/src/expansion/mod.rs` | Add `debug_assert!` for multi-band invariant in `ExpansionCtx::next()` |

### Secondary (Documentation Only)

| File | Change |
|------|--------|
| `crates/context-read/HIGH_LEVEL_GUIDE.md` | Update iterator architecture section |
| `crates/context-read/src/expansion/chain/mod.rs` | Add doc comments describing the async yield point pattern |

---

## Analysis

### Current Atom Resolution

`SegmentIter` currently receives a fully-materialized `Vec<NewAtomIndex>`:

```
Input: "abcab"
         │
         ▼
new_atom_indices("abcab")
  → 'a' → get_atom_index('a') → Err → insert_atom('a') → New(0)
  → 'b' → get_atom_index('b') → Err → insert_atom('b') → New(1)
  → 'c' → get_atom_index('c') → Err → insert_atom('c') → New(2)
  → 'a' → get_atom_index('a') → Ok(0)                  → Known(0)
  → 'b' → get_atom_index('b') → Ok(1)                  → Known(1)
         │
         ▼
  Vec [New(0), New(1), New(2), Known(0), Known(1)]
         │
         ▼
  SegmentIter { iter: vec.into_iter().peekable() }
```

**Problems:**
1. All atoms resolved before processing starts (memory: entire sequence materialized)
2. Cannot accept streaming input (`impl Read`, network socket, stdin pipe)
3. Classification is frozen at creation time — if the graph changes during processing (which it does, since we insert atoms), the classification is stale for chars that haven't been consumed yet

In practice, problem (3) doesn't currently cause bugs because `new_atom_indices` inserts unknown atoms immediately during the eager pass. But it violates the **lazy resolution principle** (D10): a char's status should be determined when it's consumed, not when the input is first scanned.

### Lazy Resolution Design

Characters should be resolved to atoms **at consumption time**, not upfront:

```
Input: "abcab" (or impl Read)
         │
         ▼  (lazy — no allocation of full Vec)
  LazyAtomIter { chars: <lazy char source>, graph: HypergraphRef }
    → .next() called by SegmentIter
      → 'a' → get_atom_index('a') → Err → insert_atom('a') → New(0)
    → .next()
      → 'b' → get_atom_index('b') → Err → insert_atom('b') → New(1)
    → .next()
      → 'c' → get_atom_index('c') → Err → insert_atom('c') → New(2)
    → .next()
      → 'a' → get_atom_index('a') → Ok(0) → Known(0)  ← resolved lazily!
    → .next()
      → 'b' → get_atom_index('b') → Ok(1) → Known(1)
    → .next()
      → None (exhausted)
```

**Key insight:** `SegmentIter` already consumes atoms **one at a time** via `Peekable`. Making it lazy means the input iterator is lazy too — the `SegmentIter` logic doesn't change, only the backing iterator type.

### Block-Based Segmentation

The unknown/known segmentation semantics remain correct under lazy resolution:

```
"abcab" with empty graph:

  SegmentIter consumes:
    next_pattern_where(is_new):  → [New(0), New(1), New(2)]  ← 'a','b','c' first seen
    next_pattern_where(is_known): → [Known(0), Known(1)]     ← 'a','b' now exist
    → NextSegment { unknown: [a,b,c], known: [a,b] }

  SegmentIter consumes again:
    → None (exhausted)
```

Unknown atoms **close** a known block. This is correct: an unknown atom means no existing patterns can span across it, so the expansion algorithm starts fresh after unknown segments.

### Generic SegmentIter Design

The current `SegmentIter`:
```rust
pub(crate) struct SegmentIter {
    iter: std::iter::Peekable<std::vec::IntoIter<NewAtomIndex>>,
}
```

Proposed:
```rust
pub(crate) struct SegmentIter<I: Iterator<Item = NewAtomIndex>> {
    iter: std::iter::Peekable<I>,
}
```

This is a **backward-compatible generalization**. Existing call sites using `Vec<NewAtomIndex>` produce `SegmentIter<std::vec::IntoIter<NewAtomIndex>>`. New lazy sources produce `SegmentIter<LazyAtomIter<C>>`.

---

## Execution Steps

### Phase 1: Lazy Atom Resolution Infrastructure

#### Step 1: Design `from_reader(impl Read)` adapter

**Goal:** Convert a byte/char stream into lazy atom tokens.

**Design:**

```
                    ┌──────────────────────────────────────────────┐
                    │            from_reader ADAPTER CHAIN          │
                    └──────────────────────────────────────────────┘

  impl Read         BufReader          char iterator        LazyAtomIter
 ───────────► BufReader::new(r) ──► .bytes()/.chars() ──► LazyAtomIter {
  (file,                             (UTF-8 decoded)        graph,
   stdin,                                                   chars: Box<dyn Iterator<Item=char>>
   socket)                                                 }
                                                              │
                                                              ▼
                                                          impl Iterator<Item = NewAtomIndex>
                                                            .next():
                                                              char → get_or_create_atom → NewAtomIndex
```

**New type in `segment.rs`:**

```rust
/// Lazy atom resolution iterator.
/// Resolves each character to a NewAtomIndex on demand.
pub(crate) struct LazyAtomIter<C: Iterator<Item = char>> {
    chars: C,
    graph: HypergraphRef,
}

impl<C: Iterator<Item = char>> Iterator for LazyAtomIter<C> {
    type Item = NewAtomIndex;
    fn next(&mut self) -> Option<NewAtomIndex> {
        self.chars.next().map(|ch| {
            let atom = Atom::Element(ch);
            match self.graph.graph().get_atom_index(atom) {
                Ok(i) => NewAtomIndex::Known(i),
                Err(_) => {
                    let i = self.graph.graph().insert_atom(atom);
                    NewAtomIndex::New(i.index)
                },
            }
        })
    }
}
```

**Files:** `crates/context-read/src/segment.rs`
**Test:** Unit test with `Cursor::new(b"hello")` verifying lazy resolution produces correct `NewAtomIndex` sequence.

---

#### Step 2: Design lazy `NewAtomIndex` resolution — generic `SegmentIter`

**Goal:** Make `SegmentIter` generic over any `Iterator<Item = NewAtomIndex>`.

**Changes to `segment.rs`:**

```rust
// Before:
pub(crate) struct SegmentIter {
    iter: std::iter::Peekable<std::vec::IntoIter<NewAtomIndex>>,
}

// After:
pub(crate) struct SegmentIter<I: Iterator<Item = NewAtomIndex> = std::vec::IntoIter<NewAtomIndex>> {
    iter: std::iter::Peekable<I>,
}
```

The **default type parameter** preserves backward compatibility: all existing code that says `SegmentIter` without a type parameter continues to work with the eager `Vec`-based iterator.

**`SegmentIter::new` becomes generic:**

```rust
impl<I: Iterator<Item = NewAtomIndex>> SegmentIter<I> {
    pub(crate) fn new(sequence: impl IntoIterator<IntoIter = I, Item = NewAtomIndex>) -> Self {
        Self { iter: sequence.into_iter().peekable() }
    }
}
```

**Propagation:** `ReadCtx` will need a type parameter or use `Box<dyn Iterator>` to erase the iterator type. Recommended approach: **type erasure** at the `ReadCtx` boundary to keep the public API simple.

```rust
pub(crate) type ErasedSegmentIter = SegmentIter<Box<dyn Iterator<Item = NewAtomIndex>>>;
```

**Files:** `crates/context-read/src/segment.rs`, `crates/context-read/src/context/mod.rs`
**Test:** Existing `SegmentIter` tests still pass; new test with `LazyAtomIter` produces identical segments.

---

#### Step 3: Design `ReadSequenceIter` — public iterator wrapper

**Goal:** Provide a public iterator that wraps `ReadCtx` and yields per-segment results, making the read pipeline composable.

**Current `ReadCtx` iterator:**
```rust
impl Iterator for ReadCtx {
    type Item = ();        // ← yields nothing useful
    fn next(&mut self) -> Option<Self::Item> {
        self.segments.next().map(|block| self.read_segment(block))
    }
}
```

**Design — `ReadSequenceIter`:**

```
                    ┌──────────────────────────────────────────┐
                    │           ReadSequenceIter                │
                    └──────────────────────────────────────────┘

  ReadSequenceIter::new(graph, input)
    │
    ▼
  impl Iterator<Item = SegmentResult>
    │
    ├─ SegmentResult::Unknown { atoms: Pattern, root: Token }
    │    └─ After appending unknown atoms to RootManager
    │
    └─ SegmentResult::Known { expansion: BandState, root: Token }
         └─ After processing known block through BlockExpansionCtx
```

```rust
/// Result of processing one segment in the read pipeline.
#[derive(Debug)]
pub enum SegmentResult {
    /// Unknown atoms were appended directly to the root.
    Unknown {
        /// The unknown atom pattern that was appended
        atoms: Pattern,
        /// Current root token after appending
        root: Option<Token>,
    },
    /// Known atoms were processed through the expansion pipeline.
    Known {
        /// The known pattern that was expanded
        pattern: Pattern,
        /// Current root token after expansion + commit
        root: Option<Token>,
    },
}

/// Public iterator over the read pipeline.
/// Yields one `SegmentResult` per segment (unknown or known block).
pub struct ReadSequenceIter {
    ctx: ReadCtx,
}

impl Iterator for ReadSequenceIter {
    type Item = SegmentResult;
    fn next(&mut self) -> Option<Self::Item> {
        // delegates to ReadCtx but returns structured results
    }
}
```

**Files:** `crates/context-read/src/context/mod.rs`, `crates/context-read/src/lib.rs`
**Test:** Iterate `ReadSequenceIter` over "abcab", verify two segments yielded with correct types.

---

### Phase 2: Cleanup & Constructors

#### Step 4: Clean up unused async dependencies in Cargo.toml

**Goal:** Remove aspirational async dependencies that are not used in any source file.

**Remove:**
- `async-std = "1.12"`
- `async-trait = "^0.1"`
- `async-recursion = "1"`
- `futures = "^0.3"`
- `pin-project-lite = "^0.2"`

**Keep behind feature flag:**
```toml
[features]
default = []
async = ["tokio", "tokio-stream"]

[dependencies.tokio]
version = "^1"
optional = true
features = ["sync", "rt", "macros", "time"]

[dependencies.tokio-stream]
version = "^0.1"
optional = true
features = ["sync", "time", "io-util"]
```

**Verification:** `cargo check -p context-read` succeeds. `cargo check -p context-read --features async` succeeds.

**Files:** `crates/context-read/Cargo.toml`

---

#### Step 5: Add `from_reader` constructor to `ReadCtx`

**Goal:** Allow `ReadCtx` to accept streaming input via `impl Read`.

```rust
impl ReadCtx {
    /// Create a ReadCtx from a byte stream reader.
    ///
    /// Characters are lazily resolved to atoms as they are consumed
    /// by the segmentation iterator. Unknown characters are inserted
    /// into the graph on demand.
    pub fn from_reader(graph: HypergraphRef, reader: impl Read + 'static) -> Self {
        use std::io::BufRead;
        let buf_reader = std::io::BufReader::new(reader);
        // decode_utf8 produces chars from the buffered byte stream
        let chars = buf_reader.bytes()
            .filter_map(|b| b.ok())
            .flat_map(|b| std::char::from_u32(b as u32)); // simplified; real impl uses utf8 decoding
        let lazy_atoms = LazyAtomIter { chars, graph: graph.clone() };
        Self {
            segments: SegmentIter::new(Box::new(lazy_atoms) as Box<dyn Iterator<Item = NewAtomIndex>>),
            root: Some(RootManager::new(graph)),
        }
    }
}
```

> **Note:** Proper UTF-8 decoding from `impl Read` should use a crate like `utf8-read` or manually decode with `String::from_utf8_lossy` on buffered chunks. The exact decoding strategy is an implementation detail.

**Files:** `crates/context-read/src/context/mod.rs`
**Test:** `ReadCtx::from_reader(graph, Cursor::new(b"hello"))` produces same result as `ReadCtx::new(graph, "hello".chars())`.

---

### Phase 3: Design Documentation & Invariants

#### Step 6: Document the future async Stream adapter pattern (DESIGN ONLY)

**Goal:** Document how a `Stream<Item = char>` could wrap the synchronous iterator, including yield points and the `spawn_blocking` pattern. **Do not implement.**

See [Future Async Stream Pattern](#future-async-stream-pattern) section below.

**Files:** `crates/context-read/src/expansion/chain/mod.rs` (doc comments), `crates/context-read/HIGH_LEVEL_GUIDE.md`

---

#### Step 7: Add `debug_assert` for the multi-band invariant in `ExpansionCtx`

**Goal:** Enforce at debug time that `BandState` never enters an invalid state during the expansion loop.

**Invariant (D8):** A `BandState::WithOverlap` must be committed (via `RootManager::commit_state`) before any further expansion steps. The `ExpansionCtx::next()` method already checks `self.state.has_overlap()` and returns `None`, but adding an explicit `debug_assert!` makes the invariant self-documenting.

**Change in `expansion/mod.rs`, `ExpansionCtx::next()`:**

```rust
fn next(&mut self) -> Option<Self::Item> {
    // Multi-band invariant: if we have an overlap, we must commit before continuing.
    // The caller (BlockExpansionCtx::process) is responsible for committing.
    debug_assert!(
        !self.state.has_overlap(),
        "BandState has uncommitted overlap — commit via RootManager::commit_state() \
         before calling ExpansionCtx::next() again. State: {:?}",
        self.state,
    );

    // existing logic...
    ExpandCtx::try_new(self)
        .and_then(|mut ctx| { /* ... */ })
        .and_then(|op| self.apply_op(op))
}
```

**Files:** `crates/context-read/src/expansion/mod.rs`
**Test:** Existing tests pass. Add a `#[test] #[should_panic]` test that calls `.next()` without committing after overlap.

---

## Lazy Atom Resolution Design

### Detailed Comparison

**Current (eager):**

```
text.chars()
  │
  ▼
graph.new_atom_indices(chars)     ← allocates Vec<NewAtomIndex> for ENTIRE input
  │
  ▼
Vec<NewAtomIndex>                 ← fully materialized in memory
  │
  ▼
SegmentIter::new(vec)
  │
  ▼
Peekable<IntoIter<NewAtomIndex>>  ← consumes one at a time, but all were created upfront
```

**Proposed (lazy):**

```
text.chars()  OR  BufReader::new(reader).bytes()→chars()
  │
  ▼
LazyAtomIter { chars, graph }     ← NO allocation; holds reference to char source
  │
  ▼
impl Iterator<Item=NewAtomIndex>  ← resolves ONE atom per .next() call
  │
  ▼
SegmentIter::new(lazy_iter)
  │
  ▼
Peekable<LazyAtomIter<C>>        ← consumes AND resolves one at a time
```

### Why This Is Safe

Lazy resolution means graph state changes between atom lookups. This is safe because:

1. **Unknown atoms are created immediately.** When `get_atom_index` fails, `insert_atom` is called on the spot. The atom exists in the graph from that point forward. No deferred creation.

2. **The "known" classification is based on current graph state.** If atom `'a'` is first seen as `New(0)`, then later `'a'` appears again, it's classified as `Known(0)`. This is correct — the classification reflects the graph at consumption time, not at input time.

3. **The expansion algorithm handles concurrent graph modifications.** The graph uses interior mutability (`HypergraphRef` is `Rc<RefCell<...>>`). The expansion pipeline already reads and writes the graph during processing. Lazy atom resolution simply moves the "write new atoms" step to be interleaved with consumption rather than batched upfront.

4. **Segment boundaries are preserved.** The `SegmentIter` partitions based on `is_new()` / `is_known()` predicates. Whether these are evaluated eagerly or lazily produces identical segmentation because:
   - A `New` atom is created in the graph when first encountered
   - A `Known` atom was created during a previous `.next()` call (or was already in the graph)
   - The `peeking_take_while` loop consumes contiguous runs of same-kind atoms

### Edge Case: Single-Character Input

```
"a" (not in graph)
  → LazyAtomIter.next() → New(0)
  → SegmentIter: unknown=[a], known=[]
  → NextSegment { unknown: [a], known: [] }
  → RootManager::append_pattern([a])
  → root = Some(a)
```

Same behavior as current eager path.

### Edge Case: All-Known Input

```
"ab" (both 'a' and 'b' already in graph)
  → LazyAtomIter.next() → Known(0)
  → LazyAtomIter.next() → Known(1)
  → SegmentIter: unknown=[], known=[a,b]
  → NextSegment { unknown: [], known: [a,b] }
  → BlockExpansionCtx processes known block
```

Same behavior as current eager path.

### Edge Case: Interleaved New/Known

```
"aba" where only 'b' is in graph:
  → next() → 'a' → New(0)     ← creates 'a'
  → next() → 'b' → Known(1)   ← 'b' was already in graph
  → next() → 'a' → Known(0)   ← 'a' was created two steps ago (LAZY BENEFIT!)

  With EAGER resolution, this would be:
    → 'a' → New(0), 'b' → Known(1), 'a' → Known(0)
    Same result! Because eager also creates 'a' before checking 'a' again.
```

The lazy approach produces identical results because `new_atom_indices` creates atoms during iteration too — the only difference is that eager collects them all into a Vec first.

---

## Future Async Stream Pattern

> **⚠️ DESIGN ONLY — DO NOT IMPLEMENT**
>
> This section documents the intended async Stream wrapper for future reference.
> The sync iterator core (D9) must be working correctly before wrapping in async.

### Architecture

```
                    ┌──────────────────────────────────────────┐
                    │          FUTURE ASYNC ARCHITECTURE        │
                    └──────────────────────────────────────────┘

  Stream<Item = char>         AsyncReadCtx              Stream<Item = SegmentResult>
 ────────────────────►  AsyncReadCtx::from_stream() ──────────────────────────────►
  (tokio channel,         │                              (async yields between
   network socket,        │                               segments)
   file watcher)          ▼
                     ┌─────────────────────────┐
                     │  spawn_blocking(|| {     │
                     │    sync_ctx.next()       │   ← CPU-intensive graph ops
                     │  })                      │     run on blocking thread pool
                     └─────────────────────────┘
```

### Yield Points

Two levels of yield granularity:

**Level 1 — Between segments (recommended initial approach):**

```rust
impl AsyncReadCtx {
    pub fn into_stream(self) -> impl Stream<Item = SegmentResult> {
        stream! {
            let mut ctx = self.sync_ctx;
            loop {
                // Yield between segments — each segment is processed synchronously
                let result = tokio::task::spawn_blocking(move || {
                    let item = ctx.next();
                    (ctx, item)
                }).await.unwrap();

                ctx = result.0;
                match result.1 {
                    Some(segment_result) => yield segment_result,
                    None => break,
                }
            }
        }
    }
}
```

**Level 2 — Between expansion steps (finer granularity, future optimization):**

```
for each segment:
  yield_point ← between segments
  for each expansion step:
    yield_point ← between ExpandCtx iterations (within BlockExpansionCtx::process)
```

This would require splitting `BlockExpansionCtx::process()` into an iterator that yields intermediate states, which is a larger refactor not needed initially.

### spawn_blocking Pattern

CPU-intensive graph operations (insert, search, complement building) should NOT run on the async runtime's cooperative threads. The `tokio::task::spawn_blocking` pattern is correct here:

```
Async task (tokio runtime)
  │
  ├─ Receives chars from Stream<Item = char>
  ├─ Buffers into segment-sized chunks (or feeds lazily)
  ├─ spawn_blocking(|| process_segment(chunk))
  │    └─ Runs on dedicated thread pool
  │    └─ Calls graph.insert_or_get_complete, ComplementBuilder, etc.
  ├─ .await the blocking result
  └─ yield SegmentResult
```

### Required Changes (When Implementing)

1. Feature-gate async module behind `async` feature flag
2. `ReadCtx` must be `Send` — verify `HypergraphRef` is `Send` (currently `Rc<RefCell<...>>`, which is NOT `Send`; would need `Arc<RwLock<...>>` — see `PLAN_fine_grained_locking.md`)
3. Add `pin-project-lite` back for `Stream` impl (only under `async` feature)
4. The `from_stream` constructor would buffer chars into a channel, consumed by `LazyAtomIter` on the blocking thread

### Blocking Concern

`HypergraphRef` is currently `Rc<RefCell<Hypergraph>>`, which is `!Send`. The async pattern requires either:
- **Option A:** Switch to `Arc<RwLock<Hypergraph>>` (see `20260115_PLAN_fine_grained_locking.md`)
- **Option B:** Keep all graph access on a single dedicated thread, communicate via channels

Option A is the long-term direction. Option B is a short-term workaround. Neither is needed until async is actually implemented.

---

## Validation

### Test Scenarios

| # | Scenario | What to verify |
|---|----------|---------------|
| T1 | `LazyAtomIter` with empty input | Returns `None` immediately |
| T2 | `LazyAtomIter` with "abc" on empty graph | Three `New` atoms, all inserted in graph |
| T3 | `LazyAtomIter` with "aba" where 'b' exists | `New(a)`, `Known(b)`, `Known(a)` — lazy re-classification |
| T4 | `SegmentIter<LazyAtomIter>` produces same segments as eager | Side-by-side comparison for "abcab" |
| T5 | `ReadCtx::from_reader` with `Cursor::new(b"hello")` | Same root token as `ReadCtx::new(graph, "hello".chars())` |
| T6 | `ReadSequenceIter` yields correct number of segments | "abcab" → 1 segment (unknown+known); "xyz" all new → 1 segment (unknown only) |
| T7 | `debug_assert` fires on uncommitted overlap | `#[should_panic]` test calling `next()` after overlap detected |
| T8 | Large input via `from_reader` | 10K+ chars from file, verify memory stays bounded (no full materialization) |
| T9 | Cargo.toml cleanup compiles | `cargo check -p context-read` without removed deps |

### Verification Commands

```bash
# All context-read tests pass
cargo test -p context-read

# Check compilation without async deps
cargo check -p context-read

# Check async feature flag (when implemented)
cargo check -p context-read --features async

# No regressions in dependent crates
cargo test -p context-read -p context-insert -p context-search
```

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Generic `SegmentIter<I>` propagates type parameter through `ReadCtx`, making API complex | Medium | Medium | Use type erasure (`Box<dyn Iterator>`) at `ReadCtx` boundary. Small runtime cost (vtable dispatch per atom) is negligible vs. graph operations. |
| Lazy resolution changes timing of atom creation vs. eager | Low | Low | Proven equivalent (see Analysis section). Both approaches create atoms on first encounter. Add property test comparing lazy vs. eager on random strings. |
| Graph state changes between `LazyAtomIter.next()` calls | Low | Low | Already handled by interior mutability. The expansion pipeline mutates the graph during processing. Atom insertion is strictly additive (never removes/modifies existing atoms). |
| Removing async deps breaks downstream crates | Low | High | `grep -r "async-std\|async-trait\|async-recursion\|futures\|pin-project-lite" crates/context-read/src/` confirms zero usage. Run full workspace `cargo check` after removal. |
| `from_reader` UTF-8 decoding edge cases (multi-byte chars, BOM, invalid sequences) | Medium | Medium | Use `std::io::BufRead::lines()` or a proper UTF-8 streaming decoder. Add tests with non-ASCII input (emoji, CJK, mixed scripts). |
| `ReadSequenceIter` changes public API surface | Low | Low | `ReadCtx::read_sequence()` remains the primary entry point. `ReadSequenceIter` is additive — new API for users who want segment-by-segment control. |
| `debug_assert` in `ExpansionCtx::next()` triggers in existing tests | Low | Medium | The current `BlockExpansionCtx::process()` loop commits overlap state before calling `next()` again. If any test fails, it reveals an existing bug in the commit protocol. |

---

## Iterator Chain Diagram (Final Design)

```
                     ┌─────────────────────────────────────────────────────┐
                     │                  PROPOSED FLOW                       │
                     └─────────────────────────────────────────────────────┘

                         ┌─────────────┐
  impl Read ───────────► │ BufReader   │
  OR                     │ + UTF-8     │
  &str ──► .chars() ───► │ decode      │
                         └──────┬──────┘
                                │
                           char iterator (lazy)
                                │
                         ┌──────▼──────┐
                         │ LazyAtomIter│  graph.get_atom_index(ch)
                         │ { chars,    │  → Known(idx)
                         │   graph }   │  OR graph.insert_atom(ch)
                         │             │  → New(idx)
                         └──────┬──────┘
                                │
                        NewAtomIndex (one at a time)
                                │
                    ┌───────────▼───────────┐
                    │  SegmentIter<I>       │  peeking_take_while(is_new)
                    │  { Peekable<I> }      │  peeking_take_while(is_known)
                    └───────────┬───────────┘
                                │
                         NextSegment { unknown, known }
                                │
                    ┌───────────▼───────────┐
                    │      ReadCtx          │
                    │  { root: RootManager, │
                    │    segments }          │
                    └───────────┬───────────┘
                                │
                    ┌───────────▼───────────────────────────────┐
                    │           read_segment(segment)            │
                    │  ┌────────────────┐  ┌──────────────────┐ │
                    │  │ root.append_   │  │ BlockExpansionCtx│ │
                    │  │ pattern(unknown)│  │ ::new(root,known)│ │
                    │  └────────────────┘  │ ::process()      │ │
                    │                      │ ::finish()→root  │ │
                    │                      └────────┬─────────┘ │
                    └───────────────────────────────┼───────────┘
                                                    │
                    ┌───────────────────────────────▼───────────┐
                    │         ExpansionCtx::next() loop          │
                    │  debug_assert!(!state.has_overlap())       │
                    │  ┌────────────────────────────────┐       │
                    │  │ ExpandCtx → PostfixIterator    │       │
                    │  │ (BandExpandingIterator)         │       │
                    │  │ yields ChainOp::Expansion       │       │
                    │  │    or  ChainOp::Cap             │       │
                    │  └──────────────┬─────────────────┘       │
                    │                 │                          │
                    │  apply_op(Cap) → BandState::append()      │
                    │  apply_op(Expansion) →                    │
                    │    ComplementBuilder::build()              │
                    │    → BandState::set_overlap()             │
                    │    → yield BandState (to be committed)    │
                    └───────────────────────────────────────────┘
                                │
                         SegmentResult
                                │
                    ┌───────────▼───────────┐
                    │  ReadSequenceIter     │  ← new public wrapper
                    │  yields SegmentResult │
                    └───────────────────────┘
```

---

## Notes

### Questions for User
- Should `from_reader` handle encoding other than UTF-8? (Assumed: UTF-8 only for now)
- Should `ReadSequenceIter` be `pub` or `pub(crate)`? (Assumed: `pub` — useful for CLI/API consumers)
- Should the `async` feature flag also gate `tokio` in `[target.'cfg(not(wasm))']`? (Assumed: yes)

### Design Decisions Log
- **Type erasure over generics:** Chose `Box<dyn Iterator>` at `ReadCtx` boundary rather than making `ReadCtx<I>` generic. Reason: keeps public API stable, runtime cost negligible.
- **Default type parameter on `SegmentIter`:** Allows existing internal code to use `SegmentIter` without specifying type. New code can use `SegmentIter<LazyAtomIter<C>>` explicitly.
- **Feature-gated async deps:** Rather than removing `tokio`/`tokio-stream` entirely, move them behind `async` feature. The sync core is the default.

### Related Documents
- [`20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md`](20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md) — parent plan
- [`20260218_PLAN_CONTEXT_READ_COMPLETION.md`](20260218_PLAN_CONTEXT_READ_COMPLETION.md) — cursor advancement, expansion fixes
- [`20260115_PLAN_fine_grained_locking.md`](20260115_PLAN_fine_grained_locking.md) — `Arc<RwLock>` migration (prerequisite for async)
- [`20260310_PLAN_CONTEXT_API_PHASE2.md`](20260310_PLAN_CONTEXT_API_PHASE2.md) — algorithm commands including `read`
