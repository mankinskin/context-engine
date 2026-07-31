## Problem
`gate.rs` cost logic is hardwired into the proxy path in `proxy.rs`. To make mcp-toolmon a general-purpose proxy (reload/lifecycle features are policy-agnostic), the cost gate must become one pluggable implementation, not the only possible behavior. Part of epic 25780944; depends on T1 (post-rename paths). This ticket also owns the tokio async-runtime port for the proxy core so the reload subsystem can be built on async cancellation, timers, and select-based supervision instead of raw thread pumps.

## Approach
Define a `Policy` trait (e.g. `fn on_tools_list`, `fn on_tool_call_request`, `fn on_tool_call_response`) in the proxy core. Move existing `gate.rs` cost logic behind a `CostGatePolicy` implementation of that trait. Port the transport/proxy core onto tokio (`tokio::process::Command`, async stdio pumps, async timers, and cancellation-safe supervision) while keeping behavior neutral. `proxy.rs` calls into `Box<dyn Policy>` / generic policy instead of calling gate functions directly. Behavior-neutral: identical JSON-RPC behavior, identical `costGateWarning` injection, identical env var handling (`COST_GATE_*` unchanged).

## Acceptance criteria
- [ ] `Policy` trait defined with hook points covering current gate interception points (tools/list schema injection, tools/call request/response handling)
- [ ] `CostGatePolicy` struct implements `Policy`, wrapping existing `gate.rs` logic with no behavior change
- [ ] `proxy.rs` invokes policy through the trait, not direct gate function calls, and the transport/proxy core runs on tokio async primitives instead of raw thread pumps
- [ ] All existing unit tests in `gate.rs` and `proxy.rs` pass unchanged
- [ ] `tests/integration_gate.rs` passes unchanged (rename allowed, assertions untouched)
- [ ] No new env vars or CLI flags introduced by this ticket (that's T3/T6)
- [ ] `costGateWarning` injection point and JSON shape unchanged

## Files touched
- memory-api/tools/mcp/mcp-toolmon/src/proxy.rs
- memory-api/tools/mcp/mcp-toolmon/src/gate.rs
- memory-api/tools/mcp/mcp-toolmon/src/lib.rs
- memory-api/tools/mcp/mcp-toolmon/src/policy.rs (new)
- memory-api/tools/mcp/mcp-toolmon/tests/integration_gate.rs