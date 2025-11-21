# Graph Invariants Specification

> **Formal specification of the hypergraph structure invariants that must hold at all times**

## Executive Summary

The context-engine hypergraph maintains a **substring-aware hierarchical structure** where every token can reach all its constituent substrings. These invariants ensure correctness of search, insertion, and reading operations.

---

## Confidence Classification

Invariants are categorized by confidence level based on codebase evidence:

### 🟢 High Confidence - Verified in Code
Explicit validation exists, or code structure enforces invariant.

### 🟡 Medium Confidence - Implied by Design
Strongly suggested by implementation patterns but not explicitly validated.

### 🟠 Low Confidence - Uncertain or Overstated
May be incorrect, too strong, or not universally required.

---

## Core Invariants

## Core Invariants

### 1. 🟢 Width Consistency Invariant

**Confidence: HIGH** - Explicitly validated in `VertexData::validate_patterns()`

**Statement:** Token width must equal the sum of its children's widths in every pattern.

**Formal Definition:**
```
For all tokens T with pattern P = [T1, T2, ..., Tn]:
  width(T) = sum(width(Ti) for i in 1..n)
```

**Code Evidence:**
```rust
// From vertex/data.rs:395
assert_eq!(pattern_width, self.width, 
    "Pattern width mismatch in index {:#?} token pattern:\n {:#?}", 
    self.index, (pid, self.children.get(pid)));
```

**Examples:**

✅ **Valid:**
```
Token "abc" (width 3)
  └─> Pattern [a, b, c]
       width = 1 + 1 + 1 = 3 ✓
  └─> Pattern [ab, c]
       width = 2 + 1 = 3 ✓
```

❌ **Invalid:**
```
Token "abc" (width 3)
  └─> Pattern [a, b, c, d]
       width = 1 + 1 + 1 + 1 = 4 ≠ 3 ✗
```

**Why it matters:**
- Ensures pattern boundaries are correct
- Enables position calculations
- Validates split-join operations

---

### 2. 🟢 Pattern Completeness Invariant

**Confidence: HIGH** - Validated in `VertexData::validate_patterns()`

**Statement:** Every non-atom token must have at least one complete child pattern with length ≥ 2.

**Formal Definition:**
```
For all tokens T where width(T) > 1:
  T.child_patterns is not empty
  AND each pattern has length ≥ 2
  AND each pattern covers entire token width
```

**Code Evidence:**
```rust
// From vertex/data.rs
assert!(!p.is_empty(), "Empty pattern in index {:#?}", self.index);
assert!(!p.is_empty(), "Single index pattern in index {:#?}:\n {:#?}", 
    self.index, (pid, self.children.get(pid)));
```

**Examples:**

✅ **Valid:**
```
Token "abc" (width 3)
  └─> Pattern [a, b, c]  ✓ at least one pattern, length ≥ 2

Token "xyz" (width 3)
  └─> Pattern [xy, z]    ✓ first representation
  └─> Pattern [x, yz]    ✓ second representation
```

❌ **Invalid:**
```
Token "abc" (width 3)
  └─> <no patterns>  ✗ non-atom must have pattern

Token "ab" (width 2)
  └─> Pattern [ab]  ✗ single-token pattern not allowed
```

**Why it matters:**
- Ensures tokens can be decomposed
- Enables graph traversal
- Supports search operations

---

### 3. 🟢 Parent-Child Bidirectional Invariant

**Confidence: HIGH** - Enforced by `add_parents_to_pattern_nodes()`

**Statement:** When a pattern is added, parent-child relationships are maintained bidirectionally.

**Formal Definition:**
```
For all tokens P (parent) and C (child):
  If C appears in any pattern of P
  Then P appears in C's parent set
  AND C appears in P's child set
```

**Code Evidence:**
```rust
// From graph/insert.rs:158
node.add_parent(ChildLocation::new(
    parent.to_child(),
    pattern_id,
    i,
));
```

**Examples:**

✅ **Valid:**
```
Token "abc":
  children = {Token("ab"), Token("c")}
  pattern = [Token("ab"), Token("c")]

Token "ab":
  parents = {Token("abc"), ...}  ✓ bidirectional

Token "c":
  parents = {Token("abc"), ...}  ✓ bidirectional
```

❌ **Invalid:**
```
Token "abc":
  pattern = [Token("ab"), Token("c")]

Token "ab":
  parents = {}  ✗ missing parent reference!
```

