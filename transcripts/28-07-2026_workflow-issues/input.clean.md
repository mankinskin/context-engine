# Workflow and Tooling Issues to Fix

There are several problems with how we use tickets and specs, and with how our agents use those tools. Grouped by area, with relevant existing references found in the repo.

## 1. Ticket and spec update semantics

- Some agents seem to think they can update a ticket description while preserving the previous content, but the update actually overwrites it, and the ticket content is lost. This does not raise an error, so it is silent data loss.
- Specs have a related problem: it is currently possible to send an update with empty content, and the call still succeeds. In complex workflows, this forces extra verification just to confirm whether anything was actually updated. An empty spec update should be rejected outright, so agents can trust that a successful call really changed something.
- Reference: [.agents/instructions/spec/spec-system.instructions.md](.agents/instructions/spec/spec-system.instructions.md) currently only says specs should be created or updated when requirements change; it does not yet document rejecting empty/no-op updates. This is the natural place to add that rule once the underlying store behavior is fixed.

## 2. Handoff packages need a persistent step graph

- Handoffs currently lack a persistent workflow step list or step graph that encodes the next steps. That is arguably the most important part of a handoff, and it is missing right now.
- At minimum, a handoff should contain an explicit list of the next steps. Ideally, it should contain a complete graph, so the next session can pick up a real plan and execute it end to end instead of starting over.
- Reference: [.agents/instructions/session/session-workflow.instructions.md](.agents/instructions/session/session-workflow.instructions.md) already defines a durable session workflow graph (nodes, edges, statuses) for multi-step work. Handoff packages should be wired to reuse this mechanism instead of leaving next-steps as unstructured prose.

## 3. Duplicate entity creation instead of delta updates

- Multiple orchestrated agents created the same tickets in sequence: the top-level orchestrator had a ticket track created by one session, reviewed that ticket track, and then a new sub-agent duplicated many of the same tickets one-to-one. This shows agents are not reliably checking what already exists before acting.
- The default behavior should be to treat every requested change as a delta from the current state — for tickets, specs, or any other entity. A new entity should only be created when it is truly necessary.
- Reference: [.agents/instructions/ticket/workflow.instructions.md](.agents/instructions/ticket/workflow.instructions.md) already states "Always search for existing tickets before creating new ones. Duplicate tickets ..." — this rule exists but was not followed in the observed session, so it needs stronger enforcement or a clearer trigger point (e.g. before an Implement Agent is dispatched after a ticket-track review).

## 4. Interruption recovery

- We need instructions, and possibly a separate prompt template, for recovering after an interruption. In one observed case, a sub-agent's run was interrupted; when the orchestrator was told to continue, it assumed the sub-agent had already finished and moved on to the next step instead of resuming it.
- The correct behavior is for the next agent to find the loose ends and resume at the right point, not assume an interrupted part is complete or restart from a rough boundary.
- This should be documented explicitly, and a prompt template for "resume an interrupted agent" would make it easy to apply consistently.

## 5. New Research Agent for online search

- We need an explicit Research Agent whose job is online search, distinct from the existing repo-scoped Research/Explore agents.

## 6. MCP cost gate: model resolution should tolerate common naming deviations

- The cost gate ([memory-api/tools/mcp/mcp-cost-gate/src/gate.rs](memory-api/tools/mcp/mcp-cost-gate/src/gate.rs)) already resolves `caller_model` with an exact match first, then a case-insensitive substring match against `provider_id`/`model_id` in the price table (`resolve_output_mtok`). If nothing matches, the call is rejected outright via `unknown_model_guidance`.
- The problem: some clients pass the model string with the agent client appended in parentheses, for example `"Claude Sonnet 5 (copilot)"`. Because the price-table id uses hyphens and no suffix (e.g. `claude-sonnet-5`), neither the caller string nor the table id is a substring of the other once spaces, casing, and the trailing `(copilot)` are involved — so the whole call is rejected even though the intended model is unambiguous.
- Intended fix: before substring matching, normalize the caller-supplied model string — strip a trailing parenthetical agent/client qualifier such as `(copilot)`, and normalize separators (spaces/underscores to hyphens) — and apply a small amount of structural understanding of how models are typically named/identified, so minor formatting deviations don't cause the entire tool call to be rejected. This keeps the "reject unknown models" safety property while no longer punishing harmless formatting differences.