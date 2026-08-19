#!/bin/bash
set -euo pipefail
set -x

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
cd "$repo_root"

# Keep index normalization independent of machine-wide Windows Git defaults.
git config core.autocrlf false
git config core.eol lf
git config core.safecrlf warn
git config merge.renormalize true
git config core.hooksPath .githooks

# Reconcile session.json/transcript.json conflicts between the main-checkout
# registry mirror and each session's own worktree branch (see .gitattributes)
# instead of leaving textual conflict markers. Shared across worktrees since
# they use the same repo config.
git config merge.session-record.name "context-engine session record merge"
git config merge.session-record.driver \
    "cargo run --manifest-path \"$repo_root/Cargo.toml\" --quiet -p session-record-merge -- %O %A %B %P"

# `git submodule update --init --recursive` aborts the whole command the
# instant one submodule fails (e.g. a stale/unreachable pinned commit), and
# a failure partway through can also leave a later submodule with an
# incomplete/corrupted checkout. Retry once, then repair any submodule left
# with a fully-deleted working tree, so a single transient submodule
# problem does not require the user to hand-diagnose git internals.
if ! git submodule update --init --recursive; then
    echo "setup_git.sh: initial submodule update failed; retrying once after a fetch" >&2
    git submodule foreach --recursive 'git fetch origin || true'
    git submodule update --init --recursive
fi

bash tools/repair-submodule-worktrees.sh

bash tools/checkout-submodule-branches.sh