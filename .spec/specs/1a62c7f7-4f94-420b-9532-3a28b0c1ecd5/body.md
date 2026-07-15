<!-- aligned-structure:v2 -->
# VS Code Copilot local MCP server configuration

## Motivation
Agent prompts need direct access to the repository's feedback, session, inspection, rule, audit, and compact terminal capabilities. Invalid, incomplete, identity-colliding, or development-build-only workspace MCP registrations prevent GitHub Copilot Chat from discovering those local stdio servers.

## Dependent expectation
If this spec is implemented, dependents can rely on GitHub Copilot Chat discovering the configured local MCP servers after a VS Code window reload. Each server returns its true package name and version in the MCP initialize `serverInfo`, advertises the tools capability, and can enumerate its tools over stdio.

`.vscode/mcp.json` and `.github/mcp.json` contain the same supported top-level `servers` object. Each configured server launches its installed executable directly, rather than using `cargo run`. `./install-tools.sh --mcp` installs the complete configured MCP binary set into Cargo's binary directory, using an isolated `target/install-tools` build directory so Windows locks held by running development servers cannot block installation. Cargo's binary directory must be on VS Code's inherited `PATH`.

## Guards
- `val-vscode-copilot-mcp-registration`: requires schema validation, matching direct-binary configuration copies, complete MCP installation, and successful installed-binary protocol checks.
- Passing evidence: `exec-vscode-copilot-mcp-installed-binaries-20260715` verifies all eleven configured binaries are installed, both configuration copies contain direct commands, and installed `feedback-mcp` exposes all five feedback tools.

## Positions
- `implemented` — `code_ref: .github/mcp.json`: canonical supported `servers` registrations for eleven direct executable MCP targets.
- `implemented` — `code_ref: .vscode/mcp.json`: active VS Code configuration aligned with the canonical direct-binary registrations.
- `implemented` — `code_ref: install-tools.sh`: `--mcp` installs the complete configured server set with an isolated installation target directory.
- `implemented` — `code_ref: memory-api/tools/mcp/feedback-mcp/src/server.rs`: reports `feedback-mcp`, advertises tools, and exposes five feedback tools.

## Governing-rule requirement
The session bootstrap policy at `.rule/rules/89330b3b-4d28-4c48-80dd-203311dbe855/body.md` must introduce task-relevant workspace guidance, including this MCP availability contract when agent tooling configuration is changed.

## Traceability
- Configuration ticket: `.ticket/tickets/d505282b-ba78-4ddc-9071-20060630a86f/ticket.toml`.
- Validation spec: `.test/default/specs/val-vscode-copilot-mcp-registration.json`.
- Passing execution: `.test/default/executions/exec-vscode-copilot-mcp-installed-binaries-20260715.json`.

A VS Code window reload is required after registration, installed-binary, or server capability changes so Copilot rebuilds its MCP server and tool snapshot.