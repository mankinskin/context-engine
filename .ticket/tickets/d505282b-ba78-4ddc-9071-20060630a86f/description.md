## Goal
Make feedback-mcp, session-mcp, peek-mcp, rule-mcp, audit-mcp, and compact-terminal-mcp discoverable and usable from GitHub Copilot Chat in this VS Code workspace.

## Root cause
- Five requested servers were absent from the shared MCP registration.
- `.vscode/mcp.json` is a symlink to `.github/mcp.json`.
- The shared target contained a top-level `mcpServers` compatibility object that the VS Code MCP schema rejects. This could invalidate discovery, including the already-present feedback-mcp entry.
- Copilot snapshots MCP tools for a chat/window; a window reload is required after registration changes.
- Feedback-mcp started successfully but VS Code discovered zero tools because its `ServerInfo` did not advertise the `tools` capability.

## Implementation
- Added session-mcp, peek-mcp, rule-mcp, audit-mcp, and compact-terminal-mcp to the supported `servers` map.
- Removed the unsupported `mcpServers` object from the shared target.
- Preserved feedback-mcp and existing context/ticket/spec/test/log-viewer registrations.
- Feedback-mcp now advertises its tools capability and includes a focused regression assertion.

## Validation
- Editor diagnostics: no errors for either linked MCP config path or the feedback server.
- `cargo test -p feedback-mcp`: passed.
- Fresh release stdio probe: initialize advertises tools; `tools/list` returns `feedback_ingest`, `feedback_inbox`, `feedback_query`, `feedback_mine`, and `feedback_summary`.
- Recorded evidence: `exec-vscode-copilot-feedback-mcp-tools-20260715` under `val-vscode-copilot-mcp-registration`.
- `cargo fmt --check -p feedback-mcp` remains blocked by pre-existing formatting differences in the feedback server and `main.rs`; those unrelated lines were left unchanged.
- Requirement spec: `.spec/specs/1a62c7f7-4f94-420b-9532-3a28b0c1ecd5/spec.toml`.

## Activation
Run `Developer: Reload Window` in VS Code. The restarted feedback-mcp instance will now report and expose its five tools to Copilot.