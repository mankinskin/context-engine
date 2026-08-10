---
name: "epic-db6980d1"
description: "Run the directed implementation program for the worktree provisioning and session-worktree lifecycle epic."
argument-hint: "Optional phase, ticket id, or 'resume'. Defaults to the first unblocked phase."
agent: "Orchestrator Agent"
---

# Implement Worktree Lifecycle Epic

Drive epic `db6980d1` (Worktree provisioning and session-worktree lifecycle) as a directed, multi-session program. The orchestrator owns planning, dispatch, aggregation, verification, and escalation. Worker agents receive one complete implementation unit, validate it, return evidence, and terminate.

Do not start a later phase until the current phase's gates pass. Do not substitute a ticket's self-report for source, test, or recorded validation evidence.

## Program Goal

Make worktree provisioning, session-to-worktree assignment, lifecycle tooling, and worktree-aware routing safe and observable across the repository. The program must eliminate the observed safety failures before adding broader cleanup, recycling, rename, or automatic provisioning behavior.

The worktree epic covers the following workstreams:

- Safety and recovery: `723c2bea`, `e068602b`, and `69e69b4b`.
- Shell and Rust lifecycle tooling: `2b65715`, `4ef88dbc`, `503b9711`, `5e6cf4f8`, and `72314c5e`.
- Provisioning and capture: `a1b911ab`, `40349f3f`, and `3d535b2c`.
- Session, board, and MCP routing: `c060bf94`, `fa2ba34b`, and `ff83caf7`.
- Guidance and verification discipline: `e38c258e` and `bd044e88`.

Historical context only: `e2189e9d`, `326bfe38`, `68a49ca7`, and `b6af9f40` are done. Do not reopen the historical tickets unless new evidence proves a regression in their own scope.

Before Phase 0, re-check epic membership and attach the currently unlinked in-scope tickets when they appear in the current ticket store: `565ae4b1` (provisioning observability), `614fae19` (board/session claim conflict), and `e70471d4` (MCP path rewriting). Do not create duplicates for the unlinked tickets.

## Non-Negotiable Safety Rules

- Run root-only Git and worktree operations from the repository root, never from inside `.worktrees/<name>`.
- Before any worktree-state diagnosis or repair, dispatch a capable executor to capture `pwd`, `git rev-parse --show-toplevel`, `git rev-parse --abbrev-ref HEAD`, `git worktree list`, root `git status --short`, and root `git submodule status`.
- Resolve `core.worktree` relative to `.git/modules/<submodule>/`. The value `../../../<submodule>` is valid when it resolves to the main checkout's submodule directory.
- Do not run `git submodule deinit`, `git reset --hard`, a force-push, or an unscoped prune sweep.
- Treat an internally inconsistent worktree-state report as invalid. Re-run the check with a capable implementation tier before changing Git metadata.
- Do not remove, recycle, or broaden automatic cleanup until Phase 1 safety gates pass.
- Do not use an existing dirty worktree for a new implementation unit. Rename is permitted with untracked `.session/sessions/` artifacts only, but staged or unstaged tracked modifications block rename until committed or stashed.
- After ticket, spec, or session-store writes in a worktree, dispatch an executor to run `git status --porcelain -- .ticket .spec .session` and include the result in the handoff.

## Tool Boundary

The Orchestrator Agent makes decisions and never runs diagnostics, validation commands, Git operations, or ticket mutations directly. The Orchestrator Agent dispatches the Explore Agent for the mandatory pre-dispatch gate, a capable T2-or-higher executor for worktree-state diagnostics and repair planning, the Implement Agent for one implementation step, and the Commit Agent for approved integration work. The root orchestrator retains the decision to fast-forward `main`; the Commit Agent executes the approved root-only merge from the repository root.

## Orchestration Protocol

For every implementation unit:

1. Dispatch a discovery or gate agent to resolve the ticket, current state, acceptance criteria, stored edges, linked spec, and active board ownership.
2. Create or resume the implementation session in a dedicated worktree. Rename the hook placeholder to `{short-id}-{topic-slug}` before `session_check_in` when the topic is known.
3. Run the pre-dispatch gate with the Explore Agent. A gate failure blocks the worker dispatch until the blocker is resolved or escalated.
4. Build an inline context bundle containing the ticket body, acceptance criteria, relevant spec body, target file skeletons, exact validation commands, current dependency evidence, and the worker contract.
5. Dispatch exactly one implementation step to an Implement Agent. The worker must not rediscover unclear scope; the worker escalates incomplete handoff packages.
6. Require the Implement Agent to run the narrowest focused validation after the first substantive edit and before further exploration or edits.
7. Dispatch Testing or Review to record validation evidence independently. Review acceptance criteria against tests, source, and test-api evidence; do not trust ticket text alone.
8. Have the appropriate ticket-owning agent move a ticket to `in-review` only after required validation passes, the linked spec is current, and the evidence is persisted.
9. Before Review, require the implementation session to rebase its feature branch onto current `main`, resolve conflicts, and re-run focused validation. Run Review and resolve any open decision through Interview. Reviewer-requested code fixes return to the Implement Agent with a fresh, complete handoff, then repeat validation, rebase, and review. After the branch is marked ready to merge, the root orchestrator authorizes one Commit Agent to attempt only a fast-forward merge from the repository root, remove the completed worktree, and delete the merged branch. A failed fast-forward returns the ticket to rebase, validation, and review; never resolve rebase conflicts during integration.
10. Re-read the epic graph after every merged unit. Start the next unblocked phase only when its gates hold.

