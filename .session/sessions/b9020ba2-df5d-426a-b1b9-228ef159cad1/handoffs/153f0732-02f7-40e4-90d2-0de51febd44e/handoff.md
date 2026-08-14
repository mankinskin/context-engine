# Handoff: 153f0732-02f7-40e4-90d2-0de51febd44e

Safely complete the workflow-tools repository restructure by repairing stale guidance and tooling, recording durable architecture and delegation policy, and addressing verified follow-up defects without touching unrelated in-progress submodule changes.

## Upward Context
[7bc328d7 Repository guidance and agent-template learnings from the workflow-tools restructuring](.ticket/tickets/7bc328d7/ticket.toml) (epic) -> [07a3eb2d Repair build and install tooling referencing removed ticket-cli package](.ticket/tickets/07a3eb2d-8868-4c36-a60a-e93cc787c065/ticket.toml), [dc13ffb4 Define a shared terminal return contract for all sub-agents](.ticket/tickets/dc13ffb4-f172-469c-a0ad-454354aa4f28/ticket.toml), [7063311c Author spec for repository architecture and dependency policies](.ticket/tickets/7063311c-a380-4269-adcf-7d1388ab5f39/ticket.toml)

## Summary
- **Workspace Session**: `b9020ba2-df5d-426a-b1b9-228ef159cad1`
- **Outgoing Run**: `15950a31-3ceb-4a53-a421-b0f3dcec3093`
- **Created**: 2026-08-14T11:33:33.037580500+00:00
- **Objective**: Hand off the ticket-only plan for repository guidance, architecture policy, agent-template, observability, tooling, and graph-rendering follow-up work discovered during the workflow-tools restructuring review.
- **Implementation Ready**: true

## Resume Command
```bash
session-cli resume --session-id b9020ba2-df5d-426a-b1b9-228ef159cad1 --predecessor-run-id 15950a31-3ceb-4a53-a421-b0f3dcec3093
```

## Target Tickets
| Ticket | What it does | Why |
| --- | --- | --- |
| [07a3eb2d Repair build and install tooling referencing removed ticket-cli package](.ticket/tickets/07a3eb2d-8868-4c36-a60a-e93cc787c065/ticket.toml) |  | Repair real build/install breakage caused by tooling that still references the removed ticket-cli package. |
| [dc13ffb4 Define a shared terminal return contract for all sub-agents](.ticket/tickets/dc13ffb4-f172-469c-a0ad-454354aa4f28/ticket.toml) |  | Establish a shared terminal return contract after four wasted sub-agent dispatches returned clarifying questions or false-impossibility claims. |
| [7063311c Author spec for repository architecture and dependency policies](.ticket/tickets/7063311c-a380-4269-adcf-7d1388ab5f39/ticket.toml) |  | Author the root architecture/dependency specification that unblocks five downstream tickets. |

## Target Files
- `AGENTS.md`

## Decisions
- This session produced only the ticket plan; no source-code files changed and no tests were run.
- The planning commit is fb91ca73 (chore(ticket): plan repository-guidance and agent-template learnings epic) on agent/b9020ba2-df5d-426a-b1b9-228ef159cad1/workflow-tools-restructure.
- Ticket dependency semantics were verified: link --from A --to B --kind depends_on writes B into A's depends_on list; B is the prerequisite that blocks A.
- The epic has 30 open child tickets, with 19 depends_on and 5 linked edges and no detected cycles.
- Recommended implementation order: [07a3eb2d Repair build and install tooling referencing removed ticket-cli package](.ticket/tickets/07a3eb2d-8868-4c36-a60a-e93cc787c065/ticket.toml), then [dc13ffb4 Define a shared terminal return contract for all sub-agents](.ticket/tickets/dc13ffb4-f172-469c-a0ad-454354aa4f28/ticket.toml), then [7063311c Author spec for repository architecture and dependency policies](.ticket/tickets/7063311c-a380-4269-adcf-7d1388ab5f39/ticket.toml).

## Non-Goals
- Do not modify source code as part of this handoff.
- Do not touch the uncommitted memory-api/crates/log-api/Cargo.toml or memory-api/crates/test-api/Cargo.toml changes, or the dirty memory-api submodule, until provenance is established.
- Do not rebase or merge the branch until the next session makes that decision.

## Context Anchors
- memory-api/crates/ticket-api/src/storage/store.rs
- memory-api/crates/log-api/Cargo.toml
- memory-api/crates/test-api/Cargo.toml
- AGENTS.md

## Risk Notes
Open items: graph audit flagged the [a74f09cf State CLI binary naming policy as an explicit rule](.ticket/tickets/a74f09cf-2c4b-4c13-9247-cd74519b6b7e/ticket.toml) -> [7063311c Author spec for repository architecture and dependency policies](.ticket/tickets/7063311c-a380-4269-adcf-7d1388ab5f39/ticket.toml) depends_on edge as dubious; user must decide whether to remove the edge or change the relation to linked. Ticket-store integrity is suspected because the CLI reports 2763 global edges while ticket.toml manifests expose only 1867 depends_on/linked entries; no ticket exists and user input is pending. Two observations remain unverified and unticketed: intermittently cancelled/interleaved terminal output and worktree-local checkouts missing target/debug binaries. The branch is neither rebased onto main nor merged.

## Workflow
- **Nodes**: 0
- **Edges**: 0
- **Not Done**: 0

## Validation
- `ticket-plan-only`: not run: no source-code changes; planning commit only (optional)
