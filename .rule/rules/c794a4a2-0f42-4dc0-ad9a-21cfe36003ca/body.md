---
description: "Use when creating or updating specs. Covers spec discovery, authoring workflow, and traceability expectations across tickets, tests, validation, and related specs."
applyTo: "**"
---

## Scope

Applies when creating, updating, reviewing, or validating specifications through the repository's spec system.

## Design Constraints

- Prefer one clear owning spec per behavior or requirement slice.
- Keep specs focused on system properties, acceptance criteria, evidence, and non-goals.
- Keep implementation plans, rollout sequencing, and execution notes in tickets unless they materially affect the contract.
- Preserve traceability between specs, tickets, validation evidence, and neighboring specs.

## Spec Quality — Standing Obligations

These rules apply whenever spec work is involved, not only when editing spec-system code.

### Orientation (start of every session)

Before writing or editing a spec:

- search existing specs for the behavior first
- search related tickets so the spec can link the current execution plan
- check whether a neighboring or parent spec already owns the requested slice

Prefer `spec-mcp` and `ticket-mcp` tools when available. Fall back to `./target/debug/spec.exe` and `./target/debug/ticket.exe` when needed.

### Discovery Before Creating

Always search for an existing spec before creating a new one. Duplicate specs weaken the repository contract.

Prefer updating a matching spec when:
- the behavior belongs to the same component and scope
- the existing spec can absorb the acceptance criteria without becoming unfocused
- the requested change is a refinement rather than a new contract slice

Create a new spec when:
- the requested behavior is a distinct contract slice
- the existing spec would become too broad or mix unrelated concerns
- the new work needs its own acceptance criteria and evidence trail

## Spec Authoring Workflow

### Choose Component, Slug, and Parent

- Use the owning subsystem or workflow area as the component.
- Keep slugs lowercase, use `-` within segments, and `/` between segments.
- Prefer a parent spec only when the new slice is clearly a child of an existing broader contract.
- Avoid creating shallow duplicate siblings with overlapping goals.

### Structure the Spec

Each spec should make review straightforward. Capture, when relevant:
- goal or intended behavior
- problem or current-state gap
- scope
- non-goals
- acceptance criteria
- traceability or evidence expectations

Acceptance criteria should be concrete enough that a reviewer can tell what evidence proves the work is done.

### Link Tickets, Tests, and Related Specs

Specs should explicitly link the work needed to satisfy or verify the contract.

- Link the exact related ticket folder paths returned by ticket tools. Do not synthesize ticket paths.
- Record the validation plan or completed validation results needed to evaluate the spec.
- Link related specs when they define prerequisites, shared contracts, or adjacent behavior.
- When docs or generated guidance are part of the deliverable, include them in the traceability or evidence section.

Use a clear evidence vocabulary when possible, including validation commands, expected evidence objects, and blocked or passing results.

### Validation Before Review

Before moving spec work toward review, verify:
- the acceptance criteria are testable
- the linked tickets are sufficient to execute the work
- the validation evidence is concrete, not implied
- related specs are linked where cross-spec behavior matters
- the spec still describes the contract, not a ticket-sized implementation plan

## Workflow Expectations

- When requirements, goals, or behavior change, create or update the relevant spec before implementation.
- When implementation reveals a contract change, update the spec and its evidence trail immediately.
- Keep ticket links, validation results, and related spec references current enough that another agent can continue the work without reconstructing intent.
- Use the Spec Agent when work is primarily about creating or refining specs rather than implementing code.
- If ambiguity remains after focused search, ask one concise clarification instead of guessing.
