Phase C. Migrate tool-scoped artifacts (tickets, specs, docs, rules, tests) that currently live in the context-engine default store or the memory-api store into the correct per-tool repo stores, preserving cross-reference integrity via the safe cross-workspace move tooling.

## Scope
- Classify existing artifacts as: cross-tool (→ workflow-tools repo-level stores), tool-scoped (→ that tool's repo store), or target-app (stay in context-engine).
- Execute journaled, reference-relinking moves (memory-api `505b2cd4`).
- Validate no dangling references remain after each batch.

## Acceptance criteria
- Every tool-scoped artifact relocated to its owning tool repo store.
- Cross-tool artifacts relocated to workflow-tools repo-level stores.
- Reference-integrity validation passes (no dangling edges/URNs) after migration.

## Dependencies
- Blocked by artifact-store establishment.
- Hard prerequisite: cross-store move tooling (memory-api `505b2cd4`) green.