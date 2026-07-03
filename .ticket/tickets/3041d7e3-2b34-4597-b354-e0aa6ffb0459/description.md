# Goal

Ensure transport lifecycle events correlate with domain operations, log sessions, and operation journals.

## Scope

- Define request/tool/command ids for HTTP, MCP, and CLI.
- Propagate correlation ids into tracing spans and log-api session metadata.
- Include journal ids in apply/resume/rollback responses and spans.
- Normalize error classification, status codes, durations, and route/tool names.

## Acceptance criteria

- A failed HTTP/MCP/CLI operation can be traced from transport request to domain operation to log session and journal id.
- Correlation fields are consistent across transports.
- Sensitive request payloads are not logged by default.