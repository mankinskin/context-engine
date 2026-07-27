<!-- spec-index:tree-entry generated=true -->
<!-- spec-index:entry id=39983ddf-1f7e-4081-a060-6b8258eb4c41 slug=model-prices/price-awareness-orchestration digest=1c66ce992d34 -->

# Model price awareness: orchestrator-mode enforcement across guidance, tool wrapper, and agent

- slug: `model-prices/price-awareness-orchestration`
- component: model-prices
- state: draft
- index_ref: `.spec/specs/39983ddf-1f7e-4081-a060-6b8258eb4c41/spec.toml`

## Summary

"Context stack price awareness" makes an agent switch into **orchestrator mode** based on the cost of its own underlying model, so expensive models are reserved for strategic decisions, code/change p…

## Acceptance Criteria Excerpt

[x] Model→cost mapping derivable from `model_prices.json`, referenced (not duplicated) by guidance. [x] AGENTS.md contains an explicit orchestrator-mode rule keyed to `output_mtok > X` with `X = 15`. [x] Threshold field + value recorded in T-PARENT and the transcript. [ ] All MC…

## Navigation

- Parent: _(root)_
- Children: _(none)_
