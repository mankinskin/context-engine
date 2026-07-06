# [workspace-policy] 7/6 Fixture boundary cleanup + policy-excluded reference audit guard

## Why

Workspace-policy rollout should prevent future indexing/visibility of excluded workspaces, but the store currently contains historical "retro" tickets and cross-workspace edges that reference fixture workspace tickets. Those references should be removed so ticket-graph/audit signals represent only intended workspace scope.

## Scope

1. Remove introduced retrospective/retro tickets that were created only to satisfy orphan cleanup for fixture stores.
2. Remove **all** graph edges where:
   - edge target ticket is under a policy-excluded workspace (fixture/test workspace), and
   - edge source ticket is outside that excluded workspace.
3. Keep fixture-internal graph relationships intact (inside→inside edges remain allowed).
4. Implement audit detection to fail when any policy-excluded workspace ticket is referenced by tickets outside those excluded workspaces.

## Audit rule contract

- Trial: `ticket_graph`
- Metric name: `policy_excluded_reference_count`
- Threshold: `0`
- Finding category: `ticket_graph`
- Evidence includes:
  - `edge_kind`
  - `source_ticket_id`, `source_path`, `source_workspace_root`
  - `target_ticket_id`, `target_path`, `target_workspace_root`
  - policy exclusion reason / matched rule

## Acceptance criteria

- [ ] No retro cleanup tickets remain that were added solely to bridge fixture-store orphan findings.
- [ ] No non-fixture tickets reference fixture tickets via any edge kind.
- [ ] `audit run` reports `policy_excluded_reference_count = 0` for a clean store.
- [ ] Regression test covers at least one violating edge and one allowed fixture-internal edge.
- [ ] Ticket description records exact cleanup commands and before/after evidence artifact paths.

## Notes

- This ticket is a follow-on hardening/cleanup slice under the workspace-policy tracker.
- Execute cleanup after policy enforcement slices are merged to avoid reintroducing references during rescan/reindex.
