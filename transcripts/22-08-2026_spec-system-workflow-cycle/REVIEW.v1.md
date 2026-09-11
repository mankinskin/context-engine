# Stage 5 Second Informed Review — Spec-System Workflow-Cycle Dossier

## Verdict

**Approved as scoped.**

The Stage 4 route shape (bounded work packages, explicit `approve`/`replan`
gate, migration-tooling-first sequencing) is sound and the ticket/spec-store
facts it asserts about `.workflow-tools` are verified live and accurate. The
mechanical link and wording findings from the initial second pass were fixed:
current links resolve to `.workflow-tools`, and future migration commands are
clearly marked unavailable until W6.1 implements them. No open question
remains.

## Resolution

Findings 1-5 from the initial review pass were corrected in the current
`ROADMAP.md`, `ARTIFACTS.md`, `01-migration-tooling.md`, and
`03-spec-system-dogfood-migration.md`. Finding 6 required no action. The
current dossier is ready for the explicit final `approve` or `replan` decision;
approval would authorize later roadmap execution, not imply that W6.1's
dependency has been bypassed.

## Findings

| # | Severity | Finding | Evidence | Required change |
|---|---|---|---|---|
| 1 | High | `ROADMAP.md`'s "Review And Approval Status" section links both governing specs via `../../.spec/specs/<id>/body.md` (2 levels up, `.spec` prefix) — a path that does not resolve — while the same file's "Verified Planning Facts" section, higher up, links the identical two specs via `../../../.workflow-tools/spec/specs/<id>/spec.toml` (3 levels up, `.workflow-tools` prefix), which does resolve. This is an internal contradiction within one file, not just an isolated stale link. | `mcp_fs-mcp_fs_stat` against `context-engine/.spec/specs/f1b8f01a-c7da-4a71-97c5-39519a7d7f38/spec.toml` returns "could not be canonicalized" (not found); the equivalent `.workflow-tools/spec/specs/f1b8f01a-c7da-4a71-97c5-39519a7d7f38/spec.toml` exists (`{"exists":true,...}`). | Repoint the "Review And Approval Status" section's two spec links to the same `../../../.workflow-tools/spec/specs/<id>/spec.toml` (or `body.md` if that file exists there) form already used in "Verified Planning Facts", so the file no longer contradicts itself. |
| 2 | High | `ROADMAP.md`'s "Relevant Artifact IDs" section and `ARTIFACTS.md`'s artifact table both link `2ccde9ee` (Presentation System spec) and `0ee95228` (presentation epic) via the old 2-level `../../.spec/specs/...` and `../../.ticket/tickets/...` forms. Both do not resolve; both entities actually live under `.workflow-tools`. | `mcp_fs-mcp_fs_stat` confirms `context-engine/.spec/specs/2ccde9ee-85ac-4c87-9601-f6099f5be01c/spec.toml` and `context-engine/.ticket/tickets/0ee95228-475d-4706-a108-fd208f7c4098/ticket.toml` are both not found; `.workflow-tools/spec/specs/2ccde9ee-.../spec.toml` and `.workflow-tools/ticket/tickets/0ee95228-.../ticket.toml` both exist, and `spec.exe get 2ccde9ee --workspace .workflow-tools` resolves with `state: reviewed`. | Repoint both links in `ROADMAP.md` and `ARTIFACTS.md` to the `.workflow-tools` store using the 3-level-up relative form already established for `f1b8f01a`/`fa6e85c8` in "Verified Planning Facts". |
| 3 | High | `ARTIFACTS.md` links seven guidance-corpus artifacts — `AGENTS.md`, `.agents/instructions/ticket/workflow.instructions.md`, `.agents/instructions/ticket/lifecycle.instructions.md`, `.agents/instructions/orchestration/phase-separation.instructions.md`, `.agents/instructions/orchestration/escalation-gate.instructions.md`, `.agents/prompts/spec.prompt.md`, `.agents/agents/spec.agent.md` — at `context-engine`-relative paths. None resolve: the live guidance corpus has moved to `workflow-tools/` (e.g. `workflow-tools/AGENTS.md`, `workflow-tools/.agents/instructions/workflow/phase-separation.instructions.md`, `workflow-tools/ticket/.agents/instructions/ticket/workflow.instructions.md`, `workflow-tools/spec/.agents/prompts/spec.prompt.md`). This is the same class of drift as findings 1-2 but affects the *instruction* corpus rather than the entity store, and was not caught by the store-path fix already applied elsewhere in the file. | `mcp_fs-mcp_fs_stat` on all seven `context-engine`-relative candidate paths returns "not found"; `file_search` for each filename resolves each one uniquely under `workflow-tools/` (or a `workflow-tools/<domain>/.agents/...` submodule path). No `context-engine/AGENTS.md` or `context-engine/.agents/` tree exists at all. | Repoint every guidance-corpus link in `ARTIFACTS.md` (and the bare inline reference to `.agents/instructions/ticket/lifecycle.instructions.md` in `ROADMAP.md`'s Validation Gates section) to its actual `workflow-tools/`-rooted path. |
| 4 | Medium | `ROADMAP.md`'s "Resolved Decisions" asserts the spec-before-ticket correction to `AGENTS.md`'s Task Routing text is "already fixed there as of this iteration" — a load-bearing claim for Waypoint 11's dependency — but no reviewed file in this dossier names the correct, resolvable `AGENTS.md` path (see finding 3), so the claim cannot be checked against live content by following the dossier's own links. | Same evidence as finding 3: the only `AGENTS.md` link in the reviewed artifact set does not resolve, so the "already fixed" claim is currently unverifiable from the dossier alone. | Once finding 3 repoints the link to `workflow-tools/AGENTS.md`, confirm the cited Task Routing correction is actually present there; if verified, no further roadmap text change is needed beyond the link fix. |
| 5 | Medium | Work Packages 1, 3, and 5 (`01-migration-tooling.md`, `03-spec-system-dogfood-migration.md`, `05-final-validation.md`) each list `./target/debug/spec.exe migrate --dry-run --all` in their "Executable Validation" block with no caveat, implying it is runnable today. `ROADMAP.md`'s own top-level "Validation Gates" section states this exact command "must be added ... once the reviewed mapping and journaled runner exist. Until then, do not treat migration as implemented" — the three work packages omit that caveat. | `spec.exe migrate --help` returns `error: unrecognized subcommand 'migrate'` against the live binary. The command genuinely does not exist yet, confirming `ROADMAP.md`'s own caveat and the work packages' omission of it. | Add the same "not yet implemented; pending W6.1" caveat (or an equivalent forward reference to `ROADMAP.md`'s Validation Gates section) to the Executable Validation block in `01-migration-tooling.md`, `03-spec-system-dogfood-migration.md`, and `05-final-validation.md`, so a reader following only the work package does not treat the command as currently runnable. |
| 6 | Low (confirmed, no action) | All six W6.x tickets (`30bbbace`, `73b2cd22`, `c61d8b27`, `17aa8220`, `b3626b87`, `50dc45df`), their titles, `state: open`, and declared `depends_on` chains match `ROADMAP.md`/`02-w6-closure-readiness.md` exactly. Both governing specs (`f1b8f01a`, `fa6e85c8`) are confirmed `state: draft`. Ticket `5b50329b` is confirmed absent from every discoverable ticket store. `context-engine/.presentation/deck.toml` is confirmed `id = "context-engine"`, `composes = ["workflow-tools"]`, matching Waypoint 10's description. | Direct `ticket.exe get`/`spec.exe get` reads against `.workflow-tools`, and a direct read of `context-engine/.presentation/deck.toml`. | None — these load-bearing facts are accurate as stated; no reconciliation needed. |

