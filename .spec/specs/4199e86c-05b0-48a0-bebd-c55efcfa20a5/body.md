<!-- aligned-structure:v2 -->

# Next Iteration

## Responsibility And Interface

Record the user's validated-response judgment and close the cycle or turn a
follow-up into a new Request. Consume Validated response's three criteria. A
passing iteration creates `.session/sessions/<session-id>/handoffs/` through
`./target/debug/session.exe handoff`; the required package is defined by
`.spec/specs/5e52039d-aabc-434d-bdf3-eca63e312476/body.md`.

## Behavior And Contract

- `iteration-recorded-judgment`: preserves satisfaction or requested follow-up.
- `iteration-follow-up-transition`: closes on satisfaction; otherwise starts Request
	with the follow-up outcome and its open questions.
- Passing work follows Review -> Interview -> Commit -> Handoff with empty
	`open_escalations` before implementation readiness.

## Boundaries And Failure Cases

Do not close with unresolved escalation, an unmet criterion, or a missing next
handoff. Do not commit unapproved partial work. A follow-up cannot be silently
added to old implementation scope; ambiguous intent returns to discovery/interview.

## Acceptance Evidence And Position

Review the recorded judgment and, when continuing, the handoff's objective,
target paths, decisions, non-goals, anchors, and empty `open_escalations`; check
the new Request artifact for outcome/questions. No `validated_by` run exists.
Loop closure and handoff schema are the implemented governing contracts.
