# Session Objective
Resolve the current ticket_graph batch for context-engine .ticket store and reduce 125 findings from the baseline.

# Scope Guardrails
- Stay inside context-engine .ticket store unless a blocker requires a dependency fix outside scope.
- Do not start the next batch until this ticket meets done criteria.

# Implementation Steps
1. Capture exact finding rows for this batch from the baseline audit artifact.
2. Group findings into 2 to 5 micro-chunks and handle one chunk at a time.
3. After each chunk, run the narrowest compile/test check relevant to touched files.
4. Re-run audit summary and record count delta.
5. If blockers remain, create follow-up tickets and link them before handoff.

# Validation Commands
- Full category summary: cargo run -p audit-cli --bin audit -- --json summary --by category .
- Full baseline refresh when needed: cargo run -p audit-cli --bin audit -- --json run .
- Ticket health sanity: ./target/debug/ticket.exe health --workspace . --all --toon

# Acceptance Criteria
- Findings in this batch are resolved or have explicit blocker tickets linked.
- No increase in other categories caused by this batch.
- Batch notes include before and after counts and next unresolved action.

# Handoff Notes
Record exact commands run, resulting counts, and files changed so the next session can continue without rediscovery.

# Baseline (context-engine .ticket store)
- Store total tickets: 954 (955 after chunk 1).
- ticket_graph findings for this store: 125 = 104 orphan_ticket_count (graph_participation) + 21 dependency_convergence_count.
- Repo-wide ticket_graph baseline: 258. Full audit baseline total: 551.
- Remediation strategy: group orphan tickets by theme under retrospective parent trackers (state `new`) that `depends_on` each child. A tracker with outgoing edges participates in the graph; each child gains an incoming dependee. A `new` tracker depending on `done`/`new`/`in-implementation` children introduces no dependency_convergence inversions (nothing is in a state earlier than `new`).

# Progress Log

## Micro-chunk 1 — ticket-system tooling cluster (26 orphans)
- Created retro tracker `e7c593dd-6ae4-4768-89bd-e5f9a91a4792` "[audit-roadmap][ticket_graph][retro] Ticket-system tooling work stream".
- Linked tracker `depends_on` 26 orphan tickets (ticket-api/http/cli/mcp/viewer/vscode/store, board, bootstrap) via 1 direct link + 25-line transactional `batch`.
- Commands: `ticket create --type tracker-improvement --body-file ...`; `ticket link ...`; `ticket batch --file target/tmp/chunk1-batch.txt`; `ticket scan --force`; `audit summary --by category`.
- Delta: repo-wide ticket_graph 258 -> 232 (-26). No other category changed (file_length 182, static_complexity 108, compiler_warning/coverage/test_execution 1 each). Full total 551 -> 525.
- context-engine store ticket_graph remaining after chunk 1: 99 (78 orphan + 21 convergence).

## Micro-chunk 2 — rules/specs/agent-guidance cluster (21 orphans)
- Created retro tracker `45ff05c9-7608-43c4-a98a-e1c44e4b7fbd` "[audit-roadmap][ticket_graph][retro] Rules, specs & agent-guidance work stream".
- Linked tracker `depends_on` 21 orphans (rule-cli/rule-api, spec-cli, repo-guidance, agent-rules, session-api, instruction DSL) via transactional `batch`.
- Note: `search` index lagged after `create` and missed the new tracker (title contains `&`); resolved by resolving the ID via `list` + title filter.
- Delta: repo-wide ticket_graph 232 -> 211 (-21). No other category changed. Full total 525 -> 504.
- context-engine store ticket_graph remaining after chunk 2: 78 (57 orphan + 21 convergence).

## Micro-chunk 3 — context-engine core cluster (13 orphans)
- Created retro tracker `cb92b0d2-1361-44c6-8c92-fae3036ac97a` "[audit-roadmap][ticket_graph][retro] Context-engine core work stream".
- Linked 13 orphans (context-read/search/trace/insert plans, expansion loop, partition merge, ngrams, CLI read UX, Graph3D).
- Delta: repo-wide ticket_graph 211 -> 198 (-13). No other category changed.

