Add tests and validation to ensure Worker-tier (write-and-die) sub-agents terminate after one isolated step.

Acceptance criteria:
- Tests assert a Worker sub-agent cannot continue to a second step in the same session and must return `{pass:false, blocker:"..."}` when blocked.
- Documentation updated with test evidence and a reproduction harness snippet.

Traceability:
- References instruction write-and-die.instructions.md and ticket 1fbf2d84-4a6b-4d8e-a69e-45aec87ff95f for related loop-closure changes.
- Tied to spec 63c60c9d-adbe-4ddb-8c1d-6156610d0753 as an example worker usage.

Notes:
- Workspace: C:/Users/linus/git/context-engine/.ticket