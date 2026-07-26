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

Extends feature 445a2d76 (price awareness / orchestrator mode). Its cost-class thresholds feed the self-optimization loop in ticket 8ad2581e.