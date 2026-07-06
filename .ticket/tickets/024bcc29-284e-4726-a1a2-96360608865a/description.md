# Session Objective
Resolve the current ticket_graph batch for viewer-api .ticket store and reduce 50 findings from the baseline.

# Scope Guardrails
- Stay inside viewer-api .ticket store unless a blocker requires a dependency fix outside scope.
- Do not start the next batch until this ticket meets done criteria.

# Implementation Steps
1. Capture exact finding rows for this batch from the baseline audit artifact.
2. Group findings into 2 to 5 micro-chunks and handle one chunk at a time.
3. After each chunk, run the narrowest compile/test check relevant to touched files.
4. Re-run audit summary and record count delta.
5. If blockers remain, create follow-up tickets and link them before handoff.

# Validation Commands
- Full category summary: cargo run -p audit-cli --bin audit -- --json summary --by category .
- Full baseline refresh when needed: cargo run -p audit-cli --bin audit -- --json run .
- Ticket health sanity: ./target/debug/ticket.exe health --workspace . --all --toon

# Acceptance Criteria
- Findings in this batch are resolved or have explicit blocker tickets linked.
- No increase in other categories caused by this batch.
- Batch notes include before and after counts and next unresolved action.

# Handoff Notes
Record exact commands run, resulting counts, and files changed so the next session can continue without rediscovery.

# Execution Log (2026-07-06)

## Deterministic Baseline Reconciliation
- `rtk ./target/debug/ticket.exe scan --workspace viewer-api --force --toon`
- `rtk ./target/debug/ticket.exe health --workspace viewer-api --all --toon`
- `rtk cargo run -p audit-cli --bin audit -- --json summary --by category viewer-api`

Baseline observed for this slice is `ticket_graph=53` (drift from title snapshot `50`).

Initial split:
- orphan: 18
- convergence: 35

## Micro-Chunk 1 (orphan set, 10 tickets)
- Created tracker in viewer-api store:
	- `4d9df9df-24c3-4378-bb06-ed86f0b3de6a`
	- title: `[audit-roadmap][ticket_graph][batch-3][chunk-1] viewer-api orphan micro-chunk (10)`
- Linked 10 orphan tickets with `depends_on` edges from tracker to child tickets.

Commands:
- `rtk ./target/debug/ticket.exe create --workspace viewer-api --title "[audit-roadmap][ticket_graph][batch-3][chunk-1] viewer-api orphan micro-chunk (10)" --type tracker-improvement --json`
- `rtk ./target/debug/ticket.exe link --workspace viewer-api --from <chunk-1-tracker> --to <child-id> --kind depends_on --reason "batch-3 chunk-1 orphan cleanup" --json`

## Post-Chunk 1 Delta
After reconciliation and re-audit:
- viewer-api `ticket_graph`: `53 -> 43` (delta `-10`)
- root `ticket_graph`: `70` (post-chunk checkpoint)

Post-chunk split:
- orphan: 8
- convergence: 35

Immediate next action:
- Run orphan micro-chunk 2 for remaining 8 orphan IDs, then handle convergence residuals in deterministic slices.