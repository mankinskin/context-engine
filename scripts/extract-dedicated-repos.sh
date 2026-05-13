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
                         Valid names: memory-api, viewer-api, memory-viewers, context-stack
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
  ${SCRIPT_NAME} --project context-stack --replace
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
    memory-api|viewer-api|memory-viewers|context-stack) ;;
    *) die "Unknown project: $1" ;;
  esac
}

project_keep_regex() {
  local project="$1"
  local common='\.gitignore|\.gitattributes|AGENTS\.md|\.github/README\.md|\.github/copilot-instructions\.md|\.github/COPILOT_INSTRUCTIONS_GUIDE\.md|\.agents/agents(/|$)'

  case "$project" in
    memory-api)
      printf '%s' "(${common}|\\.agents/instructions/(audit\.instructions\.md|mcp-tools\.instructions\.md|tests\.instructions\.md|ticket-system\.instructions\.md)|crates/(audit-api|memory-api|spec-api|ticket-api)(/|$)|tools/cli/(audit-cli|spec-cli|ticket-cli)(/|$)|tools/http/(spec-http|ticket-http)(/|$)|tools/mcp/(audit-mcp|spec-mcp|ticket-mcp)(/|$)|tools/ticket-vscode(/|$))"
      ;;
    viewer-api)
      printf '%s' "(${common}|viewer-ctl\.toml|\\.agents/instructions/(frontend\.instructions\.md|tests\.instructions\.md|viewer-api-tools\.instructions\.md)|tools/viewer/(viewer-api|viewer-ctl)(/|$))"
      ;;
    memory-viewers)
      printf '%s' "(${common}|\\.agents/instructions/(frontend\.instructions\.md|tests\.instructions\.md|viewer-api-tools\.instructions\.md|ticket-system\.instructions\.md)|tools/viewer/(ticket-viewer|spec-viewer)(/|$))"
      ;;
    context-stack)
      printf '%s' '(crates/(context-api|context-insert|context-read|context-search|context-trace|context-trace-macros|ngrams)(/|$)|crates/deps/petgraph(/|$))'
      ;;
    *)
      die "No keep-regex defined for project: $project"
      ;;
  esac
}

context_stack_rewrite_branch() {
  printf '%s' 'context-stack-source'
}

context_stack_branch_root() {
  printf '%s' 'crates/context-stack'
}

