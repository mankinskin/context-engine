# Routing Research Inventory

## Evidence Status

| Claim | Evidence | Conclusion |
| --- | --- | --- |
| `ticket-extraction-finish` misroute | No UUID, hook payload, resolver result, proxy log, or before/after store paths supplied. | Unreproduced; not a confirmed regression of [0afe45b5 [ticket-api][session-api] Store resolution enumerates .worktrees/* and mis-anchors the active store](.ticket/tickets/0afe45b5-9ec8-4f4a-af74-f46f06cc7516/ticket.toml). |
| D1/D2 versus present contract | Source/guidance disagree on eager trigger; both use positional discovery. | Architectural replacement, not local repair. |

## Routing Today

| Surface | Verified behavior | Evidence |
| --- | --- | --- |
| Inputs | `ResolveRequest` takes UUID session id, optional relative path, and store directory; workspace is not a worktree selector. | [resolver](memory-api/crates/session-workspace-resolver/src/lib.rs#L111-L127) |
| Anchor | Resolver uses MCP process cwd; `mcp-toolmon` permits `MCP_MAIN_CHECKOUT` override. | [resolver](memory-api/crates/session-workspace-resolver/src/lib.rs#L129-L154), [proxy](memory-api/tools/mcp/mcp-toolmon/src/proxy.rs#L400-L421) |
| Discovery | Exactly one Git checkout at `.worktrees/<uuid>/` wins. Only zero nested candidates permits one legacy `<short-id>-*` path with a local record. | [resolver](memory-api/crates/session-workspace-resolver/src/lib.rs#L249-L332) |
| Guard | Canonical root must be repository-contained Git checkout; relative path stays inside it; main is rejected by `require_mutation_target()`. | [resolver](memory-api/crates/session-workspace-resolver/src/lib.rs#L64-L106), [resolver](memory-api/crates/session-workspace-resolver/src/lib.rs#L190-L247) |
| Default failure | At repository anchor, no default-workspace discovery becomes `MainCheckoutMutationBlocked`; absolute workspace must be under resolved worktree. | [proxy](memory-api/tools/mcp/mcp-toolmon/src/proxy.rs#L423-L511) |

A worktree caller's cwd is not resolver input. `mcp-toolmon` routes from its own anchor and positional candidates. Current source rejects a missing/ambiguous candidate instead of forwarding mutation to main; a confirmed main write therefore needs evidence of a bypass, another version/configuration, or a recorded conflicting resolution.

## “MCP Proxy” Mapping

| Reporter term | Component | Role |
| --- | --- | --- |
| MCP proxy | [mcp-toolmon proxy](memory-api/tools/mcp/mcp-toolmon/src/proxy.rs#L1-L40) | JSON-RPC interceptor; validates model/session, rewrites workspace paths, forwards allowed calls. |
| Routing owner | [session-workspace-resolver](memory-api/crates/session-workspace-resolver/src/lib.rs#L158-L332) | Positional discovery and target classification. |
| Downstream tools | `audit-mcp`, `compact-terminal-mcp`, `feedback-mcp`, `fs-mcp`, `peek-mcp`, `rule-mcp`, `session-mcp`, `spec-mcp`, `test-mcp`. | [memory-api/tools/mcp](memory-api/tools/mcp/) |

## Capture Today

| Step | Behavior | Evidence |
| --- | --- | --- |
| Order | `run()` initializes routing before store selection. Source provisions only at `SessionStart`; guidance states `UserPromptSubmit`. | [capture hook](memory-api/crates/session-capture-hook/src/main.rs#L52-L151), [capture hook](memory-api/crates/session-capture-hook/src/main.rs#L240-L338), [guidance](.agents/instructions/session/worktree-provisioning.instructions.md#L11-L18) |
| Store | Explicit root wins; otherwise `MCP_MAIN_CHECKOUT`/cwd anchor, anchor `.session`, resolver, and `mutation_store_root(".session")` are required; failure skips capture. | [capture hook](memory-api/crates/session-capture-hook/src/main.rs#L701-L783) |
| Write | Capture persists transcript/event, then best-effort infers worktree data and refreshes metrics. | [capture hook](memory-api/crates/session-capture-hook/src/main.rs#L80-L151), [capture hook](memory-api/crates/session-capture-hook/src/main.rs#L815-L834) |

## Provisioning Today

| Item | Behavior | Evidence |
| --- | --- | --- |
| Eager control | `WORKTREE_EAGER_PROVISION` is enabled unless exactly `0`. | [capture hook](memory-api/crates/session-capture-hook/src/main.rs#L333-L338) |
| Ordering | Nested reuse, legacy reuse, reclaim, then create. | [policy](memory-api/crates/session-worktree-provision/src/policy.rs#L349-L429) |
| Controls | `WORKTREE_EAGER_PROVISION`, `WORKTREE_MAX`, `WORKTREE_IDLE_SECS`, `WORKTREE_STALE_SECS`, `MCP_MAIN_CHECKOUT`. | [guidance](.agents/instructions/session/worktree-provisioning.instructions.md#L245-L253) |

## Session Record and Git Tracking

| Fact | Evidence |
| --- | --- |
| Shape | `SessionRecord` holds history/metadata; `metadata.worktree` contains path, branch, allocation mode, status, predecessors. | [session model](memory-api/crates/session-api/src/model.rs#L298-L383) |
| Real record | Persisted sample contains path, branch, allocation mode, status. | [sample record](.session/sessions/044bb46a-1b50-44da-8351-f5953828afac/session.json#L1-L25) |
| Assignment write | `check_in_worktree()` writes path-bearing active assignment; it is not a transfer transaction. | [session runtime](memory-api/crates/session-api/src/store/config/worktree_runtime.rs#L1-L151) |
| Git | `.session/sessions/**` is unignored/tracked; `.session/local/**` is ignored, confirmed with `git check-ignore`. | [.gitignore](.gitignore#L30-L47) |

## Deregistration Today

| Command | Existing behavior | Evidence |
| --- | --- | --- |
| `remove` | Removes/prunes Git worktrees, not session bindings. | [worktree-ctl](tools/worktree/worktree-ctl/src/main.rs#L360-L423) |
| `finish` | Rebase/remove/prune only; no no-diff proof or binding clear. | [worktree-ctl](tools/worktree/worktree-ctl/src/main.rs#L453-L481) |
| `rename` / `doctor` | Rename Git state; repair submodule Git configuration/prune. Neither clears binding. | [worktree-ctl](tools/worktree/worktree-ctl/src/main.rs#L425-L451), [worktree-ctl](tools/worktree/worktree-ctl/src/main.rs#L680-L746) |

## Reproduction Status

The incident is unreproduced. A minimal fixture uses a fresh UUID, records proxy cwd and `MCP_MAIN_CHECKOUT`, creates a named linked worktree, records main/worktree session records, invokes one mutating MCP tool with default and absolute selectors, and asserts proxy rewritten roots plus the sole modified store. Preserve hook stderr and `SessionStart`/`UserPromptSubmit` payloads. Do not run the fixture in this dossier.
