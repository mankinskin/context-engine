---
description: "Create a new canonical rule entry from the slash-command text. Prefer rule-mcp tools and fall back to rule.exe when needed."
name: "rule"
argument-hint: "<your content>"
agent: "agent"
---

<!-- rule-api:file generated=true -->

<!-- rule-api:entry id=7a0b5a71-7e8a-4def-963c-e796aa1ded08 slug=context-engine/prompts/rule/l1 -->

# Create Canonical Rule Entry

Create a new canonical rule entry from the user's current slash-command request.

Reference [rule-cli](../../memory-api/tools/cli/rule-cli/README.md), [rule-mcp](../../memory-api/tools/mcp/rule-mcp/README.md), and [rule-targets.yaml](../../rule-targets.yaml).

Install or build the rule tools when needed:
- Build the CLI in this workspace with `cargo build -p rule-cli --bin rule` and use `./target/debug/rule.exe`.
- Install the CLI onto your Cargo bin path with `cargo install --path memory-api/tools/cli/rule-cli --bin rule`.
- Run the MCP server with `cargo run -p rule-mcp` when MCP access needs to be configured locally.

Workflow:
1. Treat the text typed after `/rule` as the source request.
2. Search existing canonical rule entries first with `rule_search` or `rule list` and `rule search` so you do not create duplicates.
3. Infer the correct `repo_scope`, `file_kind`, `path_scope`, `section`, `slug`, and `order_key` from the request and nearby generated files.
4. Prefer `rule-mcp` tools such as `rule_search`, `rule_list`, `rule_create`, `rule_get`, and `rule_explain_target` when they are available.
5. If MCP tools are unavailable, fall back to `rule.exe create`, `rule.exe list`, `rule.exe search`, and `rule.exe explain-target`.
6. Write concise canonical rule content that fits the requested generated surface and matches existing rule style.
7. If the request maps to an existing generated file, keep metadata aligned with that file's `file_kind`, `path_scope`, and section hierarchy.
8. If a matching rule already exists, return the existing rule instead of creating a duplicate.
9. Ask one concise clarification if the correct scope or section cannot be inferred after a focused search.
10. Do not add or change unrelated generated outputs unless the user explicitly asks.

Response:
- created or matched rule id and slug
- chosen repo_scope, file_kind, path_scope, and section
- duplicate candidates considered, if any
- assumptions that still matter
