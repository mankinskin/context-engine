---
description: "Start a surgical implementation slice from a ticket, failing behavior, file, or symbol. Anchors on one concrete target, validates immediately, and returns concise evidence."
name: "implement"
argument-hint: "Ticket id, failing behavior, file, symbol, or narrow implementation scope."
agent: "Implement Agent"
---

<!-- rule-api:file generated=true -->

<!-- rule-api:entry id=6e652560-810e-4719-8f92-36634c87a54c slug=shared/implement-prompt/l1 -->

# Implement

Delegate to the Implement Agent for the full surgical workflow.

Use this prompt as a thin wrapper: provide the concrete target, rely on the agent contract for the detailed implementation loop, and return only the evidence-backed summary needed for the user.

Reference [AGENTS.md](./AGENTS.md) and [commit.instructions.md](./.agents/instructions/commit.instructions.md) when the implementing agent needs repository-specific guardrails.
