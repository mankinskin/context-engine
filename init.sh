#!/bin/bash
set -euo pipefail
set -x

if command -v copilot >/dev/null 2>&1; then
    copilot init
fi

ticket init
spec init
rule init
mkdir -p .doc