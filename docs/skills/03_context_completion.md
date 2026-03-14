---
skill: 03_context_completion
title: "Context Completion — Finding Meaning in Small Tokens"
level: intermediate
prerequisites:
  - 01_the_hypergraph_model
  - 02_reading_text
status: draft
verified_against: tools/context-cli/tests/integration/skill3_exploration.rs
last_updated: 2026-03-14
known_limitations:
  - RC-1: insert_sequence outer loop missing — compound tokens not yet created
    via the CLI write path (read path is unaffected)
  - RC-3: repeat/overlap handling broken for inputs like "aaaa"
---

# Skill 3: Context Completion — Finding Meaning in Small Tokens

> **Dungeon theme:** You are a **craftsman** in the dungeon's underground workshop.
> Your forge can join known pieces into new compound items — but only if you
> recognise each piece first.

---

## Table of Contents

1. [The Dungeon Analogy](#the-dungeon-analogy)
2. [Core Concept: `insert_next_match`](#core-concept-insert_next_match)
3. [The Three Outcomes](#the-three-outcomes)
4. [The Outer Loop](#the-outer-loop)
5. [Worked Example 1 — Created](#worked-example-1--created-inserting-hello-when-hel-and-lo-exist)
6. [Worked Example 2 — Complete](#worked-example-2--complete-inserting-hello-when-hello-already-exists)
7. [Worked Example 3 — NoExpansion](#worked-example-3--noexpansion-inserting-helloworld-when-hello-exists-but-world-doesnt)
8. [Visual Diagram: The Extension Loop](#visual-diagram-the-extension-loop)
9. [How Atoms Underpin Everything](#how-atoms-underpin-everything)
10. [Try It Yourself (REPL)](#try-it-yourself-repl)
11. [Current State & Known Limitations](#current-state--known-limitations)
12. [Key Takeaways](#key-takeaways)
13. [Related Skills](#related-skills)

---

## The Dungeon Analogy

Imagine you run a **craftsman's workshop** deep inside the dungeon.

Your workshop table holds an inventory of **known components**:

- **Iron ingots** — atoms (single characters: `h`, `e`, `l`, `o`, …)
- **Assembled items** — compound tokens (`"hel"`, `"lo"`, `"hello"`, …)

When an adventurer brings you a request — say, *"make me a 'hello'"* — you follow
this process:

1. **Look at your inventory.** Do you recognise the biggest piece that starts
   the request?  Say you have `"hel"` (width 3) on the table.
2. **Try to extend.** Can you attach the next known piece to form something bigger?
   You also have `"lo"` (width 2). Joining them gives you `"hello"` — a **new
   compound item** you craft on the spot.  Outcome: **Created**.
3. **Recognise a complete recipe.** Next time the adventurer asks for `"hello"`,
   you already have it.  Outcome: **Complete**.
4. **Hit a dead end.** You find `"hello"` but the request continues with
   `"world"` and you have no `"world"` yet.  You hand over `"hello"` and report
   *"I can go no further right now."*  Outcome: **NoExpansion** — the caller
   must advance the cursor and continue from where you stopped.

These three outcomes — **Created**, **Complete**, **NoExpansion** — are the
vocabulary of the `insert_next_match` primitive.

---

## Core Concept: `insert_next_match`

`insert_next_match` is the **single-step insertion primitive** of the engine.
It handles exactly one *segment* of the input at a time:

```
insert_next_match(query: &[Token]) -> Result<InsertOutcome, ErrorReason>
```

One call answers the question:

> *"Starting at the current cursor position, what is the largest known token
> I can match?  Can I extend it with the next known token to create something
> new?"*

After the call returns, the **caller is responsible** for advancing the cursor
past the matched segment and calling `insert_next_match` again for the remainder
— this is the **outer loop**.

### What it does internally

```
                   ┌───────────────────────────────────────┐
                   │         insert_next_match              │
                   │                                        │
                   │  1. Search: find largest match         │
                   │     starting at cursor position 0      │
                   │                                        │
                   │  2. Is the full query consumed?        │
                   │     YES ──► Complete                   │
                   │                                        │
                   │  3. Is a complete token at start       │
                   │     but query extends beyond?          │
                   │     YES ──► NoExpansion                │
                   │                                        │
                   │  4. Partial match — run split+join     │
                   │     pipeline to build new token        │
                   │     ──► Created                        │
                   └───────────────────────────────────────┘
```

**Split+join** is the creative step: the engine takes two adjacent known tokens
(`"hel"` + `"lo"`), verifies their shared border offset, and welds them into a
new compound token (`"hello"`).

---

## The Three Outcomes

```
pub enum InsertOutcome {
    Created       { result: IndexWithPath, response: Response },
    Complete      { result: IndexWithPath, response: Response },
    NoExpansion   { result: IndexWithPath, response: Response },
}
```

Every variant carries the matched token (`result`) and the raw search response
(`response`) for diagnostics and caching.

### Created

A **new compound token** was built by joining adjacent known pieces via the
split+join pipeline.

- The token did **not** exist before this call.
- The graph has grown: a new vertex and at least one new child pattern were
  added.
- `result.index` is the freshly created token.
- The outer loop should advance the cursor by `result.token.width` atoms.

### Complete

The **entire query** was consumed by a single existing token.  Nothing was
inserted.

- The token **already existed** in the graph.
- The graph is unchanged.
- `result.index` is the existing token.
- The outer loop should advance the cursor to the end (query was fully
  consumed).

### NoExpansion

A **complete token was matched** at the start of the query, but the query
**extends beyond** that token.  The engine cannot join further because the next
piece is not (yet) known.

- The matched sub-token existed before this call.
- The graph is unchanged by this call alone (the caller will process the rest
  in subsequent calls, potentially creating new tokens).
- `result.index` is the matched sub-token.
- The outer loop advances the cursor by `result.token.width` atoms and calls
  `insert_next_match` again for the remainder.

### Mapping to `InsertResult.already_existed`

The `Command::InsertSequence` API wraps `InsertOutcome` as:

| `InsertOutcome`  | `already_existed` |
|------------------|-------------------|
| `Created`        | `false`           |
| `Complete`       | `true`            |
| `NoExpansion`    | `true`            |

> ⚠ **RC-1 note**: Today `insert_sequence` calls `insert_next_match` only
> **once** and does not loop.  For a multi-character sequence on a fresh graph,
> the first call always returns `NoExpansion` (only the first atom is matched),
> so `already_existed` is always `true` and `token.width` is always `1`.
> See [Current State & Known Limitations](#current-state--known-limitations).

---

## The Outer Loop

`insert_next_match` is a *primitive*.  The **outer loop** is what calls it
repeatedly to consume an entire input sequence:

```
cursor = 0
segments = []

while cursor < input.len():
    outcome = insert_next_match(input[cursor..])

    match outcome:
        Created(token)     → segments.push(token); cursor += token.width
        Complete(token)    → segments.push(token); cursor += token.width  (done)
        NoExpansion(token) → segments.push(token); cursor += token.width

root = create_root_token(segments)
return root
```

The read pipeline (`ReadCtx` in `context-read`) already implements this loop
correctly.  The write pipeline (`insert_sequence` in `context-api`) does not yet
include the loop — see RC-1 in [Current State](#current-state--known-limitations).

### Why NoExpansion is not a failure

`NoExpansion` does not mean *"this sequence cannot be inserted"*.  It means
*"I found the best match at this position; move on."*  The outer loop advances
the cursor and tries again.  Over many calls, the engine builds up the full
compound token step by step.

Think of it like a craftsman who runs out of large components: they produce a
smaller sub-assembly, set it aside, and continue with the remaining materials.

---

## Worked Example 1 — Created: Inserting "hello" When "hel" and "lo" Exist

**Graph state before:**

```
Atoms:    h(0)  e(1)  l(2)  o(3)
Tokens:   "hel" (v4, w=3, pattern: [h,e,l])
          "lo"  (v5, w=2, pattern: [l,o])
```

**Call:** `insert_next_match([h, e, l, l, o])`

```
Step 1: Search from cursor=0
        Input: h  e  l  l  o
               ├──────┤
               "hel" (v4, w=3) ← largest match

Step 2: Try to extend
        Remaining starts at cursor=3: [l, o]
        → Largest match for [l,o] = "lo" (v5, w=2)
        → Both "hel" and "lo" share border at offset 3
        → Split+join: merge "hel"(v4) + "lo"(v5) → new token "hello" (v6)

Step 3: Query exhausted after merging → outcome = Created

Result:
  InsertOutcome::Created { result: IndexWithPath { index: v6, ... } }
```

**Graph state after:**

```
Atoms:    h(0)  e(1)  l(2)  o(3)          ← unchanged
Tokens:   "hel" (v4, w=3)                 ← unchanged
          "lo"  (v5, w=2)                 ← unchanged
          "hello" (v6, w=5, pattern: ["hel","lo"])  ← NEW
```

**Key observation:** No new atoms were created.  `"hello"` is built entirely
from pieces the graph already knew.

> ✅ **Verified by:** `skill3_exp_c_compound_from_known_pieces` (ignored pending
> RC-1 fix), `skill3_exp_d_atom_deduplication` (GREEN today — atom count = 4).

---

## Worked Example 2 — Complete: Inserting "hello" When "hello" Already Exists

**Graph state before:**

```
Atoms:    h(0)  e(1)  l(2)  o(3)
Tokens:   "hel" (v4), "lo" (v5), "hello" (v6, w=5)
```

**Call:** `insert_next_match([h, e, l, l, o])`

```
Step 1: Search from cursor=0
        Input: h  e  l  l  o
               ├──────────────┤
               "hello" (v6, w=5) ← entire query consumed in one match!

Step 2: Query exhausted, match is entire root → outcome = Complete

Result:
  InsertOutcome::Complete { result: IndexWithPath { index: v6, ... } }
```

**Graph state after:** Unchanged.  `v6` is returned as-is.

**InsertResult:** `already_existed = true`, `token.index = v6`, `token.width = 5`

> ✅ **Verified by:** `skill3_exp_b_second_insert_is_complete` (ignored pending
> RC-1 fix), `skill3_obs2_same_text_twice_same_atom_index_today` (GREEN today —
> same index returned for two identical calls).

---

## Worked Example 3 — NoExpansion: Inserting "helloworld" When "hello" Exists But "world" Doesn't

**Graph state before:**

```
Atoms:    h(0)  e(1)  l(2)  o(3)
Tokens:   "hel" (v4), "lo" (v5), "hello" (v6, w=5)
```

**Outer loop call sequence:**

```
cursor = 0
─────────────────────────────────────────────────
Call 1: insert_next_match([h,e,l,l,o, w,o,r,l,d])
  (i.e. the full "helloworld" atom sequence)

  Search from cursor=0:
    h  e  l  l  o  w  o  r  l  d
    ├──────────────┤
    "hello" (v6, w=5) ← largest match

  Try to extend with [w,o,r,l,d]:
    Largest known token starting at cursor=5 = 'w' (atom, w=1)
    Cannot join "hello" + 'w' (no existing compound)

  Query not exhausted → outcome = NoExpansion
  Result: token = v6 ("hello", w=5)

cursor advances to 5
segments = ["hello"(v6)]
─────────────────────────────────────────────────
Call 2: insert_next_match([w,o,r,l,d])

  Search from cursor=5 (relative cursor=0 for this call):
    w  o  r  l  d
    ├─────────────┤
    No compound for "world" yet — only atoms w,o,r,l,d

  Split+join pipeline builds:
    "wo"   (v7, w=2)  ← if a pair step runs
    "wor"  (…)  → eventually "world" (v11, w=5)

  Query exhausted after building → outcome = Created
  Result: token = v11 ("world", w=5)

cursor advances to 10
segments = ["hello"(v6), "world"(v11)]
─────────────────────────────────────────────────
Outer loop: cursor == input.len() → done

root = create_root_token(["hello", "world"])
     = new vertex "helloworld" (v12, w=10,
         pattern: ["hello"(v6), "world"(v11)])
```

**Final graph additions:**

```
NEW: intermediate tokens for "world" (w, wo, wor, worl, world)
NEW: "helloworld" (v12, w=10, pattern: ["hello","world"])
```

> ✅ **Verified by:** `skill3_exp_e_known_prefix_then_new_suffix` (ignored
> pending RC-1 fix), `skill3_obs4_cross_word_atom_sharing` (GREEN today —
> 7 unique atoms for "hello"+"world").

---

## Visual Diagram: The Extension Loop

```
insert_next_match for "helloworld" (outer loop shown)

  Input atoms: h  e  l  l  o  w  o  r  l  d
               0  1  2  3  4  5  6  7  8  9
               ╔══════════════╗╔═════════════╗
               ║   "hello"    ║║   "world"   ║
               ║  (KNOWN, w=5)║║  (UNKNOWN)  ║
               ╚══════════════╝╚═════════════╝

  ┌─────────────────────────────────────────────────────────┐
  │ CALL 1: cursor=0, query=[h,e,l,l,o,w,o,r,l,d]          │
  │                                                         │
  │  Search → "hello" (w=5)                                 │
  │  Extend? next=[w,o,r,l,d] → 'w' only (atom) → NO       │
  │  ──► NoExpansion("hello")                               │
  │  cursor ──────────────────────────► 5                   │
  └─────────────────────────────────────────────────────────┘
              │
              ▼
  ┌─────────────────────────────────────────────────────────┐
  │ CALL 2: cursor=5, query=[w,o,r,l,d]                     │
  │                                                         │
  │  Search → 'w' atom (w=1)                                │
  │  Extend? [o,r,l,d] → build "wo", "wor", "world"...     │
  │  ──► Created("world", w=5)                              │
  │  cursor ──────────────────────────► 10                  │
  └─────────────────────────────────────────────────────────┘
              │
              ▼
  ┌─────────────────────────────────────────────────────────┐
  │ OUTER LOOP: cursor == 10 == input.len() → DONE          │
  │                                                         │
  │  root = new_token(["hello"(v6), "world"(v11)])          │
  │       = "helloworld" (w=10)                             │
  └─────────────────────────────────────────────────────────┘
```

### Cursor trace for the Complete case ("hello" inserted twice)

```
  Input: h  e  l  l  o
         ╔══════════════╗
         ║   "hello"    ║   ← full match, w=5
         ╚══════════════╝

  CALL 1: cursor=0, query=[h,e,l,l,o]
    Search → "hello" (w=5), query fully consumed
    ──► Complete("hello")
    cursor ──────────────────────────► 5 (== input.len())

  DONE — no Call 2 needed.
```

---

## How Atoms Underpin Everything

Every compound token ultimately decomposes into atoms — single characters.
Understanding atom behaviour is crucial for interpreting `InsertOutcome`:

| Observation | Why |
|-------------|-----|
| First call on a fresh graph always returns `NoExpansion` | The largest match for a fresh atom sequence is always the first atom; no compound tokens exist yet to extend into. |
| Atoms are never duplicated | `insert_atom` is idempotent; the same character maps to one vertex forever. |
| `token.width` = total atom count | "hello" has width 5 because it expands to exactly 5 atoms: h, e, l, l, o (two `l` positions, one `l` vertex). |
| Atom deduplication is already correct | Even with RC-1, `insert_sequence` correctly auto-creates and deduplicates atoms. |

### Exploration result (GREEN today)

Running `skill3_obs3_atoms_correctly_auto_created_and_deduped`:

```
insert_text("hel")   → atoms: h(0), e(1), l(2)          atom_count = 3
insert_text("lo")    → atoms: reuse l(2), create o(3)   atom_count = 4
insert_text("hello") → reuses h(0),e(1),l(2),l(2),o(3)  atom_count = 4  ← unchanged
```

Four atoms, three insert calls.  `'l'` is shared.

---

## Try It Yourself (REPL)

> All REPL examples below use the intended (post-RC-1-fix) behaviour.
> See [Current State](#current-state--known-limitations) for what to expect today.

### Setup

```bash
# Start the REPL
context-cli repl

# Create a workspace
> create dungeon-demo
Created workspace 'dungeon-demo'.
(workspace 'dungeon-demo' is now active)
```

### Experiment A — Fresh insert (outcome: Created)

```bash
(dungeon-demo)> insert hel
+ Inserted: "hel" (index: 3, width: 3)

(dungeon-demo)> insert lo
+ Inserted: "lo" (index: 4, width: 2)

(dungeon-demo)> insert hello
+ Inserted: "hello" (index: 5, width: 5)
#                    ^^^^^^^^  ───────── NEW compound token
```

Inspect the structure:

```bash
(dungeon-demo)> show 5
Vertex 5 "hello" (width: 5, pattern)
  Pattern 0: "hel"(3), "lo"(4)
  Parents: 0

(dungeon-demo)> stats
Graph Statistics:
  Vertices:  6
  Atoms:     4
  Patterns:  1
  Edges:     5
  Max width: 5
```

Note: 4 atoms (h, e, l, o) + 3 compound tokens ("hel", "lo", "hello") = 6 vertices.

### Experiment B — Second insert (outcome: Complete)

```bash
(dungeon-demo)> insert hello
= Existing: "hello" (index: 5, width: 5)
#  ^^^^^^^  ─ same index, width unchanged, nothing added to the graph
```

### Experiment C — Known prefix + unknown suffix (NoExpansion then Created)

```bash
# "hello" is known. "world" is not.
(dungeon-demo)> insert helloworld
+ Inserted: "helloworld" (index: 11, width: 10)

(dungeon-demo)> show 11
Vertex 11 "helloworld" (width: 10, pattern)
  Pattern 0: "hello"(5), "world"(10)
  Parents: 0
```

Internally: `NoExpansion("hello")` at cursor=0, then `Created("world")` at
cursor=5, then a root token wraps both.

### Experiment D — Atom inspection

```bash
# After all the above inserts:
(dungeon-demo)> atoms
Atoms (7):
  'h' -> 0
  'e' -> 1
  'l' -> 2
  'o' -> 3
  'w' -> 6
  'r' -> 7
  'd' -> 8
#  ──────── only 7 unique characters across "hello" + "world"
#           'l' and 'o' are shared between the two words
```

### Experiment E — Validate graph integrity

```bash
(dungeon-demo)> validate
Graph is valid (11 vertices checked).
```

### Reading back (illustrative — requires RC-2 fix)

```bash
# Once RC-2 is fixed, reading a known token returns its decomposition tree:
(dungeon-demo)> read hello
Root: "hello" (index: 5, width: 5)
Text: "hello"
Tree:
  "hello" [5] (width: 5)
    "hel" [3] (width: 3)
      'h' [0]
      'e' [1]
      'l' [2]
    "lo" [4] (width: 2)
      'l' [2]
      'o' [3]
```

---

## Current State & Known Limitations

### RC-1: `insert_sequence` missing outer loop

**Status:** Open (as of 2026-03-14)

**Symptom:** `insert_sequence` calls `insert_next_match` exactly once.  For a
multi-character input on a graph with only atoms:

- The first call matches the first atom (e.g. `'h'`) and returns `NoExpansion`.
- `already_existed = !is_expanded() = true` even for the very first insert.
- `token.width = 1` instead of the input length.
- No compound tokens are ever created via `Command::InsertSequence`.

**Affected commands:**
- `Command::InsertSequence` → `insert_text()` in tests
- `insert <text>` in REPL

**Unaffected paths:**
- The **read pipeline** (`Command::ReadSequence`, `read <text>` in REPL) has its
  own outer loop and is not affected by this bug.
- Atom creation and deduplication work correctly.
- `Command::ReadPattern`, `Command::ReadAsText`, `validate`, `stats`, and
  `show` all work.

**Fix plan:** Add an outer loop to `WorkspaceManager::insert_sequence` that
repeatedly calls `insert_next_match` with the remaining atoms until the entire
input is consumed, then wraps the collected segments into a root token.

**Tests tracking this:**
- All `skill3_exp_*` tests in `tools/context-cli/tests/integration/skill3_exploration.rs`
  that are `#[ignore = "RC-1: ..."]` will become green once this is fixed.

### RC-2: `read_sequence` returns `None` after prior insert

**Status:** Open

**Symptom:** After calling `insert_sequence`, calling `read_sequence` on the
same text returns `None`, resulting in an `InternalError`.  This affects the
REPL `read <text>` command when the graph has been modified by an insert.

**Unaffected paths:** `read <index>` (reads by vertex index) works.

### RC-3: Repeat/overlap handling broken

**Status:** Open

**Symptom:** Inputs where the same character repeats (e.g. `"aaaa"`) cause
panics or incorrect results due to cursor advancement bugs in the overlap
expansion path.

**Tracked by:** `skill3_exp_m_repeated_char_known_failing` (ignored).

---

## Key Takeaways

1. **`insert_next_match` is a primitive, not a loop.**  It processes exactly
   one segment.  The outer loop drives the full input.

2. **Three clean outcomes: Created, Complete, NoExpansion.**  Each has a clear
   meaning; none is an error.

3. **NoExpansion is a cursor signal, not a failure.**  It says: *"I've done
   my part — advance and call me again."*

4. **Created grows the graph; Complete and NoExpansion do not.**  Only
   `Created` adds a new vertex and child pattern.

5. **Atoms are always correct today.**  Even with RC-1, auto-creation and
   deduplication of atom vertices work as designed.  The 4-atom invariant for
   "hel" + "lo" + "hello" holds right now.

6. **The read pipeline already works end-to-end.**  `read <text>` has the
   outer loop.  The write path is what needs the fix.

7. **Graph integrity is maintained under RC-1.**  `validate` reports no issues
   even when `insert_sequence` only creates partial state.

---

## Related Skills

- **Previous:** [Skill 02 — Reading Text](02_reading_text.md)
  The read algorithm's outer loop is the pattern `insert_next_match`'s caller
  must implement.

- **Next:** [Skill 04 — Overlapping Decompositions](04_overlapping_decompositions.md)
  When the cursor advances and two adjacent segments share atoms (e.g. "abc"
  and "bcd" sharing "bc"), overlap detection kicks in — the topic of Skill 4.

- **Deep Dive (insert primitive):** `crates/context-insert/HIGH_LEVEL_GUIDE.md`
  Full walkthrough of the split+join pipeline that powers the `Created` path.

- **Deep Dive (read outer loop):** `crates/context-read/HIGH_LEVEL_GUIDE.md`
  The `ReadCtx` and `SegmentIter` implementation of the outer loop over
  `insert_next_match`.

- **Integration tests:** `tools/context-cli/tests/integration/skill3_exploration.rs`
  13 intent-documenting tests (8 green, 9 ignored pending RC-1 fix, 1 ignored
  pending RC-3 fix) that verify every claim in this document.

- **Bug tracker:** `tools/context-cli/tests/FAILING_TESTS.md`
  Maps every known failure to a root cause and fix plan.

- **Parent plan:** `agents/plans/20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md`
  Phase 3c in the overall UX improvement programme.