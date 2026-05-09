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