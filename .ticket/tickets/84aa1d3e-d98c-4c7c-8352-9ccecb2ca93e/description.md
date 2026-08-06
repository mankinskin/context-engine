Add the mandated pre-dispatch gate checks for Implement delegations (Explore Agent gate).

Acceptance criteria:
- Gate verifies ticket dispatchability, spec coverage, target paths existence, and validation commands presence (see pre-dispatch gates doc).
- Gate returns `{pass: true, bundle: ...}` or `{pass: false, blocker: "<exact reason>"}` per contract.
- Gate enforces its ≤5 turns/≤10 tool calls ceiling and fails with a precise blocker when exceeded.

Traceability:
- References orchestration gate guidance ticket 46d8b25d-e80c-4170-9601-1c26a7a0bcb8 (Move quality gates before dispatch).
- Tied to spec 63c60c9d-adbe-4ddb-8c1d-6156610d0753 for handoff expectations and to epic e342cc4c-a7a4-42de-81fc-572d0497d12b.

Notes:
- Workspace: C:/Users/linus/git/context-engine/.ticket