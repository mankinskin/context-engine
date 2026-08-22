<!-- aligned-structure:v2 -->

# Specification

## Responsibility And Interface

Convert a sufficiently understood request into a durable, reviewable goal and
definition of success. Consume Request's two criteria; write draft records below
`.spec/specs/` through `./target/debug/spec.exe --workspace . create|update` and
`.agents/prompts/spec.prompt.md`; provide the three criteria consumed by Tickets.

## Behavior And Contract

- `spec-goal`: defines the requested property before implementation planning.
- `spec-owned-criteria`: assigns each criterion to one provider; consumers cite it.
- `spec-traceability`: names related specs, resolved ticket paths when present,
  and concrete evidence needed for review.

## Boundaries And Failure Cases

Do not create a duplicate or claim draft validation passed. Independently
addressable components require a thin root and explicit `parent` children. If
the goal, owner, or success condition is unclear, stop for user/interview work
rather than producing vague criteria or a ticket-sized implementation plan.

## Acceptance Evidence And Position

`spec.exe --workspace . health --all` checks store structure and `get <id> --json`
proves draft identity/parentage; review checks single criterion ownership. No
`validated_by` is asserted. The CLI and spec prompt are implemented; the spec
system instruction is the governing rule.
