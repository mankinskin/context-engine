#!/usr/bin/env bash
# start-viewer.sh — Generic launcher for context-engine viewer tools.
#
# Handles four viewers: doc-viewer, log-viewer, ticket-viewer, spec-viewer.
# For each viewer:
#   1. Detects + kills any process already listening on the viewer's port.
#   2. Builds the frontend artifacts (Vite or Dioxus, auto-detected).
#   3. Launches the cargo-built server in the foreground.
#
# Usage:
#   scripts/start-viewer.sh <viewer> [--no-build] [--check-only] [-- <extra cargo args>]
#
# Environment overrides:
#   PORT       — override the default port for this viewer.
#   NO_BUILD=1 — equivalent to passing --no-build.
#
# Examples:
#   scripts/start-viewer.sh doc-viewer
#   scripts/start-viewer.sh ticket-viewer --no-build
#   PORT=4010 scripts/start-viewer.sh spec-viewer
#   scripts/start-viewer.sh log-viewer -- --static

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# ── Argument parsing ────────────────────────────────────────────────────────
if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <doc-viewer|log-viewer|ticket-viewer|spec-viewer> [--no-build] [--check-only] [-- <extra args>]" >&2
  exit 2
fi

VIEWER="$1"; shift
NO_BUILD="${NO_BUILD:-0}"
CHECK_ONLY=0
EXTRA_ARGS=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --no-build)   NO_BUILD=1; shift ;;
    --check-only) CHECK_ONLY=1; shift ;;
    --)           shift; EXTRA_ARGS+=("$@"); break ;;
    *)            EXTRA_ARGS+=("$1"); shift ;;
  esac
done

# ── Per-viewer configuration ────────────────────────────────────────────────
# Selects the default port, frontend kind (vite|dioxus), and cargo run args.
VIEWER_DIR="$REPO_ROOT/tools/viewer/$VIEWER"
case "$VIEWER" in
  doc-viewer)
    DEFAULT_PORT=3001
    FRONTEND_KIND="vite"
    FRONTEND_DIR="$VIEWER_DIR/frontend"
    STATIC_DIR="$VIEWER_DIR/static"
    CARGO_PKG="doc-viewer"
    ;;
  log-viewer)
    DEFAULT_PORT=3000
    FRONTEND_KIND="vite"
    FRONTEND_DIR="$VIEWER_DIR/frontend"
    STATIC_DIR="$VIEWER_DIR/static"
    CARGO_PKG="log-viewer"
    # log-viewer needs --static to disable the dev-proxy mode by default.
    if [[ ${#EXTRA_ARGS[@]} -eq 0 ]]; then
      EXTRA_ARGS=(--static)
    fi
    ;;
  ticket-viewer)
    DEFAULT_PORT=3002
    FRONTEND_KIND="trunk"
    FRONTEND_DIR="$VIEWER_DIR/frontend/dioxus"
    STATIC_DIR="$VIEWER_DIR/frontend/dioxus/dist"
    CARGO_PKG="ticket-viewer"
    ;;
  spec-viewer)
    DEFAULT_PORT=4002
    FRONTEND_KIND="trunk"
    FRONTEND_DIR="$VIEWER_DIR/frontend/dioxus"
    STATIC_DIR="$VIEWER_DIR/frontend/dioxus/dist"
    CARGO_PKG="spec-viewer"
    ;;
  *)
    echo "[start-viewer] unknown viewer: $VIEWER" >&2
    echo "  expected one of: doc-viewer log-viewer ticket-viewer spec-viewer" >&2
    exit 2
    ;;
esac

PORT="${PORT:-$DEFAULT_PORT}"

log()  { printf '\033[36m[%s]\033[0m %s\n' "$VIEWER" "$*"; }
warn() { printf '\033[33m[%s]\033[0m %s\n' "$VIEWER" "$*"; }
err()  { printf '\033[31m[%s]\033[0m %s\n' "$VIEWER" "$*" >&2; }

