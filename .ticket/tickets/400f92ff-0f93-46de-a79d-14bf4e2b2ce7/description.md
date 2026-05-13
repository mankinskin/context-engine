# Problem

The context-stack tool history is now present in `../context-stack`, but the imported manifests still reflect the old monorepo-relative dependency layout. That means the history move succeeded while the standalone repository integration is still incomplete.

# Scope

Retarget the imported tool manifests for the standalone `context-stack` repository layout and decide which tools should become active workspace members there.

The work should cover:

- `tools/cli/context-cli/Cargo.toml`
- `tools/mcp/context-mcp/Cargo.toml`
- `tools/http/context-http/Cargo.toml`
- `tools/context-editor/kernel/Cargo.toml`
- `tools/context-editor/sandbox-app/Cargo.toml`
- the dependency boundary for `viewer-api`
- the workspace-membership decision for any tool that should build inside standalone `context-stack`

# Acceptance Criteria

- Imported tool manifests in `../context-stack/tools/**` no longer point at invalid monorepo-relative paths.
- The standalone layout decision is explicit for each imported tool: workspace member, deferred, or blocked.
- Any tool admitted into the standalone workspace passes focused cargo validation.
- The remaining unsupported dependency boundaries are documented explicitly.
