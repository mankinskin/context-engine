---
description: "Use before turning a raw prompt into tickets, a spec, or any other complex downstream workflow. Defines the structural shell of the prompt-ingestion pipeline — dossier layout, the stage sequence from a refined intent through artifact inventory, research-informed restructure, traceability checklist, and roadmap compilation — plus the decision boundary and when to run it."
applyTo: "**/*.md"
---

## Purpose

A raw prompt — a rambling transcript, a dictated ask, a stream-of-consciousness request — must not be handed directly to `tickets.prompt.md`, `spec.prompt.md`, or an implementation session. Structure and scope are extracted first, cheaply, in a bounded pipeline, and only the resulting dossier is used to seed tickets/specs. This closes the gap the raw-prompt path otherwise leaves open: unbounded scope, no verification lens, and no evidence that the eventual tickets actually cover what the requester said.

This file is the ingestion shell: it owns the dossier folder layout and the structural stages that run once a prompt's intent is pinned down. [intent-refinement.instructions.md](intent-refinement.instructions.md) owns the two stages that clarify what the prompt is actually asking for; that verdict is the input to everything below.

## The Six Stages

Run each stage as a distinct pass; do not collapse them. Each stage has one job and one exit artifact.

1. **Denoise (cheap).** Owned by [intent-refinement.instructions.md](intent-refinement.instructions.md). Output: `input.md`, `input.clean.md`.
2. **Review gate.** Owned by [intent-refinement.instructions.md](intent-refinement.instructions.md). Output: `REVIEW.md` with an `Approved as scoped` verdict and a scope decision.
3. **Artifact inventory.** Once the scope is bounded, dispatch a read-only [Explore Agent](../../agents/explore.agent.md) or [Research Agent](../../agents/research.agent.md) pass to gather every existing artifact relevant to the bounded scope: tickets (ids + state), specs (ids + slugs), docs, prior transcripts/dossiers, and concrete code/config file paths the eventual work will touch or depend on. Do not re-derive this list later — every downstream stage cites entries from it instead of re-discovering paths. Output: `ARTIFACTS.md`, one row per artifact with id/path, a one-line relevance note, and its current state (e.g. ticket state, spec state, file exists/does not exist yet).
4. **Research-informed restructure.** With the scope bounded and the artifact inventory in hand, dispatch [Research Agent](../../agents/research.agent.md) or [Structured Research Agent](../../agents/structured-research.agent.md) (dialectic pass, when the first answer needs adversarial testing) to check each reviewed concern against actual repository capability, then rewrite the bounded scope as concrete, independently actionable work packages — each with an outcome, a non-goal, and a validation method. Output: one or more numbered documents (`01-...md`, `02-...md`, ...).
5. **Traceability checklist.** Close the loop: map every requirement from the raw prompt to the dossier location that addresses it, run deterministic checks that the expected artifacts exist and are non-empty, and list genuinely open questions rather than silently resolving them. Output: a final numbered `NN-completion-checklist.md` and a `README.md` index stating reading order, scope, and the decision boundary below.
6. **Roadmap compilation (iterative).** Compile the entire dossier — artifact inventory, restructured work packages, and traceability results — into a single `ROADMAP.md` that becomes the entry point for the first executing session. Then dry-run it and refine it before handing it off. See "Roadmap Compilation and Versioning" and "Roadmap Improvement Loop" below for its required contents, dry-run procedure, and iteration rule.

## Roadmap Compilation and Versioning

`ROADMAP.md` is the single, current, most-refined artifact the pipeline produces. It is the entry point a fresh executing session reads first — it must be self-contained enough that a session starting cold from `ROADMAP.md` alone (plus the cited artifact ids/paths) can begin work without re-reading the whole dossier.

**Required contents, in order**:

