## Problem

Guidance across the repo still implied that creating or editing agent-customization files (`.agents/instructions/*.instructions.md`, `.agents/prompts/*.prompt.md`, `.agents/agents/*.agent.md`, `.agents/skills/**`) plus root `AGENTS.md` and `.github/copilot-instructions.md` must be routed through the rule system (`rule create` / `rule sync-targets`). In reality the `rule-targets` configs no longer generate these files — they are hand-owned. The stale guidance and stale generator state caused (a) agents to needlessly create rule entries, and (b) the pre-commit `rule sync-targets --check` gate to fail on orphaned generated-target state.

## Resolution (this run)

### AC1 — generation status of AGENTS.md / copilot-instructions.md
Verified NOT rule-generated: no `rule-targets` target produces them (the `--check` listed `context-engine-agents -> AGENTS.md` and `context-engine-copilot-instructions -> copilot-instructions.md` only as *stale removed-from-config* records) and neither file carries the `<!-- rule-api:file generated=true -->` marker. Corrected the "Rule-generated files" table, pre-commit trigger table, the "Resolving pre-commit failures" example, and the "Rule-managed file workflow" example in `.agents/instructions/commit.instructions.md`; extended the hand-owned note to cover root `AGENTS.md` + `.github/copilot-instructions.md`.

### AC2 — surrounding guidance cleanup
- `.agents/skills/README.md`: already correct (hand-owned, never generated) — no change needed.
- `.rule/README.md`: generated rule-*catalog* (lists rule entries); carries no routing-through-rule guidance for new agent files — no change needed.
- `.agents/agents/commit.agent.md` and `.agents/prompts/commit.prompt.md`: corrected the "never edit rule-managed files (AGENTS.md, copilot, .agents/**)" constraints to say those surfaces are hand-owned and edited directly; still-generated outputs (`.clinerules/**`, submodule READMEs) regenerate via `rule sync-targets`.
- Final grep across `.agents/**` + `.rule/README.md` for routing-through-rule language on instruction/skill/prompt/agent authoring: empty.

### AC3 — no config targets `.agents/**`
Confirmed for root and all submodule configs: `grep` for `.agents/`, `copilot-instructions`, and AGENTS.md output paths across `rule-targets.yaml` + `rule-targets/` (root, memory-api, memory-viewers, viewer-api) returns NONE. Submodule `--check` runs all pass.

### AC4 — pre-commit `rule sync-targets --check` no longer flags hand-owned files
Two-part durable fix:
1. `.githooks/pre-commit` root trigger narrowed to `^(rule-targets\.yaml|\.rule/.*|\.clinerules/.*|\.github/README\.md)$` — staging hand-owned `AGENTS.md`, `copilot-instructions.md`, or `.agents/instructions/*` no longer invokes the drift gate.
2. Tool fix in `memory-api/tools/cli/rule-cli/src/cli/rendering.rs`: `sync_targets_payload` now partitions stale (removed-from-config) records — a record is only an *orphaned generated artifact* (fails `--check`, file removed on real sync) when its output file exists AND still starts with `GENERATED_FILE_COMMENT`. Decoupled records (output migrated to a marker-free hand-owned file, or already deleted) are pruned from tracking state without touching the file and never fail `--check`. This makes the documented `rule sync-targets` remediation self-heal instead of erroring on `refusing to remove non-generated file AGENTS.md`.

Local self-heal executed: root `--check` now exits 0; a real `sync-targets` pruned 41 orphaned generated-target state records (`.rule/entities/` 161→120, all decoupled) with zero hand-owned file deletions.

## Validation

- `cargo test -p rule-cli`: 48 passed / 0 failed. Added `sync_targets_prunes_decoupled_hand_owned_outputs_without_deleting_them`; existing `sync_targets_prunes_removed_outputs_from_previous_sync` (marker-carrying removal) still green.
- Repaired `repo_spec_prompt_target_matches_expectation_oriented_contract` — it invoked `GenerateTarget` for the retired `context-engine-prompt-spec` target (removed by the sibling prompt-migration); trimmed to keep the hand-owned content guard on `.agents/prompts/spec.prompt.md`.
- Root + all 3 submodule `rule sync-targets --check`: exit 0.

## Notes

Extends spec a9b7ef39 (AC5: migrate instruction files off rule-generated to hand-owned) to the surrounding guidance and the generator-state/tooling tail so agents stop creating rule entries for hand-owned files and the drift gate stays green. The many concurrently-modified `.agents/prompts/*` and `.agents/agents/*` files are the sibling marker-stripping migration (tickets 76d0ace3 / 16cfd19f) — background activity, not owned by this ticket.