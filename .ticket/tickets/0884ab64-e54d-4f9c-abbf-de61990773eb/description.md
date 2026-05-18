# Session execution, per-session MCP, and artifact capture

## Scope

- Copilot completions client and session runner.
- Per-session MCP server isolation.
- Agent process execution inside the provisioned Firecracker microVM when the workload fits the microVM path.
- Browser runner implementation for `browser-container` and `browser-gpu-container` sessions after admission succeeds.
- Deterministic Chromium-family launch profile with pinned browser version, viewport, device scale factor, locale, timezone, and font bundle.
- Playwright attachment and browser artifact collection for compatibility-lane sessions.
- Budget and time guardrails sized for first functional operation.
- Capture stdout, stderr, test output, screenshots, and structured results to `.aoh/archive/...`.
- Browser or GPU-bound tasks use the narrow compatibility runtime path or are explicitly rejected by policy; they are not silently pushed through the Firecracker primary path.
- No orchestrator-owned chromiumoxide CDP control or live screencast streaming.

## Acceptance criteria

- A session can start, run, and finish inside an isolated Firecracker-backed sandbox.
- A session can start, run, and finish in the selected runtime lane, including browser compatibility lanes when the selector routes it there.
- Per-session MCP tool access is isolated.
- The runner captures artifacts and writes a usable archive bundle.
- Browser sessions write the required lane metadata, browser version, viewport, and GPU status into the archive bundle.
- Failure paths produce typed status and archived evidence.
- Workload routing between the Firecracker path and the compatibility path is explicit and deterministic.
- GPU-required browser sessions either run on an admitted GPU lane or fail admission; they are never silently downgraded to software rendering.