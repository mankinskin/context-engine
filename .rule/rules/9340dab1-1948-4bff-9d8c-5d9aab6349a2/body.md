### Schema-Enforced Workflow (`required_states`)

The ticket type schema can declare `required_states` — a list of states that
**must** appear in a ticket's history before the store allows a transition to
a terminal state (default terminal: `done`).