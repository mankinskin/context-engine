# Improved One-Session Plan

## Objective

Produce a concise, evidence-backed roadmap for making verification the primary
driver of workflow design while reducing avoidable agent and MCP cost.

## Completion Contract

The session succeeds when the dossier contains the five artifacts indexed in
`README.md`, every transcript requirement maps to a dossier section, and every
roadmap item has a bounded outcome and validation method. The session does not
implement product code or alter workflow state.

## Working Interpretation

"Do not use the tools directly" means do not operate or mutate the product
workflow tools during this session. Bounded inspection of source and
documentation is permitted because research is explicitly requested.

## Pre-Edit Decision Checkpoint

Before the first artifact is written:

1. state the deliverable as a dossier rather than product implementation;
2. select the four research surfaces: validation evidence, validation logs,
   audit trials, and MCP cost control;
3. set the research limit to one bounded source slice per surface;
4. choose the criterion record and verifier hierarchy as the review lens.

After the first edit, do not expand the surface list or interview the user.
Record unresolved questions instead.

## Work Packages

| Package | Estimated size | Inputs | Deliverable | Validation |
| --- | --- | --- | --- | --- |
| A. Scope and review | Small: one document | Clean transcript | Review with findings, scope decision, and repair actions | Each finding maps to a required improvement. |
| B. Capability inventory | Small: four source slices | Test, Log, Audit, and cost-gate code | Existing capability, gap, and implication per surface | Each claim names a source path; no implementation assumptions. |
| C. Operating model | Small: one document | Transcript plus inventory | Criterion record, verifier hierarchy, evidence chain, flakiness and ownership policies | Every criterion field has a purpose and a verifier class. |
| D. Roadmap | Small: one document | Review, inventory, operating model | Prioritized follow-up work packages | Each item has outcome, non-goal, dependency, and validation method. |
| E. Dossier verification | Small: one checklist | All dossier documents | Traceability and self-review record | Every checklist row is marked pass, gap, or open question. |

## Prioritized Follow-Up Roadmap

### 1. Define and validate the verification contract

**Outcome:** a versioned contract for request-to-criterion-to-evidence links,
including verifier class and flakiness metadata.

**Non-goal:** redesigning ticket or test storage before a concrete model gap is
demonstrated.

**Validation:** sample criteria produce linked specifications, executions, and
logs; a query retrieves the complete chain.

### 2. Inventory deterministic checks and enforce the highest-value gaps

**Outcome:** a matrix from desired rules to existing audit trials, lifecycle
gates, or a new check proposal.

**Initial candidate rules:** missing acceptance criteria, attempted closure
without evidence, missing reviewer decision for a test, and broken evidence
links.

**Non-goal:** implementing every possible linter in one change.

**Validation:** each rule has a passing fixture and a failing fixture; a state
transition is blocked only when its policy requires that block.

### 3. Establish flakiness and verifier-quality governance

**Outcome:** a policy and storage design for repeated runs, variance,
inconclusive results, and verification review.

**Non-goal:** declaring LLM assessment equivalent to deterministic evidence.

**Validation:** repeated-run fixtures classify stable, flaky, and inconclusive
cases; reviewers can approve or reject verifier quality with recorded reasons.

### 4. Measure MCP cost and redesign the highest-friction surface

**Outcome:** a baseline identifying the costliest tools and a focused interface
proposal for one surface.

**Non-goal:** a broad rewrite of all MCP tools.

**Validation:** compare the baseline and proposal by tool-call count, context
tokens, total cost, completion rate, and deterministic coverage on the same
representative task set.

### 5. Formalize product versus customer-project boundaries

**Outcome:** a boundary specification for portable tools, configurable product
contracts, and project-local artifacts.

**Non-goal:** moving existing artifacts until the dependency rule is approved.

**Validation:** at least one product tool and one customer-project workflow can
be classified without ambiguity; forbidden dependency directions are tested.

## Sequencing and Stop Rule

Start with the verification contract. It makes later linting, flakiness, and
cost measurement comparable. Complete only the dossier in this session. Stop
after the completion checklist passes or records explicit open questions.
