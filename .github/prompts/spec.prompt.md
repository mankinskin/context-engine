---
description: "Create a new draft spec entry from the slash-command text. Prefer spec-mcp tools and fall back to spec.exe when needed."
name: "spec"
argument-hint: "<your content>"
agent: "agent"
---

# Create Draft Spec Entry

Create a new draft spec entry from the user's current slash-command request.

Reference [Spec Tool](../../agents/skills/SPEC_TOOL.md) for slug rules, store layout, and transport details.

Workflow:
1. Treat the text typed after `/spec` as the source request.
2. Search existing specs first to avoid duplicates and to identify the best component and parent.
3. Prefer `spec-mcp` tools such as `spec_search`, `spec_list`, `spec_tree`, and `spec_create` when they are available.
4. If `spec-mcp` is unavailable, fall back to `./target/debug/spec.exe` and register `.spec/specs` with `spec.exe add-root .spec/specs --label default --json` if needed.
5. Infer a clear title, slug, component, and parent. Keep slugs lowercase, use `-` within segments, and `/` between segments.
6. Create the spec in `draft` state with a useful initial body covering motivation, intended behavior or scope, constraints or non-goals when obvious, and initial acceptance criteria.
7. If a matching spec already exists, do not create a duplicate. Return the existing spec instead.
8. If required details are still ambiguous after a focused search, ask one concise clarification rather than guessing.
9. Do not implement code or change unrelated files unless the user explicitly asks.

Response:
- created or matched spec slug and id
- chosen component and parent
- key assumptions
- duplicate candidates considered, if any
