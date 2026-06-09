## Problem

The memory-index track requires automatic regeneration of `.ticket/README.md`, `.ticket/index.toon`, `.spec/index.toon`, `.rule/index.toon`, `.audit/index.toon`, workspace summaries, and `.agents/` hook entries when source content changes. The current repository state has no git-hook implementation or manifest entry that performs this work. Existing hook surfaces only cover rule sync, docs validation, terminal PWD capture, and session capture.

## Goal

Define and implement the repository-local git-hook automation for store-index generation so the generator tickets have one concrete execution surface instead of each ticket hand-waving at `pre-commit/post-commit hooks`.

## Scope

- Extend the repository git hook surface under `.githooks/`, not the Copilot hook manifest under `.github/hooks/hooks.json`.
- Define the exact staged-path trigger matrix for each generator domain and the generated outputs it owns.
- Add one shared regeneration entrypoint or wrapper command per domain so pre-commit can invoke the correct generator deterministically.
- Define and implement the pre-commit behavior when generated files drift: regenerate, report the touched outputs, and require the user to re-stage them.
- Define and implement the fallback path when the profiled pre-commit budget is exceeded: post-commit regeneration, or an explicit documented opt-out for domains that cannot safely run inside pre-commit.
- Record the latency budget and profiling evidence expected by the memory-index plan.
- Update the five generator tickets so they depend on this hook automation slice instead of each carrying an implicit, duplicated hook requirement.

## Acceptance Criteria

- `.githooks/pre-commit` has an explicit, repository-local branch for store-index generation.
- The hook trigger matrix names the staged path patterns and generated outputs for ticket, spec, rule, audit, and workspace index generation.
- The implementation distinguishes git hooks from editor/Copilot hooks so the execution surface is no longer ambiguous.
- When generated index outputs drift, the hook regenerates them and exits non-zero with a clear restage instruction.
- The fallback behavior for commands exceeding the budget is defined and implemented or explicitly documented as unsupported for a domain.
- Profiling evidence is captured for incremental runs, including the target threshold from the owning spec.

## Non-goals

- Does not define the semantic digest inputs for each domain.
- Does not redesign the `IndexEntry` or `IndexSidecar` schema.
- Does not replace existing docs/session/editor hooks in `.github/hooks/hooks.json`.

## Resolved decisions carried into this ticket

- Git hook automation belongs in `.githooks/pre-commit` with optional post-commit fallback, per the memory-index plan and spec.
- This is repository hook automation, not Copilot hook automation.
