# Plans Index

Plans for major refactorings and features before execution.

## Active Plans

### PLAN_EFFICIENT_CHECKPOINTED_CURSOR.md
- **Status:** 📋 Ready for Review (Created 2025-11-27)
- **Priority:** High (blocks find_consecutive1 test + architecture improvement)
- **Effort:** ~9.5 hours (8 phases)
- **Tags:** #architecture #cursor #checkpoints #space-optimization #type-safety
- **Summary:** Space-efficient Checkpointed cursor with Option-based candidate storage (50% space savings), AdvanceCheckpointed trait for encapsulated advancement, and MatchResult integration to represent advanced query cursors.
- **Problem:** Current Checkpointed stores redundant data when checkpoint==current; MatchResult cannot represent advanced queries (causes find_consecutive1 failure with wrong end_index)
- **Solution:** `candidate: Option<C>` instead of `current: C`, reusable in MatchResult
- **Next Steps:** Review design questions, confirm approach, begin Phase 1 implementation

### PLAN_checkpoint_architecture_refactor.md
- **Status:** ⚠️ Superseded by PLAN_EFFICIENT_CHECKPOINTED_CURSOR.md
- **Priority:** ~~High~~ Resolved by new plan
- **Tags:** #architecture #refactor #checkpoints #cursor-state
- **Summary:** ~~Centralize checkpoint management to fix position semantic issues~~ → New plan addresses root cause
- **Blocking:** ~~find_consecutive1~~, prefix1, range1 tests (prefix/range fixed, consecutive addressed in new plan)
- **Note:** Original plan identified issues but new plan provides comprehensive solution

### PLAN_TEST_ECOSYSTEM_IMPROVEMENTS.md
- **Status:** 📋 Ready for Implementation (Created 2025-11-30, Updated after clarification)
- **Priority:** High (fix position bug + improve test maintainability)
- **Effort:** Sprint 0 (2 days bug fix) + 5 weeks (infrastructure + coverage)
- **Tags:** #testing #bug-fix #infrastructure #coverage #refactor #documentation #quality
- **Summary:** Fix position caching bug in hierarchical prefix matches, then comprehensive test improvements: reduce duplication, add macros/fixtures, expand coverage, reorganize structure, and document patterns.
- **Problem:** 
  - **BUG:** Top-down position tracking produces position `1` instead of `2` in hierarchical prefix with non-empty end paths
  - Test duplication (6+ scenarios)
  - No systematic coverage of hierarchical prefix/width/positions
  - Hard-to-read assertions, 40+ boilerplate repetitions
- **Solution:** 
  - **Phase 0 (Sprint 0):** Investigate and fix position bug, document calculation rules
  - **Phase 1-5 (5 sprints):** Create macros/fixtures/helpers, add 20+ tests, refactor existing, reorganize & document
- **Key Findings:**
  - `prefix1` test expectations are CORRECT (position 2)
  - Implementation has bug producing position 1 (off-by-one in top-down traversal)
  - Duplicate test removed (had wrong expectations)
  - Only 1 test covering this critical scenario (now failing correctly)
  - 53 tests total, weak edge case coverage, significant duplication
- **Next Steps:** 
  1. Investigate where position is calculated in top-down traversal code
  2. Find the off-by-one error or incorrect calculation
  3. Create bug report document
  4. Fix the bug
  5. Verify all tests pass
  6. Document position calculation rules
  7. Begin infrastructure improvements

## Templates

- `PLAN_TEMPLATE.md` - Template for new plans
