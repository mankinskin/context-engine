## Goal

Let the `agent-core` ReAct loop emit UI-sandbox interaction directives as part of a normal turn, and correlate user responses back into the same session.

## Requirements

- Add an emission path so the loop can send any interaction kind (Q&A, input request, mic toggle, text output, image display, scene/color/message-format control) over the agent-server broadcast channel.
- Correlate inbound user responses (from the response envelope) to the originating interaction id and resume the loop deterministically within one session.
- Respect existing async-runtime isolation and session/mode model (interactive <-> loop) so UI interactions never deadlock core logic.

## Depends on

- agent-shared interaction protocol (UI1).

## Acceptance criteria

- agent-core can emit each interaction kind and await/correlate the matching response within a session.
- Unit/integration tests cover emit + response correlation and session continuity.
- `cargo test -p agent-core` passes; failure logs under target/test-logs/.