#!/bin/bash
set -euo pipefail
set -x

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
cd "$repo_root"

if command -v copilot >/dev/null 2>&1; then
    copilot init
fi

ticket init --index-root "$repo_root/.ticket"
spec init --index-root "$repo_root/.spec"
rule init --index-root "$repo_root/.rule"
mkdir -p "$repo_root/.doc"