#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_root=$script_dir
common_install_helpers="$repo_root/tools/install/common.sh"

if [[ ! -f "$common_install_helpers" ]]; then
    printf 'error: missing shared installer helpers: %s\n' "$common_install_helpers" >&2
    exit 1
fi

# shellcheck source=tools/install/common.sh
source "$common_install_helpers"

dependency_names=(
    ripgrep
    rtk
)

print_supported_dependencies() {
    local dependency

    for dependency in "${dependency_names[@]}"; do
        printf '%s\n' "$dependency"
    done
}

contains_dependency() {
    local needle=$1
    shift

    local item
    for item in "$@"; do
        if [[ "$item" == "$needle" ]]; then
            return 0
        fi
    done

    return 1
}

append_dependency() {
    local dependency=$1

    if ! contains_dependency "$dependency" "${dependency_names[@]}"; then
        printf 'error: unsupported dependency: %s\n' "$dependency" >&2
        printf 'supported dependencies:\n' >&2
        print_supported_dependencies >&2
        exit 1
    fi

    if ! contains_dependency "$dependency" "${selected_dependencies[@]}"; then
        selected_dependencies+=("$dependency")
    fi
}

append_csv_dependencies() {
    local csv=$1
    local item

    IFS=',' read -r -a csv_dependencies <<< "$csv"
    for item in "${csv_dependencies[@]}"; do
        item=${item//[[:space:]]/}
        [[ -n "$item" ]] || continue
        append_dependency "$item"
    done
}

usage() {
    cat <<'EOF'
Usage: ./install-deps.sh [options] [dependency ...]

Install selected repository-wide developer dependencies.

Options:
  --dependency <name>      Install one dependency; repeatable.
  --dependencies <a,b,c>   Install a comma-separated list of dependencies.
  --all                    Install all supported dependencies.
  --list                   Print supported dependencies and exit.
  --dry-run                Print the install commands without running them.
  -h, --help               Show this help text.

Supported dependencies:
  ripgrep
  rtk

Environment:
  INSTALL_DEPS             Comma-separated dependency list used when none are passed.

Examples:
  ./install-deps.sh
  ./install-deps.sh ripgrep rtk
  ./install-deps.sh --dependency ripgrep
  ./install-deps.sh --dependencies "ripgrep,rtk"
  ./install-deps.sh --dry-run
EOF
}

selected_dependencies=()
installed_dependencies=()
failed_dependencies=()
dry_run=0

while [[ $# -gt 0 ]]; do
    case "$1" in
        --dependency)
            [[ $# -ge 2 ]] || {
                printf 'error: --dependency requires a value\n' >&2
                exit 1
            }
            append_dependency "$2"
            shift 2
            ;;
        --dependencies)
            [[ $# -ge 2 ]] || {
                printf 'error: --dependencies requires a value\n' >&2
                exit 1
            }
            append_csv_dependencies "$2"
            shift 2
            ;;
        --all)
            selected_dependencies=()
            append_csv_dependencies "ripgrep,rtk"
            shift
            ;;
        --list)
            print_supported_dependencies
            exit 0
            ;;
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
            while [[ $# -gt 0 ]]; do
                append_dependency "$1"
                shift
            done
            break
            ;;
        -*)
            printf 'error: unknown option: %s\n' "$1" >&2
            exit 1
            ;;
        *)
            append_dependency "$1"
            shift
            ;;
    esac
done

if [[ ${#selected_dependencies[@]} -eq 0 && -n "${INSTALL_DEPS:-}" ]]; then
    append_csv_dependencies "$INSTALL_DEPS"
fi

if [[ ${#selected_dependencies[@]} -eq 0 ]]; then
    append_csv_dependencies "ripgrep,rtk"
fi

install_one() {
    local dependency=$1
    local command=()

    case "$dependency" in
        ripgrep)
            command=(cargo install ripgrep --quiet --force)
            ;;
        rtk)
            command=(cargo install --git https://github.com/rtk-ai/rtk --quiet --force)
            ;;
        *)
            printf 'error: unsupported dependency: %s\n' "$dependency" >&2
            failed_dependencies+=("$dependency")
            return 1
            ;;
    esac

    printf '==> %s\n' "$dependency"
    printf '    %s\n' "${command[*]}"

    if [[ $dry_run -eq 1 ]]; then
        installed_dependencies+=("$dependency")
        return 0
    fi

    if (
        cd "$repo_root"
        run_filtered_command "$dependency" "${command[@]}"
    ); then
        installed_dependencies+=("$dependency")
        return 0
    fi

    failed_dependencies+=("$dependency")
    printf 'error: install failed for %s\n' "$dependency" >&2
    return 1
}

for dependency in "${selected_dependencies[@]}"; do
    install_one "$dependency" || true
done

if ! installer_print_summary "${#selected_dependencies[@]}" installed_dependencies failed_dependencies "./install-deps.sh" "./install-deps.sh --help"; then
    exit 1
fi
