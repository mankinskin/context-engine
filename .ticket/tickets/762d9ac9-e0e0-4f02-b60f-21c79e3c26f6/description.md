# Problem

The repository guidance does not consistently enforce one workflow for normal engineering work.

The intended workflow for this repository is:

1. Create one or more tickets for the work.
2. Update the spec with the new requirements and goals.
3. For each ticket: implement the change, run the required validation until it passes or repeatedly fails, update docs for the changed codebases, verify the spec links to the docs, the tickets, and the test results, then move the ticket to `in-review` for peer review.
4. Summarize the status of implementation, validation, and documentation.

Right now those expectations are fragmented across AGENTS rules, ticket/spec prompts, and ticket-system instructions. Some surfaces do not mention the spec step at all; some mention tests or docs but not how they relate to tickets and specs; and some guidance covers review-state transitions without requiring the spec/doc/test linkage that should exist before review.

# Scope

Update the canonical workflow guidance so the generated files under `.agents/` and `.github/` consistently require the repository workflow above.

This includes:

- shared AGENTS workflow rules that govern routing and quality gates
- ticket/spec prompt rules under `.github/prompts/`
- ticket-system instructions under `.agents/instructions/`
- any adjacent generated guidance needed to keep the workflow coherent after regeneration

# Acceptance criteria

- A root spec entry documents the repository workflow requirements and goals for ticket creation, spec updates, validation, documentation, review, and status reporting.
- Canonical rule entries are updated so generated guidance under `.agents/` and `.github/` consistently reflects the workflow.
- Generated guidance files are regenerated from the canonical rule store.
- Validation confirms the regenerated guidance is in sync.
- The implementation summary reports status for workflow guidance changes, validation, and documentation updates.
