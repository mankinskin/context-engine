# Impl: ticket serve mode (HTTP + auth + workspace-aware ticket endpoints)

**Wave 1 / Track C** | Component: `context-tasks`

## Design inputs
- API contract: `21a1b9ca/assets/design/api-contract-v0.1.md`
- Auth lifecycle: `68dfc679/assets/design/auth-lifecycle-v0.1.md`

## Objective
Add a `ticket serve` subcommand to the `ticket` CLI that starts an Axum HTTP
server exposing the REST API defined in `api-contract-v0.1.md`. The server must:
- Require bearer-token auth on all `/api/*` routes
- Support workspace selection via query param
- Return structured error envelopes on 4xx/5xx
- Expose `GET /healthz` without auth

## Implementation plan

### Step 1 — Cargo dependencies
- Add `axum`, `tower`, `tower-http` (for auth middleware), `tokio` (multi-thread runtime)
  to `crates/context-tasks/Cargo.toml`
- Add `serde_json` if not already present

### Step 2 — `serve/mod.rs` module structure
Create `crates/context-tasks/src/serve/`:
```
serve/
  mod.rs        — exports, AppState, serve() entry
  routes.rs     — route table, router builder
  auth.rs       — bearer token middleware (Tower layer)
  handlers/
    health.rs   — GET /healthz
    workspaces.rs — GET /api/workspaces
    tickets.rs  — GET /api/tickets, GET /api/tickets/{id}
    edges.rs    — GET /api/edges
    graph.rs    — GET /api/graph/subgraph
    stream.rs   — GET /api/stream (SSE stub — wired by 5e68c2e1)
  error.rs      — error envelope type, IntoResponse impl
```

### Step 3 — AppState
```rust
#[derive(Clone)]
pub struct AppState {
    pub store: Arc<TicketStore>,
    pub workspace_registry: Arc<WorkspaceRegistry>,
    pub auth: Arc<AuthState>,
}
```
`AuthState` is implemented by `00ee9f46`.

### Step 4 — Bearer auth middleware
- Extract `Authorization: Bearer <token>` header
- Compare against `AuthState::token_set()`
- Return `401` JSON error envelope on mismatch
- Pass `workspace_id` claim in request extensions

### Step 5 — Route handlers
Implement each handler per the API contract response shapes:
- Pagination: cursor support on `/api/tickets` and `/api/graph/subgraph`
- `request_id`: generate `uuid::Uuid::new_v4()` per request
- `workspace` validation: 404 if unknown workspace

### Step 6 — `ticket serve` CLI command
In `cli.rs`:
```
ticket serve [--port 8080] [--host 0.0.0.0] [--workspace <name>]
```
- Read token from `TICKET_SERVE_TOKEN` env var (or config file)
- Start tokio runtime, call `serve::serve(config, store)`

### Step 7 — Integration tests
- `tests/integration_serve_healthz.rs`: start server on random port, `GET /healthz`
- `tests/integration_serve_auth.rs`: 401 on missing token, 200 with valid token
- `tests/integration_serve_tickets.rs`: list/get round-trip

## Acceptance criteria
- [ ] `ticket serve --port 8080` starts and serves `/healthz`
- [ ] All `/api/*` routes require valid bearer token
- [ ] Ticket list/get/edges return correct shapes from live store
- [ ] Workspace selection enforced on all data endpoints
- [ ] Structured error envelope returned on 4xx/5xx
- [ ] Integration tests: healthz, auth, tickets round-trip

## Handoff to
- `5e68c2e1` — SSE stream route stub (`GET /api/stream`) must exist before SSE impl wires it
- `00ee9f46` — Auth token reload plugs into `AuthState`
- `a1259318` — viewer-api extraction may refactor shared server infra