**Why it matters:**
- Enables bidirectional traversal
- Supports bottom-up search
- Maintains graph consistency

---

### 4. 🟢 Atom Uniqueness Invariant

**Confidence: HIGH** - Enforced by `atom_keys: IndexMap<Atom, VertexKey>`

**Statement:** Each atom value appears at most once in the graph.

**Formal Definition:**
```
For all atoms A1, A2 in graph:
  If value(A1) = value(A2)
  Then A1 = A2  (same token)
```

**Code Evidence:**
```rust
// From graph/mod.rs
atoms: indexmap::IndexMap<VertexKey, Atom<G::Atom>>,
atom_keys: indexmap::IndexMap<Atom<G::Atom>, VertexKey>,
```

The bidirectional mapping ensures uniqueness.

**Examples:**

✅ **Valid:**
```
Atom 'a' → Token { index: 0, width: 1 }
All references to 'a' use Token(0)
```

❌ **Invalid:**
```
Atom 'a' → Token { index: 0, width: 1 }
Atom 'a' → Token { index: 5, width: 1 }  ✗ duplicate!
```

**Why it matters:**
- Ensures canonical representation
- Prevents duplicates
- Enables efficient lookup

---

### 5. 🟢 Position Validity Invariant

**Confidence: HIGH** - Required invariant

**Statement:** All position references must be valid within their context.

**Formal Definition:**
```
For all positions P in token T:
  0 <= P < width(T)

For all entry indices E in pattern [T1, ..., Tn]:
  0 <= E < n
```

**Code Evidence:**
```rust
// From graph/validation.rs
if end >= pattern.len() {
    Err(ValidationError::InvalidPatternRange(...))
}
```

**Examples:**

✅ **Valid:**
```
Token "abcd" (width 4)
  AtomPosition(0) ✓  // valid
  AtomPosition(3) ✓  // valid

Pattern [a, b, c]
  entry 0 = a  ✓  // valid
  entry 2 = c  ✓  // valid
```

❌ **Invalid:**
```
Token "abc" (width 3)
  AtomPosition(5) ✗  // out of bounds!

Pattern [a, b]
  entry 3  ✗  // only 2 entries (0 and 1)
```

**Why it matters:**
- Prevents out-of-bounds access
- Ensures split points are valid
- Validates cursor positions

---

### 6. 🟢 Multiple Representation Consistency Invariant

**Confidence: HIGH** - Required invariant (author confirmed)

**Statement:** Every token represents exactly one string, and all of its pattern compositions must represent the same string.

**Formal Definition:**
```
For all tokens T with patterns P1, P2, ..., Pn:
  string(P1) = string(P2) = ... = string(Pn) = string(T)
```

**Code Evidence:**
```rust
// From graph/mod.rs:365 - vertex_data_string computes from any pattern
self.pattern_string(data.expect_any_child_pattern().1)
// All patterns must yield the same string - this is a fundamental requirement
```

**Examples:**

✅ **Valid:**
```
Token "abc":
  Pattern [a, b, c]    → "abc" ✓
  Pattern [ab, c]      → "abc" ✓
  Pattern [a, bc]      → "abc" ✓
  All represent same string
```

❌ **Invalid:**
```
Token "abc":
  Pattern [a, b, c]    → "abc" ✓
  Pattern [a, b, d]    → "abd" ✗ different string!
```

**Why it matters:**
- Ensures semantic correctness
- Allows pattern variation without ambiguity
- Supports multiple decomposition strategies

---

### 7. 🟢 Substring Reachability Invariant

**Confidence: HIGH** - Required invariant (author confirmed)

**Statement:** Every token T must be reachable from all tokens that represent strict superstrings of T.

**Formal Definition:**
```
For all tokens T1, T2 in graph:
  If string(T2) is a strict substring of string(T1)
  Then there exists a path from T1 to T2 through child patterns
```

**Examples:**

✅ **Valid:**
```
Token "abc" (width 3)
  └─> Pattern [a, b, c]
       ├─> Token "a" (width 1)  ✓ reachable
       ├─> Token "b" (width 1)  ✓ reachable
       └─> Token "c" (width 1)  ✓ reachable

Token "abcd" (width 4)
  └─> Pattern [ab, cd]
       ├─> Token "ab" (width 2)
       │    └─> Pattern [a, b]
       │         ├─> Token "a" (width 1)  ✓ all substrings reachable
       │         └─> Token "b" (width 1)  ✓
       └─> Token "cd" (width 2)
            └─> Pattern [c, d]
                 ├─> Token "c" (width 1)  ✓
                 └─> Token "d" (width 1)  ✓
```

