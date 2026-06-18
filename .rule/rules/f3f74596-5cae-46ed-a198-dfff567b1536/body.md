---
description: "Continue with the next iteration of the current work using actionable ticket selection and focused validation."
name: "next"
argument-hint: "[current|query|ticket-id]"
agent: "agent"
---

# Next Iteration

Continue the next iteration of work in the same manner as the repository workflow: actionable ticket first, minimal slice second, validation immediately after, and explicit evidence tracking.

Reference [ticket-next](./ticket-next.prompt.md), [ticket-cli](../../memory-api/tools/cli/ticket-cli/README.md), [spec-cli](../../memory-api/tools/cli/spec-cli/README.md), and [audit-cli](../../memory-api/tools/cli/audit-cli/README.md).

## Workflow

1. Check whether there is an active or recently touched ticket that should continue.
2. If not, fall back to the same actionable-ticket discovery flow as `/ticket-next`.
3. Prefer the smallest adjacent follow-up slice that keeps momentum without widening scope.
4. Reconfirm the local hypothesis, the first validation step, and the relevant spec/ticket context before editing.
5. Track validation using `test-api` terms and capture documentation or log references using `doc-api` and `log-api` terms when they apply.
6. If the current track is blocked, say so clearly and propose the nearest unblocked alternative.

## Response

Return:
- the work item being continued or newly selected
- the immediate next slice
- the first validation command or check
- the evidence you expect to record
- the next blocker or handoff point