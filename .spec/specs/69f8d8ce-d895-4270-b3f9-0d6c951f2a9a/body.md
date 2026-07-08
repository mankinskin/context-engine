<!-- aligned-structure:v1 -->

# Summary

Define a reusable generated prompt under `.agents/prompts/reviews.prompt.md` for reviewing the highest-ranked `in-review` tickets using the repository ticket workflow.

## Behavior Story

Define a reusable generated prompt under `.agents/prompts/reviews.prompt.md` for reviewing the highest-ranked `in-review` tickets using the repository ticket workflow.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# Reviews Prompt Target

## Purpose

Define a reusable generated prompt under `.agents/prompts/reviews.prompt.md` for reviewing the highest-ranked `in-review` tickets using the repository ticket workflow.

## Behavior

The prompt must instruct the agent to:

- discover `in-review` tickets using ticket-system tooling, preferring the ranking surface already exposed by the ticket system
- process tickets in ranked order with a configurable per-call limit
- default the limit to `5`
- support an unbounded mode with values such as `infinite`, `endlessly`, or `until no more candidates are found`
- stop immediately when no `in-review` candidates are available
- for each selected ticket, audit ticket quality, gather context from related tickets and specs, inspect referenced code, verify acceptance criteria, and decide whether the ticket should be moved back to implementation or closed

## Tooling expectations

The prompt must direct the agent to use:

- ticket-system tools for candidate discovery, ranking, context, and state transitions
- spec tools for requirement and reference research
- audit tools for repository or code-quality review where applicable

## Generated surface

- Output path: `.agents/prompts/reviews.prompt.md`
- File kind: `.prompt`
- Repo scope: `context-engine`

## Validation

The target must be generated through the rule system and pass the target check flow after generation.

## Traceability

- Ticket: `C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/4c937720-4e35-4db6-bce7-608fdad5b6c5`
- Validation commands:
	- `./target/debug/rule.exe explain-target --config rule-targets.yaml --target context-engine-prompt-reviews --json`
	- `./target/debug/rule.exe generate-target --config rule-targets.yaml --target context-engine-prompt-reviews --json`
	- `./target/debug/rule.exe generate-target --config rule-targets.yaml --target context-engine-prompt-reviews --check --json`