# ── Step 1: free the port ───────────────────────────────────────────────────
# Returns the PIDs (one per line) of any process currently listening on $1.
find_listeners_on_port() {
  local port="$1"
  if command -v ss >/dev/null 2>&1; then
    ss -ltnp "sport = :$port" 2>/dev/null \
      | awk -v p=":$port$" '$4 ~ p { match($0, /pid=([0-9]+)/, a); if (a[1]) print a[1] }'
  elif command -v lsof >/dev/null 2>&1; then
    lsof -ti ":$port" -sTCP:LISTEN 2>/dev/null || true
  elif command -v netstat >/dev/null 2>&1; then
    # Windows / MSYS netstat -ano: TCP <local> <remote> LISTENING <pid>
    netstat -ano 2>/dev/null \
      | awk -v p=":$port" '$1 ~ /^TCP/ && $2 ~ p"$" && $4 == "LISTENING" { print $5 }' \
      | sort -u
  fi
}

kill_pid() {
  local pid="$1"
  [[ -n "$pid" ]] || return 0
  warn "killing existing listener pid=$pid on port $PORT"
  if command -v taskkill >/dev/null 2>&1; then
    taskkill //F //PID "$pid" >/dev/null 2>&1 || true
  else
    kill "$pid" 2>/dev/null || true
    sleep 1
    kill -9 "$pid" 2>/dev/null || true
  fi
}

log "checking port $PORT for existing instances..."
mapfile -t LISTENERS < <(find_listeners_on_port "$PORT")
if (( ${#LISTENERS[@]} > 0 )); then
  warn "port $PORT in use by pid(s): ${LISTENERS[*]}"
  for pid in "${LISTENERS[@]}"; do
    kill_pid "$pid"
  done
  sleep 1
  mapfile -t REMAINING < <(find_listeners_on_port "$PORT")
  if (( ${#REMAINING[@]} > 0 )); then
    err "port $PORT still occupied by: ${REMAINING[*]} — aborting."
    exit 1
  fi
  log "port $PORT freed."
else
  log "port $PORT is free."
fi

if (( CHECK_ONLY )); then
  exit 0
fi

# ── Step 2: build frontend artifacts ────────────────────────────────────────
build_vite() {
  if [[ ! -d "$FRONTEND_DIR" ]]; then
    err "vite frontend directory not found: $FRONTEND_DIR"
    exit 1
  fi
  log "ensuring node_modules in $FRONTEND_DIR"
  if [[ ! -d "$FRONTEND_DIR/node_modules" ]] \
     || [[ ! -d "$FRONTEND_DIR/node_modules/@context-engine/viewer-api-frontend" ]]; then
    (cd "$FRONTEND_DIR" && npm install)
  fi
  log "vite build → $STATIC_DIR"
  (cd "$FRONTEND_DIR" && npx vite build)
  if [[ ! -f "$STATIC_DIR/index.html" ]]; then
    err "vite build did not produce $STATIC_DIR/index.html"
    exit 1
  fi
}

build_trunk() {
  if [[ ! -d "$FRONTEND_DIR" ]]; then
    err "trunk frontend directory not found: $FRONTEND_DIR"
    exit 1
  fi
  if ! command -v trunk >/dev/null 2>&1; then
    err "'trunk' not found on PATH. Install with: cargo install trunk"
    exit 1
  fi
  log "trunk build (release) in $FRONTEND_DIR → $STATIC_DIR"
  (cd "$FRONTEND_DIR" && trunk build --release)
  if [[ ! -f "$STATIC_DIR/index.html" ]]; then
    err "trunk build did not produce $STATIC_DIR/index.html"
    exit 1
  fi
}

if (( ! NO_BUILD )); then
  case "$FRONTEND_KIND" in
    vite)  build_vite ;;
    trunk) build_trunk ;;
  esac
  log "frontend artifacts ready."
else
  log "skipping frontend build (--no-build)"
fi

# ── Step 3: launch the server ───────────────────────────────────────────────
log "starting $CARGO_PKG on port $PORT"
cd "$REPO_ROOT"
export PORT
exec cargo run --quiet -p "$CARGO_PKG" -- "${EXTRA_ARGS[@]}"
