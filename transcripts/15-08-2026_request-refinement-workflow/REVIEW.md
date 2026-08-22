# Review: Formalize a Request-Refinement Workflow

## Verdict

**Approved as scoped, with two required refinements.** The request names a
concrete precedent ([transcripts/15-08-2026_verification-first-workflow/](../15-08-2026_verification-first-workflow/))
and a clear intended trigger point ("before we use a user's prompt to kick off
tickets or other complex workflows"), so the scope is bounded enough to
execute in one session — unlike the precedent transcript itself, which needed
a review pass to bound it.

## Existing-Capability Check

| Existing capability | Evidence | Implication |
| --- | --- | --- |
| A denoise/restructure/verify pipeline for raw input already exists. | [transcription.agent.md](../../.agents/agents/transcription.agent.md) + [audio-transcript.instructions.md](../../.agents/instructions/transcripts/audio-transcript.instructions.md) | Reuse this as Stage 1 verbatim; do not design a second denoise pipeline. |
| A review/critique contract for verdicts + findings + scope decisions already exists. | [review.agent.md](../../.agents/agents/review.agent.md) | Reuse the same shape (verdict, findings table, scope decision) for Stage 2 of the new pipeline, adapted from "review an implementation" to "review a raw request". |
| Research and dialectic-research agents already exist for checking a claim against real repository capability. | [research.agent.md](../../.agents/agents/research.agent.md), [structured-research.agent.md](../../.agents/agents/structured-research.agent.md) | Reuse for Stage 3 rather than inventing a new research contract. |
| Phase separation (discovery before implementation) and escalation-on-ambiguity are already repository law. | [phase-separation.instructions.md](../../.agents/instructions/orchestration/phase-separation.instructions.md), [escalation-gate.instructions.md](../../.agents/instructions/orchestration/escalation-gate.instructions.md) | The new pipeline is this existing law applied specifically to a raw, not-yet-scoped request; it must cite these files rather than restate them. |
| `AGENTS.md` Task Routing already decides ticket-vs-spec-vs-simple-fix routing, but has no branch for "the input itself isn't scoped yet". | [AGENTS.md](../../AGENTS.md) Task Routing section | Add exactly one new routing row pointing at the new pipeline; do not restructure the existing rows. |

## Findings

| Severity | Finding | Why it matters | Required improvement |
| --- | --- | --- | --- |
| High | The request's phrase "kick off tickets... without going ahead and implementing" is a decision boundary, not just a preference. | Without a stated boundary, the pipeline could quietly drift into creating tickets or editing code during Stage 3 research, defeating the request's purpose. | State explicitly, in the pipeline's own instruction file, that it never creates tickets/specs/code and that ticket/spec creation is a separate later step consuming its output. |
| Medium | The request says "always do this in the future" but does not bound when it applies. | Running a 4-stage dossier pipeline on an already two-line, unambiguous request is pure overhead. | Add an explicit skip condition: bounded, single-file asks with clear acceptance criteria bypass the pipeline. |
| Medium | The precedent transcript is valuable evidence but is not itself a reusable process — it is one instance. | If only the precedent is referenced without extracting the repeatable steps, the next raw request has nothing operational to run. | Produce both: (1) a canonical instructions file describing the repeatable 4-stage process referencing the precedent as the worked example, and (2) a slash-command prompt (`/refine-ingest`) that sequences the stages, mirroring `transform-transcript.prompt.md`. |
| Low | The request itself is a candidate to run the new pipeline against, per its own last sentence. | Demonstrating the pipeline on its own originating request is the most direct verification that the pipeline works. | Produce this dossier (`transcripts/15-08-2026_request-refinement-workflow/`) as the applied instance, alongside the reusable instructions/prompt files. |

## Scope Decision

This dossier is complete when it contains:

1. this review with verdict and required improvements;
2. a completion checklist mapping every requirement in `input.clean.md` to where it is addressed;
3. the reusable pipeline instructions file and slash-command prompt, created as repository artifacts (not duplicated inside this dossier).

This dossier excludes: creating any ticket, creating or editing any spec, and any change to product code or workflow/store state.
