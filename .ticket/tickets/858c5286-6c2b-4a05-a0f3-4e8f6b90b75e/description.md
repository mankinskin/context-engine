Phase B parent tracker. Split each of the 11 domain tools into its own bare-named repository under github.com/mankinskin. Each tool repo bundles that domain's api crate + all transports (cli, mcp, http) + its viewer + vscode extension (where they exist) + its tool-scoped artifact stores.

## Common per-tool extraction recipe
1. Assemble the tool's crates/transports from memory-api/{crates,tools} and its viewer from memory-viewers into the new repo.
2. Declare `memory-kernel`, `viewer-api`, and `memory-fixtures` as external dependencies.
3. Preserve git history where practical (subtree/filter-repo).
4. Migrate the tool's tool-scoped artifacts (tickets/specs/docs for that tool) into the tool repo's own stores using the safe cross-workspace move tooling (memory-api `505b2cd4`) so references are relinked, not broken.
5. Independent build + test + transport smoke (cli/mcp/http) + viewer browser verification.
6. Register the tool repo as a dependency of `workflow-tools`.

## Children (one per tool)
ticket, spec, rule, doc, test, log, feedback, session, audit, peek, interview.

## Acceptance criteria
- Every child tool repo builds/tests independently and passes transport + viewer verification.
- Tool-scoped artifacts moved with reference integrity preserved (no dangling refs).
- All child tickets closed and aggregated into workflow-tools.

## Dependencies
- Blocked by foundations (memory-kernel, shared libs).
- Artifact moves blocked on cross-store move tooling (memory-api `505b2cd4`).