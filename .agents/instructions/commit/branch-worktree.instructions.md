---
description: "Use when starting, committing, or integrating any implementation task. Covers the one-worktree-one-branch isolation protocol, session and board check-in, the rebase-then-ready-to-merge handoff, and the root-orchestrator merge monopoly."
---

## Why

Multiple agents editing the same checkout at the same time is the failure mode this protocol exists to prevent: one agent's `cargo fmt`, revert, or `git add -A` silently swallows another agent's in-progress work, and the resulting commit cannot be attributed to either session. Isolation is structural — each implementation session gets its own git worktree on its own branch, so two agents physically cannot write the same file.

## The Loop

One implementation task, start to merge:

1. **Bootstrap** — the root orchestrator creates the worktree and branch from `main`.
2. **Claim** — the implementation agent checks in on the session store and the ticket board.
3. **Work** — all edits, builds, and tests happen inside the worktree only.
4. **Commit** — commits land on the feature branch, never on `main`.
5. **Rebase** — the feature branch rebases onto the updated `main` and resolves every conflict on its own side.
6. **Mark ready** — the agent checks out of the board with a `ready-to-merge:` reason and moves the ticket to `in-review`.
7. **Merge** — the root orchestrator session, and only it, fast-forwards `main` and tears the worktree down.

Steps 1 and 7 belong to the root orchestrator session. Steps 2 through 6 belong to the implementation session.

## Naming

| Thing | Form | Example |
|---|---|---|
| Branch | `agent/<ticket-short-id>-<slug>` | `agent/0869353b-handoff-edge-model` |
| Worktree path | `.worktrees/<ticket-short-id>-<slug>` | `.worktrees/0869353b-handoff-edge-model` |

`<ticket-short-id>` is the first 8 characters of the ticket id. `<slug>` is a lowercase hyphenated shortening of the ticket title, 40 characters or fewer. One ticket, one branch, one worktree — never two tickets on one branch.

`.worktrees/` is git-ignored at the repository root. Never commit a worktree directory.

## 1. Bootstrap (root orchestrator)

Run from the repository root, on `main`, before dispatching the implementation agent:

```bash
bash tools/worktree/worktree.sh new <short-id> <slug>
```

This is the canonical invocation — it is the single source of truth for the exact git sequence, so hand-typed variants cannot drift from it. Under the hood it runs: `git worktree add .worktrees/<short-id>-<slug> -b agent/<short-id>-<slug> main` (branching directly from LOCAL `main`, no fetch, no origin dependency), then populates every submodule OFFLINE by giving each one its own linked worktree — `git -C <main-checkout>/<submodule> worktree add --detach .worktrees/<short-id>-<slug>/<submodule> <recorded-sha>` — rolling back the partial worktree on persistent failure. Local `main` is authoritative here, not `origin/main`: this repo's local `main` and its recorded submodule commits are routinely ahead of, or entirely absent from, `origin`, so origin is never a valid source for either. That sequence stays discoverable here as a manual fallback if the script is ever unavailable. Pass `--dry-run` to print the exact commands without running them.

The branch is always cut from `main`, never from another feature branch. If the branch already exists, the worktree is being re-created for an interrupted task — use `git worktree add .worktrees/<short-id>-<slug> agent/<short-id>-<slug>` without `-b`.

Pass the resolved worktree path to the implementation agent in its context bundle. The agent does not derive the path itself.

## 2. Claim (implementation agent, before the first edit)

Two claims, both required, in this order.

**Session claim** — records the authoritative session-to-worktree-to-branch assignment and rejects a second session claiming the same worktree:

```
session_check_in {
  workspace: "default",
  session_id: "<this session id>",
  owner_id: "<agent id>",
  ticket_id: "<full ticket uuid>",
  worktree_path: "<path to .worktrees/<short-id>-<slug>>",
  branch: "agent/<short-id>-<slug>"
}
```

**Board claim** — records ticket and file ownership so other agents can see the scope is taken:

```
board_check_in {
  workspace: "default",
  ticket_id: "<ticket id>",
  agent_id: "<agent id>",
  intent: "branch=agent/<short-id>-<slug> worktree=.worktrees/<short-id>-<slug> — <one-line intent>",
  files: ["<repo-relative path>", "..."]
}
```

A board entry has no dedicated branch column, so the branch and worktree ride in the `intent` prefix in exactly the `branch=… worktree=… — …` form above. That prefix is what a later reader greps to answer "which branch holds this ticket's work".

If `session_check_in` reports a worktree conflict, or the board shows the ticket already actively held by a different `agent_id`, **stop and escalate**. Do not proceed on a shared worktree.

## 3. Work

- Every read, edit, build, and test runs with the worktree as the working directory. A command run from the repository root is a bug — it touches the wrong checkout.
- Never run `git checkout`, `git switch`, or `git stash` in the repository root from inside an implementation session.
- Keep the claimed file list current with `board_update_files` when scope shifts.
- Refresh `board_heartbeat` before the TTL elapses on long tasks.

## 4. Commit

Commits land on the feature branch inside the worktree. [workflow.instructions.md](workflow.instructions.md) still governs staging batches, generated outputs, and message conventions, and [submodule.instructions.md](submodule.instructions.md) still governs deepest-first submodule ordering. Two additions:

