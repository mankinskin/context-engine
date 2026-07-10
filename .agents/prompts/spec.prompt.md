---
description: "Create or update a draft spec entry from the slash-command text. Prefer spec-mcp tools and fall back to spec.exe when needed."
name: "spec"
argument-hint: "<your content>"
agent: "agent"
---

<!-- rule-api:file generated=true -->

<!-- rule-api:entry id=0719f0c1-9036-4983-912c-599de3a37d23 slug=context-engine/prompts/spec/l1 -->

# Create or Update Draft Spec Entry

Create or update a draft spec entry from the user's current slash-command request.

Reference [Spec Tool](../../agents/skills/SPEC_TOOL.md) for slug rules, store layout, and transport details.

Workflow:
1. Treat the text typed after `/spec` as the source request.
2. Search existing specs first to avoid duplicates and to identify the best component and parent.
3. Search existing tickets for the same work before deciding whether this should create a new spec or update an existing one.
4. Prefer `spec-mcp` tools such as `spec_search`, `spec_list`, `spec_tree`, `spec_create`, and `spec_update` when they are available.
5. If `spec-mcp` is unavailable, fall back to `./target/debug/spec.exe` and register `.spec/specs` with `spec.exe add-root .spec/specs --label default --json` if needed.
6. Infer a clear title, slug, component, and parent. Keep slugs lowercase, use `-` within segments, and `/` between segments.
7. Prefer updating a matching spec over creating a near-duplicate. If no matching spec exists, create one in `draft` state.
8. Ensure the spec body captures the intended system properties, explicit acceptance criteria, required evidence or traceability needed to evaluate implementation, and non-goals when obvious. Keep problem statements, current-state analysis, rollout sequencing, blockers, and implementation notes in related tickets unless the user explicitly asks for them in the spec.
9. When linking tickets in the spec or chat output, never synthesize a ticket folder path from a UUID, a store root, or an example path. If the first ticket-api response omits the path, run a follow-up ticket-api command that returns the authoritative path before responding. Use the exact returned folder path as the link base and append `/ticket.toml` for the markdown target so editors can open the file directly.
10. If the request clearly implies implementation work and the related ticket does not exist yet, create the needed ticket first or state explicitly that ticket creation is still required before implementation begins.
11. If required details are still ambiguous after a focused search, ask one concise clarification rather than guessing.
12. Do not implement code or change unrelated files unless the user explicitly asks.

Response:
- created or updated spec slug and id
- chosen component and parent
- related tickets, rendered as markdown links of the form `[<short-id> <title>](<exact ticket folder path returned by ticket-api output>/ticket.toml)`, if any
- key assumptions
- duplicate candidates considered, if any
