## Transition Ownership

**The Iteration Agent owns ALL ticket state transitions.** Sub-agents report verdicts and findings only.

The Review Agent is **strictly verdict-only**: it must never call `close_ticket`, never pass `to_state` to `update_ticket`, and never move a spec to `reviewed`. This applies ALWAYS, not just when the Review Agent is invoked by the Iteration Agent.

When the Review Agent reports a verdict, the Iteration Agent applies the resulting ticket transition:
- Review pass → advance toward `done` (after Commit)
- Review fail → return to `in-implementation`

This rule prevents conflicting state changes and ensures the Iteration Agent maintains a consistent view of the transition lifecycle.