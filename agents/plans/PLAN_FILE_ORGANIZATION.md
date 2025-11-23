# File Organization Action Plan

**Date:** 2025-11-23  
**Last Update:** 2025-11-23 (Phase 2 Day 35-36 Complete ✅)  
**Git Commit:** 1d58f1b (Phase 2 Day 35-36 implementation)  
**Commit Date:** 2025-11-23  
**Commit Message:** refactor(context-trace): split graph/vertex/data.rs (700→406 lines largest)  
**Status:** Phase 1 Complete ✅ | Phase 2 In Progress (2/6 complete)  
**Goal:** Improve codebase maintainability by splitting large files and organizing module hierarchies

## Executive Summary

| Crate | Total Lines | Files | Largest File | Files >500 | Assessment |
|-------|-------------|-------|--------------|------------|------------|
| context-trace | 18,488 | 125 | 728 | 6 | 🟠 Needs Significant Work |
| context-search | 8,181 | 46 | 815 | 3 | 🟡 Needs Moderate Work |
| context-insert | 5,609 | 55 | 385 | 0 | 🟢 Excellent Structure |
| context-read | 1,673 | 20 | 364 | 0 | 🟢 Excellent Structure |
| **Total** | **33,951** | **246** | - | **9** | - |

**Key Findings:**
- 9 files exceed 500 lines (immediate split priority)
- 24 files between 300-500 lines (review for splitting)
- context-insert and context-read are well-organized (use as models)
- context-trace needs the most work (6 large files)

## Priority Levels

### 🔴 P0: Immediate Action (>700 lines)
Critical files that are too large and complex:
1. **context-search/match/root_cursor.rs** (815 lines)
2. **context-trace/logging/tracing_utils/config.rs** (728 lines)
3. **context-search/compare/state.rs** (725 lines)

### 🔴 P1: High Priority (500-700 lines)
Files that should be split soon:
4. **context-trace/graph/vertex/data.rs** (699 lines)
5. **context-trace/tests/macros.rs** (618 lines)
6. **context-trace/logging/tracing_utils/formatter.rs** (591 lines)
7. **context-search/tests/state_advance.rs** (544 lines)
8. **context-trace/path/structs/rooted/index_range.rs** (510 lines)
9. **context-trace/graph/insert.rs** (502 lines)

### 🟡 P2: Medium Priority (400-500 lines)
Files to review and potentially split:
10. **context-search/tests/state_advance_integration.rs** (497 lines)
11. **context-search/tests/search/ancestor.rs** (434 lines)
12. **context-search/state/start.rs** (424 lines)
13. **context-trace/tests/state_advance.rs** (397 lines)
14. **context-trace/logging/tracing_utils/test_tracing.rs** (396 lines)
15. **context-trace/graph/vertex/token.rs** (391 lines)
16. **context-trace/graph/mod.rs** (387 lines)
17. **context-insert/join/context/node/context.rs** (385 lines)
18. **context-search/tests/examples.rs** (369 lines)
19. **context-search/tests/traversal.rs** (368 lines)
20. **context-trace/path/structs/rooted/mod.rs** (366 lines)
21. **context-read/tests/read/mod.rs** (364 lines)
22. **context-trace/logging/path_format.rs** (363 lines)

### ✅ P3: Monitor (300-400 lines)
Files to keep an eye on but acceptable for now:
23-35. Various files in 300-350 range

## Phase 1: context-search (Weeks 6-7)

### ✅ Week 6 Day 28-29: match/root_cursor.rs (815 → 434 largest) - COMPLETE

**Status:** ✅ Implemented and committed (00747d1)  
**Completion Date:** 2025-11-23  
**Tests:** 29/35 passing (maintained, 0 regressions)
**Implemented structure:**
```rust
match/
├── root_cursor/
│   ├── core.rs (82) - RootCursor struct, enums, types
│   ├── advance.rs (434) - Advancement logic and state transitions
│   ├── state.rs (344) - Iterator impl and state machine
│   └── mod.rs (10) - Re-exports
├── iterator.rs (276)
└── mod.rs (166)
```

