# Duplicate Passages

One row per marked-section finding (both files' line ranges, classification, verbatim excerpt for exact/near duplicates). Populated by merging batch-worker results, phase by phase.

| Batch | File A | A lines | File B | B lines | Classification | Excerpt / Note |
|---|---|---|---|---|---|---|
| A1a | mission-planning.agent.md | 23 | ticket-refinement.agent.md | 24 | near-duplicate | "Do not implement code or edit tickets/specs beyond recording the mission statement and its supporting facts." vs "Do not implement code changes unless explicitly asked." |
| A1a | mission-planning.agent.md | 23 | transcription.agent.md | 58 | near-duplicate | Same "do not implement code" guardrail restated per-agent. |
| A1a | mission-planning.agent.md | 12 | intent-refinement.instructions.md | 16 | thematic overlap | Both describe the informed review + interview loop and its dispatch of Mission Planning Agent for mission-goal-level gaps. |
| A1a | mission-planning.agent.md | 12 | prompt-ingestion.instructions.md | 12 | thematic overlap | Both place the loop (owned by intent-refinement) inside the pipeline and Mission Planning's role in it. |
| A1a | mission-planning.agent.md | 26 | board.instructions.md | 48 | thematic overlap | Both treat unresolved conflicts as an escalation, not something to work around. |
| A1a | mission-planning.agent.md | 12-16 | engine.instructions.md | n/a | no overlap | No meaningful overlap between interview/handoff rules and engine-level ticket-system constraints. |
| A1a | mission-planning.agent.md | 26 | lifecycle.instructions.md | 128-131 | thematic overlap | Both invoke a gate concept before progressing (escalation-gate vs. Review Gate Before Closing). |
| A1b | mission-planning.agent.md | 21-26 | workflow.instructions.md | 218-246 | thematic overlap | Both discuss ticket/workflow boundaries for when ticket/spec edits vs. code changes occur. |
| A1b | mission-planning.agent.md | 30-34 | audio-transcript.instructions.md | 75-90 | thematic overlap | Both require verifying intent against source/research rather than inventing. |
| A1b | mission-planning.agent.md | 23 | refine-ingest.prompt.md | 31 | near-duplicate | "Do not implement code or edit tickets/specs..." vs "Do not create a ticket, create or edit a spec, or change any store/workflow state during this pipeline...". |
| A1b | mission-planning.agent.md | 19 | refine-ingest.prompt.md | 23 | near-duplicate | Guide-not-execute handoff to Mission Planning/Interview Agent restated near-verbatim in refine-ingest Stage 3. |
| A1b | mission-planning.agent.md | 30-34 | ticket-next.prompt.md | 16-26 | thematic overlap | Both cover flow from a scoped goal to selecting/scoping the next actionable ticket slice. |
| A1b | mission-planning.agent.md | 23 | tickets.prompt.md | 34 | near-duplicate | "Do not implement code..." guardrail restated as "Do not implement code or change unrelated tickets, specs, or dependencies unless the user explicitly asks." |
| A1b | mission-planning.agent.md | 30-34 | transform-transcript.prompt.md | 27-34 | thematic overlap | Both require a staged pipeline with fidelity-to-source verification. |
| A2a | ticket-refinement.agent.md | 31-60 | transcription.agent.md | 1-80 | no overlap | Ticket-refinement workflow vs. transcript denoise pipeline — no meaningful similarity. |
| A2a | ticket-refinement.agent.md | 33-44 | intent-refinement.instructions.md | 10-17 | thematic overlap | Both describe critique → verdict → interview-only-what-evidence-cannot-resolve. |
| A2a | ticket-refinement.agent.md | 33-44 | prompt-ingestion.instructions.md | 18-22 | thematic overlap | Both require research-first before drafting/interviewing. |
| A2a | ticket-refinement.agent.md | 29 | board.instructions.md | 10-26 | thematic overlap | Auditable-update requirement matches board's check-in/check-out `--reason`/heartbeat traceability. |
| A2a | ticket-refinement.agent.md | 27 | engine.instructions.md | 48 | thematic overlap | MCP-first preference matches engine's guidance to verify MCP/CLI-facing flows. |
| A2a | ticket-refinement.agent.md | 26 | lifecycle.instructions.md | 5-27 | thematic overlap | "Keep lifecycle transitions valid" maps to lifecycle's full state-machine rules. |
| A2a | ticket-refinement.agent.md | 36 | workflow.instructions.md | 106 | thematic overlap | "Search for related tickets before creating" restates workflow's "Discovery Before Creating". |
| A2b | ticket-refinement.agent.md | 39-41 | audio-transcript.instructions.md | 20-21 | thematic overlap | Both instruct surfacing genuine ambiguity rather than guessing. |
| A2b | ticket-refinement.agent.md | 33-37 | refine-ingest.prompt.md | 22-24 | near-duplicate | "Research first" / discover workspace / search tickets vs. Stage 2 research-and-artifact-inventory dispatch. |
| A2b | ticket-refinement.agent.md | 39-41 | refine-ingest.prompt.md | 23 | near-duplicate | "Ask concise, decision-driving questions" vs. Stage 3 interview-only-what-evidence-cannot-resolve. |
| A2b | ticket-refinement.agent.md | 24 | refine-ingest.prompt.md | 31-32 | near-duplicate | "Do not implement code changes unless explicitly asked" vs. refine-ingest's no-ticket/spec/code-change constraints. |
| A2b | ticket-refinement.agent.md | 33-36 | ticket-next.prompt.md | 16 | near-duplicate | "Discover the active ticket workspace"/`ticket next` vs. "Inspect the draftboard and actionable ticket queue". |
| A2b | ticket-refinement.agent.md | 36 | tickets.prompt.md | 22 | near-duplicate | Same discovery-before-creating cross-reference to workflow.instructions.md, restated verbatim in both. |
| A2b | ticket-refinement.agent.md | 27 | tickets.prompt.md | 24 | near-duplicate | "Prefer MCP ticket tools first" vs. "Prefer `ticket-mcp` tools ... when they are available." |
| A2b | ticket-refinement.agent.md | 24 | tickets.prompt.md | 35 | near-duplicate | "Do not implement code changes unless explicitly asked" vs. "Do not implement code or change unrelated tickets... unless the user explicitly asks." |
| A2b | ticket-refinement.agent.md | 24 | transform-transcript.prompt.md | 57 | near-duplicate | Same "do not implement code unless asked" guardrail restated. |
| A2b | ticket-refinement.agent.md | 39-41 | transform-transcript.prompt.md | 33-34 | thematic overlap | Ambiguity handling vs. Stage 3 "surface anything unresolved as an Open questions note". |
| A3a | transcription.agent.md | 14 | intent-refinement.instructions.md | 8 | thematic overlap | Both state denoising is delegated to audio-transcript.instructions.md/Transcription Agent — same delegation fact, not restated pipeline text. |
| A3a | transcription.agent.md | 44-46 | prompt-ingestion.instructions.md | 18 | near-duplicate | Anchor's 3-stage summary vs. prompt-ingestion's "same three-stage denoise/restructure/verify pipeline" reference. |
| A3a | transcription.agent.md | 1-67 | board.instructions.md | 1-140 | no overlap | No similarity between transcript pipeline and board coordination. |
| A3a | transcription.agent.md | 1-67 | engine.instructions.md | 1-120 | no overlap | No similarity between transcript pipeline and ticket-engine internals. |
| A3a | transcription.agent.md | 1-67 | lifecycle.instructions.md | 1-240 | no overlap | No similarity between transcript pipeline and ticket lifecycle rules. |
| A3a | transcription.agent.md | 1-67 | workflow.instructions.md | 1-320 | no overlap | No similarity between transcript pipeline and ticket workflow operations. |
| A3a | transcription.agent.md | 31-38 | audio-transcript.instructions.md | 13-23 | near-duplicate | **DELEGATION RESTATEMENT**: "The transform is lossless in intent, lossy only in noise" restated near-verbatim from the Core Principle section instead of a clean reference. |
| A3a | transcription.agent.md | 17-28 | audio-transcript.instructions.md | 11 | near-duplicate | **DELEGATION RESTATEMENT**: Input Modes (path vs. raw text, dated folder convention, `input.md`) fully restates audio-transcript's Scope paragraph 3. |
| A3a | transcription.agent.md | 44 | audio-transcript.instructions.md | 26-52 | near-duplicate | **DELEGATION RESTATEMENT**: Stage 1 — Denoise description restates audio-transcript's Stage 1 checklist (filler/false-starts/self-correction/translation). |
| A3a | transcription.agent.md | 45 | audio-transcript.instructions.md | 53 | near-duplicate | **DELEGATION RESTATEMENT**: Stage 2 — Restructure description restates audio-transcript's Stage 2 goal and rules. |
| A3a | transcription.agent.md | 46 | audio-transcript.instructions.md | 64 | near-duplicate | **DELEGATION RESTATEMENT**: Stage 3 — Verify checklist (constraint inventory, no-new-info, correction integrity, translation fidelity, intent equivalence) restated item-for-item. |
| A3a | transcription.agent.md | 53-57 | audio-transcript.instructions.md | 34 | near-duplicate | **DELEGATION RESTATEMENT**: Constraints list (preserve identifiers, keep final self-correction value, split distinct asks, collapse emphasis-repetition) restates audio-transcript's rules. |
| A3a | transcription.agent.md | 62-67 | audio-transcript.instructions.md | 101-106 | near-duplicate | **DELEGATION RESTATEMENT**: Output Requirements list restates audio-transcript's Output Requirements almost verbatim. |
| A3a | transcription.agent.md | 29;49 | audio-transcript.instructions.md | 80-84 | near-duplicate | **DELEGATION RESTATEMENT**: Merge/fold `merged.clean.md` composition rules restate audio-transcript's Multi-Transcript Composition section. |
| A3b | transcription.agent.md | 44 | refine-ingest.prompt.md | 21 | exact duplicate | "**Stage 1 — Denoise.**" heading text identical in both. |
| A3b | transcription.agent.md | 23-25 | refine-ingest.prompt.md | 18-19 | near-duplicate | Dated-folder-creation convention restated. |
| A3b | transcription.agent.md | 44-46 | refine-ingest.prompt.md | 21-23 | thematic overlap | Three-stage pipeline mapped to refine-ingest's staged sequence. |
| A3b | transcription.agent.md | 1-67 | ticket-next.prompt.md | 1-36 | no overlap | No similarity. |
| A3b | transcription.agent.md | 1-67 | tickets.prompt.md | 1-42 | no overlap | No similarity. |
| A3b | transcription.agent.md | 41 | transform-transcript.prompt.md | 27 | exact duplicate | "Run the three-stage pipeline as distinct passes. Do not collapse them." — identical in both files. |
| A3b | transcription.agent.md | 23-25 | transform-transcript.prompt.md | 21 | near-duplicate | Dated-folder-creation convention restated, near-identical wording. |
| A3b | transcription.agent.md | 44-46 | transform-transcript.prompt.md | 29-34 | near-duplicate | **DELEGATION RESTATEMENT**: Stage 1-3 summary restates the same three stages with near-identical bullet content. |
| A3b | transcription.agent.md | 47-50 | transform-transcript.prompt.md | 47-50 | near-duplicate | Delivery/report requirements restate the same reported fields (resolved input path, noise removed vs. intent preserved, ambiguities). |
| A3b | transcription.agent.md | 49 | transform-transcript.prompt.md | 36-43 | near-duplicate | Multi-transcript compose/fold rules restated as "Multi-Transcript Refinement" section. |
| A3b | transcription.agent.md | 58 | transform-transcript.prompt.md | 57 | near-duplicate | "Do not implement code changes described in the transcript..." restated near-verbatim. |
| A4a | intent-refinement.instructions.md | 23 | prompt-ingestion.instructions.md | 20 | exact duplicate | "Output: `REVIEW.md` with an `Approved as scoped` verdict and a scope decision." — identical in both files. |
| A4a | intent-refinement.instructions.md | 24 | prompt-ingestion.instructions.md | 22 | exact duplicate | "This loop replaces a separate traceability-checklist stage: its job is to make sure `ROADMAP.md` ships with zero open questions — every one gets an interview, not a checklist entry." — identical in both files. |
| A4a | intent-refinement.instructions.md | 12 | prompt-ingestion.instructions.md | 20 | near-duplicate | "Never critique or interview from the raw prompt's words alone..." restated as "Critique the cleaned prompt against the research just gathered — never against the raw words alone...". |
| A4a | intent-refinement.instructions.md | 8 | audio-transcript.instructions.md | 22 | thematic overlap | Anchor delegates denoising to audio-transcript's three-stage pipeline; shared "do not guess on ambiguity" principle. |
| A4a | intent-refinement.instructions.md | 12 | refine-ingest.prompt.md | 36 | near-duplicate | "Never critique or interview from the raw prompt's words alone" restated near-verbatim in refine-ingest's constraints. |
| A4a | intent-refinement.instructions.md | 23-24 | refine-ingest.prompt.md | 36-38 | near-duplicate | Both loops (Stage 3/5) and the "ship ROADMAP.md with zero open questions" exit condition restated. |
| A4a | intent-refinement.instructions.md | 1-29 | board.instructions.md | 1 | no overlap | No similarity between the review+interview loop and board coordination. |
| A4a | intent-refinement.instructions.md | 1-29 | engine.instructions.md | 1-40 | no overlap | No similarity; engine covers ticket-system internals. |
| A4a | intent-refinement.instructions.md | 1-29 | lifecycle.instructions.md | 1-80 | no overlap | No similarity; lifecycle covers ticket state machine. |
| A4a | intent-refinement.instructions.md | 1-29 | workflow.instructions.md | 1-200 | no overlap | No similarity; workflow covers session/ticket operations. |
| A4b | intent-refinement.instructions.md | 12-17 | ticket-next.prompt.md | 16-26 | thematic overlap | Both ground decisions/critique in gathered evidence before acting. |
| A4b | intent-refinement.instructions.md | 15 | ticket-next.prompt.md | 30-36 | thematic overlap | Verdict/findings-table requirement maps to ticket-next's validation-step/evidence-plan output requirement. |
| A4b | intent-refinement.instructions.md | 12-17 | tickets.prompt.md | 22-24 | thematic overlap | Grounding interviews in research maps to "search existing tickets/specs first". |
| A4b | intent-refinement.instructions.md | 15 | tickets.prompt.md | 33-34 | thematic overlap | Verdict/findings-table requirement maps to recording assumptions/follow-up gaps. |
| A4b | intent-refinement.instructions.md | 8 | transform-transcript.prompt.md | 12 | near-duplicate | Both point to audio-transcript.instructions.md + Transcription Agent as the authoritative denoise process, in near-identical wording. |
| A5a | prompt-ingestion.instructions.md | 82-90 | board.instructions.md | 5-40 | thematic overlap | Ticket-creation-during-refinement rules overlap board's ownership/coordination rules. |
| A5a | prompt-ingestion.instructions.md | 53 | engine.instructions.md | 44 | thematic overlap | "Validation gates" section maps to engine's "## Validation" guidance. |
| A5a | prompt-ingestion.instructions.md | 101-106 | lifecycle.instructions.md | 5-30 | thematic overlap | "When to Run This Pipeline" sequencing maps to lifecycle's state/review-gate sequencing. |
| A5a | prompt-ingestion.instructions.md | 82-96 | workflow.instructions.md | 218-230 | thematic overlap | Ticket-creation/decision-boundary guidance overlaps workflow's ticket-first expectations. |
| A5a | prompt-ingestion.instructions.md | 18 | audio-transcript.instructions.md | 82 | near-duplicate | **DELEGATION RESTATEMENT**: multi-part naming convention (`input.md`/`input-N.md`/`merged.clean.md`) restated from audio-transcript's Multi-Transcript Composition section. |
| A5a | prompt-ingestion.instructions.md | 18 | audio-transcript.instructions.md | 24 | near-duplicate | **DELEGATION RESTATEMENT**: "same three-stage denoise/restructure/verify pipeline" restates audio-transcript's "process the transcript in three ordered stages... do not collapse". |
| A5a | prompt-ingestion.instructions.md | 18-24 | refine-ingest.prompt.md | 21-25 | near-duplicate | Stage 1-3 responsibilities (denoise/research/first review loop) restated near-verbatim as refine-ingest's numbered workflow steps. |
| A5a | prompt-ingestion.instructions.md | 25-34 | refine-ingest.prompt.md | 16-18 | near-duplicate | Dossier-resumption/continuation-detection guidance restated near-verbatim. |
| A5a | prompt-ingestion.instructions.md | 45-66 | refine-ingest.prompt.md | 30-39 | near-duplicate | Roadmap compilation/versioning and dry-run-improvement-loop rules restated in refine-ingest's Stage 6-9. |
| A5a | prompt-ingestion.instructions.md | 82-91 | refine-ingest.prompt.md | 29;31 | near-duplicate | Ticket-creation-exception / decision-boundary rule restated near-verbatim in refine-ingest's Constraints. |
| A5a | prompt-ingestion.instructions.md | 101-106 | ticket-next.prompt.md | 25 | thematic overlap | Pipeline sequencing (refinement before implementation) vs. ticket-next's state-sequencing guidance. |
| A5b | prompt-ingestion.instructions.md | 54 | tickets.prompt.md | 26 | thematic overlap | Both prefer minimal/smallest ticketization when converting scope into tickets. |
| A5b | prompt-ingestion.instructions.md | 82-90 | tickets.prompt.md | 30-33 | thematic overlap | Ticket-creation-during-refinement vs. tickets.prompt.md's create/update-spec-after-tickets procedure. |
| A5b | prompt-ingestion.instructions.md | 51 | tickets.prompt.md | 31-33 | thematic overlap | Reference-by-id/path + Clickable Reference Policy requirement restated in both. |
| A5b | prompt-ingestion.instructions.md | 16 | transform-transcript.prompt.md | 27 | near-duplicate | "Run each stage as a distinct pass; do not collapse them" vs. "Run the three-stage pipeline as distinct passes. Do not collapse them." |
| A5b | prompt-ingestion.instructions.md | 18 | transform-transcript.prompt.md | 29-33 | near-duplicate | Stage 1 denoise description restated. |
| A5b | prompt-ingestion.instructions.md | 36-39 | transform-transcript.prompt.md | 21 | near-duplicate | `input-N.md` numbering convention restated. |
| A5b | prompt-ingestion.instructions.md | 37 | transform-transcript.prompt.md | 41-43 | near-duplicate | `merged.clean.md` update-from-full-set-of-parts rule restated. |
| A6a | board.instructions.md | 7-8 | workflow.instructions.md | 18-19 | near-duplicate | Draftboard check-in/out description restated as the `ticket board show` orientation step. |
| A6a | board.instructions.md | 7-8 | ticket-next.prompt.md | 16 | near-duplicate | Same draftboard check-in/out description restated as "Inspect the draftboard...before choosing work." |
| A6a | board.instructions.md | 1-95 | engine.instructions.md | 1-99 | no overlap | Board coordination vs. engine-level ticket-system internals — no overlap. |
| A6a | board.instructions.md | 1-95 | lifecycle.instructions.md | 1-200 | no overlap | Board coordination vs. ticket state machine — no overlap. |
| A6a | board.instructions.md | 1-95 | audio-transcript.instructions.md | 1-300 | no overlap | Board coordination vs. transcript pipeline — no overlap. |
| A6a | board.instructions.md | 1-95 | refine-ingest.prompt.md | 1-400 | no overlap | Board coordination vs. prompt-ingestion pipeline — no overlap. |
| A6a | board.instructions.md | 1-95 | tickets.prompt.md | 1-400 | no overlap | Board coordination vs. ticket-creation prompt — no overlap. |
| A6b | board.instructions.md | entire file | transform-transcript.prompt.md | entire file | no overlap | No matching sections or thematic overlap found. |
| A7 | engine.instructions.md | 16 | lifecycle.instructions.md | 5-10 | thematic overlap | Both cover lifecycle/state-machine invariants ("Respect ticket lifecycle/state machine invariants" vs. the one-way state machine definition). |
| A7 | engine.instructions.md | 29-42 | workflow.instructions.md | 43-60 | thematic overlap | Both cover index/store discovery, reconciliation, correct-store targeting. |
| A7 | engine.instructions.md | n/a | audio-transcript.instructions.md | n/a | no overlap | No similarity between ticket-engine design and transcript pipeline. |
| A7 | engine.instructions.md | n/a | refine-ingest.prompt.md | n/a | no overlap | No similarity between ticket-engine design and prompt-ingestion sequencing. |
| A7 | engine.instructions.md | 16 | ticket-next.prompt.md | 25 | thematic overlap | Lifecycle/state-machine invariant vs. "move the ticket through the correct state sequence". |
| A7 | engine.instructions.md | 7-12 | tickets.prompt.md | 15-17 | thematic overlap | Ticket-tool paths (`ticket-cli`, `ticket-api`) vs. tickets.prompt.md's build/use `ticket-cli`/`ticket-mcp` instructions. |
| A7 | engine.instructions.md | n/a | transform-transcript.prompt.md | n/a | no overlap | No similarity between ticket-engine design and transcript-transform prompt. |
| A8 | lifecycle.instructions.md | 103-127 | workflow.instructions.md | 67-95 | thematic overlap | workflow.instructions.md references lifecycle's `planned`-state freeze contract (cross-reference, not restatement). |
| A8 | lifecycle.instructions.md | 1-206 | audio-transcript.instructions.md | 1-115 | no overlap | No matching content. |
| A8 | lifecycle.instructions.md | 1-206 | refine-ingest.prompt.md | 1-47 | no overlap | No matching content. |
| A8 | lifecycle.instructions.md | 12-26 | ticket-next.prompt.md | 25 | thematic overlap | "Update ticket state immediately..." vs. "Move the ticket through the correct state sequence...". |
| A8 | lifecycle.instructions.md | 176 | tickets.prompt.md | 30 | near-duplicate | "The relevant spec links the exact ticket folder path(s), the updated docs, and the passing or blocked validation results" vs. "create or update the relevant spec after the ticket set is created or matched." |
| A8 | lifecycle.instructions.md | 1-206 | transform-transcript.prompt.md | 1-57 | no overlap | No matching content. |
| A9 | workflow.instructions.md | n/a | audio-transcript.instructions.md | n/a | no overlap | No overlapping sections found. |
| A9 | workflow.instructions.md | 220 | refine-ingest.prompt.md | 31 | thematic overlap | Both discuss ticket-creation boundaries during a pipeline/session. |
| A9 | workflow.instructions.md | 119-126 | ticket-next.prompt.md | 16 | thematic overlap | Picking next work / inspecting draftboard before choosing work. |
| A9 | workflow.instructions.md | 106 | tickets.prompt.md | 22 | near-duplicate | "Always search for existing tickets before creating new ones..." vs. "Search existing tickets first per workflow.instructions.md#discovery-before-creating" (direct cross-reference restating the anchor rule). |
| A9 | workflow.instructions.md | n/a | transform-transcript.prompt.md | n/a | no overlap | No overlapping sections found. |
| A10 | audio-transcript.instructions.md | 11 | refine-ingest.prompt.md | 19 | near-duplicate | Dated-folder-creation-for-raw-text convention restated near-verbatim. |
| A10 | audio-transcript.instructions.md | 32 | ticket-next.prompt.md | — | no overlap | No matching sections. |
| A10 | audio-transcript.instructions.md | 32 | tickets.prompt.md | — | no overlap | No matching sections. |
| A10 | audio-transcript.instructions.md | 32 | transform-transcript.prompt.md | 30 | near-duplicate | **DELEGATION RESTATEMENT**: translate-non-English-to-English instruction restated near-verbatim. |
| A11 | refine-ingest.prompt.md | 22-27 | ticket-next.prompt.md | 16-26 | thematic overlap | Artifact discovery / minimal-context-gathering for next slice, expressed differently in each pipeline. |
| A11 | refine-ingest.prompt.md | 31-37 | tickets.prompt.md | 21-35 | thematic overlap | Ticket/spec creation rule-space overlaps (refine-ingest forbids it mid-pipeline; tickets.prompt.md defines the full flow). |
| A11 | refine-ingest.prompt.md | 19 | transform-transcript.prompt.md | 21 | near-duplicate | Dated raw-text folder + `input.md` convention restated near-verbatim (same pair also surfaced via the audio-transcript anchor in A10). |
| A11 | refine-ingest.prompt.md | 21 | transform-transcript.prompt.md | 40-41 | near-duplicate | `merged.clean.md` update-from-full-set-of-clean-parts rule restated. |
| A11 | refine-ingest.prompt.md | 32 | transform-transcript.prompt.md | 57 | near-duplicate | "Do not implement any code change described in the prompt/transcript" guardrail restated near-verbatim. |
| A12 | ticket-next.prompt.md | 12 | tickets.prompt.md | 13 | near-duplicate | Reference-list line restating the same `ticket-cli`/`ticket-mcp` README links (superset vs. subset). |
| A12 | ticket-next.prompt.md | 16-26 | tickets.prompt.md | 21-29 | thematic overlap | Pre-work discovery / ticket-selection behavior described differently per prompt's purpose. |
| A12 | ticket-next.prompt.md | 30-36 | tickets.prompt.md | 37-42 | near-duplicate | Both "Return"/"Response" sections use the same Clickable-Reference-Policy phrase and a near-identical bullet-list shape. |
| A12 | ticket-next.prompt.md | 1-36 | transform-transcript.prompt.md | 1-57 | no overlap | No similarity between ticket-selection prompt and transcript-transform prompt. |
| A13 | tickets.prompt.md | 20-35 | transform-transcript.prompt.md | 25-34 | thematic overlap | Both have a titled "Workflow" section enumerating stepwise stages. |
| A13 | tickets.prompt.md | 35 | transform-transcript.prompt.md | 57 | near-duplicate | "Do not implement code or change unrelated tickets, specs, or dependencies unless the user explicitly asks" vs. "Do not implement any code changes described in the transcript... unless the user explicitly asks to act on it afterward." |
| A13 | tickets.prompt.md | 37-42 | transform-transcript.prompt.md | 51-55 | thematic overlap | Both list required output/report items (paths, what was removed/preserved, ambiguities/open questions). |
