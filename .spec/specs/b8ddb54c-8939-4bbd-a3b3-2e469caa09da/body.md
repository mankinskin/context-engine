# Summary

Give `context-stack` a repo-local rule workspace and generate both its root README and its first-level child README tree from local rules.

## Problem

`context-stack` is still outside the repo-local README generation pattern. It has no local `.rule` store, its root README is manual, and its first-level child READMEs do not form a generated parent/child navigation chain.

## Scope

This spec covers:

- a repo-local `context-stack/.rule` store
- a local `context-stack/rule-targets.yaml` shim plus themed fragments
- a generated `context-stack/README.md`
- generated first-level child README targets for the in-scope `context-stack` surfaces

## Intended Behavior

- `context-stack` can run `rule explain-target`, `rule sync-targets`, and `rule sync-targets --check` from its own repo root.
- The root README clearly states the repo has no root executable binary surface.
- First-level child READMEs link back to `context-stack/README.md` via repo-internal parent blocks.
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
