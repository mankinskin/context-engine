## Problem
After C2 lands the 9 new templates, the 17 superseded `.agent.md` files and the folded-in `.prompt.md` files remain, duplicating guidance and re-introducing the exact collisions this epic removes.

## Scope
- Remove the 17 superseded `.agents/agents/*.agent.md` files once their content is confirmed folded into the C2 templates.
- Remove/merge the folded `.agents/prompts/*.prompt.md` files: audit.prompt.md, spec.prompt.md, interview.prompt.md, ticket.prompt.md, tickets.prompt.md, implement.prompt.md, next.prompt.md, ticket-next.prompt.md, debug-test.prompt.md, tdd.prompt.md, reviews.prompt.md, iteration.prompt.md, handoff.prompt.md, handoff-tickets.prompt.md, commit.prompt.md, rule.prompt.md, rule-target.prompt.md.
- Risk (a) from epic: merging a `.prompt.md` into an `.agent.md` removes a slash-command surface. For each prompt file, verify whether its slash-command entry point is load-bearing (referenced in workflows, docs, or user habits); if load-bearing, keep a thin passthrough prompt that delegates to the new agent template instead of deleting outright.
- KEEP UNTOUCHED (do not touch): default.agent.md, transcription.agent.md, transform-transcript.prompt.md, memory-setup.prompt.md, user-training.prompt.md, sync-model-prices.prompt.md, build-validate-tools.prompt.md, tool-grant-regression-probe.prompt.md.

## Affected paths
- .agents/agents/*.agent.md (17 removals, excluding KEEP-UNTOUCHED list)
- .agents/prompts/*.prompt.md (16 removals or thin-passthrough rewrites)

## Acceptance criteria
- [ ] All 17 superseded agent templates removed (except KEEP-UNTOUCHED files)
- [ ] Each folded prompt file removed, or replaced with a thin passthrough documented as load-bearing
- [ ] No dangling references to removed files remain in AGENTS.md, other templates, or docs
- [ ] KEEP-UNTOUCHED list left byte-identical
