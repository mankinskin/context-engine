## Test Execution Strategy

- Start with nearest unit/integration tests.
- Expand to crate-level runs once local failures are resolved.
- Keep working outward until the required validation passes or you have a clearly repeated blocker to report.
- If dedicated automation is unavailable, use the closest manual or command-line validation path and record the limitation in the status summary.
- Avoid unrelated full-workspace test runs unless required.