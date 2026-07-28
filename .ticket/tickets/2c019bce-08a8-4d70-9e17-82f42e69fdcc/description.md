## Problem

Multiple orchestrated agents created the same tickets in sequence: an orchestrator reviewed an existing ticket track, then a dispatched sub-agent duplicated many of the same tickets one-to-one. Two concrete duplicate pairs were confirmed in-store and cancelled during triage (e42d8e0a, 2bb8b3e1).

The rule already exists. .agents/instructions/ticket/workflow.instructions.md#L61-L72 "Discovery Before Creating" says: "Always search for existing tickets before creating new ones. Duplicate tickets degrade store quality." But it reaches agents only through a path-scoped instruction file, and of 16 agent templates only `research`, `spec`, and `ticket-refinement` restate it. `default` and `implement` — both able to create tickets — do not.

## Decisions (interview-resolved)

- Make search-before-create an **explicit pre-create workflow gate step**, not merely injected advisory prose.
- The default posture for every requested change is a **delta against current state** — for tickets, specs, or any other entity. Create a new entity only when genuinely necessary.
- Apply the gate to **entity-creating roles only**. Read-only / verdict-only roles (explore, roast, review, testing, transcription) are exempt by design.
- Do NOT add a server-side fuzzy duplicate-title warning — it would produce too many false positives. The workflow gate is the mechanism.

## Trigger point

The observed failure was a sub-agent dispatched immediately after a ticket-track review. The gate must fire at that boundary in particular.