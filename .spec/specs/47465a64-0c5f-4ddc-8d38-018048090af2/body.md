# Repository Workflow Guidance

## Goal

Define the required repository workflow for guidance, planning, implementation, validation, documentation, and review.

## Required workflow

1. Create one or more tickets for the requested work before implementation starts.
2. Update or create the relevant spec entry so the new requirements and goals are recorded before code changes proceed.
3. For each ticket:
   - implement the requested change
   - run the required validation until it passes or repeatedly fails
   - update the docs for each affected codebase or generated guidance surface
   - verify the spec links to the docs, the tickets, and the test or validation results
   - move the ticket to `in-review` for peer review
4. Summarize the current status of implementation, validation, and documentation.

## Requirements for guidance surfaces

The generated guidance under `.agents/` and `.github/` should consistently reinforce this workflow.

At minimum, guidance should make the following expectations explicit:

- ticket creation comes before implementation work
- spec updates come before implementation work for new or changed requirements/goals
- validation is required and repeated failures must be reported clearly when they block completion
- docs must be updated for changed codebases and generated guidance surfaces
- review readiness includes verifying that the spec links to the relevant docs, tickets, and test results
- status summaries must cover implementation, validation, and documentation

## Implementation status

- workflow rules were updated in the canonical rule store and regenerated into the repository guidance targets
- partial tooling is accounted for explicitly: dedicated test-tool missing, doc-tool partial, cross-store linking partial
- the guidance now requires the strongest available substitute plus explicit gap reporting instead of silently skipping workflow steps
- follow-up tickets now track the missing workflow tooling surfaces called out by the guidance rewrite

## Traceability

### Completed workflow ticket

- [.ticket/tickets/762d9ac9-e0e0-4f02-b60f-21c79e3c26f6](.ticket/tickets/762d9ac9-e0e0-4f02-b60f-21c79e3c26f6)

### Follow-up tooling tickets

- [.ticket/tickets/02bf9cf0-7e14-46f8-b80a-9e66b38878f9](.ticket/tickets/02bf9cf0-7e14-46f8-b80a-9e66b38878f9)
- [.ticket/tickets/042efd55-80a7-4a79-a821-75972f8886e3](.ticket/tickets/042efd55-80a7-4a79-a821-75972f8886e3)
- [.ticket/tickets/74b32430-cd23-43ad-94dd-086ff752e2b4](.ticket/tickets/74b32430-cd23-43ad-94dd-086ff752e2b4)

### Guidance docs

- [AGENTS.md](AGENTS.md)
- [.github/prompts/spec.prompt.md](.github/prompts/spec.prompt.md)
- [.github/prompts/ticket.prompt.md](.github/prompts/ticket.prompt.md)
- [.github/prompts/tickets.prompt.md](.github/prompts/tickets.prompt.md)
- [.agents/instructions/tests.instructions.md](.agents/instructions/tests.instructions.md)
- [.agents/instructions/ticket-system.instructions.md](.agents/instructions/ticket-system.instructions.md)

### Validation results

- [target/tmp/repository-workflow-guidance-validation.md](target/tmp/repository-workflow-guidance-validation.md)

## Status summary

- Implementation: complete for the targeted canonical rules and regenerated guidance outputs.
- Validation: `rule sync-targets` and `rule sync-targets --check` passed.
- Documentation: AGENTS, ticket/spec prompts, and ticket/test instructions now require the repository workflow.
- Follow-up planning: dedicated tickets now exist for workflow validation tooling, documentation tooling coverage, and cross-store traceability links.