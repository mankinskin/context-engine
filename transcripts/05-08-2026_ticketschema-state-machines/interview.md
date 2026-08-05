# Ticket Schema Modernization Interview

This file records the full interview conducted after the initial review of
[input.clean.md](input.clean.md). Entries are chronological. A decision marked
**Superseded** was replaced by a later answer and is not part of the final
planning contract.

## Final Planning Contract

- Feature functionality is represented by specs, not ticket types.
- New work-item ticket types are `epic`, `bug`, `research`, `planning`,
  `implementation`, `review`, `interview`, and `testing`.
- `task`, `feature`, and `tracker-improvement` are legacy ticket types until
  their individual Track 5 migrations.
- Schemas use strict, runtime, single-parent inheritance beneath a universal
  work-item schema. Resolved schemas are dynamically generated and cached.
- The universal work-item lifecycle categories are `plan`, `act`, and
  `verify`. Every concrete work-item type refines one or more nodes in each
  category and has at least one valid path across all three categories.
- A schema's directed state-transition structure is the **lifecycle graph**.
  Ticket links such as `depends_on`, `linked`, and
  `remediation_approved_by` form the **relation graph**.
- Derived schemas refine inherited lifecycle nodes into category-contained
  subgraphs. A resolved node retains the inherited category label. Edges may
  remain within a category or cross only between neighbouring category edges:
  `plan -> act`, `act -> verify`, `verify -> act` for rework, and
  `act -> plan` for replanning. No normal lifecycle skips are permitted.
- Each resolved work-item schema has exactly one plan-category root with no
  inbound lifecycle edge; creation begins there. Every transition names a
  concrete target state and is validated against the resolved lifecycle graph.
- Cancellation is a derived `verify` terminal leaf. It is the sole exception
  that may be reached directly from any lifecycle category. Terminal nodes
  have no outgoing edges.
- Ticket, spec, and existing rule schemas reuse shared directed lifecycle
  primitives while maintaining separate local graphs and separate type-ID
  namespaces.
- Schema inheritance is linear: zero or one parent; missing parents and cycles
  are rejected. Parent/cycle/type-ID validation occurs during an explicit,
  atomic load or reload. A failed reload retains the last valid registry.
- On explicit reload, source and ancestor content hashes invalidate affected
  resolved-schema cache entries, resolved manifests, catalog indexes, and
  client cache versions atomically.
- The typed resolved-registry manifest includes `model_version`, `sources`,
  `schemas`, and `graphs`. Schema records include IDs, parent chains, hashes,
  resolved nodes, transitions, terminals, and type metadata. Graph records
  name their entity kind and schema IDs.
- Shipped ticket schemas become JSON sources. External/custom schema directories
  permanently support both TOML and JSON through a shared model/parser.
  Built-in and custom schemas share one combined lexical path sort. Any
  duplicate type ID within one schema kind, including a built-in/custom
  collision, rejects the complete load and reports every colliding path in
  lexical order. Custom schemas cannot override built-ins. CI compares
  equivalent TOML/JSON fixtures by resolved manifest.
- Tracks are ordered: 1. engine/inheritance, 2. dual-format loader,
  3. generated catalog and shipped-ticket JSON conversion, 4. CLI and VS Code
  clients, 5. migration, 6. release validation.
- `bug` and `epic` retain their canonical IDs. Their schemas and active states
  upgrade in place only after global preflight. Active means `open`, `planned`,
  `in-implementation`, `in-review`, or `on-hold`. Each active record needs a
  direct lifecycle route or reviewer-selected remediation approved by a typed
  `remediation_approved_by` relation edge to a review ticket with an immutable
  decision part. Missing proof blocks the canonical-type upgrade and the whole
  release. Pre-model active records are exempt until individual migration.
- Current manifest rules govern a transition. All legacy records created before
  the model cutover are one exemption cohort: workflow checks are suspended
  until the individual record is migrated, prior history is grandfathered, and
  the first transition after migration follows the new model. A legacy
  `on-hold` record preserves its last non-on-hold lifecycle category.
- A Track 5 research sub-ticket defines deterministic rules-first classifier
  weights and ties for `task` and `tracker-improvement`, using title,
  description, fields, state, and relation edges against targets `research`,
  `planning`, `implementation`, `review`, `interview`, and `testing`. Only a
  unique top score of at least 0.80 auto-migrates. Tied, lower, and missing
  scores require a linked review-ticket decision, with no timeout or default.
- Legacy `feature` tickets do not use the classifier. A nonterminal linked
  child/dependency creates an epic; unchecked acceptance criteria alone create
  an implementation ticket; both create an epic; neither produces an archived
  spec. The shared active state list defines nonterminal work.
