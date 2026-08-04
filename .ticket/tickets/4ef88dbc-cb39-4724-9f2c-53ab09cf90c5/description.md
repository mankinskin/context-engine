Problem: the git-worktree isolation protocol used by agents (bootstrap/rebase/merge/teardown) exists only as prose in .agents/instructions/commit/branch-worktree.instructions.md, .agents/agents/orchestrator.agent.md, and .spec/specs/2a710b29-13e9-40b8-8a53-c0ea366bd0bf/body.md. A repo-wide search across .sh/.ps1/.py/.md/.toml/.rs found ZERO executable worktree tooling — agents hand-type the multi-command git sequences every time, which has already caused real damage.

Motivating incidents (all discovered by running the protocol manually):
(a) `git worktree remove` refuses outright when the worktree has initialized submodules; `git submodule deinit --all --force` must run first.
(b) `git submodule deinit` executed INSIDE a worktree rewrites `submodule.*` entries in the SHARED .git/config, silently deinitializing the MAIN checkout's submodules. This actually happened and had to be repaired by hand — the trailing `git submodule init` in teardown is a repair step, not a formality.
(c) Bootstrap must initialize ALL submodules (memory-viewers, context-stack, memory-api, viewer-api, memory-kernel — 5 total per .gitmodules), not just the one being edited, because the root Cargo.toml has workspace members inside several of them and cargo cannot load the workspace while any are empty. Skipping this cost one wasted implementation dispatch.

This is a PROTOTYPE ticket: correctness of the encoded git sequences and the safety guards matters more than polish (error message wording, help text, etc.).

Authoritative command sequences to encode, verbatim from .agents/instructions/commit/branch-worktree.instructions.md:

Bootstrap:
```
git fetch origin
git checkout main
git pull --ff-only origin main
git worktree add .worktrees/<short-id>-<slug> -b agent/<short-id>-<slug> main
git -C .worktrees/<short-id>-<slug> submodule update --init --recursive
```

Rebase:
```
git fetch origin
git -C <worktree> rebase origin/main
git -C <worktree> rebase --continue
```

Merge:
```
git checkout main
git merge --ff-only agent/<short-id>-<slug>
```

Teardown:
```
git -C .worktrees/<short-id>-<slug> submodule deinit --all --force
git worktree remove --force .worktrees/<short-id>-<slug>
git worktree prune
git submodule init
git branch -d agent/<short-id>-<slug>
```

Repo conventions to follow: bash is the dominant scripting language (sh=16, ps1=1, py=6 in the repo). Follow the style used by install-tools.sh (`#!/usr/bin/env bash`, `set -euo pipefail`, resolve `script_dir`/`repo_root`). Multi-file tools live under `tools/<name>/` (existing precedent: tools/install, tools/model-prices, tools/agent-hooks, tools/tracing-analyzer, tools/cli). `.worktrees/` is already git-ignored.

Documentation fix in scope: `git worktree prune` appears in the teardown block of .agents/instructions/commit/branch-worktree.instructions.md but is MISSING from the equivalent teardown block in .agents/agents/orchestrator.agent.md. Add it there for consistency.

This ticket implements the existing spec 2a710b29-13e9-40b8-8a53-c0ea366bd0bf (the worktree isolation protocol spec) — do not create a new spec.
```
## Implementation evidence

Delivered on branch agent/4ef88dbc-worktree-helper, merged fast-forward to main as bb0b0ca3.
Ticket-store commit: a415b25c.

Created: tools/worktree/worktree.sh (425 lines).
Modified: .agents/agents/orchestrator.agent.md — added the missing `git worktree prune` line to the
teardown block, resolving the drift against .agents/instructions/commit/branch-worktree.instructions.md.

Subcommands implemented: new, list, rebase, merge, remove, doctor; --dry-run honored by every
mutating subcommand; destructive operations refuse to run from inside a linked worktree
(guard compares `git rev-parse --git-dir` against `--git-common-dir`).

Validation performed:
- `bash -n` parse check: pass. shellcheck: NOT RUN — not installed in this environment.
- `--help`, `list`, `doctor` executed against the real repository: pass.
- `--dry-run` on new/merge/remove mutated nothing (`git worktree list` identical before/after).
- Worktree guard refused a destructive subcommand run from inside a linked worktree.
- Real round-trip: `new test0000 scratch` bootstrapped a worktree with all 5 submodules
  initialized, then `remove test0000-scratch` tore it down; the main checkout's submodules
  survived intact (5/5, no leading `-`).
- Dogfooded: this ticket's own worktree was torn down by running the merged script itself.
  Its output shows the submodule deinit clearing all 5 submodules from the shared .git/config
  and the `git submodule init` repair step re-registering them — the sharp edge is real and
  the script handles it.
- Independent verification of all claims was performed by a separate agent against the
  committed tree rather than accepted from the implementing agent's self-report.

Known gaps: shellcheck lint unverified (not installed); the script is a prototype and is not
wired into install-tools.sh.
```


