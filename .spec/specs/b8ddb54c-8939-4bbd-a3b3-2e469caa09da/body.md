<!-- aligned-structure:v1 -->

# Summary

Give `context-stack` a repo-local rule workspace and generate both its root README and its first-level child README tree from local rules.

## Behavior Story

Give `context-stack` a repo-local rule workspace and generate both its root README and its first-level child README tree from local rules.

## Provided Surface Contracts

- README surfaces in scope follow an explicit rule-backed contract instead of one-off rollout prose.
- Parent and child README navigation stays repo-internal and mechanically derivable.
- README completeness and rollout status are verified by mechanical validation rather than manual review.

## Required Validation

- Contract clause validation: The migrated spec names the intended README structure and navigation behavior as explicit contract properties.
- Contract clause validation: The migrated spec names the validation path that checks the README contract mechanically.
- Contract clause validation: The migrated spec records enough evidence to tell whether the README contract is satisfied or blocked.
- The authored spec body documents the README contract, scope boundaries, and navigation expectations for this migration slice.
- The authored spec body documents the mechanical validation path required to prove this migration slice.
- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# Summary

Give `context-stack` a repo-local rule workspace and generate both its root README and its first-level child README tree from local rules.

## Problem

`context-stack` is still outside the repo-local README generation pattern. It has no local `.rule` store, its root README is manual, and its first-level child READMEs do not form a generated parent/child navigation chain.

## Scope

This spec covers:

- a repo-local `context-stack/.rule` store
- a local `context-stack/rule-targets.yaml` shim plus themed fragments
- a generated `context-stack/README.md`
- generated first-level child README targets for `context-api`, `context-trace`, `context-search`, `context-insert`, `context-read`, `context-trace-macros`, `ngrams`, and `packages/context-types`

## Intended Behavior

- `context-stack` can run `rule explain-target`, `rule sync-targets`, and `rule sync-targets --check` from its own repo root.
- The root README clearly states the repo has no root executable binary surface.
- First-level child READMEs link back to `context-stack/README.md` via repo-internal parent blocks.
- The previously undocumented first-level children (`context-trace-macros`, `ngrams`, and `packages/context-types`) gain generated README surfaces instead of remaining root-level exceptions.
- The generated README tree does not infer any external submodule parent.

## Assumptions To Prove

- A repo-local `.rule` store in `context-stack` will not conflict with ancestor-store resolution.
- The first-level child rollout can stop at repo-owned children and does not need to recurse through every deeper docs subtree immediately.
- The generated child READMEs can provide direct command-doc coverage through local or explicit external references.

## Test Strategy

1. Bootstrap the local rule store and root README target.
2. Validate local and ancestor-invoked generation for the root target.
3. Extend the same pipeline to first-level child README targets and re-run sync checks.

## Acceptance Criteria

- `context-stack` has a repo-local rule store and README target config.
- The root README and in-scope first-level child READMEs are generated from local rules.
- The `context-stack` README tree is internally navigable and checkable.

## Traceability

- [3f62f10e context-stack bootstrap ticket](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/3f62f10e-6f7d-4fa1-b205-97fe62babaf2/ticket.toml)
- [c785a6f6 context-stack child README ticket](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/c785a6f6-57d3-46d1-9a0e-36e1a4b74a47/ticket.toml)
