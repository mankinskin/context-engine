---
description: "Transform a raw audio-transcript file, folder, or the raw transcribed text pasted directly into a clean, English, well-structured markdown artifact with all verbal noise removed and intent preserved."
name: "transform-transcript"
argument-hint: "<path to a transcript file or folder, or the raw transcribed text itself>"
agent: "Transcription Agent"
---

# Transform Transcript

Turn a noisy raw audio transcript into one coherent, concise, grammatically correct **English** markdown artifact that faithfully captures the speaker's final intent — nothing invented, nothing meaningful lost.

Follow [audio-transcript.instructions.md](../instructions/transcripts/audio-transcript.instructions.md) as the authoritative process and the [Transcription Agent](../agents/transcription.agent.md) contract.

## Input Resolution

1. Treat the slash-command text as one of:
   - a path to a single transcript file (for example a `*.transcript.md`, `*.audio.md`, or any raw transcript text file),
   - a folder that contains the raw transcript, or
   - the raw transcribed text itself, pasted directly instead of a path.
2. For a file or folder path, locate and read the raw transcript in place (prefer an obvious raw/source transcript inside a folder; if multiple candidates exist and the choice is material, ask one short clarifying question).
3. For raw pasted text, create a new dated folder under `transcripts/` at the repo root following the existing convention — `transcripts/DD-MM-YYYY_<short-kebab-slug>/` (add `-HHMMSS` only if that date+slug folder already exists) — and write the text verbatim into `input.md` inside it (`input-2.md`, ... if `input.md` already exists there from an earlier run today). Treat that new file as the raw transcript for the rest of the workflow.
4. Note any requested target format (default: a single concise prompt/document).
5. A request to merge or fold transcripts is an operation on existing clean artifacts, not raw transcript content. Do not save operational request text as an `input-N.md` file.

## Workflow

Run the three-stage pipeline as distinct passes. Do not collapse them.

1. **Stage 1 — Denoise (to English).**
   - If any part of the transcript is not in English, translate it to English, then compare the English rendering against the original to confirm the meaning is preserved and the translation is correct.
   - In the same stage, strip filler and false starts, resolve every self-correction to the speaker's final choice, apply corrected terminology consistently, and fix obvious mis-transcriptions (flag any that context cannot resolve).
   - Produce a faithful, still-linear, fully English denoised signal. Do not restructure yet.
2. **Stage 2 — Restructure.** Reshape the denoised English signal into the intended markdown format (concise prose, ordered/bulleted lists, or sections the speaker implied). Correct grammar and merge redundant restatements without adding scaffolding the speaker did not intend.
3. **Stage 3 — Verify.** Run the checklist: constraint inventory, no-new-information check, correction integrity, translation fidelity (output is fully English and meaning-preserving), and intent equivalence. Fix and re-verify any discrepancy; surface anything unresolved as a short "Open questions" note.

## Multi-Transcript Refinement

For a larger discussion captured across multiple transcripts, use a durable source-to-composition workflow:

1. Create and verify a distinct numbered raw/clean pair for every pasted transcript.
2. When asked to merge clean transcripts, create or update `merged.clean.md` in the same folder from the selected clean files. Preserve every raw and individual clean file.
3. When asked to fold a new transcript into the merged artifact, first create and verify the numbered raw/clean pair, then update `merged.clean.md` from that clean file. Keep compatible earlier content and apply a later transcript only where the speaker explicitly changes or supersedes earlier intent.
4. Treat `merged.clean.md` as the maintained concise view of the evolving discussion, not as a replacement for its raw or individually cleaned sources.

## Output

- Write the final clean English transcript to an output file.
  - For a single input file, save the result next to the source with a clarified name (for example append `.clean.md` or replace a `.raw`/`.transcript` marker with a clean-output marker) without overwriting the raw source.
  - For a folder input, write the output file into that same folder alongside the raw transcript.
  - For raw pasted text, write the output as `input.clean.md` (or the matching `input-N.clean.md`) inside the new `transcripts/DD-MM-YYYY_<slug>/` folder created during input resolution.
- Report:
  - the resolved input path (or the newly created folder and raw/clean file paths, for pasted text) and the written output path
  - what was removed as noise versus preserved as intent
  - source language(s) detected and translated, if any
  - any flagged ambiguities or open questions

Do not implement any code changes described in the transcript — the deliverable is the refined English artifact only, unless the user explicitly asks to act on it afterward.