## Protocol correction: submodule deinit removed from teardown

Post-merge sandbox research on git 2.54.0.windows.1 overturned a core assumption baked into the
original teardown sequence and this ticket's own acceptance criteria (AC5, AC6).

**Superseded teardown sequence** (originally documented and implemented):
```
git submodule deinit --all --force
git worktree remove --force <path>
git worktree prune
git submodule init
git branch -d <branch>
```

**Corrected teardown sequence** (now implemented and documented):
```
git worktree remove --force <path>
git worktree prune
git branch -d <branch>
```

**Empirical basis:** sandbox reproduction on git 2.54.0.windows.1 proved `git submodule deinit`
is both unnecessary and the sole cause of shared-config corruption. `git worktree remove --force
<path>` alone handles submodules — git's own docs state "Unclean worktrees or ones with submodules
can be removed with --force." `extensions.worktreeConfig` does NOT isolate submodule config;
submodule init/deinit always read and write the shared .git/config regardless of that setting.

**Disproven hypothesis:** "each submodule needs its own worktree" was tested and disproven.
`git -C <submodule> worktree add` works mechanically, but (1) the superproject still detects
submodules structurally via the tracked .gitmodules file plus the submodule's nested .git file, so
the removal refusal still triggers; (2) `git submodule status` reports the submodule as
uninitialized because it was never registered through the normal path; (3) each submodule worktree
would need its own separate teardown, multiplying the problem instead of solving it.

**Fix applied:** commit 4909db52 (fix(tooling): drop harmful submodule deinit from worktree
teardown) applied the corrected sequence across tools/worktree/worktree.sh,
.agents/instructions/commit/branch-worktree.instructions.md, and .agents/agents/orchestrator.agent.md
so all three now agree.

**Validation:** sandbox reproduction of both the original failure mode and the fix; `bash -n`
parse check; `--help`/`list`/`doctor` against the live repository; `--dry-run` proven non-mutating;
the in-worktree guard proven to refuse destructive operations; two real round-trips (`new
test0000 scratch` → `remove test0000-scratch`) against the live repository with the main
checkout's 5 submodules and all 5 `[submodule]` config sections verified intact afterward.
Evidence commit b84506a3 (chore(tickets): record worktree helper protocol fix evidence) recorded
this in the ticket store. Superproject commit c6eb0a3a (memory-api submodule) was pushed to its
own origin before the superproject push, per the "submodules pushed before superproject" rule.


## Follow-up: submodule main-branch integration

`merge <name>` must process every initialized submodule worktree nested below `.worktrees/<name>` before fast-forwarding the superproject. For a nested submodule worktree with a branch, the helper must fast-forward the corresponding main-checkout submodule's local `main` from that branch, then ensure the main-checkout submodule is on `main`. Detached nested submodule worktrees have no branch to integrate and must be ignored. The superproject merge runs only after all required submodule integrations succeed, so the superproject pointer records the resulting submodule `main` commits.

Validation: use an isolated two-repository fixture with a committed submodule feature branch; assert `merge <name>` advances the submodule `main`, leaves the main submodule checkout on `main`, and then fast-forwards the superproject. Confirm a detached nested submodule is skipped and dry-run performs no mutation.


Validation evidence: `.test/default/` validation spec `worktree-submodule-merge` and passing execution `exec-worktree-submodule-merge-20260805`. Reproduce by querying executions for ticket `4ef88dbc-cb39-4724-9f2c-53ab09cf90c5` and validation spec `worktree-submodule-merge`.