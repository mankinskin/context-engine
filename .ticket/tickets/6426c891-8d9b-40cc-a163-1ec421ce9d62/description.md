## Problem
Guidance files under .agents/instructions/** have grown organically; sub-agents pay a token tax on every call reading noisy, possibly outdated, or over-specific rules. There is no repeatable process to prune them.

## Goal
Add a Simplify Agent (agent mode or skill) that:
1. Scans .agents/instructions/** (and related AGENTS.md/copilot-instructions.md) and builds a flat list of discrete rules grouped by category.
2. Interviews the user per rule group: is it still accurate, is it too specific to a past bug, could a linter/compiler enforce it instead of an LLM instruction.
3. Rewrites accepted rules as concise imperative statements, removing fluff and philosophical framing, and records rejected/superseded rules.

## Acceptance criteria
- New agent/skill definition exists and is invocable.
- Running it against the current instructions corpus produces a rule graph (categories + discrete rule list) and a condensed rewrite proposal for at least one instruction file, driven by an interview loop.
- Rejected or merged rules are recorded (not silently dropped).

## Source
Derived from AGENT_WORKFLOW_OPTIMIZATIONS.md conversation, "Step 1: The First Action".