#!/usr/bin/env bash

set -Eeuo pipefail

SCRIPT_NAME="$(basename "$0")"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
SOURCE_REPO_DEFAULT="$(cd -- "${SCRIPT_DIR}/.." && pwd -P)"
TARGET_PARENT_DEFAULT="$(cd -- "${SOURCE_REPO_DEFAULT}/.." && pwd -P)"

CURRENT_STEP="startup"
DRY_RUN=false
REPLACE_EXISTING=false
TARGET_PARENT="$TARGET_PARENT_DEFAULT"
SOURCE_REPO="$SOURCE_REPO_DEFAULT"
LOG_ROOT=""

declare -a REQUESTED_PROJECTS=()

timestamp() {
  date '+%Y-%m-%d %H:%M:%S'
}

log() {
  printf '[%s] %s\n' "$(timestamp)" "$*"
}

die() {
  log "ERROR: $*"
  exit 1
}

on_error() {
  local exit_code=$?
  log "FAILED: ${CURRENT_STEP} (exit ${exit_code})"
  exit "${exit_code}"
}

trap on_error ERR

format_command() {
  local part
  for part in "$@"; do
    printf '%q ' "$part"
  done
}

start_step() {
  CURRENT_STEP="$1"
  log "START: ${CURRENT_STEP}"
}

finish_step() {
  log "DONE: ${CURRENT_STEP}"
  CURRENT_STEP="idle"
}

run_cmd() {
  local description="$1"
  shift

  start_step "$description"
  log "COMMAND: $(format_command "$@")"
  if [[ "$DRY_RUN" == true ]]; then
    log "DRY RUN: command not executed"
  else
    "$@"
  fi
  finish_step
}

run_cmd_to_log() {
  local description="$1"
  local log_file="$2"
  shift 2

  start_step "$description"
  log "COMMAND : $(format_command "$@")"
  log "LOG FILE: ${log_file}"
  if [[ "$DRY_RUN" == true ]]; then
    log "DRY RUN: command not executed"
  else
    mkdir -p -- "$(dirname -- "$log_file")"
    "$@" >"$log_file" 2>&1
  fi
  finish_step
}

usage() {
  cat <<EOF
Usage: ${SCRIPT_NAME} [options]

Create history-filtered sibling repositories for dedicated tool splits.

Options:
  --project <name>       Extract only one project. Repeatable.
                         Valid names: memory-api, viewer-api, memory-viewers
  --source-repo <path>   Source repository to clone and filter.
                         Default: ${SOURCE_REPO_DEFAULT}
  --target-parent <path> Parent directory where sibling repos are created.
                         Default: ${TARGET_PARENT_DEFAULT}
  --replace              Delete an existing destination directory before cloning.
  --dry-run              Print every step without executing git commands.
  --help                 Show this help text.

Examples:
  ${SCRIPT_NAME} --dry-run
  ${SCRIPT_NAME} --project viewer-api --replace
  ${SCRIPT_NAME} --target-parent /c/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app --replace
EOF
}

canonical_dir() {
  local dir="$1"
  [[ -d "$dir" ]] || die "Directory does not exist: $dir"
  (
    cd -- "$dir"
    pwd -P
  )
}

validate_project() {
  case "$1" in
    memory-api|viewer-api|memory-viewers) ;;
    *) die "Unknown project: $1" ;;
  esac
}

