## Problem
Sub-agents currently receive the full applicable instruction corpus for their mode regardless of the specific ticket's domain, paying token cost for irrelevant sections (e.g. a database-migration worker still reads frontend/Playwright guidance).

## Goal
Define (spec or instruction, to be decided during implementation) a mechanism to inject only the instruction subset relevant to a ticket's tags/domain into a worker's context:
- Derive the relevant instruction subset from ticket tags/type/component rather than the full applyTo-matched set.
- Document how this interacts with the existing applyTo-pattern instruction-loading mechanism described in AGENTS.md and copilot-instructions.md so it narrows rather than replaces it.

## Acceptance criteria
- A concrete mechanism (even if manual/documented-convention rather than automated) exists for narrowing injected guidance by ticket domain tag.
- Documented interaction with the existing applyTo pattern mechanism, with no contradiction between the two.
- At least one worked example showing full corpus vs. narrowed corpus for a sample ticket domain.

## Source
Derived from AGENT_WORKFLOW_OPTIMIZATIONS.md conversation, "Step 3: Optimizing Guidance and System Files", "Dynamic Prompt Injection".
Review verdict: approve-with-fixes; meta-glob defect fixed in follow-up; all 4 acceptance criteria met; verification grep: all 20 files have `applyTo`, 8 of them `"**"`, zero remaining `.agents/**`.