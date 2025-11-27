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

## Templates

- `PLAN_TEMPLATE.md` - Template for new plans
