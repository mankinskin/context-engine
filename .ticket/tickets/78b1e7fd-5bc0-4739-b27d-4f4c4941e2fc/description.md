## Problem

The memory-index plan currently treats generated index files as a separate rendering path, but there is no integration plan with the existing `rule-api` generation pipeline. That would introduce multiple file-rendering paradigms side-by-side and make future generated ticket/spec content harder to unify.

## Goal

Define how store-index generation integrates with the repository's shared rendering pipeline so generated files use one coherent rendering model that can later support index digests, ticket/spec derived content, and other generated documentation surfaces.

## Scope

- Decide whether store-index generators should reuse the existing `rule-api` generator pipeline directly or extract a shared rendering layer that both systems use.
- Define the rendering boundary between domain data collection, generic rendering, and file output.
- Identify the extension points needed for future generated content such as ticket descriptions, spec bodies, dependency summaries, section digests, and acceptance-criteria renderers.
- Update the generator tickets so they depend on the chosen shared rendering plan rather than inventing their own file-rendering flow.
- Record any constraints on output stability, diff quality, and generated-file ownership that the shared pipeline must preserve.

## Acceptance Criteria

- The ticket track names one shared rendering strategy for generated store files.
- The relationship between memory-index generation and `rule-api` generation is explicit and reviewable.
- Future generated file surfaces in tickets/specs have a defined extension path instead of requiring a second rendering paradigm.
- Generator tickets are updated to point at the shared rendering plan.

## Non-goals

- Does not implement the shared renderer.
- Does not redesign current rule content semantics.
- Does not define per-domain digest inputs or git-hook wiring.

## Resolved direction carried into this ticket

- Memory-index generation should not introduce an isolated rendering stack when the repository already has generator infrastructure that may be reusable or extractable.