Build a generator that reads the test-api and log-api evidence stores and emits a markdown test catalog under `.test/README.md` and `.test/index.toon`. Gated behind dependent log/test-api bootstrap completions.

## Scope
- Implement a `test-catalog` subcommand that reads test evidence from test-api and log-api.
- Write output to `.test/README.md` (human) and `.test/index.toon` (machine, TOON primary, D8).
- For missing/not-run test cases, status must be mapped to `not-run`. The catalog is a complete registry of expected validation tests, not a results-only view (D7). Failures, benchmarks, and test audits may be surfaced on additional surfaces later.
- Group by component and owning spec, highlighting failures prominently.
- Emit an `.agents/` agent-hook node pointing agents at the test catalog (D1).
- All index files committed to git (D5).

## Dependency gating (D6)
- Depends on test-api bootstrap (86bf3da2) and log-api bootstrap (0805fb76), both currently `in-review`. Do NOT build against a stub/mock — wait for both to reach `done`.
- Any new store-contract surface this generator needs (e.g. a stats/summary or evidence-listing endpoint) must be folded into the existing test-api / log-api planning rather than added here. File or link the required follow-ups against those crates before implementation starts.

## Acceptance criteria
- Catalog output behaves as a complete test registry under `.test/`.
- Unexecuted test cases show up as `not-run`.
- Output format validates against the ContextNode schema (0dba399a).
- Any required test-api/log-api contract additions are tracked in those crates' planning, not improvised here.

## Non-goals
- Does not execute test runs; only aggregates store-owned evidence.
- No central `.context/` store directory.

## Resolved design decisions
- D6: wait for test-api/log-api bootstrap to be implemented; tie required new features into those crates' existing planning.
- D7: not-run entries included; catalog is a complete registry. D8: TOON sidecar. D5: committed.