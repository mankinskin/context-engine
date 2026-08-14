## Gap

`.agents/agents/implement.agent.md` has its Required Workflow at line 53. The workflow has no pre-migration inventory rule before bulk file moves.

## Session Evidence

A restructuring implementation began with one giant chained `git mv`. A single non-existent path, `src/bin/ticket.rs`, aborted the chain after partial execution and left a broken half-migrated worktree. A more expensive agent had to repair the migration.

## Required Corrected State

Add a preparatory step before Required Workflow step 1: before any bulk relocation, enumerate every source and test path, verify each path exists, then execute moves in phases with validation between phases.