# Handoff: 1c8c5e9d-c294-4b03-88af-697dd5b664d1

## Summary
- **Workspace Session**: `8ba9f3a5-39d2-4d74-a61e-8e26ad048f6b`
- **Outgoing Run**: `b71d26b3-112d-451a-85f6-d630fa291ee8`
- **Created**: 2026-07-28T22:44:09.871363500+00:00
- **Objective**: Restore the caller_model naming-deviation tolerance in mcp-cost-gate by re-applying commit 53de6c5's implementation, which was accidentally erased by c58e9be, then verify the restored behavior against committed HEAD rather than the working tree.
- **Implementation Ready**: true

## Resume Command
```bash
session-cli resume --workspace-session-id 8ba9f3a5-39d2-4d74-a61e-8e26ad048f6b --predecessor-run-id b71d26b3-112d-451a-85f6-d630fa291ee8
```

## Target Tickets
- `32067e83-7c60-40b7-9d2e-4c419020adcf`

## Target Files
- `memory-api/tools/mcp/mcp-cost-gate/src/proxy.rs`
- `memory-api/tools/mcp/mcp-cost-gate/src/gate.rs`

## Decisions
- Commit c58e9be is treated as an ACCIDENTAL clobber (bad rebase, cherry-pick, or merge-conflict resolution), not a deliberate revert. User decision: re-apply 53de6c5's diff cleanly on top of HEAD and re-validate. Do not re-open design discussion on whether the feature is wanted.
- Criterion 10 (docs and examples) remains WAIVED by prior user decision: satisfied by the tools list schema description in inject_caller_model_schema; no mcp-cost-gate README exists. Remaining instruction-doc drift is split out to ticket 4ca4ce83-3baa-4676-8001-bc72b2c99352.
- Restoration must be verified against HEAD after committing, not against the working tree. This ticket's entire failure mode was a reviewed tree being superseded before close.
- The unknown_model_guidance text in gate.rs currently advertises tolerance that does not execute. Restoring the proxy.rs fallback makes it accurate again; if for any reason the fallback is not restored, that text must be corrected instead of left misleading.

## Non-Goals
- Do not redesign the normalization rules or re-litigate whether caller_model tolerance is desirable; 53de6c5 already implemented the agreed behavior.
- Do not touch the session-api persistence config file at memory-api crates session-api src store config persistence.rs.
- Do not change exact-match or substring-match precedence in gate.rs; normalization is fallback-only, after both fail.
- Do not address the telemetry-collection gap (4aa13ba7) or the session_tool_metrics crash (574560bf) here.

## Context Anchors
- Commit 53de6c5 in the memory-api submodule contains the complete, correct implementation (gate.rs plus 41 lines, proxy.rs plus 147 lines). Recover the diff from there rather than reimplementing from the acceptance criteria.
- Commit c58e9be, eight minutes later, is the clobber: proxy.rs plus 3, minus 144. Its message reads as if adding the feature. Current submodule HEAD for these files is e73687b.
- gate.rs was NOT touched by c58e9be, so resolves(), available_model_ids() and the updated unknown_model_guidance text still exist in HEAD. Only the proxy.rs side needs restoring. Check for a partially-applied state before re-applying, or you will duplicate gate.rs symbols.
- Absent from HEAD and needing restoration: normalize_caller_model, the fallback retry branch in handle_client_message, the costGateWarning injection in handle_server_message, the allow-normalized telemetry decision label, the updated description in inject_caller_model_schema, and three normalization unit tests.
- Current test baseline is 51 passed with 0 failed. The ticket's original handoff claimed 54, which equals 51 plus the three deleted normalization tests. Restoring them should return the suite to 54.
- The ticket's prior review_notes recorded a fabricated PASS and have been superseded in place; do not treat them as evidence.
- Resolution semantics are pinned by spec 9f0b9e30-e32c-4092-b2a2-68179141cfc4.
- Ticket 32067e83 was moved from done back to in-implementation on 2026-07-28 as part of this finding.

## Risk Notes
This ticket has already been falsely closed once on a fabricated review. Do not report completion from a green working tree: after committing, re-read the target files at HEAD and confirm the restored symbols are present, and re-run the suite from that committed state. The specific trap here is that a later commit silently reverted an earlier correct one within the same working session, so a check performed before the final commit proves nothing.

## Workflow
- **Nodes**: 0
- **Edges**: 0
- **Not Done**: 0

## Validation
- `mcp-cost-gate-suite`: 54 passed and 0 failed, up from the current 51, with the three restored normalization tests among them (required)
- `persistence-rs-untouched`: empty (required)
- `restoration-present-at-head`: symbol present at HEAD, not merely in the working tree (required)
