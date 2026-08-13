#!/usr/bin/env bash
# Single entry point for bootstrapping a fresh clone of context-engine.
#
# Runs, in order, every step documented in README.md's Getting Started
# section, reports pass/fail per step, and stops at the first hard failure
# instead of continuing into a broken later step. Each step can also still
# be run standalone (see README.md) for partial/advanced setups.
set -uo pipefail

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
cd "$repo_root"

steps=(
    "Git submodules|bash setup_git.sh"
    "Developer dependencies|./install-deps.sh"
    "Repository tools + MCP servers|./install-tools.sh --mcp"
    "Workspace stores + Copilot CLI surfaces|bash init.sh"
    "Bootstrap verification|bash tools/verify-bootstrap.sh"
)

failed_step=""
for entry in "${steps[@]}"; do
    label=${entry%%|*}
    cmd=${entry#*|}

    printf '\n=== %s ===\n' "$label"
    printf '$ %s\n' "$cmd"

    if ! bash -c "$cmd"; then
        failed_step="$label"
        break
    fi
done

if [[ -n "$failed_step" ]]; then
    printf '\nbootstrap.sh: FAILED at step "%s".\n' "$failed_step" >&2
    printf 'Re-run just that step (see the command printed above) after fixing the\n' >&2
    printf 'reported error, then re-run ./bootstrap.sh to continue from the top —\n' >&2
    printf 'every step here is safe to re-run.\n' >&2
    exit 1
fi

printf '\nbootstrap.sh: all steps passed. context-engine is ready.\n'