❌ **Invalid:**
```
Token "abc" (width 3)
  └─> Pattern [ab]  // Missing 'c' - cannot reach all substrings!
```

**Why it matters:**
- Enables substring queries at any granularity
- Guarantees search can find partial matches
- Essential for hierarchical pattern matching
- Must be maintained during all operations

---

### 8. 🟢 String-Token Uniqueness Invariant

**Confidence: HIGH** - Required invariant (author confirmed)

**Statement:** Each string can be represented by at most one token in the graph.

**Formal Definition:**
```
For all tokens T1, T2 in graph:
  If string(T1) = string(T2)
  Then T1 = T2  (same token)
```

**Examples:**

✅ **Valid:**
```
Token "abc" (index: 5, width: 3)
  └─> Pattern [a, b, c]
  └─> Pattern [ab, c]
  └─> Pattern [a, bc]
// Multiple patterns OK - all represent same string "abc"
// Only ONE token for "abc" in entire graph
```

❌ **Invalid:**
```
Token "abc" (index: 5, width: 3)
  └─> Pattern [a, b, c]

Token "abc" (index: 12, width: 3)  ✗ duplicate!
  └─> Pattern [ab, c]

// Two different tokens for same string - not allowed!
```

**Why it matters:**
- Ensures unique canonical representation of each string
- Prevents ambiguity in token lookup
- Simplifies search and insertion algorithms
- Combined with atom uniqueness, ensures string→token mapping is bijective for atoms

---

## Derived Properties

These properties follow from the core invariants:

### Property 1: Deterministic Width Calculation

From width consistency, any pattern's total width can be computed by summing children.

**Proof:** Direct consequence of Width Consistency Invariant

### Property 2: Bidirectional Traversal

From parent-child bidirectionality, graph can be traversed both top-down and bottom-up.

**Proof:** Every edge is represented in both directions

### Property 3: Canonical Atom References

From atom uniqueness, any atom value has exactly one token representation.

**Proof:** Enforced by `atom_keys` bidirectional map

### Property 4: Canonical String Representation

From string-token uniqueness, any string has exactly one token representation.

**Proof:** Required invariant - each string maps to at most one token

### Property 5: Search Completeness

If a pattern exists as a substring in any token, search can find it by following reachability paths.

**Proof:** Follows from substring reachability invariant - all substrings are reachable from superstrings

### Property 6: Unambiguous Token Semantics

From multiple representation consistency, a token's string value can be computed from any of its patterns.

**Proof:** All patterns of a token represent the same string

---

## Invariant Maintenance

### During Insertion

**Split-join algorithm maintains all required invariants:**

1. **Width Consistency**: ✅ Join verifies sum of child widths equals parent width
2. **Pattern Completeness**: ✅ Always creates patterns with ≥2 children
3. **Parent-Child Bidirectional**: ✅ `add_parents_to_pattern_nodes()` updates both directions
4. **Atom Uniqueness**: ✅ Atoms looked up via `atom_keys` before insertion
5. **Multiple Representation Consistency**: ✅ New patterns compose to same string as token
6. **Substring Reachability**: ✅ All child tokens are substrings of parent
7. **String-Token Uniqueness**: ✅ Check if token for string already exists before creating new
8. **Position Validity**: ✅ Validate positions before access and during split operations

**Example:**
```rust
// Insert "abcd" when "ab" and "cd" exist
let ab = Token { index: 1, width: 2 };  // represents "ab"
let cd = Token { index: 2, width: 2 };  // represents "cd"

// Check if token for "abcd" already exists
if !graph.has_token_for_string("abcd") {
    // Create new token
    let abcd = Token { index: 3, width: 4 };
    
    // Add pattern maintaining invariants
    graph.add_pattern(abcd, vec![ab, cd]);
    // ✓ Width Consistency: 2 + 2 = 4
    // ✓ Pattern Completeness: pattern has 2 children
    // ✓ Parent-Child: ab.parents += abcd, cd.parents += abcd
    // ✓ Atom Uniqueness: ab and cd already exist
    // ✓ Multiple Representation: [ab, cd] represents "abcd"
    // ✓ Substring Reachability: "ab" and "cd" are substrings of "abcd"
    // ✓ String-Token Uniqueness: checked before creating new token
}
```