project_keep_regex() {
  local project="$1"
  local common='\.gitignore|\.gitattributes|AGENTS\.md|\.github/README\.md|\.github/copilot-instructions\.md|\.github/COPILOT_INSTRUCTIONS_GUIDE\.md|\.github/agents(/|$)'

  case "$project" in
    memory-api)
      printf '%s' "(${common}|\\.github/instructions/(audit\.instructions\.md|mcp-tools\.instructions\.md|tests\.instructions\.md|ticket-system\.instructions\.md)|crates/(audit-api|memory-api|spec-api|ticket-api)(/|$)|tools/cli/(audit-cli|spec-cli|ticket-cli)(/|$)|tools/http/(spec-http|ticket-http)(/|$)|tools/mcp/(audit-mcp|spec-mcp|ticket-mcp)(/|$)|tools/ticket-vscode(/|$))"
      ;;
    viewer-api)
      printf '%s' "(${common}|viewer-ctl\.toml|\\.github/instructions/(frontend\.instructions\.md|tests\.instructions\.md|viewer-api-tools\.instructions\.md)|tools/viewer/(viewer-api|viewer-ctl)(/|$))"
      ;;
    memory-viewers)
      printf '%s' "(${common}|\\.github/instructions/(frontend\.instructions\.md|tests\.instructions\.md|viewer-api-tools\.instructions\.md|ticket-system\.instructions\.md)|tools/viewer/(ticket-viewer|spec-viewer)(/|$))"
      ;;
    *)
      die "No keep-regex defined for project: $project"
      ;;
  esac
}

