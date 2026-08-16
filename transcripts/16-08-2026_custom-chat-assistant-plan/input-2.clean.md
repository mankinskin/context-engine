# Architecture and Verification Plan: Rig-Based Agent Chat

Extend the planned Agent Harness as the single architecture for a modular Rust
chat system. The system must use the Rust `rig` library for provider access and
must validate both an agent CLI and a browser-hosted Dioxus frontend.

## Architecture Baseline

Preserve the planned Agent Harness boundaries:

- `agent-shared` owns versioned, serde-typed session, chat, attachment, and
  streamed-response events.
- `agent-core` owns the session state machine, chat orchestration, provider
  interface, policy hooks, and budgets.
- `agent-server` owns Axum HTTP/WebSocket endpoints, authentication hand-off,
  and fan-out of session events.
- `agent-uapi` owns presentation-specific adapters for the native Ratatui client
  and browser Dioxus client.
- The CLI and browser client consume the same server protocol and session model.

The existing provider workstream currently specifies `genai`. Using `rig` is a
required design change and must be recorded as an amendment or a dependent
provider-integration ticket before implementation.

## Target Modular Design

Use the following dependency direction:

```text
agent-shared <- agent-core <- agent-server <- agent-uapi
                   ^                         ^
                   |                         |
           agent-rig-adapter            agent-cli / Dioxus browser client
```

- `agent-shared`: provider-neutral request, response, attachment, authentication
  state, and stream-event schemas. Include serde round-trip tests.
- `agent-core`: a provider trait, chat/session orchestration, attachment metadata
  validation, budget and policy checks, and persistence/checkpoint hooks. Do not
  import `rig` here.
- `agent-rig-adapter`: a leaf crate that implements the `agent-core` provider
  trait using `rig`. It owns `rig` configuration, provider authentication,
  request/response conversion, streaming conversion, and a mockable test seam.
- `agent-server`: authenticated chat and attachment endpoints plus WebSocket
  event streaming. It converts transport requests into `agent-core` commands and
  never exposes provider credentials to either client.
- `agent-cli`: a thin command client for sending a chat message, supplying an
  attachment, reading streamed/final output, and reporting authentication errors.
- Dioxus browser frontend: an authenticated chat surface with a text composer,
  attachment control, progressive response rendering, and visible failure state.

## Verification Plan

### 1. Fast Unit Tests

- `agent-shared`: verify every event and attachment schema can serialize and
  deserialize without changing its tagged representation.
- `agent-core`: use `MockProvider` to verify a simple prompt produces ordered
  response events; reject an attachment that violates the defined metadata or
  size policy; verify unauthenticated provider state is returned as a typed error.
- `agent-rig-adapter`: test request, authentication, and streaming conversion
  against a local fake transport. Tests must not call an external provider.

Run the relevant `cargo test -p <crate>` command for each changed crate.

### 2. Server and CLI Integration Tests

Launch `agent-server` with the fake Rig transport and exercise the public
protocol through `agent-cli`:

1. Authenticate with a valid test credential and confirm the session reaches an
   authenticated state.
2. Send a simple chat message and assert that the CLI receives the expected
   ordered response frames and final answer.
3. Attach a small fixture file, assert that the server records attachment
   metadata, and assert that the agent response can reference the attachment.
4. Use invalid credentials and assert that no provider request is made and that
   the CLI emits a redacted authentication failure.
5. Disconnect during a streamed answer, reconnect, and confirm the same session
   can read the final stored answer.

The server test harness uses only local mocks, fixture files, and ephemeral test
state. Provider credentials and real network calls are excluded from CI.

### 3. Browser End-to-End Tests

Add Playwright coverage around a Dioxus browser build backed by the same fake
server. The suite must cover:

1. A user sends a simple message and sees progressive response text followed by
   the final answer.
2. The authentication UI accepts a valid test credential, persists only a safe
   authenticated state, and sends an authorized request.
3. A user selects or drops a fixture file, sees the attachment in the chat
   composer, submits it, and sees the returned answer.
4. Invalid authentication shows an actionable error without exposing a token.

Capture Playwright traces and screenshots for each browser-facing flow. Perform
one external Chromium verification at a recorded desktop resolution after the
automated suite passes.

## Delivery Sequence

1. Amend the provider architecture to make `rig` an approved provider adapter
   dependency and define its authentication contract.
2. Deliver the provider-neutral schemas and `agent-core` provider trait with
   mock-based unit tests.
3. Add `agent-rig-adapter`, register its Cargo dependency, and validate the fake
   provider path without network access.
4. Deliver the smallest vertical slice in `agent-server` and `agent-cli`:
   authenticate, send text, and read a final answer.
5. Extend the slice with file attachments and streamed-response resume behavior.
6. Deliver the Dioxus browser flow using the same protocol and fake server.
7. Add the full Playwright suite, screenshots, manual Chromium evidence, and the
   release-gate documentation.

## Required Decisions Before Implementation

1. Which `rig` provider integration and authentication method are required?
2. Which credential storage and session-expiry policy is acceptable for CLI and
   browser clients?
3. Which attachment types, size limits, retention rules, and provider-visible
   metadata are supported?
4. Does a completed answer require token-by-token streaming, resumable chunks, or
   only a final response?
5. Which agent CLI commands are public, and which actions require confirmation?

## Non-Goals

- Do not couple `agent-core` directly to `rig`.
- Do not use real provider credentials or external provider calls in automated
  tests.
- Do not expand the file attachment feature into a full editor, repository
  browser, or native Dioxus application in the first verification slice.

## Evidence Anchors

- `.ticket/tickets/0f4b3c5b-c5e9-45c4-968c-a8878f359de8/description.md`
- `.ticket/tickets/036c270f-6ca7-4372-96e2-570a26e3fdd0/description.md`
- `.ticket/tickets/3c208991-1d98-4a9c-be29-890d15244b8d/ticket.toml`
- `.ticket/tickets/86f95ad8-8d61-43b6-a463-8719b29007c0/ticket.toml`
- `DESIGN_AGENT_HARNESS.md`
- `AGENTS.md`