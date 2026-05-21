# Problem

Invalid ticket state transitions are enforced, but they are not explained well enough at the moment they fail.

In this session, `new -> in-implementation` was rejected, and the only practical recovery was to inspect the schema and CLI help, then manually infer the required progression through intermediate states.

That is technically correct behavior but poor operator UX.

# Session Evidence

- The session attempted to move a new ticket directly into `in-implementation` and hit the state-machine guard.
- The operator then had to inspect schema/help and route the ticket through `in-refinement` and `ready` before implementation could begin.
- The failure path did not directly surface the allowed next states or the required intermediate sequence.

# Scope

1. Extend transition failures so they return machine-readable guidance:
   - current state
   - allowed next states
   - required intermediate states, when applicable
   - ticket type / schema name
2. Teach the CLI to print a clear recovery message with the legal next transitions and example retry commands.
3. Expose the same guidance through MCP and any HTTP-facing mutation surfaces.
4. Add a lightweight command or sub-view to inspect legal transitions for a given ticket or ticket type.
5. Update workflow docs so the user sees the same progression described in the tools and in the generated guidance.

# Regression Validation Requirements

- **Specification / docs:** define the invalid-transition error contract and the required recovery fields.
- **CLI:** add integration coverage for one blocked transition like `new -> in-implementation` and assert that the error lists the legal next steps.
- **MCP:** add parity coverage for the same transition error structure.
- **Schema-aware validation:** include at least one ticket type with a nontrivial path so the guidance is not hard-coded to a single state machine.
- **Manual validation:** reproduce the exact blocked transition from this session and confirm the first error message is sufficient to recover without separate help spelunking.

# Acceptance Criteria

- A blocked transition reports the current state and allowed next states.
- When intermediate states are mandatory, the tool explicitly names them.
- CLI, MCP, and any related mutation surfaces use the same recovery fields.
- One command or view can show the legal transition graph for the current ticket or ticket type.
- Workflow docs and generated guidance reflect the same state progression that the tools enforce.

# Likely Surfaces

- `crates/ticket-api/`
- `tools/ticket-cli/`
- `tools/ticket-mcp/`
- `.agents/instructions/ticket-system.instructions.md`
- `memory-viewers/memory-api/.spec/`
