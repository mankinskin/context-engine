## Problem

The `workspace` parameter is overloaded across the memory-api MCP servers. Read tools accept the slug `"default"`; write tools require an explicit absolute store path. Nothing in the tool schema says so, and the failure is opaque enough that agents abandon MCP entirely.

Observed in `3e9bc20b`:

- `mcp_test-mcp_test_record_spec` failed twice with `{"workspace": "default", "id": "val-session-api-lib-suite", ...}`
- `mcp_test-mcp_test_record_execution` failed twice
- `mcp_spec-mcp_spec_create` failed once; the agent's own recovery reasoning was *"I need to provide an explicit workspace path, not just `default`. Let me use the actual workspace root path."*

Consequence: subagent `[11] Materialize spec and validation files` ran **72 terminal commands in 42 turns**, including `cargo build -p spec-cli --release` and repeated `cargo run --release -p test-cli -- record --workspace . --id exec-val-...`, rebuilding CLIs from source to do what the failed MCP calls would have done in five calls.

This reproduced live during the analysis session: `create_ticket` with `workspace: "default"` returned

```
MPC -32602: invalid workspace selector 'default': entity creation requires an
explicit workspace path; do not use omitted, empty, 'default', '.', or '..'
```

That message is good — it names the constraint and the rejected values. `spec-mcp` and `test-mcp` did not produce anything comparable, which is why the agent guessed instead of correcting.

## Secondary finding: budget-gate error is actionable, subgraph is not reachable

`mcp_ticket-mcp_subgraph` returned:

```
Tool 'subgraph' requires cost 75 but model 'claude-opus-4-5' has effective
budget 58 ... Delegate via runSubagent(model=<cheaper>, ...).
```

Correct behaviour, but it forces a full sub-agent spawn (~110k tokens minimum) for a read that the CLI performs for free. Worth reviewing whether read-only graph traversal should cost 75.

## Scope

- Audit `workspace` parameter semantics across `spec-mcp`, `test-mcp`, `rule-mcp`, `feedback-mcp`, `session-mcp`, `ticket-mcp`. Document per tool whether a slug is accepted.
- Make every rejection message match the `ticket-mcp` shape: name the constraint, list the rejected forms, state the accepted form.
- Encode the constraint in the tool schema description so it is visible before the call, not only after failure.
- Prefer unifying the semantics: if write tools can resolve `"default"` to the owning store, do that instead of rejecting.
- Review read-only tool costs in the price-awareness gate — specifically `subgraph` at 75 against a 58 budget for Opus.

## Acceptance Criteria

1. Every MCP tool taking `workspace` documents in its schema whether a slug is accepted.
2. Rejections name the constraint and the accepted form; no rejection leaves the caller guessing.
3. Either write tools accept `"default"`, or the restriction is stated in the parameter description of every affected tool.
4. A test asserts the error shape for at least `spec_create`, `test_record_spec`, and `test_record_execution`.
5. Read-only traversal tools are reachable from an Opus-tier caller, or the cost is documented as intentional with the cheaper alternative named in the error.

## Evidence

- `.session/sessions/3e9bc20b-4fe8-4996-ae7f-7be32525e429/events.json` — failures at events 595, 596, 622, 624, 752 with recovery reasoning
- Remediation cost: subagent `[11]`, 42 turns / 72 terminal commands
- Live reproduction during analysis of epic `79c4ac3e`