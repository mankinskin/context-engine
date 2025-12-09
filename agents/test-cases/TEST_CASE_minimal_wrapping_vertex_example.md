# Test Case: Minimal Wrapping Vertex Example

**Date:** 2025-12-09  
**Purpose:** Demonstrate the minimal wrapping vertex concept with a concrete example

## Test Scenario: Inserting "mnoxyp"

### Initial Graph Setup

```rust
// Create atoms
let [h, i, j, k, l, m, n, o, p, q, r, s, t, x, y] = graph.insert_atoms([
    Atom::Element('h'),
    Atom::Element('i'),
    Atom::Element('j'),
    Atom::Element('k'),
    Atom::Element('l'),
    Atom::Element('m'),
    Atom::Element('n'),
    Atom::Element('o'),
    Atom::Element('p'),
    Atom::Element('q'),
    Atom::Element('r'),
    Atom::Element('s'),
    Atom::Element('t'),
    Atom::Element('x'),
    Atom::Element('y'),
])[..] else { panic!() };

// Create composite tokens
let lmn = graph.insert_pattern(vec![l, m, n]);      // lmn
let opq = graph.insert_pattern(vec![o, p, q]);      // opq

// Create the original pattern: [h, i, j, k, lmn, x, y, opq, r, s, t]
let original_pattern = graph.insert_pattern(vec![
    h, i, j, k, lmn, x, y, opq, r, s, t
]);
```

### Insertion Request

Insert the pattern "mnoxyp" = `[m, n, o, x, y, p]`

```rust
let query = vec![m, n, o, x, y, p];
```

### Expected Behavior

#### 1. Identify Overlapping Range

The query overlaps with entries at indices 4-7 in the original pattern:
- Entry 4: `lmn` (overlaps with m, n)
- Entry 5: `x` (overlaps with x)
- Entry 6: `y` (overlaps with y)
- Entry 7: `opq` (overlaps with o, p)

These entries form the range `[lmn, x, y, opq]`

#### 2. Join Inner Partitions

During insertion, inner partitions are joined:
- `[x, y]` → `xy` (2 entries become 1 entry)
- This creates a "delta" of 1 (pattern size reduction)

The range becomes: `[lmn, xy, opq]`

#### 3. Create Minimal Wrapper Vertex

Create a wrapper for **only** the overlapping entries (4-7), not the entire pattern:

```rust
// Wrapper vertex "lmnxyopq" with two patterns:
lmnxyopq = [
    [lmn, xy, opq],      // Pattern 1: full entry tokens with joined middle
    [l, mnoxyp, q]       // Pattern 2: split tokens with inserted pattern
]
```

Where:
- `lmn` is the full entry token from index 4
- `xy` is the joined token from entries 5-6
- `opq` is the full entry token from index 7
- `l` is the first child of `lmn` (complement token)
- `mnoxyp` is the newly inserted pattern
- `q` is the last child of `opq` (complement token)

#### 4. Replace Range in Original Pattern

Replace entries [4, 5, 6, 7] with the wrapper:

```rust
// Original: [h, i, j, k, lmn, x, y, opq, r, s, t]
//                        └─────┬─────┘
//                        replaced with wrapper

// Result:   [h, i, j, k, lmnxyopq, r, s, t]
```

#### 5. Verify Surrounding Context

The surrounding tokens remain **unchanged**:
- Entries 0-3: `[h, i, j, k]` - unchanged
- Entry 4: `lmnxyopq` - new wrapper vertex
- Entries 5-7: `[r, s, t]` - unchanged (indices shifted)

### Expected Test Assertions

```rust
// The wrapper vertex should exist
assert_indices!(graph, lmnxyopq);

// The wrapper should have both patterns
assert_patterns! {
    graph,
    xy => [[x, y]],
    mnoxyp => [[m, n, o, x, y, p]],
    lmnxyopq => [
        [lmn, xy, opq],
        [l, mnoxyp, q]
    ]
};

// The original pattern should be updated
assert_patterns! {
    graph,
    original_pattern => [[h, i, j, k, lmnxyopq, r, s, t]]
};

// Verify we can find the query
let result = graph.search(&query);
assert!(result.query_exhausted());
assert_eq!(result.root_token(), original_pattern);
```

### Key Properties Verified

1. **Minimal Wrapping**: Only entries 4-7 are wrapped, not the entire pattern
2. **Context Preservation**: Surrounding tokens `[h, i, j, k]` and `[r, s, t]` are unchanged
3. **Pattern Size Delta**: The wrapper accounts for the size change when `[x, y]` → `xy`
4. **Multiple Representations**: The wrapper vertex contains both `[lmn, xy, opq]` and `[l, mnoxyp, q]`
5. **Query Findability**: The original query pattern can be found in the modified graph

## Benefits of This Approach

1. **No Context Duplication**: We don't create `[h, i, j, k, l, mnoxyp, q, r, s, t]` which would duplicate the surrounding context
2. **Efficient Storage**: Only the overlapping range is wrapped in a new vertex
3. **Minimal Changes**: The original pattern structure is preserved with minimal modification
4. **Correct Semantics**: The wrapper represents the exact range that overlaps with the insertion

## Comparison with Incorrect Approach

### ❌ Wrong: Atom-Level Wrapping
If we wrapped at the atom level, we might create:
- `[h, i, j, k, l, m, n, o, x, y, p, q, r, s, t]` (duplicates all context)
- Multiple unnecessary intermediate vertices

### ✅ Correct: Entry-Level Wrapping
We wrap only the entry range [4-7]:
- `[h, i, j, k, wrapper, r, s, t]` (preserves context)
- Single wrapper vertex for the overlapping range

## Implementation Notes

This test case demonstrates:
- How to identify the entry index range from role paths
- Why "delta" refers to pattern size changes during joining
- How to create minimal wrappers without duplicating context
- The importance of working at the pattern-entry level, not atom level
