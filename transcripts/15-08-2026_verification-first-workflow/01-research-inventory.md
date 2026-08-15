# Research Inventory

## Method and Boundary

This research uses bounded source inspection only. The session does not invoke
or modify product workflow tools. The inventory answers one question per
surface: what already exists, what remains unproven, and what a follow-up must
validate before implementation.

## Validation Evidence

`memory-api/crates/test-api/src/store.rs` persists validation specifications
and executions. Executions can be queried by ticket, validation specification,
outcome, duration, provenance, and run identifier.

**Implication:** a verification-first workflow should extend the existing
specification/execution evidence model. It should not introduce a parallel
record type without demonstrating a gap in the current model.

## Validation Logs

`memory-api/crates/log-api/src/lib.rs` defines a validation-log capture linked
to a validation execution. The interoperability contract rejects a capture
without an execution identifier and the matching execution link; it also carries
ticket, specification, acceptance-criterion, and document-evidence links.

**Implication:** a proposed verifier should produce evidence that can be traced
from a request criterion to an execution and then to captured output.

## Audits

`memory-api/crates/audit-api/src/trials/mod.rs` exposes cargo-quality,
file-length, rule-overlap, session-workflow, specification-fulfillment,
static-metric, and ticket-graph trials.

**Implication:** the first gap analysis must map desired deterministic checks to
these trials before proposing new linting. A new check is justified only when
the audit inventory cannot express the required condition.

## Cost Control

`memory-api/tools/mcp/toolmon-costgate/src/gate.rs` implements a model-aware
cost gate. The gate resolves model output pricing, calculates a budget, consults
empirical or fallback tool costs, applies grants, and returns an allow or
delegate decision.

**Implication:** the roadmap must distinguish existing cost enforcement from
MCP-surface usability. A proposal for lower cost must state whether it changes
tool selection, tool payloads, tool-call count, context size, or agent routing.

## Research Gaps to Close Later

| Question | Required follow-up evidence |
| --- | --- |
| Which required deterministic checks are missing? | A matrix mapping each desired rule to an existing audit trial or a documented gap. |
| Which ticket state changes already enforce validation? | Ticket lifecycle tests and state-transition code. |
| Which Test API fields cannot model criterion quality or review? | Public model definitions, persistence tests, and one sample record. |
| What causes the highest MCP cost? | Tool metrics grouped by tool, model, input/output tokens, and task outcome. |
| Which claims require human or LLM review? | A criterion catalog that explains why deterministic validation is insufficient. |
