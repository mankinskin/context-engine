#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd -- "$script_dir/../.." && pwd)
toolchain=${RUSTUP_TOOLCHAIN:-$(<"$script_dir/viewer-toolchain.txt")}
base_image=${RUST_BASE_IMAGE:-rust:1.91-bookworm}
node_base_image=${NODE_BASE_IMAGE:-node:20-bookworm-slim}
tag=${DOCKER_IMAGE_TAG:-context-engine-viewer-install-validation:${toolchain}}

echo "[docker-build] Building $tag with $toolchain"
docker build \
    --build-arg "RUST_BASE_IMAGE=$base_image" \
    --build-arg "NODE_BASE_IMAGE=$node_base_image" \
    -f "$script_dir/Dockerfile.viewer" \
    -t "$tag" \
    "$repo_root"

echo "[docker-run] Running $tag"
docker run --rm "$tag"