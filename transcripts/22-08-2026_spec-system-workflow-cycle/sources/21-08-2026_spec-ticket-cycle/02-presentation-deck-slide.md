# 02 — Add the Cycle to the Presentation Deck

## Outcome

The root `.presentation/` ("context-engine") deck gains a slide or section presenting the closed-loop cycle (request → spec → tickets → tests → implementation → validated response → next iteration) as the outline of the repository's complete production workflow.

## Description

Per the transcript: "I also want us to include this in our presentation as the outline of our complete cycle. All the components I mentioned already have initial implementations, and we want to implement this loop." Confirmed via `ARTIFACTS.md` that `.presentation/` (repo root, `id = "context-engine"`, composes `workflow-tools`) is the correct, repo-wide deck for this — not `workflow-tools/.presentation/`, which is a sub-deck for a narrower scope.

The slide should visually mirror the 7-step cycle from work package 01, and should note (per the transcript) that most components already have initial implementations — this is a maturity/status statement, not a proposal for net-new capability, except where work package 03 below flags a real gap.

## Non-Goal

Do not redesign the deck's existing structure, theming, or unrelated slides. Do not duplicate `01`'s full instruction-file prose verbatim in the slide — the slide is a visual/summary artifact, not a second copy of the instruction file.

## Validation Method

Manual/visual: build the deck (`npm run dev` / `npm run build` per `.presentation/README.md`) and confirm the new slide renders and reads correctly. No automated test applies.
