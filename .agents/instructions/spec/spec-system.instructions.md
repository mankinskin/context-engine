---
description: "Use when creating or updating specs. Covers spec discovery, authoring workflow, and traceability expectations across tickets, tests, validation, and related specs."
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

### Rule-Governed Introduction by Readiness

Every spec must be introduced in-session by a governing PolicyRule, conditioned on the spec's computed readiness status:

- **implemented** — present the spec as a live, fully dependable contract dependents can immediately rely on.
- **partial-with-gaps** — present the spec but list the explicit unimplemented positions so agents do not assume gaps are complete.
- **coming-soon / not-implemented** — present a "coming soon" note so agents know the spec is defined but unimplemented.

This keeps spec availability legible to agents, avoids context bloat, and ensures every active spec has an active governing rule.

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

### Component Hierarchy

When a request names independently addressable components, create one thin parent root and one child spec per component. The root carries only shared motivation, cross-component invariants, and the component relationship map; each child MUST set `parent` to that root and own its component contract. Root specs MUST NOT carry per-component acceptance criteria.

Create the root first, then create each child with:

```bash
spec create --workspace <repo-root> --title "<child-title>" --slug <child-slug> --component <component> --parent <root-id-or-slug>
```

Use the [spec-editor hierarchy](../../../.spec/specs/788e91e4-32d7-4ff5-bf68-485235f8211f/body.md) as the imitable precedent.

### Choose Component, Slug, and Parent

- Use the owning subsystem or workflow area as the component.
- Keep slugs lowercase, use `-` within segments, and `/` between segments.
- Every component child MUST set `parent` to its root. Root specs are reserved for shared scope and MUST NOT carry per-component criteria.
- Avoid creating shallow duplicate siblings with overlapping goals.

### Structure the Spec (aligned-structure:v2)

Each spec must act as a dependable, verifiable contract. Every spec must start with the `<!-- aligned-structure:v2 -->` template marker and define the following five required sections:

1. **Motivation ("why")** — The user requirement or behavior need this spec satisfies, with optional links to feedback explaining its origin.
2. **Dependent expectation** — An explicit, clear contract clause: "If this spec is implemented, dependents can rely on behavior X."
3. **Guards** — Declared test-api `ValidationSpec` ids that gate the spec. The spec's `verified` state is COMPUTED from guard execution outcomes, never hand-set.
4. **Positions** — Current implementation/readiness status per referenced code symbol/path: `implemented`, `partial`, `not-implemented`, or `deprecated` with an explicit `code_ref`.
5. **Governing-rule requirement** — Link to the PolicyRule(s) that must introduce/explain this spec in-session (governed by the rule-introduces-spec mechanism).

Acceptance criteria and guards must be concrete enough that a reviewer or automated tool can tell exactly what evidence proves the contract is satisfied.

#### Anti-Boilerplate Gate

Every child spec MUST state its responsibility, interfaces and dependencies, observable behavior, boundaries and failure cases, and concrete acceptance evidence. Omit any mandated section that would contain only a placeholder. A one-sentence purpose with no behavior, boundary, or failure detail is incomplete and MUST be rejected in review; for example, "Capture the requested outcome and any open questions that must be resolved before durable planning." is not a sufficient component contract.

### Link Tickets, Tests, and Related Specs

Specs should explicitly link the work needed to satisfy or verify the contract.

- Link the exact related ticket folder paths returned by ticket tools. Do not synthesize ticket paths.
- Render ticket references per the Clickable Reference Policy in `AGENTS.md`.
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
