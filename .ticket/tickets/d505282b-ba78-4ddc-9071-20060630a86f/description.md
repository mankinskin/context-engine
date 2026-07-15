## Goal
Make locally installed MCP binaries discoverable and usable from GitHub Copilot Chat in this VS Code workspace.

## Resolved root causes
- The active `.vscode/mcp.json` was an independent regular file, not the expected symlink, and still used Cargo commands after `.github/mcp.json` was corrected.
- The canonical configuration had also regressed to the unsupported top-level `mcpServers` object.
- `install-tools.sh` lacked an MCP-specific installation flow.
- Running Cargo-launched MCP processes locked workspace `target/release` binaries on Windows, preventing some `cargo install` operations.

## Implementation
- Both active configuration copies now use VS Code's supported top-level `servers` object and invoke installed binaries directly.
- Added `./install-tools.sh --mcp`, which installs all eleven configured MCP binaries.
- Installer builds now use `target/install-tools`, avoiding locks held by running development servers.
- Documented installation, PATH, and reload activation in the root README.

## Validation
- `./install-tools.sh --mcp`: all eleven binaries installed successfully.
- Both configuration copies are byte-identical, have clean editor diagnostics, and validate as eleven direct-binary servers with no Cargo commands.
- Installed `feedback-mcp` passed fresh stdio initialize and tools/list, exposing feedback_ingest, feedback_inbox, feedback_query, feedback_mine, and feedback_summary.
- Recorded evidence: `exec-vscode-copilot-mcp-installed-binaries-20260715` under `val-vscode-copilot-mcp-registration`.

## Activation
Run `Developer: Reload Window` now. VS Code will restart from the active `.vscode/mcp.json` and launch the installed `feedback-mcp.exe` directly.