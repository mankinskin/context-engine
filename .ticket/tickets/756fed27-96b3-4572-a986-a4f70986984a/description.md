# Goal

Replace ad hoc tracing subscriber setup in memory-api CLI/MCP/HTTP tools with one shared initialization path.

## Scope

- Compare `viewer-api::init_tracing_full`, `context-api::tracing_capture`, and `context-trace` test tracing.
- Implement or select a dependency-light shared tracing runtime for memory-system tools.
- Support EnvFilter directives, stdout/stderr, rolling JSONL file sinks, non-blocking guard ownership, env/TOML config, and optional log-api session registration.
- Migrate representative CLI, MCP, and HTTP binaries first, then document the rest of the rollout.

## Acceptance criteria

- At least one CLI, one MCP server, and one HTTP server use the shared initializer.
- New transports no longer need hand-rolled `tracing_subscriber::fmt()` setup.
- Guard ownership and shutdown flushing are tested or documented.