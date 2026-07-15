<!-- aligned-structure:v2 -->
# VS Code Copilot local MCP server configuration

## Motivation
Agent prompts need direct access to the repository's feedback, session, inspection, rule, audit, and compact terminal capabilities. Invalid, incomplete, or identity-colliding workspace MCP registrations prevent GitHub Copilot Chat from discovering those local stdio servers.

## Dependent expectation
If this spec is implemented, dependents can rely on GitHub Copilot Chat discovering `feedback-mcp`, `session-mcp`, `peek-mcp`, `rule-mcp`, `audit-mcp`, and `compact-terminal-mcp` after a VS Code window reload. Each server returns its true package name and version in the MCP initialize `serverInfo`, advertises the tools capability, and can enumerate its tools over stdio.

`.vscode/mcp.json` is a repository symlink to `.github/mcp.json`, making the target the single source of truth. The shared target uses only the top-level `servers` object accepted by the VS Code MCP schema. It must not contain the unsupported top-level `mcpServers` compatibility object.

## Guards
- `val-vscode-copilot-mcp-registration`: requires editor schema validation, all six registrations, six unique package-level initialize names, and successful MCP `initialize` plus `tools/list` responses.
- Passing evidence: `exec-vscode-copilot-feedback-mcp-tools-20260715` verifies `feedback-mcp` advertises tools and lists its five feedback operations over a fresh release stdio connection.

## Positions
- `implemented` — `code_ref: .github/mcp.json`: canonical `servers` registrations for the six local MCP packages.
- `implemented` — `code_ref: .vscode/mcp.json`: VS Code-facing symlink to the canonical configuration.
- `implemented` — `code_ref: memory-api/tools/mcp/feedback-mcp/src/server.rs`: reports `feedback-mcp`, advertises tools, and exposes five feedback tools.
- `implemented` — `code_ref: memory-api/tools/mcp/session-mcp/src/server.rs`: reports `session-mcp` and the package version.
- `implemented` — `code_ref: memory-api/tools/mcp/peek-mcp/src/server.rs`: reports `peek-mcp` and the package version.
- `implemented` — `code_ref: memory-api/tools/mcp/rule-mcp/src/server.rs`: reports `rule-mcp` and the package version.
- `implemented` — `code_ref: memory-api/tools/mcp/audit-mcp/src/server.rs`: reports `audit-mcp` and the package version.
- `implemented` — `code_ref: memory-api/tools/mcp/compact-terminal-mcp/src/server.rs`: reports `compact-terminal-mcp` and the package version.

## Governing-rule requirement
The session bootstrap policy at `.rule/rules/89330b3b-4d28-4c48-80dd-203311dbe855/body.md` must introduce task-relevant workspace guidance, including this MCP availability contract when agent tooling configuration is changed.

## Traceability
- Configuration ticket: `.ticket/tickets/d505282b-ba78-4ddc-9071-20060630a86f/ticket.toml`.
- Identity remediation ticket: `memory-api/.ticket/tickets/2318f63f-7113-4987-87b7-ca26afa04d11/ticket.toml`.
- Validation spec: `.test/default/specs/val-vscode-copilot-mcp-registration.json`.
- Passing execution: `.test/default/executions/exec-vscode-copilot-feedback-mcp-tools-20260715.json`.

A VS Code window reload is required after registration or server capability changes so Copilot rebuilds its MCP server and tool snapshot.