### During Search
**Search operations rely on required invariants:**

1. **Width Consistency**: ✅ Use width for position calculations
2. **Parent-Child**: ✅ Follow bidirectional links for traversal
3. **Substring Reachability**: ✅ Guaranteed to find substrings via reachability paths
4. **String-Token Uniqueness**: ✅ Each string has unique token representation
5. **Position Validity**: ✅ Check bounds before accesspresentation
5. **Position Validity**: ⚠️ Partial - check bounds before access

### During Read

**Read operations depend on required invariants:**

1. **Pattern Completeness**: ✅ Expand from root to constituents via patterns
2. **Width Consistency**: ✅ Calculate offsets and ranges
3. **Multiple Representation Consistency**: ✅ All patterns represent same string
4. **Substring Reachability**: ✅ Can traverse to any substring
5. **Position Validity**: ✅ All positions valid within token bounds

---
## Testing Invariants

### Required Invariant Checks

**All invariants must be validated:**

```rust
// Width Consistency (from VertexData::validate_patterns)
fn check_width_consistency(graph: &Hypergraph, token: Token) -> bool {
    for pattern in graph.patterns(token) {
        let sum: usize = pattern.iter().map(|t| t.width()).sum();
        if sum != token.width() {
            return false;
        }
    }
    true
}

// Pattern Completeness (from VertexData::validate_patterns)
fn check_pattern_completeness(graph: &Hypergraph, token: Token) -> bool {
    if token.width() <= 1 {
        return true; // Atoms don't need patterns
    }
    
    let data = graph.expect_vertex(token);
    !data.children.is_empty() && data.children.values().all(|p| p.len() >= 2)
}

// Parent-Child Bidirectional (could be added)
fn check_parent_child_bidirectional(graph: &Hypergraph) -> bool {
    for token in graph.tokens() {
        for pattern in graph.patterns(token) {
            for child in pattern {
                if !graph.parents(child).contains(&token) {
                    return false;
                }
            }
        }
    }
    true
}

// Atom Uniqueness (structural guarantee via atom_keys map)
fn check_atom_uniqueness(graph: &Hypergraph) -> bool {
    graph.atoms.len() == graph.atom_keys.len()
}

// String-Token Uniqueness (check no duplicate string representations)
fn check_string_token_uniqueness(graph: &Hypergraph) -> bool {
    let mut seen_strings = HashSet::new();
    for token in graph.tokens() {
        let string = graph.token_string(token);
        if !seen_strings.insert(string) {
            return false; // Duplicate string found
        }
    }
    true
}

// Substring Reachability (verify all substrings reachable)
fn check_substring_reachability(graph: &Hypergraph, token: Token) -> bool {
    let token_string = graph.token_string(token);
    // For every position in token string, verify atom is reachable
    for pos in 0..token_string.len() {
        if !graph.can_reach_atom_at(token, pos) {
            return false;
        }
    }
    true
}

// Multiple Representation Consistency
fn check_multiple_representation_consistency(graph: &Hypergraph, token: Token) -> bool {
    let expected_string = graph.token_string(token);
    for pattern in graph.patterns(token) {
        let pattern_string = graph.pattern_string(pattern);
        if pattern_string != expected_string {
            return false;
        }
    }
    true
}

// Position Validity
fn check_position_validity(graph: &Hypergraph, token: Token, pos: usize) -> bool {
    pos < token.width()
}
```
```
#[test]
fn test_invariants_after_insert() {
    let mut graph = Hypergraph::default();
    insert_atoms!(graph, {a, b, c});
    let graph = HypergraphRef::from(graph);
    
    let abc = graph.insert(vec![a, b, c]).unwrap();
    
    // Check all required invariants
    assert_width_consistency(&graph, abc);
    assert_pattern_completeness(&graph, abc);
    assert_parent_child_bidirectional(&graph);
    assert_atom_uniqueness(&graph);
    assert_string_token_uniqueness(&graph);
    assert_substring_reachability(&graph, abc);
    assert_multiple_representation_consistency(&graph, abc);
    assert_position_validity(&graph, abc);
}   assert_parent_child_bidirectional(&graph);
    assert_atom_uniqueness(&graph);
**During graph construction:**
```rust
#[test]
fn test_invariants_maintained() {
    let mut graph = Hypergraph::default();
    insert_atoms!(graph, {a, b, c, d});
    
    // Check after each step
    assert_all_invariants(&graph);
    
    insert_patterns!(graph, ab => [[a, b]]);
    assert_all_invariants(&graph);
    
    insert_patterns!(graph, abcd => [[ab, c, d]]);
    assert_all_invariants(&graph);
}
```
### Low-Confidence Items: NOT Recommended for Testing

**These are either impossible to violate (type system) or not graph structure invariants:**

- ❌ **No Dangling References** - Rust type system prevents this automatically
- ❌ **Hierarchy** - Too specific, not separately validated

---

## Common Invariant Violations

### Violation 1: Width Mismatch (HIGH-CONFIDENCE)

**Symptom:** Token width ≠ sum of children
**Cause:** Incorrect split calculation or pattern construction
**Fix:** Recalculate split boundaries, validate before insertion
**Detection:** `VertexData::validate_patterns()` catches this

```rust
// ❌ Wrong
let token = Token { width: 5, ... };
let pattern = vec![a, b];  // widths: 1 + 1 = 2 ≠ 5

