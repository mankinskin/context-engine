# SSE Contract Test Plan v0.1

Status: draft for review

## Goals
- Ensure event names and payload shape remain stable.
- Validate ordering and duplicate tolerance assumptions.
- Ensure conflict/diagnostic events are emitted under failure conditions.

## Test matrix
1. `ticket.upsert`
- Trigger create/update.
- Assert required fields and valid state value.

2. `ticket.delete`
- Trigger delete.
- Assert deleted payload fields.

3. `edge.upsert` / `edge.delete`
- Trigger link/unlink operations.
- Assert edge tuple and kind.

4. `ticket.conflict`
- Simulate optimistic version mismatch.
- Assert expected/observed revision metadata.

5. `snapshot.ready`
- Trigger reconcile path.
- Assert node/edge count fields.

6. `diagnostic.warning`
- Simulate hook emission failure.
- Assert warning code and message.

## Semantics tests
- Ordering is preserved for same workspace stream.
- Duplicate events may appear; client idempotency path validated.
- No replay behavior in v1 is explicitly documented and tested.

## Automation shape
- Integration tests in `context-tasks` server mode.
- Golden JSON fixtures for payload schema snapshots.
- Compatibility gate in CI to detect contract drift.