1. **Relevant artifact IDs** — pull forward the rows from `ARTIFACTS.md` that the current roadmap depends on: ticket ids, spec ids/slugs, doc paths, code/config file paths. Reference by id/path only; do not re-paste artifact bodies.
2. **Active blockers** — anything currently unresolved that would stop an executing session cold: missing decisions, unresolved research questions from the traceability checklist, unmet preconditions. State each as a concrete, actionable blocker, not a vague risk.
3. **Validation gates** — the most important commands/checks that must pass during and after execution (test commands, `sync-targets --check`, browser verification, spec/ticket linkage checks). Name exact commands where they exist; do not leave a gate as "run the tests."
4. **Full roadmap** — the complete set of scoped tasks, each with a single, clearly measurable objective (one outcome, one acceptance check — not a bundle of loosely related changes). Order tasks by dependency. For each task, size it: a task completable in one session is marked single-session; a task too large for one session, or with internal dependencies complex enough to need cross-session tracking, is **not** decomposed inline in `ROADMAP.md` — it is turned into a ticket (see "Ticket Creation During Refinement" below), and the roadmap carries only the ticket id and a one-line summary.
5. **Heads-up notes** — a flat list of quirks, gotchas, and good-to-know information gathered during research (surprising existing behavior, known broken tooling, naming inconsistencies, things that looked like bugs but are documented behavior) that would otherwise cost a fresh session time to rediscover.

**Size constraint**: `ROADMAP.md` is a root anchor for the entire effort, not an exhaustive plan — keep it readable in one pass. If compiling it produces a sprawling task list or deeply nested sub-tasks, that is a signal to push the complexity into tickets (item 4 above) rather than growing the file. The roadmap should read like a table of contents with status, not a full project plan.

**Iteration rule**: `ROADMAP.md` is expected to be revised as research deepens or execution surfaces new information. Never overwrite a prior iteration in place. Before writing an improved version, rename the existing `ROADMAP.md` to a versioned name (`ROADMAP.v1.md`, `ROADMAP.v2.md`, ...) inside the same dossier folder, then write the new, more refined content to `ROADMAP.md`. Only one file is ever named `ROADMAP.md` — it is always the most current, most refined iteration. The dossier's `README.md` index must point at `ROADMAP.md`, not at a versioned snapshot.

## Roadmap Improvement Loop

A compiled roadmap is a draft until it has been dry-run at least once. Repeat this loop until a dry-run pass surfaces no new blocker or structural defect, then treat the current `ROADMAP.md` as ready to hand off.

**Dry-run procedure**:

1. Read `ROADMAP.md` cold, as the first executing session would — do not use any context from having written it.
2. Walk the task list in order. For each task, check whether everything it needs is actually present: the artifacts it cites resolve (via a bounded `peek-mcp`/`ticket-mcp`/`spec-mcp` probe, not by assumption), its stated objective is a single measurable outcome, its validation gate is an exact command, and its declared dependencies (prior tasks, tickets, decisions) are already satisfied by that point in the order.
3. Record every gap surfaced this way as one of two kinds:
   - **Blocker** — something that would stop the executing session cold (unresolved decision, missing precondition, an artifact that does not exist, a dependency ordered after the task that needs it).
   - **Informational gap** — something the session could stumble on but is not fully blocking (an ambiguous acceptance check, a missing heads-up note, an unclear ownership boundary between two tasks).
4. Fix cheap findings directly in `ROADMAP.md`: reorder a task, sharpen an objective to be single-outcome, add a missing validation command, add a heads-up note. Route expensive findings to a ticket per "Ticket Creation During Refinement" below instead of expanding the roadmap prose.

**Structural advice for even flow and dependency resolution**:

