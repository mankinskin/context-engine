# Artifact Inventory

| Artifact | Current state | Relevance |
| --- | --- | --- |
| [input.clean.md](input.clean.md) | Exists | Canonical refined request for the MVP. |
| [workflow-tools/agent-builder/src/main.rs](../../workflow-tools/agent-builder/src/main.rs) | Exists; one-file Rig/Copilot prompt example | Direct implementation entry point. It creates a Copilot client from environment variables, builds an agent with a fixed preamble, sends a fixed prompt, and prints the response. |
| [workflow-tools/agent-builder/Cargo.toml](../../workflow-tools/agent-builder/Cargo.toml) | Exists; standalone crate | Declares `rig` 0.42.0, `tokio`, and `anyhow`; the crate is not listed in the root workspace members. |
| [Cargo.toml](../../Cargo.toml) | Exists | Establishes the workspace boundary; `workflow-tools/agent-builder` is not a member. |
| [.github/mcp.json](../../.github/mcp.json) | Exists | Registers both `ticket-mcp` and `fs-mcp` as stdio MCP servers, proving the selected tool domains are available in the repository configuration. |
| [memory-api/crates/memory-fixtures/src/lib.rs](../../memory-api/crates/memory-fixtures/src/lib.rs) | Exists | Provides `materialize_fixture()` and copies the checked-in fixture into an isolated temporary workspace. |
| [memory-api/test-fixtures/memory-workspace-fixture/fixtures.toml](../../memory-api/test-fixtures/memory-workspace-fixture/fixtures.toml) | Exists | Documents the checked-in fixture manifest convention and includes a `.ticket` store. |
| [.ticket/tickets/0f4b3c5b-c5e9-45c4-968c-a8878f359de8/ticket.toml](../../.ticket/tickets/0f4b3c5b-c5e9-45c4-968c-a8878f359de8/ticket.toml) | Planned, high-risk agent-harness epic | Related long-term agent-harness work; this MVP must remain independent of its TUI/WASM, session-loop, and streaming scope. |
| `workflow-tools/agent-builder/tests/age_lookup_e2e.rs` | Does not exist yet; waypoint 03 output | Focused credential-gated integration test for the fixture scenario. |
| `workflow-tools/agent-builder/test-fixtures/age-lookup/` | Does not exist yet; waypoint 03 output | Isolated agent-builder fixture root containing `agent-templates/`, configuration, attached-person input, and `.ticket/` data. |

## Evidence Notes

- The existing binary is authenticated through `copilot::Client::from_env()` and documents `COPILOT_API_KEY` and `OPENAI_API_KEY` as inputs.
- No agent-builder-specific configuration parser, attachment abstraction, MCP client, template loader, or test fixture was found in the current implementation surface.
- The request's interchangeable ticket/spec-store wording is resolved for this MVP to `ticket-mcp`, because the fixture convention already contains a ticket store and the requested proof needs only one MCP-backed lookup.
- The missing agent-builder test and fixture paths above are intentional waypoint outputs, not prerequisites that must already exist before execution starts.