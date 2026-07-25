All three enforcement layers are implemented and moved to `in-review`.

**T-PARENT (guidance):** [AGENTS.md](../../AGENTS.md) "Model cost awareness & routing" now carries the enforceable `output_mtok > 15` orchestrator rule, the `model_prices.json` mapping source-of-truth, and a "Cost gate (enforcement helper)" bullet pointing at `cost_gate.py` and the orchestrator agent.

**T-WRAP (tool wrapper):** [tools/model-prices/cost_gate.py](../../tools/model-prices/cost_gate.py) is the transport-agnostic decision core. It resolves `output_mtok` from the shared table (case-insensitive match, conservative max on ambiguity, conservative delegate for unknown models), classifies tools (`token_heavy` / `always_allowed` / `light`), and returns `allow` or `delegate` with delegation guidance. CLI: `--model <id> --tool <name>` with exit codes 0=allow, 3=delegate, 2=error. This is the shared core for both BLOCK-3 transports:
- **Enforcement (option C):** the client/extension layer — which knows the active model and cannot be spoofed — shells out to (or imports) the gate before a tool call reaches any MCP server. This is the real enforcement surface; MCP stdio servers have no caller-model channel.
- **Portable fallback (option A):** an explicit `caller_model` parameter for non-VS-Code transports calls the same `evaluate()`.
Validated by [tools/model-prices/test_cost_gate.py](../../tools/model-prices/test_cost_gate.py) (18 tests passing; evidence `exec-vt-cost-gate-20260725`).

**T-ORCH (constrained agent):** [.agents/agents/orchestrator.agent.md](../../.agents/agents/orchestrator.agent.md) is a distinct agent template restricted to `tools: [agent]` (exactly the sub-agent primitive) — no file/search/execute/MCP access. It plans, delegates each unit to a cheaper sub-agent via `runSubagent(model=<cheaper>)`, and aggregates results.

## Resolved BLOCK Decisions

- **BLOCK-1/2:** field `output_mtok`, `X = 15` USD/1M (strict `>`).
- **BLOCK-3:** option C (client/extension enforcement) as the real surface, option A (explicit `caller_model` param) as portable fallback; both call the shared `cost_gate.py`.
- **BLOCK-4:** distinct `.agent.md` orchestrator template exposing only the sub-agent tool.

## Remaining Integration (follow-up)

The gate logic and constrained agent exist and are tested; wiring the client/extension layer to invoke the gate on every tool call (option C) is a runtime-integration follow-up, since VS Code's MCP client does not expose the active model to servers.