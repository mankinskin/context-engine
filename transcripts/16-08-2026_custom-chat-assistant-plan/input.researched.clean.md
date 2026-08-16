# Research-Backed Request: Extend the Agent Harness Plan

Evaluate the planned Agent Harness tracker before proposing a new chat-assistant
architecture. Use the existing Rust-first design as the baseline; do not create a
parallel or duplicate plan.

## Goal

Determine how the custom developer chat assistant can extend the existing Agent
Harness into one coherent product with a terminal client and a Dioxus client. The
clients must share session semantics and handle user input, streamed agent output,
tool activity, and change previews.

## Existing Baseline

Treat ticket `0f4b3c5b` as the primary plan. The tracker already specifies a Rust
core, a Ratatui client, a Dioxus/WASM client, shared sessions, MCP tool routing,
streaming, sandboxing, and diff previews. Map every requested capability to the
relevant existing workstream or child ticket before proposing new work.

## Capabilities To Map Or Specify As Deltas

- Provider client and authentication: assess whether the requested Rust `rig`
  library should replace, wrap, or remain separate from the planned provider
  abstraction. The repository currently has no verified `rig` dependency.
- Terminal client: extend the planned Ratatui operator interface with the required
  chat input and output workflow.
- Dioxus client: support the browser client and determine the required native
  target. Existing planning explicitly covers Dioxus/WASM, not native desktop
  packaging.
- Sessions: preserve the existing unified session model across terminal and browser
  clients, including live output, pause/resume, and reconnect behavior.
- Workspace workflow: identify the smallest safe scope for a file tree, opening a
  file in an editor, visible change tracking, drag-and-drop file insertion, and
  loading a Git repository. Distinguish already planned diff previews from new
  file-management features.
- MCP tools: use the planned per-session tool-routing envelope and define the
  required permission and configuration experience in each client.
- Provider-supplied images: clarify whether this means image input, image output,
  or supplementary provider metadata. Define browser rendering and terminal
  degradation only after the intended behavior is confirmed.

## Required Research Deliverable

Produce a capability matrix with these columns:

1. Requested capability
2. Existing ticket, spec, or implementation surface
3. Coverage status: covered, partial, or missing
4. Required delta and owner boundary
5. Dependencies and validation evidence

Then propose only the new tickets needed for missing work, with explicit dependency
edges to the existing Agent Harness tickets. Preserve the existing plan's all-Rust
core, shared session protocol, MCP safety gates, sandboxing, and browser-validation
requirements.

## Decisions Required Before Implementation

1. Is `rig` mandatory, and if so, which existing provider abstraction will own the
   integration?
2. Which native Dioxus target is required, and must it have feature parity with the
   browser client?
3. Does "open files in an editor" mean invoking an external editor or implementing
   an in-app editor?
4. Which Git operations may the user perform, and which operations require explicit
   confirmation or sandboxing?
5. What exact provider-image behavior is required?
6. Which MCP tools are enabled by default, and how are grants approved, displayed,
   and revoked per session?

## Non-Goals

- Do not replace the Agent Harness architecture without evidence.
- Do not begin implementation during the research and planning phase.
- Do not assume a multi-provider strategy, a native Dioxus target, embedded editing,
  or image semantics before the listed decisions are resolved.

## Evidence Anchors

- `.ticket/tickets/0f4b3c5b-c5e9-45c4-968c-a8878f359de8/description.md`
- `.ticket/tickets/3c208991-1d98-4a9c-be29-890d15244b8d/ticket.toml`
