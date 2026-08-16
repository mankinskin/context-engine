# Objective

Provide a secure observer-terminal capability in which a human enters terminal
commands while agents can observe bounded output, working directory, exit
status, and session state. Agents must never have an API that sends terminal
input or executes an arbitrary command in that observer session.

# Requirements

- A human owns all command input to an observer-terminal session.
- Agents can create or attach to a named session, read bounded output, and
query status without an input-sending operation.
- Output carries a session identity and can be used as evidence by guidance
agents.
- Sessions terminate explicitly or by a safe timeout, and stale output is not
presented as current evidence.
- The implementation decision follows the capability inventory in ticket
0dd23fe6-6892-4d21-9927-4a81584dc77a.

# Acceptance Criteria

1. The selected UI and transport expose human-input ownership and agent
observation as separate capabilities.
2. An integration test demonstrates human-side input, agent-side bounded output
readback, and absence of any agent input operation.
3. The implementation documents working-directory and timeout semantics.
4. The ticket links the interactive-learning guidance spec and records focused
validation evidence.

# Non-Goals

- Replacing the existing compact single-shot terminal tool.
- Granting an Explainer or Teacher Agent command execution.
- Persisting sensitive terminal input.
