# Roadmap v6 Archive — Stage 3 Planning Baseline

This archive preserves the Stage 3 planning baseline that preceded the Stage 4
work-package route. The active roadmap at the time recorded the component-oriented
specification-system outcome, the completed specification and ticket-planning
waypoints, the missing UUID-to-`component_id` mapping and journaled migration
runner, the self-migration target, documentation and presentation follow-up, and
the final validated-response cycle.

Stage 4 supersedes the active entrypoint because live evidence changed the route:

- Waypoint 6 ticket creation is complete; the W6.x ticket set is open in the
  `.workflow-tools` ticket store.
- [30bbbace W6.1](../../../.workflow-tools/ticket/tickets/30bbbace-7ce3-4f7e-acb4-202d08cf91d7/ticket.toml)
  remains blocked by authoritative open dependency
  [73b2cd22](../../../.workflow-tools/ticket/tickets/73b2cd22-942b-4205-86e5-333df2373211/ticket.toml).
- [f1b8f01a Component-Oriented Specification System](../../../.workflow-tools/spec/specs/f1b8f01a-c7da-4a71-97c5-39519a7d7f38/spec.toml)
  and [fa6e85c8 Worktree Control Component Pilot](../../../.workflow-tools/spec/specs/fa6e85c8-866c-4d53-bb50-b78bd651e8ce/spec.toml)
  remain `draft`; their approval is planning evidence, not a lifecycle transition.

The preserved planning decisions remain authoritative: provider-owned criteria,
the spec system as the first dogfood migration target, spec-before-ticket
ordering, and best-effort criterion validation. Stage 4 changes neither code nor
ticket/spec/store state and does not authorize execution.
