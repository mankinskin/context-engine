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

echo "Seeded bootstrap task tracker backlog."
