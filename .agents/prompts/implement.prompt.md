---
description: "Start a surgical implementation slice from a ticket, failing behavior, file, or symbol. Anchors on one concrete target, validates immediately, and returns concise evidence."
name: "implement"
argument-hint: "Ticket id, failing behavior, file, symbol, or narrow implementation scope."
agent: "Implement Agent"
---

<!-- rule-api:file generated=true -->

<!-- rule-api:entry id=6e652560-810e-4719-8f92-36634c87a54c slug=shared/implement-prompt/l1 -->

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

1. Anchor on a concrete ticket, failing behavior, file, symbol, or generated target.
2. Check the nearest owning code path, related ticket/spec context, and one neighboring test or call site.
3. State one local hypothesis and the first cheap falsifying check.
4. Make the smallest grounded edit that tests or implements that hypothesis.
5. Run the first focused validation immediately after that edit.
6. Iterate locally until the slice is correct, then summarize the result and evidence with minimal extra narration.

## Response

Return:
- implementation target and owning slice
- hypothesis and first check
- edits made
- validation run
- remaining risk, if any
- next action or done
