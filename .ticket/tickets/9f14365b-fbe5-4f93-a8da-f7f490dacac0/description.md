# Problem

Even after the rollout lands, the README tree will drift again unless there is a mechanical check for generated ownership, parent and child navigation blocks, installable-content coverage, and direct command-doc references.

## Scope

Add a repo-level completeness check or audit flow that validates the shared README contract across the in-scope workspaces and documents the canonical verification commands.

## Assumptions To Prove

- The final README contract can be checked mechanically rather than only by ad hoc review.
- `sync-targets --check` runs across the root, `context-stack`, `memory-viewers`, `memory-api`, and `viewer-api` are a stable part of the final validation story.
- Missing parent blocks, child blocks, installable-content sections, or command-doc links can be reported precisely enough to drive fixes.

## Test-Driven Plan

1. Define the failure cases the audit must detect.
2. Add the smallest audit or test surface that makes those failures mechanical.
3. Wire the verification flow into the repo-local README maintenance workflow.

## Acceptance Criteria

- A documented or automated README completeness check exists for the in-scope workspaces.
- The final verification flow includes `sync-targets --check` coverage for every workspace touched by the rollout.
- Missing required README blocks fail mechanically rather than depending on manual review alone.

## Validation

- `./target/debug/rule.exe sync-targets --check --config rule-targets.yaml --workspace-root .`
- `./target/debug/rule.exe --workspace-root context-stack sync-targets --check --config context-stack/rule-targets.yaml`
- `./target/debug/rule.exe --workspace-root memory-viewers sync-targets --check --config memory-viewers/rule-targets.yaml`
- `./target/debug/rule.exe --workspace-root memory-viewers/memory-api sync-targets --check --config memory-api/rule-targets.yaml`
- `./target/debug/rule.exe --workspace-root memory-viewers/viewer-api sync-targets --check --config viewer-api/rule-targets.yaml`
