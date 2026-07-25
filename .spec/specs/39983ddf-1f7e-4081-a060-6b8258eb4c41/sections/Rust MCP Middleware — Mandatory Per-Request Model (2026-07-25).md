The file-based approach is replaced by a Rust middleware that makes the model identity a **mandatory per-request argument**, removing all shared mutable state.

**Middleware.** `memory-api/tools/mcp/mcp-cost-gate` (`mcp-cost-gate -- <real-server> [args]`) is a stdio JSON-RPC proxy. On `tools/list` it injects a **required** `caller_model` field into every advertised tool schema; on each `tools/call` it reads `arguments.caller_model`, rejects the call if absent, refuses token-heavy calls from orchestrator-tier models with delegation guidance, and otherwise strips `caller_model` and forwards the cleaned call. `gate.rs` ports the cost logic (output_mtok resolution, X=15 strict threshold, tool classification); `proxy.rs` holds pure, unit-tested interception functions.

**Wiring.** All 11 servers in `.vscode/mcp.json` and `.github/mcp.json` launch through `mcp-cost-gate` with `COST_GATE_TABLE` pointing at `model_prices.json`. Registered in `install-tools.sh` (`--mcp` set) and the workspace members. Fail-open when the table cannot be loaded.

**Why this supersedes the file source.** No `.active-model` file to desync or overwrite; the model declares its identity on every call, so enforcement is authoritative and per-request. Ticket 1157638e (file updater) is cancelled; the mechanism is tracked by 042109c0.

**Validation.** 14 Rust lib tests (evidence exec-vt-mcp-cost-gate-rs-20260725) plus live confirmation: the installed middleware rejected an in-session MCP call lacking caller_model and allowed it once supplied, and refused an opus token-heavy call against the real ticket-mcp binary.

**Operational note.** Once installed and wired, the middleware is active for the running client. Clients must surface the injected `caller_model` argument (from the re-fetched `tools/list`) and populate it on every call; MCP tools whose cached schema predates the wiring will be rejected until the client reloads tool definitions. Domain CLIs (ticket, spec, ...) bypass MCP and are unaffected.
