# Production Workflow Cycle

## Purpose

Define the complete production workflow as a closed cycle: Request -> Spec ->
Tickets -> Tests -> Implementation -> Validated response -> Next iteration.
Each stage is a component that owns only its outward-facing obligations. The
following records are deliberately shaped to migrate mechanically into the
Waypoint 2 component-oriented artifact model.

## Target-model Encoding

`spec_id` in each record is `90e4fb79-2c60-42a6-ab10-91d243693150`. `context`,
`related_spec_ids`, `related_evidence_ids`, and `code_refs` are optional
component fields and are omitted when empty. Criteria without `validated_by`
are complete documented requirements, not failed validation.

### Evidence References

| id | spec_id | target_kind | target_ref | relation | locator |
| --- | --- | --- | --- | --- | --- |
| ev-core-cycle | `90e4fb79-2c60-42a6-ab10-91d243693150` | instruction | `.agents/instructions/orchestration/core-cycle.instructions.md` | defines | `Core Cycle; Contract Rules` |
| ev-loop-closure | `90e4fb79-2c60-42a6-ab10-91d243693150` | instruction | `.agents/instructions/orchestration/loop-closure.instructions.md` | constrains | `Loop Closure; Rules` |
| ev-phase-separation | `90e4fb79-2c60-42a6-ab10-91d243693150` | instruction | `.agents/instructions/orchestration/phase-separation.instructions.md` | constrains | `Discovery and Planning Phase; Implementation Phase` |
| ev-escalation-gate | `90e4fb79-2c60-42a6-ab10-91d243693150` | instruction | `.agents/instructions/orchestration/escalation-gate.instructions.md` | constrains | `Escalation Protocol; Rules` |

## Components And Provider-Owned Criteria

### Component: request

