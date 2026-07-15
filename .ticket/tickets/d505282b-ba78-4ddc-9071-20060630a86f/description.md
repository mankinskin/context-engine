## Goal
Make locally installed MCP binaries discoverable and correctly identified from GitHub Copilot Chat in this VS Code workspace.

## Root cause
- Five handlers (`context-mcp`, `ticket-mcp`, `spec-mcp`, `test-mcp`, and `log-viewer-mcp`) omitted `ServerInfo.server_info`.
- rmcp therefore supplied its default server label, `rmcp`, even though the configured executable and tools were otherwise correct.

## Implementation
- Preserve the direct installed-binary configuration and `--mcp` installation flow.
- Add explicit package-derived `server_info` name and version to all five affected handlers, matching the existing feedback/session/peek/rule identity pattern.

## Validation plan
- Build and run targeted affected packages.
- Probe each installed executable's MCP initialize response and require its configured server label rather than `rmcp`.
- Reload VS Code to force fresh server initialization and labels.

## Activation
After installation of rebuilt binaries, run `Developer: Reload Window`.