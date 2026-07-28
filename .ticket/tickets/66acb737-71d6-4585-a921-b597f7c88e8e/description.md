## Implementation summary

Added a `model:` frontmatter field (bare model_id, no vendor suffix) to all 16 `.agents/agents/*.agent.md` templates:

- `orchestrator.agent.md` → Claude Opus 5 (T0)
- `default.agent.md`, `implement.agent.md`, `audit.agent.md`, `handoff.agent.md`, `interview.agent.md`, `iteration.agent.md`, `review.agent.md`, `roast.agent.md`, `spec.agent.md`, `testing.agent.md`, `ticket-refinement.agent.md` → Claude Sonnet 5 (T2, default)
- `explore.agent.md`, `research.agent.md`, `commit.agent.md` → GPT-5 mini (T3) — satisfies AC3
- `transcription.agent.md` → GPT-5.4 mini (T3, bulk transform needing real reasoning)

Documented the full `model:` contract (schema, AC2 resolution rule, AC4 override-audit rule, per-template tier table) in [.agents/instructions/orchestration/model-routing.instructions.md](../../.agents/instructions/orchestration/model-routing.instructions.md) under a new "Per-Template `model:` Declaration" section.

## Spec-vs-instruction decision

Spec ec3b13f1 explicitly excludes the `model:` contract as a non-goal. No new spec was created: this instruction file is the sufficient contract surface because `model:` is a routing default with one producer (template loader) and one consumer (`runSubagent`'s no-`model` path), fully checked by the grep/name-match validation below rather than needing acceptance-criteria-driven product testing. Promote to a spec later only if resolution/override mechanics need code-level enforcement.

## AC status

- AC1: MET — `rtk git grep -n "^model:" .agents/agents` returns 16/16 templates.
- AC2: MET (documented) — resolution rule specified in model-routing.instructions.md: no explicit `model` on runSubagent falls back to the template's declared `model:`.
- AC3: MET — Explore, Research, Commit all declare GPT-5 mini, cheaper than Sonnet 4.5 (and Sonnet 5) on every priced axis.
- AC4: MET (documented) — override-audit rule specified: any override to a model above the declared tier requires a one-line reason in the session record; downward overrides need no justification.
- AC5: DEFERRED-pending-10d21210 — the synthetic benchmark ticket 10d21210 is not yet built, so the "model distribution across delegations is no longer uniform" measurement cannot be run without fabricating numbers. The structural precondition (non-uniform per-template tiers, mechanical classes on T3) is in place.

## Validation

- `rtk git grep -n "^model:" .agents/agents | wc -l` → 16 (matches template count).
- Each declared value (`Claude Opus 5`, `Claude Sonnet 5`, `GPT-5 mini`, `GPT-5.4 mini`) confirmed present as a `model_id` in tools/model-prices/model_prices.json via targeted greps.
- No Rust dispatch/loader code was touched (config/docs-only change) — `cargo test --all --lib` does not apply.

Do NOT start 10d21210 or 77eb143b from this ticket.