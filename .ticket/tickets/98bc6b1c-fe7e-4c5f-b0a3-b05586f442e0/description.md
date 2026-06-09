## Problem

The current track mentions profiling in passing, but there is no dedicated plan for how generator latency will be benchmarked and profiled across domains. Without a concrete performance plan, pre-commit integration risks stalling commits and generator tickets have no shared evidence standard for runtime cost.

## Goal

Define the benchmarking and profiling plan for store-index generation so hook automation and generator implementations share explicit latency budgets, measurement methodology, and evidence requirements.

## Scope

- Define the latency budgets for incremental pre-commit runs and larger full regeneration runs.
- Define which generator paths need benchmarks, micro-profiles, or trace instrumentation.
- Define the representative workloads and fixtures for ticket, spec, rule, audit, and workspace generation.
- Define how results are recorded and how regressions are evaluated before enabling hook automation by default.
- Clarify how this evidence feeds into `52dfd793` so hook automation has a concrete performance contract.

## Acceptance Criteria

- The track has explicit latency targets for generator execution in commit-time workflows.
- A repeatable benchmarking/profiling method is defined for the relevant generator domains.
- Hook automation tickets can cite this plan instead of inventing one-off timing checks.
- Generator tickets know what evidence is required before they can move through review.

## Non-goals

- Does not implement the generators.
- Does not wire git hooks.
- Does not redefine digest inputs or rendering behavior.