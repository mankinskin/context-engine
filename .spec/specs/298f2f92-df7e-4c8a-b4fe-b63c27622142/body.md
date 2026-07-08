<!-- aligned-structure:v1 -->

# Summary

Define the repository's default specification contract around the behavior a component promises, not around workflow narration. A spec is the compact system anchor for intended behavior, consumer-visible contracts, and the validation required to keep those contracts true.

## Behavior Story

Define the repository's default specification contract around the behavior a component promises, not around workflow narration. A spec is the compact system anchor for intended behavior, consumer-visible contracts, and the validation required to keep those contracts true.

## Provided Surface Contracts

- The spec store anchors desired component behavior as a compact contract instead of a workflow-heavy narrative.
- Each spec states the provided surface contracts that consumers may rely on.
- New behavior is described through required validation steps in multiple compatible forms: executable checks, natural-language contract clauses, and code or schema references when available.
- Specs prefer entity references and context rendering over embedding fully expanded entity contents.
- CLI, MCP, HTTP, and viewer surfaces render or expand the same stored contract instead of redefining it.

## Required Validation

- Contract clause validation: The canonical contract defines a spec in terms of a short behavior story, provided surface contracts, executable validation requirements, and related implementation tickets.
- Contract clause validation: Workflow instructions, rollout notes, blockers, and housekeeping guidance are kept out of the spec contract itself unless they directly change the promised behavior.
- Contract clause validation: Guidance for new features requires validation triangulation across executable checks, natural-language contract text, and code or schema references when those references exist.
- Contract clause validation: Specs rely on entity references and context rendering for expandable background knowledge instead of duplicating full entity payloads in the spec body.
- Contract clause validation: All tool surfaces treat the stored spec as the canonical contract and only adapt or render it for their transport.
- The canonical spec body and the guidance surfaces describe the same behavior-first spec contract and responsibilities.
- Focused validation confirms the updated spec entry and guidance surfaces remain structurally sound after the overhaul.
- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- c:/Users/linus/git/graph_app/context-engine/.ticket/tickets/37d7fac3-cc7d-44b9-b6e1-f199fca8e901/ticket.toml
- ticket id: 37d7fac3-cc7d-44b9-b6e1-f199fca8e901

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# Summary

Define the repository's default specification contract around the behavior a component promises, not around workflow narration. A spec is the compact system anchor for intended behavior, consumer-visible contracts, and the validation required to keep those contracts true.

## Required Shape

Each spec encapsulates:

- the short story of the behavior
- the provided surface contracts that consumers can rely on
- the required validation steps and executable tests or checks that prove those contracts
- the related implementation tickets

Specs may link entities and background knowledge by reference. They do not need to inline fully expanded entity contents when context rendering can resolve the latest database state.

## Authoring Rules

- Keep rollout plans, migration notes, blockers, operator checklists, and housekeeping instructions in tickets or workflow guidance unless they directly change the promised behavior.
- When describing a new feature, triangulate the target behavior in as many compatible forms as possible:
  - executable validation steps and commands
  - natural-language contract clauses
  - code, schema, API, or other structural references when available
- Keep the spec short enough to review as a contract. Link outward for implementation detail, evidence history, and background knowledge.

## Provided Contracts

- Consumers can treat the spec store as the canonical anchor for desired behavior.
- CLI, MCP, HTTP, and viewer surfaces may render or expand the contract, but they do not redefine it.
- Entity references plus the latest database state are authoritative for expanding supporting detail.

## Validation Expectations

- Every contract clause should map to at least one required validation step when feasible.
- Validation may mix executable tests, focused manual checks, and schema or code reference checks, but the spec must say what proves the contract.
- Missing or failing evidence should be representable without rewriting the contract itself.

## Related Work

- Related implementation and follow-up work lives in linked tickets.
- Housekeeping facilities belong to the spec system and validation stores, not to the narrative content of each spec.
