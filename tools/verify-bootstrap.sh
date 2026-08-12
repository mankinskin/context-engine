#!/usr/bin/env bash
set -euo pipefail

skip_check=false
case $# in
  0) ;;
  1)
    if [[ "$1" != "--skip-check" ]]; then
      printf 'usage: %s [--skip-check]\n' "$0" >&2
      exit 2
    fi
    skip_check=true
    ;;
  *)
    printf 'usage: %s [--skip-check]\n' "$0" >&2
    exit 2
    ;;
esac

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cargo_bin_dir="${CARGO_HOME:-$HOME/.cargo}/bin"
if [[ ! -f "$cargo_bin_dir/audit" && ! -f "$cargo_bin_dir/audit.exe" ]]; then
  IFS=: read -r -a path_entries <<< "$PATH"
  for path_entry in "${path_entries[@]}"; do
    if [[ -f "$path_entry/audit" || -f "$path_entry/audit.exe" ]]; then
      cargo_bin_dir="$path_entry"
      break
    fi
  done
fi
binaries=(
  audit compact-terminal compact-terminal-mcp context-cli context-mcp doc-viewer
  feedback feedback-mcp fs fs-mcp install-ctl log-viewer mcp-toolmon peek
  peek-mcp rule rule-mcp session session-capture-hook session-mcp spec spec-mcp
  spec-viewer test test-mcp ticket ticket-mcp ticket-viewer worktree-ctl
)
stores=(.ticket .spec .rule .audit .session .test .feedback .doc)
missing_binaries=()
missing_stores=()
cargo_check_failed=false

cargo_binary_exists() {
  [[ -f "$cargo_bin_dir/$1" || -f "$cargo_bin_dir/$1.exe" ]]
}

cargo_binary_path() {
  if [[ -f "$cargo_bin_dir/$1" ]]; then
    printf '%s\n' "$cargo_bin_dir/$1"
  elif [[ -f "$cargo_bin_dir/$1.exe" ]]; then
    printf '%s\n' "$cargo_bin_dir/$1.exe"
  else
    return 1
  fi
}

for binary in "${binaries[@]}"; do
  if [[ "$binary" == "test" ]]; then
    cargo_binary_exists "$binary" || missing_binaries+=("$binary")
  elif ! command -v "$binary" >/dev/null && ! cargo_binary_exists "$binary"; then
    missing_binaries+=("$binary")
  fi
done

for store in "${stores[@]}"; do
  [[ -d "$repo_root/$store" ]] || missing_stores+=("$store")
done

if ! "$skip_check"; then
  if cargo_command="$(cargo_binary_path cargo)"; then
    :
  elif command -v cargo >/dev/null; then
    cargo_command=cargo
  else
    cargo_check_failed=true
  fi
  if ! "$cargo_check_failed" && ! (cd "$repo_root" && "$cargo_command" check --workspace); then
    cargo_check_failed=true
  fi
fi

if ((${#missing_binaries[@]} > 0 || ${#missing_stores[@]} > 0)) || "$cargo_check_failed"; then
  printf 'Bootstrap verification failed.\n' >&2
  if ((${#missing_binaries[@]} > 0)); then
    printf 'Missing binaries: %s\n' "${missing_binaries[*]}" >&2
    printf 'Install them with: ./install-tools.sh --mcp\n' >&2
  fi
  if ((${#missing_stores[@]} > 0)); then
    printf 'Missing domain stores: %s\n' "${missing_stores[*]}" >&2
    printf 'Initialize them with: bash init.sh\n' >&2
  fi
  if "$cargo_check_failed"; then
    printf 'Workspace compile failed: cargo check --workspace\n' >&2
  fi
  exit 1
fi

printf 'Bootstrap verification passed.\n'