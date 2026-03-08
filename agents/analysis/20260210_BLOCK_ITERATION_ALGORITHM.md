---
tags: `#analysis` `#algorithm`
summary: ```
---

# Block Iteration Algorithm: Visual Documentation

## Example: Reading "ababab"

### Input State After Segmentation
```
Input: "ababab"
       ├──────┤
       unknown=[a, b]  (positions 0-1, new atoms)
       └──────────────────────┤
              known=[a, b, a, b]  (positions 2-5, atoms exist in graph)

Graph before processing:
  "a"(0) ─┬─ parent → "ab"(2)
  "b"(1) ─┘
```

---

## Phase 1: Process Unknown Block

```
unknown=[a, b]

Action: insert_pattern([a, b])
Result: Creates/finds "ab" token

Root state:
  root = "ab"(2)
  is_fresh = true
```

---

## Phase 2: Process Known Block

### Step 2.1: Initialize BlockExpansionCtx

```
known = [a, b, a, b]
         0  1  2  3   (indices in known pattern)
         
PatternCursor:
  pattern: [a, b, a, b]
  position: 0  ← starts at beginning
```

### Step 2.2: Search for Largest Prefix (insert_or_complete)

```
Search query: known=[a, b, a, b]
              cursor at position 0

Search finds:
  - "a"(0) matches at position 0
  - Check parents of "a": found "ab"(2)
  - "ab" matches [a, b] at positions 0-1 ✓
  - Continue: Check parents of "ab": none
  
Result:
  matched_token = "ab"(2)
  matched_width = 2
  cursor advances to position 2
```

### Step 2.3: Append Match to Root

```
Before:
  root = "ab"(2)
  matched = "ab"(2)

Action: root.append_token(matched)

IMPORTANT: Since matched.vertex == root.vertex:
  → Cannot extend in-place (same vertex!)
  → Create new pattern: [root, matched] = [ab, ab]
  → New root = "abab"(3)

Root state:
  root = "abab"(3)
  is_fresh = false  (had to create new composite)
```

### Step 2.4: BandChain State After First Match

```
BandChain {
  bands: [
    Band {
      pattern: [ab],        ← The first matched block
      start_bound: 0,
      end_bound: 2,         ← cursor was at position 2
    }
  ]
}

Cursor state:
  pattern: [a, b, a, b]
  position: 2  ← pointing to remaining [a, b]
```

### Step 2.5: Look for Overlaps with Remaining Context

```
Current state:
  root = "abab"(3)
  cursor_position = 2
  remaining = [a, b] at positions 2-3

Question: Do any postfixes of root "abab" overlap into remaining?

Postfix iteration of "abab"(3):
  postfix_iter() yields: [(pos:2, "ab"(2))]
  
  "ab"(2) at position 2 in "abab":
    
    "abab" = [a, b, a, b]
              0  1  2  3
                    └──┤
                 postfix "ab" starts at position 2

Check: Can "ab"(2) match remaining [a, b]?
  - remaining pattern = [a, b]
  - "ab" is parent of [a, b]
  - YES! This is an overlap!
```

### Step 2.6: Expansion Creates Overlap Band

```
                    Overlap visualization:
                    
Position in full input "ababab":
     0   1   2   3   4   5
     a   b   a   b   a   b
     ├───────────────┤
     root "abab"(3) covers [0,4)
                 ├───────────┤
         postfix "ab" expands to cover [2,4) + remaining [4,6)
         
Combined coverage:
     0   1   2   3   4   5
     a   b   a   b   a   b
     ├───────────────────────┤
     "ababab" = [abab, ab] decomposition found!

Alternative decomposition (via postfix complement):
     0   1   2   3   4   5
     a   b   a   b   a   b
     ├───┤   ├───────────────┤
     "ab"    "abab"
     
     "ababab" = [ab, abab] decomposition!
```

### Step 2.7: Finding the Complement

```
When postfix "ab" overlaps into remaining context:

Root "abab":
  child_pattern = [a, b, a, b]
                   0  1  2  3
                        └──┤ postfix "ab" starts at index 2
  
Complement = everything BEFORE postfix_start:
  [a, b] at indices 0-1 in root's pattern
  → This is token "ab"(2)!

So the overlap gives us TWO decompositions:
  1. [root_before_postfix, postfix_expanded] = [ab, abab]
  2. [current_block, remaining] = [abab, ab]
