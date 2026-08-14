# Problem

Measured in one real orchestrator session: of about 58 sub-agent dispatches, at least 6 failed outright and 9 units required re-dispatch. The failures fall into three root-cause classes.

1. **Orchestrator fault**: Three blocked dispatches received the untested command `ticket.exe list ... | head -40`. The command triggers an exit-130 broken-pipe panic, so all three ticket-creation workers aborted before doing work.
2. **Sub-agent fault**: Two read-only workers returned clarifying questions instead of requested deliverables despite complete objectives. One worker declared a ticket retitle impossible because `ticket.exe update` has no `--title` flag, without probing the documented `--field title=...` alternative that a later worker found immediately.
3. **Tooling fault**: The ticket CLI panics with exit 130 when stdout closes early, so ordinary shell pipelines break. Ticket `2e07430b` owns the underlying CLI defect.

Many mechanical ticket-update and retry dispatches were also routed to high-cost models without justification from the tier ladder.

# Requirements

- Add a checkable obligation to orchestrator guidance: any command handed to a worker must have been verified by the orchestrator in the same session, or must carry an explicit `verify before relying on this` marker. An untested command in a dispatch prompt is an orchestrator defect.
- Add a consistent failure taxonomy to the guidance corpus: orchestrator fault, sub-agent fault, and tooling fault.
- Require a worker that believes a task is impossible to probe at least one alternative mechanism and report the attempted probe before returning a blocker.
- Require retry dispatches of mechanical units not to escalate model tier by default. Tier escalation is only for quality insufficiency and must be justified in dispatch rationale.

# Scope Note

Do not restate the ticket CLI broken-pipe defect as new guidance ownership. Reference ticket `2e07430b` as the owning ticket for that defect.