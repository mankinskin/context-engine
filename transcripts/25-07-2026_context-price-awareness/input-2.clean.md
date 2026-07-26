# Delegation & Escalation Policy: Cost/Quality Trade-off and Self-Optimization

## Idea

Extend the price-awareness / orchestrator design by specifying, precisely, the rules that govern delegation and escalation inside an orchestrated session:

- Define exactly in which cases work should be delegated, and to which model — or, more specifically, to which model class / cost class.
- Define in which scenarios the agent should escalate instead: when to hand the work to a model of a higher class, and when to consult the user.
- Provide the agent with explicit instructions that explain, within an orchestrated session (an orchestrated delegation), *when* and *how* it should delegate versus act directly.

## Feedback Signal

We can also take into account that giving feedback is possible — flagging specific problem spots and drawing attention to particular scenarios. This feedback is mostly coupled to a session.

- This capability may still need work; we need to check how far its implementation has progressed.

## Core Trade-off

The driving concern is the trade-off between a model's cost and the quality of its output — equivalently, the frequency of its errors.

- More expensive models tend to make fewer errors, can consider more context, and can fix errors faster or even more cost-effectively.
- However, agents mostly fail at implementing small problems without losing sight of their original goal or losing sharpness. Larger models are usually better at solving these small errors, but they are significantly more expensive.
- Therefore we should minimize or eliminate error spots as far as possible and fix them continuously, so that even a cheap agent — given the right specifications — can carry out a complex workflow without problems.

## Model Allocation Strategy

- Overall, use as many small models as possible.
- Use the large models only to capture the entire context once and to sequence the individual steps.
- The actual implementation of the steps — interaction with the outer world, with unforeseen events, or with large amounts of data — must be carried out by the smallest models possible.

## Self-Optimization and Metrics

The system must optimize itself to gradually make better use of the models' quality and cost.

- For this optimization, collect data from the sessions, from the tool calls, and from the delegated sessions.
- Place quality gates before and after the sessions to understand how often the delegated sessions produce satisfactory work.
- This lets us compare more expensive and cheaper models and find the cheapest model that meets our standards — a metric we can record.
- Consequently, when delegating, the orchestrating agent must be aware of the evaluation of performance and of the goal solution, and must also collect these.

## Open Questions

- The concrete delegation thresholds (which cost class maps to which case) and escalation triggers (when to move up a class versus consult the user) are described as goals to define, but their specific values were not given in the transcript.
- The current implementation status of the session-coupled feedback capability is unknown and needs to be checked.
