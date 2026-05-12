#!/usr/bin/env bash
set -euo pipefail

readonly CONTRACT_SLUG="memory-api/install-contracts/cli-and-viewer-installation"

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd -- "$script_dir/../.." && pwd)
work_root=/tmp/context-engine-viewer-install-validation
install_cargo_home=$work_root/cargo-home
install_home=$work_root/home

log_step() {
    printf '\n[%s] %s\n' "$1" "$2"
}

fail() {
    printf 'error: %s\n' "$*" >&2
    exit 1
}

find_install_contract_dir() {
    local dir

    for dir in "$repo_root/memory-viewers/memory-api/.spec/specs"/*; do
        [[ -f "$dir/spec.toml" ]] || continue
        if grep -Fq "slug = \"$CONTRACT_SLUG\"" "$dir/spec.toml"; then
            printf '%s\n' "$dir"
            return 0
        fi
    done

    fail "could not find install contract spec for $CONTRACT_SLUG"
}

contract_dir=$(find_install_contract_dir)
viewer_matrix_path="$contract_dir/sections/viewer-install-boundary.md"

scenario_row() {
    local scenario_id=$1
    local row

    row=$(grep -F "| $scenario_id |" "$viewer_matrix_path" | head -n 1 || true)
    [[ -n "$row" ]] || fail "missing scenario row for $scenario_id"
    printf '%s\n' "$row"
}

scenario_commands_cell() {
    local scenario_id=$1

    scenario_row "$scenario_id" | cut -d'|' -f5
}

extract_backtick_tokens() {
    local text=$1

    while [[ "$text" =~ \`([^\`]*)\` ]]; do
        printf '%s\n' "${BASH_REMATCH[1]}"
        text=${text#*\`}
        text=${text#*\`}
    done
}

scenario_commands() {
    extract_backtick_tokens "$(scenario_commands_cell "$1")"
}

prepare_install_env() {
    rm -rf "$work_root"
    mkdir -p "$install_cargo_home/bin" "$install_home"
    export CARGO_HOME="$install_cargo_home"
    export HOME="$install_home"
    export PATH="$install_cargo_home/bin:$PATH"
}

run_command() {
    local workdir=$1
    local command=$2

    printf '  $ %s\n' "$command"
    (
        cd "$workdir"
        bash -c "$command"
    )
}

assert_viewer_ctl_installed() {
    [[ -x "$install_cargo_home/bin/viewer-ctl" ]] \
        || fail "missing installed tool: viewer-ctl"
    "$install_cargo_home/bin/viewer-ctl" --help >/dev/null
    "$install_cargo_home/bin/viewer-ctl" list >/dev/null
}

assert_viewer_server_installed() {
    local viewer_name=$1

    [[ -x "$install_cargo_home/bin/$viewer_name" ]] \
        || fail "missing installed server binary: $viewer_name"
}

assert_viewer_frontend_installed() {
    local viewer_name=$1
    local static_root=$install_home/.context-engine/static/$viewer_name

    [[ -f "$static_root/index.html" ]] \
        || fail "missing installed frontend bundle: $static_root/index.html"
}

run_view_01() {
    local command

    log_step VIEW-01 "install viewer-ctl into an isolated Cargo home"
    while IFS= read -r command; do
        [[ -n "$command" ]] || continue
        run_command "$repo_root" "$command"
    done < <(scenario_commands VIEW-01)
    assert_viewer_ctl_installed
}

run_view_02() {
    local command
    local viewer_name

    log_step VIEW-02 "install all managed viewer server/frontend artifacts"
    while IFS= read -r command; do
        [[ -n "$command" ]] || continue
        run_command "$repo_root" "$command"
    done < <(scenario_commands VIEW-02)

    for viewer_name in doc-viewer log-viewer ticket-viewer spec-viewer; do
        assert_viewer_server_installed "$viewer_name"
        assert_viewer_frontend_installed "$viewer_name"
    done

    log_step VIEW-02 "rerun the managed viewer install commands"
    while IFS= read -r command; do
        [[ -n "$command" ]] || continue
        run_command "$repo_root" "$command"
    done < <(scenario_commands VIEW-02)

    for viewer_name in doc-viewer log-viewer ticket-viewer spec-viewer; do
        assert_viewer_server_installed "$viewer_name"
        assert_viewer_frontend_installed "$viewer_name"
        run_command "$repo_root" "viewer-ctl static-dir $viewer_name | grep -Fx '$install_home/.context-engine/static/$viewer_name'"
    done
}

log_step setup "using $(rustc --version)"
prepare_install_env
run_view_01
run_view_02
log_step done "managed viewer install scenarios passed in Docker"