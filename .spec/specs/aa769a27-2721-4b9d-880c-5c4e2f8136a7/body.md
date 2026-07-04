# Goal

Define a shared observability contract for `memory-api`, its domain crates, context-stack graph operations, and every CLI, MCP, HTTP, benchmark, test, and long-running server transport so internal operations can be traced consistently with the Rust `tracing` ecosystem.

The architecture must make high-signal diagnostics available in files and, as `log-api` matures, in the log store with searchable metadata and cross-store references to tickets, specs, tests, benchmarks, journals, graph visualizations, and agent sessions.

# Problem

The repository already has useful tracing and journaling pieces, but they are fragmented:

- `context-trace` provides graph visualization events and test-log capture for context-domain operations.
- `context-api` already has per-command trace capture and log parsing/query commands.
- `viewer-api` provides reusable server tracing/file logging for viewer tools.
- `memory-api` has a domain-neutral cross-store move kernel with `MovePlan`, `MoveJournal`, `resume_move`, and `rollback_move`, but this journal model is scoped to move operations.
- Several `memory-api` HTTP and MCP tools initialize `tracing-subscriber` independently.
- `log-api` currently models validation-log captures, not general runtime log sessions, operation journals, or live-indexed diagnostic streams.
- Domain crates emit sparse or inconsistent spans, so complex store, search, move, board, graph, and transport behavior can be hard to reconstruct after a failure.

This makes observability optional and uneven. Long-running HTTP transports are especially difficult to debug because server behavior, domain store decisions, journaled operation state, graph replay state, and test/agent context are not consistently correlated.

# Scope

This spec covers the architecture and rollout requirements for:

- `memory-api/crates/{memory-api,ticket-api,spec-api,doc-api,rule-api,audit-api,session-api,test-api,log-api}` and future memory-domain crates.
- `context-stack` graph operation logs and replayable graph-operation journals where they feed log-viewer visualizations.
- `memory-api/tools/cli/*`, `memory-api/tools/mcp/*`, and `memory-api/tools/http/*` transport layers.
- Shared tracing setup, log file creation, log metadata, operation journal metadata, and log-api indexing/search contracts.
- Correlation metadata for test runs, benchmarks, HTTP request ids, MCP tool calls, CLI invocations, ticket/spec ids, validation execution ids, graph operation ids, journal ids, and agent/session ids.

# Non-goals

- Replacing `tracing` with a custom logging facade.
- Duplicating separate tracing initialization code in every binary.
- Requiring the unfinished `log-api` store before file logging provides value.
- Treating trace logs as the durable source of truth for reversible state changes.
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

Existing ad hoc subscribers in memory-api tools should be replaced by this shared facility. Existing `viewer-api::init_tracing_full`, `context-api::tracing_capture`, and `context-trace` test tracing should be treated as design inputs; avoid copy-pasting their internals into every domain transport.

## Logs, journals, and visualization events

The design separates three related artifacts:

- **Trace logs** observe runtime behavior. They are append-only diagnostic records with timing, span hierarchy, levels, targets, errors, and structured fields. They are ideal for search, profiling, and debugging, but not sufficient as the durable source of truth for reversible operations.
- **Operation journals** describe planned and applied state transitions. They are deterministic ledgers with preflight inputs, validation blockers, ordered steps, phase transitions, rollback/resume metadata, touched files/entities, and recovery instructions. They are the source of truth for replayable or revertable operations.
- **Visualization events** describe UI replay state. They may be emitted through tracing logs for live observation, but should have stable operation/journal ids so log-viewer can replay them from log files, journals, or derived indexes.

The current move kernel's `MovePlan`/`MoveJournal` proves the viability of journaled operation execution for cross-store file mutations. The broader design should generalize that pattern into an `OperationJournal` model without weakening the existing move journal compatibility.

## Operation journaling contract

A generalized journal should support:

