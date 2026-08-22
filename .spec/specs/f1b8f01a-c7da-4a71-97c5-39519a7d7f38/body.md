# Component-Oriented Specification System

## Purpose

Define the target specification-system artifact model and the contracts it
requires from ticket, document, and validation tooling. Components exclusively
own their outward-facing criteria; consumers record dependency edges that refer
to provider criteria without copying them. These records deliberately match the
Waypoint 2 target model and will migrate mechanically when it is implemented.

## Target-model Encoding

`spec_id` in each record is `f1b8f01a-c7da-4a71-97c5-39519a7d7f38`. `context`,
`related_spec_ids`, `related_evidence_ids`, and `code_refs` are optional
component fields and are omitted when empty. A criterion with no `validated_by`
is complete; health never requires satisfied evidence.

### Evidence References

| id | spec_id | target_kind | target_ref | relation | locator |
| --- | --- | --- | --- | --- | --- |
| ev-roadmap | `f1b8f01a-c7da-4a71-97c5-39519a7d7f38` | document | `transcripts/22-08-2026_spec-system-workflow-cycle/ROADMAP.md` | defines | `Resolved Decisions; Waypoints 2, 4, 7, 8, 12` |
| ev-schema | `f1b8f01a-c7da-4a71-97c5-39519a7d7f38` | code | `workflow-tools/spec/crates/spec-api/schemas/specification.toml` | supersedes | `Current specification manifest schema` |
| ev-ticket-ref | `f1b8f01a-c7da-4a71-97c5-39519a7d7f38` | code | `workflow-tools/spec/crates/spec-api/src/ticket_ref.rs` | constrains | `TicketRef explicit workspace/store-root resolution` |
| ev-workflow-cycle | `f1b8f01a-c7da-4a71-97c5-39519a7d7f38` | spec | `90e4fb79-2c60-42a6-ab10-91d243693150` | related | `Production Workflow Cycle` |
| ev-schema-test | `f1b8f01a-c7da-4a71-97c5-39519a7d7f38` | test | `workflow-tools/spec/crates/spec-api/tests/schema_test.rs` | validates | `Schema compatibility test suite` |
| ev-health | `f1b8f01a-c7da-4a71-97c5-39519a7d7f38` | command | `./target/debug/spec.exe --workspace . health --all` | validates | `Spec-store structural health` |

## Components And Provider-Owned Criteria

### Component: specification-root

