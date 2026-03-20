#!/usr/bin/env bash
set -euo pipefail

# Seed bootstrap backlog into the ticket CLI.
# Assumes ticket binary is available as `ticket` in PATH.

ticket create --id 4f2d2a5e-5df1-4bd8-9b65-0d4de0a0a5c1 --type tracker-improvement --title "[bootstrap] wire create/get/update/list/delete to storage backend" --state open --field component=storage --field risk_level=high --field acceptance_criteria="CRUD commands execute real backend paths; deterministic JSON envelopes; no draft status in successful flow" --field bootstrap_blocker=true --field rollout_stage=mirror

ticket create --id 2a1fa2f2-56ce-45cc-a5d4-915d90e6b7a2 --type tracker-improvement --title "[bootstrap] implement lease lifecycle with stale recovery" --state open --field component=lease --field risk_level=high --field acceptance_criteria="claim/unclaim/heartbeat paths implemented; stale lease cleanup verified by integration test" --field bootstrap_blocker=true --field rollout_stage=mirror

ticket create --id de6c3391-27c2-4e27-bde8-1456f0eb3f43 --type tracker-improvement --title "[bootstrap] add crash-recovery test for atomic write plus reconcile" --state open --field component=watcher --field risk_level=high --field acceptance_criteria="simulated mid-write crash recovers via scan/reconcile with no silent data loss" --field bootstrap_blocker=true --field rollout_stage=mirror

ticket create --id 77f1eb5c-dc38-4221-89e9-2bdf2b8d3ca4 --type tracker-improvement --title "[bootstrap] wire history, diff, and revert end-to-end" --state open --field component=history --field risk_level=medium --field acceptance_criteria="history/diff/revert commands produce expected revisions and preserve forward-only semantics" --field bootstrap_blocker=false --field rollout_stage=mirror

ticket create --id ee43f72e-53ef-4937-8216-92e17f185d85 --type tracker-improvement --title "[bootstrap] implement unified query execution on real indexes" --state open --field component=search --field risk_level=medium --field acceptance_criteria="mixed metadata + text queries return stable ranked results; unknown fields return deterministic errors" --field bootstrap_blocker=false --field rollout_stage=hybrid

ticket create --id 5e4727f9-53a6-4d36-a98f-4c9a6db81216 --type tracker-improvement --title "[bootstrap] implement deps, blocked-by, and validate-graph commands" --state open --field component=graph --field risk_level=medium --field acceptance_criteria="dependency traversal and cycle/dangling detection pass scenario tests" --field bootstrap_blocker=false --field rollout_stage=hybrid

ticket create --id be1a3de7-f44f-496d-b4c6-b4f8a120dc97 --type tracker-improvement --title "[bootstrap] add merge queue scheduler with lease conflict overlay" --state open --field component=graph --field risk_level=medium --field acceptance_criteria="merge queue honors dependency and conflict constraints; queue output is machine-readable" --field bootstrap_blocker=false --field rollout_stage=hybrid

ticket create --id 9d0258de-bf87-4b7e-b8f0-e78f4fdf0b58 --type tracker-improvement --title "[bootstrap] define backup and restore procedure for index plus history" --state open --field component=storage --field risk_level=medium --field acceptance_criteria="documented and tested restore flow for redb index and git history repo" --field bootstrap_blocker=false --field rollout_stage=hybrid

ticket create --id c91a334e-26cf-4cf2-9212-4288a07bbf09 --type tracker-improvement --title "[bootstrap] establish observability and failure diagnostics standard" --state open --field component=watcher --field risk_level=low --field acceptance_criteria="worker failures emit structured diagnostics with correlation ids and suggested recovery action" --field bootstrap_blocker=false --field rollout_stage=tracker-first

ticket create --id 48ea4df8-25f5-46ce-b2cc-ff00d32ddd47 --type tracker-improvement --title "[bootstrap] run one-week dogfood trial and publish go-no-go report" --state open --field component=cli --field risk_level=low --field acceptance_criteria="trial has zero critical data-loss or deadlock incidents; rollout recommendation published" --field bootstrap_blocker=false --field rollout_stage=tracker-first

ticket create --id a8d6c1d2-2b64-4d9a-9f1d-1e2a3b4c5d61 --type tracker-improvement --title "[bootstrap][T1] startup and auth bootstrap for host executor" --state open --field component=lease --field risk_level=high --field acceptance_criteria="host executor starts in stdio mode, scoped token auth succeeds, unauthorized request returns structured auth error" --field validation_plan="integration test T1 in HOST_EXECUTOR_AUTH_PROVIDER.md" --field validation_status=pending --field release_target=phase-1.5 --field bootstrap_blocker=true --field rollout_stage=mirror

ticket create --id b1f3e2a4-6c7d-4e8f-9a0b-2c3d4e5f6a72 --type tracker-improvement --title "[bootstrap][T2] enforce assignment start context branch and cwd checks" --state open --field component=cli --field risk_level=high --field acceptance_criteria="assignment packet branch/cwd constraints are verified at session start and mismatches produce recoverable structured errors" --field validation_plan="integration test T2 in HOST_EXECUTOR_AUTH_PROVIDER.md" --field validation_status=pending --field release_target=phase-1.5 --field bootstrap_blocker=true --field rollout_stage=mirror

ticket create --id c2a4b6d8-7e9f-4a1b-8c2d-3e4f5a6b7c83 --type tracker-improvement --title "[bootstrap][T3] validate ticket lifecycle happy path under executor" --state open --field component=lease --field risk_level=high --field acceptance_criteria="claim->update->evidence->unclaim path works and assignment_id appears on emitted events" --field validation_plan="integration test T3 in HOST_EXECUTOR_AUTH_PROVIDER.md" --field validation_status=pending --field release_target=phase-1.5 --field bootstrap_blocker=true --field rollout_stage=mirror

ticket create --id d3b5c7e9-8f1a-4b2c-9d3e-4f5a6b7c8d94 --type tracker-improvement --title "[bootstrap][T4] implement validator handoff with separation-of-duties" --state open --field component=lease --field risk_level=high --field acceptance_criteria="worker->validator handoff works in validating state and same-identity validator assignment is rejected" --field validation_plan="integration test T4 in HOST_EXECUTOR_AUTH_PROVIDER.md" --field validation_status=pending --field release_target=phase-1.5 --field bootstrap_blocker=true --field rollout_stage=mirror

ticket create --id e4c6d8f1-9a2b-4c3d-8e4f-5a6b7c8d9ea5 --type tracker-improvement --title "[bootstrap][T5] handle early-stop recovery and reassignment" --state open --field component=watcher --field risk_level=high --field acceptance_criteria="session early-stop invalidates tokens, resolves lease state, emits incident event, and moves ticket to blocked/review with blocker metadata" --field validation_plan="integration test T5 in HOST_EXECUTOR_AUTH_PROVIDER.md" --field validation_status=pending --field release_target=phase-1.5 --field bootstrap_blocker=true --field rollout_stage=mirror

ticket create --id f5d7e9a2-ab3c-4d5e-9f5a-6b7c8d9eaf16 --type tracker-improvement --title "[bootstrap][T6] verify merge and completion linkage with assignment chain" --state open --field component=history --field risk_level=medium --field acceptance_criteria="validation passed -> release-candidate -> merge path records assignment chain, merge commit, and release target/version metadata" --field validation_plan="integration test T6 in HOST_EXECUTOR_AUTH_PROVIDER.md" --field validation_status=pending --field release_target=phase-1.5 --field bootstrap_blocker=false --field rollout_stage=hybrid

echo "Seeded bootstrap task tracker backlog."