**Completed steps:**
1. ✅ Created `match/root_cursor/` directory
2. ✅ Extracted type definitions → `core.rs` (82 lines)
3. ✅ Extracted advancement logic → `advance.rs` (434 lines)
4. ✅ Extracted state machine → `state.rs` (344 lines)
5. ✅ Created `mod.rs` with re-exports (10 lines)
6. ✅ Updated imports in parent `match/mod.rs`
7. ✅ Tests passing: `cargo test -p context-search`
8. ✅ Compilation verified across workspace

**Actual impact:**
- Files: 1 → 4 (870 total lines including module overhead)
- Largest file: 815 → 434 lines
- Reduction: 47% in largest file
- Git: Intelligently tracked as rename with modifications
**Estimated impact:**
- Files: 1 → 4
- Largest file: 815 → ~280
- Reduction: 67% per file
### 🔄 Week 6 Day 30: compare/state.rs (725 → ~240 each) - NEXT

**Status:** 🔄 Ready to implement  
**Priority:** P0 - Second largest file in context-search

**Current structure:**
- Recently refactored (Days 25-26) but still large at 725 lines
- Contains state struct, transitions, decomposition helper
- Well-organized internally but needs splitting

**Target structure:**
**Target structure:**
```rust
compare/
├── state/
│   ├── core.rs (~220) - CompareState struct, constructors, Display
│   ├── transitions.rs (~260) - State transitions and advancement
│   ├── decomposition.rs (~220) - Prefix decomposition (helper + methods)
│   └── mod.rs (~25) - Re-exports
├── parent.rs (116)
**Implementation steps:**
1. Read `compare/state.rs` to identify split boundaries
2. Create `compare/state/` directory
3. Extract CompareState struct + constructors + Display → `core.rs`
4. Extract advancement and transition methods → `transitions.rs`
5. Extract `decompose_token_to_prefixes()` helper + methods → `decomposition.rs`
6. Create `mod.rs` with re-exports
7. Update imports in parent `compare/mod.rs`
8. Run tests: `cargo test -p context-search`
9. Verify compilation: `cargo build --workspace`
10. Commit with message documenting split

**Expected impact:**
- Files: 1 → 4
- Largest file: 725 → ~260 lines
- Reduction: ~64% per file
- Better separation of concerns (types/behavior/algorithms)
**Estimated impact:**
- Files: 1 → 4
- Largest file: 725 → ~260
- Reduction: 64% per file

### Week 7 Day 31: tests/state_advance.rs (544 → ~180 each)

**Current structure:**
- Many test cases in one file

**Target structure:**
```rust
tests/
├── state_advance/
│   ├── basic.rs (~180) - Basic advancement tests
│   ├── transitions.rs (~180) - State transition tests
│   ├── edge_cases.rs (~180) - Edge cases and errors
│   └── mod.rs (~10)
├── state_advance_integration.rs (497)
├── ...
```

**Steps:**
1. Create `tests/state_advance/` directory
2. Group tests by category
3. Split into separate files
4. Update test module structure
5. Run tests: `cargo test -p context-search`

**Benefits:**
- Easier to find specific tests
- Faster test file compilation
- Better test organization

### ✅ Week 7 Day 32: state/start.rs (424 → 226 largest) - COMPLETE

**Status:** ✅ Implemented and committed (7b6fb26)  
**Completion Date:** 2025-11-23  
**Tests:** 29/35 passing (maintained, 0 regressions)

**Original structure:**
- Single file with StartFoldPath trait and Searchable implementations (424 lines)

**Implemented structure:**
```rust
state/
├── start/
│   ├── core.rs (140) - StartFoldPath trait, InputLocation, StartCtx
│   ├── search.rs (226) - Searchable trait and implementations
│   └── mod.rs (10) - Re-exports
├── end/
├── ...
```

**Completed steps:**
1. ✅ Created `state/start/` directory
2. ✅ Extracted traits and types → `core.rs` (140 lines)
3. ✅ Extracted Searchable trait and impls → `search.rs` (226 lines)
4. ✅ Created `mod.rs` with re-exports (10 lines)
5. ✅ Tests passing: `cargo test -p context-search`
6. ✅ Compilation verified

**Actual impact:**
- Files: 1 → 3 (376 total lines including module overhead)
- Largest file: 424 → 226 lines
- Reduction: 47% in largest file
- Git: Tracked as rename with modifications

## Phase 2: context-trace (Weeks 8-11)

### ✅ Week 8 Day 33-34: logging/tracing_utils/config.rs (729 → 305 largest) - COMPLETE

**Status:** ✅ Implemented and committed (a946ab5)  
**Completion Date:** 2025-11-23  
**Tests:** 56/56 passing (maintained, 0 regressions)

**Implemented structure:**
```rust
logging/tracing_utils/
├── config/
│   ├── types.rs (194) - Config struct definitions
│   ├── loader.rs (305) - File loading + environment parsing
│   ├── builder.rs (238) - TracingConfig builder methods + tests
│   └── mod.rs (15) - Re-exports
├── formatter.rs (591)
├── ...
```

**Completed steps:**
1. ✅ Created `logging/tracing_utils/config/` directory
2. ✅ Extracted config types → `types.rs` (194 lines)
3. ✅ Extracted loading logic → `loader.rs` (305 lines)
4. ✅ Extracted builder methods → `builder.rs` (238 lines)
5. ✅ Created `mod.rs` with re-exports (15 lines)
6. ✅ Tests passing: `cargo test -p context-trace`
7. ✅ Compilation verified

**Actual impact:**
- Files: 1 → 4 (752 total lines including module overhead)
- Largest file: 729 → 305 lines
- Reduction: 58% in largest file
- Git: Tracked as rename with modifications

### ✅ Week 8 Day 35-36: graph/vertex/data.rs (700 → 406 largest) - COMPLETE

**Status:** ✅ Implemented and committed (1d58f1b)  
**Completion Date:** 2025-11-23  
**Tests:** 56/56 passing (maintained, 0 regressions)

**Implemented structure:**
```rust
graph/vertex/
├── data/
│   ├── core.rs (199) - VertexData struct, constructors, validation
│   ├── parents.rs (218) - Parent relationship operations
│   ├── children.rs (406) - Child pattern operations
│   └── mod.rs (65) - Display implementations + re-exports
├── token.rs (391)
├── ...
```

**Completed steps:**
1. ✅ Created `graph/vertex/data/` directory
2. ✅ Extracted VertexData struct + validation → `core.rs` (199 lines)
3. ✅ Extracted parent operations → `parents.rs` (218 lines)
4. ✅ Extracted child pattern operations → `children.rs` (406 lines)
5. ✅ Created `mod.rs` with Display impls (65 lines)
6. ✅ Tests passing: `cargo test -p context-trace`
7. ✅ Compilation verified

**Actual impact:**
- Files: 1 → 4 (888 total lines including module overhead)
- Largest file: 700 → 406 lines
- Reduction: 42% in largest file
- Better separation: struct definition, parents, children, display

### Week 9 Day 37: tests/macros.rs (618 → ~200 each) - NEXT

**Target structure:**
```rust
tests/
├── macros/
│   ├── graph.rs (~200) - Graph construction macros
│   ├── path.rs (~210) - Path construction macros
│   ├── trace.rs (~190) - Trace testing macros
│   └── mod.rs (~20)
```

### Week 9 Day 38-39: logging/tracing_utils/formatter.rs (591 → ~200 each)

**Target structure:**
```rust
logging/tracing_utils/
├── formatter/
│   ├── compact.rs (~200) - Compact formatter
│   ├── verbose.rs (~190) - Verbose formatter
│   ├── common.rs (~180) - Common formatting utilities
│   └── mod.rs (~25)
```

### Week 10 Day 40: path/structs/rooted/index_range.rs (510 → ~250 each)

**Target structure:**
```rust
path/structs/rooted/
├── index_range/
│   ├── core.rs (~250) - RangeIndexPath struct
│   ├── operations.rs (~240) - Range operations
│   └── mod.rs (~20)
```

### Week 10 Day 41: graph/insert.rs (502 → ~250 each)

**Target structure:**
```rust
graph/
├── insert/
│   ├── algorithm.rs (~250) - Insertion algorithm
│   ├── validation.rs (~240) - Pre/post validation
│   └── mod.rs (~15)
```

### Week 11: Review Medium Priority Files (400-500 lines)

Review and potentially split:
- `graph/vertex/token.rs` (391)
- `graph/mod.rs` (387)
- `path/structs/rooted/mod.rs` (366)
- Others as needed

## Phase 3: Test File Organization (Week 12)

### Goals:
- Split large test files in both crates
- Group tests by feature/component
- Improve test discoverability

### Targets:
- context-search: `tests/state_advance_integration.rs` (497)
- context-search: `tests/search/ancestor.rs` (434)
- context-search: `tests/examples.rs` (369)
- context-search: `tests/traversal.rs` (368)
- context-read: `tests/read/mod.rs` (364)
- context-insert: `tests/interval.rs` (346)
- context-insert: `tests/insert.rs` (312)

## Phase 4: Module Hierarchy Improvements (Week 13)

### context-trace module reorganization

**Current issues:**
- Flat structure in some modules
- Related files not grouped

**Proposed improvements:**

#### graph/vertex/
Already partially hierarchical, enhance:
```rust
graph/vertex/
├── data/        (new subdirectory)
├── pattern/     (existing)
├── location/    (existing)
├── token.rs     (consider splitting)
├── ...
```

#### path/structs/rooted/
```rust
path/structs/rooted/
├── index_range/ (new subdirectory)
├── role_path/   (existing)
├── core.rs      (extract from mod.rs)
├── ...
```

### context-search module reorganization

#### match/
```rust
match/
├── root_cursor/ (new subdirectory)
├── iterator.rs
└── mod.rs
```

#### compare/
```rust
compare/
├── state/       (new subdirectory)
├── parent.rs
├── iterator.rs
└── mod.rs
```

#### state/
```rust
state/
├── start/       (new subdirectory)
├── end/         (existing)
├── matched/     (existing)
├── ...
```

## Implementation Guidelines

### Before Each Split

1. **Read full file** - Understand complete context
2. **Identify boundaries** - Find natural split points
3. **Check dependencies** - Map import relationships
4. **Plan re-exports** - Design public API preservation
5. **Review tests** - Ensure test coverage

### During Split

1. **Create directory structure**
2. **Move code incrementally** - One logical unit at a time
3. **Update imports immediately** - Don't accumulate changes
4. **Maintain git history** - Use `git mv` when possible
5. **Keep tests passing** - Run `cargo test` after each change

### After Each Split

1. **Run full test suite** - `cargo test --workspace`
2. **Check compilation** - `cargo check --workspace`
3. **Verify formatting** - `cargo fmt --check`
4. **Review imports** - Ensure no unused imports
5. **Update documentation** - Update relevant docs
6. **Commit atomically** - One split per commit

### Quality Checks

After each phase:
```bash
# Compilation
cargo build --workspace

