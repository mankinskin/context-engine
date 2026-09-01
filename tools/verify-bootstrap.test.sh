#!/usr/bin/env bash
# Focused regression test for verify-bootstrap.sh's store policy (ticket fd5487b4).
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
target="$script_dir/verify-bootstrap.sh"
fail=0

# The required-store policy must match STORE_MARKERS in discovery.rs exactly,
# and must not require the absent `.doc` store.
expected_stores=(.ticket .spec .rule .audit .session .test .feedback)
stores_line="$(grep -m1 '^stores=(' "$target")"
for store in "${expected_stores[@]}"; do
  if [[ "$stores_line" != *"$store"* ]]; then
    printf 'FAIL: required store %s missing from verify-bootstrap.sh policy\n' "$store" >&2
    fail=1
  fi
done
if [[ "$stores_line" == *".doc"* ]]; then
  printf 'FAIL: verify-bootstrap.sh still requires absent .doc store\n' >&2
  fail=1
fi

# The verifier itself must pass in the real baseline repository layout.
if ! bash "$target" --skip-check >/dev/null; then
  printf 'FAIL: bash verify-bootstrap.sh --skip-check did not pass in baseline layout\n' >&2
  fail=1
fi

if ((fail == 0)); then
  printf 'verify-bootstrap.test.sh: all checks passed.\n'
fi
exit "$fail"
