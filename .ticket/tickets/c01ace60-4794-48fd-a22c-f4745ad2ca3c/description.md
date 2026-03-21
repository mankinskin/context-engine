---
tags: `#plan` `#context-trace` `#context-search` `#context-insert` `#algorithm` `#debugging` `#testing` `#api` `#performance`
summary: Create an extensible, declarative testing harness that:
status: 📋
---

# Plan: End-to-End Test Registry & Declarative Test Cases

**Date:** 2025-12-06  
**Status:** Planning  
**Priority:** High - Infrastructure improvement for comprehensive testing

## Objective

Create an extensible, declarative testing harness that:
1. Centralizes reusable graph environments across all test crates
2. Shares expected output values for operations (search, insert, interval, etc.)
3. Provides type-safe, user-friendly test case definitions
4. Enables end-to-end validation across the entire call chain
5. Reduces duplication and improves test maintainability

## Current State Analysis

### Existing Test Patterns

**Test Environments (context-trace/tests/env/mod.rs):**
- ✅ Centralized graph initialization
- ✅ Returns all relevant tokens/patterns/IDs
- ❌ Only contains graph state, no expected outputs
- ❌ No standardized naming/organization convention

**Search Tests (context-search/tests/search/):**
- ✅ Expected `Response` values with trace caches
- ✅ Validates search algorithm correctness
- ❌ Duplicates graph creation from environments
- ❌ Expected values hard-coded in test functions

**Insert Tests (context-insert/tests/insert.rs):**
- ✅ Uses test environments via `initialize_expected()`
- ✅ Tests insertion logic with real graphs
- ❌ No shared expected values with search tests
- ❌ Manual verification of results

**Interval Tests (context-insert/tests/interval.rs):**
- ✅ Uses test environments (Env1, Env2)
- ✅ Has `TraceCase` trait for expected outputs
- ✅ Validates split/join operations
- ❌ Limited to interval-specific operations

### Current Test Flow

```
context-trace (environments)
    │
    ├──> context-search (tests with expected Response)
    │        │
    │        └──> Manual duplication of expectations
    │
    └──> context-insert (tests using environments)
             │
             └──> No shared expectations
```

## Proposed Architecture

### 1. Declarative Test Case Structure

```rust
/// Core trait for test case definitions
pub trait TestCase {
    /// Name/identifier for the test case
    fn name(&self) -> &'static str;
    
    /// Tags for categorization and filtering
    fn tags(&self) -> &[TestTag];
    
    /// Description of what this test case validates
    fn description(&self) -> &'static str;
}

/// Tags for organizing and filtering test cases
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestTag {
    /// Test involves pattern matching
    Pattern,
    /// Test involves prefix matching
    Prefix,
    /// Test involves postfix matching
    Postfix,
    /// Test involves infix/range matching
    Infix,
    /// Test involves complete token matching
    Complete,
    /// Test involves interval operations
    Interval,
    /// Test involves split operations
    Split,
    /// Test involves join operations
    Join,
    /// Tests search algorithm
    Search,
    /// Tests insertion
    Insert,
    /// Known bug/failing test
    Bug,
    /// Performance-critical test
    Performance,
}

/// Graph environment with initialization and metadata
pub trait GraphEnvironment: Sized {
    /// Unique identifier for this environment
    fn id() -> &'static str;
    
    /// Initialize a fresh instance of the environment
    fn initialize() -> Self;
    
    /// Get graph reference
    fn graph(&self) -> &HypergraphRef;
    
    /// Tags describing the graph structure
    fn tags() -> &'static [TestTag];
    
    /// Human-readable description
    fn description() -> &'static str;
}
```

### 2. Operation-Specific Test Case Traits

