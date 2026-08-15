# Review: Verification-First Repository Plan

## Verdict

**Changes requested.** The thesis is strong, and the repository already has
relevant primitives, but the original plan is not implementable as one
uninterrupted session. The plan describes a broad product strategy without
defining a bounded artifact, a research budget, decision rules, or verifiable
completion conditions.

The revised plan must produce a review dossier, not implement verification or
cost-control features. Product changes require follow-up tickets after the
dossier has established an evidence-backed backlog.

## Research Findings

| Existing capability | Evidence | Planning implication |
| --- | --- | --- |
| Validation specifications and executions can be stored and queried. | `TestStoreConfig` persists validation specs and executions and filters executions by ticket, specification, outcome, duration, and provenance. | Start with the existing evidence model; do not design a second validation-record format in this session. |
| Validation logs carry links to executions, tickets, specifications, criteria, and documents. | `ValidationLogCapture` rejects missing execution identity or an absent execution link. | Treat linked evidence as a required contract in proposed controls. |
| Audit has existing cargo, file-length, rule-overlap, workflow, specification, and ticket-graph trials. | `audit-api` already exposes these trial modules. | Inventory gaps against existing audit coverage before proposing a new linter. |
| Cost control already has a model-aware gate. | `toolmon-costgate` derives a model budget from price data and tool cost, then allows or delegates a call. | Separate MCP ergonomics and workflow-policy gaps from the existing cost-gate mechanism. |

## Findings and Required Improvements

| Severity | Finding | Why it blocks execution | Required improvement |
| --- | --- | --- | --- |
| Critical | The target is a repository-wide strategy, but the requested session has no bounded product outcome. | A one-shot session cannot research, design, validate, and implement all verification and cost improvements safely. | Produce a time-boxed dossier with an inventory, gap analysis, prioritized roadmap, and explicit non-goals. Defer all product code changes. |
| Critical | "Verification first" is a principle, not an operational workflow. | No request-to-criterion mapping, criterion classes, evidence schema, or policy gate is specified. | Define a minimum verification model: requirement, criterion, verifier type, evidence, pass rule, flakiness policy, and decision owner. |
| High | The plan names Test API, Log API, Audit API, GitHub, linters, tickets, tests, and MCP tools without deciding which capability is evaluated first. | Research will sprawl and recommendations will not be comparable. | Limit discovery to four named surfaces: validation evidence, log linkage, audit trials, and MCP cost/ergonomics. Record each surface as existing capability, gap, proposed control, and validation method. |
| High | Deterministic, human, and LLM verification are mentioned but not distinguished by authority, cost, or repeatability. | An LLM judgment could silently become a release gate even when a local deterministic check exists. | Use a verifier hierarchy: local deterministic check first; hosted deterministic check second; human review for subjective criteria; LLM assessment only as advisory unless explicitly approved. |
| High | Flakiness is acknowledged without a measurement or disposition policy. | Repeated failures cannot be separated from unstable checks, and result quality cannot improve. | Require a baseline run count, captured environment/provenance, variance threshold, and a disposition: stable pass, stable fail, flaky, or inconclusive. |
| High | General product tooling and customer-project artifacts are not yet modeled as an ownership boundary. | The plan risks coupling shared product behavior to repository-specific data and policy. | Create a classification table with owner, storage scope, portability, and allowed dependencies for every proposed artifact. |
| Medium | The cost problem conflates tool interface friction with model routing and output volume. | Improvements could target the wrong layer and fail to reduce cost. | Evaluate each proposed improvement against token/context cost, tool-call count, user/agent round trips, and deterministic coverage. |
| Medium | The no-tool instruction conflicts with the request for research and a usable plan. | The constraint is ambiguous and could prevent evidence gathering. | Interpret the constraint as: do not mutate or operate product workflow tools in the session; source and documentation inspection is allowed. Record this interpretation in the revised plan. |
| Medium | The plan has no completion test of its own. | The session cannot answer the central question: whether all requested points are covered. | Add a requirements traceability matrix and a deterministic deliverable checklist. |

## Scope Decision

The one-session deliverable is complete only when the transcript folder
contains:

1. an index that links the dossier files and declares scope;
2. a research inventory of the four selected repository surfaces;
3. a verification operating model with criterion and evidence rules;
4. a prioritized roadmap of independently executable follow-up work packages;
5. a traceability checklist proving that the dossier covers the transcript.

The one-session deliverable excludes source-code changes, store migrations,
new MCP tools, new linter implementations, GitHub configuration, and ticket
state changes.

## Review Criteria

| Criterion | Review result |
| --- | --- |
| Preserves verification as the central thesis | Pass |
| Preserves the cost and MCP-ergonomics concern | Pass |
| Makes a one-session outcome bounded and feasible | Needs the revised plan |
| Defines evidence-backed research rather than generic exploration | Needs the revised plan |
| Makes the plan itself verifiable | Needs the revised plan |