# Summary

Define an expectation-oriented specification contract for this repository so specs capture intended system properties plus explicit acceptance and evidence requirements, while tickets carry problem statements, current-state notes, rollout sequencing, and implementation details.

## Motivation

The current spec workflow combines two different responsibilities in one document shape:

- the product or architecture contract that should remain stable enough to evaluate
- the implementation and migration narrative that changes as work is discovered

That overlap was visible again in the recent README-rollout specs, which read more like rollout plans than expectations. The underlying architecture already points toward store-owned workflow metadata in `spec-api`, `doc-api`, future `test-api`, future `log-api`, and derived reporting in `audit-api`, but the repository does not yet have one coherent implementation track that turns that direction into the default spec contract.

## Scope

This spec covers:

- redefining the meaning of a spec around expected properties, acceptance clauses, and evidence requirements
- extending `spec-api` so those semantics are first-class store data rather than markdown folklore
- exposing the richer contract consistently through `spec-cli`, `spec-mcp`, and `spec-http`
- integrating store-owned documentation, validation, and log evidence with spec fulfillment
- reporting derived fulfillment status through `audit-api`
- piloting the new contract and migrating the affected specs and tickets homogeneously

## Intended Behavior

- The first implementation slice may keep the current spec body and section format while redefining what the spec means.
- A spec documents intended properties of the system and the executable or manual acceptance criteria that determine whether those properties are satisfied, blocked, or missed.
- Problem statements, current-state analysis, rollout sequencing, blockers, and implementation notes belong in tickets rather than in the spec contract.
- `spec-api` owns native metadata for expected properties, acceptance clauses, evidence requirements, and fulfillment state.
- `doc-api` owns documentation-validation records, manual verification steps, and explicit documentation coverage gaps.
- A future `test-api` owns validation specifications, executions, and outcomes such as `passed`, `failed`, and `blocked`.
- A future `log-api` owns validation-log capture and retrieval linked to validation executions.
- `audit-api` derives repository-level fulfillment rollups from store-owned metadata rather than becoming the source of truth itself.
- The migration from the current mixed model is performed homogeneously across the affected specs and tickets so contract, plan, current state, and evidence no longer drift between artifacts.

## Assumptions To Prove

- The current spec body and section format can remain in place for the first slice while the contract definition changes underneath it.
- First-class expectation and evidence metadata can be added to `spec-api` without breaking existing authored bodies, sections, or generated spec artifacts.
- Extending `doc-api` and bootstrapping minimal `test-api` and `log-api` models is sufficient to prove one end-to-end fulfillment slice.
- `audit-api` can compute useful derived rollups from store-owned metadata without introducing a second authoritative artifact store.
- One pilot workflow spec plus one pilot README-rollout spec is enough to prove the homogeneous migration mapping before broader adoption proceeds.

## Relationship To Existing Specs

This work builds on the workflow-architecture direction already captured in:

- [workflow validation metadata and default tool behavior](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.spec/specs/a4f48d84-50ed-4769-a42f-38321ea9600c/body.md)
- [workflow documentation validation via doc-api and doc-cli](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.spec/specs/cf5e2942-1a47-43cc-a0ee-14e5774680a6/body.md)
- [cross-store workflow traceability metadata](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.spec/specs/38e337c2-cdda-4488-9aa7-b47a300563b0/body.md)
- [doc-api family plan](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/memory-api/.spec/specs/24baf686-38fd-417d-9528-bebc02a556d0/body.md)
- [spec-api generated documents](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/memory-api/.spec/specs/1cf68c36-7f64-4d81-b553-1947b978fbe3/body.md)

It is intentionally compatible with the current file-oriented spec shell in the first slice; the initial change is to the definition and native metadata contract, not necessarily to the markdown file layout.

## Test Strategy

1. Add failing blackbox tests and authoring guidance checks that define the observable expectation-oriented spec contract.
2. Extend `spec-api` until the store, health, and query behavior can represent the richer contract natively.
3. Add transport parity tests for CLI, MCP, and HTTP surfaces.
4. Extend `doc-api`, bootstrap minimal `test-api` and `log-api` models, and add derived `audit-api` rollups over the resulting metadata.
5. Run a pilot migration on one workflow spec and one README-rollout spec, then run a bounded follow-on tracker over the fixed remaining inventory.

## Acceptance Criteria

- The repository defines specs primarily in terms of expected properties, acceptance clauses, and evidence requirements rather than problem statements and rollout prose.
- The first slice can use the current spec body and section format while enforcing the new contract definition.
- `spec-api` exposes native metadata for expected properties, acceptance clauses, evidence requirements, and fulfillment state.
- `spec-cli`, `spec-mcp`, and `spec-http` expose the richer contract consistently.
- `doc-api`, future `test-api`, and future `log-api` provide store-owned evidence links that can satisfy or block spec acceptance clauses.
- `audit-api` reports derived satisfied, blocked, and missed fulfillment status from authoritative store-owned metadata.
- At least one workflow spec and one README-rollout spec are migrated successfully, and a follow-on tracker applies one documented homogeneous mapping to the broader affected specs and tickets.

## Traceability

- [bc19467f Expectation-oriented spec contract rollout](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/bc19467f-b4d4-48c3-be92-b551d4fe6679/ticket.toml)
- [b744bcf5 Expectation-oriented spec contract and model](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/b744bcf5-05a5-4601-bbe1-caae9d42ea5f/ticket.toml)
- [0b6e1bf3 Define blackbox contract and authoring guidance for expectation-oriented specs](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/0b6e1bf3-2478-40a5-a619-085d8691835a/ticket.toml)
- [c73d4a6b Add native expectation, acceptance, and evidence fields](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/c73d4a6b-2610-4e69-9fc3-bfedcf2ec53d/ticket.toml)
- [c666f0b3 Expose expectation and evidence parity across transports](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/c666f0b3-f1e6-4073-852f-e494bf5c1272/ticket.toml)
- [aaa90ee6 Store-owned spec evidence integration](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/aaa90ee6-1358-41ad-b19e-61abdc3f1dc2/ticket.toml)
- [618f6ce4 Bootstrap doc-api, test-api, and log-api evidence stores for spec fulfillment](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/618f6ce4-e7b3-48f2-9c9e-840247a119da/ticket.toml)
- [87001cb8 Add documentation-validation evidence identities for spec fulfillment](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/87001cb8-46c4-4921-a336-dc0cf0c1f66a/ticket.toml)
- [86bf3da2 Bootstrap validation specification and execution identities for spec fulfillment](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/86bf3da2-b6cc-4fc7-898d-044403283550/ticket.toml)
- [0805fb76 Bootstrap validation-log identities for spec fulfillment](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/0805fb76-f99b-45a5-87c6-5a8e65bdb2da/ticket.toml)
- [635b7e37 Derive spec fulfillment rollups from store-owned evidence](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/635b7e37-8bed-4622-a38d-ef87bb08f46c/ticket.toml)
- [6e5306fb Pilot expectation-oriented spec contract on one workflow spec and one README-rollout spec](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/6e5306fb-c1b3-4aec-991d-fabaf3096e23/ticket.toml)
- [577df498 Homogeneously migrate remaining expectation-oriented specs and tickets](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/577df498-d468-448f-afc1-3e35e48e5f12/ticket.toml)