```rust
/// Test case for search operations
pub trait SearchTestCase: TestCase {
    /// The environment type this test uses
    type Environment: GraphEnvironment;
    
    /// Get the environment
    fn environment(&self) -> Self::Environment;
    
    /// Input query for search
    fn query(&self) -> Vec<Token>;
    
    /// Expected search response
    fn expected_response(&self) -> Response;
    
    /// Execute and validate the search
    fn validate(&self) -> Result<(), TestError> {
        let env = self.environment();
        let query = self.query();
        let expected = self.expected_response();
        
        let actual = Searchable::<AncestorSearchTraversal<_>>::search(
            query.clone(),
            env.graph().clone().into(),
        ).map_err(|e| TestError::SearchFailed(e))?;
        
        if actual != expected {
            return Err(TestError::ResponseMismatch {
                expected,
                actual,
                test_case: self.name(),
            });
        }
        
        Ok(())
    }
}

/// Test case for insert operations
pub trait InsertTestCase: TestCase {
    /// The environment type this test uses
    type Environment: GraphEnvironment;
    
    /// Get the environment (pre-initialized state)
    fn environment(&self) -> Self::Environment;
    
    /// Input tokens to insert
    fn input_tokens(&self) -> Vec<Token>;
    
    /// Expected resulting token
    fn expected_token(&self) -> Token;
    
    /// Expected token string representation
    fn expected_string(&self) -> &str;
    
    /// Expected pattern structure (for assertions)
    fn expected_patterns(&self) -> Vec<ExpectedPattern>;
    
    /// Execute and validate the insert
    fn validate(&self) -> Result<(), TestError>;
}

/// Test case for interval operations (split/join)
pub trait IntervalTestCase: TestCase {
    /// The environment type this test uses
    type Environment: GraphEnvironment;
    
    /// Get the environment
    fn environment(&self) -> Self::Environment;
    
    /// Input interval specification
    fn init_interval(&self) -> InitInterval;
    
    /// Expected trace cache
    fn expected_trace_cache(&self) -> TraceCache;
    
    /// Expected split cache
    fn expected_split_cache(&self) -> SplitCache;
    
    /// Expected join cache  
    fn expected_join_cache(&self) -> JoinCache;
    
    /// Execute and validate the interval operation
    fn validate(&self) -> Result<(), TestError>;
}

/// Combined test case for end-to-end validation
pub trait EndToEndTestCase: TestCase {
    type Environment: GraphEnvironment;
    
    /// Sequential operations to perform
    fn operations(&self) -> Vec<Operation>;
    
    /// Validate the entire operation chain
    fn validate(&self) -> Result<(), TestError>;
}

/// Operations that can be performed in a test
#[derive(Debug, Clone)]
pub enum Operation {
    /// Search for a pattern
    Search {
        query: Vec<Token>,
        expected: Response,
    },
    /// Insert tokens
    Insert {
        tokens: Vec<Token>,
        expected_token: Token,
        expected_patterns: Vec<ExpectedPattern>,
    },
    /// Create interval graph
    Interval {
        init: InitInterval,
        expected_trace: TraceCache,
        expected_split: SplitCache,
        expected_join: JoinCache,
    },
}
```

### 3. Test Case Registry Structure

```rust
/// Central registry of all test cases
pub struct TestRegistry {
    /// All graph environments
    environments: HashMap<&'static str, Box<dyn GraphEnvironment>>,
    
    /// Search test cases
    search_tests: HashMap<&'static str, Box<dyn SearchTestCase>>,
    
    /// Insert test cases
    insert_tests: HashMap<&'static str, Box<dyn InsertTestCase>>,
    
    /// Interval test cases
    interval_tests: HashMap<&'static str, Box<dyn IntervalTestCase>>,
    
    /// End-to-end test cases
    e2e_tests: HashMap<&'static str, Box<dyn EndToEndTestCase>>,
}

impl TestRegistry {
    /// Get the global registry
    pub fn global() -> &'static TestRegistry;
    
    /// Register a test case
    pub fn register_search<T: SearchTestCase + 'static>(&mut self, test: T);
    pub fn register_insert<T: InsertTestCase + 'static>(&mut self, test: T);
    pub fn register_interval<T: IntervalTestCase + 'static>(&mut self, test: T);
    pub fn register_e2e<T: EndToEndTestCase + 'static>(&mut self, test: T);
    
    /// Query tests by tags
    pub fn find_by_tags(&self, tags: &[TestTag]) -> Vec<&dyn TestCase>;
    
    /// Run all tests (or filtered subset)
    pub fn run_all(&self, filter: Option<&[TestTag]>) -> TestResults;
}
```

### 4. Concrete Test Case Example

