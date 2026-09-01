---
description: "Transform a raw audio-transcript file, folder, or the raw transcribed text pasted directly into a clean, well-structured markdown artifact (English by default, or another language if explicitly requested) with all verbal noise removed and intent preserved."
name: "transform-transcript"
argument-hint: "<path to a transcript file or folder, or the raw transcribed text itself>"
agent: "Transcription Agent"
---

# Transform Transcript

Turn a noisy raw audio transcript into one coherent, concise, grammatically correct markdown artifact — written in the resolved target output language (English by default, unless another language was explicitly requested) — that faithfully captures the speaker's final intent — nothing invented, nothing meaningful lost.

Follow [audio-transcript.instructions.md](../instructions/transcripts/audio-transcript.instructions.md) as the authoritative process and the [Transcription Agent](../agents/transcription.agent.md) contract.

## Input Resolution

Resolve the slash-command argument using [Transcription Agent's Input Modes](../agents/transcription.agent.md#input-modes): a path to an existing file/folder, or raw transcribed text to save as a new dated `transcripts/DD-MM-YYYY_<slug>/input.md` at the active VS Code workspace root (not inside a nested submodule such as `context-engine/`). Note any requested target format (default: a single concise prompt/document). A request to merge or fold transcripts is an operation on existing clean artifacts, not raw transcript content — do not save it as an `input-N.md` file.

## Output Language Resolution

Before running the pipeline, resolve the target output language per [audio-transcript.instructions.md's Resolve Target Output Language](../instructions/transcripts/audio-transcript.instructions.md#resolve-target-output-language-before-stage-1): default to English; use a different language only when the transcript or the surrounding request explicitly asks for one (e.g. "auf Deutsch", "in German", "en español"). Ask one short clarifying question if the requested language is unclear rather than guessing.

## Workflow

Run the three-stage pipeline in [audio-transcript.instructions.md's Required Pipeline](../instructions/transcripts/audio-transcript.instructions.md#required-pipeline) — Denoise, Restructure, Verify — as distinct passes. Do not collapse them.

## Multi-Transcript Refinement

For a discussion spanning multiple transcripts, follow [audio-transcript.instructions.md's Multi-Transcript Composition](../instructions/transcripts/audio-transcript.instructions.md#multi-transcript-composition): a distinct, verified numbered raw/clean pair per transcript; `merged.clean.md` created or updated on request from the selected clean files; a fold first verifies the next pair, then updates `merged.clean.md` from it without discarding earlier compatible content.

## Output

Meet the Output Requirements in [audio-transcript.instructions.md](../instructions/transcripts/audio-transcript.instructions.md#output-requirements), including writing the output entirely in the resolved target output language. Additionally:

- Write the final clean transcript to an output file: next to the source with a clarified name for a file/folder input, or as `input.clean.md` (or the matching `input-N.clean.md`) inside the new `transcripts/DD-MM-YYYY_<slug>/` folder for raw pasted text.
- Report the resolved input path (or the newly created folder and raw/clean file paths), the resolved target output language (only if not the English default), the written output path, source language(s) translated if any, and any flagged ambiguities or open questions.

Do not implement any code changes described in the transcript — the deliverable is the refined English artifact only, unless the user explicitly asks to act on it afterward.
