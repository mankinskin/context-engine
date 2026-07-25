Ticket 14c0995c (successor tail of AC5) decoupled the *guidance* and *generator-state/tooling* from the rule system after the file migration.

Guidance corrections (hand-owned authoring is now the documented path):
- `.agents/instructions/commit.instructions.md`: removed `AGENTS.md`/`.github/copilot-instructions.md` from the "Rule-generated files" table and the pre-commit trigger table; fixed the remediation + workflow `git add` examples to still-generated outputs (`.clinerules/10-core-rules.md`); extended the hand-owned note to cover root `AGENTS.md` + `.github/copilot-instructions.md`.
- `.agents/agents/commit.agent.md` and `.agents/prompts/commit.prompt.md`: reworded the "never edit rule-managed files" constraints so `AGENTS.md`, `.github/copilot-instructions.md`, and everything under `.agents/**` are edited directly.
- `.agents/skills/README.md` and `.rule/README.md` needed no change (skills README already states hand-owned; `.rule/README.md` is a generated rule *catalog* with no routing guidance).

Verification:
- No `rule-targets` config (root, memory-api, memory-viewers, viewer-api) outputs `.agents/**`, `AGENTS.md`, or `.github/copilot-instructions.md`; neither root file carries the generated marker.

Pre-commit gate (AC4) — durable two-part fix:
- `.githooks/pre-commit` root trigger narrowed to `^(rule-targets\.yaml|\.rule/.*|\.clinerules/.*|\.github/README\.md)$` so staging hand-owned files no longer invokes the drift gate.
- `rule-cli` `sync_targets_payload` now classifies stale (removed-from-config) records: only outputs that still exist AND start with `GENERATED_FILE_COMMENT` are treated as orphaned generated artifacts (fail `--check`, removed on real sync); decoupled outputs (migrated to marker-free hand-owned files, or already deleted) are pruned from tracking state without file removal and never fail `--check`. Fixes the prior `refusing to remove non-generated file AGENTS.md` deadlock so the documented `rule sync-targets` remediation self-heals. Local run pruned 41 orphaned records (`.rule/entities/` 161→120) with zero hand-owned file deletions; root + 3 submodule `--check` now exit 0.

Validation: `cargo test -p rule-cli` 48 passed / 0 failed (added `sync_targets_prunes_decoupled_hand_owned_outputs_without_deleting_them`; repaired `repo_spec_prompt_target_matches_expectation_oriented_contract` which invoked the retired `context-engine-prompt-spec` target). Evidence: test-api `exec-rule-cli-decouple-agents-20260725`.

Note: ticket 14c0995c stays in `new` — state transitions remain blocked by the pre-existing `no schema for type 'task'` store loader issue (same as PROMPTS/AGENTS-MIG); completion recorded in the ticket description and here.