# Use Case — Messenger Digests for Long-Running Swarm Tasks

## Scenario

A long-running swarm processes many tickets over hours. Users need curated progress and escalation messages in a social messenger instead of polling CLI output.

## Problem

Without asynchronous messaging, users miss important completion/failure signals or must continuously monitor the tracker directly.

## Solution

- Emit tracker events for lease, state, blocker, and completion changes.
- Route selected events into messenger digests by policy:
  - periodic progress summary
  - completion highlights
  - blocker/escalation alerts
- Allow constrained reply actions (allowlist only) mapped to safe ticket intents.

## Reference

- Phase 1 lease/claim workflows.
- Phase 6 dogfooding governance rules.
- Phase 7 messenger integration plan.

## Acceptance Signals

- Digest cadence and throttling prevent message spam while preserving critical events.
- Every outbound/inbound messenger action is audit-linked to ticket ids and worker ids.
- Reply commands can only trigger allowlisted state/comment operations.
