# Summary

Add a mechanical completeness check for the README contract so missing generation, parent or child navigation blocks, installable-content sections, or command-doc references fail before review.

## Problem

Without a final check surface, the README rollout can regress back into ad hoc review. The same structural gaps that triggered this work would simply reappear in a few weeks under different filenames.

## Scope

This spec covers:

- a repo-level completeness check or audit flow for the in-scope README trees
- the canonical workspace-by-workspace verification commands
- precise failure reporting for missing required README blocks

## Intended Behavior

- The README contract can be checked mechanically.
- The final verification flow includes `sync-targets --check` for every workspace touched by the rollout.
- Missing required README blocks fail with actionable diagnostics.

## Assumptions To Prove

- The final contract can be expressed mechanically enough for a focused audit or validation command.
- Existing rule-target validation plus a small amount of additional logic is enough; a whole separate docs platform is not required.
- The rollout is not complete until the check covers root, `context-stack`, `memory-viewers`, `memory-api`, and `viewer-api`.

## Test Strategy

1. Define the failure cases the audit must detect.
2. Add the smallest mechanical check surface that detects them.
3. Validate the check across all in-scope workspaces after the rollout branches land.

## Acceptance Criteria

- A documented or automated completeness check exists for the in-scope README family.
- Workspace `sync-targets --check` coverage is part of the final validation story.
- Missing parent blocks, child blocks, installable-content sections, or command-doc links fail mechanically.

## Traceability

- [9f14365b README QA ticket](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/9f14365b-fbe5-4f93-a8da-f7f490dacac0/ticket.toml)
