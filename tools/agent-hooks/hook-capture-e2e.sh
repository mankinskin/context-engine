#!/usr/bin/env bash
# End-to-end fixture: trigger real Copilot hooks from a real (headless) Copilot
# CLI session, capture every hook's raw stdin payload, and assert the payloads
# match the documented VS Code Copilot hooks schema.
#
# Reference: https://code.visualstudio.com/docs/agents/reference/hooks-reference
#
# The GitHub Copilot CLI reads the same hook schema as VS Code
# (`.github/hooks/*.json`, see `copilot help config` -> `hooks`), and `-p`
# runs a prompt non-interactively. That combination is what makes an automated
# hook trigger possible without driving the VS Code UI.
#
# Usage:
#   bash tools/agent-hooks/hook-capture-e2e.sh
#
# Requirements:
#   - `copilot` on PATH (GitHub Copilot CLI)
#   - `jq` on PATH
#   - Copilot CLI authentication: one of COPILOT_GITHUB_TOKEN, GH_TOKEN,
#     GITHUB_TOKEN, or a completed `copilot` `/login`.
#
# Exit codes:
#   0  all asserted events fired with schema-valid payloads
#   1  an assertion failed
#   77 skipped (missing prerequisite such as authentication)

set -uo pipefail

skip() {
    echo "SKIP: $*" >&2
    exit 77
}
fail() {
    echo "FAIL: $*" >&2
    exit 1
}

command -v copilot >/dev/null 2>&1 || skip "copilot CLI not on PATH"
command -v jq >/dev/null 2>&1 || skip "jq not on PATH"

workdir=$(mktemp -d)

cleanup() {
    # Capture the triggering exit status before cleanup commands change it.
    local exit_status=$?

    if [[ "${HOOK_E2E_KEEP_WORKDIR:-}" == "1" || "$exit_status" -ne 0 && "$exit_status" -ne 77 ]]; then
        echo "KEEP: preserving diagnostic workdir $workdir (captures and run log kept for inspection)" >&2
    else
        rm -rf "$workdir"
    fi

    return "$exit_status"
}
trap cleanup EXIT

mkdir -p "$workdir/.github/hooks" "$workdir/captures"

# A recorder hook per event. Each writes its raw stdin to captures/<event>.json
# and returns the empty success object so the session is never blocked.
cat > "$workdir/record.sh" <<'RECORDER'
#!/usr/bin/env bash
event=$1
mkdir -p captures
stderr_path="captures/$event.stderr"
exit_path="captures/$event.exit"

exec 3>&2
{
    cat > "captures/$event.json"
    echo '{}'
} 2>"$stderr_path"
hook_status=$?
printf '%s\n' "$hook_status" > "$exit_path"

if [[ -s "$stderr_path" ]]; then
    cat "$stderr_path" >&3
fi

exit "$hook_status"
RECORDER

cat > "$workdir/.github/hooks/hooks.json" <<'HOOKS'
{
  "hooks": {
    "SessionStart": [
      { "type": "command", "command": "bash record.sh SessionStart", "cwd": ".", "timeout": 30 }
    ],
    "UserPromptSubmit": [
      { "type": "command", "command": "bash record.sh UserPromptSubmit", "cwd": ".", "timeout": 30 }
    ],
    "PreToolUse": [
      { "type": "command", "command": "bash record.sh PreToolUse", "cwd": ".", "timeout": 30 }
    ],
    "PostToolUse": [
      { "type": "command", "command": "bash record.sh PostToolUse", "cwd": ".", "timeout": 30 }
    ],
    "Stop": [
      { "type": "command", "command": "bash record.sh Stop", "cwd": ".", "timeout": 30 }
    ]
  }
}
HOOKS

# Repo-level hooks load only when the fixture directory is a Git repository.
(
    cd "$workdir"
    git init --quiet
    git config user.email "hook@example.com"
    git config user.name "hook"
    git commit --quiet --allow-empty -m init
)

# The prompt deliberately forces a tool call so PreToolUse and PostToolUse fire
# in addition to the session-lifecycle events.
prompt='Run the shell command `echo hook-fixture-probe` and then reply with only the word done.'

echo "running headless Copilot session in $workdir ..." >&2
run_log="$workdir/copilot-run.log"
(
    cd "$workdir" && copilot -p "$prompt" --allow-all-tools --no-color -s
) >"$run_log" 2>&1
run_status=$?