- Migration and approval evidence is immutable. The latest approved decision
  is authoritative. Track 5 dry-runs and applies idempotent transactional
  batches that roll back only a failed batch. A committed approved Track 5
  batch, with its immutable migration part, removes that record's exemption.
  A completion record ends Track 5.
- Track 6 blocks the entire release for any failed validation or corrective
  batch. Track 6 never rolls back completed batches or restores an exemption;
  Track 6 creates a new idempotent transactional corrective batch from current
  state. One corrective retry is allowed. A second failure keeps the release
  blocked until a human-approved remediation review ticket exists.
  Corrective work appends an immutable `corrective-migration` part that
  `supersedes` the original immutable migration part and records the review
  decision and current-state mapping. The latest approved superseding
  corrective part is authoritative for future validation.
- Validation requires focused unit and contract tests, CLI/MCP/HTTP parity,
  TOML/JSON resolved-manifest fixture parity, migration dry-run/idempotence/
  rollback/live-cutover/remediation tests, Playwright coverage for client work,
  and an all-pass Track 6 validation matrix.

## Interview Record

### Initial taxonomy and workflow decisions

1. **Question:** Which concrete ticket types should exist after the schema
   change?
   **Answer:** Feature work is spec-owned. Use `epic`, `bug`, `research`,
   `planning`, `implementation`, `review`, `interview`, and `testing`.

2. **Question:** Which inheritance and transition contract should schemas use?
   **Answer:** Initially selected no runtime inheritance with generated/shared
   templates. **Superseded** later by runtime inheritance.

3. **Question:** How much lifecycle enforcement should each concrete ticket
   type receive?
   **Answer:** Every ticket follows a universal `plan -> act -> review`
   lifecycle, and types refine the lifecycle into more specific steps.
   **Superseded terminology:** the final third category is `verify`, not
   `review`, to avoid conflict with the `review` ticket type.

4. **Question:** How should TOML and JSON schema files coexist?
   **Answer:** Initially selected both formats with duplicate type-ID rejection.
   This was briefly changed to JSON-only, then **superseded** by permanent dual
   support for external/custom TOML and JSON. Shipped built-ins convert to JSON.

5. **Question:** What should phase-one migration do with existing records?
   **Answer:** Inventory types/counts/states and preserve current records
   initially; mappings require dry-run validation. **Refined** later by the
   Track 5 live migration contract.

6. **Question:** Which implementation tracks should be created?
   **Answer:** Initially five linked tracks. **Superseded** by six tracks after
   the engine/inheritance work became an explicit prerequisite.

7. **Question:** May any ticket bypass the universal lifecycle?
   **Answer:** No waiver or privileged bypass in phase one.

### First roast follow-ups

8. **Question:** How should inventory gate client defaults and type pickers?
   **Answer:** No inventory gate; client/default integration may proceed before
   inventory compatibility results. This is an accepted tradeoff.

9. **Question:** How should directed lifecycle categories preserve legacy
   tickets?
   **Answer:** Initially selected global directed migration. **Refined** later:
   pre-model legacy records are exempt from workflow checks until individual
   migration.

10. **Question:** Must duplicate diagnostics be deterministic?
    **Answer:** Yes. Sort paths and report collisions deterministically.

11. **Question:** Where should global directed transition/category migration
    live?
    **Answer:** A separate sixth prerequisite track.

12. **Question:** Can a ticket return from verification to work for rework?
    **Answer:** Yes: explicit `verify -> act -> verify` lifecycle loop.

13. **Question:** How should legacy schemas be handled by the global engine?
    **Answer:** Initially selected exemption from category enforcement until
    type migration. **Final:** the whole pre-cutover legacy cohort suspends
    workflow checks until individual migration.

14. **Question:** Who produces TOML and JSON artifacts?
    **Answer:** Initially decided JSON-only. **Superseded** by JSON shipped
    built-ins plus permanently supported custom TOML/JSON inputs.

15. **Question:** Should duplicate diagnostics list every collision?
    **Answer:** Yes, all colliding paths in lexical order.

16. **Question:** How should `review` type be distinguished from lifecycle
    category?
    **Answer:** Rename the lifecycle category to `verify`.

17. **Question:** Are epics exempt from the universal lifecycle?
    **Answer:** No. An epic traverses all categories; its `act` is orchestration
    and dependency coordination.

### Cutover and migration decisions

18. **Question:** Are duplicate type IDs fatal?
    **Answer:** Yes. Every duplicate type ID is a full registry-load error.

19. **Question:** How does directed enforcement apply to historical legacy
    transitions?
    **Answer:** Prospectively only, anchored by the shared manifest
    `model_version` cutover. Old history is unchanged and grandfathered.

20. **Question:** Does Track 5 execute live migration?
    **Answer:** Yes. Track 5 inventories, maps, dry-runs, executes idempotent
    transactional batches, removes exemptions, and writes a completion record.

