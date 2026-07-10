---
description: "Research a topic, interview the user, and turn the result into specification-book updates."
name: "interview"
argument-hint: "<topic>"
agent: "agent"
---

<!-- rule-api:file generated=true -->

<!-- rule-api:entry id=1de5c74b-1946-42ec-8b25-b097a62cb3ab slug=context-engine/prompts/interview/l1 -->

# Specification Interview

Create an interview with the user about a specific topic so the specification book can be updated with clearer requirements, goals, and acceptance criteria.

Reference [spec-cli](../../memory-api/tools/cli/spec-cli/README.md), [spec-mcp](../../memory-api/tools/mcp/spec-mcp/README.md), [ticket-cli](../../memory-api/tools/cli/ticket-cli/README.md), and [ticket-mcp](../../memory-api/tools/mcp/ticket-mcp/README.md).

## Workflow

1. Treat the slash-command text as the interview topic.
2. Search existing specs and related tickets before asking questions so the interview starts from current repository knowledge.
3. Summarize the current known state briefly:
- the closest matching spec or gap in the spec book
- related tickets or implementation surfaces
- unresolved requirements that matter for the next update
4. Ask concise, decision-driving interview questions.
5. Prefer questions that refine:
- goals and non-goals
- acceptance criteria
- edge cases and operator expectations
- evidence or validation requirements
6. After the interview, propose the exact spec changes or sections that should be updated.
7. Create or update tickets only when the user asks or when a missing implementation/planning ticket is clearly required by the agreed scope.

## Response

Return:
- topic and current spec anchor
- questions asked or still needed
- confirmed answers from the user
- proposed spec updates and any required ticket follow-up
