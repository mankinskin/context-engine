# Goal

Remove avoidable per-entity SQLite and remaining index-write churn from scan reconciliation after the Tantivy batching fix.

## Problem

The current hot-path work eliminated per-document Tantivy commit costs, but residual persistence work can still serialize reconciliation if metadata/index writes remain too granular.

## Scope

- profile the remaining SQLite/index write hotspots in scan reconciliation
- batch or transactionally group writes where correctness permits
- preserve crash-safety and index/store consistency guarantees

## Non-goals

- reworking the already-fixed batched Tantivy writer path
- schema redesign unrelated to batching existing writes

## Acceptance criteria

1. Scan reconciliation no longer performs avoidable per-ticket SQLite/index write churn on the profiled hot path.
2. Batching strategy preserves existing correctness and recovery expectations.
3. Focused validation demonstrates reduced persistence/write phase cost on the existing perf fixture or targeted benchmark.
4. Ticket notes document any remaining non-batchable writes and why they stay granular.

## Validation

- `cargo test -p ticket-api --test e2e_perf_move_health -- --nocapture`
- targeted profiling or benchmark evidence for the affected write phases

## Context

This ticket intentionally excludes the Tantivy commit batching already handled in `bf094901-cdb6-4b25-8ccd-3eb7716f9d20`.