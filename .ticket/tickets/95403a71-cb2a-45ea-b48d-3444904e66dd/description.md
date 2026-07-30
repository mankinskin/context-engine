## Problem
question-quality.instructions.md line 16 already bans ambiguous pronouns, but only for interview questions. General agent chat responses still use "this", "that", "the engine" without establishing which entity is meant, making transcripts hard to follow without re-reading context.

## Goal
Author a new dedicated instructions file (e.g. .agents/instructions/orchestration/entity-disambiguation.instructions.md) defining a general-purpose protocol for all agent responses:
- First mention of an entity: full establishment (type, fully-qualified name/id, short summary or tags).
- Subsequent mentions within context proximity (same turn/recent turns): short id or name only is fine.
- Context reset (switching file/step/task): re-establish using the first-mention format.
- No bare pronoun references to entities ever.
Also update question-quality.instructions.md to reference/defer to the new file so the two are not duplicative, and update AGENTS.md's Clickable Reference Policy cross-reference if needed.

## Acceptance criteria
- New instructions file exists with the three-tier protocol (first mention / proximity / reset) and concrete examples.
- question-quality.instructions.md cross-references it instead of restating the general rule.
- Rule is scoped so agent modes beyond Interview Agent pick it up (verify applyTo/scope).

## Source
Derived from AGENT_WORKFLOW_OPTIMIZATIONS.md conversation, "Step 3: Fixing Entity Ambiguity & Level of Detail". User decision: create new dedicated file AND update question-quality.instructions.md.
## Review verdict: pass

**Files changed (this unit only):**
- NEW: .agents/instructions/orchestration/entity-disambiguation.instructions.md (three-tier protocol + bare-pronoun ban, 8 concrete bad/good examples)
- MODIFIED: .agents/instructions/orchestration/question-quality.instructions.md (item 2 now cross-references the new file instead of restating the rule)

**AC verdicts:**
1. New file specifies all three tiers (first mention/proximity/context reset) plus the bare-pronoun ban, each with a concrete bad/good example pair. ~40 lines (not ~23 as estimated), actionable — met.
2. question-quality.instructions.md item 2 links to entity-disambiguation.instructions.md and does not restate the rule — met. No conflict with AGENTS.md's Clickable Reference Policy: the new file explicitly defers ("link format itself is owned by the Clickable Reference Policy in AGENTS.md") and does not restate viewer/manifest/description link-format rules — met.
3. Scope check: no sibling file in .agents/instructions/orchestration/ (model-routing, phase-separation, escalation-gate, etc.) uses `applyTo` frontmatter; all rely on description-based relevance loading, which is the convention this repo's agent harness uses to surface org-wide instructions to any mode. The new file's description ("Use in every agent response, not only interview questions...") matches this convention exactly — met, no fix required.

**Pre-existing validation failures confirmed unrelated (reproduced independently):**
- `rtk cargo run -p rule-cli --bin rule -- sync-targets --config rule-targets.yaml --check` → `storage error: workspace not initialized at rule-targets/../memory-api\.spec`. Root cause is an uninitialized spec workspace under memory-api, unrelated to either changed instructions file.
- `rtk ticket store-index --check` → .ticket/README.md, .ticket/index.toon, .agents/ticket-catalog.md out of date. Root cause is ticket-store churn from unrelated tickets (multiple untracked ticket folders visible in `rtk git status`), not from editing instructions files.

**Unintended files:** `rtk git status` shows only question-quality.instructions.md (modified) and entity-disambiguation.instructions.md (new) attributable to this ticket; all other working-tree changes (other ticket folders, session folders, memory-api submodule pointer, deleted swarm-worker.prompt.md) are unrelated background activity from other agents and were left untouched.

No fix needed — no defect found. State set to in-review; done requires separate human approval.