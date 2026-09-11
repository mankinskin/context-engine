# Interview 2 — Spec Lifecycle State

## Evidence

The roadmap says specs `f1b8f01a` and `fa6e85c8` were approved for implementation planning on 2026-08-23, but both persisted `spec.toml` files still have `state = "draft"`.

## Decision

Keep both specs persisted as `draft`. The approval is planning-level evidence recorded in roadmap prose; this execute-ingest run must not mutate spec lifecycle state.