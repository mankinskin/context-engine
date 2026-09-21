# Resolve context-engine dependency_convergence (state-inversion) findings

## Problem
After audit-roadmap batch-1 (ca788fe3) cleared all 104 orphan_ticket_count findings in the context-engine `.ticket` store, 21 `dependency_convergence_count` findings remain. Each flags a dependent ticket sitting in a *later* workflow state than one of its `depends_on` prerequisites (state inversion).

## Residual findings (6 dependent heads, 21 edges)
1. `0727b7dd` "Plan: Context API — master multi-phase architecture plan" (**ready**) depends_on 16 child plans/designs still in **new** (gap 1). Parent plan advanced to `ready` on 2026-04-14 and has been stable since; children never started. Resolving requires deciding whether the master plan should be reverted to `new`/`in-refinement` to sit with its children, or whether a groomed parent ahead of unstarted children is an accepted exception.
2. `0ffac34a` "Implement Docker harness..." (**in-implementation**) depends_on `5d320d7e` "Design reproducible Docker validation..." (**ready**) gap 1.
3. `416ebd52` "[ticket-http] Return only authoritative resolved hits" (**in-review**) depends_on `91011568` "[ticket-cli][ticket-mcp] Expose authoritative..." (**in-implementation**) gap 1.
4. `7b8d2e81` "[readmes][generated-repos] Adopt shared README schema" (**in-implementation**) depends_on `26f570e2` "[memory-viewers] Adopt shared README schema" (**new**, cross-store: memory-viewers) gap 2.
5. `8ab31960` "[memory-api][ticket-api]..." (**in-implementation**) depends_on `68e3c713` "[ticket-cli] Fix next --filter matching" (**new**) gap 2.
6. `9b9df133` "[token-efficiency] Track token-efficient agent tooling" (**in-implementation**) depends_on `0dd23fe6` "[token-efficiency] Audit execute MCP tools" (**new**) gap 2.

## Why not auto-fixed in batch-1
The audit fix options are: advance the prerequisite, revert the dependent, or correct/remove the edge. All three change real work-ticket state or graph semantics on active/completed tickets. Applying them mechanically would either fabricate progress on unstarted prerequisites or misrepresent completed work, so they need per-ticket product judgment rather than a bulk graph edit. Batch-1 was scoped to orphan connectivity only.

## Acceptance criteria
- For each of the 6 dependent heads, either advance the real prerequisite, revert the dependent's state to remove the inversion, or remove/replace the stale dependency edge — with a recorded reason.
- context-engine `.ticket` store `dependency_convergence_count` reaches 0 (verify: `audit run` filtered to the context-engine store).
- No orphan regression introduced (orphan_ticket_count stays 0 for the store).

## Validation
- `cargo run -p audit-cli --bin audit -- --json run .` then filter ticket_graph findings whose path is the context-engine `.ticket` store.
- `./target/debug/ticket.exe health --workspace . --all --toon`
