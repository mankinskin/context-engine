---
tags: `#plan` `#documentation` `#skills` `#dungeon-crawler` `#hypergraph` `#educational` `#external-facing`
summary: Create 4 educational skill documents in docs/skills/ that explain the hypergraph model and algorithms using dungeon-crawler game examples.
status: 📋
parent: 20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md
depends_on:
  - 20260218_PLAN_CONTEXT_READ_COMPLETION.md
  - (future) PLAN_INSERT_NEXT_MATCH.md
  - (future) PLAN_READ_STREAM_DESIGN.md
design_decision: D15 — Skill docs in `docs/skills/` (separate from `agents/guides/`)
---

# Plan: Dungeon Crawler Skill Documentation

**Date:** 2026-03-14
**Scope:** Medium (4 new documents + index, no code changes, requires algorithm verification)
**Location:** `docs/skills/`
**Audience:** External — engineers new to the context-engine

---

## Table of Contents

1. [Objective](#objective)
2. [Context](#context)
3. [Directory Structure](#directory-structure)
4. [Document Template](#document-template)
5. [Content Outlines](#content-outlines)
6. [Execution Steps](#execution-steps)
7. [Validation](#validation)
8. [Risks & Mitigations](#risks--mitigations)
9. [Related Documents](#related-documents)

---

## Objective

Create **4 educational skill documents** in `docs/skills/` that explain the context-engine's hypergraph model and core algorithms using **dungeon-crawler terminal game examples** as the running analogy. These are external-facing documents designed for engineers encountering the context-engine for the first time.

The goal: make the abstract concepts of tokens, patterns, read algorithms, insertion, and overlapping decompositions **tangible and memorable** by grounding them in a domain everyone can visualize — exploring dungeon rooms, collecting items, reading scrolls, and crafting weapons.

---

## Context

### Parent Plan

This plan is a child of `20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md` (Phase 2: Design). It was identified during the research interviews as one of 6 plan files to create during the design phase.

### Dependencies

Understanding the algorithms documented in these skill files requires familiarity with:

- **`insert_next_match`** (formerly `insert_or_get_complete`) — the core insertion loop documented in the future `PLAN_INSERT_NEXT_MATCH.md`
- **Read stream design** — the text→atoms→segments→expansion→root pipeline documented in the future `PLAN_READ_STREAM_DESIGN.md`
- **`context-search`** — the search algorithm that finds largest known prefixes
- **`context-trace`** — the foundational graph structures (vertices, patterns, atoms)

These skill documents can be written in parallel with the algorithm plans. The examples should be verified against actual `context-cli` behavior once the algorithm plans are implemented.

### Design Decision D15

> Skill docs go in `docs/skills/` (not `agents/guides/`).

Rationale: `agents/guides/` contains internal development guides for AI agents working on the codebase. Skill documents are **external-facing educational content** for humans learning the system. They serve different audiences and have different writing styles.

### Why Dungeon Crawlers?

A dungeon-crawler terminal game maps perfectly onto hypergraph concepts:

| Game Concept | Hypergraph Concept | Why It Works |
|---|---|---|
| Dungeon map (rooms + corridors) | Graph topology (vertices + edges) | Spatial navigation = graph traversal |
| Items (sword, potion) | Atoms / tokens | Discrete named objects |
| Compound items ("magic sword") | Compound tokens (patterns of atoms) | Composition from primitives |
| Inventory (ordered item list) | Patterns (ordered token sequences) | Ordered collections |
| Exploring rooms to find items | Search algorithm | Traversal to discover structure |
| Crafting (combining items) | Insert algorithm | Combining tokens to create new ones |
| Reading a scroll | Read algorithm | Decomposing text into known components |
| Same room reachable via multiple paths | Overlapping decompositions | Multiple valid routes to the same vertex |

Dungeon-crawler game logs also produce **highly repetitive text** — `"You see a goblin"`, `"Room: Cave"`, `"HP: 100/100"` — making them perfect for demonstrating hierarchical deduplication.

---

## Directory Structure

```
docs/
└── skills/
    ├── README.md                              ← Index + reading order + prerequisites
    ├── 01_the_hypergraph_model.md             ← Skill 1: Tokens All the Way Down
    ├── 02_reading_text.md                     ← Skill 2: Iterative Largest-Match
    ├── 03_context_completion.md               ← Skill 3: Finding Meaning in Small Tokens
    └── 04_overlapping_decompositions.md       ← Skill 4: Why One Token Has Many Patterns
```

### Naming Convention

- Numbered prefix (`01_`, `02_`, ...) for reading order
- Lowercase with underscores
- No date prefix (these are living documents, not timestamped plans)

### Relationship to Other Documentation

```
docs/skills/          ← External-facing educational (THIS PLAN)
agents/guides/        ← Internal agent development guides
agents/plans/         ← Task plans (like this file)
agents/implemented/   ← Completed implementation summaries
crates/*/HIGH_LEVEL_GUIDE.md  ← Per-crate technical reference
```

---

## Document Template

Each skill document follows a consistent structure:

```markdown
# [Title] — [Subtitle]

> **Skill Level:** Beginner / Intermediate / Advanced
> **Prerequisites:** [Links to prior skills]
> **Time to Complete:** ~N minutes

## The Dungeon Analogy

[Opening story — 2-3 paragraphs setting the scene in dungeon-crawler terms.
This section hooks the reader and creates the mental model they'll carry
through the rest of the document.]

## Core Concept

[Formal definition — what the concept actually is in hypergraph terms.
This section bridges from the analogy to the real model. Includes:
- Precise definitions
- Mathematical properties where relevant
- Relationship to other concepts]

## Worked Example

[Step-by-step walkthrough with actual graph operations.
Shows real inputs and outputs. Uses monospace formatting for
token representations. Each step is numbered and annotated.]

### Step 1: [Description]
[Input → Operation → Output, with explanation]

### Step 2: [Description]
...

## Visual Diagram

[ASCII art showing the structure. Large enough to be clear,
small enough to fit in a terminal. Annotated with labels.]

## Key Takeaways

- [Bullet 1: most important insight]
- [Bullet 2: common misconception addressed]
- [Bullet 3: connection to broader system]

## Try It Yourself

[CLI commands that reproduce the worked example. Copy-pasteable.
Includes expected output for verification.]

```bash
# Step 1: Create a workspace
context-cli create dungeon-demo

# Step 2: ...
```

## Related Skills

- **Previous:** [Link to prior skill]
- **Next:** [Link to next skill]
- **Deep Dive:** [Link to relevant HIGH_LEVEL_GUIDE.md or plan]
```

---

## Content Outlines

### Skill 1: "The Hypergraph Model — Tokens All the Way Down"

**File:** `docs/skills/01_the_hypergraph_model.md`
**Skill Level:** Beginner
**Prerequisites:** None
**Estimated Length:** ~400 lines

#### The Dungeon Analogy

- Scene: You are a cartographer mapping an underground dungeon
- The dungeon has **rooms** (vertices) connected by **corridors** (edges)
- Each room contains a **treasure chest** with items inside (a token's child patterns)
- Some items are **atomic** — a single gold coin, a single gem (atoms = single characters)
- Some items are **compound** — a "magic sword" is really [enchantment scroll + iron sword] (compound tokens = patterns of other tokens)
- A **treasure map** (pattern) is an ordered list of items describing how to decompose a room's treasure
- Key twist: every item IS also a room — you can always "zoom in" and find sub-items
- The dungeon goes **all the way down**: rooms contain items, items are rooms, those rooms contain items...

#### Core Concept

- **Vertex**: a node in the hypergraph. Has a unique numeric index. Every token is a vertex.
- **Token**: the content associated with a vertex. Defined by its **child patterns** — sequences of other tokens that decompose it.
- **Atom**: a leaf token with no children. Represents a single indivisible character (e.g., `'a'`, `'b'`, `' '`).
- **Pattern**: an ordered sequence of token references stored as a child of some parent token. `"abc"` might have child pattern `[a, b, c]` (three atoms).
- **Width**: the total number of atoms reachable by fully expanding all patterns. `"abc"` has width 3. A compound token `["abc", "def"]` has width 6. **Width = total atom count, NOT direct children count.**
- **Parent references**: every token tracks which larger tokens contain it as part of a pattern. `"abc"` is a parent of `"a"`, `"b"`, and `"c"`.
- **The containment invariant**: a path exists between two vertices if and only if one is a substring of the other.

#### Worked Example: Building a Small Dungeon

Build a graph representing a simple dungeon with these rooms/items:

1. Start with atoms: `a`, `b`, `c`
2. Create token `"ab"` = `[a, b]`
3. Create token `"bc"` = `[b, c]`
4. Create token `"abc"` = `[a, b, c]` — but also `[ab, c]` and `[a, bc]`
5. Show that `"abc"` has **three** child patterns (three ways to decompose it)
6. Show width calculations: `"ab"` → width 2, `"abc"` → width 3

#### Visual Diagram: ASCII Hypergraph

```
                    ┌───────────────────────────┐
                    │  "abc"  (vertex 5, w=3)   │
                    │                           │
                    │  patterns:                │
                    │    [a, b, c]              │
                    │    [ab, c]                │
                    │    [a, bc]                │
                    └─────┬──────────┬──────────┘
                          │          │
               ┌──────────┘          └──────────┐
               ▼                                ▼
    ┌──────────────────┐             ┌──────────────────┐
    │ "ab" (v3, w=2)   │             │ "bc" (v4, w=2)   │
    │ patterns: [a, b] │             │ patterns: [b, c] │
    └───┬─────────┬────┘             └───┬─────────┬────┘
        │         │                      │         │
        ▼         ▼                      ▼         ▼
   ┌────────┐ ┌────────┐           ┌────────┐ ┌────────┐
   │ a (v0) │ │ b (v1) │           │ b (v1) │ │ c (v2) │
   │ atom   │ │ atom   │           │ atom   │ │ atom   │
   └────────┘ └────────┘           └────────┘ └────────┘
```

- Note `b` (vertex 1) is shared between `"ab"` and `"bc"` — deduplication!
- Note `"abc"` has three decomposition paths — overlapping decompositions!

#### Key Takeaways

- Every token IS a vertex, and vertices contain patterns of other tokens
- Width = total atom count, not direct children count
- The same atom/token can appear in many parent patterns (shared structure)
- Multiple child patterns = multiple valid decompositions of the same token
- The graph stores ALL valid decompositions, not just one

#### Try It Yourself

```bash
# Create a workspace and add atoms
context-cli workspace create dungeon-demo
context-cli read dungeon-demo "abc"

# Inspect the resulting graph
context-cli inspect dungeon-demo --all
# Look for: atoms a, b, c and compound tokens ab, bc, abc
```

#### Related Skills

- **Next:** [02 — Reading Text](02_reading_text.md)
- **Deep Dive:** `crates/context-trace/HIGH_LEVEL_GUIDE.md`

---

### Skill 2: "Reading Text — The Iterative Largest-Match Algorithm"

**File:** `docs/skills/02_reading_text.md`
**Skill Level:** Intermediate
**Prerequisites:** Skill 1 (The Hypergraph Model)
**Estimated Length:** ~450 lines

#### The Dungeon Analogy

- Scene: You are a **scout** who has just found an ancient scroll in a dungeon room
- The scroll describes a path through rooms you may have already explored
- Your job: decode the scroll by matching its text against your **existing map** (the graph)
- Strategy: at each position, find the **longest known sequence** starting there
  - If you recognize "Room: Cave" as a single known landmark, great — mark it and move on
  - If you only recognize "Room" and ": Cave" separately, mark those two pieces
  - If you see something completely new, you add it to your map one character at a time
- The scout reads left to right, always greedily matching the biggest thing they know
- After the scroll is fully decoded, your map has grown — new landmarks have been added

#### Core Concept

The read algorithm is an **iterative largest-match** process:

1. **Atomize**: convert input text to a sequence of atoms (one per character)
2. **Search**: at current cursor position, find the largest known token starting here
3. **Advance**: move cursor past the matched token
4. **Repeat**: go back to step 2 until the entire input is consumed
5. **Build root**: the sequence of matched tokens becomes a child pattern of a new root token

The key operation is `insert_next_match` — it searches for the biggest match, potentially creates a new compound token via split-join insertion, and returns both the matched token and how far to advance.

#### Worked Example 1: Reading "abcabc" When "abc" Is Known

Starting state: graph contains atoms `a`, `b`, `c` and token `"abc"` = `[a, b, c]`.

```
Input: a b c a b c
       ^
Cursor position: 0

Step 1: Search from position 0
  → Largest match: "abc" (width 3)
  → Record: ["abc"]
  → Advance cursor to position 3

Input: a b c a b c
             ^
Cursor position: 3

Step 2: Search from position 3
  → Largest match: "abc" (width 3)
  → Record: ["abc", "abc"]
  → Advance cursor to position 6

Cursor at end. Done!
Result: root token "abcabc" with child pattern ["abc", "abc"]
```

#### Worked Example 2: Reading "abcdef" When "abc" and "def" Are Known

```
Input: a b c d e f
       ^
Step 1: Largest match from 0 → "abc" (w=3), advance to 3
Step 2: Largest match from 3 → "def" (w=3), advance to 6
Result: root "abcdef" with pattern ["abc", "def"]
```

#### Worked Example 3: Reading "abcbcd" When "abc" and "bcd" Are Known (Overlap!)

```
Input: a b c b c d
       ^
Step 1: Largest match from 0 → "abc" (w=3), advance to 3

Input: a b c b c d
             ^
Step 2: Largest match from 3 → "bcd" (w=3), advance to 6

Result: root "abcbcd" with pattern ["abc", "bcd"]

But wait — "abc" ends with "bc" and "bcd" starts with "bc"!
The overlap "bc" is detected and collapsed.
See Skill 4 for how overlapping decompositions are handled.
```

#### Visual Diagram: Cursor Advancement

```
Reading "You see a goblin" (second time, "You see a " already known):

Position: 0         5         10        15
Input:    Y o u ' ' s e e ' ' a ' ' g o b l i n
          ├─────────────────────┤├──────────────┤
          "You see a "           "goblin"
          (KNOWN, w=10)          (search continues...)

Cursor trace:
  [0] ──search──► match "You see a " (w=10)
                  ──advance──► [10]
  [10] ──search──► match "goblin" (w=6)
                   ──advance──► [16]
  [16] ──end──► Done!

Root: ["You see a ", "goblin"]  ← reuses existing "You see a " token!
```

#### Key Takeaways

- The read algorithm is **greedy** — it always takes the largest match at each position
- Greedy matching maximizes compression (reuses the biggest known structures)
- The cursor only moves forward, never backward — linear scan
- New tokens are created for previously unseen sequences
- Known sequences are reused, enabling structural deduplication
- Overlaps between adjacent matches are detected but resolved separately (→ Skill 4)

#### Try It Yourself

```bash
# First read: everything is new
context-cli read dungeon-demo "You see a goblin"

# Second read: "You see a " is now known
context-cli read dungeon-demo "You see a chest"

# Inspect: "You see a " should be shared between both
context-cli inspect dungeon-demo "You see a "
```

#### Related Skills

- **Previous:** [01 — The Hypergraph Model](01_the_hypergraph_model.md)
- **Next:** [03 — Context Completion](03_context_completion.md)
- **Deep Dive:** `crates/context-read/HIGH_LEVEL_GUIDE.md`

---

### Skill 3: "Context Completion — Finding Meaning in Small Tokens"

**File:** `docs/skills/03_context_completion.md`
**Skill Level:** Intermediate
**Prerequisites:** Skill 1 (The Hypergraph Model), Skill 2 (Reading Text)
**Estimated Length:** ~400 lines

#### The Dungeon Analogy

- Scene: You are a **craftsman** in the dungeon's workshop
- You have a table full of components: iron ingots, enchantment scrolls, leather straps
- Your job: when someone brings you a request like "make a magic sword", you try to **extend** what you know
  - Start with the biggest component you recognize — maybe "magic" is known, or "sword" is known
  - Try to combine it with adjacent components: can "magic" + " " + "sword" form something known?
  - If the full combination already exists → **Complete** (nothing new to craft)
  - If the combination is new → **Created** (you've crafted a new compound item)
  - If the extension fails (you don't know the next piece at all) → **NoExpansion** (you can only work with what you have)
- Three outcomes of the crafting attempt: Created, Complete, or NoExpansion

#### Core Concept: `insert_next_match`

The `insert_next_match` function is the core insertion loop:

1. **Search**: find the largest known token that matches the input starting at the current position
2. **Try to extend**: can we combine this match with the next token to form something bigger?
3. **Return outcome**:
   - **`Created`** — a new compound token was created by joining existing pieces (split-join insertion)
   - **`Complete`** — the entire input already exists in the graph, no new token needed
   - **`NoExpansion`** — the matched portion is recorded, but the remaining input starts a new segment

This is not just a read operation — it **mutates the graph** by inserting new tokens when needed.

#### Worked Example 1: Inserting "hello" When "hel" and "lo" Exist

```
Graph state: atoms h,e,l,o + tokens "hel"=[h,e,l], "lo"=[l,o]

Insert "hello":
  Step 1: Search from 0 → largest match = "hel" (w=3)
  Step 2: Remaining = "lo" → search → "lo" (w=2) is known!
  Step 3: Combine "hel" + "lo" → creates new token "hello" = ["hel", "lo"]
  
  Outcome: Created
  New token: "hello" (vertex N, w=5, pattern: ["hel", "lo"])
```

#### Worked Example 2: Inserting "hello" When "hello" Already Exists

```
Graph state: (includes "hello" from previous example)

Insert "hello":
  Step 1: Search from 0 → largest match = "hello" (w=5) — full match!
  Step 2: No remaining input.
  
  Outcome: Complete
  No new token created. Existing "hello" token returned.
```

#### Worked Example 3: Inserting "helloworld" When "hello" Exists But "world" Doesn't

```
Graph state: atoms + "hello"=[h,e,l,l,o]

Insert "helloworld":
  Step 1: Search from 0 → largest match = "hello" (w=5)
  Step 2: Remaining = "world" → search → no known token beyond atoms
  
  Outcome for "hello" segment: NoExpansion
  The "hello" match is recorded, cursor advances to position 5.
  
  Then "world" is processed separately:
  Step 3: Search from 5 → 'w' (w=1), then try extending...
  Eventually: atoms w,o,r,l,d are combined into "world" = [w,o,r,l,d]
  
  Final root: ["hello", "world"]
  New tokens: "world" (and possibly intermediate tokens)
```

#### Visual Diagram: The Extension Loop

```
insert_next_match("helloworld", cursor=0)

  ┌──────────────────────────────────────────────┐
  │ Input: h e l l o w o r l d                   │
  │        ├─────────┤├─────────┤                │
  │        "hello"    "world"                     │
  │        (KNOWN)    (UNKNOWN)                   │
  └──────────────────────────────────────────────┘

  Search: "hello" found (w=5)
  Extend: try "hello" + next → "hellow"? Not known.
  Result: NoExpansion for "hello"

  ──► cursor advances to 5 ──►

  Search: 'w' found (w=1), atoms only
  Build:  [w,o,r,l,d] → "world" Created
  
  ──► cursor advances to 10 ──► Done!
```

#### Key Takeaways

- `insert_next_match` is the workhorse: search, try to extend, return outcome
- Three possible outcomes: Created (new token), Complete (already exists), NoExpansion (can't extend further)
- The extension loop tries to combine adjacent matches into bigger tokens
- NoExpansion doesn't mean failure — it means the current segment is done and the next one begins
- The graph grows with each insertion — future reads benefit from previously created tokens

#### Try It Yourself

```bash
# Build up some tokens
context-cli read dungeon-demo "hel"
context-cli read dungeon-demo "lo"

# Now insert "hello" — should create by joining "hel" + "lo"
context-cli read dungeon-demo "hello"

# Inspect the result
context-cli inspect dungeon-demo "hello"
# EXPECT: token "hello" with child pattern ["hel", "lo"]

# Insert again — should be Complete (already exists)
context-cli read dungeon-demo "hello"
```

#### Related Skills

- **Previous:** [02 — Reading Text](02_reading_text.md)
- **Next:** [04 — Overlapping Decompositions](04_overlapping_decompositions.md)
- **Deep Dive:** `crates/context-insert/HIGH_LEVEL_GUIDE.md`

---

### Skill 4: "Overlapping Decompositions — Why One Token Has Many Patterns"

**File:** `docs/skills/04_overlapping_decompositions.md`
**Skill Level:** Advanced
**Prerequisites:** Skills 1-3
**Estimated Length:** ~500 lines

#### The Dungeon Analogy

- Scene: You are studying the **map room** — a room that can be reached through **multiple corridors**
- Corridor A comes from the north, Corridor B from the east, Corridor C from below
- Each corridor represents a different way to **decompose** the room's identity into a sequence of smaller rooms
- Example: the "Grand Hall" can be reached via:
  - North corridor: [Entrance, Passage, Grand Hall approach] → one decomposition
  - East corridor: [Library, Secret door, Grand Hall east wing] → different decomposition
  - Below: [Cavern, Staircase, Grand Hall basement] → yet another
- All three routes arrive at the **same room** — the Grand Hall has **multiple valid decomposition patterns**
- The map stores ALL routes, not just one, because each route reveals different structural relationships

#### Core Concept: Multiple Child Patterns

A single token (vertex) in the hypergraph can have **multiple child patterns**. Each child pattern is a valid decomposition — a different way to express the same string as a sequence of sub-tokens.

This arises naturally from **overlapping matches** during the read algorithm:

- When token A ends with substring X, and token B starts with substring X, the boundary region X creates an **overlap**
- The overlap means the combined token "AB" can be decomposed in multiple ways depending on where you "cut" through the overlap region
- The `BandState` mechanism detects these overlaps and creates all valid decomposition patterns

#### Worked Example: "abcabc" with Multiple Decompositions

Starting state: atoms `a`, `b`, `c` and token `"abc"` = `[a, b, c]`.

```
Read "abcabc":

Primary decomposition (from greedy largest-match):
  "abcabc" = ["abc", "abc"]

But the graph also discovers overlap-derived decompositions:

  "abcabc" = [a, bcabc]     ← if "bcabc" is built as [bc, abc]
  "abcabc" = [ab, cabc]     ← if "cabc" is built as [c, abc]  
  "abcabc" = [abca, bc]     ← if "abca" is built as [abc, a]
  "abcabc" = [abcab, c]     ← if "abcab" is built as [abc, ab]

Each decomposition is a valid child pattern of the vertex "abcabc".
The graph stores ALL of them.
```

#### The BandState Mechanism

During the read algorithm, when adjacent matches overlap, the system enters a special state:

```
Processing: ... "abc" "abc" ...
                    ↑
            These share internal structure.
            "abc" ends with [b,c] and "abc" starts with [a,b].
            The overlap region [b,c] ∩ [a,b] at the boundary
            creates alternative decomposition paths.

BandState transitions:
  Clean       → No overlap detected, standard append
  WithOverlap → Overlap detected between last committed and new match
  Collapsed   → Overlap resolved, alternative patterns recorded
```

The BandState tracks:

1. **The committed block**: the last token that was finalized in the root pattern
2. **The new match**: the token just found by search
3. **The overlap region**: the shared substring at the boundary
4. **The complement**: the non-overlapping portions on each side

When an overlap is detected, the system creates:
- The primary decomposition (greedy: `["abc", "abc"]`)
- Alternative decompositions based on different cuts through the overlap

#### Visual Diagram: One Token, Multiple Patterns

```
                 ┌─────────────────────────────────────┐
                 │     "abcabc"  (vertex N, width=6)   │
                 │                                     │
                 │  Child Patterns:                    │
                 │    Pattern 0: [abc, abc]     ← greedy decomposition
                 │    Pattern 1: [a, bcabc]     ← overlap variant
                 │    Pattern 2: [abcab, c]     ← overlap variant
                 │    ...                              │
                 └──────┬────────────┬─────────────────┘
                        │            │
            ┌───────────┘            └──────────┐
            ▼                                   ▼
   ┌─────────────────┐                ┌─────────────────┐
   │ "abc" (w=3)     │                │ "bcabc" (w=5)   │
   │ [a, b, c]       │                │ [bc, abc]       │
   └─────────────────┘                └─────────────────┘
            ▲                                   │
            │                          ┌────────┘
            │                          ▼
            │                 ┌─────────────────┐
            └─────────────────│ "bc" (w=2)      │
              (shared child)  │ [b, c]          │
                              └─────────────────┘
```

Note: `"abc"` appears as a child in BOTH Pattern 0 of `"abcabc"` AND Pattern 1's `"bcabc"`. The shared structure means the graph captures the relationship between all these decompositions efficiently.

#### Worked Example: Dungeon Screen Boundaries

```
Game screen 1: "Room: Cave\nYou see a goblin"
Game screen 2: "You see a goblin\nHP: 100/100"

After reading screen 1:
  root1 = ["Room: Cave\n", "You see a goblin"]

After reading screen 2:
  root2 = ["You see a goblin", "\nHP: 100/100"]

Now read the combined log: "Room: Cave\nYou see a goblin\nHP: 100/100"

  The token "You see a goblin" appears at the END of the first segment
  and the START of the second segment.

  Decomposition 1 (by screen boundary):
    ["Room: Cave\n", "You see a goblin", "\nHP: 100/100"]
    
  Decomposition 2 (by sentence):
    ["Room: Cave\n", "You see a goblin\nHP: 100/100"]
    
  Both are valid child patterns of the combined log token!
  The overlap at "You see a goblin" creates the alternative decompositions.
```

#### Why Store All Decompositions?

Storing all valid decompositions enables:

1. **Different traversal strategies** — a search can choose the decomposition that best fits its goal
2. **Structural analysis** — comparing decompositions reveals which sub-tokens are "load-bearing" (appear in many decompositions) vs. incidental
3. **Compression analysis** — the number of decompositions indicates how much internal structure a token has
4. **Pattern discovery** — overlapping decompositions reveal hidden relationships between sub-tokens

#### Key Takeaways

- A single token can have **multiple child patterns** — each is a valid decomposition
- Multiple patterns arise naturally from **overlapping matches** during reading
- The `BandState` mechanism detects overlaps at match boundaries and creates alternative decomposition patterns
- The graph stores **ALL** valid decompositions, not just the greedy one
- This is a feature, not a bug — multiple decompositions enable richer structural analysis
- The number of decompositions grows with the token's internal repetitive structure

#### Try It Yourself

```bash
# Build up "abc"
context-cli read dungeon-demo "abc"

# Read "abcabc" — should create overlapping decompositions
context-cli read dungeon-demo "abcabc"

# Inspect the token "abcabc" — look for multiple child patterns
context-cli inspect dungeon-demo "abcabc"
# EXPECT: Multiple child patterns listed

# Try a dungeon-flavored example
context-cli read dungeon-demo "You see a goblin"
context-cli read dungeon-demo "You see a chest"
context-cli read dungeon-demo "You see a goblin. You see a chest."
context-cli inspect dungeon-demo "You see a "
# EXPECT: "You see a " is shared across multiple parent tokens
```

#### Related Skills

- **Previous:** [03 — Context Completion](03_context_completion.md)
- **Deep Dive:** `crates/context-read/HIGH_LEVEL_GUIDE.md`, `crates/context-insert/HIGH_LEVEL_GUIDE.md`

---

## Execution Steps

### Phase 1: Preparation

- [ ] **Step 1: Create `docs/skills/` directory and `README.md` index**
  - Create the directory structure
  - Write `README.md` with:
    - Title: "Context-Engine Skill Documents"
    - Purpose statement: educational documents for learning the hypergraph model
    - Reading order table (4 skills, numbered, with difficulty levels)
    - Prerequisites section (what you should know before starting)
    - The dungeon crawler concept (brief explanation of why this analogy)
    - Links to each skill document
    - Link to related internal documentation (`agents/guides/`, crate `HIGH_LEVEL_GUIDE.md` files)
  - **Verification:** `README.md` renders correctly, all links are valid (even if targets don't exist yet)

### Phase 2: Write Skill Documents

- [ ] **Step 2: Write Skill 1 — The Hypergraph Model**
  - Follow the template structure
  - Focus on: vertex, token, atom, pattern, width, child patterns, parent references
  - ASCII art: 5-6 vertex hypergraph with shared atoms
  - Ensure the "width = total atom count" insight is prominent
  - **Verification:** All terminology is consistent with `crates/context-trace/HIGH_LEVEL_GUIDE.md`

- [ ] **Step 3: Write Skill 2 — Reading Text**
  - Follow the template structure
  - Focus on: the iterative largest-match loop, cursor advancement, greedy strategy
  - Three worked examples: simple match, multi-match, overlap detection
  - Cursor advancement ASCII diagram
  - **Verification:** Step-by-step examples match the documented algorithm in `context-read`

- [ ] **Step 4: Write Skill 3 — Context Completion**
  - Follow the template structure
  - Focus on: `insert_next_match`, Created/Complete/NoExpansion outcomes
  - Three worked examples: join existing, already complete, partial match
  - Extension loop diagram
  - **Verification:** Outcomes match the documented `InsertOutcome` enum behavior

- [ ] **Step 5: Write Skill 4 — Overlapping Decompositions**
  - Follow the template structure
  - Focus on: multiple child patterns, BandState mechanism, overlap detection
  - Two worked examples: character-level overlap, dungeon screen boundaries
  - Multiple-pattern vertex diagram
  - **Verification:** Overlap examples are consistent with BandState logic in `context-read`

### Phase 3: Verification

- [ ] **Step 6: Review examples for accuracy**
  - Run all "Try It Yourself" CLI commands against actual `context-cli`
  - Verify expected outputs match actual outputs
  - Fix any discrepancies between documented behavior and actual behavior
  - Document any cases where actual behavior differs from expected (these may indicate bugs to file)
  - **Verification:** All CLI examples produce the documented output

- [ ] **Step 7: Cross-reference with crate HIGH_LEVEL_GUIDE.md documents**
  - Ensure terminology matches across all documents
  - Ensure no contradictions between skill docs and internal guides
  - Add cross-reference links where appropriate
  - Update `README.md` index if any changes were made
  - **Verification:** No terminology conflicts; all cross-references are bidirectional

### Phase 4: Documentation Maintenance

- [ ] **Step 8: Update plans INDEX.md**
  - Add this plan to `agents/plans/INDEX.md`
  - Mark status as appropriate after completion
  - **Verification:** INDEX.md entry is accurate

---

## Validation

### Correctness Criteria

| Criterion | How to Verify |
|---|---|
| All 4 skill documents exist | `ls docs/skills/0*.md` returns 4 files |
| README.md index exists | `cat docs/skills/README.md` shows index with links |
| Template structure followed | Each doc has all 7 sections from the template |
| ASCII art renders correctly | View each doc in a terminal at 80 columns |
| CLI examples are runnable | Execute all `context-cli` commands in "Try It Yourself" sections |
| CLI examples produce expected output | Compare actual vs documented output |
| Terminology consistency | Grep for key terms across skill docs and HIGH_LEVEL_GUIDE.md files |
| Cross-references work | All `[links](targets)` resolve to existing files |
| No internal jargon leaks | Skill docs should not reference agent guides, plans, or internal implementation details |

### CLI Smoke Test

Run this sequence to verify the core examples work end-to-end:

```bash
# Setup
context-cli workspace create skill-test

# Skill 1 verification
context-cli read skill-test "abc"
context-cli inspect skill-test "abc"
# EXPECT: token with 3 atoms, width=3

# Skill 2 verification
context-cli read skill-test "abcabc"
context-cli inspect skill-test "abcabc"
# EXPECT: pattern ["abc", "abc"]

# Skill 3 verification
context-cli read skill-test "hel"
context-cli read skill-test "lo"
context-cli read skill-test "hello"
context-cli inspect skill-test "hello"
# EXPECT: pattern ["hel", "lo"]

# Skill 4 verification
context-cli inspect skill-test "abcabc"
# EXPECT: multiple child patterns

# Cleanup
context-cli workspace delete skill-test
```

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Algorithm behavior differs from documented examples | High | High | Run all examples against actual `context-cli` before publishing. Document discrepancies as known issues. |
| `insert_next_match` rename hasn't happened yet (still `insert_or_get_complete`) | High | Low | Use the conceptual name in skill docs with a footnote about the current API name. Update after rename. |
| CLI command names may change (context-api Phase 1 not yet complete) | Medium | Medium | Write examples with placeholder commands and update once CLI is finalized. Mark "Try It Yourself" sections as draft. |
| Overlapping decomposition examples may be oversimplified | Medium | Medium | Start with simple examples, add complexity warnings. Link to `context-read` HIGH_LEVEL_GUIDE for authoritative details. |
| BandState mechanism is still being redesigned | Medium | High | Focus Skill 4 on the *concept* of multiple decompositions rather than implementation details. Keep BandState references high-level. |
| Dungeon analogy breaks down for advanced concepts | Low | Low | Use the analogy as an entry point, then transition to formal definitions. Don't force the analogy where it doesn't fit. |
| Width calculation confusion (atoms vs direct children) | Medium | Medium | Include explicit counter-examples: "width is NOT 2 for [abc, def] — it's 6 because you count all atoms" |

---

## Related Documents

### Parent Plan
- [`20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md`](20260314_PLAN_CONTEXT_READ_UX_IMPROVEMENT.md) — Master plan that spawned this skill documentation effort

### Algorithm Reference Plans
- [`20260218_PLAN_CONTEXT_READ_COMPLETION.md`](20260218_PLAN_CONTEXT_READ_COMPLETION.md) — Read algorithm design and completion plan
- (future) `PLAN_INSERT_NEXT_MATCH.md` — Rename and refactor of the core insertion function
- (future) `PLAN_READ_STREAM_DESIGN.md` — Lazy atom resolution and stream-based reading

### Crate Documentation
- `crates/context-trace/HIGH_LEVEL_GUIDE.md` — Foundational graph structures (Skill 1 reference)
- `crates/context-search/HIGH_LEVEL_GUIDE.md` — Search algorithm (Skill 2 reference)
- `crates/context-insert/HIGH_LEVEL_GUIDE.md` — Insert algorithm (Skill 3 reference)
- `crates/context-read/HIGH_LEVEL_GUIDE.md` — Read algorithm (Skills 2-4 reference)

### Sibling Plans (from same design phase)
- (future) `PLAN_INTEGRATION_TESTS.md` — CLI integration test suite (can run in parallel with this plan)

---

## Notes

### Questions for User
- Should the "Try It Yourself" sections use `context-cli` subcommand syntax (Phase 1 design) or the REPL syntax? Current plan assumes subcommand syntax.
- Should skill docs include links to the 3D viewer for visualizing graphs? This would create a dependency on the viewer being deployed.

### Deviations from Plan
<!-- Track changes made during execution -->
- (none yet)

### Lessons Learned
<!-- Post-execution: what would you do differently? -->
- (none yet)