## Goal

Define a typed, versioned agent->UI interaction protocol in `agent-shared`, enumerating every interaction the agent may perform in the UI sandbox. This is the foundation all other chat-UI-interaction work depends on.

## Interaction kinds (minimum)

- Question/answer UI element (structured prompt with selectable and/or typed responses).
- Generic input request (text, choice, and other input kinds).
- Microphone control (enable/disable capture) + capture-state events.
- Targeted text output / message emission.
- Image display (agent references an image to show).
- Presentation control: UI scene selection, colors, and chat-message format.
- User-response envelope correlating a response back to the originating interaction (id/session).

## Requirements

- Use `serde(tag = "type", content = "payload")` tagged enums, consistent with the existing agent-shared message schema; extend, do not replace it.
- Include a protocol version marker so frontends can gate rendering.
- Provide response variants so user replies flow back over the same channel/session.

## Acceptance criteria

- Interaction protocol enum + response envelope compile in `agent-shared`.
- serde round-trip unit tests cover every interaction and response variant.
- `cargo test -p agent-shared` passes; failure logs under target/test-logs/.