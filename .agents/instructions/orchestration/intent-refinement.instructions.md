---
description: "Use when running an informed review + interview loop inside the prompt-ingestion pipeline: critiquing a cleaned prompt or a drafted dossier against research already gathered, and interviewing the requester only when repository evidence can't resolve the remaining ambiguity or low-confidence item. Used twice — before drafting and again after — so the shipped roadmap never carries an open question."
applyTo: "**/*.md"
---

## Purpose

Denoising a prompt (turning raw words into clean words) is not the same job as refining its intent (turning clean words into a verified, unambiguous decision). This instruction owns the second job: the **informed review + interview loop** that [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md) runs twice — once right after research and the artifact inventory, before anything is drafted, and once right after the dossier and `ROADMAP.md` are drafted, before they ship. Denoising itself is not this file's job: it is delegated entirely to [audio-transcript.instructions.md](../transcripts/audio-transcript.instructions.md) and the [Transcription Agent](../../agents/transcription.agent.md), the same pipeline this whole ingestion process extends.

## The Informed Review + Interview Loop

Never critique or interview from the raw prompt's words alone — every pass in this loop is grounded in whatever research, inventory, or dossier state already exists at that point in the pipeline.

1. **Critique.** Review the current state — the cleaned prompt plus `ARTIFACTS.md` for the first loop; the drafted dossier and `ROADMAP.md` for the second — the way [review.agent.md](../../agents/review.agent.md) critiques an implementation: is the ask bounded, is it verifiable, does it conflate distinct concerns, what does the gathered research already answer.
2. **Verdict.** Produce a verdict (`Changes requested` or `Approved as scoped`), a findings table with severity and required improvement per finding, and an explicit scope decision — or, for the second loop, a plain statement that no open question remains.
3. **Interview only what evidence cannot resolve.** If a finding is a genuine ambiguity — the words are bounded but the underlying goal, priority, or acceptance condition is not, and nothing in the research answers it — dispatch the [Mission Planning Agent](../../agents/mission-planning.agent.md) for a mission-goal-level gap, or the [Interview Agent](../../agents/interview.agent.md) for a narrower requirement/acceptance-criteria gap. Hand the dispatched agent the research already gathered so every question it asks is grounded in a concrete finding, not a guess dressed up as a question.
4. **Repeat until clean.** Re-run the critique against the updated state after answers arrive. Do not close the loop with an unresolved finding still open — carry it to another round rather than downgrading it to a note.

Follow [escalation-gate.instructions.md](escalation-gate.instructions.md) rather than guessing at any point: an ambiguous verdict is a blocked loop, not an approved one.

## Where the Loop Runs

- **First loop** (`prompt-ingestion.instructions.md` Stage 3): runs once research and the artifact inventory exist, before anything is drafted, so both the critique and any interview question are informed by real repository findings instead of the raw prompt alone. See [prompt-ingestion.instructions.md's Six Stages](prompt-ingestion.instructions.md#the-six-stages), Stage 3, for the exit artifact.
- **Second loop** (`prompt-ingestion.instructions.md` Stage 5): runs once the dossier, `ROADMAP.md`, and `README.md` exist in draft, to catch anything the drafting pass surfaced as low-confidence or newly ambiguous. This loop replaces a separate traceability-checklist stage: its job is to make sure `ROADMAP.md` ships with zero open questions — every one gets an interview, not a checklist entry.

## Handoff

Once a loop closes with `Approved as scoped` (or, for the second loop, "no open questions remain"), control returns to [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md) for the next stage. This file never drafts dossier content itself — it only clears the ambiguity gate before and after drafting.

