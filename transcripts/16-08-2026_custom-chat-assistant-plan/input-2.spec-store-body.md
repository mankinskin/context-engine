<!-- aligned-structure:v2 -->

# Rig-Based Chat Contract for the Unified Agent Harness

## Motivation

The Unified Agent Harness needs a provider-neutral chat contract whose first
provider adapter uses Rust `rig`. The contract must let one authenticated
session accept a text message with optional file attachments and expose the same
ordered answer to a CLI client and a Dioxus browser client. The contract extends
the existing single-session model: interactive chat and autonomous loops remain
modes of one `Session`, not separate subsystems.

## Dependent Expectation

If this specification is implemented, a dependent can rely on the following
observable properties:

1. A `Session` is the sole authority for the ordering and identity of its
   authentication state, commands, messages, attachments, and answer events.
2. An authenticated `Session` accepts a valid `ChatMessage` with zero or more
   valid `Attachment` references and produces one ordered answer outcome: a
   final answer or a typed failure.
3. A CLI client and a Dioxus browser client use the same public session protocol
   and can observe the same answer sequence for the same session.
4. Direct `rig` types and raw credential material are not values of the shared,
   persisted, or client-visible contract.

## Entity Contract

| Entity | Identity | Required relations | Invariants |
|---|---|---|---|
| `Session` | `session_id` | Owns `AuthState`, `SessionCommand`, `ChatMessage`, and `ResponseEvent`. | A session is the only source of event order for its messages. |
| `CredentialCapability` | Opaque non-persisted capability | Authorizes provider use for one session context. | Raw credential material is not serialized, logged, checkpointed, or exposed to either client. |
| `ChatMessage` | `message_id` within a session | References zero or more `Attachment` values and one answer outcome. | A message is immutable after acceptance. |
| `Attachment` | `attachment_id` | Has immutable metadata and separate content storage. | Attachment bytes are absent from event logs; only the reference and metadata cross the core boundary. |
| `ResponseEvent` | `session_id`, `message_id`, `sequence` | Belongs to one accepted message. | Sequence values are strictly monotonic for a response; exactly one terminal event exists. |
| `ProviderAdapter` | Adapter implementation identity | Implements the provider contract for `agent-core`. | Provider-specific values, including `rig` types, do not cross the adapter boundary. |
| `ClientSubscription` | Client identity plus session/cursor | Observes public events for a session. | A client cannot alter session truth by local state alone. |

## Component Positions

| Position | Contract | Readiness | Code reference |
|---|---|---|---|
| `agent-shared` | Provider-neutral versioned tagged schemas for session commands, auth state, messages, attachment references, and response events. | not-implemented | Planned by [CH1](.ticket/tickets/a5f08931-24af-4b96-a156-9107c776f946/ticket.toml). |
| `agent-core` | Session state machine and provider trait that rejects unauthenticated or invalid requests and emits ordered normalized outcomes. | not-implemented | Planned by [CH2](.ticket/tickets/c684b092-7f5a-4ebe-aa6d-494f666f5dc8/ticket.toml) and [CH3](.ticket/tickets/036c270f-6ca7-4372-96e2-570a26e3fdd0/ticket.toml). |
| `agent-rig-adapter` | Leaf adapter implementing the core provider trait with `rig`, including redacted auth and normalized streaming conversion. | not-implemented | No existing code position; requires a dedicated child ticket and Cargo dependency review. |
| `agent-server` | Authenticated command transport, attachment-reference intake, event fan-out, and resume-after-cursor semantics. | not-implemented | Planned by [CH6](.ticket/tickets/8ed0edbf-a765-4f4a-b50e-695aa79e9180/ticket.toml). |
| `agent-cli` | Scriptable public-protocol client that emits answer events or final answer text and redacted failures. | not-implemented | No existing code position; requires a dedicated child ticket. |
| Dioxus browser client | Browser client that renders authenticated state, composition, attachment selection/drop, progressive answer output, and reconnect state. | not-implemented | Planned by [CH8](.ticket/tickets/86f95ad8-8d61-43b6-a463-8719b29007c0/ticket.toml). |
| MCP routing | Per-session tool routing and audit correlation. | not-implemented | Planned by [CH4](.ticket/tickets/1c63db9d-afb3-4678-b0f6-14e6a4d5daca/ticket.toml). |

## Component Contracts

### Shared Protocol

`agent-shared` defines tagged, versioned public message representations. Each
representation must round-trip without loss. `AuthState` and `AttachmentRef`
must not have a field capable of holding raw credentials or attachment bytes.

### Session Core

`agent-core` accepts `SessionCommand` values only for their owning `Session`.
The core accepts `ChatMessage` only after successful authentication and
attachment validation. For each accepted message, the core emits strictly ordered
`ResponseEvent` values ending in one final-answer event or one typed-failure
event. Policy or budget denial is a typed failure before any provider request.

### Rig Adapter

`agent-rig-adapter` translates only normalized core requests to and from `rig`.
The adapter owns provider authentication and maps provider errors to normalized
failure categories. Replacing the adapter cannot require a change to the shared
schema, core state machine, server protocol, or client protocol.

### Session Transport

