## Test Execution Strategy

- Start with nearest unit/integration tests.
- Expand to crate-level runs once local failures are resolved.
- Keep working outward until the required validation passes or you have a clearly repeated blocker to report.
- Prefer the strongest focused validation surface already owned by the changed tool or crate; run the underlying command directly and record the exact command or manual step in ticket/spec summaries.
- For documentation or generated-guidance checks, run the relevant validation command directly and record unsupported coverage or manual follow-up explicitly.
- If dedicated automation is unavailable, use the closest manual or command-line validation path and record the limitation in the status summary.
- Avoid unrelated full-workspace test runs unless required.