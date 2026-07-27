<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=1d62442b-61dc-4eeb-9b7c-e933f84470f2 slug=ticket-api/state-transition-path-unification digest=8906c0562c4a -->

# ticket-api state transition path unification

- slug: `ticket-api/state-transition-path-unification`
- component: ticket-api
- scope: internal
- state: draft
- index_ref: `.spec/specs/1d62442b-61dc-4eeb-9b7c-e933f84470f2/spec.toml`

## Summary

<!-- aligned-structure:v1 -->

## Acceptance Criteria Excerpt

`ticket update` can progress through valid consecutive intermediate states in one call. `close_ticket` and `update_ticket` share the same transition-path logic. `from_state` is removed or ignored in favor of store-derived current state. Focused ticket-api, ticket-cli, ticket-htt…

## Navigation

- Parent: _(root)_
- Children: _(none)_
