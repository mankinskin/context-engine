# 02: File and Ticket-MCP Tool Path

## Outcome

The selected template can authorize a bounded file-reading capability and one `ticket-mcp` lookup. The fixture scenario lets the model identify a fictional person from the attached file and obtain that person's age from ticket prose. Its response contract is exactly `{"age": <integer>}`; wrappers, extra keys, and prose are invalid for this scenario.

## Evidence

- [.github/mcp.json](../../.github/mcp.json) registers `fs-mcp` and `ticket-mcp` as stdio MCP servers.
- [memory-api/test-fixtures/memory-workspace-fixture/fixtures.toml](../../memory-api/test-fixtures/memory-workspace-fixture/fixtures.toml) establishes `.ticket` as a fixture-store convention.

## Non-goal

Do not support spec-mcp, arbitrary MCP-server configuration, mutation tools, or unrestricted filesystem access.

## Validation Method

Cover template/tool configuration and request construction with offline tests; exercise the configured ticket lookup and attached-file read in the credential-gated end-to-end fixture command from package 03.