# Plan: Investigation of interval_graph Test Failures

**Date:** 2025-12-04  
**Status:** Investigation  
**Priority:** High

## Objective

Investigate why `interval_graph1` and `interval_graph2` tests are failing with `query_exhausted()` assertion errors, understand the root cause, and determine the correct fix.

## Context

### Failing Tests

Both tests in `crates/context-insert/src/tests/interval.rs`:
- `interval_graph1` (line 205-235)
- `interval_graph2` (line 237-348)

### Current Failures

```
assertion failed: !res.query_exhausted()
```

Both tests expect `query_exhausted()` to return **false**, but it returns **true**.

### What We Know

1. **Query Semantics**: The search algorithm finds query patterns at multiple levels in the hypergraph hierarchy and can represent partial matches.

2. **Query Exhaustion**: A query can be:
   - Exhausted (fully matched/consumed)
   - Not able to advance at some point (incomplete match)

3. **PathCoverage Types**:
   - `EntireRoot` - matches entire token
   - `Prefix` - matches prefix of token (more content after)
   - `Postfix` - matches postfix of token (content before)
   - `Range` - matches middle range (content before and after)

4. **Test Expectations** (from comment #3609800548):
   > "we expect the query is completely consumed (exhausted) and we find a partial match in the hierarchy, in cdefghi"

   This suggests:
   - Query SHOULD be exhausted (fully matched)
   - But the match is a Range/Prefix within a larger token
   - The test expectations (`assert!(!res.query_exhausted())`) are WRONG

5. **Previous "Fix" Was Wrong**: 
   - Changing assertion to `assert!(res.query_exhausted())` makes tests pass
   - But removed detailed cache assertions
   - User feedback: "you don't just disable the test to make it pass"

## Investigation Steps

### Phase 1: Understanding Current Behavior

- [x] Confirmed tests fail with `assertion failed: !res.query_exhausted()`
- [x] Read test logs to understand actual behavior
- [ ] Analyze `query_exhausted()` implementation
- [ ] Understand when it should return true vs false
- [ ] Review PathCoverage enum and its relation to query exhaustion

### Phase 2: Analyze Test Cases

**interval_graph1:**
- Query: `[a, bc, d, e]` (4 tokens, width 5)
- Expected root: `abcdef` (width 6)
- Actual: query_exhausted=true, path=Prefix

**interval_graph2:**
- Query: `[d, e, f, g, h]` (5 tokens, width 5)
- Expected root: `cdefghi` (width 7)
- Actual: query_exhausted=true, path=Range

Questions:
- [ ] Why does the test expect query NOT exhausted?
- [ ] What semantic does "not exhausted" represent?
- [ ] Is there a relationship between PathCoverage type and query_exhausted?
- [ ] Should Prefix/Range matches have query_exhausted=false?

### Phase 3: Investigate History

- [ ] Check git history for changes to query_exhausted logic
- [ ] Check if these tests ever passed
- [ ] Look for related failing tests with similar patterns
- [ ] Check comments/docs about query_exhausted semantics

### Phase 4: Determine Root Cause

Possible scenarios:

**A) Test expectations are wrong:**
- Query CAN be exhausted even for Prefix/Range matches
- Tests should be updated to `assert!(res.query_exhausted())`
- Need to understand cache structure differences

**B) Implementation is wrong:**
- `query_exhausted()` should consider PathCoverage type
- For Prefix/Range matches, should return false (more content exists)
- Logic needs fixing in `MatchResult::query_exhausted()`

**C) Search behavior changed:**
- Search used to stop at exact matches (cdefg for query d,e,f,g,h)
- Now continues to find larger ancestors (cdefghi)
- Need to understand if this is intentional

### Phase 5: Examine Cache Mismatches

Once query_exhausted is understood, investigate why cache structures differ:

**interval_graph2 cache mismatch:**
- Expected: position 4 entries, cdefghi entries
- Actual: position 1 entries, cdefg AND cdefghi entries
- This suggests search traces through multiple levels
- Need to understand if this is correct behavior

## Questions to Answer

1. **Semantic Question**: What does "query not exhausted" mean in the context of interval graphs?
   - Does it mean the query pattern has more tokens to match?
   - Or does it mean the matched location has more content?

2. **Implementation Question**: How should `query_exhausted()` be implemented?
   ```rust
   pub fn query_exhausted(&self) -> bool {
       let checkpoint = self.cursor.checkpoint();
       let at_end = checkpoint.path.is_at_pattern_end();
       let path_empty = HasPath::path(checkpoint.path.end_path()).is_empty();
       at_end && path_empty  // Is this correct?
   }
   ```
   Should it also check `self.path` (PathCoverage type)?

3. **Test Question**: Why were these tests written with `!query_exhausted()`?
   - Was there a specific semantic reason?
   - Or were they written against incorrect behavior?

## Next Actions

1. Read `query_exhausted()` implementation and all related code
2. Check git blame for when these assertions were added
3. Look for documentation about query exhaustion semantics
4. Trace through search logic for one test case manually
5. Determine correct behavior and fix appropriately

## Related Files

- `crates/context-insert/src/tests/interval.rs` - Test file
- `crates/context-search/src/state/matched/mod.rs` - MatchResult, query_exhausted()
- `crates/context-search/src/state/end/mod.rs` - PathCoverage enum
- `target/test-logs/interval_graph*.log` - Test execution logs

## Notes

- User emphasized: "We need to understand whether the expectation is correct and why it is expected"
- Do NOT just make tests pass without understanding the why
- Cache structure assertions are important - don't remove them without understanding
