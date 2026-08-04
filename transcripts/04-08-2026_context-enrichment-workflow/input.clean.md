# Context Enrichment Workflow for Review Tickets

We are in a situation where we have done a lot of work and now have many tickets in review, but on very different topics. We need to work toward closing all of these tickets individually, so that each one can be finished, closed, and fully satisfy its acceptance criteria.

The difficult part is collecting the full context for each ticket and relating all context artifacts to one another. The good news is that all of our previous sessions are recorded. That gives us an entry point, because in the best case we can find the full ticket lifecycle, or at least the last steps taken on a ticket.

My goal is to automate this as much as possible and find a workflow we can reuse in this kind of situation. We have one or more tickets in review and no other known context, and we want to use all of our tools - Session API, Spec API, Ticket API, Feedback API, Test API, Rule API, and so on - to find the context for each ticket we want to close or advance.

## Proposed approach

- Use the Session API as the entry point for the workflow.
- Define a contract between the Session API and the Ticket API so the Session API can query which sessions worked on a given ticket.
- Optionally add the inverse query to the Ticket API later, so a ticket can ask which sessions worked on it.
- For now, focus on listing all sessions in a complete format and filtering them by the tickets they worked on or implemented.
- Possibly add selectors such as "worked on" or "people" so we can tune how strong the relation must be before a ticket is included in the list.
- Treat this as the first step before broader context enrichment across all memory API artifacts.

## Current objective

- Close all tickets currently in review by fully implementing and validating them, then moving them to accepted or done.
- If we decide not to implement a ticket, move it back to Open or Ready.
- Build a context enrichment agent, or full review agent, know-how agent, or completion agent, whose only task is to enrich context, review tickets, and close them by moving them from an active state to an inactive or completed state.
- In the long term, we want only one active ticket at a time, not ten unrelated tickets.
- That would make branch changes clearly attributable to a ticket track and the sessions working on it, with the full context isolated to a branch.
- Even so, it can happen that different tickets were worked on in the same branch, so we need to map tickets, sessions, specs, and changes back to one another.

## For this session

- Plan the next steps.
- Try the tool once as a dogfood cycle.
- Improve it so it can enrich context for tickets in review, or at least collect the currently available information about those tickets.
- This may require changes to the Session API and the Ticket API, relations to other workspace or domain stores, transport surfaces for new commands, and instruction agents that can consume the workflow.

