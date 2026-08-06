# Purpose

Define how transcript refinement maintains numbered raw and clean source pairs, then composes those clean artifacts into one maintained merged artifact.

# Requirements

- Every pasted transcript creates a verbatim `input[-N].md` and a matching `input[-N].clean.md`.
- A merge request composes selected clean artifacts into `merged.clean.md` without modifying raw or individual clean sources.
- A fold request first creates the numbered raw/clean pair, then updates `merged.clean.md` from the clean artifact.
- Operational chat instructions are not treated as source transcript content.
- The merged artifact preserves reconciled final intent and retains explicit open questions.