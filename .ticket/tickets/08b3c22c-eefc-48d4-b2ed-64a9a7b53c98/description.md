# Fresh-clone bootstrap fails: `memory-api` submodule pinned to unreachable commit

## Goal (persisted across sessions)

First-time understanding of this repository: make `context-engine` fully
bootstrap-able from a fresh clone, on any supported OS, with the documented
steps in README.md § Getting Started, producing **zero errors**:

```bash
bash setup_git.sh
./install-deps.sh
./install-tools.sh --mcp
bash init.sh
bash tools/verify-bootstrap.sh
```

## Reproduction (Windows, Git Bash, fresh clone semantics)

1. Superproject `HEAD` (`28d9ca0e`) pins submodule `memory-api` to
   `df2164aa97d0dfbe825a3cfa1a2b71a9ce2a9533` (`git ls-tree HEAD memory-api`).
2. Run `bash setup_git.sh` (documented step 1).
3. `git submodule update --init --recursive` fails:
   ```
   fatal: remote error: upload-pack: not our ref df2164aa97d0dfbe825a3cfa1a2b71a9ce2a9533
   fatal: Fetched in submodule path 'memory-api', but it did not contain
   df2164aa97d0dfbe825a3cfa1a2b71a9ce2a9533. Direct fetching of that commit failed.
   ```
   Exit code 128. Script aborts (`set -euo pipefail`) before reaching
   `tools/checkout-submodule-branches.sh`.
4. Confirmed the pinned commit is unreachable upstream: `git ls-remote origin
   'refs/*'` on `https://github.com/mankinskin/memory-api` returns no ref
   containing `df2164a...`. Remote `main` is at `40f392f9...`. The pinned
   commit was likely orphaned by a force-push/history rewrite on the
   `memory-api` repo after the superproject's gitlink was last bumped.

## Workaround found

Running `bash tools/checkout-submodule-branches.sh` directly (which
fetches+checks out each submodule's tracked `main` branch by name rather than
by the stale pinned SHA) recovers all 5 submodules successfully, including
`memory-api -> main @ 40f392f9`. But this script is never reached because
`setup_git.sh` aborts first, so a user following the README literally is
stuck with a fatal error and no clear next step.

## Fix options

1. Bump the `memory-api` gitlink in the superproject to a currently-reachable
   commit (e.g. current `origin/main` `40f392f9...`) and commit the pointer
   update — this is the same pattern as prior commits like
   `f4ea2ff1 chore(session): bump memory-api pointer for ticket 76c64b34
   closure`.
2. Harden `setup_git.sh` to fall back to `checkout-submodule-branches.sh`
   (or retry by branch name) when `submodule update --init` fails on a
   specific submodule, so a stale/unreachable pin degrades gracefully instead
   of hard-aborting the whole bootstrap.
3. Add a check in `tools/verify-bootstrap.sh` (or a pre-bootstrap check) that
   detects gitlink/remote drift for all submodules proactively.

Recommend doing both (1) as the immediate unblock and (2) as the durable fix,
so this class of failure cannot recur silently for future fresh clones.

## Acceptance

- Fresh clone of `context-engine` + `bash setup_git.sh` completes without a
  fatal git error for all 5 submodules.
- `tools/verify-bootstrap.sh` passes.
- Root cause (stale/orphaned submodule pin) documented and either prevented
  or auto-recovered.
