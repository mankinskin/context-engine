## Problem

Session 51701334 delegated effectively at the top level (orchestrator ~$0.90) but sub-agents thrashed, driving total cost to ~$9. Four design gaps in the current orchestration guidance (`.agents/instructions/orchestration/`) let this happen:

### 1. No enforced task-difficulty floor for delegation
A **3-line edit** (one word + two relative links) was delegated to a GPT-5 mini sub-agent and cost **20.1 minutes** (events.json L80467) — and still shipped a grammar bug. `model-routing.instructions.md` has a "When NOT to Delegate" floor but it is advisory and was ignored. Trivial single-file edits must be done inline by the orchestrator.

### 2. "Verify sub-agent output" is not enforced
Sub-agents repeatedly reported PASS while shipping defects: a wrong-work spawn that only edited an AGENTS.md pointer and did no split (redo required); a rule-cleanup spawn that missed 10 generated-target entities (follow-up spawn required); broken links + a grammar bug caught only at final audit. The redo cost came from trusting unverified summaries.

### 3. No timeout/hang playbook
`rule scan` hung and hit the 5-min timeout cap eight times before an agent switched to `cargo run`. Nothing instructs "one timeout -> change approach."

### 4. No sanctioned cheap validation primitive
Multiple sub-agents lost minutes to fragile ad-hoc bash/heredoc/Python validators (exit 127, Windows `\r`, back-grounded runs). Each reinvented and broke its own checker.

## Additional observed gaps
- Cost-gate dead-end: three `spec_create` rejections sent an agent on a build-the-CLI detour (no fast delegate/grant escalation).
- Bootstrap fragility: the orchestrator's own `read_file` of session-bootstrap failed (events.json L563) and `render-instructions` failed on a stale pin, yet the session proceeded blind.
- An action-7 sub-agent committed (`42427877`) without authorization, triggering a full Audit spawn.

## Goal
Amend orchestration instructions with: an enforced delegation floor (min task size/scope to justify a spawn), a mandatory verify-before-accept step for sub-agent summaries that drive edits, a one-strike timeout/hang rule, and a pointer to a sanctioned validation primitive instead of ad-hoc shell.

## Acceptance criteria
- orchestration/model-routing + orchestrator-delegation encode a concrete delegation floor and verify-output gate.
- A timeout/hang playbook exists and is referenced from tool-output guidance.
- Guidance points agents to a stable validation primitive rather than hand-rolled shell.

## Evidence
Session 51701334 events.json L563, L80467, L44080, L25664/L37503/L37995/L38711/L42579/L77293/L77820/L77999; transcript seq 49-66, 92-101, 110-121.