21. **Question:** How do migrated legacy records satisfy universal lifecycle
    evidence?
    **Answer:** Their pre-cutover history is grandfathered; subsequent
    transitions follow the new rules.

22. **Question:** What is the required track order?
    **Answer:** Final sequence: engine/inheritance, dual loader, catalog and
    shipped JSON conversion, clients, migration, validation.

23. **Question:** Are shipped built-ins all converted before conversion
    cutover?
    **Answer:** Yes. Every shipped ticket built-in is converted to JSON;
    built-in TOML sources are deleted in catalog/conversion work.

24. **Question:** What happens to existing `bug` and `epic` IDs?
    **Answer:** Retain IDs and upgrade their schemas and active states in place,
    subject to global preflight.

25. **Question:** Does the directed engine apply only to tickets?
    **Answer:** No. Ticket, spec, and existing rule schemas reuse shared
    directed primitives but define local lifecycle graphs.

26. **Question:** Which rule schemas are included?
    **Answer:** Existing rule-api schemas such as `rule-entry` and
    `generated-target`; no new rule entity is introduced.

27. **Question:** When must the dual loader exist relative to shipped TOML
    deletion?
    **Answer:** The dual loader lands before built-in TOML sources are deleted.

28. **Question:** How should external custom TOML schemas work?
    **Answer:** They remain permanently supported alongside custom JSON.

29. **Question:** May custom schemas override built-ins?
    **Answer:** No. Any collision, including built-in/custom, is fatal.

30. **Question:** What is the duplicate-ID namespace?
    **Answer:** Type IDs are unique per schema kind; ticket, spec, and rule
    registries have separate namespaces.

### Runtime inheritance and lifecycle graph decisions

31. **Question:** Should inheritance return to the design?
    **Answer:** Yes. Runtime inheritance resolves derived schemas dynamically
    and caches unchanged resolved schemas for low-latency use.

32. **Question:** What parent topology is allowed?
    **Answer:** Strict zero-or-one parent, linear parent chains only; missing
    parents and cycles fail.

33. **Question:** How should derived states relate to base lifecycle states?
    **Answer:** A derived schema refines inherited lifecycle nodes into contained
    subgraphs or tunnels. Resolved nodes retain inherited base-category labels.

34. **Question:** How does a transition enter a derived category tunnel?
    **Answer:** A transition explicitly names a concrete target state; the
    resolver validates the resolved concrete edge and inherited category pair.

35. **Question:** How should child state-object overrides work?
    **Answer:** Earlier field-wise merge and explicit entry-flag options were
    **superseded**. A child graph refines inherited nodes; child states remain
    valid under inherited category edges. The final contract expresses this as
    contained graph refinement, not a serialized state-object merge scheme.

36. **Question:** How is creation's initial state selected without an entry
    field?
    **Answer:** Each resolved work-item lifecycle graph has exactly one
    plan-category root with no inbound lifecycle edge. Creation starts there.

37. **Question:** How are category labels determined for tunnel edges?
    **Answer:** Every resolved node carries its inherited base category; an edge
    is validated using its source/target category pair.

38. **Question:** Which normal lifecycle transitions are legal?
    **Answer:** Within-category edges, `plan -> act`, `act -> verify`,
    `verify -> act` for rework, and `act -> plan` for replanning. No skips.

39. **Question:** How does cancellation fit the universal lifecycle?
    **Answer:** Final answer: cancellation is a derived `verify` terminal leaf
    with no outgoing edge. It is the sole exception that may be entered directly
    from any lifecycle category.

40. **Question:** What is a terminal state?
    **Answer:** A resolved lifecycle node with no outgoing transition.

41. **Question:** What names distinguish state changes from ticket links?
    **Answer:** `lifecycle graph` for directed schema state transitions and
    `relation graph` for ticket links such as `depends_on`, `linked`, and
    remediation relations.

42. **Question:** Do relation edges have lifecycle-category constraints?
    **Answer:** No. Relation edges may cross ticket types and categories freely;
    validation applies declared relation-kind rules and endpoint existence.

### Registry, manifest, and reload decisions

43. **Question:** When do parent/cycle/type-ID failures occur?
    **Answer:** During explicit whole-registry load or reload; a failure retains
    the prior valid generation.

44. **Question:** What invalidates resolved schema caches?
    **Answer:** Content hash changes to a schema source or ancestor, checked on
    explicit reload.

45. **Question:** Is cache invalidation atomic?
    **Answer:** Yes. Registry generation, schema cache, manifest/catalog, and
    client cache version commit together or the prior generation remains.

46. **Question:** What does the generated registry manifest contain?
    **Answer:** `model_version`, `sources`, `schemas`, and `graphs`; schema
    records have IDs, parent chains, hashes, resolved nodes/transitions/
    terminals, and type metadata; graph records name entity kind and schema IDs.