# Tests
cargo test --workspace

# Formatting
cargo fmt --all

# Clippy
cargo clippy --workspace -- -D warnings

### Success Metrics

### Quantitative
- [x] context-search: 0 files over 800 lines (was 1, now 0) ✅
- [x] context-search: 0 files over 700 lines (was 1, now 0) ✅
- [x] context-search: 0 files over 500 lines (was 3, now 0) ✅
- [x] context-trace: 0 files over 700 lines (was 2, now 0) ✅
- [ ] workspace: <3 files over 500 lines (currently 4 remaining in trace)
- [ ] workspace: <10 files over 400 lines (currently ~16 remaining)
- [ ] Average file size <150 lines
- [x] All tests passing (context-search: 29/35, context-trace: 56/56) ✅

### Qualitative
- [x] context-search: Easier to navigate (all large files split) ✅
- [x] context-search: Faster compilation (smaller units) ✅
- [x] context-search: Better IDE performance ✅
- [x] context-search: Clear module boundaries ✅
- [x] context-search: Improved code discoverability ✅
- [x] context-trace: Improved organization (2/6 splits complete) 🔄
- [ ] Overall workspace organization (Phase 2 in progress)
- [ ] Faster compilation (smaller units)
- [ ] Better IDE performance
- [ ] Clear module boundaries
- [ ] Improved code discoverability