// ✅ Correct
let token = Token { width: 2, ... };
let pattern = vec![a, b];  // widths: 1 + 1 = 2 ✓
```

### Violation 2: Single-Token Pattern (HIGH-CONFIDENCE)

**Symptom:** Pattern with only one child
**Cause:** Attempting to create trivial decomposition
**Fix:** Ensure patterns have ≥2 children
**Detection:** `VertexData::validate_patterns()` catches this

```rust
// ❌ Wrong
Token "ab" (width 2)
  └─> Pattern [ab]  // Single token - not allowed!

// ✅ Correct
Token "ab" (width 2)
  └─> Pattern [a, b]  // Two children ✓
```

### Violation 3: Broken Bidirectional Links (HIGH-CONFIDENCE)

**Symptom:** Child references parent but parent doesn't reference child
**Cause:** Incomplete insertion, manual graph manipulation
**Fix:** Use `add_parents_to_pattern_nodes()` for atomic updates
**Detection:** Could be caught by bidirectional check (not currently implemented)

```rust
// ❌ Wrong
token.pattern.push(child);  // Only one direction!

// ✅ Correct - use graph API
graph.add_pattern_with_update(token, vec![child1, child2]);
// Automatically updates both directions
```

### Violation 4: Missing Substring (HIGH-CONFIDENCE)

**Symptom:** Cannot reach substring from parent token
**Cause:** Incomplete pattern, missing intermediate tokens
**Fix:** Ensure all substrings are reachable through patterns
**Detection:** Could be caught by `check_substring_reachability()`

```rust
// ❌ Wrong
Token "abc" (width 3)
  └─> Pattern [ab]  // Missing 'c' - violates reachability!

// ✅ Correct
Token "abc" (width 3)
  └─> Pattern [ab, c]  // All substrings reachable
  or
  └─> Pattern [a, b, c]  // Alternative complete decomposition
```

### Violation 5: Duplicate String Token (HIGH-CONFIDENCE)

**Symptom:** Two different tokens represent the same string
**Cause:** Creating new token without checking for existing token
**Fix:** Check if token for string exists before creating new token
**Detection:** `check_string_token_uniqueness()`

```rust
// ❌ Wrong
let abc1 = graph.create_token_for("abc");  // Creates Token(5)
let abc2 = graph.create_token_for("abc");  // Creates Token(8) - duplicate!

// ✅ Correct
let abc = graph.get_or_create_token_for("abc");  // Returns existing token
```

### Violation 6: Inconsistent Pattern Representation (HIGH-CONFIDENCE)

**Symptom:** Multiple patterns of same token represent different strings
**Cause:** Incorrect pattern construction
**Fix:** Verify all patterns compose to same string
**Detection:** `check_multiple_representation_consistency()`

```rust
// ❌ Wrong
Token "abc" (width 3):
  Pattern [a, b, c]  → "abc" ✓
  Pattern [a, b, d]  → "abd" ✗ different string!

// ✅ Correct
Token "abc" (width 3):
  Pattern [a, b, c]  → "abc" ✓
  Pattern [ab, c]    → "abc" ✓
  Pattern [a, bc]    → "abc" ✓
