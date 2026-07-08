No live stakeholder was available during refinement, so this questionnaire
captures the questions asked and the assumed answers driving D1–D5 and scope.
Operator: confirm or overturn each; overturned answers must update the
`resolved-decisions-and-assumptions` section and affected child tickets.

**Mode transitions (interactive <-> autonomous)**
- Q: Should promoting a chat to a loop keep the same session id and full history?
  A (assumed): Yes — one session model; promotion is a mode change, not a new session.
- Q: Can a running loop be "demoted" back to interactive for a manual turn?
  A (assumed): Yes — pause loop, take interactive turns, resume loop from checkpoint.

**Safety and budget policy**
- Q: Default token budget per loop and behavior at limit?
  A (assumed): 200k tokens; hard stop + resumable checkpoint (D4).
- Q: Must every host command run sandboxed?
  A (assumed): Yes — Docker sandbox via `bollard`; deny-by-default outside the
  per-session working dir (D3).
- Q: Max concurrent autonomous loops?
  A (assumed): 4, configurable (D4).

**Observability and recovery**
- Q: What must survive a process crash?
  A (assumed): Session events + periodic checkpoints (append-only NDJSON, D2);
  a loop resumes from the last checkpoint.
- Q: Required audit granularity?
  A (assumed): tool calls, command invocations, exit codes, artifact paths, and
  budget/policy decisions, correlated by session + tool-call id.

**UX constraints (minimal TUI + WASM parity)**
- Q: Minimum shared controls across both clients?
  A (assumed): start conversation, toggle to loop, pause/resume/stop, inspect
  live events, view diffs (`similar`).
- Q: Must TUI and WASM be behaviorally identical?
  A (assumed): Same interaction model and semantics; presentation may differ,
  but control set and session semantics must match.
- Q: Browser verification bar?
  A (assumed): External Chromium-family browser at a documented resolution +
  Playwright screenshots for transient surfaces (per AGENTS quality gates).
