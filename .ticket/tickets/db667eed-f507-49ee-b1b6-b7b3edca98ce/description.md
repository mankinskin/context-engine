## Problem

The track currently treats generated index rendering as an isolated effort. There is no plan for how store-index generation should integrate with the existing `rule-api` rendering pipeline or with future structured-content rendering for tickets and specs. If each generator invents its own file rendering path, the repository will end up with multiple rendering paradigms side-by-side.

## Goal

Define the shared rendering-pipeline plan for generated store indexes so index generation aligns with existing and future file-rendering infrastructure instead of fragmenting it.

## Scope

- Review the existing `rule-api` generator/rendering pipeline and identify which parts should be reused directly, wrapped, or generalized for store-index generation.
- Define whether README/index rendering for ticket/spec/rule/audit/workspace indexes should route through shared rendering helpers, templates, manifests, or another common abstraction.
- Define how future structured rendering needs in tickets or specs can reuse the same path for descriptions, bodies, acceptance criteria summaries, or related generated artifacts.
- Record the compatibility constraints so initial generator tickets do not introduce a parallel rendering system that must later be migrated.
- Update generator tickets with the chosen rendering integration requirement.

## Acceptance Criteria

- The ticket track names one rendering paradigm for generated index files instead of per-domain ad hoc approaches.
- The relationship between store-index generation and the existing `rule-api` rendering pipeline is explicit.
- Follow-on generator tickets have a concrete integration target for rendering.
- The plan leaves room for future ticket/spec structured rendering without redesigning the first generator slice.

## Non-goals

- Does not implement rendering integration yet.
- Does not change rule entry semantics.
- Does not decide digest normalization rules.