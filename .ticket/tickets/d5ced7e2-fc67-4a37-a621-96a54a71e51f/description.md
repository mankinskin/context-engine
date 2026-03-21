# Plan: Copilot API Execution Layer (Phase 2)

## Objective
Build a Rust execution layer that can:
- authenticate with API-key based provider credentials,
- start and supervise sub-agent runs against isolated git worktrees and branches,
- coordinate ticket lifecycle and review handoff,
- notify humans via desktop/native notification adapters and messenger adapters,
- provide a terminal UI for operator control and live progress.

## Why this now
The ticket backlog can already be worked in parallel. The next capability needed is reliable automated execution per ticket assignment with isolation, observability, and review routing.

## Scope
- In scope:
  - Provider auth and API client layer for agent execution.
  - Worktree/branch sandbox provisioning per assignment.
  - Assignment-run state machine and progress watcher.
  - Review coordinator (worker -> validator transitions).
  - Notification adapters (desktop first, messenger adapter interface + one concrete adapter).
  - Terminal UI for queue, assignment, run logs, and review state.
  - E2E integration tests for happy path and recovery path.
- Out of scope:
  - Full multi-provider marketplace.
  - Web UI replacement for the TUI.

## Architecture sketch
1. `auth-provider` and `executor-client` modules provide API-key backed request flow and typed response mapping.
2. `sandbox-manager` creates per-assignment worktree + branch, validates cwd/branch, and handles cleanup/recovery.
3. `assignment-runner` drives session start, command streaming, heartbeats, and terminal states.
4. `review-coordinator` owns validating/review transitions and required evidence handoff.
5. `notify` module exposes `Notifier` trait with desktop + messenger implementations.
6. `tui` module subscribes to assignment events and provides operator actions.

## Parallelization model
- Track A (platform): auth client + sandbox manager
- Track B (orchestration): assignment runner + review coordinator
- Track C (operator UX): notifications + TUI
- Track D (quality): end-to-end tests, failure injection, observability checks

## Risks
- Credential leakage in logs.
- Worktree cleanup failures causing branch drift.
- Race conditions in ticket state transitions during handoff.
- Notification spam under high ticket churn.

## Mitigations
- Redaction and structured secret handling tests.
- Idempotent cleanup and startup reconciliation pass.
- Strong state guards around transitions; optimistic concurrency checks.
- Notification rate limiting and digest mode.

## Validation plan
- Unit tests for auth, sandbox provisioning, and transition guards.
- Integration tests for assignment lifecycle including early-stop recovery.
- E2E test suite that runs 2-3 concurrent assignments with independent branches/worktrees and verifies merge/review chain metadata.

## Initial ticket decomposition
1. Design/API contract ticket for execution provider abstraction.
2. Implementation ticket for sandbox manager.
3. Implementation ticket for assignment runner and progress watcher.
4. Implementation ticket for review coordinator and state guards.
5. Implementation ticket for notifier adapters.
6. Implementation ticket for terminal UI.
7. Integration and fault-injection ticket.

## Dependencies
- Depend on bootstrap tickets T1-T5 for host executor baseline and lifecycle behavior.
- T6 is required before declaring release linkage complete for this phase.
