# Impl: live ticket graph stream pipeline (SSE + hooks + conflict events)

**Wave 1 / Track D** | Component: `context-tasks`

## Design inputs
- API contract: `21a1b9ca/assets/design/api-contract-v0.1.md`
- SSE schema (frozen): `09a32876/assets/design/sse-schema-v1.md`
- SSE contract tests: `09a32876/assets/design/sse-contract-tests-v0.1.md`
- Hook emission contract: `24aa7e5e/assets/design/hook-contract-v0.1.md`

## Objective
Implement the `GET /api/stream` SSE endpoint and the hook-driven internal event
pipeline that feeds it. Clients receive live updates as ticket/edge mutations happen.

## Architecture

```
TicketStore mutation
       |
  HookEmitter  (per-workspace FIFO channel, at-least-once)
       |
  StreamBroker (fan-out to active SSE clients per workspace)
       |
  SSE response stream  →  client
       |
  ReconcileLoop  (periodic gap detection → snapshot.ready events)
```

## Implementation plan

### Step 1 — `stream/` module
Create `crates/context-tasks/src/serve/handlers/stream.rs` (stub prepared by `43dedd9b`):
```
stream.rs — axum SSE handler, wires StreamBroker
```
Add `crates/context-tasks/src/serve/stream/`:
```
mod.rs        — exports
broker.rs     — StreamBroker: per-workspace broadcast channel
emitter.rs    — HookEmitter: called by store mutations
event.rs      — SseEvent enum with serde serialization
reconcile.rs  — ReconcileLoop: periodic watermark check + gap fill
```

### Step 2 — `SseEvent` types
Define serializable event types matching `sse-schema-v1.md`:
- `TicketUpsert { workspace, ts, ticket: TicketSummary }`
- `TicketDelete { workspace, ts, id, deleted_at }`
- `EdgeUpsert { workspace, ts, edge: EdgeRecord }`
- `EdgeDelete { workspace, ts, edge: EdgeRecord }`
- `TicketConflict { workspace, ts, id, operation, expected_rev, observed_rev }`
- `SnapshotReady { workspace, ts, snapshot_id, node_count, edge_count }`
- `DiagnosticWarning { workspace, ts, code, message }`

Wrap in `axum::response::sse::Event` with `id` (monotonic counter) + `event` name.

### Step 3 — `HookEmitter`
- Called synchronously after every store write (create/update/delete/link/unlink)
- Sends to per-workspace `tokio::sync::broadcast::Sender<SseEvent>`
- Non-blocking: `try_send`; on full queue, increment `hook_emit_failure_total` metric
- Mutation write must not fail if emitter send fails

### Step 4 — `StreamBroker`
- Holds `HashMap<WorkspaceId, broadcast::Sender<SseEvent>>`
- `AppState` has an `Arc<StreamBroker>`
- SSE handler subscribes a `broadcast::Receiver` per connection
- On client subscribe, emit `snapshot.ready` (initial reconcile snapshot)

### Step 5 — SSE handler
```rust
async fn stream_handler(
    AuthExtension(workspace): AuthExtension<WorkspaceId>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let rx = state.broker.subscribe(&workspace);
    Sse::new(ReceiverStream::new(rx).map(|evt| Ok(evt.into_sse_event())))
        .keep_alive(KeepAlive::default())
}
```

### Step 6 — `ReconcileLoop`
- Background tokio task per workspace
- Every 30s check `storage_rev_watermark` vs `emitted_rev_watermark`
- On gap: emit batched upsert events for all tickets since last emitted rev
- Then emit `snapshot.ready`

### Step 7 — Wire HookEmitter into store mutations
Edit `storage/` write paths to call `hook_emitter.emit(evt)` after successful writes.

### Step 8 — Integration tests
- `tests/integration_sse_upsert.rs`: subscribe SSE, create ticket, assert `ticket.upsert` received
- `tests/integration_sse_delete.rs`: delete ticket, assert `ticket.delete`
- `tests/integration_sse_edge.rs`: link tickets, assert `edge.upsert`
- `tests/integration_sse_reconcile.rs`: simulate gap, assert `snapshot.ready`

## Acceptance criteria
- [ ] `GET /api/stream` returns `Content-Type: text/event-stream`
- [ ] Creating a ticket emits `ticket.upsert` SSE event on all subscribed clients
- [ ] Deleting a ticket emits `ticket.delete`
- [ ] Linking tickets emits `edge.upsert`
- [ ] Mutation write succeeds even if all SSE receivers are slow/disconnected
- [ ] Reconcile loop emits `snapshot.ready` on detected gap
- [ ] SSE event `id` is strictly monotonically increasing per workspace
- [ ] Contract tests from `sse-contract-tests-v0.1.md` pass

## Dependencies / Handoff
- Blocked on: `43dedd9b` (serve mode must provide SSE route stub and AppState)
- Produces: live stream consumed by `02dea1fa` (ticket-viewer shell)
