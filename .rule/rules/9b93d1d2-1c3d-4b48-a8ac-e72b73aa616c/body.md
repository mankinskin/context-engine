### Picking Next Work

Use `ticket next` to find the highest-priority unblocked tickets:

- Use `ticket next` for the global queue of unblocked work.
- Use `ticket next <ticket-id>` when a larger ticket is blocked and you need the immediate leaf blockers that can start now.
- `ticket next <ticket-id>` also returns a blocker tree so intermediate blocked dependencies stay visible while you execute the frontier leaves.
- Prefer this root-scoped form when unblocking tracker or epic tickets so agents pick work that directly reduces the root blocker set.