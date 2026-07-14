---
description: "Use when creating or updating specs. Covers spec discovery, authoring workflow, and traceability expectations across tickets, tests, validation, and related specs."
---

<!-- rule-api:file generated=true -->

<!-- rule-api:entry id=c794a4a2-0f42-4dc0-ad9a-21cfe36003ca slug=shared/instructions/spec-system/spec-system-instructions/l1 -->


<!-- rule-api:entry id=731d8745-3567-4e94-ab45-fa18d668c190 slug=shared/instructions/spec-system/spec-system-guidance/scope/l8 -->
## Scope

Applies when creating, updating, reviewing, or validating specifications through the repository's spec system.

<!-- rule-api:entry id=69f70fd0-350b-4440-aba5-226558651b46 slug=shared/instructions/spec-system/spec-system-guidance/design-constraints/l12 -->
## Design Constraints

- Prefer one clear owning spec per behavior or requirement slice.
- Keep specs focused on system properties, acceptance criteria, evidence, and non-goals.
- Keep implementation plans, rollout sequencing, and execution notes in tickets unless they materially affect the contract.
- Preserve traceability between specs, tickets, validation evidence, and neighboring specs.

<!-- rule-api:entry id=9eaa6cd8-fe5b-4c59-81e6-9e76a8a95ec1 slug=shared/instructions/spec-system/spec-system-guidance/spec-quality-standing-obligations/l18 -->
## Spec Quality — Standing Obligations

These rules apply whenever spec work is involved, not only when editing spec-system code.

<!-- rule-api:entry id=0c74ff98-f132-4267-adfb-4838d7b1f8ca slug=shared/instructions/spec-system/spec-system-guidance/spec-quality-standing-obligations/orientation-start-of-every-session/l22 -->
### Orientation (start of every session)

Before writing or editing a spec:

- search existing specs for the behavior first
- search related tickets so the spec can link the current execution plan
- check whether a neighboring or parent spec already owns the requested slice

Prefer `spec-mcp` and `ticket-mcp` tools when available. Fall back to `./target/debug/spec.exe` and `./target/debug/ticket.exe` when needed.

<!-- rule-api:entry id=20ee5a43-46bd-4b3f-9406-8ef7a47af112 slug=shared/instructions/spec-system/spec-system-guidance/spec-quality-standing-obligations/discovery-before-creating/l31 -->
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

<!-- rule-api:entry id=2db9f950-4234-4d99-895f-f4dab7a0cdd8 slug=shared/instructions/spec-system/spec-system-guidance/spec-authoring-workflow/l44 -->
## Spec Authoring Workflow

<!-- rule-api:entry id=c71e7aa9-bc67-49ba-8f85-4ff23aa98043 slug=shared/instructions/spec-system/spec-system-guidance/spec-authoring-workflow/choose-component-slug-and-parent/l46 -->
### Choose Component, Slug, and Parent

- Use the owning subsystem or workflow area as the component.
- Keep slugs lowercase, use `-` within segments, and `/` between segments.
- Prefer a parent spec only when the new slice is clearly a child of an existing broader contract.
- Avoid creating shallow duplicate siblings with overlapping goals.

<!-- rule-api:entry id=aa5a2e2f-16dc-4abc-b0a5-4132e5b3f0b1 slug=shared/instructions/spec-system/spec-system-guidance/spec-authoring-workflow/structure-the-spec/l52 -->
### Structure the Spec (aligned-structure:v2)

Each spec must act as a dependable, verifiable contract. Every spec must start with the `<!-- aligned-structure:v2 -->` template marker and define the following five required sections:

1. **Motivation ("why")** — The user requirement or behavior need this spec satisfies, with optional links to feedback explaining its origin.
2. **Dependent expectation** — An explicit, clear contract clause: "If this spec is implemented, dependents can rely on behavior X."
3. **Guards** — Declared test-api `ValidationSpec` ids that gate the spec. The spec's `verified` state is COMPUTED from guard execution outcomes, never hand-set.
4. **Positions** — Current implementation/readiness status per referenced code symbol/path: `implemented`, `partial`, `not-implemented`, or `deprecated` with an explicit `code_ref`.
5. **Governing-rule requirement** — Link to the PolicyRule(s) that must introduce/explain this spec in-session (governed by the rule-introduces-spec mechanism).

Acceptance criteria and guards must be concrete enough that a reviewer or automated tool can tell exactly what evidence proves the contract is satisfied.

<!-- rule-api:entry id=633ef4f2-37c4-4952-a293-494b8c44c947 slug=shared/instructions/spec-system/spec-system-guidance/spec-authoring-workflow/link-tickets-tests-and-related-specs/l61 -->
### Link Tickets, Tests, and Related Specs

Specs should explicitly link the work needed to satisfy or verify the contract.

- Link the exact related ticket folder paths returned by ticket tools. Do not synthesize ticket paths.
- Render ticket references per the Clickable Reference Policy in `AGENTS.md`.
- Record the validation plan or completed validation results needed to evaluate the spec.
- Link related specs when they define prerequisites, shared contracts, or adjacent behavior.
- When docs or generated guidance are part of the deliverable, include them in the traceability or evidence section.

Use a clear evidence vocabulary when possible, including validation commands, expected evidence objects, and blocked or passing results.

<!-- rule-api:entry id=68d59714-32de-42af-9a51-cb280e99a35f slug=shared/instructions/spec-system/spec-system-guidance/spec-authoring-workflow/validation-before-review/l71 -->
### Validation Before Review

Before moving spec work toward review, verify:
- the acceptance criteria are testable
- the linked tickets are sufficient to execute the work
- the validation evidence is concrete, not implied
- related specs are linked where cross-spec behavior matters
- the spec still describes the contract, not a ticket-sized implementation plan

<!-- rule-api:entry id=6048288b-9fe1-4fac-8786-976254191bed slug=shared/instructions/spec-system/spec-system-guidance/workflow-expectations/l78 -->
## Workflow Expectations

- When requirements, goals, or behavior change, create or update the relevant spec before implementation.
- When implementation reveals a contract change, update the spec and its evidence trail immediately.
- Keep ticket links, validation results, and related spec references current enough that another agent can continue the work without reconstructing intent.
- Use the Spec Agent when work is primarily about creating or refining specs rather than implementing code.
- If ambiguity remains after focused search, ask one concise clarification instead of guessing.
