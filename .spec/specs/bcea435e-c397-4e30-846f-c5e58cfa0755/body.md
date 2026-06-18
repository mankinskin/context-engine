# Smooth repository README surfaces

## Goal

Make the repository-root README surfaces navigable and consistent enough to iterate on without re-auditing the same structural gaps each time.

## Scope

- Manual quick-fix refresh for `context-engine/README.md` and `context-stack/README.md`.
- Rule-source updates for the generated repository READMEs in `memory-viewers`, `memory-viewers/memory-api`, and `memory-viewers/viewer-api`.

## Requirements

- Root, context-stack, memory-viewers, memory-api, and viewer-api repository READMEs expose clickable child README or documentation links where applicable.
- Generated repository READMEs are updated through their canonical `.rule` source files where generation already exists.
- Repository READMEs explicitly call out executable binaries or installable content, or state that the repo root has no installable binary surface.
- Commands referenced in those repository READMEs link to direct README or command documentation.

## Related Ticket

- [2fb3adb0 Smooth repository README surfaces](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/2fb3adb0-fa3a-41a6-8fd6-38096635a38b/ticket.toml)

## Validation

- `rule sync-targets --config memory-viewers/rule-targets.yaml --workspace-root memory-viewers`
- `rule sync-targets --config memory-api/rule-targets.yaml --workspace-root memory-viewers/memory-api`
- `rule sync-targets --config viewer-api/rule-targets.yaml --workspace-root memory-viewers/viewer-api`
- Manual sanity read of `README.md` and `context-stack/README.md` after editing.
