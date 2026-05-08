---
agent: agent
description: "Research an unfamiliar module or behavior before implementation using docs, tickets, tests, and source traversal."
---

# Research Workflow

Use this workflow when module behavior is unclear or the task requires non-trivial context gathering before coding.

## Steps

1. Define the research question
- Write a short problem statement and the modules likely involved.

2. Collect documentation context
- Search project docs and crate-level guides for relevant APIs and invariants.
- Check `CHEAT_SHEET.md` for known gotchas.

3. Check issue history
- Review ticket history for related bugs, plans, or design notes.

4. Inspect source and tests
- Read the implementation and adjacent tests to infer expected behavior.
- Track data flow and boundaries across involved crates/tools.

5. Synthesize findings
- Summarize known facts, unknowns, and likely solution paths.
- Identify risks and required validation steps.

6. Decide next action
- If clear: proceed with implementation plan.
- If still ambiguous after focused research: ask the user.

## Output Format

Provide a concise handoff with:
- scope touched
- key findings
- assumptions
- recommended implementation approach
- validation plan
