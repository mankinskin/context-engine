#!/usr/bin/env bash
set -e

if [[ -z "$1" ]]; then
  echo "Usage: $0 <viewer-name|--all>" >&2
  exit 1
fi

build_frontend() {
  local viewer="$1"
  local vite_dir="tools/viewer/$viewer/frontend/ts"
  local dioxus_dir="tools/viewer/$viewer/frontend/dioxus"

  if [[ -d "$dioxus_dir" && -f "$dioxus_dir/Dioxus.toml" ]]; then
    echo "Building Dioxus frontend for $viewer ..."
    pushd "$dioxus_dir"
    dx build --release
    popd
  elif [[ -d "$vite_dir" ]]; then
    echo "Building Vite frontend for $viewer ..."
    pushd "$vite_dir"
    npx vite build
    popd
  else
    echo "No frontend found for $viewer" >&2
    exit 1
  fi
}

if [[ "$1" == "--all" ]]; then
  for dir in tools/viewer/*/frontend/ts tools/viewer/*/frontend/dioxus; do
    [[ -d "$dir" ]] || continue
    viewer=$(basename "$(dirname "$(dirname "$dir")")")
    # Avoid building the same viewer twice if both ts/ and dioxus/ exist.
    # Prefer dioxus when present.
    if [[ "$dir" == *"/frontend/ts" && -d "tools/viewer/$viewer/frontend/dioxus" && -f "tools/viewer/$viewer/frontend/dioxus/Dioxus.toml" ]]; then
      continue
    fi
    build_frontend "$viewer"
  done
else
  build_frontend "$1"
fi
