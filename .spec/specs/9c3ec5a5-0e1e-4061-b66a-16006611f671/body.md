# Goal

Coordinate the next major store vectors (interview, feedback, health auditing, and scaffold automation) as a dependency-aware program aligned to the cross-store architecture constraints.

## Program contract

- New domain stores must follow the architecture pattern captured in `target/tmp/architecture-decisions.md` and `architecture/cross-store-workspace-interaction`:
  - domain logic stays in domain crates
  - cross-domain interaction is mediated through lightweight contract crates (leaf nodes)
  - binaries are composition roots with static wiring
- All vectors must define explicit validation evidence and operator feedback loops.
- Rule-managed instruction and skill assets remain generated from canonical rule entries, not hand-maintained in generated files.

## Delivery waves

1. Wave 1: contract and schema baselines
  - lock interview, feedback, and health signal schemas
  - lock scaffold prompt contract and generated asset ownership rules
2. Wave 2: read-path capability and operator visibility
  - feedback deep search and interview/session inspection surfaces
  - health scoring and queueing in read-only recommendation mode
3. Wave 3: write-path automation and enforcement
  - reconciliation routing and answer-sheet iteration updates
  - scaffold generation + replay regression gates + drift checks

Wave sequencing matters: Wave 1 provides stable identifiers and signal semantics needed by health scoring and scaffold conformance checks.

## Major implementation vectors

1. Interview store: persistent sessions, editable surveys, and actionable answer synthesis.
2. Feedback store: high-volume event inbox with metadata indexing, search, and reconciliation.
3. Store health auditing: stale/conflicting/low-value entry scoring with cleanup workflows.
4. Scaffold automation: one-prompt slash command that bootstraps a minimally functional domain store with generated guidance.

## Parallelization strategy

- Interview and feedback vectors can run in parallel once shared contract expectations are set.
- Health metrics can begin with read-only scoring over existing stores before adding enforcement loops.
- Scaffold automation can run in parallel with interview/feedback, but prompt regressions gate rollout.

## Cross-vector dependency policy

- Health scoring depends on feedback indexing and validation-evidence modeling; it must consume shared derived signals rather than duplicate heuristics.
- Scaffold conformance checks depend on architecture policy from tracker `671d4e47` and the validation-aware dependency model from ticket graph work.
- Feedback and interview vectors remain independent at storage level but share integration requirements for cross-store references and audit ingestion.

## Shared contracts and ownership

- Interview and feedback stores own domain persistence and domain workflow semantics.
- Audit owns severity mapping and remediation guidance, but raw signal derivation should be shared and reusable.
- Scaffold owns generation flow and template composition, while rule-api owns canonical source guidance and target rendering.
- Ticket dependencies and validation evidence semantics remain owned by ticket-api and are consumed by audit and health scoring.

## Operator-facing outcomes

- Program must yield one prioritized cleanup and improvement queue that combines: validation deficits, stale entries, conflicting entries, and high-sentiment feedback pressure.
- Program must expose explainability metadata for each recommendation: source signals, score contributions, and recommended next action.
- Program must preserve reversible operations for cleanup actions (merge, deprecate, archive), with provenance links to triggering feedback and validation evidence.

## Global risks

- duplicated health/evidence logic across ticket/spec/rule/audit surfaces
- uncontrolled generated guidance drift if rule source-of-truth is bypassed
- weak provenance from user responses/feedback to actionable artifacts
- score gaming or noisy low-signal feedback dominating queues
- privileged-agent feedback misuse without explicit provenance and policy controls

## Acceptance criteria

- each vector has tracker and child tickets with dependency edges and effort metadata
- each vector has a planning spec with explicit validation requirements
- health checks on tickets/specs pass with zero findings

## Traceability

- [8a90a63c [program][multi-store] Store expansion and operational health program](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/8a90a63c-0a07-439f-90e8-9124212b2dc8/ticket.toml)
- [913fdd33 [interview-api] Interview sessions, survey orchestration, and answer synthesis](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/913fdd33-77b3-4e40-914a-db6873bf004d/ticket.toml)
- [b1e9e744 [feedback-api] Feedback inbox, metadata indexing, and deep search](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/b1e9e744-aeac-474a-91d9-07e3a362dc76/ticket.toml)
- [bd1c7cc0 [audit-api] Continuous store health scoring and cleanup loops](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/bd1c7cc0-2850-418d-b701-981b95c587ee/ticket.toml)
- [66fae806 [scaffold] Rule-generated store bootstrap instructions and slash command skill](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/66fae806-203d-4235-9151-4272eb0bb603/ticket.toml)
- [671d4e47 [architecture][multi-store] Tracker: cross-store interaction model and migration](C:/Users/linus/git/graph_app/context-engine/.ticket/tickets/671d4e47-b53d-4a04-aa1d-30f2aa8a2bbe/ticket.toml)

## Validation

- ticket health passes for all program and vector tracker subgraphs
- spec health passes for this spec and all vector specs
- dependency graph can be rendered deterministically in Mermaid for planning reviews
- wave gates are validated before downstream vectors advance to implementation
