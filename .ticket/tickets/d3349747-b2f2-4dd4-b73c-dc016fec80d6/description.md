# Goal

Extend `log-api` beyond validation-only captures so it can represent runtime log sessions for tools, servers, tests, benchmarks, graph operations, journals, and agent sessions.

## Scope

- Add runtime log session identity and metadata types.
- Track component, transport, operation/tool/route, workspace/store roots, process/run ids, file locator, format, rotation policy, active filters, start/end/status, and byte-offset checkpoints.
- Add links to tickets, specs, docs, validation executions, benchmark ids, agent/session ids, journal ids, and graph operation ids.
- Preserve compatibility for existing validation log capture types.

## Acceptance criteria

- `log-api` can register, retrieve, list, and filter runtime log sessions independently of validation captures.
- Existing validation-log tests continue to pass.
- Metadata can represent active and completed logs.