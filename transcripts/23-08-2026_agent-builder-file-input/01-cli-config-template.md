# 01: CLI Configuration and Template Assembly

## Outcome

The `agent-builder` binary accepts a single request, an attached-file path, and configuration selecting an agent-template directory or template path; it loads the selected template, incorporates the request and attachment context into the model prompt, and prints the model response. For the age-lookup scenario, the completed template requires exactly `{"age": <integer>}` with no Markdown fence or other wrapper.

## Evidence

- [workflow-tools/agent-builder/src/main.rs](../../workflow-tools/agent-builder/src/main.rs) is the current fixed-prompt CLI entry point.
- [workflow-tools/agent-builder/Cargo.toml](../../workflow-tools/agent-builder/Cargo.toml) provides the existing Rig and Tokio dependencies.

## Non-goal

Do not add session persistence, interactive chat state, a UI, template discovery beyond the configured selection, or broad agent orchestration.

## Validation Method

Run `cargo test --manifest-path workflow-tools/agent-builder/Cargo.toml` for parsing and prompt-assembly coverage, then run the binary with a fixture configuration and explicit file path.