47. **Question:** How is format parity enforced?
    **Answer:** TOML and JSON deserialize through a shared model; CI fixture
    pairs compare resolved manifest output.

48. **Question:** How are duplicate diagnostic paths ordered?
    **Answer:** Sort the union of all eligible TOML and JSON paths lexically,
    then report all collisions in that order.

### Migration, classifier, and evidence decisions

49. **Question:** What qualifies a ticket as active for bug/epic preflight and
    feature follow-up?
    **Answer:** `open`, `planned`, `in-implementation`, `in-review`, and
    `on-hold`. The same active-state list is reused.

50. **Question:** What happens to a bug/epic active ticket without a direct
    continuation route?
    **Answer:** A reviewer may select remediation instead. A typed
    `remediation_approved_by` relation edge must point to a review ticket with
    an immutable approval decision part. Direct route and remediation are
    alternative sufficient proofs.

51. **Question:** What happens if an active bug/epic ticket lacks both proofs?
    **Answer:** The canonical-type upgrade and the whole release are blocked.

52. **Question:** Are pre-model active tickets subject to this preflight?
    **Answer:** No. They are exempt until their individual Track 5 migration.

53. **Question:** Which records are the legacy exemption cohort?
    **Answer:** All legacy records created before the model-version cutover.
    Workflow checks are suspended until individual migration; historical
    evidence is grandfathered.

54. **Question:** When does a legacy record leave the exemption cohort?
    **Answer:** Only when a committed, approved Track 5 migration batch writes
    that record's immutable migration part.

55. **Question:** How does `on-hold` map during legacy migration?
    **Answer:** Preserve and resume the ticket's last non-on-hold lifecycle
    category.

56. **Question:** Which types does the score-based classifier handle?
    **Answer:** Only legacy `task` and `tracker-improvement` tickets.

57. **Question:** What are classifier targets and inputs?
    **Answer:** Targets are `research`, `planning`, `implementation`, `review`,
    `interview`, and `testing`. Inputs are title, description, fields, state,
    and relation edges. A Track 5 research sub-ticket defines concrete rules,
    weights, and tie calculation.

58. **Question:** Which classifier result may auto-migrate?
    **Answer:** Only a unique top score of at least 0.80. Ties at any score,
    lower scores, and missing scores require linked review-ticket approval with
    no timeout or default.

59. **Question:** How are classifier and approval results stored?
    **Answer:** In immutable migration parts recording classifier version,
    candidate scores, selected target, rules, explanation, and approval.

60. **Question:** How are concurrent decisions resolved?
    **Answer:** The latest approved immutable decision part is authoritative;
    conflicting pending parts block migration.

61. **Question:** Does the feature classifier use score-based migration?
    **Answer:** No. Legacy `feature` tickets use only the deterministic feature
    conversion table.

62. **Question:** What is the legacy feature conversion table?
    **Answer:** An active linked child/dependency creates an epic. Unchecked
    acceptance criteria alone create an implementation ticket. Both conditions
    create an epic. Neither creates an archived spec.

63. **Question:** How are corrective migrations represented?
    **Answer:** Original migration parts remain immutable. Track 6 appends a
    new immutable `corrective-migration` part linked by `supersedes`, recording
    the remediation review decision and current-state mapping. The latest
    approved superseding corrective part is authoritative.

### Release, repair, and validation decisions

64. **Question:** What does a Track 5 batch failure do?
    **Answer:** Roll back the affected transactional batch. Batches are
    idempotent and dry-run before live application.

65. **Question:** When does Track 5 end?
    **Answer:** After all approved batches commit and a migration-completion
    record is written.

66. **Question:** What does Track 6 do after Track 5 completion?
    **Answer:** Validate all evidence, block the entire release on any failing
    validation or forward-repair batch, never roll back completed batches or
    restore exemptions, and repair forward using new transactional corrective
    migration batches from current state.

67. **Question:** How many corrective retries are allowed?
    **Answer:** One repaired retry. A second failure keeps release blocked and
    requires a human-approved remediation review ticket before corrective work
    may continue.

68. **Question:** What validation evidence is required?
    **Answer:** Focused unit/contract tests; CLI/MCP/HTTP parity; TOML/JSON
    resolved-manifest fixture parity; migration dry-run, idempotence, rollback,
    live-cutover, and remediation authorization tests; Playwright client tests;
    and an all-pass Track 6 matrix.

## Final Review

The final Roast Agent reviewed the completed contract and returned:

```text
verdict: pass
findings: []
```

The final Roast Agent identified residual implementation risks to capture in
future acceptance criteria, not unresolved design decisions:

- Enforce selection of the latest approved `supersedes` chain.
- Persist and enforce the one-corrective-retry limit.
- Define the terminal handling of a rejected remediation review decision.