- `id`: `workflow-cycle.request`
- `spec_id`: `90e4fb79-2c60-42a6-ab10-91d243693150`
- `title`: `Request`
- `purpose`: Capture the requested outcome and any open questions that must be resolved before durable planning.
- `related_evidence_ids`: [`ev-core-cycle`, `ev-phase-separation`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| request-outcome | `workflow-cycle.request` | The request records the intended outcome. | [`ev-core-cycle`] |
| request-questions | `workflow-cycle.request` | The request records unresolved questions or states that none remain. | [`ev-core-cycle`] |

### Component: spec

- `id`: `workflow-cycle.spec`
- `spec_id`: `90e4fb79-2c60-42a6-ab10-91d243693150`
- `title`: `Spec`
- `purpose`: Define the durable goal, provider-owned acceptance criteria, and traceability before implementation planning.
- `related_evidence_ids`: [`ev-core-cycle`, `ev-phase-separation`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| spec-goal | `workflow-cycle.spec` | The spec defines the goal and definition of success for the requested work. | [`ev-core-cycle`] |
| spec-criteria | `workflow-cycle.spec` | The spec assigns outward-facing acceptance criteria to their owning components without consumer duplication. | [`ev-core-cycle`] |
| spec-traceability | `workflow-cycle.spec` | The spec records required evidence and related artifacts sufficient for implementation follow-through. | [`ev-core-cycle`] |

### Component: tickets

- `id`: `workflow-cycle.tickets`
- `spec_id`: `90e4fb79-2c60-42a6-ab10-91d243693150`
- `title`: `Tickets`
- `purpose`: Plan executable implementation slices that reference, rather than restate, the reviewed governing spec.
- `related_evidence_ids`: [`ev-core-cycle`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| tickets-spec-reference | `workflow-cycle.tickets` | Every implementation ticket references the governing spec. | [`ev-core-cycle`] |
| tickets-executable-slices | `workflow-cycle.tickets` | Tickets define executable slices and their dependencies. | [`ev-core-cycle`] |
| tickets-after-spec | `workflow-cycle.tickets` | Tickets are planned only after the governing spec is ready; tickets do not author or restate spec content. | [`ev-core-cycle`] |

### Component: tests

- `id`: `workflow-cycle.tests`
- `spec_id`: `90e4fb79-2c60-42a6-ab10-91d243693150`
- `title`: `Tests`
- `purpose`: Measure ticket-exposed criteria and preserve validation evidence, while allowing explicitly documented non-executable criteria.
- `related_evidence_ids`: [`ev-core-cycle`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| tests-criterion-exposure | `workflow-cycle.tests` | The validation plan exposes ticket acceptance criteria to measurement where feasible. | [`ev-core-cycle`] |
| tests-evidence-record | `workflow-cycle.tests` | Validation records link applicable tickets, specs, and acceptance criteria. | [`ev-core-cycle`] |
| tests-optional-automation | `workflow-cycle.tests` | A criterion without executable validation is explicitly documented and remains valid for review. | [`ev-core-cycle`] |

### Component: implementation

- `id`: `workflow-cycle.implementation`
- `spec_id`: `90e4fb79-2c60-42a6-ab10-91d243693150`
- `title`: `Implementation`
- `purpose`: Deliver the scoped change using a planned ticket and validation approach, then make the result review-ready.
- `related_evidence_ids`: [`ev-core-cycle`, `ev-phase-separation`, `ev-escalation-gate`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| implementation-scope | `workflow-cycle.implementation` | Implementation executes the planned ticket scope rather than rediscovering or redefining the requirement. | [`ev-phase-separation`] |
| implementation-evidence | `workflow-cycle.implementation` | The scoped change, required documentation, and validation results are available for the ticket review gate. | [`ev-core-cycle`] |
| implementation-escalation | `workflow-cycle.implementation` | Incomplete or ambiguous implementation handoff context is escalated before implementation proceeds. | [`ev-escalation-gate`] |

### Component: validated-response

- `id`: `workflow-cycle.validated-response`
- `spec_id`: `90e4fb79-2c60-42a6-ab10-91d243693150`
- `title`: `Validated response`
- `purpose`: Return an evidence-backed result to the user after review and validation are available.
- `related_evidence_ids`: [`ev-core-cycle`, `ev-loop-closure`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| response-evidence | `workflow-cycle.validated-response` | The response reports relevant validation outcomes and traceability for the completed work. | [`ev-core-cycle`] |
| response-user-judgment | `workflow-cycle.validated-response` | The response gives the user a result they can judge as satisfactory or requiring follow-up. | [`ev-core-cycle`] |
| response-review | `workflow-cycle.validated-response` | The response is issued only when review and validation evidence are available. | [`ev-loop-closure`] |

### Component: next-iteration

- `id`: `workflow-cycle.next-iteration`
- `spec_id`: `90e4fb79-2c60-42a6-ab10-91d243693150`
- `title`: `Next iteration`
- `purpose`: Record the user's judgment and either close the loop or convert follow-up into the next request.
- `related_evidence_ids`: [`ev-core-cycle`, `ev-loop-closure`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| next-iteration-judgment | `workflow-cycle.next-iteration` | The user's satisfaction or follow-up judgment is recorded. | [`ev-core-cycle`] |
| next-iteration-transition | `workflow-cycle.next-iteration` | A follow-up judgment becomes the next Request input; a satisfied judgment closes the cycle. | [`ev-core-cycle`, `ev-loop-closure`] |

## Directed Contract Edges

Each downstream consumer owns its dependency edge and references provider-owned
criteria without copying them. The final edge intentionally closes the cycle.

| id | consumer_component_id -> provider_component_id | name | provider_criterion_ids |
| --- | --- | --- | --- |
| edge-spec-consumes-request | `workflow-cycle.spec` -> `workflow-cycle.request` | Request informs specification | [`request-outcome`, `request-questions`] |
| edge-tickets-consume-spec | `workflow-cycle.tickets` -> `workflow-cycle.spec` | Specification governs ticket planning | [`spec-goal`, `spec-criteria`, `spec-traceability`] |
| edge-tests-consume-tickets | `workflow-cycle.tests` -> `workflow-cycle.tickets` | Ticket plan exposes validation work | [`tickets-spec-reference`, `tickets-executable-slices`] |
| edge-implementation-consumes-tests | `workflow-cycle.implementation` -> `workflow-cycle.tests` | Validation approach governs implementation evidence | [`tests-criterion-exposure`, `tests-evidence-record`, `tests-optional-automation`] |
| edge-response-consumes-implementation | `workflow-cycle.validated-response` -> `workflow-cycle.implementation` | Review-ready implementation supports response | [`implementation-scope`, `implementation-evidence`, `implementation-escalation`] |
| edge-next-iteration-consumes-response | `workflow-cycle.next-iteration` -> `workflow-cycle.validated-response` | User judgment follows validated response | [`response-evidence`, `response-user-judgment`, `response-review`] |
| edge-request-consumes-next-iteration | `workflow-cycle.request` -> `workflow-cycle.next-iteration` | Follow-up restarts the cycle | [`next-iteration-judgment`, `next-iteration-transition`] |

## Validation Observations

The following observation records capture the evidence available at authoring
time. They validate documented workflow guidance; they are not claims that the
target-model persistence or migration exists today.

| id | criterion_id | evidence_reference_id | status |
| --- | --- | --- | --- |
| obs-request-outcome | `request-outcome` | `ev-core-cycle` | documented |
| obs-spec-goal | `spec-goal` | `ev-core-cycle` | documented |
| obs-tickets-after-spec | `tickets-after-spec` | `ev-core-cycle` | documented |
| obs-tests-optional-automation | `tests-optional-automation` | `ev-core-cycle` | documented |
| obs-implementation-escalation | `implementation-escalation` | `ev-escalation-gate` | documented |
| obs-response-review | `response-review` | `ev-loop-closure` | documented |
| obs-next-iteration-transition | `next-iteration-transition` | `ev-core-cycle` | documented |

## Acceptance Matrix

| Acceptance area | Required result | Planned evidence |
| --- | --- | --- |
| Stage coverage | All seven cycle stages are component records with purpose and provider-owned criteria. | Spec body review; `spec.exe get <id> --json` |
| Handoff ownership | Each adjacent stage handoff is a consumer-to-provider edge whose criteria belong to the provider; the closing edge is present. | Spec body review |
| Evidence semantics | Existing instruction files are evidence references; lack of executable evidence does not invalidate a criterion. | Spec body review; `spec.exe health --all` |
| Current-schema health | The draft is structurally valid in the current spec store. | `./target/debug/spec.exe health --all` |

## Traceability And Non-Goals

- Related specs: `agent-workflow/iteration-loop` and `agent-workflow/handoff-package-schema` constrain the implementation-to-handoff transition; this spec does not redefine them.
- Related tickets: none. Waypoint 6 owns implementation-ticket creation after Waypoint 5 user review; ticket `5b50329b` is intentionally absent and must not be recreated.
- Non-goals: implement the target spec model; specify the spec system or adjacent ticket/doc/test tooling; create tickets; change agent instructions or roadmap materials.
- Planned verification: `./target/debug/spec.exe health --all` and `./target/debug/spec.exe get <id> --json`.