default_projects() {
  printf '%s\n' memory-api viewer-api memory-viewers
}

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --project)
        [[ $# -ge 2 ]] || die "--project requires a value"
        validate_project "$2"
        REQUESTED_PROJECTS+=("$2")
        shift 2
        ;;
      --source-repo)
        [[ $# -ge 2 ]] || die "--source-repo requires a value"
        SOURCE_REPO="$2"
        shift 2
        ;;
      --target-parent)
        [[ $# -ge 2 ]] || die "--target-parent requires a value"
        TARGET_PARENT="$2"
        shift 2
        ;;
      --replace)
        REPLACE_EXISTING=true
        shift
        ;;
      --dry-run)
        DRY_RUN=true
        shift
        ;;
      --help)
        usage
        exit 0
        ;;
      *)
        die "Unknown argument: $1"
        ;;
    esac
  done

  if [[ ${#REQUESTED_PROJECTS[@]} -eq 0 ]]; then
    mapfile -t REQUESTED_PROJECTS < <(default_projects)
  fi
}

report_configuration() {
  start_step "Report configuration"
  log "Source repo   : ${SOURCE_REPO}"
  log "Target parent : ${TARGET_PARENT}"
  log "Replace dests : ${REPLACE_EXISTING}"
  log "Dry run       : ${DRY_RUN}"
  log "Projects      : ${REQUESTED_PROJECTS[*]}"
  finish_step
}

check_prerequisites() {
  run_cmd "Verify git is installed" git --version
  if [[ "$DRY_RUN" == true ]]; then
    return
  fi

  start_step "Verify git filter-branch is available"
  git filter-branch -h >/dev/null 2>&1
  finish_step
}

assert_source_repo() {
  SOURCE_REPO="$(canonical_dir "$SOURCE_REPO")"
  TARGET_PARENT="$(canonical_dir "$TARGET_PARENT")"
  LOG_ROOT="${TARGET_PARENT}/.extract-logs"

  start_step "Verify source repository"
  git -C "$SOURCE_REPO" rev-parse --is-inside-work-tree >/dev/null 2>&1 \
    || die "Source is not a git repository: ${SOURCE_REPO}"
  finish_step
}

prepare_log_root() {
  start_step "Prepare extraction log directory"
  log "Log root: ${LOG_ROOT}"
  if [[ "$DRY_RUN" == true ]]; then
    log "DRY RUN: directory not created"
  else
    mkdir -p -- "$LOG_ROOT"
  fi
  finish_step
}

ensure_destination_state() {
  local dest_repo="$1"

  if [[ ! -e "$dest_repo" ]]; then
    return
  fi

  if [[ "$REPLACE_EXISTING" != true ]]; then
    die "Destination already exists: ${dest_repo}. Re-run with --replace to delete it first."
  fi

  run_cmd "Remove existing destination ${dest_repo}" rm -rf "$dest_repo"
}

filter_project_repo() {
  local project="$1"
  local dest_repo="$2"
  local keep_regex="$3"
  local log_file="${LOG_ROOT}/${project}/filter-branch.log"

  local index_filter
  index_filter="git ls-files -z | grep -zvE '${keep_regex}' | xargs -0r git rm -r --cached --ignore-unmatch --"

  if [[ "$DRY_RUN" == true ]]; then
    start_step "Rewrite history for ${project}"
    log "WORKTREE: ${dest_repo}"
    log "KEEP REGEX: ${keep_regex}"
    log "LOG FILE: ${log_file}"
    log "COMMAND: $(format_command git filter-branch -f --prune-empty --index-filter "$index_filter" --tag-name-filter cat -- --all)"
    log "DRY RUN: command not executed"
    finish_step
    return
  fi

  start_step "Rewrite history for ${project}"
  log "WORKTREE: ${dest_repo}"
  log "KEEP REGEX: ${keep_regex}"
  log "LOG FILE: ${log_file}"
  log "COMMAND : $(format_command git filter-branch -f --prune-empty --index-filter "$index_filter" --tag-name-filter cat -- --all)"
  mkdir -p -- "$(dirname -- "$log_file")"
  (
    cd -- "$dest_repo"
    FILTER_BRANCH_SQUELCH_WARNING=1 git filter-branch \
      -f \
      --prune-empty \
      --index-filter "$index_filter" \
      --tag-name-filter cat \
      -- --all
  ) >"$log_file" 2>&1
  finish_step
}

cleanup_project_repo() {
  local project="$1"
  local dest_repo="$2"

  if [[ "$DRY_RUN" == true ]]; then
    start_step "Clean git rewrite artifacts for ${project}"
    log "WORKTREE: ${dest_repo}"
    log "COMMAND: rm -rf .git/refs/original .git/logs/refs/original .git-rewrite"
    log "COMMAND: git reflog expire --expire=now --all"
    log "COMMAND: git gc --prune=now"
    log "COMMAND: git remote remove origin"
    log "DRY RUN: commands not executed"
    finish_step
    return
  fi

  start_step "Clean git rewrite artifacts for ${project}"
  (
    cd -- "$dest_repo"
    rm -rf .git/refs/original .git/logs/refs/original .git-rewrite
    git reflog expire --expire=now --all
    git gc --prune=now
    if git remote | grep -qx 'origin'; then
      git remote remove origin
    fi
  )
  finish_step
}

report_project_summary() {
  local project="$1"
  local dest_repo="$2"

  start_step "Report final state for ${project}"
  if [[ "$DRY_RUN" == true ]]; then
    log "DRY RUN: final git summary skipped for ${dest_repo}"
    finish_step
    return
  fi

  (
    cd -- "$dest_repo"
    local commit_count
    local file_count
    local status_output

    commit_count="$(git rev-list --count HEAD)"
    file_count="$(git ls-files | wc -l | tr -d ' ')"
    status_output="$(git status --short)"

    log "Repo path     : ${dest_repo}"
    log "Commit count  : ${commit_count}"
    log "Tracked files : ${file_count}"
    if [[ -n "$status_output" ]]; then
      printf '%s\n' "$status_output"
      die "Destination repo is not clean after extraction: ${dest_repo}"
    fi
    log "Git status    : clean"
  )
  finish_step
}

extract_project() {
  local project="$1"
  local dest_repo="${TARGET_PARENT}/${project}"
  local keep_regex
  local clone_log="${LOG_ROOT}/${project}/clone.log"
  keep_regex="$(project_keep_regex "$project")"

  log "PROJECT: ${project}"
  log "DESTINATION: ${dest_repo}"
  log "KEEP RULES : ${keep_regex}"

  ensure_destination_state "$dest_repo"
  run_cmd_to_log "Clone source repo for ${project}" "$clone_log" git clone --no-local "$SOURCE_REPO" "$dest_repo"
  filter_project_repo "$project" "$dest_repo" "$keep_regex"
  cleanup_project_repo "$project" "$dest_repo"
  report_project_summary "$project" "$dest_repo"
}

main() {
  parse_args "$@"
  report_configuration
  assert_source_repo
  prepare_log_root
  check_prerequisites

  local project
  for project in "${REQUESTED_PROJECTS[@]}"; do
    extract_project "$project"
  done

  log "All requested projects processed."
}

main "$@"