#!/usr/bin/env bash

run_filtered_command() {
    local label=$1
    shift
    local status_file
    local last_stdout_line=""
    local saw_stdout=0
    local command_status=1
    local interactive=0

    if [[ -t 1 ]]; then
        interactive=1
    fi

    status_file=$(mktemp)

    while IFS= read -r line; do
        saw_stdout=1
        last_stdout_line=$line

        if [[ $interactive -eq 1 ]]; then
            printf '\r\033[2K    %s: %s' "$label" "$line"
        fi
    done < <(
        (
            "$@"
            printf '%s\n' "$?" > "$status_file"
        ) 2> >(cat >&2)
    )

    if [[ -s "$status_file" ]]; then
        command_status=$(<"$status_file")
    fi

    rm -f "$status_file"

    if [[ $saw_stdout -eq 1 ]]; then
        if [[ $interactive -eq 1 ]]; then
            printf '\r\033[2K    %s: %s\n' "$label" "$last_stdout_line"
        else
            printf '    %s: %s\n' "$label" "$last_stdout_line"
        fi
    fi

    return "$command_status"
}

print_joined_items() {
    local first=1
    local item

    for item in "$@"; do
        if [[ $first -eq 1 ]]; then
            printf '%s' "$item"
            first=0
        else
            printf ', %s' "$item"
        fi
    done
}

installer_print_summary() {
    local requested_count=$1
    local succeeded_name=$2
    local failed_name=$3
    local retry_prefix=${4:-}
    local help_command=${5:-}
    local -n succeeded_ref=$succeeded_name
    local -n failed_ref=$failed_name

    printf '\nInstall summary: requested=%d, succeeded=%d, failed=%d\n' \
        "$requested_count" "${#succeeded_ref[@]}" "${#failed_ref[@]}"

    if [[ ${#succeeded_ref[@]} -gt 0 ]]; then
        printf 'Succeeded: '
        print_joined_items "${succeeded_ref[@]}"
        printf '\n'
    fi

    if [[ ${#failed_ref[@]} -gt 0 ]]; then
        printf 'Failed: '
        print_joined_items "${failed_ref[@]}"
        printf '\n\n'

        if [[ -n "$retry_prefix" ]]; then
            printf 'Retry failed installs with:\n'
            printf '  %s' "$retry_prefix"
            local item
            for item in "${failed_ref[@]}"; do
                printf ' %q' "$item"
            done
            printf '\n\n'
        fi

        if [[ -n "$help_command" ]]; then
            printf 'For more options, run:\n'
            printf '  %s\n' "$help_command"
        fi

        return 1
    fi

    return 0
}