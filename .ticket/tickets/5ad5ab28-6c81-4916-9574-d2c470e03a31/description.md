Improve ticket health audit validation so the repository gets more signal from ticket quality checks, especially for newly created tickets.

## Context

Research on 2026-06-07 found that `ticket_api::health::collect_findings` is already the canonical health implementation for ticket CLI, HTTP, and MCP surfaces. Current checks cover missing title, missing/short description, dependency-state convergence, unresolved dependencies on non-new tickets, and dangling `depends_on` edges.

`audit-api` still has a separate `ticket_graph` trial that reimplements orphan ticket and dependency-convergence checks instead of consuming the canonical ticket health report. Prior completed work includes `[audit-api] Require every ticket to participate in dependency graph` (a762448e) and `[audit-api] Include dependency convergence findings in default repo audit` (95d4f986).

Current all-store baseline from `mcp_rmcp4_health_check(all=true)` checked 316 non-terminal tickets and produced 153 findings: 66 `dependency_convergence`, 61 `unblocked_with_deps`, 25 `missing_description`, and 1 `short_description`.

### Code-level facts confirmed during review (2026-06-07)

- `collect_findings` does **not** currently emit any orphan / graph-participation finding. That check exists **only** in `audit-api`'s `ticket_graph` trial (`evaluate`, orphan_findings). A naive "consume canonical health first" refactor would therefore silently **regress** the orphan check that a762448e shipped.
- `audit-api` already imports `dependency_state_inversions` from `ticket-api`, so **convergence parity is effectively done** — only orphan parity is real work for the first slice.
- `HealthFinding` currently carries none of the fields the audit wrapper needs: no ticket `path`, no `state`, no `type`, no evidence bag, no remediation `instructions`. Audit findings carry all of these. The wrapper cannot map health → audit without losing fidelity until `HealthFinding` is extended.
- Canonical health uses flat string severities (`error`/`warning`/`info`). Audit uses the `Severity` enum and derives High/Medium **from ticket state** (`in-implementation`/`in-review` → High, else Medium) for both orphan and convergence findings. The string→`Severity` mapping must live in the audit wrapper and must preserve the state-aware nuance, which requires `HealthFinding.state`.

## Supported Ticket Features Inventory

Health validation should consider these supported features:

- Manifest fields from the tracker-improvement schema: `title`, `type`, `state`, `component`, `risk_level`, `acceptance_criteria`, `validation_plan`, `validation_status`, `validator_id`, `release_target`, `release_version`, `effort`, `bootstrap_blocker`, `rollout_stage`, `doc_category`, `tags`, `workflow_stage`, `priority`, `source_agent_files`, `bug_validity`, and `phase`.
- Built-in/default-schema fields also currently include `interview_file_type` and `interview_files`; the follow-up should reconcile drift between the built-in schema and the checked-in TOML schema before treating field presence as authoritative.
- Ticket body file: `description.md` existence and useful length/content.
- Lifecycle/state-machine features: valid states, valid transitions, terminal-state required history (`in-review` before `done`), terminal states, and history snapshots.
- Graph features: edge kinds `depends_on` and `linked`, directed/acyclic `depends_on`, dangling edge detection, orphan/graph participation, dependency-state inversions, unresolved blockers, and dependency/reverse-dependency reach metrics.
- Workflow-ranking features: priority, effort, became-actionable timing, blocker progress, reverse-dependency pressure, and state progress.
- Board/work coordination features: active/stale board entries, owned files, WIP exclusions, and whether tickets in implementation/review have active ownership or recorded handoff context.
- Traceability features already used by tickets even though not fully enforced in the schema: `spec_refs`, implementation summary, validation summary/status, and evidence links.

## Proposed Health Contract

Phase 1 should classify checks without making the whole store fail unexpectedly:

- `error`: missing required schema fields, unknown ticket type, invalid state, invalid edge kind, dangling edge, impossible/invalid terminal workflow history, malformed typed field values, and schema drift that prevents validation.
- `warning`: missing description, missing component on non-terminal work, missing risk level for ready/in-implementation/in-review tickets, missing acceptance criteria for ready-or-later tickets, missing validation plan/status for in-review tickets, missing spec traceability for in-review tickets, orphan graph participation, dependency convergence, and stale in-implementation/in-review board ownership.
- `info`: short description, missing priority/effort/workflow_stage/tags, unresolved dependencies while the ticket remains `new`, and optional release/interview metadata gaps when relevant.

New tickets should receive stronger early guidance without forcing legacy cleanup in one step:

- `new` tickets should pass with at least a meaningful title, description, component or clear title prefix, risk level when non-trivial, acceptance criteria when implementation scope is known, and graph participation unless explicitly allowed as standalone.
- `ready` should require acceptance criteria, component, risk level, and dependency state coherence.
- `in-implementation` should require validation plan and either an active board entry or recorded handoff/owner metadata.
- `in-review` should require validation status/results, spec traceability when behavior or requirements changed, and no unresolved dependencies except documented exceptions.

## First Implementation Slice — orphan parity into canonical health (do in this order)

