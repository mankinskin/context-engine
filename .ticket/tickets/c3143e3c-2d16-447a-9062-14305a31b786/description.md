# [Board][Design] Stale-Entry Review, Cleanup Approval, and Conflict Resolution Workflow

## Objective

Define the human-in-the-loop workflow for stale entries, explicit cleanup, and file ownership conflicts.

The current draftboard design says cleanup should be explicit and agents should seek user permission before removing stale entries, but it does not yet specify how that permission is represented, how stale/conflict cases are reviewed, or which resolution actions exist.

## Why This Is Missing but Critical

This is where the system will fail first in practice. Stale entries are inevitable:
- agents crash
- sessions disconnect
- users pause work without checking out
- heartbeats stop during long-running or blocked work

If stale cleanup and conflict resolution are not carefully designed, agents will either:
- ignore stale entries and deadlock the board, or
- over-delete entries and break coordination history

## Questions to Resolve

### 1. Cleanup authorization
- How does an agent present stale/completed entries for user review before cleanup?
- Do we need a two-step flow such as preview → confirm?
- Should `board clean` support a dry-run mode or confirmation token?
- How is approval represented in MCP, where interactive prompts do not exist?

### 2. Stale-entry handling
- What exactly happens when an entry crosses the one-hour stale threshold?
- Is there a grace period between "warning" and "cleanup-eligible"?
- What output should `board show`, `next`, and `status` display for stale entries?
- When should the operator be urged to renew instead of remove?

### 3. Conflict resolution actions
- What explicit actions can the operator take when two entries conflict on files?
- Candidate actions:
  - renew existing entry
  - release specific files from an entry
  - transfer ownership to another agent
  - force override with audit log
  - mark one side abandoned and clean it
- Which of these are supported in v1?

### 4. Auditability
- What audit record is retained when stale entries are renewed, cleaned, overridden, or transferred?
- Do resolution actions belong in board state only, lease history, ticket history, or all three?
- How should human approval be traceable after the fact?

## Deliverables

- Cleanup approval workflow for CLI and MCP
- Stale-entry review states and operator guidance text
- Conflict resolution action set for v1
- Audit requirements for cleanup and override operations
- Recommendation for the simplest safe v1 cleanup UX

## Acceptance Criteria

- [ ] Explicit cleanup approval flow is defined for CLI and MCP use cases
- [ ] `board clean` semantics distinguish preview/review from destructive cleanup
- [ ] One-hour stale-entry escalation behavior is fully specified
- [ ] Operator guidance text is defined for stale entries in `board show`, `next`, and `status`
- [ ] Conflict resolution actions are defined and scoped for v1
- [ ] Audit requirements are documented for renew, clean, override, and transfer actions
- [ ] `8aff39cb` is updated to reference the approved stale-entry and cleanup workflow
