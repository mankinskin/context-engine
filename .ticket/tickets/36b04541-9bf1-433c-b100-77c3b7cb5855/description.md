## Problem

A sub-agent's run was interrupted. When the orchestrator was told to continue, it assumed the sub-agent had finished and moved on to the next step, silently dropping the unfinished work.

Graceful handoff→resume is already covered (.agents/prompts/handoff.prompt.md, `session_runtime_resume` with `predecessor_run_id`). Nothing covers an ABRUPT interruption where no handoff was ever written. loop-closure, escalation-gate, and phase-separation all address planned transitions only.

## Correct behaviour

The next agent must find the loose ends and resume at the right point — never assume an interrupted part is complete, and never restart from a rough boundary.

## Decisions (interview-resolved)

- Author a **standalone** instruction file (e.g. .agents/instructions/orchestration/interruption-recovery.instructions.md), not a section inside loop-closure.
- Detection signal: **stale board entries only**. Do not use workflow-node status or transcript-without-handoff heuristics.
- Do NOT add a staleness threshold for durable session runtimes.
- Recovery **auto-reconstructs a draft** handoff-equivalent, which a human then confirms.
- Author a dedicated prompt template for resuming an interrupted agent.