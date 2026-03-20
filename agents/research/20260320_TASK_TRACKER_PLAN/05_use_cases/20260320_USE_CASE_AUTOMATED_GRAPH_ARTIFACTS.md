# Use Case — Automated Graph and Board Artifact Generation

## Scenario

The tracker runs in active swarm mode with continuous dependency and state changes. Stakeholders need always-fresh visualizations without manually invoking CLI exports.

## Problem

Manual graph generation introduces stale artifacts, inconsistent formats, and operator friction.

## Solution

- Trigger artifact generation on dependency/state mutations.
- Provide HTTP endpoints for on-demand retrieval:
  - `GET /api/tickets/graph?format=dot|mermaid|json`
  - `GET /api/tickets/board?format=json|html`
  - `GET /api/tickets/critical-path?format=json`
- Run scheduled regeneration for large workspaces and publish stable snapshots.

## Reference

- Phase 4 graph export and merge queue workflows.
- Phase 7 integration plan for endpoint and automation lifecycle.

## Acceptance Signals

- Graph artifacts update within defined freshness SLO after ticket changes.
- Endpoint responses are deterministic and include schema/version metadata.
- Large graph workspaces use caching/incremental regeneration without service degradation.