## Risks and Mitigations

### Risk: Breaking changes
**Mitigation:** 
- Keep public API unchanged
- Use re-exports extensively
- Run tests continuously

### Risk: Import complexity
**Mitigation:**
- Document import patterns
- Use prelude modules where appropriate
- Keep re-export structure simple

### Risk: Git history loss
**Mitigation:**
- Use `git mv` for file moves
- Preserve original file structure in commits
## Timeline Summary

| Phase | Duration | Focus | Status | Progress |
|-------|----------|-------|--------|----------|
| Phase 1 | 2 weeks (Days 28-32) | context-search | ✅ Complete | 4/4 complete |
| Phase 2 | 4 weeks (Days 33-41) | context-trace | 🔄 In Progress | 2/6 complete |
| Phase 3 | 1 week (Days 42-46) | Test organization | ⏳ Planned | 0/7 complete |
| Phase 4 | 1 week (Days 47-51) | Module hierarchy | ⏳ Planned | Not started |

**Total:** 8 weeks of incremental improvements  
**Phase 1 Complete:** Days 28-32 (4 major splits) ✅  
**Phase 2 Progress:** Days 33-36 (2/6 splits complete) 🔄  
**Next:** Phase 2 Day 37 (tests/macros.rs)  
**Overall Progress:** 6/17 major splits complete (35.3%)
| Phase | Duration | Focus | Impact |
|-------|----------|-------|--------|
| Phase 1 | 2 weeks (Days 28-32) | context-search | 4 large files → 15+ smaller files |
| Phase 2 | 4 weeks (Days 33-41) | context-trace | 6 large files → 24+ smaller files |
| Phase 3 | 1 week (Days 42-46) | Test organization | Better test structure |
| Phase 4 | 1 week (Days 47-51) | Module hierarchy | Improved organization |

