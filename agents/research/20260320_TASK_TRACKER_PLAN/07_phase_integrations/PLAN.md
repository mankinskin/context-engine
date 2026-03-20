# Phase 5 — Visualization and Messenger Integrations

Status: PLANNED (post-core maturity)

## Objective

Add integration surfaces so tracker state can be consumed automatically by external UIs and messaging channels.

## Problem/Solution/Reference Baseline

1. Problem: graph and board outputs are useful but still require manual command execution.
Solution: add API endpoints and scheduled/on-change generation for graph/board artifacts.
Reference: Phase 3 graph export and merge queue workflow requirements.

2. Problem: long-running swarm tasks need asynchronous human interaction loops.
Solution: add event-driven messenger notifications and command-reply workflow for selected ticket updates.
Reference: swarm claim/lease workflows and handoff requirements in existing use cases.

3. Problem: direct messenger posting can leak noisy or sensitive internal activity.
Solution: policy-controlled routing with channel rules, severity thresholds, and explicit subscription filters.
Reference: transition governance model in Phase 4.

## Deliverables

- [ ] Integration event bus contract (`TicketEventEnvelope`) for status changes, lease events, merge queue changes, and completion summaries.
- [ ] Visualization endpoint set:
  - [ ] `GET /api/tickets/graph?format=dot|mermaid|json`
  - [ ] `GET /api/tickets/board?format=json|html`
  - [ ] `GET /api/tickets/critical-path?format=json`
- [ ] Auto-generation jobs:
  - [ ] on-change graph snapshot generation for watched labels/components
  - [ ] scheduled board and critical-path exports
- [ ] Messenger adapter abstraction (`MessengerSink` trait) with initial no-op/mock backend.
- [ ] Notification routing rules:
  - [ ] ticket labels/components -> messenger channel mapping
  - [ ] severity and state-transition filters
  - [ ] dedup/throttle policy for noisy updates
- [ ] Long-running swarm reports:
  - [ ] periodic progress digest
  - [ ] completion summary with links to ticket ids and graph artifacts
  - [ ] failure escalation summary with blocker list
- [ ] Reply flow contract:
  - [ ] map human reply -> ticket comment/update intent
  - [ ] explicit command parser for allowlisted actions only

## Non-Goals (initial integration phase)

- No unrestricted remote control of ticket state from messenger free-text.
- No dependency on a single messenger provider.
- No replacement of CLI/HTTP as primary control surface.

## Hosting Strategy Phasing

Ticket tracker commands are exposed through three successive hosting layers:

| Phase | Layer | Surface |
|-------|-------|---------|
| A (Phase 1) | CLI | `ticket` binary, JSON envelope |
| B (Phase 5) | HTTP | `context-http` routes under `/api/tickets/` |
| C (Phase 5+) | MCP | `context-mcp` tools wrapping HTTP endpoints |

Visualization and messenger endpoints land in Phase B/C.
CLI remains authoritative for all commands; HTTP/MCP are adapters.

## Maturity Gates to Start This Phase

- Core command reliability gate from Phase 4 dogfooding must be green.
- Graph validation and export commands from Phase 3 must be available.
- Lease/stale recovery (Phase 1.5) must be stable under parallel swarm load.

## Risks

- Message spam can reduce trust; throttling and digest mode are mandatory.
- Bidirectional integration can cause accidental state transitions; require allowlists and audit logs.
- Artifact generation load may grow with graph size; enforce per-job limits and incremental rebuild strategy.

## TODO

- TODO: Decide first messenger target (Slack/Discord/Matrix/Telegram) based on team preference.
- TODO: Define signed webhook and credential rotation strategy.
- TODO: Define retention policy for generated graph/board artifacts.
