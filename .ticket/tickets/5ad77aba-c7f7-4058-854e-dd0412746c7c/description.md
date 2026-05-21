# Problem

The ticket/spec/rule tool surfaces are not self-describing enough for operators or agents.

This session spent a disproportionate amount of effort on capability discovery instead of task execution:

- 17 `tool_search` calls
- 14 MCP ticket executions
- 0 direct MCP spec executions
- 0 direct rule-tool executions

The repeated search phrases were about basic discovery (`ticket information access`, `activate ticket information access bundle`, `spec tools`, `board tools`), not advanced functionality. That is a sign that the surfaces do not advertise the common workflows clearly enough.

# Session Evidence

- The session repeatedly searched for ticket information access and board/state tools before falling back to CLI.
- The session never used direct MCP spec tools despite heavy spec work.
- Rule tooling was effectively absent from real task execution even though rule-related tickets were in scope.

# Scope

1. Add a self-describing capability catalog / help surface for ticket/spec/rule workflows.
2. Cover, at minimum:
   - common read flows
   - mutation flows
   - board / next / why-not flows
   - validation flows
   - nested-root / nested-store targeting support
3. Expose a machine-readable form for MCP/agent consumers and a human-readable form for CLI/operators.
4. Ensure the catalog points to the canonical command/tool for a workflow instead of requiring semantic-search roulette.
5. Document known parity gaps explicitly so agents do not waste time discovering that a needed surface does not exist yet.

# Regression Validation Requirements

- **Specification / docs:** define the capability-catalog contract and the minimum workflow categories it must describe.
- **MCP / CLI:** add tests showing the help/catalog surface lists the same core workflows and targeting semantics.
- **Rule discoverability:** include at least one rule-oriented workflow in the catalog so rule tooling is not invisible in normal usage.
- **Operator validation:** replay the capability-discovery portion of this session and confirm the common ticket/spec/rule workflows can be found without repeated exploratory tool_search loops.

# Acceptance Criteria

- One command/tool can list the canonical ticket/spec/rule workflows and the parameters they require.
- The catalog explicitly states whether a workflow supports nested roots/stores.
- MCP and CLI help surfaces agree on the named workflows and targeting semantics.
- Rule-oriented workflows are discoverable from the same catalog rather than relying on ambient docs only.
- The documented parity gaps are explicit enough that agents can choose the right fallback immediately.

# Likely Surfaces

- `tools/ticket-cli/`
- `tools/ticket-mcp/`
- `tools/spec-cli/`
- `tools/spec-mcp/`
- `crates/rule-api/`
- `README.md`
- `.agents/instructions/`
