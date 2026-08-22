# Workflow Cycle: From User Request to Implementation

We use this workflow frequently, though not always: we first create tickets, follow those tickets, and implement the work steps described in them. We also have other things such as acceptance criteria. In practice, this is all just one of many ways of working, and we want to define it more clearly.

There are also other tools or means we can use to fulfill a request or achieve a specific goal:

- **Specification**: a detailed description of what we want, including when the goal is reached. In other words, the goal described in its ideal form.
- **Tickets**: the work we write for how to get from the current state to the goal.
- **Files and free-text prompts**: single files that contain the task, or the user’s free-text request itself.
- **Code**: the current implementation and source state.

We should think about all of these as structured inputs to the process.

The central pieces of this space are:

- the specification of the goal;
- the existing source code, meaning the current state, which can be explored through tests or documentation; and
- the plans for how to move from that starting state in the code, made readable through documentation, toward the goal.

This forms a closed loop. The solution we are looking for must close that loop. So we build the closed loop in from the beginning and then improve it step by step.

The goal is to take the user’s free text and turn it into an improved version of that free text. At first, we do not create tickets or specifications yet. Once we have everything we need as simply as possible in a folder with a few text files, then we can decide how large the task is and what exactly we should do.

For the simplest case, we would implement the task directly. In that case, the free-text request or the material we have so far is enough, and we can just work through a simple list. That is all possible without using the ticket system, and without needing to specify an architecture.

There is also a slightly larger step where we use only tickets and arrange tasks like a to-do list. In that case, before implementation we first model the work steps needed to reach a specific goal or specification. But it is not a requirement that a specification already exists. A ticket can exist without referring to a specific specification.

We should still work toward a state where most things are specified, because that is where tickets usually belong. But many tasks are simply not appropriate for that, and we do not want to prevent people from using the ticket system. So we should not expect everything to be tied together in a complicated way through tickets and specs.

The third level would be a real architectural decision: we create the specifications first and derive tickets from them. As a rule, we should avoid tickets that say something still needs to be specified. If something is missing from the specification, it should be mentioned in the specification itself. Then a ticket can say that it is waiting for that specification, or for the implementation of a specification.

That makes more sense: tickets should depend on specifications, and tickets should be able to close, fulfill, or implement specifications, instead of the current model where one ticket depends on another ticket and the first ticket must be closed to unblock the next one. In other words, the first ticket implements the specification, or the second ticket requires or needs it.

At that point it becomes clear that the specification is primarily a definition of the world in which our process takes place, and it is effectively a contract that we must be able to trust in order to do uncertain work.

That is also where the loop closes: the specification arises from the user’s request and from interaction with the user. We build the specification from the user’s definition.

The first step for tickets is then to develop, write, and define tests for it. The specification is effectively covered by tests, and the acceptance criteria are checked in a sandbox system or a unit test suite. We then see that a test exists that validates the specification or the acceptance criteria in it.

Those tests are then used to validate the implementation in later tickets, which implement the individual solutions, and to ensure that the specification is actually fulfilled. They also signal to the user that the request has now been processed.

The user can then decide whether all rules were followed, whether the system worked efficiently, and whether all the requirements they had in mind were met. From there, they can start another iteration through the cycle until they are effectively at the goal.

Along the way, we also have a lot of text from individual sessions that carry out these tasks. From that, we can extract a lot of information about how we can work more efficiently, translate more tasks locally into code, or explain processes better and make them more accessible.

We also need to ask how we can work more closely with the user so that they can provide complete feedback, including on the visual component. It would be great if we could anchor this model even more deeply and make it a core part of the architecture.

I also want us to include this in our presentation as the outline of our complete cycle. All the components I mentioned already have initial implementations, and we want to implement this loop.