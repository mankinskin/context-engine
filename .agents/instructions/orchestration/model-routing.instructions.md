---
description: "Use when deciding whether to delegate work to a subagent with a cheaper model. Covers capability gating, context isolation, tiered model ladder, delegation rules, and parallel fan-out."
---

## Model Cost Awareness & Routing

> **Capability gate — read first.** This strategy requires a subagent-capable surface that exposes a `runSubagent` (or equivalent) tool with a selectable `model`. If no such tool is loadable in the current session, this entire section is **inert**: do the work inline using the tactical mechanics in the other orchestration files and skip the ladder. Do not narrate a routing plan you cannot execute, and do not spend reasoning budget describing delegation that will not happen.

> **Context isolation — the single most important rule.** A subagent inherits **none** of the current session's context: no conversation history, no prior findings, no shared "we". A context-dependent prompt does not fail loudly — it burns a full agent spawn just to reply "I have no prior context" and hand the question back to you. Every subagent prompt **must be self-contained**. Checklist before dispatch:
> - name every file with its full workspace-relative path (never "the file we discussed")
> - paste the exact snippet, error, or scope the subagent must act on
> - state the repository root and any command/cwd assumptions
> - define every referent — no "this", "that fix", or "the earlier change"
> - state the exact return shape you want back (e.g. `scope | finding | pointer`)

When a subagent tool *is* available, token cost is a function of *which model* does the work, not only how much context it sees. Be model-cost-aware: reserve expensive, high-capability models for work that genuinely needs them, and delegate routine or bulk work to smaller, cheaper models.

This matters most in sessions driven by a large, expensive model. Treat that model as an active **router**: it plans and reasons at a high level, then dispatches routine subtasks to cheaper models via `runSubagent` (passing an explicit cheaper `model`) instead of spending premium tokens itself.

## Tiered Model Ladder (Smartness vs Cost)

This table is the **canonical** tier ladder for the repository. [orchestrator-delegation.instructions.md](orchestrator-delegation.instructions.md) references it rather than restating it — do not fork a second copy.

Prices come from `tools/model-prices/model_prices.json` and are USD per 1M tokens. Re-check with `sync_model_prices.py --query <model>` rather than trusting these numbers indefinitely — see [model-prices.instructions.md](model-prices.instructions.md).

**Every model named below was verified to be offered by `runSubagent` on 2026-07-27** (see "Roster is not the catalogue"). The price table lists every vendor's catalogue; this table lists only what can actually be dispatched.

| Tier | Preferred models | in$/M · cread$/M · out$/M · ctx | Use for | Avoid for |
|---|---|---|---|---|
| T0 — Orchestrator (most expensive) | Claude Opus 5 | 5 · 0.5 · 25 · 1M | Large-scope planning, cross-cutting architecture, high-level reasoning, review of dense content or a single critical artifact, final synthesis/decisions | Command batches, bulk output summarization, mechanical edits, wide file sweeps |
| | Claude Opus 4.8 | 5 · 0.5 · 25 · 1M | Prior-generation equivalent; prefer Opus 5 | |
| T1 — Complex implementation (gate ceiling) | GPT-5.6 Terra | 2.5 · 0.25 · 15 · 1.05M | High-risk or cross-cutting implementation, dense refactors, gnarly debugging where the default tier already failed once. Prefer Terra when the slice needs a very large context window | Routine slices the default tier handles; anything the cheap tier can pre-digest |
| | GPT-5.3-Codex | 1.75 · 0.175 · 14 · 400k | Heavy code-generation slices with bounded input | |
| **T2 — Default implementation** | **Claude Sonnet 5** | **2 · 0.2 · 10 · 1M** | **The default for all delegated implementation, review, targeted debugging, and moderate multi-file edits.** Strictly cheaper and newer than Sonnet 4.5 (3 · 0.3 · 15 · 1M) on every axis — never route new work to Sonnet 4.5 | Long research sweeps or bulk summarization a cheap model can pre-digest |
| T3 — Cheap worker (floor) | **GPT-5 mini** *(default)* | **0.25 · 0.025 · 2 · 400k** | Running and summarizing command/tool-call batches, condensing large tool outputs, summarizing many large files or artifacts, first-pass research triage, and judgement-free extraction. Nothing offered on this surface is cheaper on input — this is the floor | Final architectural decisions, subtle correctness review of dense artifacts |
| | Kimi K2.7 Code *(T3 code-specialist)* | 0.95 · 0.19 · 4 · 262k | Use for bulk code-shaped work where a code-specialist model materially improves edit quality; pick only when the task requires code-specialist capabilities — GPT-5 mini remains the T3 default | |
| | GPT-5.6 Luna | 1 · 0.1 · 6 · 1.05M | The wide-context cheap option: use when the input does not fit 400k, or when a cheap model must digest a huge input *and* emit non-trivial code | |
| | GPT-5.4 mini | 0.75 · 0.075 · 4.5 · 400k | Step up from GPT-5 mini when the unit needs real reasoning over what it read | |

