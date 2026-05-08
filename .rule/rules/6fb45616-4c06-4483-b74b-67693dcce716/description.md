## Primary Interface

- `POST /api/execute` is the universal RPC endpoint and should remain the primary API surface.
- Command payloads must preserve `Command` JSON shape with `"command"` discriminant.
- Optional `"trace": true` behavior should keep parity with current traced execution semantics.
- REST routes under `/api/workspaces/...` are convenience endpoints; maintain consistency with command semantics.