context_stack_tree_filter() {
  cat <<'EOF'
git ls-files -s | sed 's#\tcrates/#\tcrates/context-stack/#' | GIT_INDEX_FILE="${GIT_INDEX_FILE}.new" git update-index --index-info && mv "${GIT_INDEX_FILE}.new" "$GIT_INDEX_FILE"
EOF
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

  if [[ "$project" == 'context-stack' ]]; then
    filter_context_stack_repo "$dest_repo" "$keep_regex"
    return
  fi

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

filter_context_stack_repo() {
  local dest_repo="$1"
  local keep_regex="$2"
  local rewrite_branch
  local branch_root
  local initial_log="${LOG_ROOT}/context-stack/filter-branch.log"
  local move_log="${LOG_ROOT}/context-stack/rewrite-branch.log"
  local subdir_log="${LOG_ROOT}/context-stack/subdirectory-filter.log"
  local index_filter
  local tree_filter

  rewrite_branch="$(context_stack_rewrite_branch)"
  branch_root="$(context_stack_branch_root)"
  index_filter="git ls-files -z | grep -zvE '${keep_regex}' | xargs -0r git rm -r --cached --ignore-unmatch --"
  tree_filter="$(context_stack_tree_filter)"

  run_cmd "Create rewrite branch for context-stack" git -C "$dest_repo" checkout -B "$rewrite_branch" HEAD

  if [[ "$DRY_RUN" == true ]]; then
    start_step 'Rewrite history for context-stack'
    log "WORKTREE: ${dest_repo}"
    log "KEEP REGEX: ${keep_regex}"
    log "LOG FILE: ${initial_log}"
    log "COMMAND: $(format_command git -C "$dest_repo" filter-branch -f --prune-empty --index-filter "$index_filter" --tag-name-filter cat -- "$rewrite_branch")"
    log 'DRY RUN: command not executed'
    finish_step

    start_step 'Move context-stack crates into branch layout'
    log "WORKTREE: ${dest_repo}"
    log "BRANCH ROOT: ${branch_root}"
    log "LOG FILE: ${move_log}"
    log "COMMAND: $(format_command git -C "$dest_repo" filter-branch -f --prune-empty --index-filter "$tree_filter" --tag-name-filter cat -- "$rewrite_branch")"
    log 'DRY RUN: command not executed'
    finish_step

    start_step 'Strip context-stack branch root'
    log "WORKTREE: ${dest_repo}"
    log "SUBDIRECTORY: ${branch_root}"
    log "LOG FILE: ${subdir_log}"
    log "COMMAND: $(format_command git -C "$dest_repo" filter-branch -f --prune-empty --subdirectory-filter "$branch_root" --tag-name-filter cat -- "$rewrite_branch")"
    log 'DRY RUN: command not executed'
    finish_step

    run_cmd 'Promote context-stack rewrite branch to main' git -C "$dest_repo" checkout -B main "$rewrite_branch"
    return
  fi

  start_step 'Rewrite history for context-stack'
  log "WORKTREE: ${dest_repo}"
  log "KEEP REGEX: ${keep_regex}"
  log "LOG FILE: ${initial_log}"
  log "COMMAND : $(format_command git filter-branch -f --prune-empty --index-filter "$index_filter" --tag-name-filter cat -- "$rewrite_branch")"
  mkdir -p -- "$(dirname -- "$initial_log")"
  (
    cd -- "$dest_repo"
    FILTER_BRANCH_SQUELCH_WARNING=1 git filter-branch \
      -f \
      --prune-empty \
      --index-filter "$index_filter" \
      --tag-name-filter cat \
      -- "$rewrite_branch"
  ) >"$initial_log" 2>&1
  finish_step

  start_step 'Move context-stack crates into branch layout'
  log "WORKTREE: ${dest_repo}"
  log "BRANCH ROOT: ${branch_root}"
  log "LOG FILE: ${move_log}"
  log "COMMAND : $(format_command git filter-branch -f --prune-empty --index-filter "$tree_filter" --tag-name-filter cat -- "$rewrite_branch")"
  (
    cd -- "$dest_repo"
    FILTER_BRANCH_SQUELCH_WARNING=1 git filter-branch \
      -f \
      --prune-empty \
      --index-filter "$tree_filter" \
      --tag-name-filter cat \
      -- "$rewrite_branch"
  ) >"$move_log" 2>&1
  finish_step

  start_step 'Strip context-stack branch root'
  log "WORKTREE: ${dest_repo}"
  log "SUBDIRECTORY: ${branch_root}"
  log "LOG FILE: ${subdir_log}"
  log "COMMAND : $(format_command git filter-branch -f --prune-empty --subdirectory-filter "$branch_root" --tag-name-filter cat -- "$rewrite_branch")"
  (
    cd -- "$dest_repo"
    FILTER_BRANCH_SQUELCH_WARNING=1 git filter-branch \
      -f \
      --prune-empty \
      --subdirectory-filter "$branch_root" \
      --tag-name-filter cat \
      -- "$rewrite_branch"
  ) >"$subdir_log" 2>&1
  finish_step

  run_cmd 'Promote context-stack rewrite branch to main' git -C "$dest_repo" checkout -B main "$rewrite_branch"
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
    if [[ "$project" == 'context-stack' ]]; then
      while IFS= read -r branch_name; do
        if [[ -n "$branch_name" && "$branch_name" != 'main' ]]; then
          git branch -D "$branch_name"
        fi
      done < <(git for-each-ref --format='%(refname:short)' refs/heads)

      while IFS= read -r tag_name; do
        if [[ -n "$tag_name" ]]; then
          git tag -d "$tag_name"
        fi
      done < <(git tag --list)
    fi
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