- `id`: `spec-system.specification-root`
- `spec_id`: `f1b8f01a-c7da-4a71-97c5-39519a7d7f38`
- `title`: `Specification root`
- `purpose`: Preserve root identity, lifecycle, hierarchy, and the root-scoped namespace for target artifacts.
- `related_evidence_ids`: [`ev-schema`, `ev-roadmap`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| root-surviving-fields | `spec-system.specification-root` | A specification root retains `id`, lifecycle, `title`, `slug`, `type`, `state`, `scope`, `parent`, `code_refs`, sections, hierarchy, and structured ticket references. | [`ev-schema-test`] |
| root-component-classification | `spec-system.specification-root` | The existing root `component` classification remains distinct from a component artifact. | |
| root-artifact-namespace | `spec-system.specification-root` | Components, criteria, evidence references, and directed contract edges are scoped to exactly one specification root. | |

### Component: component-artifact

- `id`: `spec-system.component-artifact`
- `spec_id`: `f1b8f01a-c7da-4a71-97c5-39519a7d7f38`
- `title`: `Component artifact`
- `purpose`: Describe a participant that owns zero or more outward-facing criteria.
- `related_evidence_ids`: [`ev-roadmap`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| component-required-fields | `spec-system.component-artifact` | Every component has required `id`, `spec_id`, `title`, and `purpose` fields. | [`ev-schema-test`] |
| component-optional-fields | `spec-system.component-artifact` | A component may carry `context`, `related_spec_ids`, `related_evidence_ids`, and `code_refs`. | |
| component-root-membership | `spec-system.component-artifact` | Every component belongs to its declared specification root. | |
| component-criterion-ownership | `spec-system.component-artifact` | A component exclusively owns zero or more criteria that define its outward-facing obligations. | |

### Component: criterion-artifact

- `id`: `spec-system.criterion-artifact`
- `spec_id`: `f1b8f01a-c7da-4a71-97c5-39519a7d7f38`
- `title`: `Criterion artifact`
- `purpose`: State one provider-owned, root-unique acceptance obligation.
- `related_evidence_ids`: [`ev-roadmap`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| criterion-required-fields | `spec-system.criterion-artifact` | Every criterion has required `id`, `spec_id`, `owner_component_id`, and `statement` fields. | [`ev-schema-test`] |
| criterion-single-owner | `spec-system.criterion-artifact` | Each criterion has exactly one owner component in the same root. | |
| criterion-root-unique | `spec-system.criterion-artifact` | Criterion identifiers are unique within their root. | |
| criterion-optional-validation | `spec-system.criterion-artifact` | `validated_by` is optional; an unvalidated documented criterion is valid and complete. | |
| criterion-evidence-integrity | `spec-system.criterion-artifact` | Every evidence identifier in `validated_by` resolves to an evidence reference in the same root. | |

### Component: evidence-reference-artifact

- `id`: `spec-system.evidence-reference-artifact`
- `spec_id`: `f1b8f01a-c7da-4a71-97c5-39519a7d7f38`
- `title`: `Evidence reference artifact`
- `purpose`: Identify external material that defines, constrains, or validates a criterion without embedding fulfillment state.
- `related_evidence_ids`: [`ev-ticket-ref`, `ev-roadmap`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| evidence-required-fields | `spec-system.evidence-reference-artifact` | Every evidence reference has required `id`, `spec_id`, `target_kind`, `target_ref`, and `relation` fields. | [`ev-schema-test`] |
| evidence-optional-locator | `spec-system.evidence-reference-artifact` | An evidence reference may carry a `locator` for an exact target location. | |
| evidence-explicit-store-target | `spec-system.evidence-reference-artifact` | A store-backed target carries explicit workspace or store-root metadata and never relies on implicit resolution. | [`ev-ticket-ref`] |
| evidence-external-state | `spec-system.evidence-reference-artifact` | Evidence references link external material; they do not require or encode mandatory fulfillment state. | |

### Component: directed-contract-edge-artifact

- `id`: `spec-system.directed-contract-edge-artifact`
- `spec_id`: `f1b8f01a-c7da-4a71-97c5-39519a7d7f38`
- `title`: `Directed contract edge artifact`
- `purpose`: Record a consumer dependency on one or more provider-owned criteria.
- `related_evidence_ids`: [`ev-roadmap`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| edge-required-fields | `spec-system.directed-contract-edge-artifact` | Every edge has required `id`, `spec_id`, `consumer_component_id`, `provider_component_id`, `provider_criterion_ids`, and `name` fields. | [`ev-schema-test`] |
| edge-nonempty-provider-criteria | `spec-system.directed-contract-edge-artifact` | An edge references at least one provider criterion. | |
| edge-provider-ownership | `spec-system.directed-contract-edge-artifact` | Every referenced criterion belongs to the provider component. | |
| edge-distinct-endpoints | `spec-system.directed-contract-edge-artifact` | A consumer and provider are distinct components. | |
| edge-cycles-allowed | `spec-system.directed-contract-edge-artifact` | Contract-edge cycles are valid. | |
| edge-unique-claim | `spec-system.directed-contract-edge-artifact` | No root duplicates a `(consumer, provider, criterion)` dependency claim. | |
| edge-consumer-does-not-copy | `spec-system.directed-contract-edge-artifact` | A consumer owns its dependency edge but never restates the provider criterion. | |

### Component: validation-observation-artifact

- `id`: `spec-system.validation-observation-artifact`
- `spec_id`: `f1b8f01a-c7da-4a71-97c5-39519a7d7f38`
- `title`: `Validation observation artifact`
- `purpose`: Record an optional result against a criterion and evidence reference.
- `related_evidence_ids`: [`ev-roadmap`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| observation-required-fields | `spec-system.validation-observation-artifact` | Every observation has required `id`, `criterion_id`, `evidence_reference_id`, and `status` fields. | [`ev-schema-test`] |
| observation-optional-detail | `spec-system.validation-observation-artifact` | An observation may carry `observed_at` and `detail`. | |
| observation-reference-integrity | `spec-system.validation-observation-artifact` | An observation resolves its criterion and evidence reference within the owning root. | |
| observation-does-not-gate-health | `spec-system.validation-observation-artifact` | Omitted observations and unsatisfied evidence do not make a criterion or specification structurally unhealthy. | [`ev-health`] |

### Component: specification-store

- `id`: `spec-system.specification-store`
- `spec_id`: `f1b8f01a-c7da-4a71-97c5-39519a7d7f38`
- `title`: `Specification store`
- `purpose`: Persist roots and target artifacts while retaining existing section and hierarchy behavior.
- `related_evidence_ids`: [`ev-schema`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| store-persists-artifacts | `spec-system.specification-store` | The store persists root-scoped components, criteria, evidence references, contract edges, and observations. | [`ev-schema-test`] |
| store-preserves-baselines | `spec-system.specification-store` | Sections, hierarchy, and TicketRef behavior remain compatible with their current baselines. | |
| store-removes-retired-model | `spec-system.specification-store` | The store retires `contract_mode`, `expected_properties`, mandatory evidence requirements, and fulfillment summaries from the target contract. | |

### Component: health-check

- `id`: `spec-system.health-check`
- `spec_id`: `f1b8f01a-c7da-4a71-97c5-39519a7d7f38`
- `title`: `Specification health check`
- `purpose`: Report structural validity without turning best-effort validation into a gating requirement.
- `related_evidence_ids`: [`ev-health`]

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| health-validates-references | `spec-system.health-check` | Health validates required fields, root membership, ownership, uniqueness, and artifact references. | [`ev-health`] |
| health-allows-unvalidated-criteria | `spec-system.health-check` | Health accepts criteria with no `validated_by` and specifications with no validation observations. | [`ev-health`] |
| health-no-fulfillment-gate | `spec-system.health-check` | Health no longer requires satisfied evidence or fulfillment summaries. | [`ev-health`] |

### Component: ticket-store

- `id`: `adjacent.ticket-store`
- `spec_id`: `f1b8f01a-c7da-4a71-97c5-39519a7d7f38`
- `title`: `Ticket store`
- `purpose`: Plan implementation work under a governing specification and enforce the required ticket-to-spec relationship.

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| ticket-governing-spec | `adjacent.ticket-store` | A ticket that plans or fulfills governed work references a governing specification as a readiness-gating relationship, not merely an informational reference. | |
| ticket-explicit-spec-target | `adjacent.ticket-store` | The ticket-to-spec relationship identifies its specification target without implicit workspace or store-root resolution. | |
| ticket-spec-before-plan | `adjacent.ticket-store` | A ticket is created to plan implementation after its governing spec exists; it does not author or restate the requirement. | [`ev-workflow-cycle`] |

### Component: document-store

- `id`: `adjacent.document-store`
- `spec_id`: `f1b8f01a-c7da-4a71-97c5-39519a7d7f38`
- `title`: `Document store`
- `purpose`: Supply stable, locatable documents as evidence-reference targets.

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| document-stable-target | `adjacent.document-store` | A document can be referenced by a stable target reference and optional locator. | |
| document-resolution-result | `adjacent.document-store` | Document resolution exposes enough identity and location to review evidence links. | |
| document-evidence-nongating | `adjacent.document-store` | An unavailable or unobserved document result does not create a mandatory-evidence health gate. | |

### Component: validation-store

- `id`: `adjacent.validation-store`
- `spec_id`: `f1b8f01a-c7da-4a71-97c5-39519a7d7f38`
- `title`: `Test and validation store`
- `purpose`: Supply executable or recorded evidence that can be associated with criteria without requiring automation for every criterion.

| id | owner_component_id | statement | validated_by |
| --- | --- | --- | --- |
| validation-criterion-link | `adjacent.validation-store` | Validation evidence can identify applicable specification and criterion targets. | |
| validation-observation-source | `adjacent.validation-store` | Validation outcomes expose a status and optional time/detail for an observation. | |
| validation-best-effort | `adjacent.validation-store` | The absence of executable validation remains a documented, reviewable outcome. | [`ev-workflow-cycle`] |

## Directed Contract Edges

Each consumer owns its edge and references provider-owned criteria. Cycles are
allowed by the artifact contract, though none are needed for this slice.

| id | consumer_component_id -> provider_component_id | name | provider_criterion_ids |
| --- | --- | --- | --- |
| edge-component-root | `spec-system.component-artifact` -> `spec-system.specification-root` | Component belongs to specification root | [`root-artifact-namespace`] |
| edge-criterion-component | `spec-system.criterion-artifact` -> `spec-system.component-artifact` | Criterion is owned by component | [`component-criterion-ownership`] |
| edge-criterion-root | `spec-system.criterion-artifact` -> `spec-system.specification-root` | Criterion uses root namespace | [`root-artifact-namespace`] |
| edge-evidence-root | `spec-system.evidence-reference-artifact` -> `spec-system.specification-root` | Evidence belongs to specification root | [`root-artifact-namespace`] |
| edge-contract-consumes-component | `spec-system.directed-contract-edge-artifact` -> `spec-system.component-artifact` | Edge targets declared components | [`component-root-membership`, `component-criterion-ownership`] |
| edge-contract-consumes-criterion | `spec-system.directed-contract-edge-artifact` -> `spec-system.criterion-artifact` | Edge claims provider criteria | [`criterion-single-owner`, `criterion-root-unique`] |
| edge-observation-consumes-criterion | `spec-system.validation-observation-artifact` -> `spec-system.criterion-artifact` | Observation addresses criterion | [`criterion-required-fields`] |
| edge-observation-consumes-evidence | `spec-system.validation-observation-artifact` -> `spec-system.evidence-reference-artifact` | Observation cites evidence reference | [`evidence-required-fields`] |
| edge-root-consumes-store | `spec-system.specification-root` -> `spec-system.specification-store` | Root is persisted by specification store | [`store-persists-artifacts`, `store-preserves-baselines`] |
| edge-health-consumes-store | `spec-system.health-check` -> `spec-system.specification-store` | Health reads persisted artifacts | [`store-persists-artifacts`] |
| edge-spec-consumes-ticket-gate | `spec-system.specification-root` -> `adjacent.ticket-store` | Ticket depends on governing specification | [`ticket-governing-spec`, `ticket-explicit-spec-target`, `ticket-spec-before-plan`] |
| edge-evidence-consumes-documents | `spec-system.evidence-reference-artifact` -> `adjacent.document-store` | Evidence references locatable documents | [`document-stable-target`, `document-resolution-result`] |
| edge-observation-consumes-validation | `spec-system.validation-observation-artifact` -> `adjacent.validation-store` | Observations consume validation outcomes | [`validation-criterion-link`, `validation-observation-source`, `validation-best-effort`] |

## Validation Observations

These records document evidence available at authoring time, not a claim that
the target persistence model has already been implemented.

| id | criterion_id | evidence_reference_id | status |
| --- | --- | --- | --- |
| obs-root-surviving-fields | `root-surviving-fields` | `ev-schema` | documented |
| obs-criterion-required-fields | `criterion-required-fields` | `ev-roadmap` | documented |
| obs-evidence-explicit-store-target | `evidence-explicit-store-target` | `ev-ticket-ref` | documented |
| obs-edge-required-fields | `edge-required-fields` | `ev-roadmap` | documented |
| obs-health-no-fulfillment-gate | `health-no-fulfillment-gate` | `ev-roadmap` | documented |
| obs-ticket-spec-before-plan | `ticket-spec-before-plan` | `ev-workflow-cycle` | documented |

## Acceptance Matrix

| Acceptance area | Required result | Planned evidence |
| --- | --- | --- |
| Artifact completeness | The five target artifact kinds carry their required and optional fields and stated invariants. | `cargo test -p spec-api --test schema_test`; targeted store tests |
| Ownership and dependencies | Criteria have exactly one provider owner; consumer edges reference provider criteria without copying them; cycles remain valid. | Schema/store tests with a two-component cycle |
| Current-to-target delta | Root fields survive; criteria gain owner and optional validation; components and edges are added; evidence fulfillment fields and retired contract fields are absent. | Schema and manifest tests |
| Health semantics | Structural integrity is checked without requiring satisfied evidence, observations, or executable validation. | `./target/debug/spec.exe --workspace . health --all` |
| Adjacent-tool interoperability | Ticket gating, locatable document evidence, and best-effort validation observations are each represented by a directed edge. | Ticket/doc/test API tests created under the reviewed implementation tickets |
| Current-schema health | This draft remains valid while target records are body-encoded. | `./target/debug/spec.exe --workspace . get f1b8f01a-c7da-4a71-97c5-39519a7d7f38 --json`; `./target/debug/spec.exe --workspace . health --all` |

## Traceability And Non-Goals

- Related specs: `90e4fb79-2c60-42a6-ab10-91d243693150` defines the production workflow cycle; this spec defines the spec-system and adjacent-tool contracts it consumes. Existing implementation-level `spec-api` and ticket-api specifications remain neighbors, not duplicate owners of this cross-tool target contract.
- Related tickets: none. Waypoints 6 and 12 create implementation tickets only after Waypoint 5 review. The named `ticket-governing-spec` criterion and `edge-spec-consumes-ticket-gate` edge are the governing contract for Waypoint 12.
- Non-goals: implement schemas, storage, API, health, migration, or ticket gating; re-specify the production workflow cycle; create tickets; modify agent guidance, the roadmap, or presentation materials.
- Planned verification: `cargo test -p spec-api --test schema_test`, focused manifest/store tests, `./target/debug/spec.exe --workspace . get f1b8f01a-c7da-4a71-97c5-39519a7d7f38 --json`, and `./target/debug/spec.exe --workspace . health --all`.