There is no tier below T3 on this surface: no offered model undercuts GPT-5 mini on input or cache read, so judgement-free extraction and bulk triage share the same band.

### Roster is not the catalogue

`tools/model-prices/model_prices.json` is a **vendor catalogue**. It lists models `runSubagent` will refuse. Routing a unit to a catalogue-only model does not degrade gracefully — the dispatch errors outright and the spawn is wasted.

- Only route to a model verified present in the surface's model list. To re-verify, dispatch one trivial subagent with a deliberately wrong model string; the error response enumerates every available model.
- **Model-name format is exact.** `"GPT-5.3-Codex (copilot)"` is hyphenated; `"GPT-5.3 Codex (copilot)"` errors. Copy strings from the surface list, not from prose.
- If a model in this table is rejected, fall back to the next entry in the same band and update this table — do not silently land at T2.
- **Verified available, 2026-07-27:** Claude Fable 5, Claude Opus 4.6, Claude Opus 4.7, Claude Opus 4.8, Claude Opus 4.8 (fast mode) (Preview), Claude Opus 5, Claude Sonnet 4.5, Claude Sonnet 4.6, Claude Sonnet 5, Claude Opus 4.5, Claude Haiku 4.5, Gemini 3.1 Pro (Preview), Gemini 3.5 Flash, Gemini 3.6 Flash, Gemini 2.5 Pro, GPT-5.3-Codex, GPT-5.4 mini, GPT-5.4, GPT-5.5, GPT-5.6 Luna, GPT-5.6 Sol, GPT-5.6 Terra, GPT-5 mini, Kimi K2.7 Code, MAI-Code-1-Flash, Auto. Notably **absent**: every `*-flash-lite` model, GPT-5.4 nano, and GPT-5.4 Pro.
- `MAI-Code-1-Flash` is offered but carries **no row in the price table**, so it cannot be cost-ranked. Do not route to it under a cost rationale.
- `Auto` delegates model selection away from you and defeats cost-aware routing when used as a default. It is permitted only as an explicit escape hatch: use `Auto (copilot)` when no tier model fits or when the surface rejects the intended model. Because it delegates model selection away from the caller and prevents the cost gate from reasoning about the choice, any use must be stated explicitly in the dispatch rationale and not used as a default.

### Optimizing the cheap tier (T3)

Most delegated volume lands here, so T3 dominates real spend. Route it on the right metric:

- **Select on `input_mtok` + `cache_read_mtok`, not `output_mtok`.** T3 units are input-heavy and output-tiny — they swallow a long log or many files and return a few lines. The output column is nearly irrelevant to their bill; the input and cache-read columns are the whole cost.
- **This inverts the naive ranking.** Claude Haiku 4.5 (in 1 · cread 0.10 · 200k) looks cheap on output ($5/M) but is the *most* expensive input option in the band with the smallest window. GPT-5 mini (in 0.25 · cread 0.025 · 400k) costs 4x less on input, 4x less on cache read, and carries a 2x larger window. Do not reach for Haiku by habit.
- **Fit the context window before anything else.** A unit whose input does not fit forces truncation, a re-dispatch, or a silently incomplete answer — each of which costs more than the tier ever saved. Send wide sweeps to GPT-5.6 Luna (1.05M); reserve GPT-5 mini for inputs that genuinely fit 400k.
- **Batch aggressively at T3.** One subagent digesting twenty files beats twenty subagents digesting one file each; spawn overhead is per-dispatch and does not shrink with the model.
- Kimi K2.7 Code is available as a T3 code-specialist option: it costs ~4x the default's input and ~8x its cache-read, so pick it only when the task is genuinely code-specialist, not for generic bulk work.
- **Do not verify cheap output with an expensive model.** If a T3 result needs checking, check it with ground truth (a grep, a `--check` run, a bounded read), not by re-running the unit a tier up. Re-running upward erases the saving twice over.
- **One retry, then step up exactly one band.** T3 failure goes to T2. Jumping a cheap unit straight to T1/T0 is how low-tier routing turns into a net loss.
- **The Gemini Flash models are dominated.** Gemini 3.5 Flash and 3.6 Flash (in 1.5 · cread 0.15 · 1M) are beaten by GPT-5.6 Luna on input, cache read, output, and context window. Between the two, 3.6 Flash dominates 3.5 Flash. Neither belongs in T3.

