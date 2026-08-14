## Gap

No shared instruction currently makes a dispatched sub-agent terminate with a usable deliverable. Create the proposed `.agents/instructions/orchestration/subagent-return-contract.instructions.md` and reference the shared rule from agent templates instead of copying the rule into every template.

## Session Evidence

Three separate read-only dispatches in one restructuring session ended their only message with a question rather than a deliverable. Each consumed a full sub-agent spawn without returning the requested report. An earlier Research dispatch showed the same failure.

## Required Corrected State

The new instruction requires exactly one terminal message, forbids questions and offers of options, requires `BLOCKER: <reason>` for any impossible requested item, and requires delivery of every remaining requested section despite a blocker.