# Problem

Spec workflows are not root-aware enough across nested `.spec` stores.

During this session, all meaningful spec work had to be driven through explicit shell context switches and explicit executable/index-root wiring such as:

- repo-root `.spec/`
- `memory-api/.spec/`

Direct MCP spec usage stayed at zero while the session still issued 30 spec CLI commands, which is a strong signal that the current targeting model is too awkward for real multi-root work.

# Session Evidence

- The session used 30 spec CLI invocations and 0 direct MCP spec tool executions.
- The operator had to switch between root and nested `.spec` stores explicitly and restate `--index-root .spec` repeatedly.
- Health, refs validation, create, update, and search flows all depended on hand-assembled `cd ... && spec.exe ... --index-root .spec` commands.

# Scope

1. Add an explicit root/workspace selector for spec workflows in CLI and MCP.
2. Cover at least:
   - list
   - search
   - get / read
   - create
   - update
   - health
   - refs validate
3. Return active spec-root metadata in JSON output so clients do not infer it from cwd.
4. Document the nested-store model and the intended retry / targeting syntax.
5. Add regression coverage across both repo-root and nested `.spec` stores.

# Regression Validation Requirements

- **Specification / docs:** define spec-root selection semantics and the output fields that describe active scope/root.
- **CLI:** add integration coverage for repo-root and nested-root invocation, including a nested store like `memory-api/.spec/`.
- **MCP:** add parity coverage for the same root-targeting semantics.
- **Frontends / tools:** any spec-viewer or agent-facing spec tooling that loads from nested stores must consume backend root metadata instead of assuming cwd.
- **Manual validation:** reproduce the May 21 spec flow in both root and nested stores without manual executable-path or cwd spelunking.

# Acceptance Criteria

- CLI and MCP spec workflows accept an explicit root/workspace selector.
- JSON output names the active spec root/scope.
- Repo-root and nested-root spec operations are both test-covered.
- The documented nested-store targeting flow matches real operator usage.
- A user can repeat the May 21 spec work without hand-building `cd ... && spec.exe ... --index-root .spec` chains.

# Likely Surfaces

- `tools/spec-cli/`
- `tools/spec-mcp/`
- `crates/spec-api/`
- `memory-api/.spec/`
- `README.md`
