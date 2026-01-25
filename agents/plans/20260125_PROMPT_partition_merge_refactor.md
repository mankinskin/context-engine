# Agent Prompt: Partition Merge Refactoring

## Task

Refactor the partition merging code in context-insert to use generic position types and a unified MergePartitionCtx struct.

## Context

Read `agents/plans/20260125_PLAN_partition_merge_refactor.md` for the full design and implementation plan.

## Summary of Changes

1. **PatternSplits trait** (`split/pattern.rs`): Replace `atom_pos()` returning `Option<NonZeroUsize>` and `atom_pos_pair()` returning `Option<(NonZeroUsize, NonZeroUsize)>` with a generic associated type `type AtomPos` and a method `fn atom_pos(&self) -> Self::AtomPos`.

2. **VisitBorders trait** (`interval/partition/info/border/visit.rs`): Remove `info_border_with_pos` method. The `info_border` method should work with the generic `AtomPos` type from PatternSplits.

3. **MergePartitionCtx** (`join/context/node/merge/partition.rs`): Create a `MergePartitionCtx<R>` struct that holds partition context and implements merge utilities, replacing the current trait-based `MergePartition` approach.

## Key Tests

After changes, run:
```bash
cargo test -p context-insert insert_prefix1 -- --nocapture
cargo test -p context-insert insert_postfix1 -- --nocapture
cargo test -p context-insert
```

## Important Notes

- Read `AGENTS.md` first for project rules
- Check `agents/CHEAT_SHEET.md` for common patterns
- The plan file has the complete design with current vs proposed code
- `insert_prefix1` currently passes; `insert_postfix1` has issues that this refactoring may help resolve
