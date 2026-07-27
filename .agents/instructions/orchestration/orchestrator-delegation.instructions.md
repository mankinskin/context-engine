---
description: "Use at the start of and throughout every session: Cost-aware delegation and orchestration guidance. Covers cost gating, sub-agent delegation contract, context isolation, and when to delegate vs execute directly."
---

## When to Activate

Activate this rule when:
- The running model's `output_mtok` (USD per 1M output tokens) **strictly exceeds** threshold `X = 15` (1500 credits/1M at 100 credits = $1)
- The task is bulky, numerous, or context-heavy enough to benefit from delegated execution even if below threshold

**Trigger rule**: At or below X, execute directly. Strictly above X, operate as orchestrator and delegate routine execution.

**Driving field**: `output_mtok` (the `out$/M` column). Do NOT substitute a blended or input-based metric.

## Cost Gating

**Source of truth**: `tools/model-prices/model_prices.json`
- Keys: `provider_id`/`model_id`
- Fields: `input_mtok`, `output_mtok`, `cache_read_mtok`, `cache_write_mtok`, `context_window`, `deprecated`
- Never hardcode prices

**Tooling**:
- Decision helper: `tools/model-prices/cost_gate.py` resolves `output_mtok` and returns `allow` (exit 0) or `delegate` (exit 3)
- Query/regenerate: `tools/model-prices/sync_model_prices.py` with `--query <model>`, `--list`, `--format {table,csv,json}`, `--check`, `--force`

**MCP boundary enforcement**: `mcp-cost-gate` middleware injects mandatory `caller_model` field into every MCP tool schema and refuses token-heavy calls from orchestrator-tier models. Fails open if price table unavailable.

**Setting `caller_model` correctly**: Pass the **actual id of the model issuing the call** — the running model's real price-table `model_id`, e.g. `claude-opus-5`, `claude-sonnet-5`, `claude-haiku-4-5`, `gpt-5.3-codex`, `gpt-5.6-terra`. Match a `model_id` key in `tools/model-prices/model_prices.json` (query with `sync_model_prices.py --list`).
- Do **not** pass a generic vendor or product label such as `github-copilot`, `copilot`, `openai`, or `anthropic`. An unrecognized `caller_model` resolves to a **zero-cost budget**, which silently breaks price-awareness enforcement (the gate can no longer distinguish orchestrator-tier from cheap models).
- When delegating, the sub-agent sets `caller_model` to **its own** model id, not the orchestrator's.

**Tier reference at X=15**: the canonical tier ladder — models, `in$/M · cread$/M · out$/M · ctx`, and per-tier usage — lives in [model-routing.instructions.md](model-routing.instructions.md). It is **not** restated here; a second copy is how the two tables drifted apart. Read it there before pinning a model.

Operating summary: T0 (Claude Opus 5) orchestrates and delegates. T1 (GPT-5.6 Terra, GPT-5.3-Codex) and below execute directly, since they sit at or under the threshold. **T2 (Claude Sonnet 5) is the default delegation target.** T3 (GPT-5 mini, GPT-5.6 Luna, GPT-5.4 mini) is the cheap worker band and the floor. Claude Sonnet 4.5 is deprecated for new routing — superseded by Claude Sonnet 5, which is cheaper on every axis at the same 1M window.

Never hardcode prices into tooling; re-resolve with `sync_model_prices.py --query <model>` when a routing decision depends on the exact price. See [model-prices.instructions.md](model-prices.instructions.md) for working with the table.

**Low-tier selection metric**: for T3 units, rank on `input_mtok` + `cache_read_mtok` and context window — **not** `output_mtok`. These units are input-heavy and output-tiny, so the output column barely touches the bill. Claude Haiku 4.5 (in $1/M, cache read $0.10/M, 200k) is the most expensive input option in the band with the smallest window; do not default to it.

**Roster check**: the price table is a vendor catalogue and lists models `runSubagent` will refuse. Pin only models present in the surface's model list — a rejected dispatch errors outright and wastes the spawn. See "Roster is not the catalogue" in [model-routing.instructions.md](model-routing.instructions.md).

