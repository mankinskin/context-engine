# Ticket Schema and Workflow State Machines

We want to improve the ticket schema so the system can model workflow tasks as clearer state machines.

The current task types are too coarse and too similar: bug ticket, generic task, old ticket-improvement schema, and epic. They are not enough and often feel misleading.

We want to clean this up and define which workflow steps the ticket system should represent.

- Features do not belong in the ticket schema; they should be tracked by a spec.
- An epic is a tracker for a longer-running task, such as implementing a spec.
- Under epics, we need clearer definitions of the different tasks.

The center of the model is likely code changes: implementation steps that modify code, must be tested, and must be reviewed before the work can progress.

There are also explicit steps before that happens:

- planning
- research
- review
- interview
- testing

Those should also be representable as separate tasks.

We should treat tickets as progress steps with a start state and an end state. The ticket system should describe these transitions as different state machines. All work is a time step, but there are different forms of time steps:

- a large epic step with many substeps
- a small step such as making a code change or interviewing a user

This should be modeled as a hierarchical inheritance structure. State machines should be able to inherit from one another, so bug tickets and code changes can reuse the same workflow steps by specializing abstract states differently.

A likely shared structure for bug tickets, code changes, feature implementations, and user stories is:

- research
- plan the change
- implement the change
- verify or validate it
- test acceptance and confirm the goal was reached

Even with this shared structure, bug tickets and feature requests should still be distinct:

- a bug ticket starts from an error or a found problem
- a feature request or user story describes new or changed functionality without a specific problem

We can also model ticket planning itself as a task, with its own research, implementation, and validation phases if needed. Time step types can define their own validation steps, and privileged tasks can have different elevation levels above full review with full test verification, user acceptance, and adversarial agentic review.

We want to define all of these schemas now and encode the hierarchical inheritance structure and type logic in the code. For larger code changes, once the schemas are improved, we should open new tracking tickets for them.

One final requirement: the schemas should also be parsable as JSON. That should be a small change, and we want to add it as a ticket as well. YAML may be supported later, but JSON comes first.

Final migration note: we should take stock of the existing tickets and their types first. For now, keep the schemas that are used heavily, and migrate away only the types that do not have many tickets. After that, we can add the new schema first and migrate the larger old types to the new types in a later phase.
