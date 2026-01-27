# Plans Index

Plans for major refactorings and features before execution.

## Active Plans

### 20260127_SELECTIVE_PARTITION_MERGE.md
- **Status:** 📋 Design Phase
- **Priority:** High (test failures)
- **Tags:** #context-insert #merge #partition #algorithm
- **Summary:** Transform complete merge to selective partition creation. Only required partitions (target, wrapper, inner, overlaps) get tokens.
- **Implementation:** [20260127_SELECTIVE_PARTITION_MERGE_IMPL.md](20260127_SELECTIVE_PARTITION_MERGE_IMPL.md)
- **Related Test:** `insert_infix1` - expects `aby` with 1 pattern, currently has 2

### 20251204_PLAN_INTERVAL_TESTS_INVESTIGATION.md
- **Status:** ⚠️ Investigation in progress
- **Priority:** High (test failures)
- **Tags:** #testing #context-insert #position-semantics #investigation
- **Summary:** Investigation of interval_graph test failures. Position 4 confirmed correct, but cache structure mismatch identified.
- **Progress:** Root cause partially identified - cache structure doesn't match expected format
- **Next Steps:** Fix cache construction to match expected structure

### 20251204_PLAN_FIX_INTERVAL_TESTS.md
- **Status:** 📋 Ready (depends on investigation)
- **Priority:** High
- **Tags:** #testing #context-insert #bug-fix
- **Summary:** Plan to fix interval_graph test failures once investigation complete
- **Blocking:** Needs 20251204_PLAN_INTERVAL_TESTS_INVESTIGATION

### 20251204_PLAN_FIX_INDEX_PREFIX_TEST.md
- **Status:** 📋 Ready for implementation
- **Priority:** Medium
- **Tags:** #testing #context-search #position-bug
- **Summary:** Fix index_prefix1 test failure (width mismatch from wrong end_bound)

### 20251204_PLAN_FIX_INDEX_POSTFIX_TEST.md
- **Status:** 📋 Ready for implementation
- **Priority:** Medium
- **Tags:** #testing #context-search #type-mismatch
- **Summary:** Fix index_postfix1 test (PathCoverage type expectations)

### 20251203_BEST_MATCH_IMPLEMENTATION_STRATEGY.md
- **Status:** 📋 Strategy documented
- **Priority:** Medium
- **Tags:** #search #algorithm #planning
- **Summary:** Implementation strategy for proper best match tracking, queue clearing, and trace cache commitment
- **Note:** Moved from implemented/ - this is a strategy, not completed work

### 20251204_PLAN_COLOR_FORMATTING_IN_LOGS.md
- **Status:** 📋 Enhancement
- **Priority:** Low
- **Tags:** #logging #formatting #enhancement
- **Summary:** Add color formatting support to log output for better readability

### 20251204_PLAN_THREAD_LOCAL_PATTERN.md
- **Status:** 📋 Design
- **Priority:** Low
- **Tags:** #architecture #thread-safety #design
- **Summary:** Design for thread-local pattern management

### 20251203_PLAN_CANDIDATE_STATE_CONTROL.md
- **Status:** 📋 Design proposal
- **Priority:** Medium
- **Tags:** #architecture #types #candidate-state
- **Summary:** Design for removing CheckpointedRef and adding CandidateState type parameter

### 20251203_PLAN_PHASE2_CHECKPOINTED_STATE_ADVANCE.md
- **Status:** 📋 Phased implementation plan
- **Priority:** Medium
- **Tags:** #architecture #checkpointed #advance
- **Summary:** Phase 2 plan for checkpointed state advancement

### 20251203_PLAN_STATUS_CHECKPOINT.md
- **Status:** 📋 Planning
- **Priority:** Medium
- **Tags:** #checkpoints #status #tracking
- **Summary:** Plan for status checkpoint tracking

### 20251121_PLAN_position_annotated_paths.md
- **Status:** 📋 Design proposal
- **Priority:** Low
- **Tags:** #paths #positions #annotations
- **Summary:** Design for position-annotated path structures

## Superseded Plans

### 20251127_PLAN_EFFICIENT_CHECKPOINTED_CURSOR.md
- **Status:** ⚠️ Superseded by other approaches
- **Note:** Original comprehensive plan, but newer focused plans address specific issues

### 20251123_PLAN_checkpoint_architecture_refactor.md
- **Status:** ⚠️ Superseded by PLAN_EFFICIENT_CHECKPOINTED_CURSOR
- **Note:** Identified issues, but newer plan provides better solution

### 20251130_PLAN_TEST_ECOSYSTEM_IMPROVEMENTS.md
- **Status:** ⚠️ Superseded / Partially implemented
- **Note:** Many improvements have been made; review for remaining items

## Templates

- `20251203_PLAN_TEMPLATE.md` - Template for new plans
