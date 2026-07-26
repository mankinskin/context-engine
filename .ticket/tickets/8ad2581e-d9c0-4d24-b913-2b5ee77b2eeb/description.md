## Objective

Define and record the **delegation quality/cost metric** that lets the system compare expensive and cheaper models and find the cheapest model that meets our standards, closing the self-optimization loop.

## "Meets our standards" bar (decided in refinement)

- Use a **rolling-window composite score** combining satisfactory-work rate, cost, and error-recovery cost.
- The exact composite formula and window size are an **open decision to finalize within this ticket**.

## Self-optimization: phased (decided in refinement)

- **This ticket (phase 1)**: recommendation only — the metric surfaces a "cheapest model meeting standards" recommendation; a human approves any change to the T1 cost-class thresholds.
- **Phase 2 (follow-up ticket)**: automated auto-tuning of the delegation thresholds from the metric.

## Requirements

- Compare more expensive vs. cheaper models using the satisfactory-work data from the quality gates (ticket 41ff230b).
- Produce a recordable per-model metric: composite score vs. cost → "cheapest model meeting standards".
- The orchestrating agent, when delegating, must be aware of and collect the evaluation of performance and of the goal solution.

## Acceptance criteria

- A recorded per-model composite metric producing a cheapest-model-meeting-standards recommendation.
- Orchestrator delegation flow captures performance and goal-solution evaluation for each delegated unit.
- The recommendation feeds back (human-approved) into the delegation decision policy thresholds (ticket 373072a9).

## Depends on

- Quality gates + data collection (ticket 41ff230b).
- Session-coupled feedback signal (ticket 9b0147e3).