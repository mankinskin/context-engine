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

extension_names=(
    ticket-vscode
)

extension_path() {
    case "$1" in
        ticket-vscode) printf '%s\n' "memory-viewers/memory-api/tools/ticket-vscode" ;;
        *)
            printf 'error: unknown extension: %s\n' "$1" >&2
            exit 1
            ;;
    esac
}

usage() {
    cat <<'EOF'
Usage: ./install-extensions.sh [options] [extension ...]

Package and install selected workspace VS Code extensions.

Each selected extension runs its repo-local npm install script. If the
extension's node_modules directory is missing, this helper first runs `npm ci`
in that package directory.

Options:
  --extension <name>    Install one extension; repeatable.
  --extensions <a,b,c> Install a comma-separated list of extensions.
  --all                Install all supported extensions.
  --list               Print supported extensions and exit.
  --dry-run            Print the npm commands without running them.
  -h, --help           Show this help text.

Supported extensions:
  ticket-vscode

Environment:
  INSTALL_EXTENSIONS   Comma-separated extension list used when none are passed.

Examples:
  ./install-extensions.sh
  ./install-extensions.sh --extension ticket-vscode
  ./install-extensions.sh --dry-run
  INSTALL_EXTENSIONS="ticket-vscode" ./install-extensions.sh
EOF
}

print_supported_extensions() {
    local extension

    for extension in "${extension_names[@]}"; do
        printf '%s\n' "$extension"
    done
}

contains_extension() {
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

append_extension() {
    local extension=$1

    if ! contains_extension "$extension" "${extension_names[@]}"; then
        printf 'error: unsupported extension: %s\n' "$extension" >&2
        printf 'supported extensions:\n' >&2
        print_supported_extensions >&2
        exit 1
    fi

    if ! contains_extension "$extension" "${selected_extensions[@]}"; then
        selected_extensions+=("$extension")
    fi
}

append_csv_extensions() {
    local csv=$1
    local item

    IFS=',' read -r -a csv_extensions <<< "$csv"
    for item in "${csv_extensions[@]}"; do
        item=${item//[[:space:]]/}
        [[ -n "$item" ]] || continue
        append_extension "$item"
    done
}

selected_extensions=()
installed_extensions=()
failed_extensions=()
dry_run=0

while [[ $# -gt 0 ]]; do
    case "$1" in
        --extension)
            [[ $# -ge 2 ]] || {
                printf 'error: --extension requires a value\n' >&2
                exit 1
            }
            append_extension "$2"
            shift 2
            ;;
        --extensions)
            [[ $# -ge 2 ]] || {
                printf 'error: --extensions requires a value\n' >&2
                exit 1
            }
            append_csv_extensions "$2"
            shift 2
            ;;
        --all)
            selected_extensions=()
            append_csv_extensions "ticket-vscode"
            shift
            ;;
        --list)
            print_supported_extensions
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
                append_extension "$1"
                shift
            done
            break
            ;;
        -*)
            printf 'error: unknown option: %s\n' "$1" >&2
            exit 1
            ;;
        *)
            append_extension "$1"
            shift
            ;;
    esac
done

if [[ ${#selected_extensions[@]} -eq 0 && -n "${INSTALL_EXTENSIONS:-}" ]]; then
    append_csv_extensions "$INSTALL_EXTENSIONS"
fi

if [[ ${#selected_extensions[@]} -eq 0 ]]; then
    append_csv_extensions "ticket-vscode"
fi

for extension in "${selected_extensions[@]}"; do
    extension_dir=$(extension_path "$extension")
    full_dir="$repo_root/$extension_dir"
    failed=0

    if [[ ! -d "$full_dir" ]]; then
        printf 'error: extension directory not found: %s\n' "$full_dir" >&2
        failed_extensions+=("$extension")
        continue
    fi

    printf '==> %s (%s)\n' "$extension" "$extension_dir"

    if [[ ! -d "$full_dir/node_modules" ]]; then
        if [[ $dry_run -eq 1 ]]; then
            printf '    (cd %q && npm ci)\n' "$full_dir"
        else
            printf '    npm ci\n'
            if ! (
                cd "$full_dir"
                run_filtered_command "$extension (npm ci)" npm ci
            ); then
                failed=1
            fi
        fi
    fi

    if [[ $failed -eq 0 ]]; then
        if [[ $dry_run -eq 1 ]]; then
            printf '    (cd %q && npm run install:vsix:dry-run)\n' "$full_dir"
        else
            printf '    npm run install:vsix\n'
            if ! (
                cd "$full_dir"
                run_filtered_command "$extension (install)" npm run install:vsix
            ); then
                failed=1
            fi
        fi
    fi

    if [[ $failed -eq 0 ]]; then
        installed_extensions+=("$extension")
    else
        failed_extensions+=("$extension")
        printf 'error: install failed for extension %s\n' "$extension" >&2
    fi
done

if ! installer_print_summary "${#selected_extensions[@]}" installed_extensions failed_extensions "./install-extensions.sh" "./install-extensions.sh --help"; then
    exit 1
fi