This supersedes the earlier framing of "make audit consume `collect_findings` first." Because canonical health emits no orphan finding yet, the ordering must be:

1. **Add a graph-participation / orphan check group to `collect_findings`** that reproduces the orphan finding currently computed only in `audit-api`'s `ticket_graph` trial. Use the same orphan definition (no outgoing `depends_on` and no incoming dependees). Decide whether `linked`-only participation counts as connected (see Open Decisions).
2. **Extend `HealthFinding`** with the evidence fields the audit wrapper needs: ticket `path`, `state`, `type`, and remediation `instructions` (plus an optional structured evidence map). Without these the wrapper loses fidelity audit has today.
3. **Replace the orphan logic in `audit-api`'s `ticket_graph` trial** with a call into canonical ticket health, mapping health findings to audit findings. Preserve the `orphan_ticket_count` metric name and stable finding IDs, and keep the state-aware `Severity` mapping (in-implementation/in-review → High, else Medium) in the wrapper, driven by `HealthFinding.state`.
4. **Convergence parity is confirmation only** — `audit-api` already imports `dependency_state_inversions`; verify the wrapper-derived convergence findings match the existing output rather than reimplementing traversal.

Validation for this slice: `cargo test -p ticket-api health`, `cargo test -p audit-api ticket_graph`, then compare `ticket health --all --toon` with `audit run .` to confirm orphan + convergence counts match across surfaces.

## Full Implementation Plan (phased; later phases become child tickets under tracker 53f471a3)

1. Add a health policy layer in `ticket-api` that keeps `collect_findings` canonical and expands it with modular check groups: manifest/schema, description/body, lifecycle/history, graph/workflow, traceability/evidence, and board/work coordination.
2. Reconcile schema sources before enforcing field-level checks. Either load the checked-in tracker-improvement TOML consistently or update the built-in schema so supported fields do not diverge.
3. Extend `HealthFinding` evidence with optional fields that audit consumers need: ticket path, state, type, field name, expected value/policy, actual value, edge kind, and remediation instructions. (Started in the first slice.)
4. Add graph participation to canonical ticket health so `audit-api` no longer needs a separate orphan implementation. Preserve the existing `ticket_graph` audit metric name initially for compatibility, but derive it from canonical health findings. (This is the first slice.)
5. Replace `audit-api` ticket validation internals with a wrapper that opens the ticket store, calls canonical ticket health for individual tickets, maps health severities to audit severities, and emits one audit finding per health finding with stable IDs like `ticket_health:<check>:<ticket_id>`.
6. Decide and document which checks count toward audit failure versus advisory signal. Start new strict checks as warnings/info, then promote selected checks after the current store has remediation tickets.
7. Add tests in `ticket-api` for each new check key and state-specific threshold. Include fixtures for new, ready, in-implementation, in-review, done, and cancelled tickets.
8. Add parity tests proving CLI, HTTP, MCP, and audit-api surfaces report the same canonical health check keys, severities, and evidence for the same fixture store.
9. Update ticket MCP/CLI README or generated rule source so agents know which fields are expected when creating new tickets.
10. Run a migration audit on the current store, record baseline counts, and create remediation tickets for high-signal cleanup buckets instead of hand-editing hundreds of historical tickets.

## Validation Plan

- `cargo test -p ticket-api health`
- `cargo test -p ticket-cli health` or the nearest focused CLI integration tests for health output
- `cargo test -p ticket-http integration_parity` for HTTP/MCP parity if touched
- `cargo test -p ticket-mcp health` or nearest MCP server health tests if available
- `cargo test -p audit-api ticket_graph` during compatibility, then rename/add focused ticket-health audit tests once the wrapper is introduced
- Run `ticket health --all --toon` and `audit run` on the repository to compare canonical health counts with audit findings

## Traceability / Graph Context

- `depends_on` predecessors: a762448e (orphan check, done), 95d4f986 (convergence findings, done).
- Rolls up under tracker **53f471a3 Project tracker: audit quality backlog** (edge added 2026-06-07).
- `linked` (non-blocking) to evidence-substrate tickets the later traceability/evidence check group will consume: `618f6ce4` (bootstrap doc-api/test-api/log-api evidence stores) and `aaa90ee6` (store-owned spec evidence integration). These inform Open Decision on `spec_refs`/validation evidence but do not block the first slice.

## Non-Goals

- Do not block ticket creation immediately on optional quality fields.
- Do not require every historical done/cancelled ticket to be remediated before the new health surface ships.
- Do not create a second audit-only health model; audit-api must consume the ticket-api health report.
- Do not start the first slice with an audit-side parity refactor before the orphan check exists in canonical health (would regress a762448e).

## Open Decisions

- Whether `spec_refs` should become a first-class schema field or remain a convention checked by health. (Evidence-substrate work in `618f6ce4`/`aaa90ee6` is relevant input.)
- Whether missing `validation_plan` is required at `ready` or only once work reaches implementation/review.
- Whether graph participation should treat `linked` as sufficient for standalone research/planning tickets, or require `depends_on` participation for all implementation tickets. (Affects the orphan definition used in the first slice.)
- Whether stale/missing board ownership belongs in default health or a separately named operational-health check.