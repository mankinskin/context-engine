Parent epic: `0ee95228`. Spec: `2ccde9ee` (new R15/AC16 — see spec update in this same pass).

## Scope

Every deck and slide declares a position on a shared information-architecture ladder (e.g. audience-problem, workflow, tool-role, domain-contract, component, implementation-detail), its prerequisite levels, and optional drill-down targets to a more detailed nested deck. Composed decks (per `.presentation/README.md`'s `composes` mechanism) must share this level vocabulary so a super-repository deck and an imported sub-repository deck stay coherent when combined. Authors declare levels explicitly; no automatic level inference.

## Definition of done

- Deck/slide manifest schema carries a declared level, prerequisites, and optional drill-down target.
- The existing `context-engine` and `workflow-tools` sample decks are annotated with levels as a worked example.
- Composing decks with mismatched or missing levels produces an explicit diagnostic, not silent omission.