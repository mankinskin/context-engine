---
description: "Transform a raw audio-transcript file (or a folder of transcripts) into a clean, English, well-structured markdown artifact with all verbal noise removed and intent preserved."
name: "transform-transcript"
argument-hint: "<path to a transcript file or a folder containing the raw transcript>"
agent: "Transcription Agent"
---

# Transform Transcript

Turn a noisy raw audio transcript into one coherent, concise, grammatically correct **English** markdown artifact that faithfully captures the speaker's final intent — nothing invented, nothing meaningful lost.

Follow [audio-transcript.instructions.md](../instructions/audio-transcript.instructions.md) as the authoritative process and the [Transcription Agent](../agents/transcription.agent.md) contract.

## Input Resolution

1. Treat the slash-command text as a path to either:
   - a single transcript file (for example a `*.transcript.md`, `*.audio.md`, or any raw transcript text file), or
   - a folder that contains the raw transcript. When given a folder, locate the raw transcript file inside it (prefer an obvious raw/source transcript; if multiple candidates exist and the choice is material, ask one short clarifying question).
2. Read the raw transcript. Note any requested target format (default: a single concise prompt/document).

## Workflow

Run the three-stage pipeline as distinct passes. Do not collapse them.

1. **Stage 1 — Denoise (to English).**
   - If any part of the transcript is not in English, translate it to English, then compare the English rendering against the original to confirm the meaning is preserved and the translation is correct.
   - In the same stage, strip filler and false starts, resolve every self-correction to the speaker's final choice, apply corrected terminology consistently, and fix obvious mis-transcriptions (flag any that context cannot resolve).
   - Produce a faithful, still-linear, fully English denoised signal. Do not restructure yet.
2. **Stage 2 — Restructure.** Reshape the denoised English signal into the intended markdown format (concise prose, ordered/bulleted lists, or sections the speaker implied). Correct grammar and merge redundant restatements without adding scaffolding the speaker did not intend.
3. **Stage 3 — Verify.** Run the checklist: constraint inventory, no-new-information check, correction integrity, translation fidelity (output is fully English and meaning-preserving), and intent equivalence. Fix and re-verify any discrepancy; surface anything unresolved as a short "Open questions" note.

## Output

- Write the final clean English transcript to an output file.
  - For a single input file, save the result next to the source with a clarified name (for example append `.clean.md` or replace a `.raw`/`.transcript` marker with a clean-output marker) without overwriting the raw source.
  - For a folder input, write the output file into that same folder alongside the raw transcript.
- Report:
  - the resolved input path and the written output path
  - what was removed as noise versus preserved as intent
  - source language(s) detected and translated, if any
  - any flagged ambiguities or open questions

Do not implement any code changes described in the transcript — the deliverable is the refined English artifact only, unless the user explicitly asks to act on it afterward.