Use one active owner per overlapping file set. Do not concurrently edit `tools/worktree/worktree-ctl`, session-worktree provisioning, or session capture routing from two workers.

## Phase 0: Close Reviewed Foundations

Objective: turn reviewed work into verified dependencies before new implementation expands the surface.

Tickets:

- `4ef88dbc` — shell worktree helper.
- `a1b911ab` — session worktree discovery and recycling.
- `e38c258e` — one-worktree-one-branch guidance.
- `c060bf94` — board/session/worktree binding.
- `40349f3f` — capture-hook worktree assignment persistence.

Execution rules:

- Review `a1b911ab` and `40349f3f` sequentially because both touch session capture and worktree assignment behavior.
- Review `e38c258e` before accepting `c060bf94`, because board binding relies on the documented one-worktree-one-branch contract.
- `4ef88dbc` may be reviewed independently, but do not modify the lifecycle tool while Phase 1 owns the same behavior.
- Resolve the anomalous ticket states before dispatching successor implementation: `4ef88dbc` is in review despite merged evidence; `2b65715` is ready despite merged evidence.

Completion gate:

- Every reviewed ticket has independently verified acceptance criteria and a terminal or correctly reopened state.
- `a1b911ab` is done before `5e6cf4f8` begins because the Rust rewrite has a stored dependency on `a1b911ab`.
- `e38c258e` is done before `c060bf94` completes.

## Phase 1: Establish Safety And Recovery

Objective: make corrupt and dangerous worktree states detectable and recoverable before any destructive lifecycle capability is broadened.

Strict order:

1. `723c2bea` — shared-config `core.worktree` hijack.
2. `e068602b` — prune removes a live submodule linked-worktree registration.
3. `69e69b4b` — reject root-only operations from inside a worktree.

No parallelism: all three touch the current worktree lifecycle tool, doctor behavior, or root-operation safety contract.

Required outcomes:

- `worktree-ctl doctor` detects `core.worktree` as `OK`, `HIJACKED`, or `DANGLING`, resolves paths correctly, exits non-zero for `DANGLING`, reports the exact remediation command, and offers or documents a non-destructive fix path.
- Doctor detects stale linked-worktree registration targets and a submodule `.git` gitdir that points to a missing registration.
- A live worktree renamed on disk remains usable through root and per-submodule prune attempts. The regression must prove `git -C <worktree> status` succeeds for all five submodules afterwards.
- Root-only operations fail fast with an actionable non-zero error if the caller's resolved cwd is inside a worktree.
- No safety implementation relies on hand-editing Git metadata as normal recovery.

Completion gate:

- Focused regression tests cover all observed failure modes.
- A clean real-repository smoke check shows root status works, all five root submodules initialize, each registered worktree directory exists, each linked-worktree registration target exists, and no shared `core.worktree` points inside `.worktrees/`.
- A reviewer verifies that the Phase 1 implementation did not make cleanup, prune, remove, or repair more destructive.

## Phase 2: Stabilize The Existing Lifecycle

Objective: finish safe shell behavior while preserving the Phase 1 recovery guarantees.

Strict order:

1. `2b65715` — handle unregistered worktree debris during removal.
2. `503b9711` — bootstrap agent worktrees from local `main`.

`2b65715` and `503b9711` do not run in parallel: both own worktree cleanup/bootstrap semantics.

Required outcomes:

- Removal handles debris without deleting a live directory or a live submodule registration.
- Bootstrap uses local `main`, supports offline five-submodule population, and leaves the main checkout healthy after success and failure.
- Existing doctor checks remain green after every remove/bootstrap test.

Completion gate:

- Script syntax validation, relevant helper tests, dry-run coverage, and an offline create/remove smoke check pass.
- A failed bootstrap leaves no broken root status, dangling `core.worktree`, or missing live registration.

## Phase 3: Replace Lifecycle Tooling In Rust

Objective: implement `5e6cf4f8` without regressing the hardened shell behavior.

Ticket: `5e6cf4f8` — Rust lifecycle migration and recycling.

