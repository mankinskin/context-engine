# Plans Index

Plans for major refactorings and features before execution.

## Active Plans

### PLAN_checkpoint_architecture_refactor.md
- **Status:** Research/Planning
- **Priority:** High (blocks 4 failing tests)
- **Tags:** #architecture #refactor #checkpoints #cursor-state
- **Summary:** Centralize checkpoint management to fix position semantic issues. Currently `mark_match()` doesn't update `atom_position`, causing off-by-one errors in QueryExhausted cases.
- **Blocking:** find_consecutive1, find_pattern1, prefix1, range1 tests
- **Next Steps:** Create detailed execution plan, fix `mark_match()` signature, update all call sites

## Templates

- `PLAN_TEMPLATE.md` - Template for new plans