# Authentication is the one prerequisite that cannot be probed reliably before
# the run, so it is detected from the run itself and reported as a skip rather
# than as schema drift.
if grep -qi "No authentication information found" "$run_log"; then
    skip "Copilot CLI is not authenticated (set COPILOT_GITHUB_TOKEN/GH_TOKEN/GITHUB_TOKEN, or run 'copilot' and '/login')"
fi

if (( run_status != 0 )); then
    echo "note: copilot exited with status $run_status" >&2
    echo "--- Copilot run log ---" >&2
    cat "$run_log" >&2
    echo >&2
    echo "--- end Copilot run log ---" >&2
fi

# ---------------------------------------------------------------------------
# Assertions
# ---------------------------------------------------------------------------

# Fields every event carries.
common_fields=(timestamp hook_event_name session_id cwd transcript_path)

# Fields each event adds on top of the common set. Events absent from this list
# are not asserted here.
declare -A event_fields=(
    [SessionStart]="source"
    [UserPromptSubmit]="prompt"
    [PreToolUse]="tool_name tool_input tool_use_id"
    [PostToolUse]="tool_name tool_input tool_use_id tool_response"
    [Stop]="stop_hook_active"
)

# Events that must have fired for the fixture to be considered meaningful.
required_events=(SessionStart UserPromptSubmit Stop)

status=0

print_hook_diagnostics() {
    local event=$1
    local exit_record="$workdir/captures/$event.exit"
    local stderr_record="$workdir/captures/$event.stderr"
    local have_diagnostics=0

    if [[ -f "$exit_record" ]]; then
        echo "EXIT: $event hook exited with status $(<"$exit_record")" >&2
        have_diagnostics=1
    fi

    if [[ -s "$stderr_record" ]]; then
        echo "--- $event stderr ---" >&2
        cat "$stderr_record" >&2
        echo >&2
        echo "--- end $event stderr ---" >&2
        have_diagnostics=1
    fi

    if (( have_diagnostics == 0 )); then
        echo "NO-DIAGNOSTICS: $event produced no stderr or exit-code record" >&2
    fi
}

for event in "${!event_fields[@]}"; do
    capture="$workdir/captures/$event.json"

    if [[ ! -s "$capture" ]]; then
        if [[ " ${required_events[*]} " == *" $event "* ]]; then
            echo "MISSING: $event did not fire (or captured no payload)" >&2
            print_hook_diagnostics "$event"
            status=1
        else
            echo "note: $event did not fire in this run" >&2
        fi
        continue
    fi

    if ! jq -e . "$capture" >/dev/null 2>&1; then
        echo "INVALID: $event payload is not valid JSON" >&2
        print_hook_diagnostics "$event"
        status=1
        continue
    fi

    event_failed=0
    actual_event=$(jq -r '.hook_event_name // .hookEventName // ""' "$capture")
    if [[ "$actual_event" != "$event" ]]; then
        echo "MISMATCH: $event payload reports hook_event_name='$actual_event'" >&2
        status=1
        event_failed=1
    fi

    missing=()
    for field in "${common_fields[@]}" ${event_fields[$event]}; do
        camel=$(printf '%s' "$field" | awk -F_ '{printf "%s", $1; for (i=2;i<=NF;i++) printf "%s%s", toupper(substr($i,1,1)), substr($i,2)}')
        if ! jq -e --arg a "$field" --arg b "$camel" 'has($a) or has($b)' "$capture" >/dev/null 2>&1; then
            missing+=("$field")
        fi
    done

    if (( ${#missing[@]} > 0 )); then
        echo "DRIFT: $event is missing documented field(s): ${missing[*]}" >&2
        echo "       observed keys: $(jq -r 'keys | join(", ")' "$capture")" >&2
        status=1
        event_failed=1
    fi

    if (( event_failed == 1 )); then
        print_hook_diagnostics "$event"
    else
        echo "OK: $event ($(jq -r 'keys | length' "$capture") keys)" >&2
    fi

    # Preserve the observed payload shape (keys only, no values) so schema
    # drift is reviewable without leaking prompt or tool content.
    out_dir="${HOOK_CAPTURE_SCHEMA_DIR:-}"
    if [[ -n "$out_dir" ]]; then
        mkdir -p "$out_dir"
        jq -S 'keys' "$capture" > "$out_dir/$event.keys.json"
    fi
done

if (( status == 0 )); then
    echo "PASS: captured hook payloads match the documented schema" >&2
fi

exit "$status"
