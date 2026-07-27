## Objective

Author explicit agent instructions that specify, in an orchestrated session, **in which cases work is delegated and to which model class**, where classes are defined by **capability role** rather than raw price.

## Cost classes = capability roles (decided in refinement)

Define three capability-role bands, each mapped to a cost-class band that references the generated price table (never hardcoded prices):

- **Reasoner** (large / most expensive): capture the entire context once, strategic decisions, high-level reasoning.
- **Sequencer** (mid): decompose and sequence the individual steps of the work.
- **Executor** (smallest viable): actual step execution, interaction with the outer world, unforeseen events, and large-data handling.

The allocation strategy is: use executors (small models) as much as possible; reserve reasoner/sequencer (large models) for one-time context capture and step sequencing.

## Requirements

- Map concrete work cases to a capability role, and each role to a cost-class band via `tools/model-prices/model_prices.json`.
- Instructions live in the agent system-prompt surface (`AGENTS.md` and/or `.agents/instructions/orchestrator-delegation.instructions.md`), consistent with feature 445a2d76.

## Acceptance criteria

- A documented case → capability-role → cost-class table exists in the orchestrator instruction surface.
- Each case states the driving signal (scope, data volume, error-recovery need) that selects the role.
- Cost-class boundaries resolve through the generated price table, not hardcoded prices.

## Anchor

Extends feature 445a2d76 (price awareness / orchestrator mode). Its cost-class thresholds feed the self-optimization loop in ticket 8ad2581e.## Status Summary

**Implementation complete** — case → capability-role → cost-class mapping added to orchestrator-delegation.instructions.md.

### What was implemented

Added new section "Case → Capability-Role → Cost-Class Mapping" with:

1. **Three Capability-Role Bands** table:
   - Reasoner (T0): One-time context capture, strategic decisions, high-level reasoning
   - Sequencer (T1-T2): Decompose and sequence implementation steps, moderate-complexity editing
   - Executor (T3-T4): Step execution, mechanical edits, read-only triage, error recovery, large-data handling

2. **Work Case Classification** table with 14 concrete work cases:
   - Reasoner cases: strategic planning, task breakdown, conflict resolution
   - Sequencer cases: multi-file features, bug diagnosis, refactors, test authoring
   - Executor cases: single-file edits, searches, docs generation, validation runs, log summarization, error recovery

3. **Driving Signals Detail** table showing how scope breadth, data volume, reasoning depth, error-recovery need, and risk tolerance select the role

4. **Cost class band reference** with output_mtok ranges resolving through tools/model-prices/model_prices.json

### Validation

- File size: 197 lines (+88, -12)
- Git status confirms single file modified: .agents/instructions/orchestration/orchestrator-delegation.instructions.md
- No syntax errors (file is valid markdown)
- Cost-class boundaries explicitly resolve through model_prices.json (no hardcoded prices)
- Tier-step policy added: step up exactly one band on failure (T4→T3→T2→T1→T0)

### Notes for downstream tickets

- Ticket cd19fed4 (scope MCP tool grants per template): Can now reference the Executor role when scoping cheap-model tool grants
- Ticket 66acb737 (declare `model:` per agent template): Can reference these capability roles when assigning default models to agent templates
- Lane D coordination (tickets 46d8b25d, cc3324c9): The work case classification table provides the concrete case inventory for quality gate design

No rule-sync or template-generation commands were required (orchestrator-delegation.instructions.md is a direct-edit instruction file, not a generated artifact).
