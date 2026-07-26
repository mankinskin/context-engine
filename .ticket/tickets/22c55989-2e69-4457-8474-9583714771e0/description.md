## Objective

Define the escalation half of the policy: when the orchestrating agent should stop delegating downward and instead **escalate to a higher capability role / cost class**, and when it should **consult the user**.

## Triggers (decided in refinement)

Escalate UP a cost class when **either** signal fires:

- **Failure/retry count**: N failed delegated attempts at the current class (N to be fixed during implementation).
- **Quality-gate outcome**: the post-session quality gate (ticket 41ff230b) fails for the delegated unit.

Consult the **user** only for genuine ambiguity or conflicting evidence that remains unresolved after focused delegation — reserved for judgment calls, not routine failure.

## Acceptance criteria

- Documented escalation-trigger list distinguishing "move up a cost class" (failure count OR quality-gate failure) from "consult the user" (unresolved ambiguity / conflicting evidence).
- Triggers are observable: tied to failure counts and quality-gate outcomes, not subjective judgment.
- Consistent with the existing escalation rules in AGENTS.md and the orchestrator delegation failure path.

## Open decision

- Exact failure-count N before moving up a class (to be fixed during implementation).

## Anchor

Depends on the delegation decision policy (ticket 373072a9) and consumes quality-gate outcomes from ticket 41ff230b.