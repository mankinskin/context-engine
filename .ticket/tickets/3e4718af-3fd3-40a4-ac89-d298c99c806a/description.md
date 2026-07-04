# Goal

Reduce `rebuild_workflow_facts_ms` by recomputing only the workflow facts touched by a changed ticket or move operation.

## Problem

After the initial scan/search hot-path fixes, workflow-fact rebuilding is the dominant remaining scan cost on the perf fixture.

## Scope

- identify the minimal set of tickets/facts invalidated by `scan(false)` changes
- avoid store-wide workflow-facts recompute when unrelated tickets are unchanged
- keep move reconciliation limited to the moved ticket and directly impacted references/facts

## Non-goals

- changing workflow-fact semantics
- broad search-index optimization unrelated to workflow-facts invalidation

## Acceptance criteria

1. `scan(false)` on a small changed set does not trigger store-wide workflow-facts recompute.
2. Move reconciliation recomputes workflow facts only for the moved ticket and directly affected dependents/references.
3. Targeted tests cover unchanged, single-ticket-changed, and moved-ticket cases.
4. Perf validation shows a material drop in `rebuild_workflow_facts_ms` on the existing perf fixture.

## Validation

- `cargo test -p ticket-api --test e2e_perf_move_health -- --nocapture`
- focused `ticket-api` tests around workflow-facts invalidation/rebuild scope

## Context

This is the primary remaining bottleneck called out by `bf094901-cdb6-4b25-8ccd-3eb7716f9d20`.