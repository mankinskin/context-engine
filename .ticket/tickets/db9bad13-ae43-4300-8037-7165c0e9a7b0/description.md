# Goal
Normalize the artifact-routing contract between `test-api` executions, `log-api` runtime sessions, and operation journals so validation runners, benchmark harnesses, and transport diagnostics all emit a shared minimum interoperability set plus the right artifact-specific extensions.

## Why this work exists
The current observability design names correlation ids and runtime-session storage, but it still leaves too much room for implicit per-runner link choices. The repository needs a layered compatibility contract: enough common structure for seamless tooling, but not one universal field bundle that ignores the differences between executions, benchmarks, runtime sessions, and journals.

## Scope
- Define the shared minimum link set for all interoperable durable artifacts involved in tracing, validation, and runtime diagnostics.
- Define artifact-specific required extensions for validation executions, benchmark records, runtime sessions, journal-backed operations, and domain-specific records.
- Add or refine helper APIs in the owning crates so runners do not hand-roll cross-store links.
- Validate the contract against at least one validation-run path and one benchmark or transport-diagnostic path.
- Keep deterministic journal payloads separate from profiling-only metadata.

## Shared minimum interoperability contract

Every interoperable durable artifact in scope should expose, through its native schema or helper API, the smallest shared set that lets tools join records predictably:

- stable record identity in the owning store
- artifact kind or typed record role
- recorded-at timestamp or execution time
- domain and operation identity where the artifact represents an operation outcome
- correlation identity such as `run_id`, runtime-session lineage, or journal operation lineage
- outward links to related tickets, specs, or acceptance criteria when the artifact is compliance evidence

This is the common floor, not a promise that every artifact type stores identical optional fields.

## Artifact-specific required extensions

- Validation executions:
	- outcome, duration, transport, command or producer identity, and required `spec_ids` or `acceptance_criterion_ids`
	- `log_ids` when the execution emitted companion runtime logs
- Benchmark records:
	- `cell_id`, budget status, metric bundle, run grouping, and compliance links
	- companion runtime-log or profiler evidence ids only when emitted by the harness
- Runtime sessions and log captures:
	- capture ids, locator metadata, lifecycle status, and searchable session facts owned by `log-api`
	- links back to executions or journals only as references, not duplicate ownership
- Journal-backed operations:
	- authoritative operation or journal identity, replay or rollback lineage, and deterministic mutation payload ownership
	- links outward to tests or logs when they explain the operation, without moving journal authority elsewhere
- Domain-specific records:
	- any richer link requirements needed by session, replay, or viewer-driven artifacts above the shared minimum set

## Helper API and validation direction

- Shared helper APIs should construct the minimum link set and then require artifact-specific extensions based on the record type.
- Validation paths should fail fast when a required minimum link or required extension is missing for the chosen artifact kind.
- The policy should be testable through at least one existing validation path and one benchmark or transport-diagnostic path.

## Primary anchors
- `memory-api/crates/test-api/src/lib.rs`
- `memory-api/crates/test-api/src/benchmark.rs`
- `memory-api/crates/log-api/src/store.rs`
- root tracker `73b2cd22-942b-4205-86e5-333df2373211`
- completed runtime-session ticket `d3349747-b2f2-4dd4-b73c-dc016fec80d6`

## Acceptance criteria
- One documented layered compatibility contract exists with a shared minimum set and artifact-specific extensions.
- At least one shared helper or validation path prevents missing minimum links or missing artifact-specific required fields where policy requires them.
- The chosen contract is exercised by one existing validation or benchmark path and produces durable evidence in the correct stores.
- The ticket records any remaining blocker from shared tracing initialization or the generic journal envelope explicitly rather than hiding it in caller-specific code.