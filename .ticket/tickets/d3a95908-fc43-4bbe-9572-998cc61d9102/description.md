## Problem

The current track does not define how generated index artifacts will be validated for efficient agent consumption with `peek-cli`. That leaves a major integration question unanswered: whether the generated markdown and TOON outputs can be outlined, searched, and rendered at controlled levels of detail without forcing expensive full-file reads.

## Goal

Define the `peek-cli` integration and level-of-detail validation plan for generated store indexes so generator implementations are reviewed against efficient consumption requirements, not just correctness.

## Scope

- Define the validation scenarios that exercise generated README and TOON outputs through `peek-cli` outlining, grep, windowed reads, and digest inspection.
- Define what level-of-detail controls the index rendering should support, such as compact digest views, section-only summaries, or expandable detail tiers.
- Define how LOD rendering interacts with digest stability and diff stability.
- Identify the minimum cross-domain validation matrix for ticket, spec, rule, audit, and workspace outputs.
- Update generator tickets with the required validation evidence for agent-efficient consumption.

## Acceptance Criteria

- The track contains an explicit validation plan for `peek-cli` consumption of generated index artifacts.
- Required LOD surfaces or rendering modes are named clearly enough for generator implementers.
- Validation expectations cover both human-readable markdown and TOON sidecar outputs.
- Generator tickets reference the same validation expectations instead of inventing domain-specific checks.

## Non-goals

- Does not implement LOD rendering yet.
- Does not benchmark runtime performance.
- Does not replace the generic digest schema.