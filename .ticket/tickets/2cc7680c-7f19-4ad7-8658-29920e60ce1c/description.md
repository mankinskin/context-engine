Review follow-up from ticket dbe0e955-c1b4-414d-820c-10c3fbbb5d3d.

## Finding

The reference workspace proves that binaries compile and start, but its CLI command and MCP handler are empty and HTTP exposes only a health route. This is too thin to demonstrate that real domain registration remains outside the harness while shared parsing, startup, output, and error mechanics remain inside it.

## Acceptance criteria

- CLI reference parses at least one domain argument/subcommand and dispatches a domain operation through shared output handling.
- MCP reference registers and successfully invokes at least one domain tool.
- HTTP reference registers a domain success route and an error route using the harness error envelope/status mapping.
- Tests prove default feature slimness and each transport independently, plus all transports together.
- Remove placeholder-only code and document what the reference contract proves.

## Review verdict (2026-07-25): NEEDS CHANGES

Intent approved, but implementation is NOT yet authorized. The reviewer requires more research and design first. Recorded decisions:

- Durable proof home: the reference proof moves into memory-kernel integration tests (not the context-engine reference workspace). The context-engine reference becomes a thin consumer if retained at all.
- Transport coverage: a single realistic domain operation must be exposed through all three transports (CLI, MCP, HTTP).
- Assertions: tests must assert both success output AND the harness error envelope + HTTP status mapping. The error path is mandatory.
- Feature slimness: must not regress the accepted parent criterion default = [] with independently selectable CLI/MCP/HTTP.

## Required precursor

A design/research pass must complete and be accepted before this ticket may be implemented. Tracked as ticket 60114a17-c0ad-43eb-8df6-4741a59d83ce (Design memory-kernel transport-harness reference-proof integration tests). This ticket depends on it.

## Dependencies

- Depends on 60114a17 (design precursor).
- Depends on 9451f439 (submodule discoverability) — resolves the harness dependency path.
- Depends on f10f52e4 (canonical spec in memory-kernel) — resolves proof/validation home.

## First validation after implementation (unchanged)

cargo test -p example --features cli, then independent MCP and HTTP tests, then the all-feature build.