## What Stays vs What to Delegate

**Keep on expensive model**:
- Strategic decisions and tradeoffs
- Decomposing task into small delegable units
- Planning sub-agent dispatch (which agent, which model, what order)
- Aggregating/reconciling/quality-checking results
- Deciding when done or when to escalate

**Delegate (never do directly when orchestrating)**:
- Reading/editing files
- Searching workspace or web
- Running commands/tests/builds/tool-call batches
- Summarizing large tool outputs or many artifacts

## Delegation Contract

Each sub-agent dispatch MUST include:

1. **Explicit cheaper model** at or below threshold X, chosen by tier
   - Format: `"Model Name (Vendor)"`, e.g. `"Claude Sonnet 5 (copilot)"`. The string must match the surface's model list exactly, punctuation included.
   - **Default: `"Claude Sonnet 5 (copilot)"` (T2).** Use it unless the unit justifies another tier.
   - Drop to T3 (`"GPT-5 mini (copilot)"` as the cheap-worker default and cost floor, `"GPT-5.6 Luna (copilot)"` when the input exceeds 400k or the unit must emit non-trivial code, `"GPT-5.4 mini (copilot)"` when it needs real reasoning over what it read) for bulk, mechanical, read-only triage, or judgement-free extraction units.
   - Confirm the input fits the target model's context window before dispatch; a truncation-driven re-dispatch costs more than the tier saves.
   - On a T3 failure, step up exactly one band (T3→T2) — never jump a cheap unit to T1/T0.
   - Climb to T1 (`"GPT-5.3-Codex (copilot)"` for heavy code generation, `"GPT-5.6 Terra (copilot)"` for very large context) only after a T2 attempt was wrong or too shallow, or for plainly cross-cutting high-risk work — and record why.
   - Under budget pressure, shift every unit down one tier.
   - Do not route new work to `"Claude Sonnet 4.5 (copilot)"`; Claude Sonnet 5 is cheaper on input, output, and cache read at the same 1M context window.
   - Do not pin `"Auto (copilot)"` — it delegates model selection away from you and defeats cost-aware routing.
   - Never delegate to another orchestrator-tier model
   - When multiple eligible models are equal in cost, prefer the latest model version or generation, then the larger context window
2. **Single well-scoped objective** — one unit per sub-agent, never the whole task
3. **Compact return contract** — ask for exactly the facts/edits/results needed (file paths, line ranges, diff summary, decision, short findings list), not a transcript
   - Suggested shape: `scope | finding | outcome | blocker | pointer`
4. **Shared context bundle** — pass resolved artifact CONTENT inline, not just ids/paths
   - Resolved tickets: full TOML + description markdown, not just ticket id
   - Resolved specs: full body + sections, not just spec id/slug
   - Handoff package: complete JSON, not just a reference
   - Relevant file skeletons: bounded interface-level view, not "read it yourself"
   - Validation commands: exact command list, not "figure out what to run"
   - For parallel siblings: compute shared prefix ONCE, duplicate into each prompt
   - Size target: 2k-5k tokens per bundle
   - See `.agents/instructions/orchestration/shared-context-bundle.instructions.md`
5. **Workspace agent template only** — dispatch to a workspace `.agents/agents/*.agent.md` template (e.g. Research Agent, Implement Agent, Explore Agent); never dispatch to a VS Code built-in agent (e.g. the built-in Explore), which is not integrated with our MCP toolset. For read-only probes use the workspace **Explore Agent** template.

## Context Isolation

**The single most important delegation rule**: A sub-agent inherits NONE of the current session's context. No conversation history, no prior findings, no shared "we". Context-dependent prompts do not fail loudly — they burn a full agent spawn to reply "I have no prior context."

