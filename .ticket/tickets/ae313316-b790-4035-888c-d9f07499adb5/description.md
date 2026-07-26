## Goal

Extend the model price-awareness / orchestrator design (feature 445a2d76) with an explicit, self-optimizing policy that governs *when and how* an orchestrating agent delegates versus escalates inside an orchestrated session, and a feedback/metrics loop that drives the system to use the cheapest model that still meets quality standards.

## Motivation (from transcript)

- Define precisely in which cases work is delegated, and to which model **cost class**.
- Define escalation scenarios: when to hand work to a higher-class model, and when to consult the user.
- Core trade-off: model cost vs. output quality (error frequency). Expensive models make fewer errors, hold more context, and fix errors faster/cheaper — but agents mostly fail on *small* problems while losing sight of the original goal.
- Target: minimize/eliminate error spots and fix them continuously so a cheap agent with the right specifications can execute a complex workflow.
- Allocation strategy: use as many small models as possible; reserve large models to capture the whole context once and sequence steps; execute steps, outer-world interaction, unforeseen events, and large-data handling on the smallest viable models.
- The system must self-optimize: collect session/tool-call/delegated-session data, place quality gates before and after sessions, compare models, and record the cheapest model that meets standards.

## Child workstreams

1. Explicit delegation decision policy (case → cost class mapping).
2. Escalation policy (higher class vs. user consultation triggers).
3. Session-coupled feedback signal integration + implementation-status audit.
4. Quality gates + session/tool-call data collection.
5. Delegation quality/cost metric + self-optimization loop.

## Open decisions

- Concrete delegation thresholds and escalation triggers (specific values) are undefined in the source and must be decided.
- Current implementation status of the session-coupled feedback capability is unknown and must be audited before building on it.

## Related

- Depends on / extends feature 445a2d76 (Model price awareness: enforce orchestrator mode for expensive models).