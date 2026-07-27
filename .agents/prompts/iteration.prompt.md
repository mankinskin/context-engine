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

## Input Interpretation

Every invocation is a request to run the iteration loop on the described scope. Nothing else.

- Treat whatever you are given — an implementation summary, a completed-work report, a ticket id, a handoff package, or a pasted status dump — as **the scope to review**. It is never a status update to acknowledge, a plan to critique, or a request for advice.
- Start the Review phase immediately in the same run. Do not ask whether to proceed, do not ask which ticket to start next, and do not propose a sequence and wait for confirmation.
- Never substitute an assessment, recommendation, or "confirm and I will sequence" message for running the loop. User confirmation is gathered in the Interview phase, after review findings exist.
- Do not propose or perform implementation work — not even a small docs edit or a cheap follow-up ticket. Gaps found during review become review findings, interview questions, and next actions in the handoff.
- If the scope is genuinely unidentifiable, run one anchoring lookup via ticket-mcp/session-mcp before asking the user. Ask only if that lookup also fails.

## Interview Rule

Every open decision the review surfaces must be answered by the user before the loop ends. Decisions are never deferred into the handoff or into next actions.

- After the review returns, enumerate every unresolved question, waiver, ambiguity, conflict, or judgement call. If that list is non-empty, running the Interview phase is mandatory — it is not conditional on the review failing.
- Ask the user directly, one concrete question at a time, each with options and a recommended default. Do not answer them yourself, do not pick a default silently, and do not declare a question moot without the user saying so.
- Always-interview items: waiving an unmet acceptance criterion, amending acceptance criteria instead of fixing the gap, which of two conflicting ticket/commit records is authoritative, whether to open a follow-up ticket, and whether to close or return a ticket with partial evidence.
- The loop may not proceed to Commit, Handoff, or ticket closure while any such question is unanswered.
- **Next actions must be executable directives, not decisions.** "Update X to Y" or "open ticket Z" — never "decide whether…", "either A or B", or "reconcile X vs Y". A choice appearing in Next actions or in the handoff is a missed interview question.

## Workflow

1. **Read the slash-command text** and determine the implementation track to iterate (ticket id, current session, or handoff package).
2. **Anchor on the track.** Read the target ticket(s), current session state, or handoff package. Assume the described work is complete and awaiting review; do not ask the user to confirm this. Proceed directly to step 3.
3. **Delegate Review phase.** Use the agent tool to invoke the [Review Agent](.agents/agents/review.agent.md) with the target ticket(s). Instruct it to verify acceptance criteria, gather evidence, and return a pass/fail verdict with per-criterion findings, and to perform **no** ticket transitions. Use an explicit model one tier above the cheap threshold (e.g., "Claude Sonnet 4.5 (copilot)").
4. **Delegate Interview phase — on both the pass and fail paths, mandatory whenever the review raised anything unresolved.** Enumerate every open question, waiver, conflict, or judgement call from the review. Use the agent tool to invoke the [Interview Agent](.agents/agents/interview.agent.md) to put those questions to the user and collect answers. Use an explicit cheap model. Apply the answers to tickets/specs. Do not skip this step on a passing review, and do not skip it on a failed review: the returned package must carry zero open escalations and zero open decisions.
5. **Enforce escalation gate.** Confirm all escalations are resolved. If any remain, stop and escalate to the user — no ticket may reach `done`, and no handoff may be marked implementation-ready, while an unresolved user escalation exists.
6. **Enforce review gate.**
   - If review passes, proceed to step 7.
   - If review fails, **author the next-handoff inline** (do not delegate re-packaging to the Handoff Agent). The re-packaged handoff must satisfy the [handoff-package schema](.spec/specs/5e52039d-aabc-434d-bdf3-eca63e312476/spec.toml):
     - **objective** — the single goal of the next implementation unit
     - **target_tickets** — ticket ids with current state and acceptance criteria inlined
     - **target_files** — explicit workspace-relative paths expected to be touched
     - **decisions** — resolved design choices
     - **validation** — exact commands/checks that prove the unit done
     - **non_goals** — explicit out-of-scope boundaries
     - **context_anchors** — prior findings, links, and ids needed so no search is required
     - **open_escalations** — must be empty; the step 4 interview is what empties it
   - **Ask the user whether to commit the partial work as WIP** before stopping. If they approve, delegate the commit to the [Commit Agent](.agents/agents/commit.agent.md); if they decline, leave the worktree dirty and report that in the summary.
   - Move the ticket to `in-implementation` yourself with `mcp_ticket-mcp_update_ticket` or `ticket update <id> --to-state in-implementation`.
   - Persist the handoff via `session_handoff` with `mcp_session-mcp_session_handoff` (or the session-cli equivalent).
   - Stop; the iteration is complete. The next implementation session will load this re-packaged handoff.
