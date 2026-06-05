## Goal
Replace the current Python-based repo-map generation flow with a repo-aware `peek-api` skeleton/tree renderer that can accept a folder path, apply compaction/filtering rules, and emit a tree-shaped structural map without repeating path segments.

## Why
The current generator hardcodes repository-specific extraction rules and emits mostly flat sections. The new behavior should live in the same Rust inspection stack as `peek`, reuse skeletonization rules, and support directory/folder rendering directly instead of relying on a one-off Python script.

## References
- Current generated output: `repo_map.toon`
- Current structural-awareness guidance: `.agents/instructions/token-efficiency.instructions.md`
- Existing `peek` CLI behavior and skeleton docs: `tools/cli/peek-cli/src/main.rs`, `tools/cli/peek-cli/README.md`
- Workspace membership source: `Cargo.toml`
- Original repo-map ticket scope: `.ticket/tickets/72c1e92d-65e1-445b-9365-e3384d9da088/description.md`

## Dependency
This ticket depends on introducing `peek-api` first:
- parent prerequisite: `06cfe998` (`peek-api` + `peek-cli` + `peek-mcp` layering)

## Scope
Extend `peek-api` so skeleton rendering can target a folder/repository tree, not just a single file.

Required outcomes:
- folder input support for skeleton rendering
- tree-structured output that compacts repeated path prefixes
- repo-aware filtering for low-token structural maps
- replacement of the old Python shim with a Rust-based generation path
- regenerated `repo_map.toon` produced by the new path

## Proposed behavior
1. `peek-api` gains a folder skeleton mode that:
   - walks a directory tree
   - filters ignored/noisy paths
   - emits a hierarchical tree view instead of repeating full paths per row
2. `repo_map.toon` generation becomes a specialized use of that folder skeleton mode, with repository-specific sections layered on top only where needed.
3. The output should compact repeated path segments, for example by rendering nested directory trees instead of repeated `tools/...` and `memory-viewers/...` prefixes.
4. The old Python script should be removed, with the Rust path becoming the source of truth.

## Implementation plan
1. Define folder-skeleton request/response types in `peek-api`.
2. Implement directory walking with explicit filtering/compaction rules.
3. Encode tree output so shared parent segments appear once.
4. Add repo-map-specific assembly logic that pulls in:
   - workspace members from `Cargo.toml`
   - agent instruction/prompt files
   - hook files
   - key tool locations
5. Expose the new generation path through `peek-cli` (for local generation) and optionally `peek-mcp` (for agent workflows).
6. Remove the old Python generator flow in favor of the Rust-backed command.
7. Update docs and hook guidance so refresh instructions reference the Rust generator, not Python.

## Acceptance criteria
- Folder skeleton generation works on directories, not just files.
- The generated repo map uses a tree structure that avoids repeating path segments unnecessarily.
- The filtering/compaction rules keep output low-token and structurally useful.
- `repo_map.toon` can be regenerated without relying on custom Python parsing logic.
- Guidance/docs point at the new Rust-backed generation flow.

## Validation notes
Required validation before review:
- `cargo test -p peek-api`
- `cargo test -p peek-cli`
- generate `repo_map.toon` using the new path and inspect the diff
- verify the generated output still covers current sections needed by agent guidance

Recommended focused tests:
- snapshot tests for folder tree rendering
- tests for prefix compaction and ignored-path filtering
- repo-map generation snapshot against a fixture workspace
- regression test confirming crate/member discovery remains stable when `Cargo.toml` changes

## Risks / design notes
- Keep generic folder-skeleton logic in `peek-api`; avoid embedding too much repo-specific policy into the transport layer.
- The repo-map specialization should be deterministic and testable.
- Do not regress the current structural-awareness guidance while replacing the generator path.