# Impl: auth token reload and runtime reconfiguration for ticket serve

**Wave 1 / Track C2** | Component: `context-tasks`

## Design inputs
- Auth lifecycle: `68dfc679/assets/design/auth-lifecycle-v0.1.md`
- API contract: `21a1b9ca/assets/design/api-contract-v0.1.md` (healthz diagnostics)

## Objective
Implement `AuthState` — the live-reloadable bearer token registry used by the
`ticket serve` auth middleware — with atomic hot-swap semantics, grace-window
support, and audit logging, matching `auth-lifecycle-v0.1.md`.

## Architecture

```
env / config file
       |
  TokenLoader::load() → validates format + entropy
       |                   |
       |                 fail → keep previous set, emit auth.reload_failed
       |
  AuthState::reload()  — atomic ArcSwap of TokenSet
       |
  AuthState::token_set() → TokenSet (referenced by BearerAuthLayer)
       |
  DiagnosticEmitter  → auth.reloaded SSE event (generation_id bump)
```

## Implementation plan

### Step 1 — `serve/auth.rs` module
```rust
pub struct AuthState {
    current: Arc<ArcSwap<TokenSet>>,
    generation: AtomicU64,
    last_reload_ts: Mutex<Option<DateTime<Utc>>>,
}
```
Add `arc-swap` to `[dependencies]` in `Cargo.toml`.

### Step 2 — `TokenSet`
```rust
pub struct TokenSet {
    /// Hashed fingerprints only — never store raw tokens in struct fields
    fingerprints: HashSet<[u8; 32]>,  // BLAKE3 hash
}
impl TokenSet {
    pub fn contains(&self, raw_token: &str) -> bool {
        let hash = blake3::hash(raw_token.as_bytes());
        self.fingerprints.contains(hash.as_bytes())
    }
}
```
Primary token + grace_tokens are both loaded into the same `HashSet`.

**Security:** raw token string is never stored, logged, or returned via any API.

### Step 3 — `TokenLoader`
```rust
pub struct TokenLoader {
    /// Source precedence: (1) in-memory override, (2) env var, (3) config file
    env_var: String,       // "TICKET_SERVE_TOKEN"
    config_path: Option<PathBuf>,
}
impl TokenLoader {
    pub fn load(&self) -> Result<TokenSet, TokenLoadError>;
    // Validates: non-empty, minimum 32 chars entropy, valid UTF-8
}
```

### Step 4 — `AuthState::reload()`
```rust
impl AuthState {
    pub async fn reload(&self, loader: &TokenLoader, emitter: &HookEmitter)
        -> Result<(), ReloadError>
    {
        match loader.load() {
            Ok(new_set) => {
                self.current.store(Arc::new(new_set));
                self.generation.fetch_add(1, Ordering::SeqCst);
                *self.last_reload_ts.lock().unwrap() = Some(Utc::now());
                emitter.emit(SseEvent::DiagnosticWarning {
                    code: "auth.reloaded".into(),
                    message: format!("Token set reloaded (generation {})", gen),
                    ..
                }).ok();
                Ok(())
            }
            Err(e) => {
                // Keep previous set — emit failure diagnostic
                emitter.emit(SseEvent::DiagnosticWarning {
                    code: "auth.reload_failed".into(),
                    message: e.to_string(),
                    ..
                }).ok();
                Err(e.into())
            }
        }
    }
}
```

### Step 5 — Healthz diagnostics
Extend `GET /healthz` response with auth metadata:
```json
{
  "status": "ok",
  "service": "ticket-serve",
  "auth_generation": 3,
  "auth_last_reload_ts": "2026-03-21T06:00:00Z"
}
```

### Step 6 — Runtime reload trigger
Two paths:
1. **Admin API**: `POST /api/admin/auth/reload` — protected by same bearer token
2. **SIGHUP signal** (Unix only): register handler that calls `auth_state.reload()`

### Step 7 — Test `ticket serve` CLI flag
`ticket serve --reload-token` triggers one reload cycle and prints `ok`/error.

### Step 8 — Tests
- `tests/integration_auth_reload.rs`:
  - Start serve, assert initial generation=1
  - Change env var, call reload endpoint, assert generation=2
  - Assert old and new tokens both valid during grace window
  - Assert old token rejected after grace_window_ms elapses (configurable in tests)
- `tests/unit_token_set.rs`:
  - `contains` returns true for raw token matching loaded hash
  - `contains` returns false for wrong token
  - Raw token not present in `TokenSet` debug output

## Acceptance criteria
- [ ] `AuthState::reload()` atomically swaps token set without interrupting connections
- [ ] `TokenSet::contains` uses hashed comparison; raw token never stored
- [ ] `auth.reloaded` / `auth.reload_failed` SSE diagnostics emitted on each attempt
- [ ] Healthz exposes `auth_generation` + `auth_last_reload_ts`
- [ ] `POST /api/admin/auth/reload` triggers reload
- [ ] Integration tests: generation bump on successful reload, grace window

## Dependencies / Handoff
- Blocked on: `43dedd9b` — `AppState` and route structure must exist first
- Provides: `AuthState` type consumed by `43dedd9b`'s bearer middleware
- Note: parallel work with `43dedd9b` possible if `AuthState` interface agreed upfront
