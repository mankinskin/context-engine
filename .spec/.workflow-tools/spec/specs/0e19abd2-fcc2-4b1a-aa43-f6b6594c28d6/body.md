# Agent World Model and Guidance Lifecycle

## Goal

Make the agent's operating model legible as one repository narrative and make the lifecycle explicit in canonical guidance. The lifecycle covers exploration, clarification, decision, execution, world-state change, validation, and return; the narrative also explains maps, resources, probes, context-window focus, testing, and self-improvement.

## Requirements

1. `workflow-tools/.agents/` is the canonical guidance root. `context-engine/.agents/` remains a catalog and evidence area.
2. The repository has one named narrative entry point with an acyclic reading order and repository anchors for every chapter.
3. Canonical guidance assigns exactly one owner to each lifecycle transition and cross-references rather than restating shared rules.
4. Execution distinguishes editing, committing, tool calls, and program execution, with validation evidence required before return.
5. Tool use is distinguished from tool improvement: invoking an existing tool changes the requested world state, while changing the toolset changes future execution behavior.
6. Stale relative references in `context-engine/AGENTS.md` are repaired or retired as part of the guidance rewrite.
7. The formal specification is implemented through linked tickets and validation evidence; no duplicate world-model specification is created.

## Acceptance Criteria

- The canonical-root, legacy-area, stale-reference, and formal-artifact boundary checks are recorded with current results.
- The narrative entry point and chapter documents cover every concept named by the source inputs.
- The transition matrix cites existing guidance for every phase and every flagged overlap has exactly one resolution.
- The guidance rewrite is implemented in bounded ticket-sized changes, with focused validation after each change.
- The final validation records include instruction-link checks, spec health, ticket/spec linkage, and documentation integrity checks.

## Scope Boundaries

The implementation may add or revise repository guidance, narrative documentation, tickets, specs, and validation records needed to satisfy the requirements. It must not move the canonical guidance tree, rewrite unrelated application code, or silently treat historical instruction-link baseline failures as newly introduced regressions.

## Source Artifacts

- `transcripts/04-09-2026_agent-code-world-model/ROADMAP.md`
- `transcripts/04-09-2026_agent-code-world-model/01-world-model-narrative.md`
- `transcripts/04-09-2026_agent-code-world-model/02-agent-state-transitions.md`
- `transcripts/04-09-2026_agent-code-world-model/03-guidance-ownership-and-rewrite.md`
- `transcripts/04-09-2026_agent-code-world-model/04-execution-and-self-improvement.md`
