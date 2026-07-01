# Goal

Define a shared observability contract for `memory-api`, its domain crates, and every CLI, MCP, HTTP, and long-running server transport so internal operations can be traced consistently with the Rust `tracing` ecosystem.

The architecture must make high-signal diagnostics available in files and, as `log-api` matures, in the log store with searchable metadata and cross-store references to tickets, specs, tests, benchmarks, and agent sessions.

# Problem

The repository already has useful tracing pieces, but they are fragmented:

- `context-trace` provides rich tracing utilities and test-log capture for context-domain operations.
- `viewer-api` provides reusable server tracing/file logging for viewer tools.
- Several `memory-api` HTTP and MCP tools initialize `tracing-subscriber` independently.
- `log-api` currently models validation-log captures, not general runtime log sessions or live-indexed diagnostic streams.
- Domain crates emit sparse or inconsistent spans, so complex store, search, move, board, and transport behavior can be hard to reconstruct after a failure.

This makes observability optional and uneven. Long-running HTTP transports are especially difficult to debug because server behavior, domain store decisions, and test/agent context are not consistently correlated.

# Scope

This spec covers the architecture and rollout requirements for:

- `memory-api/crates/{memory-api,ticket-api,spec-api,doc-api,rule-api,audit-api,session-api,test-api,log-api}` and future memory-domain crates.
- `memory-api/tools/cli/*`, `memory-api/tools/mcp/*`, and `memory-api/tools/http/*` transport layers.
- Shared tracing setup, log file creation, log metadata, and log-api indexing/search contracts.
- Correlation metadata for test runs, benchmarks, HTTP request ids, MCP tool calls, CLI invocations, ticket/spec ids, validation execution ids, and agent/session ids.

# Non-goals

- Replacing `tracing` with a custom logging facade.
- Duplicating separate tracing initialization code in every binary.
- Requiring the unfinished `log-api` store before file logging provides value.
- Capturing secrets or full request bodies by default.
- Completing every instrumentation point in one implementation ticket.

# Architecture Direction

## Shared logging facility

Create one shared Rust logging/observability facility for memory-system tools. It may live in `log-api` if that crate owns runtime log metadata, or in a small shared crate if dependency direction requires keeping domain models independent.

The shared facility should provide:

- one initialization API for CLI, MCP, HTTP, tests, benchmarks, and long-running servers
- configuration from environment variables plus optional TOML config
- stdout/stderr and rolling JSONL file sinks
- `EnvFilter` support with module/target-specific granularity
- a non-blocking writer and guard ownership pattern suitable for server lifetimes
- common span fields and event naming conventions
- helpers/macros for domain-operation spans where they reduce repetition
- optional log-api metadata registration when a log store is available

Existing ad hoc subscribers in memory-api tools should be replaced by this shared facility. Existing `viewer-api::init_tracing_full` and `context-trace` test tracing should be treated as design inputs; avoid copy-pasting their internals into every domain transport.

## Log sessions and metadata

A log file should be represented as a log session with stable metadata, not just an anonymous path. Required metadata includes:

- log id and session/run id
- component/domain and transport (`cli`, `mcp`, `http`, `test`, `bench`, `agent-session`, or `in-process`)
- operation/tool/route where applicable
- workspace root and store root where safe to record
- process id, started_at, ended_at when known, and current status for live logs
- file locator, media type, format, rotation policy, and byte offsets/checkpoints
- filter directives active for the session
- cross-store links to tickets, specs, docs, validation specs/executions, benchmark ids, and agent/session ids

`log-api` should evolve from validation-only captures into a general runtime-log store that can still specialize validation logs through links to `test-api` executions.

## Instrumentation contract

Domain crates should instrument major internal operations with structured spans/events, including:

- workspace/store discovery and resolution
- file reads/writes, scans, index reconciliation, and generated sidecar operations
- ticket/spec/doc/rule/audit/session/test/log create/read/update/delete/query flows
- graph traversal, dependency checks, board state changes, and move preflight/apply/rollback/resume flows
- search/index queries and result counts
- validation execution recording and evidence/link resolution
- transport dispatch, request/tool lifecycle, status codes, durations, and error classification

Granularity should be level-based:

- `error`: operation failed or state may be inconsistent
- `warn`: degraded behavior, stale data, skipped records, validation warnings
- `info`: lifecycle boundaries, high-level operation outcomes, request/tool summaries
- `debug`: branch decisions, counts, resolved paths, IDs, query predicates, elapsed timings
- `trace`: per-record or tight-loop details, enabled only with targeted filters

Events must use stable snake_case field names and avoid high-cardinality or sensitive data unless explicitly requested at `trace` level.

## Live indexing and search

The first implementation can write JSONL files immediately. `log-api` should then add indexing/search that can operate on both completed and active log files by tracking offsets and partial ingestion state.

Required capabilities:

- record and list log sessions with metadata
- attach/update cross-store links after capture
- tail active JSONL files without corrupting partial records
- incrementally index new entries by file offset
- filter by level, target/module, span name, operation, request id, run id, ticket/spec/test/session links, time range, and arbitrary structured fields
- support text search and structured query predicates
- expose equivalent shared-library behavior through CLI, MCP, and HTTP transports

# Acceptance Criteria

- A single shared initialization/configuration path exists for memory-api CLI, MCP, HTTP, tests, benchmarks, and long-running servers; new transports do not hand-roll subscribers.
- Every memory-domain crate has an instrumentation map identifying its major operations, required span targets, standard fields, and expected result events.
- HTTP and MCP transports emit request/tool lifecycle spans that correlate transport work with domain-store operations.
- Log files are written as structured JSONL with stable metadata and level/target filters.
- `log-api` can represent runtime log sessions, not only validation-log captures, and can link logs to test runs, benchmarks, tickets, specs, docs, and agent sessions.
- Active log files can be tailed or incrementally indexed without waiting for process exit.
- Search/filter APIs work over log metadata and structured log entries, with compact CLI/MCP output for agent workflows.
- Documentation explains the shared field naming convention, privacy rules, and how to enable/disable file and store capture.

# Traceability

- Owning tracker: `.ticket/tickets/73b2cd22-942b-4205-86e5-333df2373211`
- Related tracker: `.ticket/tickets/def88d4e-8a3c-45bc-82c8-bdacae01a479`
- Related context trace plan: `.ticket/tickets/61f78a57-6896-4ad7-9daa-0e9e805aa397`
- Related workflow validation spec: `.spec/specs/a4f48d84-50ed-4769-a42f-38321ea9600c`
