## Gap

Ticket 66acb737 declared `model:` frontmatter on all 16 `.agents/agents/*.agent.md` templates and documented (in model-routing.instructions.md, "Per-Template `model:` Declaration") two behavioral rules:

- AC2: `runSubagent` without an explicit `model` resolves to the template's declared tier.
- AC4: any override *above* the declared tier is recorded with a one-line reason in the session record.

Both are documented conventions, not code-enforced — `runSubagent`'s model dispatch is a VS Code host-level mechanism with no in-repo dispatcher code to instrument (same reasoning cd19fed4 used to accept "documented reason, not repo-controlled" for its AC4).

## Ask

Once ticket 10d21210 (synthetic benchmark session) lands, use it (or a lightweight standalone probe) to empirically confirm:

1. A bare `runSubagent` call against a template with no explicit `model` actually resolves to that template's declared tier in the resulting session/transcript record.
2. An intentional above-tier override during a benchmark run produces a recorded one-line reason in the session record, and a downward override does not require one.

If either check fails, escalate back to a spec (ec3b13f1 or a new one) for code-level enforcement, since the "sufficient contract surface = instruction file" reasoning in 66acb737 explicitly assumed convention-following was enough.

## Traceability

- Depends on: `.ticket/tickets/10d21210-7168-4ed4-8e99-f6fb0e6e08db` (benchmark not yet built).
- Verifies: `.ticket/tickets/66acb737-71d6-4585-a921-b597f7c88e8e` AC2 and AC4.
- Related: `.agents/instructions/orchestration/model-routing.instructions.md` ("Per-Template `model:` Declaration" section).
