# Use Case: Swarm Refinement Pipeline

## Goal

Model a swarm of agents that continuously identifies open tickets, refines them into implementation-ready units, and moves them through configurable workflow states.

## Preconditions

- Scan roots are registered and watcher is running.
- Ticket type schemas define at least: triage-like states, refinement states, implementation-ready state.
- Query language supports both FTS and metadata filters.

## Scenario

1. Planner agent runs query: `state:open type:feature missing:acceptance_criteria`.
2. It selects top N tickets lacking definition quality.
3. Refinement agents claim tickets via per-ticket lock and set `working_by=<agent-id>`.
4. Each agent adds missing sections (scope, constraints, acceptance criteria, risks).
5. Validation agent checks schema and dependency integrity.
6. Tickets passing checks move to `implementation-ready`.
7. Tickets failing checks are returned to a refinement state with diagnostics.

## Data Flows

- Filesystem: `ticket.toml`, `description.md`, optional `checklist.json`.
- Global index: `state`, `ticket_type`, `working_by`, dependency counts, readiness score.
- Search index: textual quality signals and missing-field markers.
- Git history: each refinement step is diff-tracked.

## Concurrency Rules

- One writer per ticket (lock scope).
- Many tickets can be refined concurrently by many agents.
- Batch coordinator avoids duplicate work by lease TTL on `working_by`.

## Failure Modes

- Agent crash while lock held: lock TTL expiration or stale lock recovery.
- Conflicting edits through external tools: watcher flags divergence; ticket enters `needs-reconcile` state.
- Schema changes during active refinement: ticket revalidated on save.

## Success Metrics

- Median time from `open` to `implementation-ready`.
- Number of tickets auto-refined per day.
- Rework rate after refinement (tickets moved backwards in state machine).
