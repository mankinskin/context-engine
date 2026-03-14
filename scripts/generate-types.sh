#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== Generating TypeScript types ==="
cd "$ROOT_DIR"

# Clean previous output
rm -f packages/context-types/src/generated/*.ts
# Keep .gitkeep
touch packages/context-types/src/generated/.gitkeep

# Generate types from all crates that have ts-rs derives
echo "Generating context-api types..."
cargo test -p context-api --features ts-gen export_bindings -- --ignored 2>/dev/null || true

echo "Generating context-trace types..."
cargo test -p context-trace export_bindings -- --ignored 2>/dev/null || true

echo "Generating log-viewer types..."
cargo test -p log-viewer export_bindings -- --ignored 2>/dev/null || true

# Count generated files
COUNT=$(ls -1 packages/context-types/src/generated/*.ts 2>/dev/null | grep -v '.gitkeep' | wc -l)
echo "=== Generated $COUNT TypeScript type files ==="

# Build the npm package
echo "Building @context-engine/types..."
cd packages/context-types
npm run build
echo "=== Done ==="