- read-only preflight planning before mutation
- explicit blockers and warnings
- ordered planned steps with stable ids
- phase transitions (`planned`, `locked`, `applied_step`, `validated`, `completed`, `failed`, `rolled_back`, or domain-specific refinements)
- touched entities, touched files, and previous content or inverse operations where rollback is supported
- resume and rollback entrypoints where the operation is reversible
- replay entrypoints for non-mutating visualizations and postmortems
- links to trace log sessions, benchmark/profile runs, tickets, specs, tests, docs, and agent/session ids

Not every journal is rollbackable. The journal schema must represent reversibility explicitly:

- `replayable`: can reconstruct what happened or what would happen
- `rollbackable`: can mechanically restore previous state
- `manual_recovery`: requires human follow-up but records enough context to guide it

## Log sessions and metadata

A log file should be represented as a log session with stable metadata, not just an anonymous path. Required metadata includes:

- log id and session/run id
- component/domain and transport (`cli`, `mcp`, `http`, `test`, `bench`, `agent-session`, or `in-process`)
- operation/tool/route where applicable
- workspace root and store root where safe to record
- process id, started_at, ended_at when known, and current status for live logs
- file locator, media type, format, rotation policy, and byte offsets/checkpoints
- filter directives active for the session
- related operation journal ids and graph operation ids
- cross-store links to tickets, specs, docs, validation specs/executions, benchmark ids, and agent/session ids

`log-api` should evolve from validation-only captures into a general runtime-log store that can still specialize validation logs through links to `test-api` executions.

## Cross-store correlation-id contract

The observability contract uses four canonical identifiers to correlate transport spans, logs, journals, replay streams, and benchmark evidence without coupling deterministic replay payloads to profiling-only fields.

| Field | Semantics | Producer | Consumers | Cardinality/lifecycle |
|---|---|---|---|---|
| `operation_id` | Stable id for one logical operation execution (`scan`, `move_apply`, `search`, replay apply). | Operation orchestrator in memory/context domain code. | Transport spans, log session metadata, operation journals, graph replay streams. | One per operation attempt. New value on retry/resume attempt unless resume policy explicitly preserves lineage with `journal_id`. |
| `run_id` | Correlates all operations and transports in one benchmark/test/agent session or process run. | Transport/session bootstrap layer (CLI/MCP/HTTP/test/bench harness). | Log session index, benchmark reports, validation evidence links, cross-operation analytics. | One per run/session boundary. Reused across many `operation_id` values in the same run. |
| `journal_id` | Durable identity of an operation journal ledger record. | Journal store on preflight/create. | Resume/rollback/replay flows, log metadata linkage, evidence queries. | One per journal artifact, stable across apply/resume/rollback events for that journal. |
| `session_id` | External or user-facing session identity (agent session, validation execution session, viewer session). | Session manager / workflow layer. | Ticket/spec/test/log cross-store references, agent handoff correlation. | One per session scope, may span multiple runs depending on workflow policy. |

### Required-field matrix by subsystem

| Surface | Required ids | Notes |
|---|---|---|
| CLI command span/session metadata | `run_id`, `operation_id` | Attach `session_id` when invoked under managed agent/session workflow. |
| MCP tool lifecycle spans | `run_id`, `operation_id`, `session_id` | `session_id` should map to tool session or orchestrator session. |
| HTTP request spans | `run_id`, `operation_id` | If request/session cookie exists, map to `session_id`. |
| Operation journal envelopes | `journal_id`, `operation_id`, `run_id` | `session_id` optional unless operation is session-scoped. |
| Log session metadata rows | `run_id` | Include `operation_id` and `journal_id` when session is operation-bound. |
| Graph replay event streams | `operation_id`, `run_id` | Add `journal_id` when replay stream is derived from a journaled operation. |
| Benchmark evidence/report rows | `run_id`, `operation_id` | Include `session_id` when benchmark is tied to managed validation sessions. |

### Failure and absence handling

