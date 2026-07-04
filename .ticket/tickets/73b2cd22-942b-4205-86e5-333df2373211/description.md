# Memory-system observability and log-api runtime diagnostics

## Problem

The existing unified logging epic covers viewer/context logging, but `memory-api` domain crates and their CLI/MCP/HTTP transports still have uneven tracing setup and sparse instrumentation. Long-running HTTP servers, MCP tools, validation runs, benchmarks, graph operations, journals, and agent sessions need logs that can be filtered, searched, tailed while active, and linked back to tickets/specs/tests/sessions.

The design must also incorporate operation journaling. `memory-api` already proves the pattern through the cross-store move kernel (`MovePlan`, `MoveJournal`, `resume_move`, `rollback_move`), while `context-stack` graph operations already emit replay-oriented `GraphOpEvent` visualization data. The full solution should unify these concepts without mixing their responsibilities.

## Viability assessment

The requirements are viable if the architecture keeps three artifacts distinct:

- **Trace logs**: runtime observation, timings, spans, levels, errors, and structured fields.
- **Operation journals**: deterministic preflight/apply/resume/rollback/replay ledgers for state-changing or replayable operations.
- **Visualization events**: UI replay streams such as context-stack graph operation events, linked to logs and journals by operation id.

The most reliable shape is DRY but layered:

1. A shared tracing initializer handles sinks, filters, guards, and log-api registration.
2. `log-api` owns runtime log-session metadata, active indexing/search, and cross-store links.
3. A generic operation-journal contract grows from the existing move kernel semantics.
4. Context-stack graph replay gets a schema that can be viewed in log-viewer and linked back to logs/journals.
5. Memory-domain crates add instrumentation only after field names, journal expectations, and privacy rules are settled.

## Placement decision

Keep this tracker in the root `context-engine` workspace as the lowest common ancestor. It coordinates specialized tickets in `memory-api`, `log-api`, and `context-stack`; it should link those child-owned tickets rather than be moved into a lower crate workspace.

## Key risks

- Treating logs as rollback journals would make recovery unreliable.
- Putting shared tracing in the wrong crate can create dependency cycles.
- High-volume graph `trace` events can overwhelm live indexing without filters, sampling, and retention policy.
- Machine-local paths and request payloads can leak sensitive data unless redaction rules are explicit.
- Graph replay needs deterministic deltas and schema versioning; snapshots alone are useful for UI replay but weak for validation.
- Long-running servers must retain non-blocking logging guards and flush on shutdown.

## Open decisions

- Shared runtime owner: `log-api`, `memory-api`, or a small dependency-light crate?
- Journal storage layout: `.log`, domain-local stores, or a dedicated `.journal` area?
- Schema envelope: one versioned envelope for logs, journals, and graph replay, or separate versions linked by ids?
- Reversibility policy: which operations are rollbackable versus replay-only/manual-recovery?
- Context-stack boundary: canonical graph replay in `context-api`, `log-api`, or both through metadata references?
- Retention and sampling defaults for active server logs and high-volume graph traces.
- Privacy/redaction rules for paths, payloads, ids, and benchmark/profile captures.

## Roadmap

Phase 0: resolve architecture boundaries

- [84673399](.ticket/tickets/84673399-75e6-4f36-8a17-4c666001e530/ticket.toml) `[observability] Resolve logging, journaling, and replay architecture boundaries`

Phase 1: define core schemas and shared runtime

- [d3349747](.ticket/tickets/d3349747-b2f2-4dd4-b73c-dc016fec80d6/ticket.toml) `[log-api] Add runtime log session model and cross-store links`
- [6c859ac3](.ticket/tickets/6c859ac3-14c9-4d9d-b428-5b0cca03e23a/ticket.toml) `[journal] Define generic operation journal schema and store contract`
- [756fed27](.ticket/tickets/756fed27-96b3-4572-a986-a4f70986984a/ticket.toml) `[memory-api] Extract shared tracing initialization for all transports`

Phase 2: adapt proven journaling and graph replay

- [35cd05c1](.ticket/tickets/35cd05c1-45f7-4d65-b943-7c000570928f/ticket.toml) `[journal] Adapt move kernel journals to the generic operation-journal envelope`
- [1dffcf23](.ticket/tickets/1dffcf23-8a95-4f45-8163-27e4e58048c7/ticket.toml) `[context-stack] Define replayable graph-operation journal format for log-viewer`

Phase 3: instrument domains and transports

- [2e41c96d](.ticket/tickets/2e41c96d-fe9f-4cf2-b941-6f0d452f237c/ticket.toml) `[memory-api] Create domain instrumentation and journaling coverage map`
- [3041d7e3](.ticket/tickets/3041d7e3-2b34-4597-b354-e0aa6ffb0459/ticket.toml) `[transports] Correlate CLI/MCP/HTTP spans with log sessions and journals`
- [cc78d33d](.ticket/tickets/cc78d33d-1744-4945-bb77-f0fd1142568e/ticket.toml) `[memory-matrix] Capture subprocess failure bundles for transport-cell diagnostics`

