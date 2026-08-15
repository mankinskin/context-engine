<!-- aligned-structure:v2 -->

# Explainer Agent version-one contract

## Motivation

The Explainer Agent gives a human a researched, bounded explanation before any
execution occurs. The contract preserves human control, makes uncertainty
visible, and captures reusable feedback without granting the Explainer Agent
any mutation or delegation capability.

Related ticket: [79449c3 Define Explainer Agent version-one contract](.ticket/tickets/79449c3f-2f49-4925-b8fd-3751face53b5/ticket.toml).

## Dependent expectation

If this specification is implemented, a user can invoke
`.agents/agents/explainer.agent.md` and depend on the Explainer Agent to
research repository evidence, return an explanation rather than execute work,
and leave every execution decision with the human.

## Contract

### Read-only boundary

- The Explainer Agent may read repository content and research relevant
  tickets, specs, docs, code, and validation evidence before responding.
- The Explainer Agent must not write files, mutate stores or services, write
  feedback, invoke delegation, run mutating commands, or execute the proposed
  task.
- The template exposes only read and research tools. The v1 work does not
  alter runtime tools, routing, model weights, or grant a mutation exception.

### Explanation and decision

Each response distinguishes verified facts from assumptions and recommendations
and includes: objective, constraints, required human steps, relevant entities,
validation approach, risks, non-goals, and a recommended decision.

The response offers four human decisions and their consequences:

| Decision | Consequence |
| --- | --- |
| approve | A human or separately chosen process may execute the bounded work. |
| revise | The human supplies changes; a later explanation re-evaluates the revised scope. |
| narrow | The human selects a smaller scope; no execution occurs until that scope is approved. |
| decline | The proposed work ends with no execution. |
| delegate | The human, not the Explainer Agent, selects and invokes a separate execution process. |

### Feedback capture

A separate, human-invoked feedback-capture process owns `feedback_ingest`.
For every feedback-bearing run, the process stores two linked entries: one
explanation rating and one execution-result rating. Each entry records a
correlation run or session identifier, a $1$--$5$ rating, interaction category,
task type, optional free text, and the same canonical feedback target.

Anchored runs target `ce://default/ticket/<ticket-id>` or
`ce://default/spec/<spec-id>`. Unanchored runs are out of scope for v1; capture
must not substitute an ad hoc target. Both entries remain team-visible under
the ordinary repository lifecycle. A later human-reviewed maintenance workflow
may aggregate patterns by task type and interaction category; that workflow
does not change the v1 Explainer Agent at runtime.

## Scope and non-goals

In scope: the user-invocable Explainer Agent template, evidence-backed
explanations, the human decision boundary, and an external capture contract for
explanation and result feedback.

Out of scope: autonomous execution, filesystem/store/service mutation by the
Explainer Agent, delegation by the Explainer Agent, unanchored feedback runs,
runtime routing or model-weight changes, and automatic template modification
from collected feedback.

## Guards and validation evidence

No test-api `ValidationSpec` identifiers exist for this not-implemented slice.
Before the specification may be treated as verified, implementation must record
passing evidence for all of the following guards:

1. Inspect the template to prove it declares only read/research tools and no
	delegation or mutation capability.
2. Inspect a representative explanation for all required fields, separated
	verified facts and assumptions/recommendations, and each decision's stated
	consequence.
3. Trace an Explainer Agent run and prove no mutation call occurred.
4. Use the external capture process to write and read back two correlated
	$1$--$5$ entries against a canonical ticket or spec target.
5. Demonstrate a human-reviewed aggregation grouped by task type and
	interaction category without changing the template automatically.
6. Run a pilot of five representative explanations. Compute mean explanation
	rating and mean result rating independently; each must be at least $4/5$,
	and the audit must report zero approval-boundary violations.

## Positions

| Code reference | Status | Required position |
| --- | --- | --- |
| `.agents/agents/explainer.agent.md` | not-implemented | Provide the user-invocable, read-only Explainer Agent template and response contract. |
| External human-invoked feedback-capture process | not-implemented | Persist the two correlated rating entries through `feedback_ingest` and read them back. |
| Human-reviewed feedback-maintenance workflow | not-implemented | Group retained feedback by task type and interaction category without autonomous template changes. |

## Governing rule requirement

The rule-introduces-spec mechanism defined by
[51ee3a34 Rule-introduces-spec](.spec/specs/51ee3a34-110c-45ae-ba73-a0c38ba9f7a6/spec.toml)
must present this spec as coming soon while all positions are not implemented,
and must present the implemented readiness only after the guards have passing
evidence.

## Traceability

The implementation must retain the bidirectional ticket link to
[79449c3 Define Explainer Agent version-one contract](.ticket/tickets/79449c3f-2f49-4925-b8fd-3751face53b5/ticket.toml).
Related contracts are [ec3b13f1 Per-template MCP tool grants](.spec/specs/ec3b13f1-ae9f-4f11-b3f9-e8fa3877afbd/spec.toml) for the read-only grant surface and
[1b654f30 Two-tier Planner/Worker model routing architecture](.spec/specs/1b654f30-d1a4-4cb4-ab2e-8355dfe5a758/spec.toml) for the non-delegating boundary.
