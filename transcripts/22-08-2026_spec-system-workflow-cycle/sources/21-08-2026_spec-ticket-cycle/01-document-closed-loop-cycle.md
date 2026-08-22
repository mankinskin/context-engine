# 01 — Document the Closed-Loop Cycle as a Named Principle

## Outcome

A new instruction file states the closed-loop production-workflow cycle as a single, citable principle, and `AGENTS.md` gets a one-line cross-reference to it (not an inline restatement).

## Description

The cycle, in order:

1. **Request** — the user's free-text ask (or a raw transcript, per the prompt-ingestion pipeline).
2. **Spec** — the goal in its ideal, verifiable form: "the definition of the world" the process must trust, a contract. Optional for small/simple work; required once a ticket's scope needs that contract.
3. **Tickets** — the plan for moving from current state to the spec's goal. A ticket does not require a spec (small work can skip straight here), but when a spec exists, the ticket should depend on/fulfill it rather than only depending on other tickets.
4. **Tests** — executable measurements that validate the spec's acceptance criteria; test-api executions already link to both `spec_ids` and `ticket_ids` today.
5. **Implementation** — the actual code change, gated behind the ticket's `in-review` state per `lifecycle.instructions.md`.
6. **Validated response to the user** — the loop cannot close silently; it must return evidence-backed confirmation the user can judge.
7. **Next iteration** — the user's judgment of the response (rules followed? efficient? requirements met?) seeds the next pass through the same cycle.

Each pass through the cycle is expected to gather **measurable evidence** from both the system (test/validation results) and the user (satisfaction with the response) to improve system performance over time — this is the incremental/iterative optimization framing from the transcript's second input.

## Non-Goal

Do not restate or duplicate the detailed mechanics already owned by `workflow.instructions.md`, `lifecycle.instructions.md`, `spec.prompt.md`, or `escalation-gate.instructions.md` — the new file names the cycle and cross-references those files for their existing mechanics, per the corpus's existing owns/references pattern (see `evidence-grounded-refinement.instructions.md` for a recent precedent of this same pattern).

## Validation Method

Manual read-through: the new file states each of the 7 steps above, links to the file that already owns that step's mechanics, and is cross-linked from `AGENTS.md`. No automated test applies to a documentation file; validation is peer/self review confirming no step duplicates existing prose.
