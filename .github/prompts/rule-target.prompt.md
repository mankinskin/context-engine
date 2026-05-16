---
description: "Create a new rule target from the slash-command text, add any missing canonical rule entries, and generate the output with rule-api."
name: "rule-target"
argument-hint: "<your content>"
agent: "agent"
---

# Create Rule Target

Create a new rule target from the user's current slash-command request and use the rule-api flow to generate its output.

Reference [rule-cli](../../memory-viewers/memory-api/tools/cli/rule-cli/README.md), [rule-mcp](../../memory-viewers/memory-api/tools/mcp/rule-mcp/README.md), and [rule-targets.yaml](../../rule-targets.yaml).

Install or build the rule tools when needed:
- Build the CLI in this workspace with `cargo build -p rule-cli --bin rule` and use `./target/debug/rule.exe`.
- Install the CLI onto your Cargo bin path with `cargo install --path memory-viewers/memory-api/tools/cli/rule-cli --bin rule`.
- Run the MCP server with `cargo run -p rule-mcp` when MCP access needs to be configured locally.

Workflow:
1. Treat the text typed after `/rule-target` as the source request.
2. Inspect existing entries in `rule-targets.yaml` and nearby generated files before adding a new target.
3. Prefer `rule-mcp` tools such as `rule_search`, `rule_list`, `rule_create`, `rule_explain_target`, and `rule_generate_target` when they are available.
4. If MCP tools are unavailable, fall back to `rule.exe` commands.
5. Infer a target name, `repo_scope`, `file_kind`, `path_scope`, `output_path`, and the node tree needed to render the requested file.
6. Update `rule-targets.yaml` with the smallest target definition that matches the request.
7. Create any missing canonical rule entries with metadata consistent with the target's `file_kind`, `path_scope`, and section hierarchy.
8. Generate the target output with `rule_generate_target` or `rule.exe generate-target` and verify it with `--check`.
9. Reuse existing targets and rule entries when they already satisfy the request instead of duplicating them.
10. Ask one concise clarification if the output path, target structure, or file kind is still ambiguous after a focused search.
11. Do not leave the target half-configured: finish with a generated file or a clearly stated blocker.

Response:
- target name and output path
- rule entries created or matched
- generation and check commands used
- assumptions or follow-up gaps
