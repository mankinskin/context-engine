## Problem

The current track mentions profiled pre-commit hooks, but it does not yet define a complete benchmarking and profiling plan for generator performance. Without explicit measurement scope, datasets, and thresholds, generator work can quietly regress until store-index regeneration stalls commits.

## Goal

Define the benchmarking and profiling plan for store-index generation so commit-time automation has an explicit performance budget and generator implementations have reviewable evidence targets.

## Scope

- Define the benchmark matrix for ticket, spec, rule, audit, and workspace generation.
- Define representative dataset sizes and incremental versus full-regeneration scenarios.
- Define the target latency budget for pre-commit execution and the profiling evidence needed before hook automation is considered complete.
- Define the profiling surfaces and tools to use for CPU, allocation, and I/O investigation when a generator exceeds budget.
- Update hook and generator tickets so performance evidence is a concrete acceptance requirement.

## Acceptance Criteria

- The track has a documented benchmark matrix and performance budget.
- Incremental and full-regeneration scenarios are both covered.
- The hook automation ticket can point at this plan for threshold and evidence requirements.
- Generator tickets are updated so implementers know what performance evidence must be captured.

## Non-goals

- Does not optimize generator code yet.
- Does not implement benchmark harnesses unless needed to prove the plan.
- Does not replace hook automation or digest-contract tickets.

## Resolved direction carried into this ticket

- Pre-commit generation must stay fast enough to avoid making normal commits unpleasant, and that constraint needs explicit measurement rather than informal expectation.