**Pre-dispatch checklist** (every sub-agent prompt MUST be self-contained):
- Pass FULL CONTENT of artifacts via context bundle, not just ids/paths
- Include the target agent's contract excerpt inline (do not make sub-agent read its own template)
- Name every file with full workspace-relative path (never "the file we discussed")
- Paste exact snippet, error, or scope sub-agent must act on
- State repository root and any command/cwd assumptions
- Define every referent — no "this", "that fix", or "the earlier change"
- State exact return shape you want back

## Pre-Dispatch Quality Gates

Run pre-dispatch gates for EVERY delegation. Each delegation class (Implement, Review, Testing, Commit) has its own gate set. See `.agents/instructions/orchestration/pre-dispatch-gates.instructions.md` for complete definitions.

Gate failures are RE-PLAN signals: fix the precondition BEFORE dispatch, never re-dispatch a blocked unit and hope it works.

## Required Workflow

5-step orchestration loop:

1. **Plan** — ordered delegable units + done-criteria + dependencies
2. **Dispatch** — sequential when dependent, batch when independent
3. **Aggregate** — collect compact results, reconcile conflicts, fill gaps with follow-up units, keep running synthesis
4. **Verify** — confirm acceptance criteria; delegate validation and read the verdict
5. **Report or escalate** — escalate to user only on genuine ambiguity/conflicting evidence after focused delegation

## When NOT to Delegate

**The floor**: Each sub-agent is a full agent loop with real spawn overhead. Delegating a single bounded read (one small file window, one grep) costs MORE than doing it inline.

**Rule**: Delegate only when the subtask is bulky, numerous, or context-heavy. **Bulk, not trivial.** Over-delegation is its own token bonfire.

## Case → Capability-Role → Cost-Class Mapping

Define work allocation using **capability roles** rather than raw prices, where each role maps to a cost-class band in the price table.

### Three Capability-Role Bands

| Role | Purpose | Cost Class | Allocation Strategy |
|------|---------|------------|---------------------|
| **Reasoner** | One-time context capture, strategic decisions, high-level reasoning, cross-cutting tradeoffs | T0 (orchestrator tier) | Use sparingly — only for planning and decomposition |
| **Sequencer** | Decompose and sequence implementation steps, moderate-complexity editing, cross-file coordination | T1-T2 | Default delegation target (T2 = Claude Sonnet 5) |
| **Executor** | Step execution, mechanical edits, read-only triage, interaction with external systems, unforeseen error recovery, large-data handling | T3 | Use liberally for bulk/routine work |

**Allocation principle**: Use **Executors** (small models) as much as possible; reserve **Reasoners** for one-time strategic work; use **Sequencers** (mid-tier) as the default implementation workhorse.

### Work Case Classification

Each case maps to a capability role based on **driving signals**: scope breadth, data volume, error-recovery need, and reasoning depth.

| Work Case | Role | Cost Class | Driving Signal | Example Models |
|-----------|------|------------|----------------|----------------|
| **Strategic planning / decomposition** | Reasoner | T0 | Cross-cutting scope, requires full context once | Claude Opus 5 |
| **Initial task breakdown** | Reasoner | T0 | Must understand entire problem space | Claude Opus 4.8 |
| **Quality synthesis / conflict resolution** | Reasoner | T0 | Reconciling contradictory findings | Claude Opus 5 |
| **Multi-file feature implementation** | Sequencer | T2 (default) | Moderate scope, needs code reasoning | Claude Sonnet 5 |
| **Bug fix with diagnosis** | Sequencer | T2 | Requires tracing causal chain | Claude Sonnet 5 |
| **Cross-crate refactor** | Sequencer | T1 | Large scope, high risk | GPT-5.6 Terra, GPT-5.3-Codex |
| **Test authoring** | Sequencer | T2 | Needs understanding of behavior | Claude Sonnet 5 |
| **Targeted single-file edit** | Executor | T3 | Narrow scope, clear target | GPT-5 mini, GPT-5.4 mini |
| **Read-only search / grep** | Executor | T3 | Mechanical extraction | GPT-5 mini |
| **Docs generation from template** | Executor | T3 | Formula-driven, low judgement | GPT-5 mini |
| **Run validation command** | Executor | T3 | Execute + summarize result | GPT-5.4 mini |
| **Bulk file reads for triage** | Executor | T3 | High data volume, low reasoning | GPT-5.6 Luna (large context) |
| **Error log summarization** | Executor | T3 | Extract failure pattern | GPT-5 mini, GPT-5.6 Luna if the log exceeds 400k |
| **Retry after failed validation** | Executor | T3 | Unforeseen event recovery | GPT-5.4 mini |

