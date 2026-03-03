#!/usr/bin/env bash
# Generate demo log files from Rust test output.
#
# Usage:
#   ./tools/log-viewer/scripts/generate-demo-logs.sh
#
# Run from the repository root (context-engine/).
# Executes the relevant tests, then copies their log output
# from target/test-logs/ into tools/log-viewer/demo-logs/
# with human-friendly names.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$REPO_ROOT"

DEMO_DIR="tools/log-viewer/demo-logs"
LOG_DIR="target/test-logs"

# ── Mapping: test_log_name → demo_log_name ──────────────────────────
# Each entry is "crate|test_name|demo_name".
# The dump_* tests require the --ignored flag.
ENTRIES=(
  # context-trace
  "context-trace|test_graph_snapshot|test_graph_snapshot"

  # context-insert
  "context-insert|four_repeated_atoms_hierarchical|four_repeated_atoms_hierarchical"
  "context-insert|insert_infix1|insert_infix_aby"
  "context-insert|insert_postfix_bc_of_abc|insert_overlap_bc_of_abc"
  "context-insert|insert_pattern1|insert_pattern_aby"
  "context-insert|insert_postfix1|insert_postfix_bcd"

  # context-read
  "context-read|linear_read_cursor_advancement|read_cursor_advancement"
  "context-read|read_infix1|read_infix1"
  "context-read|linear_read_unique_chars|read_linear_abcdefgh"
  "context-read|linear_read_digits|read_linear_digits"
  "context-read|linear_read_no_letter_repeats|read_linear_no_repeats"
  "context-read|read_repeating_known1|read_repeating_known1"
  "context-read|read_sequence1|read_sequence1"
  "context-read|repetition_ab_separated|repetition_ab_separated"
  "context-read|repetition_abab|repetition_abab"

  # context-search  (dump_* tests are #[ignore]d)
  "context-search|dump_ancestor3|search_ancestor_aby"
  "context-search|dump_ancestor2|search_ancestor_byz"
  "context-search|dump_consecutive1|search_consecutive_ghiabc"
  "context-search|dump_long_pattern|search_long_pattern_ababababcdefghi"
)

# Known ignored tests (need --ignored flag)
IGNORED_TESTS="dump_ancestor3 dump_ancestor2 dump_consecutive1 dump_long_pattern"

# ── Collect unique crate+test pairs to run ──────────────────────────
declare -A REGULAR_TESTS   # crate -> space-separated test names
declare -A IGNORED_TESTS_MAP  # crate -> space-separated test names

for entry in "${ENTRIES[@]}"; do
  IFS='|' read -r crate test_name demo_name <<< "$entry"
  if echo "$IGNORED_TESTS" | grep -qw "$test_name"; then
    IGNORED_TESTS_MAP[$crate]+="$test_name "
  else
    REGULAR_TESTS[$crate]+="$test_name "
  fi
done

# ── Run tests per crate ─────────────────────────────────────────────
echo "=== Running tests to generate log files ==="

for crate in $(echo "${!REGULAR_TESTS[@]}" | tr ' ' '\n' | sort -u); do
  tests="${REGULAR_TESTS[$crate]}"
  echo "  cargo test -p $crate [$(echo $tests | wc -w | tr -d ' ') regular tests]..."
  for t in $tests; do
    cargo test -p "$crate" "$t" -- --nocapture 2>/dev/null || {
      echo "    WARNING: test '$t' in $crate failed" >&2
    }
  done
done

for crate in $(echo "${!IGNORED_TESTS_MAP[@]}" | tr ' ' '\n' | sort -u); do
  tests="${IGNORED_TESTS_MAP[$crate]}"
  echo "  cargo test -p $crate [$(echo $tests | wc -w | tr -d ' ') ignored tests]..."
  for t in $tests; do
    cargo test -p "$crate" "$t" -- --ignored --nocapture 2>/dev/null || {
      echo "    WARNING: ignored test '$t' in $crate failed" >&2
    }
  done
done

# ── Copy logs ────────────────────────────────────────────────────────
echo ""
echo "=== Copying logs to $DEMO_DIR ==="
mkdir -p "$DEMO_DIR"

ok=0
fail=0
for entry in "${ENTRIES[@]}"; do
  IFS='|' read -r crate test_name demo_name <<< "$entry"
  src="$LOG_DIR/${test_name}.log"
  dst="$DEMO_DIR/${demo_name}.log"
  if [[ -f "$src" ]]; then
    cp "$src" "$dst"
    echo "  ✓ $test_name -> $demo_name.log"
    ok=$((ok + 1))
  else
    echo "  ✗ MISSING: $src" >&2
    fail=$((fail + 1))
  fi
done

echo ""
echo "Done: $ok copied, $fail missing (out of ${#ENTRIES[@]} total)"
[[ $fail -eq 0 ]] || exit 1