- Verify the branch before staging. `git -C <worktree> branch --show-current` must print `agent/<short-id>-<slug>`. If it prints `main`, stop — the session is in the wrong checkout.
- Stage only files the board entry claims. `git add -A` from an implementation session is forbidden; it is exactly how an unrelated agent's work gets swallowed.

## 5. Rebase onto main

Conflicts are resolved by the feature branch, never by the integrator. Before marking ready:

```bash
bash tools/worktree/worktree.sh rebase <name>
```

This rebases `<name>`'s branch onto LOCAL `main`; it never fetches `origin`, and it stops on conflict and never auto-resolves or aborts, so conflict resolution still happens by hand inside the worktree:

```bash
git -C <worktree> status
# fix conflicts, then:
git -C <worktree> add <resolved files>
git -C <worktree> rebase --continue
```

After the rebase completes, re-run the validation commands from the handoff package. A rebase that compiles is not a rebase that passes — re-validate, do not assume.

If the rebase cannot be completed — a semantic conflict the agent cannot resolve — abort it with `git rebase --abort`, leave the branch as it was, and escalate with the conflicting paths named. Do not mark the branch ready.

## 6. Mark ready to merge

Only after the rebase is clean and validation passes:

```
board_check_out {
  workspace: "default",
  ticket_id: "<ticket id>",
  agent_id: "<agent id>",
  handoff_reason: "ready-to-merge: agent/<short-id>-<slug> @ <commit sha> — rebased onto origin/main, <validation command> passed"
}
```

Then move the ticket to `in-review`. The `ready-to-merge:` prefix is the marker the root orchestrator greps in `board_history` to find integrable branches. A branch without that marker is not ready, however finished it looks.

## 7. Merge (root orchestrator only)

**No implementation session ever merges into `main`.** The root orchestrator session holds the merge monopoly, because merge order across concurrent branches is a global decision and no worker session sees the other branches.

Because the branch already rebased onto `main`, integration is a fast-forward and must be asserted as one:

```bash
bash tools/worktree/worktree.sh merge <name>
```

For every branch-bearing nested submodule worktree, `merge` first checks out the corresponding main-checkout submodule's local `main` and fast-forwards it from the nested branch. Detached nested submodule worktrees are skipped because no branch exists to integrate. The helper then runs `git checkout main` and `git merge --ff-only agent/<short-id>-<slug>` in the superproject, so the resulting gitlink records the current local submodule `main` commit. Any submodule fast-forward failure stops integration before the superproject is merged. If the superproject `--ff-only` fails, `main` moved after the branch rebased. Do not merge — send the branch back through step 5 for a fresh rebase. Never resolve a conflict on `main`.

Tear down after a successful merge:

```bash
bash tools/worktree/worktree.sh remove <name>
```

This runs `git worktree remove --force .worktrees/<short-id>-<slug>`, `git worktree prune`, then `git branch -d agent/<short-id>-<slug>`.

`git worktree remove` refuses outright with `fatal: working trees containing submodules cannot be moved or removed` while the worktree's submodules are still initialized — `--force` alone bypasses that refusal, is safe once the branch is confirmed merged (the fast-forward above already proves it), and needs no prior deinit step. Use `-d`, never `-D`, so git refuses to delete a branch that was not actually merged.

One sharp edge to avoid, not perform: never run `git submodule deinit` inside a linked worktree. It rewrites `submodule.*` in `.git/config`, which is **shared by every worktree of the repository**, so deinitializing inside the worktree silently deinitializes the main checkout too. `--force` on `git worktree remove` handles initialized submodules directly and makes that deinit step both unnecessary and harmful — do not add it back. If a submodule deinit ever happens by accident (a hand-typed command, another tool), repair with `git submodule init && git submodule update --init --recursive` in the main checkout and confirm with `git submodule status` that no entry carries a `-` prefix; working-tree files are not lost when this happens, only the config registration. `bash tools/worktree/worktree.sh doctor` diagnoses and repairs exactly this state. Every mutating subcommand of `worktree.sh` (`new`, `rebase`, `merge`, `remove`, `doctor`) must be run from the main checkout, never from inside a linked worktree.

## Submodules

This repository is a superproject with submodules (`memory-api`, `memory-viewers`, `context-stack`, `viewer-api`, `memory-kernel`), each tracking `main`. A new superproject worktree does not populate them, which is why bootstrap runs `submodule update --init --recursive`.

When the change touches a submodule:

- Bootstrap must initialize every submodule the build needs, not just the one being edited. The root `Cargo.toml` lists workspace members inside several submodules, so `cargo` fails to load the workspace with `failed to read <submodule>/Cargo.toml` if any are left uninitialized.
- Cut a matching `agent/<short-id>-<slug>` branch inside that submodule's checkout within the worktree before editing it.
- Commit the submodule first, then the superproject pointer — the deepest-first rule in [submodule.instructions.md](submodule.instructions.md) is unchanged.
- Rebase (step 5) and merge (step 7) apply to the submodule branch too: submodule first, superproject second.

## Escalation triggers

Stop and escalate rather than improvising when:

- `session_check_in` reports a worktree conflict.
- The board shows the ticket actively held by another `agent_id`.
- `git branch --show-current` prints `main` inside an implementation session.
- A rebase conflict is semantic rather than textual.
- `git merge --ff-only` fails during integration.
