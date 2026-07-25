---
description: "Use when operating as an orchestrator or when the running model's output cost exceeds the delegation threshold. Covers cost gating, sub-agent delegation contract, context isolation, and when to delegate vs execute directly."
applyTo: "**"
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

**Tier reference at X=15**:

| Tier | out$/M range | Mode |
|------|--------------|------|
| Opus-tier | 25–75 | Orchestrate/delegate |
| o3 | 40 | Orchestrate/delegate |
| Sonnet | 15 | Execute directly (at threshold) |
| GPT-5, Gemini Pro | 10 | Execute directly |
| Haiku, Flash, mini | <10 | Execute directly |

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

1. **Explicit cheaper model** at or below threshold X (Sonnet, GPT-5, Gemini Pro, Haiku, Flash, mini)
   - Format: `"Model Name (Vendor)"`, e.g. `"Claude Haiku 4.5 (copilot)"`
   - Never delegate to another orchestrator-tier model
2. **Single well-scoped objective** — one unit per sub-agent, never the whole task
3. **Compact return contract** — ask for exactly the facts/edits/results needed (file paths, line ranges, diff summary, decision, short findings list), not a transcript
   - Suggested shape: `scope | finding | outcome | blocker | pointer`
4. **Minimum context** — pass anchors (full workspace-relative paths, ticket/spec ids, prior findings) so sub-agent does not re-discover

## Context Isolation

**The single most important delegation rule**: A sub-agent inherits NONE of the current session's context. No conversation history, no prior findings, no shared "we". Context-dependent prompts do not fail loudly — they burn a full agent spawn to reply "I have no prior context."

**Pre-dispatch checklist** (every sub-agent prompt MUST be self-contained):
- Name every file with full workspace-relative path (never "the file we discussed")
- Paste exact snippet, error, or scope sub-agent must act on
- State repository root and any command/cwd assumptions
- Define every referent — no "this", "that fix", or "the earlier change"
- State exact return shape you want back

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

**Constraint**: Keep fan-out read-only; do not parallelize writes to overlapping scope. Each parallel prompt must still be independently self-contained.

## Failure Path

- If sub-agent errors, returns empty, refuses, or says it lacks context: **retry ONCE** with more self-contained prompt (usual cause: context isolation)
- If still fails: do subtask inline and record failure as one-line finding
- Escalate subtask UP a tier only for quality insufficiency (wrong or too-shallow answer), and record why
