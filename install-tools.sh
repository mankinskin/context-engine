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

tool_names=(
    viewer-ctl
    doc-viewer
    log-viewer
    spec-viewer
    ticket-viewer
    copilot-capture-hook
    ticket-cli
    spec-cli
    audit-cli
    rule-cli
)

tool_path() {
    case "$1" in
        viewer-ctl) printf '%s\n' "viewer-api/viewer-ctl" ;;
        doc-viewer) printf '%s\n' "tools/viewer/doc-viewer" ;;
        log-viewer) printf '%s\n' "tools/viewer/log-viewer" ;;
        spec-viewer) printf '%s\n' "memory-viewers/spec-viewer" ;;
        ticket-viewer) printf '%s\n' "memory-viewers/ticket-viewer" ;;
        copilot-capture-hook) printf '%s\n' "memory-api/crates/session-api" ;;
        ticket-cli) printf '%s\n' "memory-api/tools/cli/ticket-cli" ;;
        spec-cli) printf '%s\n' "memory-api/tools/cli/spec-cli" ;;
        audit-cli) printf '%s\n' "memory-api/tools/cli/audit-cli" ;;
        rule-cli) printf '%s\n' "memory-api/tools/cli/rule-cli" ;;
        *)
            printf 'error: unknown tool: %s\n' "$1" >&2
            exit 1
            ;;
    esac
}

tool_bin() {
    case "$1" in
        viewer-ctl) printf '%s\n' "viewer-ctl" ;;
        doc-viewer) printf '%s\n' "doc-viewer" ;;
        log-viewer) printf '%s\n' "log-viewer" ;;
        spec-viewer) printf '%s\n' "spec-viewer" ;;
        ticket-viewer) printf '%s\n' "ticket-viewer" ;;
        copilot-capture-hook) printf '%s\n' "copilot-capture-hook" ;;
        ticket-cli) printf '%s\n' "ticket" ;;
        spec-cli) printf '%s\n' "spec" ;;
        audit-cli) printf '%s\n' "audit" ;;
        rule-cli) printf '%s\n' "rule" ;;
        *)
            printf 'error: unknown tool: %s\n' "$1" >&2
            exit 1
            ;;
    esac
}

usage() {
    cat <<'EOF'
Usage: ./install-tools.sh [options] [tool ...]

Install selected workspace tools with cargo install --path.

This script installs only deliverables built from this repository. External
toolchain dependencies are handled by ./install-deps.sh.

Options:
  --tool <name>       Install one tool; repeatable.
  --tools <a,b,c>     Install a comma-separated list of tools.
  --all               Install all supported tools.
  --list              Print supported tools and exit.
  --dry-run           Print the cargo install commands without running them.
  --no-force          Do not pass --force to cargo install.
  -h, --help          Show this help text.

Supported tools:
  viewer-ctl
  doc-viewer
  log-viewer
  spec-viewer
  ticket-viewer
    copilot-capture-hook
  ticket-cli
  spec-cli
  audit-cli
  rule-cli

Environment:
  INSTALL_TOOLS       Comma-separated tool list used when no tools are passed.

Examples:
  ./install-tools.sh
  ./install-tools.sh spec-cli ticket-cli
  ./install-tools.sh --tool viewer-ctl --tool ticket-cli
  ./install-tools.sh --tool doc-viewer --tool log-viewer --tool spec-viewer --tool ticket-viewer
  ./install-tools.sh --tool audit-cli --tool rule-cli
  INSTALL_TOOLS="rule-cli,spec-cli" ./install-tools.sh --dry-run
EOF
}

print_supported_tools() {
    local tool

    for tool in "${tool_names[@]}"; do
        printf '%s\n' "$tool"
    done
}

contains_tool() {
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

append_tool() {
    local tool=$1

    if ! contains_tool "$tool" "${tool_names[@]}"; then
        printf 'error: unsupported tool: %s\n' "$tool" >&2
        printf 'supported tools:\n' >&2
        print_supported_tools >&2
        exit 1
    fi

    if ! contains_tool "$tool" "${selected_tools[@]}"; then
        selected_tools+=("$tool")
    fi
}

append_csv_tools() {
    local csv=$1
    local item

    IFS=',' read -r -a csv_tools <<< "$csv"
    for item in "${csv_tools[@]}"; do
        item=${item//[[:space:]]/}
        [[ -n "$item" ]] || continue
        append_tool "$item"
    done
}

selected_tools=()
installed_tools=()
failed_tools=()
force_install=1
dry_run=0

while [[ $# -gt 0 ]]; do
    case "$1" in
        --tool)
            [[ $# -ge 2 ]] || {
                printf 'error: --tool requires a value\n' >&2
                exit 1
            }
            append_tool "$2"
            shift 2
            ;;
        --tools)
            [[ $# -ge 2 ]] || {
                printf 'error: --tools requires a value\n' >&2
                exit 1
            }
            append_csv_tools "$2"
            shift 2
            ;;
        --all)
            selected_tools=()
            append_csv_tools "viewer-ctl,doc-viewer,log-viewer,spec-viewer,ticket-viewer,copilot-capture-hook,ticket-cli,spec-cli,audit-cli,rule-cli"
            shift
            ;;
        --list)
            print_supported_tools
            exit 0
            ;;
        --dry-run)
            dry_run=1
            shift
            ;;
        --no-force)
            force_install=0
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        --)
            shift
            while [[ $# -gt 0 ]]; do
                append_tool "$1"
                shift
            done
            break
            ;;
        -*)
            printf 'error: unknown option: %s\n' "$1" >&2
            exit 1
            ;;
        *)
            append_tool "$1"
            shift
            ;;
    esac
done

if [[ ${#selected_tools[@]} -eq 0 && -n "${INSTALL_TOOLS:-}" ]]; then
    append_csv_tools "$INSTALL_TOOLS"
fi

if [[ ${#selected_tools[@]} -eq 0 ]]; then
    append_csv_tools "viewer-ctl,doc-viewer,log-viewer,spec-viewer,ticket-viewer,copilot-capture-hook,ticket-cli,spec-cli,audit-cli,rule-cli"
fi

install_one() {
    local tool=$1
    local path
    local bin
    local command
    local failed=0

    path=$(tool_path "$tool")
    bin=$(tool_bin "$tool")

    command=(cargo install --path "$path" --bin "$bin")

    command+=(--quiet)

    if [[ $force_install -eq 1 ]]; then
        command+=(--force)
    fi

    printf '==> %s\n' "$tool"
    printf '    %s\n' "${command[*]}"

    if [[ $dry_run -eq 1 ]]; then
        installed_tools+=("$tool")
        return 0
    fi

    if ! (
        cd "$repo_root"
        run_filtered_command "$tool" "${command[@]}"
    ); then
        failed=1
    fi

    if [[ $failed -eq 0 ]]; then
        installed_tools+=("$tool")
        return 0
    fi

    failed_tools+=("$tool")
    printf 'error: install failed for %s\n' "$tool" >&2
    return 1
}

for tool in "${selected_tools[@]}"; do
    install_one "$tool" || true
done

retry_prefix="./install-tools.sh"
if [[ $force_install -eq 0 ]]; then
    retry_prefix="$retry_prefix --no-force"
fi

if ! installer_print_summary "${#selected_tools[@]}" installed_tools failed_tools "$retry_prefix" "./install-tools.sh --help"; then
    exit 1
fi