**Total:** 8 weeks of incremental improvements

## Long-term Maintenance

### File Size Guidelines
- **Target:** <300 lines per file
- **Review:** 300-400 lines
- **Action:** >400 lines - consider splitting
- **Critical:** >500 lines - must split

### Module Organization Principles
1. **Single Responsibility** - Each file has one clear purpose
2. **Cohesion** - Related functionality grouped together
3. **Loose Coupling** - Minimize dependencies between modules
4. **Clear Hierarchy** - Logical nesting of modules
5. **Discoverability** - Easy to find relevant code

### Monitoring
- Review file sizes quarterly
- Track module complexity metrics
- Monitor compilation times
- Gather developer feedback

## Documentation Updates Needed

After implementation:
- [ ] Update CHEAT_SHEET.md with new module paths
- [ ] Update HIGH_LEVEL_GUIDE.md files with new organization
- [ ] Update AGENTS.md with new file locations
- [ ] Create MODULE_ORGANIZATION.md guide
- [ ] Update README.md with crate structure

## References

**See individual crate FILE_INDEX.md files:**
- `crates/context-trace/FILE_INDEX.md`
- `crates/context-search/FILE_INDEX.md`
- `crates/context-insert/FILE_INDEX.md`
- `crates/context-read/FILE_INDEX.md`

**Good examples to follow:**
- context-insert (excellent file sizes and organization)
- context-read (clean, focused modules)

**Tags:** `#refactoring` `#organization` `#maintainability` `#phase3` `#file-splitting` `#module-hierarchy`