- Missing required identifiers in a required surface are validation failures for that surface and must be reported as explicit gaps in ticket/spec evidence.
- Fallback id generation is allowed only at transport/session boundaries; downstream layers must not silently invent replacement ids when an upstream required id is absent.
- Replay payloads must keep deterministic state fields timing-free. Correlation ids are allowed in replay/journal envelopes because they are identity metadata, not timing/profile measurements.
- Resume/rollback flows may keep `journal_id` stable while rotating `operation_id` for each execution attempt, but the relationship must be recorded in journal phase metadata.

## Instrumentation contract

Domain crates should instrument major internal operations with structured spans/events, including:

- workspace/store discovery and resolution
- file reads/writes, scans, index reconciliation, and generated sidecar operations
- ticket/spec/doc/rule/audit/session/test/log create/read/update/delete/query flows
- graph traversal, dependency checks, board state changes, and move preflight/apply/rollback/resume flows
- operation journal preflight/apply/resume/rollback/replay flows
- search/index queries and result counts
- validation execution recording and evidence/link resolution
- benchmark/profiling run boundaries and measured durations
- transport dispatch, request/tool lifecycle, status codes, durations, and error classification

Returned profiling artifacts such as `phase_timings_ms`, root-entry count maps, and benchmark summaries remain useful as deterministic test/bench evidence, but they are not a substitute for tracing. When an operation already measures internal phases for reports or journals, the same phase boundaries should also emit tracing spans or completion events so transport logs, log-api indexing, and future correlation tooling can observe the work without parsing return values.

For profile-sensitive store operations such as `open_or_init`, `scan`, and journaled move apply/resume/rollback, prefer one outer `info` span per operation, nested `debug` spans/events per phase or root, stable completion events with counts and elapsed timing fields, and reserve per-record details for `trace` level only.

Granularity should be level-based:

- `error`: operation failed or state may be inconsistent
- `warn`: degraded behavior, stale data, skipped records, validation warnings
- `info`: lifecycle boundaries, high-level operation outcomes, request/tool summaries
- `debug`: branch decisions, counts, resolved paths, IDs, query predicates, elapsed timings
- `trace`: per-record or tight-loop details, enabled only with targeted filters

Events must use stable snake_case field names and avoid high-cardinality or sensitive data unless explicitly requested at `trace` level.

## Context-stack graph replay

`GraphOpEvent` already carries step, operation type, transition, path id, path graph snapshots, and graph mutation deltas through tracing logs for log-viewer. This is viable as a visualization event format, but not yet a complete operation journal.

The graph journaling design should define a replayable format that can:

- group graph events by operation id/path id/run id
- preserve deterministic step ordering
- distinguish observation-only search/read events from mutation-capable insert/update events
- carry graph deltas and optional snapshots for UI replay and validation
- link each graph replay stream to a trace log session and, when relevant, an operation journal
- support benchmark/profiling timing without polluting the replay state

## Live indexing and search

The first implementation can write JSONL files immediately. `log-api` should then add indexing/search that can operate on completed and active log files by tracking offsets and partial ingestion state.

Required capabilities:

- record and list log sessions with metadata
- record and list operation journals with metadata
- attach/update cross-store links after capture
- tail active JSONL files without corrupting partial records
- incrementally index new entries by file offset
- filter by level, target/module, span name, operation, request id, run id, ticket/spec/test/session links, journal id, graph operation id, time range, and arbitrary structured fields
- support text search and structured query predicates
- expose equivalent shared-library behavior through CLI, MCP, and HTTP transports

# Viability Assessment

The requirements are viable if the implementation keeps the artifact boundaries strict:

- Reuse `tracing` for observation and timing.
- Reuse and generalize the move kernel's journal concepts for planned/reversible operations.
- Treat context-stack `GraphOpEvent` as a visualization stream that can be indexed and linked, not as a replacement for operation journals.
- Let `log-api` own metadata, indexing, search, and cross-store links rather than duplicating this in every transport.