## Superseded-Content Check

The "Superseded Stage 3 Detail" block in `ROADMAP.md` is clearly labeled
non-authoritative ("The Stage 4 sections above are authoritative... Do not
use the retained detail to select or execute work") and its own internal
claims (Duplication-Review Consolidation Verdict, Resolved Decisions, Active
Blockers, Validation Gates, Roadmap Waypoints, Settled Contract Scope, Heads-up
Notes) are consistent with the Stage 4 summary above it — no contradiction
between the current and retained sections was found beyond findings 1-2,
which sit inside the retained block itself and are corrected in place. This
retained content may remain per the review's own historical-content allowance.

## Dependency and Order Check

Waypoint and Work Package sequencing is internally consistent: Work Package 1
(migration tooling / Waypoint 6-7 slice for W6.1) is blocked on `73b2cd22`;
Work Package 2 (W6 closure / Waypoints 6-7 for W6.2-W6.5) follows the declared
`depends_on` chain (`c61d8b27`, `17aa8220`, `b3626b87` depend on `30bbbace`;
`50dc45df` depends on all four); Work Package 3 (dogfood migration / Waypoint
8) correctly depends on Work Packages 1-2; Work Package 4 (docs/guidance /
Waypoints 9 and 11) correctly depends on Work Packages 1-3; Work Package 5
(final validation / Waypoint 13) correctly depends on Work Packages 1-4.
Waypoint 12 (ticket-depends-on-spec gating edge) is correctly independent of
Waypoint 13's gate. No dependency-order error was found.

## Final Approve/Replan Gate Check

`ROADMAP.md`'s "Boundary And Gate" section states the explicit choice
(`approve` / `replan`) in unambiguous terms, states that W6.1 remains
unavailable until `73b2cd22` reaches its required terminal state, and states
that no inferred approval or passing validation substitutes for the explicit
decision. This gate is clear and requires no change.

## Open Question Carry-Forward

None. `INTERVIEW-1.md` and `INTERVIEW-2.md` already record explicit resolved
decisions for the two open questions the Stage 3 review raised (W6.1's
dependency on `73b2cd22`, and the two specs' persisted `draft` state); both
resolutions are reflected correctly in the current `ROADMAP.md`. No new open
question was discovered by this pass — every finding above is a link or
wording correction with a concrete required change, not a decision the
requester must make.
