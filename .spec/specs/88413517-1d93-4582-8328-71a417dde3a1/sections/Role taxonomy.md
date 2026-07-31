Every session classifies into exactly one of the following 14 roles. Each role is defined by: purpose, typical trigger, allowed write scope, and default model tier (tier names per the model-routing spec: frontier / worker / lightweight — see Constraints section).

| Role | Purpose | Typical trigger | Write scope | Default model tier |
|---|---|---|---|---|
| R1 | Artifact search | "find/where is X" across tickets/specs/docs/logs | read-only | lightweight |
| R2 | Global state overview | cross-workspace board state, state distribution, highest-priority next item | read-only | lightweight |
| R3 | Ticket-track research | investigate a ticket/track before committing to a plan | read-only | worker |
| R4 | Deep reading | audit runs, health-checks, subgraph + edge inspection | read-only | worker |
| R5 | Deep writing | artifact create/modify, multi-session planning, transition review | writes (tickets/specs/docs) | frontier |
| R6 | Acceptance testing | acceptance-test execution, tool-gap/UX-friction capture | writes (test evidence, feedback) | worker |
| R7 | Executable specs + regression harness | authoring/running regression suites tied to specs | writes (test/spec evidence) | worker |
| R8 | Telemetry | cost/latency/outage measurement | read-only (measurement only) | lightweight |
| R9 | Task refinement | plans, decision records, dependency + DAG shaping | writes (tickets) | frontier |
| R10 | Task execution | implementation work along the DAG | writes (code + tickets) | worker |
| R11 | Change review | regressions/deviations/failures review | read-only (comments/state transitions only) | worker |
| R12 | Strategic planning | benefit + effort estimation, global sequencing | writes (specs/tickets, planning only) | frontier |
| R13 | Simplification | consolidation/dedup of existing artifacts | writes (artifacts, no new scope) | worker |
| R14 | Orchestration + escalation | pure delegation, escalate ambiguity to user | read-only (never acquires write tools) | frontier |

Roles are mutually exclusive by construction: a session resolves to exactly one role via the routing contract (section 4), never a blend.