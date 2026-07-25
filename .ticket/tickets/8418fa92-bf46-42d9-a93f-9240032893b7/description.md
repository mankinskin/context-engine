## Summary

Add an explicit **orchestrator agent** whose only capability is to call sub-agents. It performs high-level planning, dispatches work to multiple sub-agents, and aggregates their results back into the large (expensive) session — never doing routine execution itself.

Part of: [445a2d76 Model price awareness: enforce orchestrator mode for expensive models](../445a2d76-5795-4d7a-aec8-d1536ec61416/ticket.toml).

## Motivation

The AGENTS.md orchestrator rule is prose the model must voluntarily follow. A dedicated orchestrator agent makes the behavior structural: by exposing only a single "call sub-agent" tool, an expensive model physically cannot perform token-heavy direct work and is forced into planning + delegation + aggregation.

## Approach

- Define an orchestrator agent template that exposes exactly **one tool**: invoke/spawn a sub-agent (`runSubagent` or equivalent) with an explicit cheaper model.
- The orchestrator's responsibilities: decompose the task, plan at a high level, delegate each unit to a sub-agent, and aggregate/synthesize sub-agent results into the parent session.
- No file read/edit, no other MCP tools, no direct command execution on the orchestrator itself.
- Sub-agents run on cheaper models and return compact results the orchestrator composes.

## Scope / Deliverables

1. An orchestrator agent definition (`.agent.md` or equivalent) restricted to the single sub-agent tool.
2. High-level planning + result-aggregation guidance in the agent body.
3. Wiring so that when an expensive model would otherwise engage (per threshold `X`), the orchestrator agent is the entry point.
4. Alignment with the model-aware MCP wrapper so delegation targets cheaper models.

## Acceptance Criteria

- The orchestrator agent has exactly one tool (spawn sub-agent) and cannot read/edit files or call other MCP tools directly.
- It plans, delegates to sub-agents on cheaper models, and aggregates results into the session.
- Its use is triggered for models above threshold `X`.

## Update (2026-07-25)

**Threshold resolved (inherited from T-PARENT):** orchestrator entry is triggered when the model's `output_mtok > 15` USD per 1M output tokens (= 1500 credits/1M).

**BLOCK-4 investigation (sub-agent primitive) — largely resolved.** `runSubagent` already exists as the runtime sub-agent primitive: it appears in captured session transcripts (a "Delegate an independent code or documentation review to a subagent" tool) and AGENTS.md already documents invoking it "with an explicit cheaper `model`". So the single-tool agent should expose `runSubagent(model=<cheaper>, …)`. Remaining sub-decisions for implementation: multi-sub-agent result merge (sequential vs. parallel + reconciliation format) and whether the orchestrator is a distinct `.agent.md` template vs. a constrained mode of existing agents. Recommend a distinct `.agent.md` template exposing only `runSubagent`, with sequential dispatch + compact result aggregation. Depends on T-WRAP so delegated calls flow through the model-aware wrapper.

## Open Questions

- ~~Exact sub-agent invocation primitive~~ → resolved: `runSubagent` with explicit `model`.
- How results from multiple sub-agents are merged (sequential vs. parallel, and reconciliation format).
- Whether the orchestrator is a distinct agent template or a constrained mode of existing agents.