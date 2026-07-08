<!-- aligned-structure:v1 -->

# Summary

`WorkspaceManager::insert_first_match` induces graph structure from an existing sequence of resolved-or-resolvable tokens.

## Behavior Story

`WorkspaceManager::insert_first_match` induces graph structure from an existing sequence of resolved-or-resolvable tokens.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# insert_first_match

`WorkspaceManager::insert_first_match` induces graph structure from an existing
sequence of resolved-or-resolvable tokens.

## Contract

- The command requires at least two `TokenRef` values.
- Each `TokenRef` is resolved against the current workspace graph before any
  induction step begins.
- The command does not create atoms on its own; callers must reference existing
  tokens successfully.
- After resolution, the command feeds the resolved `Pattern` through
  `ReadCtx::read_sequence`, so the observable behavior is consistent with the
  current read-driven induction path rather than a bespoke API-side shortcut.
- The returned `InsertResult.token` is the induced or reused token.
- `InsertResult.already_existed` is `true` exactly when the operation reused an
  existing graph vertex rather than creating a new one.
- The workspace is marked dirty only when induction created new vertices.

## Notes

This spec intentionally describes the current implementation rather than the older
insert semantics guide, which still framed this command as a direct forward to
`context-insert` only.
