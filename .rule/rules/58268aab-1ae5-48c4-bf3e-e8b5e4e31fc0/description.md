### Review Gate Before Closing

**Never `close` a ticket directly from `in-implementation`.** Always move
through `in-review` first, even for small changes.
The schema's `required_states` enforcement prevents skipping `in-review`,
but you should still follow the full progression diligently.
Review readiness means the implementation, required validation, documentation updates, and spec traceability are current before the state change.