```rust
/// Example: index_prefix1 test case with shared expectations
pub struct IndexPrefix1Hell;

impl TestCase for IndexPrefix1Hell {
    fn name(&self) -> &'static str { "index_prefix1_hell" }
    
    fn tags(&self) -> &[TestTag] {
        &[TestTag::Prefix, TestTag::Search, TestTag::Insert, TestTag::Pattern]
    }
    
    fn description(&self) -> &'static str {
        "Tests prefix matching of 'hell' against 'heldld' pattern, \
         expecting partial match up to position 3 with PrefixEnd coverage"
    }
}

impl SearchTestCase for IndexPrefix1Hell {
    type Environment = EnvIndexPrefix1;
    
    fn environment(&self) -> Self::Environment {
        EnvIndexPrefix1::initialize()
    }
    
    fn query(&self) -> Vec<Token> {
        let env = self.environment();
        vec![env.h, env.e, env.l, env.l]
    }
    
    fn expected_response(&self) -> Response {
        let env = self.environment();
        Response {
            end: MatchResult {
                cursor: CheckpointedCursor::AtCheckpoint(Checkpointed::new(
                    PatternCursor {
                        path: RootedRangePath::new(
                            self.query(),
                            RolePath::new_empty(0),
                            RolePath::new_empty(2),
                        ),
                        atom_position: 3.into(),
                        _state: Default::default()
                    }
                )),
                path: PathCoverage::Prefix(PrefixEnd {
                    path: RootedRolePath::new(
                        PatternLocation::new(env.heldld, env.heldld_id),
                        RolePath::new(
                            2,
                            vec![ChildLocation::new(env.ld, env.ld_id, 0)]
                        )
                    ),
                    target: DownKey {
                        index: env.l,
                        pos: 2.into(),
                    },
                    exit_pos: 2.into(),
                    end_pos: 3.into(),
                }),
            },
            cache: build_trace_cache!(
                env.heldld => (
                    BU {},
                    TD { 2 => env.ld -> (env.heldld_id, 2) },
                ),
                env.ld => (
                    BU {},
                    TD { 2 => env.l -> (env.ld_id, 0) },
                ),
                env.h => (BU {}, TD {}),
                env.l => (
                    BU {},
                    TD { 2 => {} },
                ),
            ),
        }
    }
}

impl InsertTestCase for IndexPrefix1Hell {
    type Environment = EnvIndexPrefix1;
    
    fn environment(&self) -> Self::Environment {
        EnvIndexPrefix1::initialize()
    }
    
    fn input_tokens(&self) -> Vec<Token> {
        let env = self.environment();
        vec![env.h, env.e, env.l, env.d]
    }
    
    fn expected_token(&self) -> Token {
        self.environment().heldld
    }
    
    fn expected_string(&self) -> &str {
        "heldld"
    }
    
    fn expected_patterns(&self) -> Vec<ExpectedPattern> {
        let env = self.environment();
        vec![
            ExpectedPattern {
                token: env.heldld,
                structure: vec![env.h, env.e, env.ld, env.ld],
                pattern_ids: vec![env.heldld_id],
            }
        ]
    }
    
    fn validate(&self) -> Result<(), TestError> {
        // Implementation matches current insert test logic
        todo!()
    }
}
```

### 5. File Organization

```
crates/
  context-trace/
    src/
      tests/
        env/                          # Graph environments
          mod.rs                      # TestEnv trait + all environments
          registry.rs                 # NEW: Environment registry
        
        test_case/                    # NEW: Test case framework
          mod.rs                      # Core traits (TestCase, GraphEnvironment)
          search.rs                   # SearchTestCase trait
          insert.rs                   # InsertTestCase trait
          interval.rs                 # IntervalTestCase trait
          e2e.rs                      # EndToEndTestCase trait
          registry.rs                 # TestRegistry implementation
          
        cases/                        # NEW: Concrete test cases
          mod.rs                      # Register all test cases
          index_prefix1.rs            # IndexPrefix1Hell + variants
          index_postfix1.rs           # IndexPostfix1Bcdd + variants
          index_pattern1.rs           # IndexPattern1 variants
          index_pattern2.rs           # IndexPattern2 variants
          index_infix1.rs             # IndexInfix1 variants
          index_infix2.rs             # IndexInfix2 variants
          interval_env1.rs            # Interval tests for Env1
          interval_env2.rs            # Interval tests for Env2

  context-search/
    src/
      tests/
        search/
          mod.rs                      # Run registered search tests
          insert_scenarios.rs         # MIGRATE TO: test_case framework
          
  context-insert/
    src/
      tests/
        insert.rs                     # MIGRATE TO: test_case framework
        interval.rs                   # MIGRATE TO: test_case framework
```

## Migration Strategy

### Phase 1: Foundation (Week 1)
1. ✅ Create core traits in `context-trace/src/tests/test_case/`
   - `TestCase`, `GraphEnvironment`
   - `SearchTestCase`, `InsertTestCase`, `IntervalTestCase`
   - `TestRegistry` structure

2. ✅ Enhance existing environments
   - Add `GraphEnvironment` trait implementation
   - Add `id()`, `tags()`, `description()` metadata