Preconditions:

- `a1b911ab` is done.
- Phase 1 protections have defined the required doctor and recovery semantics.
- Phase 2 behavior is either verified or explicitly retained as compatibility coverage.

No parallel lifecycle implementation: `5e6cf4f8` owns the Rust replacement, lifecycle recycling, and migration boundary.

Required outcomes:

- Rust lifecycle commands preserve the safety contract from Phases 1 and 2 before enabling recycle or cleanup.
- The replacement preserves dirty/ahead worktree protection and all five-submodule behavior.
- Doctor diagnostics and recovery behavior are ported or shared rather than lost in migration.

Completion gate:

- The Rust test suite covers lifecycle creation, removal, recycle eligibility, doctor diagnostics, and failure recovery.
- The current shell wrapper either delegates safely or is retired with a documented migration path.

## Phase 4: Add Post-Rewrite Lifecycle Features

Objective: add topic-slug rename and session lifecycle updates after the lifecycle owner is stable.

Strict order:

1. `72314c5e` — topic-slug worktree rename command.
2. `ff83caf7` — managed session-worktree lifecycle.

Do not run the tickets in parallel. `72314c5e` defines filesystem move, repair, branch rename, and submodule safety. `ff83caf7` defines session-store update, reuse, finish, and lifecycle semantics.

Required outcomes:

- Rename provides dry-run behavior, rejects tracked dirty changes unless forced, rejects unsafe cwd, and verifies all five submodules after rename.
- Session assignment updates `worktree_path`, `branch`, and topic slug without creating an unrelated session record.
- Session and board records cannot be stranded by a rename or material topic change.

Completion gate:

- Rename tests prove old path removal, new branch ownership, five-submodule usability, and root checkout health.
- Session API tests prove worktree assignment changes preserve ticket and board resolution.

## Phase 5: Roll Out Provisioning And Routing

Objective: make the active worktree authoritative across capture, board, and MCP paths.

Strict order:

1. `fa2ba34b` — session-anchored MCP workspace resolution.
2. `3d535b2c` — prompt-time worktree bootstrap hook.

Preconditions:

- `40349f3f` and `c060bf94` are done or explicitly reopened with a resolved interface decision.
- The three stored dependencies of `3d535b2c` (`cf4d1e1a`, `f76b0fa9`, and `2d48cf8c`) are researched and satisfied before dispatch.
- `565ae4b1`, `614fae19`, and `e70471d4` are linked to the epic once their branches merge, then scheduled according to their actual edges.

No parallel capture-hook changes: MCP path routing, captured assignment, prompt injection, and worktree claims share a correctness boundary.

Required outcomes:

- MCP calls resolve to the authoritative assigned worktree without permitting cross-worktree writes.
- Prompt-time provisioning is observable as provisioned, reused, skipped, or failed.
- A simulated `UserPromptSubmit` proves first-prompt provisioning and capture assignment behavior.

Completion gate:

- Browser-facing changes receive external Chromium verification and Playwright coverage where required.
- End-to-end evidence proves board, session, and MCP calls converge on the same worktree assignment.

## Guidance Workstream

`bd044e88` may be implemented after Phase 1 establishes the verification protocol. Do not let `bd044e88` delay emergency safety work.

`bd044e88` must update agent guidance to require a capable tier for Git/worktree/submodule state verification and repair planning, explicit cwd proof, absolute-path resolution, and discard-and-rerun behavior for contradictory reports.

## Dispatch And Evidence Contract

Every worker handoff must include:

```text
objective:
primary_ticket:
acceptance_criteria:
owner_paths:
non_goals:
dependency_evidence:
starting_git_state:
validation_commands:
expected_artifacts:
return_contract: scope | edits | validation | blocker | pointers
```

Every worker return must include:

```text
scope:
edited_paths:
validation_command_and_outcome:
independent_evidence:
remaining_risk:
next_unblocked_ticket:
```

The orchestrator must spot-check load-bearing claims before using a worker result to transition a ticket, dispatch a dependent ticket, or permit a destructive lifecycle operation.

## Final Epic Closure

Close epic `db6980d1` only when every open epic member is done, cancelled with rationale, or moved outside the epic with an explicit reason. Before closure:

- Render the epic edge graph and confirm no in-scope ticket is orphaned.
- Run the full worktree health audit from the repository root.
- Confirm recovery, prune, rename, remove, recycle, provision, session assignment, board resolution, and MCP path resolution preserve the same authoritative worktree.
- Record validation evidence, complete review, commit approved work, and create the final handoff package.

## Output Format

At the end of each orchestration turn, return:

```text
current_phase:
phase_gate: pass | blocked
completed_tickets:
active_ticket:
next_dispatch:
blocked_by:
validation_evidence:
worktree_health:
epic_graph_delta:
handoff:
```