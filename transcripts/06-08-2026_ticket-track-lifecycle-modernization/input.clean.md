# Ticket Track Lifecycle

This transcript is about improving the ticket track and the schema modernization so that every ticket follows a universal lifecycle.

The core idea is that every ticket, regardless of schema, should move through three phases:

1. Planning
2. Action
3. Verification

Each schema can define its own internal states for those phases. That could mean a planning loop, an implementation loop, or a review loop. The details live in the more specialized schemas.

## Practical flow for a new request

When a new feature request or task arrives, typically as a transcript or chat message, the process should start by creating an epic ticket that tracks the full implementation path.

That epic starts in planning. In that phase, the system may run several smaller steps to improve the epic and prepare it for execution. A possible first state is research:

- look up other sources
- understand the current state
- find related tickets
- enrich the context

This planning phase can also create additional tickets or update existing ones with new information. For example, it may create:

- a research ticket
- an interview ticket

Those child tickets can depend on the epic, and the dependency structure should ensure that there is only one unblocked leaf at a time, for example the research ticket.

## Research ticket

The research ticket also goes through plan, act, and verify.

It should:

- plan the research
- search, investigate, and collect information
- verify whether everything was found
- loop back if there are still gaps

The goal is to complete the research before moving on to the next dependent ticket.

## Interview ticket

Once research is available, the interview ticket can use that information in its own planning phase.

It should:

- design the interview questions
- collect the questions
- verify that the questions make sense
- run the interview with the user
- collect the answers
- review whether the answers are complete
- loop back to planning if new questions are needed

The interview continues until there are enough answers, and then it can be closed.

## Back to the epic

After the research and interview work is finished, the epic ticket continues in planning.

At that point, it can:

- review whether more planning is needed
- create a planning ticket if future tickets should be defined
- break the implementation into concrete child tickets
- refine and review those tickets until planning is complete

When planning is done, the epic can move into a schema-specific ready state. It is still in the planning phase, but the next step is now implementation.

## Implementation and final review

From there, the epic advances into implementation and the planned tickets are executed.

Those tickets may again trigger smaller planning, action, and verification steps when something needs to be clarified or researched more precisely.

Everything should keep flowing through these smaller loops until the work is finished.

At the end, the whole ticket reaches review. The final review checks the completed work and may send it back into the loop if something still needs to be fixed.

## Main requirement

The important thing is that the system must make this flow possible:

- the step-by-step ticket progression
- the dependency structure between tickets
- the information flow from research to interview to planning to implementation to review
- the ability to loop within each phase until it is complete

