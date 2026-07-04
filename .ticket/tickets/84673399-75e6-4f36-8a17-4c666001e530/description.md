# Goal

Resolve the core architecture decisions for unified logging, operation journaling, and replayable visualization before implementation proceeds.

## Scope

- Decide owning crate boundaries for shared tracing runtime, log-api metadata, and generalized operation journals.
- Decide whether journals live under `.log`, domain stores, or a dedicated `.journal` layout.
- Define schema-versioning strategy for logs, journals, and graph replay events.
- Classify operations as replayable, rollbackable, or manual-recovery only.
- Document dependency-direction constraints so `log-api`, `memory-api`, context-stack, and viewer tooling stay DRY without cycles.

## Placement decision

Keep this ticket in the root `context-engine` workspace as the lowest common ancestor. Its output governs specialized follow-on work in lower crates; do not move it into `memory-api`, `log-api`, or `context-stack`.

## Specialized follow-on tickets to link from here

- memory-api: `756fed27`, `6c859ac3`, `2e41c96d`, `35cd05c1`, `3041d7e3`
- log-api: `d3349747`, `aa94d02e`
- context-stack: `1dffcf23`

## Implementation tracks (profiling-informed)

1. Artifact boundary contract (log vs journal vs replay)
- Produce ADR-level boundary table covering payload shape, mutability, retention, and ownership.
- Confirm deterministic replay state excludes timing-only fields.

2. Correlation contract across layers
- Define canonical `operation_id`, `run_id`, `journal_id`, and `session_id` propagation rules.
- Require these ids in transport spans (CLI/MCP/HTTP), journal records, and log metadata.

3. Tracing and profiling semantic contract
- Standardize phase naming for store/move/scan workflows.
- Define mandatory span fields and minimum event set for phase starts/completions.

4. Evidence contract and storage boundaries
- Define where profiling artifacts live and how they link to ticket/spec evidence.
- Keep benchmark timing summaries queryable without coupling to replay payloads.

## Current evidence snapshot informing this ticket

- Existing profiling evidence from ticket `49bbe3ae` shows dominant cost in integration and workflow recompute, with scan-root walking negligible.
- Open/init profiling now reports phase maps (`open_or_init_total_ms`, bootstrap and scan sub-phases) and scan reports include per-root timing/count telemetry.
- Tracing integration for open/init/scan phase boundaries is present in `ticket-api` store paths and exercised in e2e/bench runs.

## Architecture-level observability gaps to close

- No finalized cross-store schema for correlating run-level profiling evidence with journal and log sessions.
- No explicit contract for distribution metrics (`p50/p95/p99`) and regression-threshold evidence.
- No repository-level decision on profile evidence persistence/retention separate from deterministic journals.
- Incomplete guidance for redaction/privacy constraints on high-cardinality trace fields.

## Acceptance criteria

- Open decisions from spec `aa769a27` are answered or narrowed to explicit follow-up tickets.
- A short architecture decision record lists artifact boundaries: trace log vs operation journal vs visualization event.
- The tracker roadmap can proceed without ambiguous ownership for shared facilities.

## Acceptance criteria addendum (implementation-ready)

- A published correlation-id contract defines required ids and propagation points across logs, journals, replay events, and transport spans.
- A published phase taxonomy defines canonical phase keys for profiling/tracing and maps each key to owning subsystem.
- Architecture docs explicitly separate deterministic replay payloads from non-deterministic profiling evidence.
- A retention/redaction decision is documented for profiling metadata and high-volume tracing fields.