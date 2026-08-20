# Review: Presentation Automation Planning

## Verdict

**Changes requested before implementation; approved as a bounded roadmap
exercise.** The transcript supplies a clear strategic direction, but it joins
repository-derived evidence, presentation generation, visual language,
workflow explanation, and later cross-language support into one stream. The
next deliverable is therefore a dependency-ordered roadmap under the existing
presentation epic, not a single implementation session.

## Existing-Capability Check

| Existing capability | Evidence | Implication |
| --- | --- | --- |
| Repository-local Slidev decks can compose sub-repository slides. | [`.presentation/README.md`](../../.presentation/README.md) | Preserve the existing authoring and composition model while formalizing generated conceptual decks. |
| The presentation system already has a governing specification and epic. | [Presentation System](../../.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/body.md), [presentation epic](../../.ticket/tickets/0ee95228-475d-4706-a108-fd208f7c4098/description.md) | Reuse the existing plan; do not create a competing presentation architecture. |
| The planned `presentation-api` already owns deck persistence, materialization, builds, and trace links. | [Phase 2 plan](../../.ticket/tickets/3cdcaf3b-d958-44f3-afb2-b17be3484419/description.md) | Keep source extraction and normalization outside `presentation-api`. |
| `spec-api` provides file-backed manifests, sections, indexes, and store patterns. | [`workflow-tools/spec`](../../workflow-tools/spec/) | Treat specifications as the authoritative conceptual source and adapt its conventions rather than inventing a second store pattern. |
| `Peek` provides bounded structural inspection, not a cross-language repository graph. | [Peek ticket](../../.ticket/tickets/06cfe998-c2e1-48a4-83e9-11e85e7c40f4/description.md) | Start with a separately typed Git/Cargo projection contract; do not silently expand `Peek` in the first track. |

## Findings

| Severity | Finding | Required improvement |
| --- | --- | --- |
| High | A specification-derived deck needs reproducible source identity, not only a link to a mutable specification. | Define source locks, claim-level citations, and stale-output behavior before generation work. |
| High | Git containment and Cargo workspace/crate membership or dependency are different graphs. | Define two named, typed projections with source evidence and a visual legend; do not render an unlabeled hierarchy. |
| High | Generator overwrite authority can erase presenter adjustments or escape the deck boundary. | Declare managed output paths, preserve human-owned overlays, reject path escapes, and require an explicit replacement path. |
| Medium | Implementation and documentation may conflict with the authoritative specification. | Define a structured disagreement sidecar and publication behavior for material contradictions. |
| Medium | The transcript's longer-term language and tool-surface ideas would expand the first track beyond a useful validation slice. | Bound the initial track to specifications, Git/submodule topology, Cargo workspaces/crates, declarative workflows, and live human delivery. |
| Medium | Existing title-page screenshot checks do not establish conceptual-deck correctness. | Require static-build, fixed-viewport, per-slide navigation, screenshots, citations, legends, notes, and browser-error checks. |

## Scope Decision

This dossier will contain a review, artifact inventory, three independently
actionable work packages, a completion checklist, and a current roadmap.
The roadmap may reuse or update existing tickets and create new tickets only
where a multi-session work package lacks an existing owner.

The current track includes specification-derived conceptual claims, source
locks, structured disagreement sidecars, separate Git/Cargo projections,
declarative workflow illustrations, deterministic deck generation, and static
human-facing validation.

The current track excludes product-code implementation, cross-language parsing,
live telemetry, documentation/CLI/MCP/test-derived automation, a completed
custom theme pack, and any change that makes telemetry normative.