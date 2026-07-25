Build an automated prompt-replay matrix that proves an agent can locate and load the correct skill by its description for each target domain.

Anchor spec: agents/skill-infrastructure (a9b7ef39) — AC7. Runs after skills land and after migration + prune.

Scope:
- One replay case per domain: Rust, browser/Playwright, WebGPU/3D, Dioxus, interviewing, skill-authoring.
- Each case asserts the by-description loader selects the intended skill folder for a representative task prompt.
- Record outcomes as test-api validation evidence linked to this ticket and the anchor spec.

Acceptance criteria (verifiable):
- AC-1: A validation spec (test-api) exists for skill-discovery-by-description.
- AC-2: Every target domain has a replay case; each passes (correct skill selected).
- AC-3: Executions recorded in test-api, linked to this ticket + spec a9b7ef39.
- AC-4: A failing/missing skill produces a clear, actionable failure (negative case covered).