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

| Tier | Use for | Avoid for |
|---|---|---|
| High-capability (most expensive) | Large-scope planning, cross-cutting architecture, high-level reasoning, review of dense content or a single critical artifact, final synthesis/decisions | Command batches, bulk output summarization, mechanical edits, wide file sweeps |
| Mid-tier (balanced) | Focused implementation on a known slice, targeted debugging, moderate multi-file edits | Long research sweeps that a cheap model can pre-digest |
| Cheap/fast (least expensive) | Running and summarizing command/tool-call batches, condensing large tool outputs, summarizing many large files or artifacts, mechanical extraction, first-pass research triage | Final architectural decisions, subtle correctness review of dense artifacts |

## Delegation Rules

- In a large-model session, delegate to a cheaper subagent model when the subtask is: a batch of command or tool calls, summarization of large or numerous tool outputs, or research/summarization across many large files or artifacts.
- Give the subagent a **self-contained prompt** (see the context-isolation checklist above) and pin the intended cheaper `model` explicitly on `runSubagent`.
- **Model-name format:** the `model` field must be `"Model Name (Vendor)"`, e.g. `"Claude Haiku 4.5 (copilot)"`. A bare label like `"haiku"` or `"cheap"` will error or be silently ignored. When unsure of the exact string, read the `runSubagent` tool schema rather than guessing.
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
