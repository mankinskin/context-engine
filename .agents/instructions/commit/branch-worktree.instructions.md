---
description: "Use when starting, committing, or integrating any implementation task. Covers the one-worktree-one-branch isolation protocol, session and board check-in, and the rebase-then-merge-into-main workflow the implementation session completes itself."
---

## Why

A worktree may already have been provisioned automatically by the capture hook before an implementation session begins. [worktree-provisioning.instructions.md](../session/worktree-provisioning.instructions.md) documents the automatic bootstrap policy, diagnostics, and manual fallback; this guide owns the manual branch, claim, rebase, and merge protocol.

Multiple agents editing the same checkout at the same time is the failure mode this protocol exists to prevent: one agent's `cargo fmt`, revert, or `git add -A` silently swallows another agent's in-progress work, and the resulting commit cannot be attributed to either session. Isolation is structural — each implementation session gets its own git worktree on its own branch, so two agents physically cannot write the same file.

## When This Applies

Use this protocol when [AGENTS.md](../../../AGENTS.md#task-routing)'s Task Routing worktree threshold applies (changes spanning multiple files or components, submodules, active concurrent work, or risky behavior changes). The small main-checkout path does not require worktree provisioning, session check-in, or board check-in; stage only the changed path and validate before committing.

## The Loop

One worktree-backed implementation task, start to merge:

1. **Bootstrap** — the root orchestrator creates the worktree and branch from `main`.
2. **Claim** — the implementation agent checks in on the session store and the ticket board.
3. **Work** — all edits, builds, and tests for the worktree-backed task happen inside the worktree.
4. **Commit** — commits land on the feature branch, never on `main`.
5. **Rebase** — in every affected submodule first, then in the superproject, the feature branch rebases onto that repository's updated `main` and resolves every conflict on its own side.
6. **Mark ready** — the agent checks out of the board with a `ready-to-merge:` reason and moves the ticket to `in-review`.
7. **Merge** — the implementation session itself fast-forwards every affected `main` (submodules first, superproject last) and tears its own worktree down.

Step 1 belongs to the root orchestrator session. Steps 2 through 7 belong to the implementation session — the same session that did the work also finishes the merge, since it is the one holding the rebased, validated branch.

## Naming

| Thing | Form | Example |
|---|---|---|
| Branch | `agent/<full-session-uuid>/<slug>` | `agent/<full-session-uuid>/<slug>` |
| Worktree path | `.worktrees/<full-session-uuid>/<slug>` | `.worktrees/<full-session-uuid>/<slug>` |

`<full-session-uuid>` is the complete session UUID. `<slug>` is a lowercase hyphenated shortening of the task title, 40 characters or fewer. One session, one active slug directory, one branch, one worktree — never two active slug directories for one session UUID.

For auto-provisioned sessions, use `<full-session-uuid>/<topic-slug>` for the worktree and `agent/<full-session-uuid>/<topic-slug>` for the branch. `<topic-slug>` is lowercase kebab-case, describes the work rather than the ticket id, has 2-4 words, and is 40 characters or fewer. Do not use dates, agent names, or `tmp`, `test`, or `scratch`; a bare ticket id is not a slug because `<full-session-uuid>` already carries identity and the slug must add meaning.

`<full-session-uuid>/session` is the hook-assigned placeholder meaning "topic not yet declared". Rename the placeholder before session check-in. Existing flat `.worktrees/<short-id>-<slug>` worktrees remain supported during transition and are not migrated; nested layout wins when both layouts resolve for one UUID. More than one valid nested or legacy candidate is the deterministic `AmbiguousSessionWorktree` error, not a selection.

`.worktrees/` is git-ignored at the repository root. Never commit a worktree directory.

## 1. Bootstrap (root orchestrator)

Run from the repository root, on `main`, before dispatching the implementation agent:

```bash
./target/debug/worktree-ctl.exe new <full-session-uuid> <slug>
```

For a worktree that also needs the repository-local stores and generated
Copilot surfaces initialized, use the one-line bootstrap command instead:

```bash
./target/debug/worktree-ctl.exe bootstrap <full-session-uuid> <slug>
```

`bootstrap` creates or reuses the same worktree as `new`, then runs the
worktree's `init.sh`. `new` remains Git/submodule-only for callers that need
to defer repository initialization. Re-running `bootstrap` safely retries a
failed initializer without creating a second worktree; `--dry-run` reports
both actions without modifying either checkout.

This is the canonical invocation — it is the single source of truth for the exact git sequence, so hand-typed variants cannot drift from it. The CLI requires a full UUID and runs: `git worktree add .worktrees/<full-session-uuid>/<slug> -b agent/<full-session-uuid>/<slug> main` (branching directly from LOCAL `main`, no fetch and no origin dependency), then populates every submodule OFFLINE by giving each one its own linked worktree — `git -C <main-checkout>/<submodule> worktree add --detach .worktrees/<full-session-uuid>/<slug>/<submodule> <recorded-sha>` — rolling back the partial worktree on persistent failure. Local `main` is authoritative here, not `origin/main`: this repo's local `main` and its recorded submodule commits are routinely ahead of, or entirely absent from, `origin`, so origin is never a valid source for either. Pass `--dry-run` to print the exact commands without running them.

The branch is always cut from `main`, never from another feature branch. If the branch already exists, the worktree is being re-created for an interrupted task — use `git worktree add .worktrees/<full-session-uuid>/<slug> agent/<full-session-uuid>/<slug>` without `-b`.

Pass the resolved worktree path to the implementation agent in its context bundle. The agent does not derive the path itself.

## 1b. Name the topic (rename the worktree)

As soon as the implementation agent knows the topic, rename the hook-provisioned placeholder exactly once for that topic, before the first edit and before step 2 (Claim). Run the sequence from the repository root, with no shell or other process using the worktree as its current directory. Before renaming, check for uncommitted tracked changes:

```bash
git -C .worktrees/<name> diff --stat          # unstaged tracked changes
git -C .worktrees/<name> diff --stat --cached # staged tracked changes
```

Both commands must be empty; otherwise commit or stash the tracked changes first. Untracked `.session/sessions/` entries do not block a rename: the capture hook writes those continuously as background noise.

```bash
./target/debug/worktree-ctl.exe rename <full-session-uuid>/session <full-session-uuid>/<topic-slug>
```

`git worktree move` is unusable in this repository because every worktree contains five submodule linked worktrees. `worktree-ctl rename` uses filesystem relocation, top-level repair, and branch rename instead.

The ordering is mandatory: `session_check_in` records `worktree_path` and `branch`, with no update surface or topic/slug field in [memory-api/crates/session-api/src/store.rs](memory-api/crates/session-api/src/store.rs). Renaming after check-in strands the stored path and branch.

Verify that the top-level repair kept every submodule populated:

```bash
git -C .worktrees/<full-session-uuid>/<topic-slug> submodule status
```

The output must show all populated submodules: `memory-viewers`, `context-stack`, `memory-api`, `viewer-api`, and `workflow-tools` (which nests `memory-kernel` and the domain repos recursively). Only when fewer are populated, run `git -C .worktrees/<full-session-uuid>/<topic-slug>/<submodule> worktree repair` for each affected submodule. Then `cd` into `.worktrees/<full-session-uuid>/<topic-slug>` and proceed to step 2 (Claim).

### Renaming again when focus changes

Re-renaming is allowed but should be rare: run `./target/debug/worktree-ctl.exe rename <current-name> <target-name>` only when scope materially changes to a different feature or ticket, not for every sub-task. Re-run `session_check_in` with the new `worktree_path` and `branch` so the store is not stale, and run `board_check_in` when the claimed files change. Do not rename with uncommitted tracked modifications, staged or unstaged; commit or stash those first. Untracked `.session/sessions/` entries are capture-hook background noise and never block a rename. Never rename while a viewer, `cargo` build, or another agent has its current directory inside the worktree.

## 2. Claim (implementation agent, before the first edit)

Two claims, both required, in this order.

Resolve session identity and use the closing traceability footer described in [session-identity-and-handoff.instructions.md](../session/session-identity-and-handoff.instructions.md).

**Session claim** — records the authoritative session-to-worktree-to-branch assignment and rejects a second session claiming the same worktree:

```
session_check_in {
  workspace: "default",
  session_id: "<this session id>",
  owner_id: "<agent id>",
  ticket_id: "<full ticket uuid>",
  worktree_path: "<path to .worktrees/<full-session-uuid>/<slug>>",
  branch: "agent/<full-session-uuid>/<slug>"
}
```

**Board claim** — records ticket and file ownership so other agents can see the scope is taken:

```
board_check_in {
  workspace: "default",
  ticket_id: "<ticket id>",
  agent_id: "<agent id>",
  intent: "branch=agent/<full-session-uuid>/<slug> worktree=.worktrees/<full-session-uuid>/<slug> — <one-line intent>",
  files: ["<repo-relative path>", "..."]
}
```

A board entry has no dedicated branch column, so the branch and worktree ride in the `intent` prefix in exactly the `branch=… worktree=… — …` form above. That prefix is what a later reader greps to answer "which branch holds this ticket's work".

If `session_check_in` reports a worktree conflict, or the board shows the ticket already actively held by a different `agent_id`, **stop and escalate**. Do not proceed on a shared worktree.

After both claims succeed, persist `git status --short` as the session's
worktree baseline. Later commit, review, and handoff reports classify changes
relative to that baseline; they do not attribute every dirty path to the active
agent. Refresh the baseline only after a committed checkpoint, and retain the
previous checkpoint pointer.

## 3. Work

- For a worktree-backed task, every read, edit, build, and test runs with the worktree as the working directory. A command run from the repository root is a bug — it touches the wrong checkout.
- Never run `git checkout`, `git switch`, or `git stash` in the repository root from inside an implementation session.
- Keep the claimed file list current with `board_update_files` when scope shifts.
- Refresh `board_heartbeat` before the TTL elapses on long tasks.

### Entity store targeting is explicit

The active-session marker no longer exists. The assigned worktree's
`.session/sessions/<session-uuid>/session.json` manifest carries runtime state,
and agents supply the Copilot session UUID explicitly from the hook payload.

The handoff package MUST separately declare `entity_store_root`. Do not assume
that the code worktree, main checkout, or current directory owns the canonical
ticket, spec, test, or session store. Every state-store command passes the
declared root explicitly. After a write, read the entity back through the same
transport and the same root; success against a different discovered or shadow
store is not evidence that the intended mutation occurred.

`.session`, `.ticket`, and `.spec` are version-controlled, so every worktree carries its own copy. The active copy is the one **inside the session's worktree**. The main checkout's copies are a merge target: they become current only when a branch merges, never by direct edit.

- A session writes entity records only into its own worktree's stores. Writing an active store in the main checkout from an implementation session is forbidden — it forks authority between the store the agent can see and the store it actually wrote.
- Pass the worktree explicitly on every entity CLI call, e.g. `ticket.exe --workspace <worktree> …`. Omitting it falls back to process working directory, which for a long-lived server or a shell started at the repository root is the main checkout.
- The MCP servers are started once by the editor and keep the main checkout as their working directory for the whole session. Until session-anchored resolution lands (ticket `fa2ba34b-59ec-4321-a4ce-a3c3a9295ea3`), a bare `workspace: "default"` MCP write resolves to the **main** store and is therefore unsafe from a worktree. Prefer the CLI with an explicit `--workspace`, and never verify a worktree write by reading it back through MCP — that read resolves to the other store and will confirm the wrong file.
- After any batch of entity writes, confirm the main checkout stayed clean:

```bash
git -C <repo-root> status --porcelain -- .ticket .spec .session
```

  Non-empty output means the write went to the wrong store. Stop and relocate it before continuing.

## 4. Commit

Per [AGENTS.md](../../../AGENTS.md#quality-gates)'s commit rule, worktree-backed commits land on the feature branch inside the worktree; a small main-checkout change may commit directly to `main` after validation when only its explicit path is staged. [workflow.instructions.md](workflow.instructions.md) still governs staging batches, generated outputs, and message conventions, and [submodule.instructions.md](submodule.instructions.md) still governs deepest-first submodule ordering. Two additions for worktree-backed commits:

- Verify the branch before staging. `git -C <worktree> branch --show-current` must print `agent/<full-session-uuid>/<slug>`. If it prints `main`, stop — the session is in the wrong checkout.
- Stage only files the board entry claims. `git add -A` from an implementation session is forbidden; it is exactly how an unrelated agent's work gets swallowed.

## 5. Rebase onto main

Conflicts are resolved by the feature branch, never by the integrator. `worktree-ctl rebase` is submodule-aware: it iterates every affected submodule first (checking out `<name>`'s branch and rebasing it onto that submodule's local `main`, then committing the rebased gitlink), and only then rebases the superproject worktree itself:

```bash
./target/debug/worktree-ctl.exe rebase <name>
```

This rebases `<name>`'s branch (submodules, then the superproject) onto LOCAL `main`; it never fetches `origin`, and it stops on the first conflict and never auto-resolves or aborts, so conflict resolution still happens by hand inside the worktree:

```bash
git -C <worktree> status
# fix conflicts, then:
git -C <worktree> add <resolved files>
git -C <worktree> rebase --continue
```

After the rebase completes, re-run the validation commands from the handoff package. A rebase that compiles is not a rebase that passes — re-validate, do not assume.

If the rebase cannot be completed — a semantic conflict the agent cannot resolve — abort it with `git rebase --abort`, leave the branch as it was, and escalate with the conflicting paths named. Do not mark the branch ready.

`worktree-ctl rebase`, `merge`, and `sync` guard every mutation against an unclean working tree instead of failing on it. By default, an uncommitted or untracked change in the path about to be rebased/merged (a submodule, the worktree, or the main checkout) is stashed with `git stash push --include-untracked` before the operation and popped back afterward, reported as `stashed uncommitted changes in <label> (restored afterward)`. Pass `--auto-commit` to commit the dirty state instead (`worktree-ctl auto-commit before sync`) and carry it along with the rebase/merge rather than stashing it. If restoring a stash afterward genuinely conflicts with what the rebase/merge introduced at the same path, the command reports a non-zero exit naming the stash (`git -C <path> stash list`) rather than silently discarding it — treat that as a manual-resolution case, same as any other conflict.

## 6. Mark ready, then merge (implementation session)

Only after the rebase is clean and validation passes:

```
board_check_out {
  workspace: "default",
  ticket_id: "<ticket id>",
  agent_id: "<agent id>",
  handoff_reason: "ready-to-merge: agent/<full-session-uuid>/<slug> @ <commit sha> — rebased onto local `main`, <validation command> passed"
}
```

Then move the ticket to `in-review`. The `ready-to-merge:` prefix is the marker recorded in `board_history` documenting that the branch was integrable at that point, even though this same session now proceeds to merge it immediately rather than waiting for a separate session to pick it up.

Immediately continue into step 7 (Merge) in the same session — do not stop after marking ready. There is no handoff to a different orchestrator session for the merge itself; the implementation session that produced and validated the branch is also the one that integrates it, precisely because it is the only session that has seen the rebase resolve cleanly and the validation pass.

## 7. Merge (same implementation session)

**The implementation session merges its own branch into `main`.** Do not wait for or delegate to a separate root-orchestrator session to perform this step — do it directly, right after step 6, while validation is still fresh.

### Bottom-up integration sequence (canonical)

The superproject records submodules as gitlinks. **Every gitlink recorded by the superproject MUST be contained in the corresponding submodule's `main` branch.** The prior failure treated the gitlink as the record of truth while the submodule branch was left behind, leaving orphaned commits reachable only through the gitlink and vulnerable to `git gc`.

Never merge the superproject branch while any affected submodule branch is unmerged. For a change spanning submodules and the superproject, run this sequence from the superproject root:

1. **Pin before rewriting history.** In each affected submodule, pin the feature tip before rebasing: `git -C <sm> branch rescue/pre-rebase-<short-sha> <sha>`. Also pin any recorded gitlink that will be superseded: `git -C <sm> branch rescue/gitlink-<short-sha> <gitlink-sha>`.
2. **Integrate each affected submodule, deepest first.** Rebase `agent/<full-session-uuid>/<slug>` onto that submodule's `main`, resolve conflicts there, validate it, then fast-forward: `git -C <sm> checkout main && git -C <sm> merge --ff-only agent/<full-session-uuid>/<slug>`.
3. **Bump gitlinks on the superproject feature branch.** Run `git add <sm>` for every merged submodule and commit the pointer updates, so the feature branch records each submodule's new `main` tip.
4. **Verify containment before the superproject merge.** Run the invariant loop below; all five entries must print `ok`.
5. **Rebase and integrate the superproject last.** Rebase `agent/<full-session-uuid>/<slug>` onto superproject `main`, resolve conflicts on the feature branch, then fast-forward superproject `main` with `git merge --ff-only agent/<full-session-uuid>/<slug>`.
6. **Re-verify containment after the superproject merge.** Run the same loop again; all five entries must print `ok`.

```bash
for sm in context-stack memory-api memory-viewers viewer-api workflow-tools; do
  link=$(git ls-tree HEAD "$sm" | awk '{print $3}')
  if git -C "$sm" merge-base --is-ancestor "$link" main; then
    echo "ok   $sm $link"
  else
    echo "BAD  $sm $link not contained in $sm main"
  fi
done
```

`git submodule status` prefixes `+` when the checked-out submodule HEAD differs from the recorded gitlink and `-` when the submodule is uninitialized. A clean integration shows neither marker for any of the five submodules.

`worktree-ctl merge` automates the nested fast-forwards and enforces the containment invariant itself (it checks gitlink containment before and after, and refuses an unmerged submodule branch), but the manual sequence above remains authoritative for the rescue-branch pinning step, which no `worktree-ctl` subcommand performs.

`worktree-ctl merge` (and therefore `sync`) also self-heals a gitlink violation when there is exactly one safe resolution: if a submodule's recorded gitlink is a strict fast-forward ahead of that submodule's local `main` — reachable only through a detached HEAD or an unnamed commit, with no divergence to arbitrate — it fast-forwards `main` to the recorded commit automatically and reports `auto-fixed gitlink: <submodule> local main fast-forwarded to <sha>`; `--dry-run` reports the same action as `auto-fix gitlink: ...` without mutating anything. A violation is left for manual resolution whenever the fix is not unique: true divergence (recorded commit and local `main` share no fast-forward path) or a recorded commit missing from the submodule's object database still error out exactly as before.

### One-command shorthand: `worktree-ctl sync`

For the common case — no rescue-branch pinning needed — `worktree-ctl sync <name>` composes step 5 (rebase) and step 7 (merge) behind one command: it rebases every affected submodule then the superproject worktree onto local `main`, and only if that rebase succeeds does it fast-forward every affected submodule `main` then the superproject `main`. A rebase conflict stops `sync` before any merge is attempted — resolve the conflict by hand inside the worktree (`git -C <path> add <resolved files> && git -C <path> rebase --continue`), then rerun `worktree-ctl sync <name>` to finish the remaining rebase steps and merge. `--dry-run` prints the full combined plan (every submodule rebase/skip, the superproject rebase, every submodule fast-forward/skip, and the superproject fast-forward) without mutating anything.

Because every affected branch has rebased onto its repository's `main`, each integration is a fast-forward and must be asserted as one:

```bash
git -C <sm> checkout main && git -C <sm> merge --ff-only agent/<full-session-uuid>/<slug>
git checkout main && git merge --ff-only agent/<full-session-uuid>/<slug>
```

If any `--ff-only` fails, the target `main` moved after the branch rebased. Do not merge — send the branch back through step 5 for a fresh rebase. Never resolve a conflict on `main`.

Tear down after a successful merge:

```bash
./target/debug/worktree-ctl.exe remove <name>
```

This runs `git worktree remove --force .worktrees/<full-session-uuid>/<slug>`, `git worktree prune`, then `git branch -d agent/<full-session-uuid>/<slug>`. The session-UUID parent directory is removed only when empty, so a sibling slug preserves the parent.

`worktree-ctl remove` refuses a dirty worktree unless `--force` is explicit. Its successful force path uses `git worktree remove --force`, which bypasses Git's refusal to remove initialized submodules once the branch is confirmed merged (the fast-forward above already proves it) and needs no prior deinit step. The CLI uses `-d`, never `-D`, so git refuses to delete a branch that was not actually merged.

One sharp edge to avoid, not perform: never run `git submodule deinit` inside a linked worktree. It rewrites `submodule.*` in `.git/config`, which is **shared by every worktree of the repository**, so deinitializing inside the worktree silently deinitializes the main checkout too. `--force` on `git worktree remove` handles initialized submodules directly and makes that deinit step both unnecessary and harmful — do not add it back. If a submodule deinit ever happens by accident (a hand-typed command, another tool), repair with `git submodule init && git submodule update --init --recursive` in the main checkout and confirm with `git submodule status` that no entry carries a `-` prefix; working-tree files are not lost when this happens, only the config registration. `worktree-ctl doctor` diagnoses and repairs exactly this state. Every mutating `worktree-ctl` subcommand (`new`, `rebase`, `merge`, `sync`, `remove`, `rename`, `finish`, `doctor`) must be run from the main checkout, never from inside a linked worktree.

## Submodules

This repository is a superproject with submodules (`memory-api`, `memory-viewers`, `context-stack`, `viewer-api`, `workflow-tools`), each tracking `main`. `workflow-tools` nests `memory-kernel` and the domain repos as its own submodules. A new superproject worktree does not populate them, which is why bootstrap runs `submodule update --init --recursive`.

When the change touches a submodule:

- Bootstrap must initialize every submodule the build needs, not just the one being edited. The root `Cargo.toml` lists workspace members inside several submodules, so `cargo` fails to load the workspace with `failed to read <submodule>/Cargo.toml` if any are left uninitialized.
- Cut a matching `agent/<full-session-uuid>/<slug>` branch inside that submodule's checkout within the worktree before editing it.
- Commit the submodule first, then the superproject pointer — the deepest-first rule in [submodule.instructions.md](submodule.instructions.md) is unchanged.
- Rebase (step 5) and merge (step 7) apply to the submodule branch too: follow the canonical bottom-up sequence above.

## Escalation triggers

Stop and escalate rather than improvising when:

- `session_check_in` reports a worktree conflict.
- The board shows the ticket actively held by another `agent_id`.
- `git branch --show-current` prints `main` inside an implementation session.
- A rebase conflict is semantic rather than textual.
- `git merge --ff-only` fails during integration — `main` moved after the branch rebased; rebase again (step 5) and retry the merge yourself rather than treating it as someone else's problem.
