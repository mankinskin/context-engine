---
description: "Use when transforming a raw audio transcript into a clean, well-formed markdown prompt or structured document. Covers the denoise → restructure → verify pipeline, common transcription problems, and loss-free intent preservation."
---

## Scope

Applies when converting text that was transcribed from spoken audio into a coherent, grammatically correct, well-structured markdown artifact — typically a single clear prompt, but also structured notes, bulleted requirements, or a specification draft when that is what the speaker intended.

The input is noisy: it contains filler words, false starts, mid-sentence corrections, superseded terminology, repetition, thinking-aloud tangents, and outright mis-transcriptions. The output must be one concise, coherent signal that faithfully captures the speaker's final intent — nothing added, nothing meaningful lost.

The raw transcript can arrive two ways: as a path to an existing file or folder, or as the raw transcribed text pasted directly into the request. When it arrives as raw text, create the dated `transcripts/DD-MM-YYYY_<slug>/` folder (matching the existing convention), save the verbatim text as `input.md`, and treat that file as the source — the pipeline below is identical either way.

**Resolve `transcripts/` at the active VS Code workspace root** — the top-level folder of the workspace the agent is currently operating in (e.g. `meta-workspace/transcripts/`) — never relative to a nested submodule such as `context-engine/`, nor relative to the location of this instruction file itself.

## Core Principle: Faithful Compression, Not Interpretation

The transform is **lossless in intent and lossy only in noise**. Two failure modes are equally bad:

- **Information loss** — dropping a real constraint, entity, requirement, or nuance the speaker meant to convey.
- **Hallucinated addition** — introducing structure, detail, scope, or interpretation the speaker never expressed.

When intent is genuinely ambiguous, do **not** guess. Preserve the ambiguity explicitly (inline note or a short "Open questions" section) rather than silently resolving it.

## Required Pipeline

Process the transcript in three ordered stages. Do not collapse them into a single pass — the separation is what prevents noisy input from corrupting structured output.

### Stage 1 — Denoise (extract the clean signal)

Goal: produce a faithful, still-linear **English** version of the transcript with obvious noise removed and corrections resolved. **Do not restructure or reformat yet.** Preserve the speaker's ordering and phrasing except where noise forces a change.

Resolve language to English first:

- **Translate non-English content to English.** If any part of the transcript is in a language other than English, translate it into English as part of this stage so the entire denoised signal is in English. Mixed-language (code-switched) input must be unified into a single English output.
- **Verify the translation is meaning-consistent.** After translating, compare the English rendering against the original wording and confirm the meaning is preserved — no constraint, entity, qualifier, or nuance changed or lost in translation. Do the verbal-noise stripping, false-start removal, and self-correction resolution described below on the English result (translation and denoising happen together in this stage, before any restructuring).
- **Preserve untranslatable tokens verbatim.** Identifiers, file paths, command names, proper nouns, numbers, and code carry over unchanged; only natural-language content is translated.
- **Flag ambiguous translations.** If a phrase has no unambiguous English equivalent or the intended meaning is genuinely unclear, keep the most likely reading and flag it — never invent a plausible-sounding replacement.

Remove or resolve:

- **Filler and disfluency**: "um", "uh", "like", "you know", "basically", "sort of", "I mean", stutters, and repeated words.
- **False starts and restarts**: abandoned sentence fragments where the speaker restarts a thought.
- **Self-corrections**: when the speaker says one thing then corrects it ("do X — no wait, do Y", "use the foo file, sorry, the bar file"), keep **only the corrected final intent** (Y / bar) and discard the superseded version.
- **Superseded terminology**: if the speaker uses a wrong name/term and later corrects it, apply the corrected term **consistently everywhere**, including earlier mentions.
- **Obvious mis-transcriptions**: fix homophones and garbled tokens using surrounding context (e.g. "their/there", a mangled technical term). If a token is ambiguous and context does not resolve it, keep the most likely reading and flag it — never invent a plausible-sounding replacement.
- **Verbal punctuation and meta-noise**: spoken "new paragraph", "in quotes", "period", transcription artifacts, timestamps, and speaker labels that are not content.

Keep at this stage:

- Every distinct instruction, constraint, requirement, entity name, and qualifier.
- Exploratory reasoning **only if** the speaker signals it is part of the intended message; drop pure thinking-aloud tangents that were abandoned.

The Stage 1 output is a "one clear signal" — a readable, faithful, minimally cleaned, **English** rendering.

### Stage 2 — Restructure (shape to target format)

Goal: reshape the denoised signal into the format the speaker intended — usually clean markdown: a concise prompt, headings, and/or bulleted lists.

