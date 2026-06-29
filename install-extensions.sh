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

declare -A extension_expected_file=()
extension_expected_file[ticket-vscode]="out/api.js"

declare -A extension_install_file=()
extension_install_file[ticket-vscode]="out/api.js"

declare -A extension_id=()
extension_id[ticket-vscode]="context-engine.ticket-viewer"

extension_path() {
    case "$1" in
        ticket-vscode) printf '%s\n' "memory-api/tools/ticket-vscode" ;;
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
skipped_extensions=()
dry_run=0

sha256_of_file() {
    local path=$1
    if [[ ! -f "$path" ]]; then
        return 1
    fi

    sha256sum "$path" | awk '{print $1}'
}

resolve_installed_extension_dir() {
    local extension=$1
    local publisher_and_name=${extension_id[$extension]:-}
    local extensions_root=${HOME}/.vscode/extensions

    if [[ -z "$publisher_and_name" ]]; then
        return 1
    fi

    if [[ ! -d "$extensions_root" ]]; then
        return 1
    fi

    local matches=()
    local candidate
    for candidate in "$extensions_root"/"$publisher_and_name"-*; do
        [[ -d "$candidate" ]] || continue
        matches+=("$candidate")
    done

    if [[ ${#matches[@]} -eq 0 ]]; then
        return 1
    fi

    printf '%s\n' "${matches[@]}" | sort | tail -n 1
}

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
    expected_rel_file=${extension_expected_file[$extension]:-}
    installed_rel_file=${extension_install_file[$extension]:-}
    expected_file=""
    expected_hash=""
    pre_installed_hash=""
    post_installed_hash=""
    installed_dir=""
    installed_file=""

    if [[ -z "$expected_rel_file" || -z "$installed_rel_file" ]]; then
        printf 'error: missing verification file mapping for extension %s\n' "$extension" >&2
        failed_extensions+=("$extension")
        continue
    fi

    expected_file="$full_dir/$expected_rel_file"

    if [[ ! -f "$expected_file" ]]; then
        printf 'error: expected workspace binary not found for %s: %s\n' "$extension" "$expected_file" >&2
        failed_extensions+=("$extension")
        continue
    fi

    expected_hash=$(sha256_of_file "$expected_file") || {
        printf 'error: failed to hash expected workspace binary for %s: %s\n' "$extension" "$expected_file" >&2
        failed_extensions+=("$extension")
        continue
    }

    installed_dir=$(resolve_installed_extension_dir "$extension" || true)
    if [[ -n "$installed_dir" ]]; then
        installed_file="$installed_dir/$installed_rel_file"
        if [[ -f "$installed_file" ]]; then
            pre_installed_hash=$(sha256_of_file "$installed_file" || true)
        fi
    fi

    if [[ ! -d "$full_dir" ]]; then
        printf 'error: extension directory not found: %s\n' "$full_dir" >&2
        failed_extensions+=("$extension")
        continue
    fi

    printf '==> %s (%s)\n' "$extension" "$extension_dir"

    if [[ -n "$pre_installed_hash" && "$pre_installed_hash" == "$expected_hash" ]]; then
        printf '    installed binary already matches workspace build (%s); skipping install\n' "$expected_rel_file"
        skipped_extensions+=("$extension")
        continue
    fi

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
        installed_dir=$(resolve_installed_extension_dir "$extension" || true)
        if [[ -z "$installed_dir" ]]; then
            printf 'error: could not locate installed extension directory for %s after install\n' "$extension" >&2
            failed=1
        else
            installed_file="$installed_dir/$installed_rel_file"
            if [[ ! -f "$installed_file" ]]; then
                printf 'error: installed binary missing after install for %s: %s\n' "$extension" "$installed_file" >&2
                failed=1
            else
                post_installed_hash=$(sha256_of_file "$installed_file" || true)
                if [[ -z "$post_installed_hash" ]]; then
                    printf 'error: failed to hash installed binary for %s: %s\n' "$extension" "$installed_file" >&2
                    failed=1
                elif [[ "$post_installed_hash" != "$expected_hash" ]]; then
                    printf 'error: installed binary hash mismatch for %s\n' "$extension" >&2
                    printf '       expected (%s): %s\n' "$expected_rel_file" "$expected_hash" >&2
                    printf '       installed(%s): %s\n' "$installed_rel_file" "$post_installed_hash" >&2
                    failed=1
                else
                    printf '    verified installed binary hash matches workspace build (%s)\n' "$expected_rel_file"
                fi
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

if [[ ${#skipped_extensions[@]} -gt 0 ]]; then
    printf 'installer: skipped %d extension(s); binary already current:\n' "${#skipped_extensions[@]}"
    for extension in "${skipped_extensions[@]}"; do
        printf '  - %s\n' "$extension"
    done
fi

if ! installer_print_summary "${#selected_extensions[@]}" installed_extensions failed_extensions "./install-extensions.sh" "./install-extensions.sh --help"; then
    exit 1
fi