**Cost-class boundaries**: All tiers resolve through `tools/model-prices/model_prices.json` `output_mtok` field. Never hardcode prices. Query with:
```bash
./tools/model-prices/sync_model_prices.py --query <model-id> --format table
```

**When a case is ambiguous**: Start one tier lower than your intuition suggests, then step up **exactly one band** if the result is insufficient. T3→T2→T1→T0. Never skip tiers.

**Tier-step policy**: After a failed attempt, step up by exactly one tier and record the reason in your planning notes. If T3 produces a shallow answer, retry with T2 (not T0). If T2 fails, only then escalate to T1.

### Driving Signals Detail

| Signal | Reasoner (T0) | Sequencer (T1-T2) | Executor (T3) |
|--------|---------------|-------------------|------------------|
| **Scope breadth** | Cross-cutting, entire subsystem | Multi-file feature slice | Single file or bounded target |
| **Data volume** | Must digest full context once | Moderate (5-15 files) | High volume but mechanical, or tiny focused scope |
| **Reasoning depth** | Strategic tradeoffs, architectural decisions | Causal chains, behavior inference | Pattern matching, formula application |
| **Error-recovery need** | Reconcile conflicting evidence | Diagnose root cause | Retry with fixed input, summarize logs |
| **Risk tolerance** | High-risk cross-cutting change | Moderate risk with focused tests | Low-risk mechanical change or read-only |

**Cost class band reference** (from `tools/model-prices/model_prices.json`):
- **T0 Reasoner**: `output_mtok` strictly > 15 (orchestrator threshold X)
- **T1 Sequencer (high)**: `output_mtok` 14-15 (at threshold)
- **T2 Sequencer (default)**: `output_mtok` 8-12
- **T3 Executor (floor)**: `output_mtok` 2-6

Note that the T3 band is selected on `input_mtok` + `cache_read_mtok`, not on the `output_mtok` range shown here; the output range is descriptive only.

These bands are not normative tiers to maintain — they are **descriptive observations** of the current price table at the time this instruction was written. The authoritative mapping is: resolve the model's `output_mtok` from `tools/model-prices/model_prices.json`, then compare to threshold X (currently 15). The capability-role bands provide semantic guidance for **which class of work** should target **which output cost range**, but the exact model-to-tier membership will shift as prices change.

## Verify Sub-Agent Output

- Treat every sub-agent summary as an **UNVERIFIED claim** — sub-agents hallucinate
- Spot-check load-bearing findings against ground truth (real grep, `--check` run, bounded read) BEFORE any finding drives an edit or decision
- Reasoning over summary is fine; trusting it blindly is not

## Parallel Fan-Out

**Independent READ-ONLY probes** can be dispatched concurrently in a single block, then reasoned over as merged results — the highest-throughput pattern.

**Good targets**:
- Survey N files/crates at once
- Run several independent searches
- Gather evidence from multiple subsystems in parallel

**Template**: route every fan-out probe through the workspace **Explore Agent** template (`.agents/agents/explore.agent.md`), never the VS Code built-in Explore, so probes keep MCP access.

**Constraint**: Keep fan-out read-only; do not parallelize writes to overlapping scope. Each parallel prompt must still be independently self-contained.

## Failure Path

- If sub-agent errors, returns empty, refuses, or says it lacks context: **retry ONCE** with more self-contained prompt (usual cause: context isolation)
- If still fails: do subtask inline and record failure as one-line finding
- Escalate subtask UP a tier only for quality insufficiency (wrong or too-shallow answer), and record why