Primary risks:

- Overloading log files as rollback journals would make recovery unreliable.
- A shared logging crate can create dependency cycles if it depends on domain crates.
- High-volume `trace` events from graph operations can overwhelm live indexing without sampling, filters, or bounded retention.
- File paths, request payloads, and benchmark traces can leak sensitive or unstable machine-local data if redaction rules are not explicit.
- Replaying mutable graph operations requires deterministic deltas and schema versioning; snapshots alone are useful for visualization but weak for validation.
- Long-running servers need guard ownership and shutdown flushing; leaking guards or losing them can drop logs.

# Open Decisions

- Owning crate: should shared runtime logging live inside `log-api`, `memory-api`, or a small dependency-light crate used by both?
- Journal store: should generalized operation journals live under `.log`, each domain store, or a new `.journal` area indexed by `log-api`?
- Schema versioning: what stable envelope version covers log entries, operation journals, and graph replay events?
- Reversibility: which memory-store operations are rollbackable by design, and which are replay/manual-recovery only?
- Context-stack boundary: should graph journals live in `context-api` as domain artifacts, or be normalized into `log-api` only after capture?
- Retention: what default retention, rotation, and sampling policies apply to active server logs and graph trace streams?
- Privacy: what fields are always redacted or hashed by the shared facility?

# Acceptance Criteria

- A single shared initialization/configuration path exists for memory-api CLI, MCP, HTTP, tests, benchmarks, and long-running servers; new transports do not hand-roll subscribers.
- Logs, operation journals, and visualization events have distinct schemas and explicit links between them.
- A generalized operation journal contract exists, derived from the move kernel, with preflight, apply, resume, rollback, replay, and reversibility semantics.
- Every memory-domain crate has an instrumentation map identifying its major operations, required span targets, standard fields, expected result events, and journal requirements.
- Context-stack graph operations have a replayable graph event/journal contract suitable for log-viewer visualization.
- HTTP and MCP transports emit request/tool lifecycle spans that correlate transport work with domain-store operations and journal ids.
- Log files are written as structured JSONL with stable metadata and level/target filters.
- `log-api` can represent runtime log sessions and operation journal metadata, not only validation-log captures, and can link logs to test runs, benchmarks, tickets, specs, docs, journals, graph operations, and agent sessions.
- Active log files can be tailed or incrementally indexed without waiting for process exit.
- Search/filter APIs work over log metadata, journal metadata, and structured log entries, with compact CLI/MCP output for agent workflows.
- Documentation explains the shared field naming convention, journal/replay semantics, privacy rules, and how to enable/disable file and store capture.

# Traceability

- Owning tracker: `.ticket/tickets/73b2cd22-942b-4205-86e5-333df2373211`
- Related tracker: `.ticket/tickets/def88d4e-8a3c-45bc-82c8-bdacae01a479`
- Related context trace plan: `.ticket/tickets/61f78a57-6896-4ad7-9daa-0e9e805aa397`
- Related move kernel spec: `memory-api/.spec/specs/afcaccc9-5577-4556-ab6f-cfbe7a77e430`
- Related move kernel ticket: `memory-api/.ticket/tickets/0a510279-5482-4c4f-8dda-6d333dc1f222`
- Related journaled move execution ticket: `memory-api/.ticket/tickets/bc691249-5a2d-409e-8e7b-2602d80cf61e`
- Related workflow validation spec: `.spec/specs/a4f48d84-50ed-4769-a42f-38321ea9600c`
- Related benchmarking/profiling plan: `.spec/specs/c598ddb2-4d3a-4b81-90ea-8b25a54b8469`
- Correlation-id contract implementation ticket: `.ticket/tickets/529844ac-f7e5-4265-b087-5bd2b597155f`
- Profiling evidence checklist tracker: `.ticket/tickets/ff6637f5-01f6-46c3-b727-e1a19ee0f202`