Phase 4: indexing, benchmark evidence, and validation

- [aa94d02e](.ticket/tickets/aa94d02e-9620-4db6-9974-36699cd56537/ticket.toml) `[log-api] Add live indexing and search for active logs and journals`
- [ff6637f5](.ticket/tickets/ff6637f5-01f6-46c3-b727-e1a19ee0f202/ticket.toml) `[benchmarks] Capture profiling timings through logs and journals`
- [bce26d30](.ticket/tickets/bce26d30-0a79-40b4-812a-c14b4a246de5/ticket.toml) `[docs-tests] Validate unified logging and journaling architecture end to end`

## Specialized lower-crate work linked from this LCA tracker

- memory-api: [756fed27](.ticket/tickets/756fed27-96b3-4572-a986-a4f70986984a/ticket.toml), [3041d7e3](.ticket/tickets/3041d7e3-2b34-4597-b354-e0aa6ffb0459/ticket.toml), [6c859ac3](.ticket/tickets/6c859ac3-14c9-4d9d-b428-5b0cca03e23a/ticket.toml), [2e41c96d](.ticket/tickets/2e41c96d-fe9f-4cf2-b941-6f0d452f237c/ticket.toml), [35cd05c1](.ticket/tickets/35cd05c1-45f7-4d65-b943-7c000570928f/ticket.toml)
- log-api: [d3349747](.ticket/tickets/d3349747-b2f2-4dd4-b73c-dc016fec80d6/ticket.toml), [aa94d02e](.ticket/tickets/aa94d02e-9620-4db6-9974-36699cd56537/ticket.toml)
- context-stack: [1dffcf23](.ticket/tickets/1dffcf23-8a95-4f45-8163-27e4e58048c7/ticket.toml)

Matrix subprocess failure triage gap status:

- Covered by [cc78d33d](.ticket/tickets/cc78d33d-1744-4945-bb77-f0fd1142568e/ticket.toml), which adds explicit failure-bundle capture (process invocation context, output tails, error class, and linkage ids) for fast MCP/HTTP transport debugging.
- Improvement pass (2026-07-04): expanded deterministic confidence with subprocess spawn-failure coverage, parse/decode guard tests, sentinel-id mismatch checks, env-selector whitelist redaction assertions, bounded tail checks, and persisted execution/run-id correlation assertions.

Blocker coordination snapshot for this track:

- [60a2a388](memory-viewers/.ticket/tickets/60a2a388-c8b6-4e25-a80a-0ba686f11bf9/ticket.toml) remains `new` and continues to gate file-logging parity for viewer surfaces.
- [12197242](memory-api/.ticket/tickets/12197242-b7b4-4212-83a8-4b0b65a4bd7b/ticket.toml) remains `new` and continues to gate full field-name normalization compatibility in downstream log-viewer flows.

## Related evidence and design anchors

- Spec: `.spec/specs/aa769a27-2721-4b9d-880c-5c4e2f8136a7`
- Existing logging epic: `.ticket/tickets/def88d4e-8a3c-45bc-82c8-bdacae01a479`
- Context per-command tracing plan: `.ticket/tickets/61f78a57-6896-4ad7-9daa-0e9e805aa397`
- Move kernel spec: `memory-api/.spec/specs/afcaccc9-5577-4556-ab6f-cfbe7a77e430`
- Move kernel implementation ticket: `memory-api/.ticket/tickets/0a510279-5482-4c4f-8dda-6d333dc1f222`
- Journaled move execution ticket: `memory-api/.ticket/tickets/bc691249-5a2d-409e-8e7b-2602d80cf61e`
- Benchmarking/profiling plan: `.spec/specs/c598ddb2-4d3a-4b81-90ea-8b25a54b8469`

## Acceptance criteria

- One DRY tracing initialization path is used by memory-api CLI, MCP, HTTP, tests, benchmarks, and long-running servers.
- `log-api` can register, list, retrieve, link, and search runtime log sessions, while keeping validation-log captures compatible.
- Operation journaling has a generic schema and store contract that can represent move journals, graph replay streams, and future memory-store operations.
- Context-stack graph operations have a replayable format usable by log-viewer.
- Memory-domain crates have a documented instrumentation and journaling coverage map.
- HTTP/MCP/CLI transports correlate lifecycle spans with domain operations, log sessions, and journal ids.
- Structured JSONL logs can be tailed/indexed while the producing process is still running.
- Search/filter APIs support metadata, structured-entry, journal, graph-operation, and benchmark/profile queries with compact CLI/MCP output.
- Existing viewer/context logging work is linked rather than duplicated.