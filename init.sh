#!/bin/bash
set -euo pipefail
set -x

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
cd "$repo_root"

copilot_instructions="$repo_root/.github/copilot-instructions.md"
# `copilot init` launches a full nested Copilot CLI session that edits this
# file (spending AI credits and taking minutes) every time it runs. Only run
# it when the file is still the minimal placeholder Copilot CLI ships with,
# so re-running init.sh (e.g. via bootstrap.sh) stays fast and idempotent
# once a real repository-specific file has been generated.
if command -v copilot >/dev/null 2>&1; then
    if [[ ! -f "$copilot_instructions" ]] || [[ "$(wc -l < "$copilot_instructions")" -lt 20 ]]; then
        copilot init
    else
        echo "init.sh: skipping 'copilot init' — $copilot_instructions already looks populated (use 'copilot init' manually to refresh it)"
    fi
fi

ticket init --index-root "$repo_root/.ticket"
spec init --index-root "$repo_root/.spec"
rule init --index-root "$repo_root/.rule"
mkdir -p "$repo_root/.doc"

# Regenerate the .github/{agents,prompts,instructions} directories Copilot
# CLI auto-discovers, from the canonical .agents/ sources.
bash "$repo_root/tools/install/sync-copilot-surfaces.sh"