`agent-server` accepts authentication, chat, attachment, and subscription
operations and correlates all outputs with a `session_id` and `message_id`. Two
subscribers to one session observe equivalent ordered frames. A subscriber that
reconnects with its last observed sequence receives the remaining frames in
order. The transport retains attachment content separately from public events.

### Client Contract

`agent-cli` and the Dioxus browser client are interchangeable observers and
command issuers for the same public protocol. Each client can present
authentication success or safe failure, submit text, attach a file, observe
progressive answer events, and display the final answer. Neither client receives
raw provider credentials.

## Requirement-to-Evidence Contract

Every requirement is decidable only when the named guard produces passing
evidence. Guard identifiers below are required future `test-api` ValidationSpec
records; their absence means the requirement remains unverified.

| Requirement | Decidable condition | Required guard and evidence | Current status |
|---|---|---|---|
| `REQ-01` Typed session protocol | Public entity schemas round-trip and exclude secret/byte fields. | `VS-agent-shared-schema`: unit/property test report. | Guard not registered; unverified. |
| `REQ-02` Authentication gate | Unauthenticated chat is rejected; valid test authentication enables chat; invalid auth is redacted. | `VS-agent-auth`: core plus server integration report. | Guard not registered; unverified. |
| `REQ-03` Rig isolation | Only the adapter imports `rig`; normalized behavior is preserved through a fake transport. | `VS-rig-adapter`: dependency-boundary check plus adapter tests. | Guard not registered; unverified. |
| `REQ-04` Text chat answer | One valid message yields strictly ordered events and exactly one terminal outcome. | `VS-chat-stream`: core and server integration report. | Guard not registered; unverified. |
| `REQ-05` Attachment handling | A valid fixture attachment is referenced by the answer; invalid metadata is rejected; bytes are not logged. | `VS-attachment-contract`: server integration report. | Guard not registered; unverified. |
| `REQ-06` CLI conformance | CLI authenticates, sends text and fixture attachment, and prints final answer or redacted failure. | `VS-cli-chat`: black-box CLI test report. | Guard not registered; unverified. |
| `REQ-07` Browser conformance | Dioxus browser flow covers auth, message, attachment selection/drop, progressive output, final answer, and safe failure. | `VS-browser-chat`: Playwright report with trace and screenshots. | Guard not registered; unverified. |
| `REQ-08` Cross-client session truth | Equivalent subscriptions from CLI and browser observe the same ordered session events, including resume after disconnect. | `VS-session-resume`: server/client integration and browser evidence. | Guard not registered; unverified. |
| `REQ-09` MCP session isolation | Concurrent sessions route tools and audit records by their own session and tool-call identities. | Existing CH4 validation plus `VS-mcp-session-routing`. | Guard not registered; unverified. |

## Guards

No ValidationSpec IDs have been created for `REQ-01` through `REQ-09`. Therefore
this specification is **coming soon / not implemented** and no dependent may
claim verified behavior from this contract. The listed guard names are the
required ValidationSpec creation set; each guard must record its command, test
artifact, and result before this spec can be treated as verified.

## Governing-Rule Requirement

No PolicyRule currently introduces this revised specification in-session. Before
the spec becomes active, a PolicyRule must introduce
`agent-harness/unified-operator-interface` with readiness
`coming-soon / not-implemented` and direct readers to the positions and guards
above. The governing rule must switch the introduction to `partial-with-gaps` or
`implemented` only when the corresponding positions and ValidationSpec guards
provide evidence.

## Related Specs and Tickets

- Parent contract: this specification remains the owner of the unified operator
  interface, rather than creating a parallel chat-system specification.
- Tracker: [0f4b3c5b Unified Agent Harness](.ticket/tickets/0f4b3c5b-c5e9-45c4-968c-a8878f359de8/ticket.toml).
- Provider decision: [036c270f Provider abstraction](.ticket/tickets/036c270f-6ca7-4372-96e2-570a26e3fdd0/ticket.toml) requires an amendment because the existing plan names `genai` rather than `rig`.
- Streaming: [8ed0edbf Axum lifecycle and fan-out](.ticket/tickets/8ed0edbf-a765-4f4a-b50e-695aa79e9180/ticket.toml).
- Terminal: [3c208991 Ratatui interface](.ticket/tickets/3c208991-1d98-4a9c-be29-890d15244b8d/ticket.toml).
- Browser: [86f95ad8 Dioxus/WASM parity](.ticket/tickets/86f95ad8-8d61-43b6-a463-8719b29007c0/ticket.toml).
- Browser release gate: [b01a2fbf E2E and Playwright](.ticket/tickets/b01a2fbf-6682-4dee-abce-95cdcf4fd325/ticket.toml).
- The Rig adapter, CLI contract, validation guards, and governing rule need
  separate linked entities before implementation can begin.

## Spec Tooling Observation

The current spec body can name required guard records, but cannot create or
validate `test-api` ValidationSpecs atomically from the Spec CLI. A useful Spec
tool improvement is a typed `requirement` entity with stable IDs and explicit
links to `ValidationSpec`, ticket, code position, and evidence status. Such an
entity would make the requirement-to-evidence table queryable and would let
`spec health` fail when a required guard is absent or stale.