- Prefer a roadmap that reads as a dependency-ordered list with no forward references — a task must never depend on something only introduced by a later task.
- Split any task that bundles more than one measurable objective; merge trivially small fragments that only exist because a split was too aggressive.
- Flag tasks that all depend on the same upstream blocker or artifact — they are candidates for parallel execution once that blocker clears, and the roadmap should say so rather than force a false sequential order.
- Watch for a task whose real dependency is implicit (undeclared code coupling, an unstated shared file) rather than declared — the artifact inventory (`ARTIFACTS.md`) is the source to check this against.
- A roadmap with an uneven flow — a few tiny tasks followed by one sprawling task — is a decomposition defect: push the sprawling task's internal complexity into a ticket rather than leaving it lumpy in the roadmap.

## Ticket Creation During Refinement

Unlike the read-only stages above, roadmap refinement (Stage 6 and its dry-run loop) is explicitly allowed to create or update tickets and to add entries to the linked artifact set. This is the mechanism for keeping `ROADMAP.md` small: complex task dependencies and large blockers are modeled in the ticket system, not inlined into the roadmap.

- Create or update a ticket when a task is too large for one session, has internal sub-dependencies worth tracking across sessions, or is itself a blocker significant enough to need its own status and history.
- Follow the ticket threshold and workflow in `AGENTS.md`'s Task Routing and `.agents/prompts/tickets.prompt.md` when creating tickets — this stage does not bypass that threshold, it is simply where the decision to cross it gets made for roadmap-sized work.
- After creating or updating a ticket, add its id to `ARTIFACTS.md` and reference it from the roadmap's "Relevant artifact IDs" and "Full roadmap" sections instead of duplicating its content.
- Do not create a ticket for something the roadmap can state in one line (a single-session task, a simple blocker with an obvious resolution) — that is scope creep in the other direction.

## Decision Boundary

The dossier produced by this pipeline is a bounded research-and-scoping artifact, not an implementation. State this explicitly in the dossier's `README.md`:

- Stages 1-5 (denoise through traceability checklist) are read-only: they may read source, docs, tickets, and specs, but do not mutate them, do not create or edit a spec, and do not change workflow or store state.
- Stage 6 (roadmap compilation and its improvement loop) is the one exception: it is explicitly allowed to create or update tickets and to add entries to the linked artifact set, per "Ticket Creation During Refinement" above. It still does not create or edit a spec — that remains a separate, later step.
- `ROADMAP.md` is a scoping and sequencing artifact, not a spec — it names the tasks an executing session should pick up, with complex decomposition delegated to tickets created during refinement. Turning roadmap items into a spec happens in a **separate**, later step — `spec.prompt.md` — consuming `ROADMAP.md` and its linked tickets as input.

This mirrors [escalation-gate.instructions.md](escalation-gate.instructions.md) and [phase-separation.instructions.md](phase-separation.instructions.md): discovery/interview/review happen before implementation, and this pipeline is exactly that discovery phase for a raw prompt — with ticket creation as the one deliberate, scoped exception that keeps the roadmap itself small.

## When to Run This Pipeline

Run it before `tickets.prompt.md`, `spec.prompt.md`, or any multi-file implementation session whenever the incoming prompt is:

- a raw transcript, dictation, or stream-of-consciousness prompt rather than an already-scoped ask,
- broad enough that "just start implementing" would produce an unbounded session (compare the "Feature or refactor" and "Unfamiliar module" rows in `AGENTS.md`'s Task Routing table),
- ambiguous about whether it is one request or several interleaved concerns.

Skip it for an already-bounded, single-file fix or an ask that already names its acceptance criteria — running the full pipeline on a two-line, unambiguous prompt is pure overhead.

## Cost Note

Stage 1 (denoise, in [intent-refinement.instructions.md](intent-refinement.instructions.md)) runs on the cheap tier per `transcription.agent.md`'s own `model:` declaration. Stage 3 (artifact inventory) is mechanical read-only extraction and belongs on the T3 floor. Stage 2 (review), Stage 4 (research), and Stage 6 (roadmap compilation) are judgement-bearing and route per the tier ladder in [model-routing.instructions.md](model-routing.instructions.md) — do not run the whole pipeline on the orchestrator-tier model when the denoise and inventory passes alone are mechanical.
