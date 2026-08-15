# Verification Operating Model

## Principle

For every request, define the verification route before selecting a solution.
The route specifies what must be true, who or what can determine it, what
evidence is retained, and how uncertainty is handled.

## Minimum Criterion Record

Every acceptance criterion must contain the following fields:

| Field | Required content |
| --- | --- |
| Request link | The request, ticket, or specification requirement being satisfied. |
| Observable claim | A testable statement, not an implementation preference. |
| Verifier class | Local deterministic, hosted deterministic, human review, or advisory LLM assessment. |
| Verification method | Exact command, protocol, or review question. |
| Pass rule | Deterministic expected output, threshold, or explicit reviewer approval. |
| Evidence link | Validation execution and, when relevant, linked log capture. |
| Environment and provenance | Version, inputs, platform, and run identifier needed to reproduce the result. |
| Flakiness policy | Required repeat count, tolerated variance, and disposition for instability. |
| Decision owner | The system, a named reviewer role, or an explicitly approved LLM process. |

## Verifier Hierarchy

1. Use a local deterministic check when a local predicate can establish the
   claim.
2. Use a hosted deterministic check only when local execution cannot establish
   the claim.
3. Use human review for subjective, contextual, or policy-sensitive claims.
4. Use an LLM assessment only as advisory evidence unless an explicit policy
   grants the LLM decision authority.

No higher-cost or less-repeatable verifier may replace an available lower-tier
verifier without recording why the lower-tier verifier is inadequate.

## Evidence Chain

The evidence chain is:

`request -> criterion -> verification specification -> execution -> log or
other artifact -> decision`

Every link must be queryable. A result without an execution, an execution
without a criterion, or a decision without evidence is incomplete.

## Flakiness Policy

For any verifier that is rerun:

1. record the environment, input, duration, and provenance for each run;
2. classify the result as stable pass, stable fail, flaky, or inconclusive;
3. do not use flaky or inconclusive evidence as a release gate without an
   explicit human decision;
4. preserve the observed variance so a later audit can distinguish a product
   regression from an unstable verifier.

## Artifact Ownership Boundary

| Artifact class | Owner | Scope | Dependency rule |
| --- | --- | --- | --- |
| General workflow tool | Product | Portable across projects | Must not embed customer-project policy or data. |
| Product verification contract | Product | Portable but configurable | May define extension points, not project-specific criteria. |
| Ticket, specification, execution, and log evidence | Customer project | Project-local | Must link back to the product contract without becoming product state. |
| Project policy and acceptance criteria | Customer project | Project-local | May use product tools but remains under project governance. |
