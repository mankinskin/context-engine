# Design: command-hook emission contract and fallback reconciliation

## Objective
Define hook events emitted by ticket mutations and fallback behavior when hooks fail.

## Hook trigger points
- create/update/delete
- link/unlink
- state transitions
- validation/release protocol steps

## Event contract
- `event_id` UUID
- `workspace`
- `op`
- `entity_type` ticket|edge
- `entity_id`
- `rev` (if available)
- `ts`

## Delivery semantics
- At-least-once delivery to in-process stream bus.
- Idempotency key: `event_id`.
- Non-blocking emission path for write operations.

## Fallback reconcile
- Periodic reconcile sweep compares latest storage revision vs emitted revision watermark.
- Missing events produce synthetic `snapshot.ready` and batched upsert events.

## Failure policy
- Hook emission failure should not fail primary mutation write.
- Record diagnostics and increment failure counters.

## Checklist
- [ ] Trigger matrix approved
- [ ] Event fields approved
- [ ] Idempotency strategy approved
- [ ] Reconcile fallback policy approved
- [ ] Observability metrics list approved
