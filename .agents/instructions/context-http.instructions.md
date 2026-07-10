---
description: "Use when editing context-http, the HTTP and optional GraphQL adapter for context-api. Covers RPC dispatch, trace capture, state access, and HTTP error mapping."
applyTo: "tools/context-http/**"
---

<!-- rule-api:file generated=true -->

<!-- rule-api:entry id=161cffd5-5109-4e84-83d0-c0edeb63f654 slug=shared/instructions/context-http/context-http-instructions/l1 -->


<!-- rule-api:entry id=94ab63c0-86ab-449b-a1eb-f2ac581866d3 slug=shared/instructions/context-http/context-http-guidance/architecture/l8 -->
## Architecture

- `context-http` is a thin transport adapter over `context-api`.
- Keep business/domain behavior in `context-api`; keep adapter behavior in `context-http`.

<!-- rule-api:entry id=6fb45616-4c06-4483-b74b-67693dcce716 slug=shared/instructions/context-http/context-http-guidance/primary-interface/l13 -->
## Primary Interface

- `POST /api/execute` is the universal RPC endpoint and should remain the primary API surface.
- Command payloads must preserve `Command` JSON shape with `"command"` discriminant.
- Optional `"trace": true` behavior should keep parity with current traced execution semantics.
- REST routes under `/api/workspaces/...` are convenience endpoints; maintain consistency with command semantics.

<!-- rule-api:entry id=8485f4f3-d2ca-4e2c-bdf1-3813ee1d6325 slug=shared/instructions/context-http/context-http-guidance/state-and-concurrency/l20 -->
## State and Concurrency

- Use `AppState` with `Arc<Mutex<WorkspaceManager>>` patterns for shared mutable access.
- Do not bypass manager locking patterns for command execution paths.
- Keep capture-config resolution (`capture_config_for`) aligned with workspace log-directory behavior.

<!-- rule-api:entry id=cd725b3f-59ac-40ed-a157-b2098c6511a6 slug=shared/instructions/context-http/context-http-guidance/graphql/l26 -->
## GraphQL

- GraphQL is feature-gated (`graphql` feature).
- Keep GraphQL endpoint behavior isolated from core RPC logic.
- Avoid introducing multiple GraphQL libraries or duplicate schema paths.

<!-- rule-api:entry id=d54a07f0-f327-49f7-a4ea-fdb45ab1e415 slug=shared/instructions/context-http/context-http-guidance/error-mapping/l32 -->
## Error Mapping

- Preserve explicit mapping from `ApiError` categories to HTTP status codes.
- Keep response body shape stable for handler errors.
- Add new error kinds only with explicit HTTP status mapping.

<!-- rule-api:entry id=8f3ec0de-73e6-44b2-be7d-209c23d6fca7 slug=shared/instructions/context-http/context-http-guidance/runtime-and-configuration/l38 -->
## Runtime and Configuration

- Respect env-driven configuration (`PORT`, `HOST`, `LOG_LEVEL`, `LOG_FILE`, `CONTEXT_ENGINE_DIR`, optional `STATIC_DIR`).
- Prefer viewer-api tracing and router utilities over ad hoc setup.

<!-- rule-api:entry id=f44089de-9aff-49ce-890e-bb3c31076215 slug=shared/instructions/context-http/context-http-guidance/validation/l43 -->
## Validation

- Run focused adapter tests after route or error-mapping changes.
- Verify both `/api/execute` and relevant REST endpoints for behavior parity.
- If GraphQL code changes, validate feature-gated build/test behavior.