```

### Step 2.8: Updated BandChain

```
BandChain {
  bands: [
    Band {
      pattern: [abab],      ← First block (sequential bundling)
      start_bound: 0,
      end_bound: 4,
    },
    Band {
      pattern: [ab, abab],  ← Overlap decomposition!
      start_bound: 0,       ← starts from root beginning
      end_bound: 6,         ← covers full input
    }
  ]
}
```

### Step 2.9: Commit BandChain to Root

```
commit_chain():

1. Main pattern: root + last_block = abab + ab = "ababab"(4)
   Child pattern 1: [abab, ab]

2. Overlap band [ab, abab]:
   Child pattern 2: [ab, abab]

Result:
  root = "ababab"(4) with TWO child patterns:
    - [abab, ab]
    - [ab, abab]
```

---

## Final Graph State

```
"a"(0) ──┬── parent → "ab"(2) ──┬── parent → "abab"(3) ──┬── parent → "ababab"(4)
"b"(1) ──┘                      │                        │
                                └────────────────────────┘
                                (direct parent)

Vertices created: {a, b, ab, abab, ababab}

"ababab"(4):
  child_patterns:
    - [abab, ab]
    - [ab, abab]  ← from overlap expansion
```

---

## Algorithm Summary

```
┌────────────────────────────────────────────────────────────────────┐
│                      BLOCK ITERATION FLOW                          │
└────────────────────────────────────────────────────────────────────┘

1. SEGMENTATION
   ┌─────────────────────────────────────────────────────────────────┐
   │ Input → split into (unknown, known) pairs                       │
   │ unknown = atoms not in graph                                    │
   │ known = atoms already in graph                                  │
   └─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
2. PROCESS UNKNOWN
   ┌─────────────────────────────────────────────────────────────────┐
   │ insert_pattern(unknown) → creates root token                    │
   │ root = "ab", is_fresh = true                                    │
   └─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
3. PROCESS KNOWN (BlockExpansionCtx)
   ┌─────────────────────────────────────────────────────────────────┐
   │ while cursor not at end:                                        │
   │   3a. SEARCH: find largest prefix in known starting at cursor   │
   │   3b. APPEND: append matched token to root (or extend pattern)  │
   │   3c. ADVANCE: move cursor past matched portion                 │
   │   3d. BAND: add matched block to BandChain                      │
   └─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
4. OVERLAP DETECTION (ExpandCtx)
   ┌─────────────────────────────────────────────────────────────────┐
   │ For each postfix of current root/block token:                   │
   │   4a. Check if postfix can expand into remaining known          │
   │   4b. If match: find parent of (postfix + remaining)            │
   │   4c. COMPLEMENT: extract tokens before postfix in root         │
   │   4d. Add overlap band: [complement, expanded_postfix]          │
   └─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
5. COMMIT (RootManager)
   ┌─────────────────────────────────────────────────────────────────┐
   │ commit_chain(band_chain):                                       │
   │   5a. Combine root + final_block → new root token               │
   │   5b. Main child pattern: sequential decomposition              │
   │   5c. For each overlap band: add as additional child pattern    │
   │   5d. Use split cache to find complement tokens efficiently     │
   └─────────────────────────────────────────────────────────────────┘
```

---

## Key Data Structures

### PatternCursor
```rust
PatternCursor {
    pattern: [a, b, a, b],  // The known pattern
    position: 2,            // Current atom position (0-indexed)
    // position tracks how much we've consumed
}
```

### BandChain
```rust
BandChain {
    bands: BTreeSet<Band>,  // Ordered by end_bound
}

Band {
    pattern: Pattern,       // Tokens in this band (e.g., [ab] or [ab, abab])
    start_bound: AtomPosition,
    end_bound: AtomPosition,
}
```

### Split Cache (for complement finding)
```rust
// When we find postfix "ab" at position 2 in "abab":
// The split cache records:
SplitCache {
    token: "abab",
    split_position: 2,
    prefix: "ab",   // complement
    postfix: "ab",  // the postfix we expanded
}
```

---

## Why "ab" Must Be Created

The intermediate vertex "ab" is essential because:

1. **It's the match result**: When searching known=[a,b,a,b], we find "ab" as the largest prefix
2. **It's a postfix**: "ab" is a postfix of "abab", enabling overlap detection
3. **It's a complement**: When "ab" (postfix) expands into remaining, the complement before it is also "ab"
4. **Decomposition correctness**: "ababab" needs [ab, abab] and [abab, ab] decompositions

Without "ab" existing in the graph, we lose:
- The ability to detect overlaps via postfix iteration
- The ability to construct complement patterns
- The correct graph structure for pattern matching
