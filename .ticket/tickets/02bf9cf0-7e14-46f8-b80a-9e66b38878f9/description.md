# Problem

The repository workflow now requires targeted validation to run until it passes or repeatedly fails with a documented blocker, but there is no dedicated workflow test-tool that can gather those checks and produce a reusable status artifact.

Today the strongest available fallback is to run commands manually and paste the result into ticket/spec summaries. That works, but it is fragile, inconsistent, and easy to skip.

# Scope

Create a dedicated workflow validation tool surface for repository work.

The tool may be a CLI, MCP surface, or another first-class workflow entrypoint, but it must support the repository workflow rather than only one crate or viewer.

The tool should:

- run focused validation commands for a change
- capture whether a check passed, failed, or is blocked
- record the exact command or manual verification step used
- emit structured output that can be linked or copied into ticket/spec summaries
- make partial tooling support explicit instead of silently dropping checks

# Acceptance criteria

- A dedicated workflow validation tool exists for running and recording required checks.
- The tool reports pass, fail, and blocked outcomes together with the exact command or manual step used.
- The output is structured enough to feed ticket/spec status summaries and review readiness checks.
- Repository guidance can reference the tool as the preferred validation path where appropriate.
