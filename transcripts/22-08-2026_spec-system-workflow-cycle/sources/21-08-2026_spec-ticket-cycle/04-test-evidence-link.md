# 04 — Document the Test-Evidence Link as the Cycle's Validation Step

## Outcome

A short clarification (folded into work package 01's instruction file, not a separate file) states explicitly that test-api executions already close the cycle's "correctness of executable measurements" step by linking to both `spec_ids` and `ticket_ids`.

## Description

The transcript: "the acceptance criteria are checked in a sandbox system or a unit test suite... those tests are then used to validate the implementation in later tickets... and to ensure that the specification is actually fulfilled." `ARTIFACTS.md` confirms `mcp_test-mcp_record_execution`/`record_spec` already support `spec_ids`, `ticket_ids`, and `acceptance_criterion_ids` — this capability already exists; the gap is that nothing currently states it is *the* mechanism closing this specific step of the cycle.

## Non-Goal

Do not change test-api's fields or behavior. Do not write a new test-api instruction file — one line inside work package 01's cycle file, cross-linking to the existing test-api usage docs, is sufficient.

## Validation Method

Manual read-through confirming the cross-link is accurate and points at real, current test-api tool descriptions (`mcp_test-mcp_record_execution`, `mcp_test-mcp_record_spec`).
