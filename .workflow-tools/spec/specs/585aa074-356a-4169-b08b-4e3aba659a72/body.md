<!-- aligned-structure:v2 -->

# Session Worktree Lifecycle Rewrite

## Motivation

The session component needs a Rust replacement for `tools/worktree/worktree.sh` so worktree lifecycle operations preserve repository and submodule invariants while allowing completed, disposable worktrees to be reused without losing locally built artifacts or entity-store indexes.

## Dependent expectation

If this specification is implemented, dependents can rely on session worktrees being locked during active session ownership, automatically preserved when they contain user work, and reclaimed in place only when their completed state is clean and fully reachable from local `main`.

## Scope

The Rust `worktree-ctl` binary owns local worktree lifecycle operations. The binary supports `new`, `list`, `rebase`, `merge`, `remove`, `rename`, `finish`, and `doctor`.

All operations use local `main` and recorded local git objects. No operation fetches from or otherwise depends on `origin`.

### Subcommand contract

- `new <short-id> <slug>` creates or reuses `.worktrees/<short-id>-<slug>` with branch `agent/<short-id>-<slug>` from local `main`, populates every recorded submodule offline through linked submodule worktrees, and rolls back a failed partial bootstrap. A repeated request for the same session identity reuses the existing worktree. A second worktree for an identity is rejected unless `--allow-additional` is explicit. When the main checkout has tracked changes, creation refuses and identifies the changes unless `--preserve-main-changes` explicitly stashes and later allows restoration of the changes.
- `list` reports registered worktrees and their lifecycle-relevant state without mutating Git state.
- `rebase <name>` rebases the named feature branch onto local `main`; conflicts stop the operation for human resolution and are never auto-resolved or auto-aborted.
- `merge <name>` first fast-forwards every branch-bearing nested submodule worktree into the corresponding local submodule `main`, skips detached nested worktrees, then fast-forwards the superproject local `main` from `agent/<name>`. A non-fast-forward condition fails without merging the superproject.
- `remove <name>` removes a completed worktree only when clean, unless an explicit force operation is requested. A normal removal identifies blocking dirty paths and preserves the worktree. Successful force removal uses `git worktree remove --force`, prunes registrations, and deletes only a merged branch with `git branch -d`.
- `doctor` diagnoses and repairs worktree registration and submodule initialization damage without deinitializing a submodule.

### Lifecycle state machine

- `active/locked -> complete`: session ownership ends or becomes inactive; no automatic purge, rename, or repurpose is allowed while a session remains active.
- `complete -> preserved`: preserve when any tracked or untracked worktree change exists, any nested submodule is dirty, the worktree has commits not reachable from local `main`, the worktree lacks a branch, the process current directory is inside the worktree, or the worktree has not been idle longer than `WORKTREE_IDLE_SECS` (default `86400`).
- `complete -> reclaimed`: reclaim only when there is no session-store activity, the worktree has a branch, the worktree and all nested submodules are clean, the current directory is outside the worktree, the idle threshold has elapsed, and the branch is zero commits ahead of local `main`.
- `reclaimed -> active/locked`: reuse changes the worktree topic and branch in place for the incoming session, then records the new active ownership.
- `preserved -> active/locked`: an explicit agent re-topicing decision may claim the preserved worktree; automatic repurposing is prohibited.

### Reclamation and preservation

Reclamation is a filesystem rename/move in place followed by `git worktree repair` and branch rename. Remove-and-recreate is forbidden because it discards built artifacts, `target/` contents, and rebuilt entity-store indexes. The in-place path preserves those assets and must also preserve any submodule commit object that is ahead of the recorded gitlink.

`git worktree move` is prohibited because linked worktrees in this repository contain five submodules and Git rejects such a move. The required alternative is filesystem relocation followed by `git worktree repair`, with nested `git worktree repair` only when a submodule remains unregistered after top-level repair.

`git submodule deinit` is prohibited during teardown. The command rewrites shared `.git/config` state and can silently deinitialize submodules in the main checkout. `git worktree remove --force` handles initialized submodules without deinitialization.

### Git access boundary

The binary uses libgit2 for every read: repository opening, worktree enumeration, branch existence, dirty-state checks, ahead/behind evaluation, gitlink lookup, and `.gitmodules` parsing.

The binary uses the `git` subprocess only for writes that libgit2 cannot express: `git worktree add -b`, `git worktree add --detach`, filesystem relocation followed by `git worktree repair`, `git branch -m`, `git worktree remove --force`, `git worktree prune`, `git branch -d` or `git branch -D`, and nested `git worktree repair`.

