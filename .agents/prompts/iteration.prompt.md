---
description: "Orchestrate the Review → Interview → Commit → Handoff transition for a finished implementation track, enforce gates, and produce the next-handoff package."
name: "iteration"
argument-hint: "[ticket-id|current]"
agent: "agent"
---

# Iteration

Use this workflow to orchestrate the Review → Interview → Commit → Handoff transition after an implementation phase completes.

Reference [AGENTS](../../AGENTS.md), [Iteration Loop Workflow spec](.spec/specs/b71658f1-8de2-444a-9be1-64b1d8ecce70/spec.toml), [Handoff Package Schema spec](.spec/specs/5e52039d-aabc-434d-bdf3-eca63e312476/spec.toml), [ticket-cli](../../memory-api/tools/cli/ticket-cli/README.md), [ticket-mcp](../../memory-api/tools/mcp/ticket-mcp/README.md), [spec-cli](../../memory-api/tools/cli/spec-cli/README.md), [spec-mcp](../../memory-api/tools/mcp/spec-mcp/README.md), and [session-mcp](../../context-stack/tools/mcp/session-mcp/README.md).

Act as a thin orchestrator: delegate Review, Interview, Commit, and Handoff to their named agents, enforce gates, and author the next-handoff package inline when a review fails.

## Workflow

1. **Read the slash-command text** and determine the implementation track to iterate (ticket id, current session, or handoff package).
2. **Anchor on the track.** Read the target ticket(s), current session state, or handoff package. Confirm the implementation phase is complete and validation has passed.
3. **Delegate Review phase.** Use the agent tool to invoke the [Review Agent](.agents/agents/review.agent.md) with the target ticket(s). Instruct it to verify acceptance criteria, gather evidence, and return a pass/fail verdict with findings. Use an explicit cheaper model at or below the X=15 threshold (e.g., "Claude Sonnet 4.5 (copilot)").
4. **Enforce review gate.**
   - If review passes, proceed to step 5.
   - If review fails, **author the next-handoff inline** (do not delegate re-packaging to the Handoff Agent). The re-packaged handoff must satisfy the [handoff-package schema](.spec/specs/5e52039d-aabc-434d-bdf3-eca63e312476/spec.toml):
     - **objective** — the single goal of the next implementation unit
     - **target_tickets** — ticket ids with current state and acceptance criteria inlined
     - **target_files** — explicit workspace-relative paths expected to be touched
     - **decisions** — resolved design choices
     - **validation** — exact commands/checks that prove the unit done
     - **non_goals** — explicit out-of-scope boundaries
     - **context_anchors** — prior findings, links, and ids needed so no search is required
     - **open_escalations** — must be empty for a package to be implementation-ready (record review findings here if they require user input)
   - Move the ticket to `in-implementation` with `mcp_ticket-mcp_update_ticket` or `ticket update <id> --to-state in-implementation`.
   - Persist the handoff via `session_handoff` with `mcp_session-mcp_session_handoff` (or the session-cli equivalent).
   - Stop; the iteration is complete. The next implementation session will load this re-packaged handoff.
5. **Delegate Interview phase.** Use the agent tool to invoke the [Interview Agent](.agents/agents/interview.agent.md) to resolve any remaining open questions or escalations. Use an explicit cheaper model. Collect clarifications and update tickets/specs as needed.
6. **Enforce escalation gate.** Confirm all escalations are resolved. If any remain, stop and escalate to the user — no ticket may reach `done` while an unresolved user escalation exists.
7. **Delegate Commit phase.** Use the agent tool to invoke the [Commit Agent](.agents/agents/commit.agent.md) to commit the approved work (hooks, rule sync, generated files, submodule pointers, conventional messages). Use an explicit cheaper model.
8. **Delegate Handoff phase.** Use the agent tool to invoke the [Handoff Agent](.agents/agents/handoff.agent.md) to author the forward next-handoff package for the next implementation unit. Use an explicit cheaper model. The Handoff Agent produces the handoff satisfying the [handoff-package schema](.spec/specs/5e52039d-aabc-434d-bdf3-eca63e312476/spec.toml).
9. **Enforce loop-closure gate.** Confirm the ticket is transitioned (closed or returned to `in-implementation`) and a handoff package exists (either the forward handoff from step 8 or the re-packaged handoff from step 4).
10. **Persist the handoff.** Use `mcp_session-mcp_session_handoff` (or the session-cli equivalent) to persist the handoff record.

## Gates

- **Review gate (step 4):** acceptance criteria verified before commit; failures return the ticket to `in-implementation`.
- **Escalation gate (step 6):** no ticket reaches `done` while an unresolved user escalation exists.
- **Loop-closure gate (step 9):** every finished implementation terminates in a durable handoff package plus a ticket transition (closed or returned).

## Re-packaging Rule

When a review fails (step 4), the **Iteration Agent authors the re-packaged handoff inline** — it does not delegate this to the Handoff Agent. The Handoff Agent remains responsible only for authoring the forward next-handoff in step 8 of a passing run.

## Model Selection

When delegating to sub-agents, use an explicit cheaper model at or below the X=15 threshold:
- Prefer: "Claude Sonnet 4.5 (copilot)", "Claude Haiku 4.5 (copilot)", "GPT-5 (copilot)", "Gemini Pro 2.0 (copilot)", "Gemini Flash 2.0 (copilot)", "GPT mini (copilot)"
- When multiple eligible models are equal in cost, prefer the latest model version or generation.

## Output Format

Return a concise iteration summary containing:
- **Track:** the ticket id(s) or implementation scope iterated
- **Phase outcomes:** concise summary of Review (pass/fail), Interview (escalations resolved), Commit (committed work), Handoff (forward handoff produced or re-packaged inline)
- **Gates enforced:** which gates fired (review, escalation, loop-closure) and which passed
- **Ticket transitions:** state before and after iteration (e.g., `in-implementation` → `done`, or `in-implementation` → `in-implementation` after failed review)
- **Handoff package:** handoff id and whether it was a forward handoff (step 8) or a re-packaged return (step 4)
- **Blockers:** any unresolved escalations or missing required fields in the handoff package
- **Next action:** the immediate next step for the human or next agent to take
- All ticket/spec/session/handoff references rendered per the Clickable Reference Policy in [AGENTS.md](../../AGENTS.md)
