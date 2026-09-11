# Artifacts — Merged Spec-System / Workflow-Cycle Dossier

Union of both source dossiers' artifact inventories, deduplicated by id/path. See `sources/` for each dossier's original, unmodified `ARTIFACTS.md`.

| Artifact | State | Relevance |
|---|---|---|
| [ROADMAP.v6.md](ROADMAP.v6.md), [README.v1.md](README.v1.md) | Preserved Stage 3 entrypoint baseline | Retains the Stage 3 planning context before Stage 4 replaces the active roadmap and README. |
| [01-migration-tooling.md](01-migration-tooling.md) through [05-final-validation.md](05-final-validation.md) | Stage 4 planning artifacts | Bounded remaining route: migration tooling, W6 closure, dogfood migration, documentation/guidance, and final validation. No package authorizes execution. |
| [30bbbace W6.1](../../../.workflow-tools/ticket/tickets/30bbbace-7ce3-4f7e-acb4-202d08cf91d7/ticket.toml) | Open; blocked by [73b2cd22](../../../.workflow-tools/ticket/tickets/73b2cd22-942b-4205-86e5-333df2373211/ticket.toml), also open | Authoritative migration-tooling tracker and dependency. Stage 4 preserves the dependency without mutation. |
| [f1b8f01a Component-Oriented Specification System](../../../.workflow-tools/spec/specs/f1b8f01a-c7da-4a71-97c5-39519a7d7f38/spec.toml), [fa6e85c8 Worktree Control Component Pilot](../../../.workflow-tools/spec/specs/fa6e85c8-866c-4d53-bb50-b78bd651e8ce/spec.toml) | Both `draft` | Planning-approved governing specifications; Stage 4 does not transition their state. |
| [AGENTS.md](../../../workflow-tools/AGENTS.md) | Exists; Operating Principles / Task Routing / Quality Gates, no unified cycle description | Closest existing statement of the cycle principle; deliberately kept small — new cycle content lives in a dedicated instruction file cross-linked from here. |
| [workflow instructions](../../../workflow-tools/ticket/.agents/instructions/ticket/workflow.instructions.md) | Exists; `[[refs]]` typed-reference table (kind `spec` among others) | `[[refs]]` lets a ticket point at a spec informationally but does not gate ticket actionability. |
| [lifecycle instructions](../../../workflow-tools/ticket/.agents/instructions/ticket/lifecycle.instructions.md) | Exists; state machine, `in-review` gate, Review Gate Before Closing checklist | Already the closest existing "response quality gate" in the cycle. |
| [phase-separation instructions](../../../workflow-tools/.agents/instructions/workflow/phase-separation.instructions.md) | Exists | Discovery/planning before implementation — general form of the cycle's spec/ticket-before-code ordering. |
| [escalation-gate instructions](../../../workflow-tools/.agents/instructions/workflow/escalation-gate.instructions.md) | Exists | Governs the cycle's "return to the user" step when the loop can't close cleanly. |
| [spec prompt](../../../workflow-tools/spec/.agents/prompts/spec.prompt.md), [spec agent](../../../workflow-tools/spec/.agents/agents/spec.agent.md) | Exist | Current spec-authoring tooling — the mechanism the roadmap uses for later spec waypoints. |
| Ticket schema `depends_on`/`blocks`/`linked` edges; `[[refs]]` `kind=spec` | `depends_on` gates ticket-to-ticket; `spec` is informational-only (`spec_refs` observed, e.g. `memory-viewers/.ticket/tickets/0b7da330-.../history.ndjson`) | Confirms ticket-depends-on-spec is **not** a gating edge today — real ticket-api gap, tracked as a future ticket per Waypoint 12 below (not yet created; see that row). |
| test-api (`mcp_test-mcp_record_execution`/`record_spec`) — `spec_ids`, `ticket_ids`, `acceptance_criterion_ids` | Exists, already links an execution to both a spec and a ticket | Already implements the cycle's "executable measurements validate the spec" step. |
| [.presentation/deck.toml](../../.presentation/deck.toml), [.presentation/README.md](../../.presentation/README.md) | Exists; root deck `id = "context-engine"`, composes `workflow-tools` | Correct, repo-wide deck for the full-cycle diagram waypoint. |
| [transcripts/20-08-2026_presentation-automation-planning/](../20-08-2026_presentation-automation-planning/) | Prior dossier | Prior presentation-automation planning; the presentation waypoint below must extend this, not bypass it. |
| Ticket-depends-on-spec gating edge (originally created as `5b50329b`) | **Intentionally deleted by the requester** — it was created before its own governing spec existed, which the corrected spec-before-ticket ordering (see `ROADMAP.md`'s Resolved Decisions) makes premature. This was confirmed correct, not a gap to patch by recreating the ticket; a repo-wide search of every `transcripts/**/*.md` roadmap found no other ticket created the same premature way. | Genuinely distinct from Dossier B's directed-contract edge (ticket-to-spec vs. component-to-component) — see duplication-review verdict below. Do not recreate until Waypoint 4's adjacent-tooling spec covers this ticket-api gating-edge design. |
| [2ccde9ee Presentation System spec](../../../.workflow-tools/spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/spec.toml) | Reviewed manifest + prose `body.md` | Shared case-study anchor in both source dossiers: monolithic-shape example the new spec-system model must be able to represent. |
| [0ee95228 presentation epic](../../../.workflow-tools/ticket/tickets/0ee95228-475d-4706-a108-fd208f7c4098/ticket.toml) | Existing epic | Owns Presentation-System-related ticket work; the presentation-deck waypoint must coordinate with it, not duplicate it. |
| [workflow-tools/spec/crates/spec-api/src/manifest.rs](../../../workflow-tools/spec/crates/spec-api/src/manifest.rs) | Implemented (v2) | `SpecManifest` declares CURRENT_FORMAT_VERSION = 2 and exposes v2 helpers (`format_version()`, `is_v2()`, `legacy_ticket_link_entries()`) plus setters for `component_id` and typed manifest fields (criteria, evidence, outward edges). Evidence: implementation + schema tests pass. |
| [workflow-tools/spec/crates/spec-api/schemas/specification.toml](../../../workflow-tools/spec/crates/spec-api/schemas/specification.toml) | Implemented (v2 schema) | Declares v2 fields (`format_version`, `component_id`, `criteria`, `evidence`, `outward_contract_edges`) and lifecycle rules (`required_states`, directed edges) used by `spec-api` tests and validation commands. |
| [workflow-tools/spec/crates/spec-api/src/store/sections.rs](../../../workflow-tools/spec/crates/spec-api/src/store/sections.rs) | Existing implementation | Named spec-section primitive, reusable for component sub-content. |
| [workflow-tools/spec/crates/spec-api/src/store/hierarchy.rs](../../../workflow-tools/spec/crates/spec-api/src/store/hierarchy.rs) | Existing implementation | Hierarchy/parent-child traversal, reusable for component-to-system nesting. |
| [workflow-tools/spec/crates/spec-api/src/ticket_ref.rs](../../../workflow-tools/spec/crates/spec-api/src/ticket_ref.rs) | Existing implementation | Structured cross-store ticket reference — template shape for the new evidence-reference artifact. |
| [workflow-tools/spec/crates/spec-api/tests/schema_test.rs](../../../workflow-tools/spec/crates/spec-api/tests/schema_test.rs) | Existing validation | Schema test coverage; asserts v2 fields and lifecycle invariants; keep passing as schema gains artifact kinds. |

| Legacy UUID → immutable `component_id` mapping file | MISSING | Roadmap and cutover runbooks require an explicit legacy-id → `component_id` mapping artifact to deterministically assign destination `component_id`s during migration. No authoritative mapping file was found in the repository (search of `**/*mapping*` and migration scripts returned only ad-hoc references). |

| Journaled, idempotent migration runner (`spec migrate` / migration CLI) | MISSING | No single canonical, journaled, idempotent migration runner was found in `spec-api` or adjacent crates. Migration logic exists in tests and scripts (e.g. `workflow-tools/test/crates/test-api/src/migration.rs` and `transcripts/*/migrate-domain.sh`), but a production `spec migrate` runner (with dry-run/preflight, journaled progress, resume/rollback) is not present and is required by the ROADMAP's Waypoint cutover gates. |

| [workflow-tools/test/crates/test-api/src/migration.rs](../../../workflow-tools/test/crates/test-api/src/migration.rs) | Exists (test harness) | Contains resumable migration logic used by test fixtures and validation runs — useful prototype evidence but not a production runner; shows migration primitives and resume semantics that a canonical runner should adopt. |
| [duplication-reviews/22-08-2026_spec-system-workflow-cycle-merge/](../../duplication-reviews/22-08-2026_spec-system-workflow-cycle-merge/) | New, produced by this merge | Duplication-review workspace (`pair-ledger.md`, `duplicate-passages.md`, `duplication-report.md`) this consolidation draws from. |
| `sources/21-08-2026_spec-ticket-cycle/`, `sources/20-08-2026_specification-architecture-guidelines/` | Relocated, unedited | Full original per-dossier history; every consolidated statement below traces back to one of these two folders. |

No ticket was created by this merge pass itself, and no spec was created or edited. The originally-created ticket `5b50329b` was intentionally deleted by the requester (see the gating-edge row above) before this pass began — that deletion is not this merge's action, but is now reflected accurately in `ROADMAP.md` rather than treated as an open blocker.

## Prerequisite closure for ticket 30bbbace-7ce3-4f7e-acb4-202d08cf91d7

The table below is the exact dependency closure computed from the `.workflow-tools` ticket store for `30bbbace-7ce3-4f7e-acb4-202d08cf91d7` (W6.1). "Wave" is the minimal execution wave where wave 0 = prerequisite-free tickets and each dependent ticket's wave = 1 + max(wave of its prerequisites). "Governing spec" marks whether the ticket references a spec id in its `linked` field pointing at a `.workflow-tools/spec/specs` manifest. "Validation evidence" indicates whether the ticket carries explicit validation artifacts (validation parts, `validation_status`/`validation_summary`, or `validation_execution_id`).

| id | title (short) | state | type | wave | governing_spec? | validation evidence |
|---|---:|---|---|---:|---|---|
| 1c56033e-5c30-46bd-a0bd-2209b8841876 | Publish canonical profiling/tracing phase taxonomy | done | tracker-improvement | 0 | no | parts(validation) present |
| 529844ac-f7e5-4265-b087-5bd2b597155f | Define cross-store correlation-id contract | done | tracker-improvement | 0 | no | parts(validation) present |
| 72b3545c-ceb9-4cb2-a8d4-c146fc9b460a | Define profiling metadata retention/redaction policy | done | tracker-improvement | 0 | no | parts(validation) present |
| 8b1eab26-389b-4125-86ec-886c9d48702b | Deterministic replay vs profiling boundary | done | tracker-improvement | 0 | no | parts(validation) present |
| 84673399-75e6-4f36-8a17-4c666001e530 | Resolve logging/journaling/replay architecture boundaries | done | tracker-improvement | 1 | no | parts/objective present |
| 6c859ac3-14c9-4d9d-b428-5b0cca03e23a | Define generic operation journal schema and store contract | done | tracker-improvement | 2 | no | parts/objective present |
| 15ce7eab-4048-48f2-9296-4c427f23455d | Emit tracing spans for journaled move execution | done | tracker-improvement | 0 | no | validation_status = passed |
| 1dffcf23-8a95-4f45-8163-27e4e58048c7 | Define replayable graph-operation journal format | done | tracker-improvement | 3 | no | parts/objective present |
| 2e41c96d-fe9f-4cf2-b941-6f0d452f237c | Create domain instrumentation and journaling coverage map | done | tracker-improvement | 3 | no | parts/objective present |
| 756fed27-96b3-4572-a986-a4f70986984a | Extract shared tracing initialization for transports | done | tracker-improvement | 2 | no | validation_status = passed |
| 8ad77570-6dbe-4647-9d58-bf324a82b4fc | Promote subprocess failure-bundle log_session_ids linkage | done | tracker-improvement | 5 | no | validation_execution_id = exec-8ad77570-log-session-linkage-20260705 |
| 3041d7e3-2b34-4597-b354-e0aa6ffb0459 | Correlate CLI/MCP/HTTP spans with log sessions/journals | done | tracker-improvement | 3 | no | validation_status = passed |
| cc78d33d-1744-4945-bb77-f0fd1142568e | Subprocess failure bundle capture for transport cells | done | tracker-improvement | 4 | no | parts(validation) present |
| d3349747-b2f2-4dd4-b73c-dc016fec80d6 | Add runtime log session model and cross-store links | done | tracker-improvement | 2 | no | parts/validation present |
| d760a9bb-b970-4a97-8e38-fb4d78a5ea10 | Capture structured tracing for perf harnesses | done | tracker-improvement | 2 | no | validation_status = passed |
| dda04f91-3e03-46bb-a553-d9a172139027 | Add reconcile lifecycle tracing (ticket-api watcher) | done | tracker-improvement | 0 | no | validation_status = passed |
| e813f958-f0ee-42c3-a732-e26516eef311 | Add structured tracing for SpecStore runtime ops | done | tracker-improvement | 2 | no | validation_status = passed |
| efdcbb83-b198-4444-ba92-df39079d3004 | Add structured tracing for RuleStore runtime ops | done | tracker-improvement | 2 | no | validation_status = passed |
| ff6637f5-01f6-46c3-b727-e1a19ee0f202 | Capture profiling timings through logs and journals | done | tracker-improvement | 2 | no | parts/validation present |
| 363e26d6-0b4d-469e-a53b-5c3424262085 | Correlate Playwright runs with backend tracing sessions | done | tracker-improvement | 0 | no | validation_status = passed |

# Frontier (open) nodes directly blocking 73b2cd22
| 35cd05c1-45f7-4d65-b943-7c000570928f | Adapt move kernel journals to generic operation-journal envelope | open | tracker-improvement | 3 | no | no recorded validation yet |
| aa94d02e-9620-4db6-9974-36699cd56537 | Add live indexing and search for active logs and journals | open | tracker-improvement | 4 | no | no recorded validation yet |
| bce26d30-0a79-40b4-812a-c14b4a246de5 | Validate unified logging and journaling architecture end-to-end | open | tracker-improvement | 5 | no | no recorded validation yet (declares validation in parts) |

| 73b2cd22-942b-4205-86e5-333df2373211 | [memory-api] Shared tracing and log-api runtime diagnostics (aggregator) | open | tracker-improvement | 6 | no | linked = [61f78a57...] (ticket plan); some validation executions exist on dependent nodes |
| 30bbbace-7ce3-4f7e-acb4-202d08cf91d7 | [spec-system][W6.1] Implement v2 model, storage, and explicit migration | open | tracker-improvement | 7 | no | roadmap references `cargo test -p spec-api` and `spec.exe health` for later validation; ticket has `parts` notes but no recorded validation yet |

### Current facts (concise)
- Exact dependency closure (transitive) for `30bbbace` is the set listed above; every `depends_on` relation was read from `.workflow-tools/ticket/tickets/*/ticket.toml` and reflects the live store as of this dossier.
- Of the 19 direct prerequisites of `73b2cd22`, 16 are `done` (instrumentation, tracing extraction, rule/spec store tracing, benchmarks, transport correlation, and subprocess log-session linkage). The only open prerequisites that remain are `35cd05c1`, `aa94d02e`, and `bce26d30` (bce26d30 transitively depends on the first two).
- Governing spec linkage: none of the prerequisite tickets in this closure include a `linked` reference to a `.workflow-tools/spec/specs/*` id that would constitute a governing spec. `73b2cd22` links to `61f78a57...` which is a plan ticket (not a spec manifest). The dossier's roadmap still treats `f1b8f01a` and `fa6e85c8` as draft specs but they are not recorded as gating `linked` targets on these runtime-tracing tickets.
- Validation evidence: all `done` prerequisites that implemented tracing/instrumentation include validation artifacts (either `validation_status = "passed"`, `validation_summary`, or `validation_execution_id` / `parts` with `kind = "validation"`). The open frontier nodes do not yet show passed validation executions recorded in their ticket manifests.

### Minimal actionable frontier (ready-to-run tickets)
- `35cd05c1-45f7-4d65-b943-7c000570928f` — open but its single prerequisite (`6c859ac3`) is `done`; actionable now.
- `aa94d02e-9620-4db6-9974-36699cd56537` — open; all its declared prerequisites are `done`; actionable now.

### Short recommended next step (no mutation performed by this dossier)
- Run the declared validation executions for `35cd05c1` and `aa94d02e` (record execution via test-api) and, on success, mark them `done` so `bce26d30` and then `73b2cd22` can progress; once `73b2cd22` is `done`, W6.1 (`30bbbace`) becomes dispatchable per ROADMAP.md.

## Stage 4 prerequisite work packages

| Artifact | State | Relevance |
|---|---|---|
| [06-prerequisite-wave-1.md](06-prerequisite-wave-1.md) | New Stage 4 planning artifact | Defines parallel completion of actionable tickets `35cd05c1` and `aa94d02e`, with ticket-local validation discovery before execution. |
| [07-prerequisite-wave-2.md](07-prerequisite-wave-2.md) | New Stage 4 planning artifact | Defines completion of `bce26d30` after both wave-1 tickets are done. |
| [08-prerequisite-wave-3.md](08-prerequisite-wave-3.md) | New Stage 4 planning artifact | Defines completion of `73b2cd22` after `bce26d30` is done. |
| [09-prerequisite-wave-4.md](09-prerequisite-wave-4.md) | New Stage 4 planning artifact | Defines W6.1 `30bbbace` completion before the existing downstream migration route. |

