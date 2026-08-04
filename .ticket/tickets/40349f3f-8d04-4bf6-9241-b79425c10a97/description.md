## Problem

`sessions_for_ticket` (spec `e5f8a2c1`) answers "which sessions worked on this ticket" via three cumulative tiers strict ⊆ linked ⊆ mentioned, using ONLY structured signals (never transcript text). The strict tier reads `SessionMetadata.ticket_id`, which is written by `check_in_worktree`.

A dry-run backfill against the real store of 231 sessions measured:
- `branch` populated: 0/231
- `worktree_path` populated: 0/231
- linkage recoverable only via handoff `target_tickets`: 37 associations across 4 sessions (~1.7%)

Root cause: the Copilot capture hook (`memory-api/crates/session-api/src/bin/copilot-capture-hook.rs`) creates sessions passively on every turn and never calls `check_in_worktree`. Branch and worktree path are therefore never recorded for the vast majority of sessions, and `ticket_id` has nowhere to come from.

## Fix (this ticket)

Added `SessionStoreConfig::infer_worktree_from_environment` (`memory-api/crates/session-api/src/store/config/worktree_capture_inference.rs`), wired into the capture hook after every transcript capture. It:
- resolves the current git branch and worktree root via `git rev-parse`,
- reuses the backfill's short-id parser/resolver (`parse_agent_branch_short_id`, `resolve_ticket_prefix`) to check the branch against the ticket store,
- only writes when no worktree assignment already exists (never overwrites a real `check_in_worktree`),
- never writes an unresolved ticket id,
- is fully best-effort: any resolution failure is swallowed with an `eprintln!` warning so capture never fails.

## Linked

Root cause of ticket `2b75bac2-ff14-43c3-8e87-1e801772f309` (sessions_for_ticket returns nothing).
