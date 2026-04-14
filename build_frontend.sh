#!/usr/bin/env bash
set -e

if [[ -z "$1" ]]; then
  echo "Usage: $0 <viewer-name|--all>" >&2
  exit 1
fi

build_frontend() {
  local viewer="$1"
  local vite_dir="tools/viewer/$viewer/frontend"
  local dioxus_dir="tools/viewer/$viewer/dioxus-frontend"

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
  for dir in tools/viewer/*/frontend tools/viewer/*/dioxus-frontend; do
    [[ -d "$dir" ]] || continue
    viewer=$(basename "$(dirname "$dir")")
    # Avoid building the same viewer twice if both frontend/ and dioxus-frontend/ exist.
    # Prefer dioxus-frontend when present.
    if [[ "$dir" == *"/frontend" && -d "tools/viewer/$viewer/dioxus-frontend" && -f "tools/viewer/$viewer/dioxus-frontend/Dioxus.toml" ]]; then
      continue
    fi
    build_frontend "$viewer"
  done
else
  build_frontend "$1"
fi
