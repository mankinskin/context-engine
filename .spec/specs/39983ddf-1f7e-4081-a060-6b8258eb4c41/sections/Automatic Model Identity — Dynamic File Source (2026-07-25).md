The manual VS Code prompt is removed in favor of automatic, dynamic model identity.

**Per-call model resolution.** The proxy now resolves the active model *fresh on every `tools/call`*, preferring a file source (`CALLER_MODEL_FILE`) over the static `CALLER_MODEL` env var. Rewriting the file flips enforcement mid-session with no server restart (verified: one running proxy refuses `read_file` for opus and forwards it for gpt-5-mini purely by rewriting the file).

**No prompt.** [.vscode/mcp.json](../../.vscode/mcp.json) no longer uses an `${input:callerModel}` prompt. Every server now gets `CALLER_MODEL_FILE=${workspaceFolder}/.vscode/.active-model` (git-ignored) plus a `CALLER_MODEL=${env:CALLER_MODEL}` fallback. With no file present it is a transparent passthrough, so nothing is blocked until a model id is supplied.

**Automation hook.** Enforcement becomes automatic the moment any updater that knows the active model writes it to `.vscode/.active-model` (e.g. `echo claude-opus-4-8 > .vscode/.active-model`). That updater is the remaining piece: VS Code does not expose the active chat model to MCP requests, so a thin VS Code extension/hook (candidate: the existing ticket-vscode extension) must observe the active model and write the marker file. Tracked as follow-up.

Validated by 9 e2e tests in [test_mcp_cost_gate_proxy.py](../../tools/model-prices/test_mcp_cost_gate_proxy.py) (evidence `exec-vt-cost-gate-proxy-20260725`), including 4 dynamic file-source cases.