3. ✅ Create registry infrastructure
   - Global registry singleton
   - Registration macros for convenience

### Phase 2: Search Test Migration (Week 1-2)
1. ✅ Create concrete test cases for search tests
   - `IndexPrefix1Hell`
   - `IndexPostfix1Bcdd`
   - All other insert_scenarios tests

2. ✅ Update search tests to use registry
   - Keep existing test functions as thin wrappers
   - Call into test case validation

3. ✅ Validate no behavioral changes

### Phase 3: Insert Test Migration (Week 2)
1. ✅ Create concrete test cases for insert tests
   - Share `expected_response()` from search test cases
   - Add insert-specific expectations

2. ✅ Update insert tests to use test cases
   - Use same environment instances
   - Share expected values

3. ✅ Validate end-to-end consistency

### Phase 4: Interval Test Migration (Week 2-3)
1. ✅ Create concrete test cases for interval tests
   - Leverage existing `TraceCase` patterns
   - Integrate with test case framework

2. ✅ Update interval tests
   - Use registry
   - Share environments

### Phase 5: End-to-End Tests (Week 3)
1. ✅ Create combined test cases
   - Search → Insert chains
   - Insert → Interval chains
   - Full pipeline tests

2. ✅ Add cross-crate validation
   - Search output → Insert input
   - Trace cache consistency

### Phase 6: Enhancement & Documentation (Week 4)
1. ✅ Add test filtering/selection
2. ✅ Add performance benchmarks
3. ✅ Document test case authoring guide
4. ✅ Create examples for new test cases

## Benefits

### For Developers
- **Single source of truth** for expected values
- **Reusable environments** across all test crates
- **Type-safe** test case definitions
- **Easy to add** new test cases
- **Discoverability** via registry and tags

### For Testing
- **End-to-end validation** across operation chains
- **Consistency checking** between crates
- **Reduced duplication** of graph setup
- **Better error messages** with test case context
- **Easier debugging** with centralized expectations

### For Maintenance
- **Centralized changes** when APIs evolve
- **Clear migration path** for new test patterns
- **Self-documenting** via descriptions and tags
- **Automated validation** of test case completeness

## Example Usage

```rust
// In any test crate
#[test]
fn test_index_prefix1_search() {
    let test_case = IndexPrefix1Hell;
    test_case.validate().expect("Search validation failed");
}

#[test]
fn test_index_prefix1_insert() {
    let test_case = IndexPrefix1Hell;
    test_case.validate_insert().expect("Insert validation failed");
}

#[test]
fn test_index_prefix1_e2e() {
    let test_case = IndexPrefix1Hell;
    test_case.validate_e2e().expect("E2E validation failed");
}

// Run all prefix tests
#[test]
fn test_all_prefix_scenarios() {
    let registry = TestRegistry::global();
    let results = registry.run_all(Some(&[TestTag::Prefix]));
    assert!(results.all_passed(), "Some prefix tests failed: {:?}", results.failures());
}
```

## Open Questions

1. **Static vs Dynamic Dispatch**: Use `dyn Trait` for registry flexibility or generic parameters for performance?
   - **Recommendation**: Start with dynamic dispatch for flexibility, optimize later if needed

2. **Environment Caching**: Should environments be cached or created fresh for each test?
   - **Recommendation**: Fresh instances for test isolation, but cache initialization pattern

3. **Expected Value Construction**: Build at runtime or use declarative macros?
   - **Recommendation**: Hybrid - runtime for flexibility, macros for common patterns

4. **Cross-Crate Dependencies**: How to handle test dependencies between crates?
   - **Recommendation**: All test cases in `context-trace`, other crates import

5. **Failure Reporting**: What level of detail in test failure messages?
   - **Recommendation**: Full diff output + test case context + debugging hints

## Success Metrics

- ✅ Zero duplication of graph initialization
- ✅ Shared expected values between search/insert tests
- ✅ End-to-end test coverage for major operations
- ✅ 50% reduction in test code volume
- ✅ 100% test case coverage in registry
- ✅ Clear documentation for adding new test cases

## Next Steps

1. **Review this plan** with team
2. **Create prototype** of core traits
3. **Implement Phase 1** foundation
4. **Migrate one test case** as proof of concept
5. **Iterate on design** based on feedback
6. **Full migration** following phases

---

**Notes:**
- This is an infrastructure change - existing tests should continue to work during migration
- Prioritize backward compatibility - old test patterns can coexist with new
- Focus on developer experience - make it easy to write and maintain tests
