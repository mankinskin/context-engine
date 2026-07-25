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

## Implementation (2026-07-25): DONE

Design precursor 60114a17 accepted; implemented per its accepted design.

- Durable proof lives in memory-kernel integration test `crates/transport-harness/tests/reference_proof.rs`. One realistic op `describe(id)` over a tiny fixed registry (`harness` -> "Shared transport harness"; unknown -> `unknown item: <id>`). Fixture domain is inline + dev-only, so the library surface and `default = []` slimness are untouched.
- CLI: parses a `describe --id` subcommand and dispatches through `cli::run_from` + shared `Output::json`; asserts the one-line JSON success, the `HarnessError::Domain` unknown-id error, and `HarnessError::Arguments` on bad args.
- MCP: registers a real `describe` tool (`#[tool_router]`/`#[tool]`/`#[tool_handler]`) and invokes it in-process; asserts the success item and the domain-error mapping.
- HTTP: registers `GET /describe/{id}`; success -> 200 + item JSON, unknown -> `HttpError` 404 with envelope `{"code":"not_found","message":"unknown item: missing"}`; asserts BOTH status and envelope via `tower::oneshot`.
- Added a `core_proof` test that runs under `default = []`, proving shared output/error mechanics work with no transport deps.
- context-engine reference retained as a thin compile-only consumer (builds all three gated binaries via the branch-pinned git dep); no behavioral assertions duplicated there.
- Harness correctness fix discovered by the proof: `write_output`'s JSON branch appended the JSON-encoded string `"\n"` (quoted) instead of a real newline byte. Fixed to write the value then `b"\n"`; added guarding unit test `write_output_appends_one_real_newline_to_json`.

### Validation (all passed)

- `cargo test -p transport-harness --features cli` -> 4 ok; `--features mcp` -> 3 ok; `--features http` -> 3 ok.
- `cargo test -p transport-harness --all-features` -> 6 unit + 8 integration ok.
- `cargo build -p transport-harness` (default) ok; `cargo tree` default shows no clap/axum/rmcp/tokio (only 2 pre-existing dead-code warnings for transport-only helpers).
- `cargo clippy -p transport-harness --all-features --tests` -> clean.
- Reference `cargo build -p example --features cli,mcp,http` -> ok via git dep.
- Evidence: memory-kernel/.test (workspace-slug memory-kernel) execution exec-vt-transport-harness-reference-proof-20260725 (spec vt-transport-harness-reference-proof).

### State move blocker

`update_ticket to_state` returns `store error: no schema for type 'task'` — the known schema defect. Ticket remains in `new`; state not falsified.