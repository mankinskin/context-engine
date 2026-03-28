#!/usr/bin/env bash
set -e

if [[ -z "$1" ]]; then
  echo "Usage: $0 <viewer-name|--all>" >&2
  exit 1
fi

build_frontend() {
  pushd "tools/viewer/$1/frontend"
  npx vite build
  popd
}

if [[ "$1" == "--all" ]]; then
  for dir in tools/viewer/*/frontend; do
    viewer=$(basename "$(dirname "$dir")")
    build_frontend "$viewer"
  done
else
  build_frontend "$1"
fi
