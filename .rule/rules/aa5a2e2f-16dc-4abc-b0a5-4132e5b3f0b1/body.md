### Structure the Spec (aligned-structure:v2)

Each spec must act as a dependable, verifiable contract. Every spec must start with the `<!-- aligned-structure:v2 -->` template marker and define the following five required sections:

1. **Motivation ("why")** — The user requirement or behavior need this spec satisfies, with optional links to feedback explaining its origin.
2. **Dependent expectation** — An explicit, clear contract clause: "If this spec is implemented, dependents can rely on behavior X."
3. **Guards** — Declared test-api `ValidationSpec` ids that gate the spec. The spec's `verified` state is COMPUTED from guard execution outcomes, never hand-set.
4. **Positions** — Current implementation/readiness status per referenced code symbol/path: `implemented`, `partial`, `not-implemented`, or `deprecated` with an explicit `code_ref`.
5. **Governing-rule requirement** — Link to the PolicyRule(s) that must introduce/explain this spec in-session (governed by the rule-introduces-spec mechanism).

Acceptance criteria and guards must be concrete enough that a reviewer or automated tool can tell exactly what evidence proves the contract is satisfied.