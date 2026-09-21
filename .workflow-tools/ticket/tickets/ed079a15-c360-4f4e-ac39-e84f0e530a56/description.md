## Outcome
Benchmark and profile the shared cross-workspace move (preflight -> apply -> rollback/resume) operation across all five domains that implement it (ticket, spec, session, rule, audit), then use the measured numbers to redesign the shared move kernel for efficient large-batch operation. This directly targets the ~10-hour wall time observed during the September 2026 context-engine -> meta-workspace entity migration.

## Scope decision
- Domains: ticket, spec, session, rule, audit (all five MoveDomain implementors of workflow-tools/memory-kernel/src/storage/move_kernel.rs).
- Redesign target: the shared kernel first; a domain-specific adapter is only touched if the benchmark report shows a bottleneck the kernel fix doesn't cover.
- Checkpoint: hard boundary between Waypoint A (benchmark) and Waypoint B (redesign) -- B does not start until A's report is reviewed.

## Full dossier
See transcripts/01-09-2026_move-performance-benchmarking/README.md (reading-order index), ROADMAP.md (waypoints), and ARTIFACTS.md (existing code inventory: move_kernel.rs, move_kernel_types.rs, existing ticket-api benches/move_health.rs already covering reference-heavy preflight/execute/rollback).

## Sub-tickets (depends_on order)
1. Extend ticket-domain benchmark scenarios (01-ticket-benchmark-scenarios.md)
2. Spec/session benchmark harness (02-spec-session-benchmark-harness.md)
3. Rule/audit benchmark harness (03-rule-audit-benchmark-harness.md)
4. Run benchmarks and compile cross-domain report (04-run-benchmarks-and-report.md) -- gated on 1-3
5. Kernel redesign (05-kernel-redesign.md) -- gated on 4's reviewed report

## Non-goals
- Does not change the Mandatory Batch Protocol's correctness contract (preflight -> snapshot+checksum -> apply -> verify -> accept-or-rollback stays intact).
- Does not re-run the real production migration as part of this epic.
- Does not migrate the deferred 310 session records or resolve the dangling spec record 585aa074-356a-4169-b08b-4e3aba659a72 -- separate, already-tracked follow-ups.

## Acceptance Criteria
- All five domains have a move_health benchmark covering the full scenario matrix (entity count x link presence x link density x phase).
- A cross-domain benchmark report exists identifying the dominant cost phase(s).
- The shared move kernel is redesigned based on that report's findings, with measured improvement shown via re-run benchmarks and no regression in existing move-related tests.