- Apply the structure the speaker **implied**: if they enumerated "first… then… also…", render an ordered or bulleted list; if they described distinct requests, separate them into distinct items.
- Correct grammar, tense, and sentence flow so the result reads as deliberate prose, not a transcript.
- Merge redundant restatements of the same point into one clear statement.
- Prefer the tightest phrasing that still carries every constraint. Concision is a goal, but never at the cost of a real requirement.
- Do **not** introduce headings, sections, or scaffolding that imply structure the speaker did not intend. Match the intended shape, not a generic template.
- Preserve technical precision exactly: identifiers, file paths, numbers, and names carry over verbatim from Stage 1.

### Stage 3 — Verify (compare against the source)

Goal: prove the transform is loss-free in intent and free of additions. Verify **each stage's output against its input**, and the final output against the original transcript.

Perform an explicit checklist:

1. **Constraint inventory** — enumerate every atomic instruction, constraint, entity, requirement, and qualifier present in the original transcript. Confirm each one is represented in the final output (or intentionally dropped only because it was a superseded/self-corrected statement).
2. **No-new-information check** — scan the final output and confirm every claim, item, and detail is traceable back to the input. Remove anything that is not.
3. **Correction integrity** — confirm every self-correction resolved to the speaker's final choice, and superseded terms/values do not leak into the output.
4. **Translation fidelity** — if the source contained non-English content, confirm the final output is entirely in English and that every translated statement preserves the original meaning, with no drift, loss, or invented detail introduced by translation.
5. **Intent equivalence** — read the output as if you were the downstream agent receiving the prompt, and confirm it would act on the speaker's actual goal.

If any check fails, return to the responsible stage, fix it, and re-verify. Do not ship an output with an unresolved discrepancy; instead surface it as an explicit "Open questions" note.

## Multi-Transcript Composition

When a speaker provides a larger topic across multiple raw transcripts, preserve the individual source artifacts and compose only from verified clean artifacts.

1. **Clean each transcript first.** Every raw pasted transcript gets its own verbatim `input.md`, `input-2.md`, and so on, plus the matching `input.clean.md`, `input-2.clean.md`, and so on. Do not merge raw transcripts.
2. **Merge on request.** When asked to merge selected transcripts, create or update `merged.clean.md` in the same folder. Reconcile duplicate statements and later self-corrections across the selected clean artifacts, retaining the speaker's final intent. Preserve the individual raw and clean files unchanged.
3. **Fold on request.** When asked to fold a later transcript into a merged artifact, first create and verify the next numbered raw/clean pair. Then update `merged.clean.md` from that clean artifact, retaining all earlier compatible requirements and replacing only statements explicitly superseded by the newer transcript.
4. **Keep operational instructions separate.** Requests such as "merge these transcripts" or "create source files" are workflow instructions, not transcript content. Do not create a raw/clean pair from those instructions unless the user explicitly identifies the instruction text as a transcript.

The merged artifact is a maintained composition, not a substitute for the source artifacts. Its purpose is to make repeated refinement of a long, evolving spoken design discussion manageable while keeping every original input and its independently verified clean rendering available.

## Common Problem Situations (handle explicitly)

- **Layered self-correction**: multiple corrections chained ("X, actually Y, well really Z"). Resolve to the last value only.
- **Mid-thought scope creep**: the speaker adds then retracts a requirement. Track retractions; a later "actually never mind that" removes the earlier item.
- **Ambiguous referents**: "that file", "the thing we discussed", "it". Resolve from context when unambiguous; otherwise keep the speaker's wording and flag it rather than binding it to a specific entity.
- **Multiple distinct asks in one ramble**: split into separate, clearly delimited items so none is buried or merged.
- **Loose enumerations**: spoken lists ("do this, and this, and also this") — render as a clean list, preserving count and order.
- **Emphasis vs. content**: repetition used for emphasis is not new information; collapse it but retain the emphasis only if it changes priority.
- **Meta-instructions about the transcript itself**: if the speaker gives directions about how to process the transcript ("make this a bulleted list", "keep it short"), treat those as formatting directives for Stage 2, not as output content.

## Output Requirements

- Well-formed, grammatically correct markdown.
- One coherent, concise artifact reflecting the speaker's intended structure.
- Entirely in English, even when the source transcript was partly or wholly in another language.
- No filler, no corrections, no transcription artifacts remaining.
- Every real requirement preserved; nothing invented.
- Any unresolved ambiguity surfaced as an explicit, short note — never silently guessed.

## Anti-Patterns

- Doing all three stages in one pass and letting noise bleed into structure.
- "Improving" the request by adding scope, rationale, or steps the speaker never stated.
- Dropping a constraint because it was phrased awkwardly.
- Resolving an ambiguous mis-transcription by inventing a confident-sounding term.
- Keeping both sides of a self-correction "to be safe".
