# Impl: remove inbound HTTP auth from ticket serve

## Refined Design (post-interview)

> **Key insight:** The bearer token is not for protecting the ticket HTTP API.
> The ticket store is local developer data — it needs no access control.
> The token is only ever needed when the backend makes **outbound calls to
> agent APIs** (GitHub Copilot, Claude, OpenAI).  That feature does not exist
> yet.  This ticket removes the premature auth gate from the HTTP layer entirely.

---

## What changes

### 1. Remove inbound HTTP authentication from all routes

`routes.rs` currently wraps every `/api/*` route in `bearer_auth_mw`.  Remove
that layer entirely.  All endpoints become publicly accessible — no
`Authorization` header required.

### 2. Remove `AuthState` from `AppState` and `serve()`

`AppState` currently holds `auth: Arc<AuthState>`.  Remove the field.

`serve()` currently takes `auth: AuthState` as a parameter.  Remove it:

```rust
// Before
pub async fn serve(config: ServeConfig, registry: WorkspaceRegistry, auth: AuthState) -> ...

// After
pub async fn serve(config: ServeConfig, registry: WorkspaceRegistry) -> ...
```

`AuthState` is not deleted — it stays in `auth_state.rs` as the future home
for outbound agent API token management — but it is no longer wired into the
HTTP serving path.

### 3. Remove `--token` / `--token-file` from the `serve` CLI subcommand

`ServeCliArgs` currently has `token: Option<String>` and
`token_file: Option<PathBuf>`.  Remove both.  `cmd_serve` no longer builds an
`AuthState` or passes one to `serve()`.

The `TICKET_SERVE_TOKEN` environment variable is unused by serve for now.
It will be picked up by the agent executor subsystem when that is implemented.

### 4. Remove token length validation

`validate_token_str` enforces a 16-character minimum.  This was only relevant
for inbound validation.  For outbound agent API tokens the only meaningful
error is the one returned by the remote API (Copilot / Claude / OpenAI) — not
a local length check.  Remove `validate_token_str` and `TokenLoadError::TooShort`.

### 5. Clean up `healthz`

`HealthResponse` currently reports `auth_generation` and `auth_last_reload_ts`
because those come from `AppState.auth`.  Once `auth` is removed from
`AppState`, drop those two fields entirely.

```rust
// After
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
}
```

### 6. Frontend — token input stays, backend ignores it

The ticket-viewer UI keeps the 🔑 token input.  Users can still type a token
and it will be sent as `Authorization: Bearer`.  The backend simply doesn't
validate it.  When agent API coordination lands, the backend will pick up the
token from its own config (env / file) — not from the HTTP request.

---

## Self-documenting `AuthMode` enum (future use)

When agent API calls are added, introduce `AuthMode` in `auth_state.rs`:

```rust
pub enum AuthMode {
    /// No agent API configured — ticket serve runs standalone.
    Standalone,
    /// Agent API token loaded — backend can initiate Copilot/Claude/OpenAI calls.
    AgentEnabled(AuthState),
}
```

That is **out of scope for this ticket** but documents the intended direction.

---

## Files to change

| File | Change |
|------|--------|
| `crates/context-tasks/src/serve/mod.rs` | Remove `auth` from `AppState`; remove `auth` param from `serve()` |
| `crates/context-tasks/src/serve/routes.rs` | Remove `bearer_auth_mw` layer from API routes |
| `crates/context-tasks/src/serve/handlers/health.rs` | Drop `auth_generation` / `auth_last_reload_ts` |
| `crates/context-tasks/src/serve/auth_state.rs` | Remove `validate_token_str` + `TokenLoadError::TooShort`; keep rest for future agent use |
| `crates/context-tasks/src/cli.rs` | Remove `--token`/`--token-file` from `ServeCliArgs`; simplify `cmd_serve` |

---

## Acceptance criteria

- `ticket serve --port 4000` starts without any token argument.
- `GET /api/workspaces` returns 200 with no `Authorization` header.
- `GET /api/tickets` returns 200 with no `Authorization` header.
- `GET /api/stream` opens the SSE stream with no `Authorization` header.
- Passing `Authorization: Bearer anything` is silently accepted (not rejected).
- `healthz` no longer includes `auth_generation` / `auth_last_reload_ts`.
- All existing tests pass with no changes needed in test setup.

---

## Out of scope

- Agent API call coordination (future ticket).
- Per-route access control.
- Multi-user / network-exposed deployment hardening.