### Dry run

Every mutating lifecycle action, including `new`, `rebase`, `rename`/re-topic, `merge`, `remove`, `finish`/completion handling, and `doctor`, accepts `--dry-run`. A dry run emits its local-Git plan, makes no filesystem or Git mutation, changes neither worktree paths nor local `main`, and never references `origin`.

## Non-goals

- Rewriting or extending the retiring Bash implementation to satisfy lifecycle behavior.
- Fetching, comparing against, or otherwise requiring `origin` or `origin/main`.
- Automatic reclamation of a dirty worktree, a worktree with unreachable commits, or a worktree owned by an active session.
- Remove-and-recreate reclamation, `git worktree move`, or `git submodule deinit`.
- Automatic conflict resolution for rebase or non-fast-forward merge.

## Acceptance criteria

1. A Rust `worktree-ctl` replacement implements the declared lifecycle surface and retires `tools/worktree/worktree.sh` plus the Bash test harness after equivalent Rust coverage exists.
2. `new` boots from local `main`, works without `origin`, resolves a submodule object recorded only in local `main`, and populates submodules offline at their recorded gitlinks.
3. Worktree identity reuse, explicit `--allow-additional`, dirty-main acknowledgement/preservation, in-place re-topic, and dirty-removal refusal satisfy the lifecycle predicates above.
4. Completion preserves uncommitted changes and commits unreachable from `main`; only the complete-to-reclaimed predicate permits automated reuse.
5. Reclamation retains built artifacts and entity indexes through filesystem relocation plus repair, never remove-and-recreate or `git worktree move`.
6. `rebase`, `merge`, `remove`, and `doctor` uphold the local-main, fast-forward, submodule, and no-deinitialization guarantees above.
7. `--dry-run` is available for every mutating lifecycle action and is observational only.
8. The Rust test suite covers the intent of all 16 retiring shell tests: `test_all_lifecycle_ops_support_dry_run`, `test_bootstrap_populates_submodule_offline`, `test_bootstrap_resolves_main_only_commit`, `test_create_preserves_dirty_main_checkout`, `test_create_requires_acknowledgement_when_dirty`, `test_dry_run_plan_has_no_origin`, `test_finish_rebases_marks_ready_and_removes`, `test_no_origin_references`, `test_no_submodule_deinit`, `test_no_worktree_move`, `test_parses_clean`, `test_remove_refuses_dirty_worktree`, `test_rename_is_remove_and_recreate`, `test_rename_preserves_commit_ahead_of_gitlink`, `test_second_worktree_requires_explicit_override`, and `test_session_reuses_existing_worktree`.

## Guards

No validation-spec identifiers exist yet. Review requires `cargo test -p worktree-ctl`, which passes the replacement Rust suite covering all retired shell contracts.

## Positions

- `tools/worktree/worktree.sh`: retired after equivalent Rust coverage passed.
- `tools/worktree/tests/run.sh`: retired after the Rust suite superseded all 16 shell contracts.
- `memory-api/crates/session-worktree-provision/src/policy.rs`: implemented prior art; reuse, reclaim, and create gates define the compatible lifecycle predicate and hybrid Git-access model.
- `worktree-ctl` crate: implemented with 28 passing tests (10 unit, 15 lifecycle-contract, and 3 maintenance tests).

## Governing-rule requirement

The session worktree lifecycle policy must introduce this specification whenever a session is assigned a worktree, provisions a worktree, or evaluates completed worktree reclamation. The policy must state the active lock, preservation predicate, reclaim predicate, and Git safety prohibitions before lifecycle automation acts.

## Traceability and evidence

- Related ticket: [5e6cf4f8 Rewrite worktree.sh as a Rust binary and add worktree lifecycle recycling](../../../.ticket/tickets/5e6cf4f8-120c-4674-95de-d7b79c99f5b3/ticket.toml).
- Current entry point: `./target/debug/worktree-ctl.exe`.
- Retired shell-contract runner: `tools/worktree/tests/run.sh`.
- Prior-art implementation: `memory-api/crates/session-worktree-provision/src/policy.rs`.
- Required current-contract command: `rtk cargo test -p worktree-ctl`.
- Implementation evidence must include the replacement Rust test command(s), a passing result for the migrated 16-contract suite, and proof that no retiring shell script remains as the executable lifecycle implementation.

## Related specifications

No existing specification owns ticket `5e6cf4f8-120c-4674-95de-d7b79c99f5b3`. Nearby ticket-move and recurring-principles specifications are not lifecycle contracts and are intentionally not dependencies of this specification.