### Picking a tier

- **Start at T2.** Claude Sonnet 5 is the default implementation model; deviate only for a stated reason.
- **Drop to T3** when the unit is bulk, mechanical, or read-only triage, or needs no judgement at all — that is where most delegated volume belongs.
- **Climb to T1** only after a T2 attempt was wrong or too shallow, or when the slice is plainly cross-cutting and high-risk. Record why in the delegation note.
- **Budget pressure** shifts the whole ladder down one step: T2 work goes to T3, T1 work goes to T2.
- Among models of equal cost, prefer the latest generation and the larger context window.

## Delegation Rules

- In a large-model session, delegate to a cheaper subagent model when the subtask is: a batch of command or tool calls, summarization of large or numerous tool outputs, or research/summarization across many large files or artifacts.
- Give the subagent a **self-contained prompt** (see the context-isolation checklist above) and pin the intended cheaper `model` explicitly on `runSubagent`.
- **Model-name format:** the `model` field must be `"Model Name (Vendor)"`, e.g. `"Claude Sonnet 5 (copilot)"` or `"GPT-5 mini (copilot)"`. A bare label like `"mini"` or `"cheap"` will error or be silently ignored. The string must match the surface's model list exactly, punctuation included — see "Roster is not the catalogue" above.
- **Default model:** when a dispatch does not justify a different tier, pin `"Claude Sonnet 5 (copilot)"`. Do not fall back to `"Claude Sonnet 4.5 (copilot)"` — it is strictly more expensive for the same context window.
- Ask the subagent to return only the distilled finding — scope, result, blocker, pointer — not raw output. The expensive model reasons over the summary, not the bulk.
- Reserve the high-capability tier for planning, high-level reasoning, and review of dense content or individual artifacts.

## When NOT to Delegate (The Floor)

- Each subagent is a full agent loop with real spawn overhead. Delegating a single bounded read (one `peek --grep`, one small file window) costs **more** than doing it inline.
- Delegate only when the subtask is *bulky*, *numerous*, or *context-heavy* — many files, a long log to digest, a wide search to triage. For one small bounded operation, just do it yourself.
- "Delegate routine or bulk work" means **bulk**, not **trivial**. Over-delegation is its own token bonfire.

## Verify Subagent Output Before Acting

- Treat every subagent summary as an **unverified claim**. Subagents hallucinate like any model.
- Spot-check load-bearing findings against ground truth (a real grep, a `--check` run, a bounded read) **before** any finding drives an edit or a decision.
- Reasoning over a summary is fine; trusting it blindly is how a hallucinated refactor ships.

## Parallel Fan-Out (Highest-Throughput Pattern)

- Independent **read-only** probes can be dispatched **concurrently** in a single block, then reasoned over as merged results. This is the single highest-throughput delegation pattern.
- Good fan-out targets: survey N files/crates at once, run several independent searches, gather evidence from multiple subsystems in parallel.
- Keep fan-out to read-only work; do not parallelize writes to overlapping scope.
- Each parallel prompt must still be independently self-contained.

## Failure Path

- If a subagent errors, returns empty, refuses, or replies that it lacks context: first retry **once** with a more self-contained prompt (the usual cause is context isolation, not a broken tool).
- If it still fails, abandon delegation and do the subtask inline; record the failure as a one-line finding.
- Escalate a subtask *up a tier* only for **quality** insufficiency (the cheaper model's answer is wrong or too shallow), and record why.

## Inspection Before Delegation or Premium Reasoning

- Use bounded inspection tooling (`peek` CLI, `repo_map.toon`, interface skeletons) to render reduced, focused views of artifacts before either spending expensive-model tokens or handing the artifact to a subagent.
- A focused, reduced view is often enough for the expensive model; the full artifact usually is not needed.
