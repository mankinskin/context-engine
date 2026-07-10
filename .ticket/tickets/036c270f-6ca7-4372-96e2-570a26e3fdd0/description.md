# CH3 — Provider abstraction + guidance injector + budget/policy hooks

WS2. Implements D4 (budgets) and D5 (guidance precedence).

## Scope
- `genai` layer forwarding to OpenAI/Anthropic/OpenRouter by env vars.
- `.agentguidance` ingestion with deterministic layering:
  repo-root < path-scoped < session/task; most-specific wins; equal specificity ->
  newer/explicit wins; override recorded in audit trail.
- Async preflight hook trait array: token-budget check, policy check, loop throttle;
  run before expensive provider calls. Over-budget -> hard stop + resumable checkpoint.
- Default budgets: 200k tokens/loop, per-iteration caps (configurable).

## Acceptance criteria
- Guidance merge output deterministic for a given input set (test-asserted).
- Over-budget loop stops and records a resumable checkpoint reference.
- Provider selectable via env without code change.

## Dependencies
- depends_on CH2. Spec: unified-operator-interface D4, D5, AC 4.

## Validation matrix
| Part | Command / evidence |
|---|---|
| Fast check | `cargo check -p agent-core` |
| Primary gate | `cargo test -p agent-core` — provider selection, deterministic merge, budget hook block + record |
| Manual/browser | Not applicable |
| Failure logs | `target/test-logs/` |
