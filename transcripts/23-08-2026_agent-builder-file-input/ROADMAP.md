# Agent Builder MVP Roadmap

## Outcome Summary

This roadmap delivers a narrow, locally executed agent-builder request path that proves the current Rig/Copilot client can combine a configured agent template, an attached file, and one ticket-backed MCP lookup. Its purpose is to establish an executable vertical slice before session architecture, UI work, or generalized multi-tool orchestration begins. The result is validated against an isolated fixture and a JSON-only answer contract.

## Relevant Artifacts

- [input.clean.md](input.clean.md)
- [ARTIFACTS.md](ARTIFACTS.md)
- [workflow-tools/agent-builder/src/main.rs](../../workflow-tools/agent-builder/src/main.rs)
- [workflow-tools/agent-builder/Cargo.toml](../../workflow-tools/agent-builder/Cargo.toml)
- [.github/mcp.json](../../.github/mcp.json)
- [memory-api/crates/memory-fixtures/src/lib.rs](../../memory-api/crates/memory-fixtures/src/lib.rs)
- [memory-api/test-fixtures/memory-workspace-fixture/fixtures.toml](../../memory-api/test-fixtures/memory-workspace-fixture/fixtures.toml)
- [.ticket/tickets/0f4b3c5b-c5e9-45c4-968c-a8878f359de8/ticket.toml](../../.ticket/tickets/0f4b3c5b-c5e9-45c4-968c-a8878f359de8/ticket.toml)
- `workflow-tools/agent-builder/test-fixtures/age-lookup/` (created by waypoint 03)
- `workflow-tools/agent-builder/tests/age_lookup_e2e.rs` (created by waypoint 03)

## Active Blockers

None. The live validation gate requires Copilot credentials at execution time; this is an environment precondition, not an unresolved product decision.

## Validation Gates

- `cargo test --manifest-path workflow-tools/agent-builder/Cargo.toml`
- `cargo test --manifest-path workflow-tools/agent-builder/Cargo.toml --test age_lookup_e2e -- --ignored` with both `COPILOT_API_KEY` and `OPENAI_API_KEY` set; the test fails fast when either is absent.
- The live response must be exactly `{"age": <fixture-age>}` with no wrapper or extra key.

## Roadmap Waypoints

1. **Single-session: CLI configuration and template assembly.** Implement the configuration, template-path selection, attached-file argument, and one-request prompt construction described in [01-cli-config-template.md](01-cli-config-template.md). Acceptance check: offline tests pass.
   Non-goal: sessions, UI, or general orchestration.
2. **Single-session: File and ticket-MCP tool path.** Implement the template-authorized file read and the one `ticket-mcp` lookup described in [02-file-and-ticket-mcp.md](02-file-and-ticket-mcp.md). Acceptance check: offline tool/config coverage passes and the capabilities are available to the request flow.
   Non-goal: spec-store support, mutation tools, or arbitrary MCP servers.
3. **Single-session: Fixture-backed live validation.** Create `workflow-tools/agent-builder/test-fixtures/age-lookup/` and `workflow-tools/agent-builder/tests/age_lookup_e2e.rs`, then add and run the executable fixture scenario described in [03-live-fixture-validation.md](03-live-fixture-validation.md). Acceptance check: with `COPILOT_API_KEY` and `OPENAI_API_KEY` set, `cargo test --manifest-path workflow-tools/agent-builder/Cargo.toml --test age_lookup_e2e -- --ignored` returns exactly `{"age": <fixture-age>}`.
   Non-goal: broad provider benchmarking or credential storage.

## Heads-up Notes

- `workflow-tools/agent-builder` is a standalone Cargo crate, not a root workspace member; use its manifest path for Cargo commands.
- The current code has no configuration or test surface, so the first waypoint must establish those boundaries before integrating tools.
- Keep the live provider assertion narrow and schema-based; model prose formatting is intentionally constrained by the template rather than assumed by the test.
- The broader agent-harness ticket is planned and high-risk; this MVP must not absorb its TUI, WASM, event-streaming, isolation, or long-running-loop goals.