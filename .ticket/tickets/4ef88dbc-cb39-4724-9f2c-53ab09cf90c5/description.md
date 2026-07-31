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