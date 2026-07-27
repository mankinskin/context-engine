<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=3e9d967e-68c6-4c4c-96ae-eb3f974931cf slug=agent-harness/interactive-chat-ui digest=04e14f89e528 -->

# Agent-driven interactive chat UI (UI sandbox interaction protocol + skill)

- slug: `agent-harness/interactive-chat-ui`
- component: agent-harness
- state: agent-harness
- index_ref: `.spec/specs/3e9d967e-68c6-4c4c-96ae-eb3f974931cf/spec.toml`

## Summary

Extend the agent harness so the agent can **actively drive the shared chat UI** ("UI sandbox") — the common interface between agent and user. The agent controls a virtual world through the chat surfa…

## Acceptance Criteria Excerpt

`agent-shared` exposes a versioned interaction protocol enum with round-trip `serde` coverage. Agent loop in `agent-core` can emit each interaction kind and correlate user responses within one session. Dioxus WASM chat UI renders every interaction kind; Ratatui TUI renders each …

## Navigation

- Parent: _(root)_
- Children: _(none)_
