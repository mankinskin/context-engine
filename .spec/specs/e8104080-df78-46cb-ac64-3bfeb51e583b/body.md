<!-- aligned-structure:v2 -->

# Production Workflow: Request

## Target Code Location

[.agents/instructions/orchestration/core-cycle.instructions.md](.agents/instructions/orchestration/core-cycle.instructions.md) owns request-to-spec routing; [AGENTS.md](AGENTS.md) owns task-routing thresholds and feedback recording.

## Naming Conventions

Use a request dossier with `outcome`, `constraints`, and `open_questions` fields.
This component owns `request-outcome` and `request-open-questions`.

## Requester Input

> Specify the full workflow cycle's components in the current spec system.

## Reading Order

1. [.agents/instructions/orchestration/core-cycle.instructions.md](.agents/instructions/orchestration/core-cycle.instructions.md) — request handoff owner.
2. [d6b4d989 Production Workflow: Specification](.spec/specs/d6b4d989-f9ac-428a-9dbc-68400006fc96/body.md) — consumer of request criteria.
3. [4199e86c Production Workflow: Next Iteration](.spec/specs/4199e86c-05b0-48a0-bebd-c55efcfa20a5/body.md) — provider of follow-up input.

## Responsibility

If implemented, Specification can rely on a request artifact that states the
desired observable outcome and either lists unresolved decisions or says `none`.

## Interfaces And Dependencies

Input is free text, a transcript, or a Next Iteration follow-up. Output is a
dossier, not a ticket or implementation plan, with the two owned criteria.

## Behavior

- `request-outcome` records the requested result rather than an inferred fix.
- `request-open-questions` preserves ambiguous scope or decisions for discovery.
- Routing applies the shared model: direct small work can proceed to Implementation,
  an approved ticket-only slice can enter Tickets, and new requirements enter
  Specification after this request is sufficiently understood.

## Boundaries And Failure Cases

Request capture may research and interview but cannot claim acceptance criteria
or executable slices. Missing outcome, constraints, or decisions remain open;
they are not guessed or silently converted into a ticket.

## Provider/Consumer Contract

Provides `request-outcome` and `request-open-questions` to [d6b4d989 Production Workflow: Specification](.spec/specs/d6b4d989-f9ac-428a-9dbc-68400006fc96/body.md); consumes `iteration-follow-up-transition` from [4199e86c Production Workflow: Next Iteration](.spec/specs/4199e86c-05b0-48a0-bebd-c55efcfa20a5/body.md).

## Examples

`Outcome: export audit history; constraints: retain CSV compatibility; open_questions: none` is a complete request artifact. `Outcome: improve exports; open_questions: which reports?` must remain in Request.

## Evidence

Review the dossier, then run `./target/debug/spec.exe --workspace . get d6b4d989-f9ac-428a-9dbc-68400006fc96 --json` to inspect the consuming draft. Position: `implemented` guidance, with no executable guard for request quality.

## Scope

Owns intake and proportional routing; it does not create specifications, tickets, or code.
