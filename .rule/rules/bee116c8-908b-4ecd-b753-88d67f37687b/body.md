> **IMPORTANT — state machine is one-way.** Transitions only go forward.
> Progress through **every** state in order — do not skip states. The schema
> defines `required_states` (e.g. `["in-review"]`) that **must** appear in a
> ticket's history before it can reach a terminal state (`done`). Attempting to
> close a ticket without visiting all required states will be rejected by the
> store.
>
> If a state was reached prematurely, use `update --undo` to revert the last
> transition and re-progress correctly.