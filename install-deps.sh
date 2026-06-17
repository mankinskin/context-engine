#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_root=$script_dir

usage() {
    cat <<'EOF'
Usage: ./install-deps.sh [options]

Install repository-wide developer dependencies.

Options:
  --dry-run   Print the cargo install commands without running them.
  -h, --help  Show this help text.

Installs:
  ripgrep
  rtk

Examples:
  ./install-deps.sh
  ./install-deps.sh --dry-run
EOF
}

dry_run=0

while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run)
            dry_run=1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        --)
            shift
            break
            ;;
        -*)
            printf 'error: unknown option: %s\n' "$1" >&2
            exit 1
            ;;
        *)
            printf 'error: unexpected argument: %s\n' "$1" >&2
            exit 1
            ;;
    esac
done

install_one() {
    local label=$1
    shift

    printf '==> %s\n' "$label"
    printf '    %s\n' "$*"

    if [[ $dry_run -eq 1 ]]; then
        return 0
    fi

    (
        cd "$repo_root"
        "$@"
    )
}

install_one "ripgrep" cargo install ripgrep --force
install_one "rtk" cargo install --git https://github.com/rtk-ai/rtk --force
