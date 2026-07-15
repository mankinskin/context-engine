<!-- aligned-structure:v2 -->
# VS Code Copilot local MCP server configuration

## Motivation
Agent prompts need direct access to the repository's MCP capabilities under stable, intelligible server identities. Incomplete server metadata makes VS Code display the rmcp library name instead of the configured server name, obscuring the tool source and weakening discovery diagnostics.

## Dependent expectation
If this spec is implemented, dependents can rely on every configured local MCP server returning its package name and version in the MCP initialize `serverInfo`, advertising the tools capability, and enumerating its tools over stdio. In particular, `context-mcp`, `ticket-mcp`, `spec-mcp`, `test-mcp`, and `log-viewer` must never identify as `rmcp`.

`.vscode/mcp.json` and `.github/mcp.json` contain the same supported top-level `servers` object. Each configured server launches its installed executable directly, rather than using `cargo run`. `./install-tools.sh --mcp` installs the complete configured MCP binary set into Cargo's binary directory, using an isolated `target/install-tools` build directory so Windows locks held by running development servers cannot block installation. Cargo's binary directory must be on VS Code's inherited `PATH`.

## Guards
- `val-vscode-copilot-mcp-registration`: requires schema validation, matching direct-binary configuration copies, complete MCP installation, unique package server identities, and successful installed-binary protocol checks.
- Existing evidence: `exec-vscode-copilot-mcp-installed-binaries-20260715` verifies direct binary installation and feedback tool exposure; affected identity evidence is pending.

## Positions
- `implemented` — `code_ref: .github/mcp.json`: canonical supported `servers` registrations for eleven direct executable MCP targets.
- `implemented` — `code_ref: .vscode/mcp.json`: active VS Code configuration aligned with the canonical direct-binary registrations.
- `implemented` — `code_ref: install-tools.sh`: `--mcp` installs the complete configured server set with an isolated installation target directory.
- `partial` — `code_ref: context-stack/tools/mcp/context-mcp/src/server/metadata.rs`: lacks explicit package identity metadata.
- `partial` — `code_ref: memory-api/tools/mcp/ticket-mcp/src/server.rs`: lacks explicit package identity metadata.
- `partial` — `code_ref: memory-api/tools/mcp/spec-mcp/src/server.rs`: lacks explicit package identity metadata.
- `partial` — `code_ref: memory-api/tools/mcp/test-mcp/src/server.rs`: lacks explicit package identity metadata.
- `partial` — `code_ref: memory-viewers/log-viewer/src/mcp_server.rs`: lacks explicit package identity metadata.

## Governing-rule requirement
The session bootstrap policy at `.rule/rules/89330b3b-4d28-4c48-80dd-203311dbe855/body.md` must introduce task-relevant workspace guidance, including this MCP availability contract when agent tooling configuration is changed.

## Traceability
- Configuration ticket: `.ticket/tickets/d505282b-ba78-4ddc-9071-20060630a86f/ticket.toml`.
- Validation spec: `.test/default/specs/val-vscode-copilot-mcp-registration.json`.

A VS Code window reload is required after installed-binary or server identity changes so Copilot rebuilds its MCP server and tool snapshot.