7. **Delegate Commit phase.** Use the agent tool to invoke the [Commit Agent](.agents/agents/commit.agent.md) to commit the approved work (hooks, rule sync, generated files, submodule pointers, conventional messages). Use an explicit cheap model. Capture the resulting commit sha(s).
8. **Delegate Handoff phase.** Use the agent tool to invoke the [Handoff Agent](.agents/agents/handoff.agent.md) to author the forward next-handoff package for the next implementation unit. Use an explicit cheap model. The Handoff Agent produces the handoff satisfying the [handoff-package schema](.spec/specs/5e52039d-aabc-434d-bdf3-eca63e312476/spec.toml).
9. **Perform the ticket transition and enforce the loop-closure gate.** You own every state transition: close the ticket yourself on a pass (`close_ticket`), or return it to `in-implementation` on a fail. Confirm a handoff package exists (either the forward handoff from step 8 or the re-packaged handoff from step 6).
10. **Persist the handoff.** Use `mcp_session-mcp_session_handoff` (or the session-cli equivalent) to persist the handoff record.

## Gates

- **Review gate (step 6):** acceptance criteria verified before commit; failures return the ticket to `in-implementation`.
- **Escalation gate (step 5):** no ticket reaches `done`, and no handoff is implementation-ready, while an unresolved user escalation or unanswered review decision exists.
- **Loop-closure gate (step 9):** every finished implementation terminates in a durable handoff package plus a ticket transition (closed or returned).

## Ticket Transition Ownership

Sub-agents report verdicts and findings only. The Iteration Agent performs **all** ticket state transitions — the Review Agent must not close or move tickets.

## Re-packaging Rule

When a review fails (step 6), the **Iteration Agent authors the re-packaged handoff inline** — it does not delegate this to the Handoff Agent. The Handoff Agent remains responsible only for authoring the forward next-handoff in step 8 of a passing run.

## Model Selection

- **Review phase:** one tier above the cheap threshold — prefer "Claude Sonnet 4.5 (copilot)".
- **Interview, Commit, Handoff phases:** at the cheap threshold — prefer "Claude Haiku 4.5 (copilot)", "GPT-5 mini (copilot)", or "Gemini Flash 2.0 (copilot)".
- When multiple eligible models are equal in cost, prefer the latest model version or generation.

## Output Format

End the run with a single inline summary block using **bold-label bullets**, one per field, in this exact order. Do not use a table, and do not print the full handoff package in chat.

- **Track:** the ticket id(s) or implementation scope iterated
- **Phase outcomes:** one line each for Review (pass/fail), Interview (escalations resolved), Commit (committed / skipped / declined by user), Handoff (forward or re-packaged inline)
- **Review findings:** each acceptance criterion mapped to its verdict, as a nested list of `criterion → verdict`
- **Ticket transitions:** state before → state after, per ticket (e.g., `in-implementation` → `done`)
- **Commits:** the commit sha(s) produced this iteration, or `none`
- **Handoff package:** a clickable link to the persisted handoff plus a one-line restatement of its `objective` — never the full eight fields
- **Next actions:** the immediate next steps for the human or next agent, phrased as executable directives. Never a decision, choice, or open question — those are resolved in the Interview phase. Any unresolved escalation is reported here; there is no separate blockers field.

Omit no field: render `none` when a field is empty. Render all ticket/spec/session/handoff references per the Clickable Reference Policy in [AGENTS.md](../../AGENTS.md).
