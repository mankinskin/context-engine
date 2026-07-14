# Relocate viewer and peek crates into owning repositories

Move repository-owned packages to their canonical Git submodule owners without changing package names or runtime behavior.

## Requirements

- Move `tools/viewer/doc-viewer/` to `memory-viewers/doc-viewer/`.
- Move `tools/viewer/log-viewer/` to `memory-viewers/log-viewer/`.
- Move `tools/peek-api/` to `memory-api/crates/peek-api/`.
- Move `tools/cli/peek-cli/` to `memory-api/tools/cli/peek-cli/`.
- Move `tools/mcp/peek-mcp/` to `memory-api/tools/mcp/peek-mcp/`.
- Move `tools/mcp/compact-terminal-mcp/` to `memory-api/tools/mcp/compact-terminal-mcp/`.
- Preserve Cargo package and binary names and public/runtime behavior.
- Keep the root Cargo workspace as the aggregate build coordinator.
- Update active workspace members, path dependencies, viewer registry, VS Code integration, install/validation scripts, hooks, workflows, current docs, tests, generated rule sources, and current spec code references.
- Do not rewrite historical ticket descriptions or revision logs solely to replace old paths.
- Do not retain compatibility shims at the old source paths.
- Preserve unrelated user changes, including existing deleted log-viewer-related files outside the requested source tree.

## Acceptance criteria

- All six old package directories are absent and all six canonical destination directories exist.
- `cargo metadata` resolves every moved package from its destination path with unchanged package names.
- Focused Rust tests/checks pass for `peek-api`, `peek-cli`, `peek-mcp`, `compact-terminal-mcp`, `doc-viewer`, `log-viewer`, and both Dioxus frontend crates.
- Managed `viewer-ctl` prepare/start behavior works for doc-viewer and log-viewer from the new paths.
- Existing frontend lint/typecheck/unit and managed Playwright checks pass without path-related failures.
- Active path references no longer point at the removed locations, except intentionally historical records.
- Root, memory-api, and memory-viewers Git states represent the cross-repository relocation without reverting unrelated changes.

## Non-goals

- Rename packages, binaries, ports, MCP tool names, or user-facing behavior.
- Refactor viewer or peek implementation logic.
- Rewrite immutable historical evidence.

## Implementation summary

- Relocated all six source trees to their canonical `memory-viewers` and `memory-api` owners with package identities unchanged.
- Reconciled aggregate workspace members, relative dependencies, viewer registry, frontend assets/imports, Playwright roots, install scripts, generated rules, repo map, current docs, and operational configuration.
- Corrected one-level path-depth assumptions in relocated viewer runtime defaults and generated TypeScript export paths.
- Preserved historical ticket/rule revisions and omitted old-path compatibility shims.

## Validation summary

- `exec-workspace-relocated-crates-20260714`: passed; Cargo metadata succeeded and 96 native tests passed across 12 suites. WASM and TypeScript frontend builds passed; log Vitest passed 18 tests.
- `exec-workspace-relocated-viewers-browser-20260714`: passed; fresh managed release deployments resolved paths under `context-engine` and rendered in external Chromium at 1280x800 and 390x844. Screenshots are under `target/tmp/viewer-relocation/`.
- `exec-workspace-relocated-active-refs-20260714`: passed; active references use canonical paths, with only intentionally historical records retaining old paths.
- Residual UI risk: doc-viewer's initially open mobile drawer has an off-viewport close-button hit target; its DOM close handler and settled mobile content work. This behavior predates and is outside the relocation scope.
