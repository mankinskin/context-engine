---
description: "Start a surgical implementation slice from a ticket, failing behavior, file, or symbol. Anchors on one concrete target, validates immediately, and returns concise evidence."
name: "implement"
argument-hint: "Ticket id, failing behavior, file, symbol, or narrow implementation scope."
agent: "Implement Agent"
---

# Implement

Make the smallest correct change that satisfies the requested behavior, validate it immediately, and return a concise evidence-backed summary.

Reference [AGENTS.md](./AGENTS.md) and [commit.instructions.md](./.agents/instructions/commit.instructions.md).

## Scope

- Implement narrow fixes and bounded features once the target slice is clear.
- Work from the owning code path, nearby tests, and existing guidance instead of broad repo tours.
- Keep edits small, local, and reversible until the first focused validation passes.
- Update nearby tests, docs, specs, and ticket evidence only when the changed behavior requires it.

## Constraints

- Prefer surgical edits over broad refactors.
- Do not spend tokens on long research or narration once you have the owning slice.
- Before the first edit, gather only enough context to state one falsifiable local hypothesis and one cheap disconfirming check.
- After the first substantive edit, run the narrowest focused validation before more reading or patching.
- If the first validation fails, repair the same slice or take one nearby hop to the controlling code path; do not reopen broad exploration.
- Keep status output brief and implementation-focused.
- Stop and ask for direction only when a focused search still leaves a material product or architecture ambiguity.

## Required Workflow

1. **Resolve session-backed worktree first.** Before starting implementation, check into the session tool to obtain a session record and authoritative worktree working directory. Never share the root checkout staging area across parallel sessions.
2. **Anchor on a concrete ticket.** Check the nearest owning code path, related ticket/spec context, and one neighboring test or call site.
3. **State one local hypothesis** and the first cheap falsifying check.
4. **Make the smallest grounded edit** that tests or implements that hypothesis.
5. **Run the first focused validation** immediately after that edit.
6. **Iterate locally** until the slice is correct, then summarize the result and evidence with minimal extra narration.

## Worktree-First Session Rules
- **Session Check-In**: New sessions must check into `session-api` to receive their authoritative working directory.
- **Board Coordination**: Perform draftboard check-in and file claims *after* worktree assignment, targeting the assigned working directory.
- **Reuse vs Rotation**: Same-session revival can reuse a healthy assignment. Cross-session handoffs or invalid worktrees rotate to a new worktree and record predecessor lineage.
- **Stop/Handoff Capture**: Stop hooks capture evidence and transcript state, but do not reassign worktree ownership implicitly.

## Response

Return:
- implementation target and owning slice
- hypothesis and first check
- edits made
- validation run
- remaining risk, if any
- next action or done