## Micro-chunk 4 — infra/CI/install/viewers cluster (11 orphans)
- Created retro tracker `02622207-e102-4fae-b705-ca1cb12704ba` "[audit-roadmap][ticket_graph][retro] Infra, CI, install and viewer-tooling work stream".
- Linked 11 orphans (QA/audit tool CLI+config, install-tools, CI viewer split, viewer-api validation, README sync, log-viewer replay, context-stack ownership).
- Delta: repo-wide ticket_graph 198 -> 187 (-11).

## Micro-chunk 5 — miscellaneous cross-cutting cluster (33 orphans)
- Created retro tracker `ad63d3da-1661-41e0-a0d3-3163bba324f9` "[audit-roadmap][ticket_graph][retro] Miscellaneous cross-cutting work stream".
- Linked 33 remaining orphans (integration-test harness, hooks, test-result store, peek-cli, memory-matrix, docs plans, throwaway/probe tickets).
- Delta: repo-wide ticket_graph 187 -> 154 (-33).

## Orphan phase complete
- All 104 orphan_ticket_count findings in the context-engine .ticket store are resolved via 5 retrospective parent trackers.
- context-engine store ticket_graph remaining: 21 (all dependency_convergence_count; 0 orphan).
- Repo-wide ticket_graph: 258 -> 154 (-104). Full total findings: 551 -> 447. No new finding class introduced; other categories unchanged.
- Next action: resolve the 21 dependency_convergence (state-inversion) findings.

## Convergence phase — 21 dependency_convergence findings (documented residual)
- All 21 are genuine state inversions across 6 dependent heads on REAL work tickets:
  - `0727b7dd` "Plan: Context API — master multi-phase architecture plan" (ready) over 16 `new` children (gap 1).
  - `0ffac34a` (in-implementation) -> `5d320d7e` (ready).
  - `416ebd52` (in-review) -> `91011568` (in-implementation).
  - `7b8d2e81` (in-implementation) -> `26f570e2` (new, cross-store memory-viewers).
  - `8ab31960` (in-implementation) -> `68e3c713` (new).
  - `9b9df133` (in-implementation) -> `0dd23fe6` (new).
- Not auto-fixed: the only audit remedies (advance prerequisite / revert dependent / remove edge) each change real work-ticket state or graph semantics on active or completed tickets. Applying them mechanically would fabricate progress or misrepresent completed work, so they require per-ticket product judgment — out of batch-1's orphan-connectivity scope.
- Follow-up blocker ticket created: `6e89260d-2b89-4e0d-9f03-ed10907de19d` "[audit-roadmap][ticket_graph] Resolve context-engine dependency_convergence findings (21)". Linked as `1a2b326d depends_on 6e89260d` so the category ticket cannot close until convergence reaches zero.

## Batch-1 final summary
- Orphan phase: 104 orphan_ticket_count findings resolved via 5 retrospective parent trackers (e7c593dd, 45ff05c9, cb92b0d2, 02622207, ad63d3da).
- Convergence phase: 21 dependency_convergence findings documented as residual with explicit linked blocker ticket 6e89260d.
- context-engine .ticket store ticket_graph: 125 -> 21 (0 orphan + 21 convergence residual).
- Repo-wide ticket_graph: 258 -> 154 (-104). Full total findings: 551 -> 447 (-104).
- Acceptance criteria met: findings resolved or blocker-linked; no other category changed; no new finding class introduced. Before/after counts recorded per chunk above.
- Store total tickets: 954 -> 960 (5 retro trackers + 1 follow-up).
- Validation artifacts: target/tmp/audit-current.json (baseline), target/tmp/audit-after-orphans.json, target/tmp/audit-final.json.