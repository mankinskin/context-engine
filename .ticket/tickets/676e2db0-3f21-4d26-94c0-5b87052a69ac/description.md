## Problem

The track does not yet define how generated indexes are consumed efficiently together with `peek-cli`, nor how index rendering can expose adjustable levels of detail to manage token cost. Without that validation plan, generators can produce files that are technically correct but inefficient for agent consumption and search.

## Goal

Define the consumption and validation contract for generated store indexes so they support efficient `peek-cli` outlining, bounded inspection, and optional level-of-detail rendering where that improves token cost and usability.

## Scope

- Define how generated index files should be structured so `peek-cli` can outline and inspect them efficiently.
- Define whether README and TOON digest/index outputs expose multiple levels of detail, and if so, where the LOD choice lives.
- Identify the validation scenarios that prove generated outputs are usable for bounded reads, search, and agent consumption.
- Cover at least ticket, spec, rule, audit, and workspace indexes in the validation plan.
- Update generator tickets so efficient consumption and LOD validation are explicit deliverables rather than implicit nice-to-haves.

## Acceptance Criteria

- The track includes an explicit `peek-cli` consumption story for generated index outputs.
- A validation plan exists for bounded outline/search workflows over generated indexes.
- The repository has a reviewed decision on whether LOD rendering is supported, and where it applies.
- Generator tickets are updated so token-cost-sensitive consumption is part of their expected validation evidence.

## Non-goals

- Does not implement `peek-cli` itself.
- Does not implement all generator outputs.
- Does not replace the digest schema or shared sidecar validator.

## Resolved direction carried into this ticket

- Generated indexes should be optimized for efficient agent consumption, not only human readability.