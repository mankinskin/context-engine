#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)
TEST_DIR="$ROOT/tools/worktree/tests"
RESULT_DIR=$(mktemp -d)
trap 'rm -rf "$RESULT_DIR"' EXIT
status=0
declare -a test_names=()
declare -a test_pids=()

for test_file in "$TEST_DIR"/test_*.sh; do
    test_name=$(basename "$test_file" .sh)
    [[ "$test_name" == "test_static" ]] && continue
    bash "$test_file" "$ROOT" >"$RESULT_DIR/$test_name" 2>&1 &
    test_names+=("$test_name")
    test_pids+=("$!")
done

for index in "${!test_names[@]}"; do
    test_name=${test_names[$index]}
    if wait "${test_pids[$index]}"; then
        printf 'PASS %s\n' "$test_name"
    else
        cat "$RESULT_DIR/$test_name" >&2
        printf 'FAIL %s\n' "$test_name"
        status=1
    fi
done

exit "$status"