# Memory-system observability and log-api runtime diagnostics

## Problem

The existing unified logging epic covers viewer/context logging, but `memory-api` domain crates and their CLI/MCP/HTTP transports still have uneven tracing setup and sparse instrumentation. Long-running HTTP servers, MCP tools, validation runs, benchmarks, and agent sessions need logs that can be filtered, searched, tailed while active, and linked back to tickets/specs/tests/sessions.

## Scope

Implement the architecture described by spec `aa769a27` (`memory-api/observability/runtime-logging`) in staged slices:

1. Shared logging facility
   - Choose the owning crate boundary (`log-api` or a small shared crate) and add one initialization/configuration API for memory-system binaries and test/bench harnesses.
   - Support stdout/stderr plus rolling JSONL file sinks, `EnvFilter` directives, non-blocking guards, environment/TOML config, and optional log-api metadata registration.
   - Replace ad hoc `tracing_subscriber::fmt()` setup in memory-api CLI/MCP/HTTP tools.

2. Runtime log-api model
   - Extend `log-api` beyond validation-only captures with runtime log session metadata.
   - Track component, transport, operation/tool/route, workspace/store roots, process/run ids, active/completed state, file locator, format, rotation policy, active filters, and cross-store links.
   - Preserve validation-log links by modeling validation logs as a specialization of runtime log sessions.

3. Domain instrumentation map
   - Audit memory-domain crates and define major operation spans/events for store discovery, scan/index reconciliation, CRUD/query flows, graph/dependency traversal, board updates, move preflight/apply/rollback/resume, validation evidence, and log capture/indexing.
   - Add consistent snake_case fields and level semantics (`error`, `warn`, `info`, `debug`, `trace`).

4. Transport instrumentation
   - Add request/tool lifecycle spans for HTTP and MCP transports and command lifecycle spans for CLI surfaces.
   - Correlate transport ids with domain-operation spans and log session metadata.

5. Live indexing/search
   - Add incremental log-api indexing for active JSONL files using byte offsets and partial-record tolerance.
   - Search/filter by metadata, level, target/module, span name, operation, request id, run id, ticket/spec/test/session links, time range, arbitrary structured fields, and text/regex predicates.
   - Expose shared behavior through CLI, MCP, and HTTP with compact output suitable for agents.

6. Documentation and validation
   - Document configuration, field naming, privacy rules, and examples for tests, benchmarks, long-running servers, and agent sessions.
   - Add tests for shared initialization, metadata persistence, link filtering, active-file incremental indexing, and transport/domain correlation.

## Acceptance criteria

- One DRY tracing initialization path is used by memory-api CLI, MCP, HTTP, tests, benchmarks, and long-running servers.
- `log-api` can register, list, retrieve, link, and search runtime log sessions, while keeping validation-log captures compatible.
- Memory-domain crates have a documented instrumentation map and initial implementation coverage for their major internal operations.
- HTTP/MCP/CLI transports correlate lifecycle spans with domain operations and log sessions.
- Structured JSONL logs can be tailed/indexed while the producing process is still running.
- Search/filter APIs support both metadata and structured-entry queries with compact CLI/MCP output.
- Existing viewer/context logging work is linked rather than duplicated.

## Related work

- Spec: `.spec/specs/aa769a27-2721-4b9d-880c-5c4e2f8136a7`
- Existing logging epic: `.ticket/tickets/def88d4e-8a3c-45bc-82c8-bdacae01a479`
- Context per-command tracing plan: `.ticket/tickets/61f78a57-6896-4ad7-9daa-0e9e805aa397`
- Workflow validation metadata spec: `.spec/specs/a4f48d84-50ed-4769-a42f-38321ea9600c`
