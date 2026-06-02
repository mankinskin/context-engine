---
description: "Coach the user from an idea to a concrete ticket/spec/validation workflow batch."
name: "user-training"
argument-hint: "<goal or work idea>"
agent: "agent"
---

# User Training

Work with the user to shape new work they want to finish, then guide them through the repository workflow from discovery to ticketing, specification, validation, and review.

Reference [AGENTS](../../AGENTS.md), [tickets prompt](./tickets.prompt.md), [spec prompt](./spec.prompt.md), [ticket-cli](../../memory-viewers/memory-api/tools/cli/ticket-cli/README.md), and [spec-cli](../../memory-viewers/memory-api/tools/cli/spec-cli/README.md).

## Workflow

1. Treat the slash-command text as the user's initial goal.
2. Clarify the intended outcome, constraints, and urgency with concise questions.
3. Search for existing tickets, specs, prompts, or code that already cover the same work.
4. Turn the goal into a practical workflow batch:
- recommended tracker or child tickets
- whether an existing spec should be updated or a new one created
- the first actionable implementation slice
- the initial validation strategy
5. Keep the batch small enough that the first ticket can be implemented and validated immediately.
6. Explain how `test-api`, `doc-api`, and `log-api` evidence should be captured for the slice when those concepts matter.
7. End with one concrete next step rather than a long backlog.

## Response

Return:
- the shaped work item in one sentence
- the recommended ticket/spec batch
- the first actionable ticket and why it goes first
- the initial validation and evidence plan
- the next step for the user
