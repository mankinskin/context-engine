# Goal

Provide targeted reconcile/scan modes so move flows and internal tooling can update only the touched ticket set instead of walking unrelated roots.

## Problem

`scan(false)` is a useful improvement over forced rebuild scans, but move flows and store tooling still need a narrower reconcile mode when the touched entity set is already known.

## Scope

- define internal reconcile entry points or modes for a known set of touched tickets/roots
- wire move execution to use the targeted mode where safe
- identify related internal tooling paths that should use the same narrowed reconcile behavior

## Non-goals

- public API redesign for ticket CLI, HTTP, or MCP
- speculative optimization of unrelated store operations

## Acceptance criteria

1. Move execution can reconcile a known touched set without walking unrelated ticket roots.
2. Internal tooling paths that already know the touched set can opt into the targeted reconcile mode.
3. Tests cover correctness for moved tickets, affected references, and no-op unaffected tickets.
4. Validation evidence shows reduced scan/reconcile work relative to the current `scan(false)` fallback in the targeted scenarios.

## Validation

- `cargo test -p ticket-api --test e2e_perf_move_health -- --nocapture`
- targeted `ticket-api` tests for move/tooling reconcile scope

## Context

This follow-up narrows the remaining move/tooling reconciliation work after the coarse `scan(false)` switch in `bf094901-cdb6-4b25-8ccd-3eb7716f9d20`.