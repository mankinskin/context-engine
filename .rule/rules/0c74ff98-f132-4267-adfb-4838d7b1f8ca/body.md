### Orientation (start of every session)

Before writing or editing a spec:

- search existing specs for the behavior first
- search related tickets so the spec can link the current execution plan
- check whether a neighboring or parent spec already owns the requested slice

Prefer `spec-mcp` and `ticket-mcp` tools when available. Fall back to `./target/debug/spec.exe` and `./target/debug/ticket.exe` when needed.