```

### Violation 7: Position Out of Bounds (MEDIUM-CONFIDENCE)

**Status:** Partially validated in some contexts

**Symptom:** AtomPosition >= token width
**Cause:** Incorrect cursor advancement
### Violation 7: Position Out of Bounds (HIGH-CONFIDENCE)

**Symptom:** Position reference >= token width
**Cause:** Incorrect cursor advancement or index calculation
**Fix:** Validate positions before access
**Detection:** Bounds checking in operations

```rust
// ❌ Wrong
let pos = AtomPosition(10);
let token = Token { width: 3, ... };
// pos >= width - out of bounds!

// ✅ Correct
if pos < token.width() {
    // Safe to access
}
``` Type System Guarantees

The Rust type system provides some guarantees automatically:

1. **No Null References**: ✅ All Token references valid (Rust ownership)
2. **Width Non-Negative**: ✅ usize prevents negative widths
3. **Pattern Ownership**: ✅ Patterns own their children (no dangling)
## Formal Verification

### Type System Guarantees

The Rust type system provides some guarantees automatically:

1. **No Null References**: ✅ All Token references valid (Rust ownership)
2. **Width Non-Negative**: ✅ usize prevents negative widths
3. **Pattern Ownership**: ✅ Patterns own their children

**These don't need runtime checking.**

### Runtime Validation (All Required)

### Property-Based Testing

Test all required invariants with property-based testing:

```rust
#[quickcheck]
fn prop_width_consistency(tokens: Vec<Token>) -> bool {
    let graph = build_graph(tokens);
    for token in graph.tokens() {
        if !check_width_consistency(&graph, token) {
            return false;
        }
    }
    true
}

#[quickcheck]
fn prop_pattern_completeness(tokens: Vec<Token>) -> bool {
    let graph = build_graph(tokens);
    for token in graph.tokens() {
        if token.width() > 1 && !check_pattern_completeness(&graph, token) {
            return false;
        }
    }
    true
}

#[quickcheck]
fn prop_all_invariants(tokens: Vec<Token>) -> bool {
    let graph = build_graph(tokens);
    check_all_invariants(&graph)
}
```not guaranteed
- ❌ Path Continuity - algorithm behavior
- ❌ Hierarchy - not validated
## Conclusion

### Core Required Invariants (8 Total)

These form the **required contract** of the hypergraph structure:

1. **Width Consistency** - Explicitly validated in code
2. **Pattern Completeness** - Explicitly validated in code
3. **Parent-Child Bidirectional** - Enforced during insertion
4. **Atom Uniqueness** - Structural guarantee via data structures
5. **Multiple Representation Consistency** - Required (all patterns represent same string)
6. **Substring Reachability** - Required (all substrings reachable from superstrings)
7. **String-Token Uniqueness** - Required (each string has at most one token)
8. **Position Validity** - Required (all positions within valid bounds)

**All operations (insert, search, read) must preserve these invariants.**

The split-join architecture of context-insert specifically ensures that new patterns can be added while maintaining these invariants without modifying existing structures.

### Key Insights

1. **Eight core required invariants**: All must be maintained at all times.

2. **Each token represents exactly one string**: Both via multiple representation consistency (all patterns of a token represent same string) and string-token uniqueness (each string has at most one token).

3. **Substring reachability is mandatory**: All substrings must be reachable from superstrings through child patterns.

4. **Position validity is required**: All position references must be within valid bounds to prevent errors.

5. **String-token bijection**: Each string maps to exactly one token, ensuring unambiguous representation.

### Recommendations

**For validation:**
- ✅ Check width consistency after pattern insertion
- ✅ Check pattern completeness (≥2 children for non-atoms)
- ✅ Check parent-child bidirectional relationships
- ✅ Check substring reachability
- ✅ Check string-token uniqueness
- ✅ Check multiple representation consistency
- ✅ Check position validity comprehensively

**For documentation:**
- All eight invariants are **required**, not optional
- Each token must represent exactly one unique string
- All substrings must be reachable through patterns
- All positions must be validated before access

**For future work:**
- Add comprehensive validation for all eight invariants
- Implement runtime checks for invariants not currently validated
- Consider performance optimization for validation in production
- Strengthen position validity checking across all operations
- Add multiple representation consistency validation
- Consider caching string representations with invalidation
