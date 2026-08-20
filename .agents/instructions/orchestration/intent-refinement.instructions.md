---
description: "Use when a raw prompt's actual intent is unclear before it enters the prompt-ingestion pipeline. Deep-dive on denoising an unstructured prompt and running the review gate: producing a verdict, findings table, and explicit scope decision that pins down what the prompt is really asking for."
applyTo: "**/*.md"
---

## Purpose

A prompt's literal words and its carried intent are not the same thing — filler, self-correction, translation noise, and conflated concerns all obscure what the requester actually wants. This instruction owns turning a raw prompt into a pinned-down, verifiable intent statement before [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md) inventories artifacts and compiles a roadmap against it. Treat these two stages as the intent-refinement half of the pipeline; the remaining stages are the ingestion shell.

## Stage 1: Denoise (cheap)

Dispatch the [Transcription Agent](../../agents/transcription.agent.md) per [audio-transcript.instructions.md](../transcripts/audio-transcript.instructions.md), even when the source was typed rather than spoken — the same denoise/restructure/verify pipeline removes filler, resolves self-corrections, and translates non-English input without adding or dropping content.

Output: `input.md` (raw, verbatim) and `input.clean.md` (denoised) in a dated `transcripts/DD-MM-YYYY_<slug>/` folder.

## Stage 2: Review Gate

Critique the cleaned prompt the way [review.agent.md](../../agents/review.agent.md) critiques an implementation: is the ask bounded, is it verifiable, does it conflate distinct concerns, what existing repository capability already answers part of it?

Produce:

- a verdict: `Changes requested` or `Approved as scoped`
- a findings table with severity and required improvement per finding
- an explicit scope decision listing what the eventual dossier will and will not contain

Output: `REVIEW.md`.

## When the Intent Stays Ambiguous

If the review gate cannot resolve ambiguity through repository evidence alone — the words are bounded but the underlying goal is not — do not record a best-guess scope decision. Dispatch the [Mission Planning Agent](../../agents/mission-planning.agent.md) to interview the requester directly about where the project is meant to go, or the [Interview Agent](../../agents/interview.agent.md) when the gap is a narrower requirement or acceptance-criteria question rather than the overall mission. Feed the resulting mission statement or recorded decision back into the scope decision before Stage 2 output is considered final.

Follow [escalation-gate.instructions.md](escalation-gate.instructions.md) rather than guessing: an ambiguous scope decision is a blocked review gate, not an approved one.

## Handoff

Once `REVIEW.md` carries an `Approved as scoped` verdict, [prompt-ingestion.instructions.md](prompt-ingestion.instructions.md) takes over for artifact inventory, research-informed restructure, traceability, and roadmap compilation. This file does not perform those stages.
