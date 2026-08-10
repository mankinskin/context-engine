<!-- aligned-structure:v1 -->

# Summary

Make dedicated git worktrees the default workflow for new agent sessions in this repository so parallel implementation tracks do not share one staging area.

## Behavior Story

Make dedicated git worktrees the default workflow for new agent sessions in this repository so parallel implementation tracks do not share one staging area.

## Provided Surface Contracts

- Define provided contracts for this behavior slice.

## Required Validation

- Triangulate behavior with executable checks, natural-language clauses, and code/schema/API references when available.

## Related Implementation Tickets

- No related implementation ticket is linked yet.

## Background Knowledge References

- Prefer entity references and context rendering over embedding fully expanded payloads in this spec body.

## Legacy Content (Preserved)

# Goal
Make dedicated git worktrees the default workflow for new agent sessions in this repository so parallel implementation tracks do not share one staging area.

# Scope
- require new sessions to check into the session tool before implementation begins
- assign each checked-in session an authoritative worktree working directory
- expose that working directory through targeted `session-api` CLI, MCP, hook, or startup surfaces
- define how ticket-board ownership, session state, and worktree lifecycle interact
- define revival and handoff behavior for reusing or rotating session-owned worktrees
- update workflow guidance and hooks so the worktree-first path becomes the repository default

# Workflow Contract
1. **Session check-in comes first.** A new implementation session must obtain a session record before code changes, board file claims, or startup guidance treats the session as active work. The session record carries the owner identity, ticket context, and worktree assignment status.
2. **Worktree assignment is authoritative in `session-api`.** The assigned working directory returned by session check-in or resume is the source of truth for CLI startup, MCP startup, hooks, and guidance. Other tools may reference the path, but they must not derive or replace it independently.
3. **Board coordination begins after worktree assignment.** The draftboard remains authoritative for ticket activity and file ownership. `session-api` remains authoritative for session identity, worktree metadata, and revival history. Hooks may read both stores for reminders or evidence capture, but they must not silently allocate worktrees or transfer board ownership on their own.
4. **Default rollout order is fixed.** Planning defines the contract first, `session-api` implements the worktree assignment surfaces second, then repository guidance and hooks adopt the new default startup path. The tracker closes only after all three slices are linked back to this spec.
5. **Bootstrap is prompt-time, not process-relocating.** A prompt-time bootstrap hook may automate the mandatory check-in by reading the submitted prompt, calling the authoritative `session-api` check-in surface, and injecting the resolved worktree path into agent context. The hook surfaces the path; it does not allocate worktrees independently, transfer board ownership, or relocate the running agent process. True process-level placement in the assigned worktree (re-rooting the workspace or spawning the agent with that working directory) is the responsibility of a separate launcher, not the hook.

# Ownership and Lifecycle
- One active session owns exactly one active worktree assignment at a time.
- Worktree metadata must record at least the assigned path, branch, allocation mode (`new`, `reused`, or `rotated`), and predecessor reference when rotation occurs.
- Multiple sessions may target the same ticket over time, but they must not silently share one active worktree assignment.
- Session stop hooks capture evidence and transcript state, but they do not reassign worktree ownership by themselves.

# Reuse vs Rotation Contract
- **Reuse** the existing worktree when the same session, or an explicit revival of that same owner/session lineage, resumes and the recorded path and branch invariants still hold.
- **Rotate** to a new worktree when a fresh session starts after handoff, when the previous worktree is missing or fails invariants, or when another active session still owns the prior assignment.
- **Default handoff behavior is rotation.** Cross-session or cross-owner reuse requires an explicit adopt flow with validation; it is not the silent startup default.

# Non-goals
- reviving the cancelled AOH sandbox-manager implementation as-is
- changing the existing append-only `session-api` transcript persistence contract
- designing full merge automation or PR orchestration in this slice

# Acceptance Criteria
1. This spec defines the mandatory startup order: session check-in, authoritative worktree assignment, board check-in or file claims, implementation, then stop or handoff capture.
2. This spec defines the ownership boundary between `session-api`, the ticket draftboard, and repository hooks or guidance so each surface has one clear authority.
3. This spec defines reuse vs rotation rules for resume, revival, handoff, and invalid worktree recovery, including rotation as the default handoff path.
4. The `session-api` implementation slice is required to return the authoritative working directory plus allocation metadata and to validate the reuse or rotation rules with focused tests.
5. The guidance or hook slice is required to consume the assigned worktree path, explain the board interaction order, and document stop or handoff expectations without reassigning ownership implicitly.
6. The planning slice links the existing related research and completed session capture work so implementation can reuse them instead of re-deciding the model.
7. The prompt-time bootstrap slice is required to add a startup (`UserPromptSubmit`) hook that resolves session/worktree context from the prompt through the authoritative `session-api` check-in surface and injects the resolved worktree path into agent context, without synthesizing the path, allocating worktrees, or transferring board ownership inside the hook. It must document the launcher boundary for true process-level placement.

# Traceability
- Tracker: [b6af9f40 Default worktree-backed session workflow](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/b6af9f40-e1f7-4f68-92e7-0a063a4ac020/ticket.toml)
- Planning: [68a49ca7 Plan default worktree-backed session workflow](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/68a49ca7-a6f6-42a8-b820-0a86e6a4de2e/ticket.toml)
- Infrastructure: [e2189e9d Implement session check-in and worktree assignment surfaces](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/e2189e9d-8ea7-4747-bda9-51e573ba51ca/ticket.toml)
- Guidance: [326bfe38 Add worktree-first session guidance and hooks](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/326bfe38-6f5e-4000-9ffc-e5be0839194f/ticket.toml)
- Bootstrap hook: [3d535b2c Add prompt-time worktree bootstrap hook](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/3d535b2c-7361-4f08-bfb4-63b0b3174afc/ticket.toml)
- Bootstrap prerequisite — session surfaces: [f76b0fa9 Add session-cli and session-mcp for session subcommands](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/f76b0fa9-d880-45da-b039-b483e904ee2f/ticket.toml)
- Bootstrap prerequisite — store-root resolution: [cf4d1e1a Resolve session workspace relative to tool execution](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/cf4d1e1a-5315-4aa8-b836-5a90996e63c4/ticket.toml)
- Related completed work: [e663f9e9 Wire VS Code Copilot stop-hook session capture](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/e663f9e9-ac52-4c0e-8e07-d17c8a15b48d/ticket.toml)
- Related research: [09b68366 Multi-agent coordination and cross-agent communication protocols](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/09b68366-486e-4e39-a610-1d14676368aa/ticket.toml)
- Superseded implementation context: [51471c3e Sandbox Manager -- per-assignment worktree and branch isolation](C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/graph_app/context-engine/.ticket/tickets/51471c3e-a088-47d4-9922-ba49d914af17/ticket.toml)

# Session Anchoring and Managed Lifecycle (2026-08-06 refinement)

The Workflow Contract above establishes that worktree assignment is authoritative in `session-api`. Two gaps became visible when that contract met a real multi-worktree session: nothing makes tooling USE the assignment, and nothing manages the assigned worktree over time.

## Session is the only anchor

Workflow Contract item 2 says other tools "must not derive or replace" the assigned path independently. That prohibition has no enforcement point today, so every tool that resolves a workspace from its own process working directory violates it silently.

- The session id is the ONLY legitimate anchor for resolving a workspace. Process working directory is never an acceptable fallback, because a long-lived server process has a cwd unrelated to where the agent is working.
- A workspace selection that cannot be resolved through a session anchor is REJECTED, not defaulted. Silently choosing a store is the failure mode this rule exists to prevent.
- Because the resolved scope must be able to distinguish a worktree from the main checkout, it is a typed value rather than a bare path, sufficient to express a policy that BLOCKS mutating work resolved to the main checkout.

The cross-cutting protocol that enforces this at the MCP boundary is specified separately in `context-engine/mcp/session-anchored-workspace-resolution` and implemented by ticket `fa2ba34b-59ec-4321-a4ce-a3c3a9295ea3`.

## Managed lifecycle

The Ownership and Lifecycle and Reuse vs Rotation sections define the STATES an assignment can be in. This section defines the OPERATIONS that move between them.

### Preservation on creation

Creating a session worktree while the main checkout carries uncommitted changes must preserve those changes. Preservation is available as an explicit option (a recorded, restorable save); when it is not requested, the operation reports what it found and requires explicit acknowledgement. Silently stranding uncommitted work in a checkout the agent has just navigated away from is prohibited.

### Reuse by default

A session holds ONE worktree. A creation request for a session that already holds an `Active` assignment returns the existing worktree; allocating an additional worktree for the same session requires an explicit override. This makes Ownership item 1 ("one active session owns exactly one active worktree assignment") operational rather than merely descriptive.

### Rename on topic change

When the subject of the work changes, the worktree and its branch are RENAMED, not replaced. Creating a fresh worktree and abandoning the old one leaves an orphan whose name no longer describes its contents.

Rename is constrained by a hard git limitation in this repository: `git worktree move` fails with `fatal: working trees containing submodules cannot be moved or removed`, and no flag relaxes it. Rename is therefore specified as remove, recreate at the new path on the renamed branch, then fetch submodule objects from the main checkout's clones by local path. The local fetch is mandatory because a linked worktree receives a SEPARATE submodule clone under `.git/worktrees/<name>/modules/<submodule>` that lacks commits existing only in the main checkout.

A rename updates the `SessionWorktreeAssignment` path and branch in place, so the session anchor keeps resolving across the rename instead of pointing at a removed directory. Rename is distinct from rotation: rotation starts a new assignment lineage, rename preserves the existing one.

### Finish

A finish operation takes a session's worktree from work-complete to released: rebase onto the updated `main`, mark the branch ready to merge, remove the worktree. Consistent with repository policy, finish never merges into `main` and never commits to `main`. Removing a worktree with uncommitted or unpushed work is refused unless explicitly forced, and the refusal names what would be lost.

## Additional Acceptance Criteria

8. This spec defines the session id as the sole workspace anchor and prohibits process-working-directory fallback, with unresolvable selections rejected rather than defaulted.
9. This spec defines the resolved scope as typed, sufficient to distinguish repository root, worktree, and main checkout, and to support blocking mutating work in the main checkout.
10. This spec defines preservation of uncommitted main-checkout changes on worktree creation, with no silent-stranding path.
11. This spec defines one-worktree-per-session reuse as the default, with explicit override required for additional allocation.
12. This spec defines rename-on-topic-change as remove plus recreate plus local submodule-object fetch, updates the assignment in place, and distinguishes rename from rotation.
13. This spec defines a finish operation that rebases, marks ready to merge, and removes, without merging or committing to `main`, and refuses to discard uncommitted work unless forced.

## Additional Traceability

- Lifecycle implementation: ticket `ff83caf7-059b-4f2e-a0fb-eaa7757096a8` — Managed session-worktree lifecycle: preserve, reuse, rename, and finish.
- Resolution protocol: ticket `fa2ba34b-59ec-4321-a4ce-a3c3a9295ea3` — session-anchored MCP workspace resolution.
- Bootstrap defect: ticket `503b9711-3f69-4765-88f9-83779b71c8f8` — offline submodule population, which carries the field evidence for the rename constraint.
- Design source: `transcripts/06-08-2026_worktree-session-proxy/merged.clean.md`.

## Known defect in this spec

The Traceability links above use absolute paths under `C:/Users/linus_behrbohm/git/SECOND_CHECKOUT/...`, which do not resolve in this repository and violate the repository's repo-root-relative reference policy. They are left unmodified here to keep this refinement scoped; correcting them is follow-up work.

## Session Identity and Cross-Session Handoff Protocol (2026-08-10)

### Motivation

An implementation agent needs a stable way to identify the active session, its isolated checkout, and the durable evidence needed to resume a predecessor's work without replaying a raw transcript. Ambiguous session identifiers can select the wrong session record or make a valid CLI lookup appear missing.

### Dependent Expectation

If this specification is implemented, dependents can rely on agents resolving and declaring the authoritative per-session UUID, worktree path, and branch before substantive work; on cross-session handoffs consulting durable records before bounded transcript evidence; and on tools receiving the identifier form required by the selected command.

### R1: Dual identity and identifier discipline

The system exposes two identifiers that agents MUST NOT conflate:

1. `workspace_session_id` is a slug-plus-hex logical workspace identifier, for example `epic-kickoff-8fdfe135`. Its local source of truth is `.session/local/active_workspace_session.json`, whose only keys are `workspace_session_id` and `updated_at`.
2. The per-session identifier is a UUID, for example `16263c13-7f29-4780-ba09-bf94190cb87f`, and keys the record directory `.session/sessions/<uuid>/`.

CLI parameter names do not supersede this identity distinction. Some commands accept only the UUID form: `session.exe subagent-rollups --workspace-session-id epic-kickoff-8fdfe135` reports `session data was not found`, while the same command with the UUID succeeds. Agents MUST use the identifier form verified for the command being invoked.

### R2: Self-identification before substantive work

Before the first substantive action, an agent MUST resolve its own identity with `./target/debug/session.exe init --workspace . --toon`. The command returns `context.session_id`, `context.workspace_session_id`, `active_run_id`, and `runs[]`. When the CLI surface is unavailable, the agent MAY read `workspace_session_id` from `.session/local/active_workspace_session.json` as a fallback, while treating a UUID lookup as separately required where the target command requires one.

### R3: Authoritative worktree binding

The isolated checkout for a session is `.worktrees/<short-id>-<slug>` on branch `agent/<short-id>-<slug>`, where `<short-id>` is the first eight hexadecimal characters of the session UUID. `./target/debug/session.exe lookup --session-id <uuid> --workspace . --toon` is authoritative for a session's `worktree_path` and `branch`.

The claim and rename command sequence belongs exclusively to [.agents/instructions/commit/branch-worktree.instructions.md](.agents/instructions/commit/branch-worktree.instructions.md). This specification references that ownership boundary and does not duplicate the commands.

### R4: Transcript-visible traceability

Every agent's first substantive response MUST declare the per-session UUID, worktree path, and branch. Every agent's final response MUST repeat the same three values. This response-level declaration makes lineage greppable from the chat transcript alone; board and session-store records remain supporting evidence rather than the exclusive source.

### R5: Prior-session inspection and handoff evidence

When resuming handed-off work, an agent MUST read durable artifacts in this order: the linked ticket, this or another owning specification, and the handoff package. Only then may the agent inspect bounded transcript slices. An agent MUST NOT dump a raw transcript.

The inspection surfaces are:

- `./target/debug/session.exe sessions-for-ticket <ticket-id> --workspace . --toon` returns `count` and `sessions[]`.
- `./target/debug/session.exe lookup --session-id <uuid> --workspace . --toon` returns `session_id`, `owner_id`, `ticket_id`, `worktree_path`, `branch`, `allocation_mode`, and `status`.
- `./target/debug/session.exe peek-skeleton --session-id <uuid> --preview-chars N --toon` returns `total_turns` and `entries[]{sequence,role,preview,content_len}`.
- `./target/debug/session.exe peek-range --session-id <uuid> --start N --end M --toon` returns `turns[]{sequence,role,content}`.
- `./target/debug/session.exe subagent-rollups --workspace-session-id <uuid> --toon` returns per-run turn, tool-call, and token counts.

Prior handoff packages are read from `.session/sessions/<uuid>/handoffs/<handoff-id>/handoff.json` and `handoff.md`. There is no read subcommand: `session.exe handoff` is write-only and requires `--objective`, `--higher-level-objective`, and at least one `--upward-context` JSON entry.

### Guards and Evidence

The implementation and guidance evidence must demonstrate the identifier distinction with the successful UUID `subagent-rollups` invocation and the failing slug-plus-hex invocation; capture `session.exe init` and `session.exe lookup` TOON output; and demonstrate durable-artifact-first inspection using `sessions-for-ticket`, `peek-skeleton`, and a bounded `peek-range` call. A review of the first and final agent responses must confirm the UUID, worktree path, and branch are declared in both locations.

### Positions

- `partial` — `.session/local/active_workspace_session.json` and the session CLI provide the identity and inspection surfaces; ticket [7be23bd8 Agent session identity, worktree traceability, and prior-session inspection protocol](.ticket/tickets/7be23bd8-9793-4f86-a96d-403824f8af94/ticket.toml) owns the guidance contract that makes agents use them consistently.
- `implemented` — [branch-worktree.instructions.md](.agents/instructions/commit/branch-worktree.instructions.md) owns worktree claim and rename commands.
- `implemented` — [worktree-provisioning.instructions.md](.agents/instructions/session/worktree-provisioning.instructions.md), [session-artifacts.instructions.md](.agents/instructions/orchestration/session-artifacts.instructions.md), and [write-and-die.instructions.md](.agents/instructions/orchestration/write-and-die.instructions.md) define neighboring provisioning, durable-artifact, and worker-lifecycle guidance.

### Governing-rule Requirement

The guidance introduced for ticket 7be23bd8 must introduce this specification's dual-identity, response-traceability, and durable-artifact-first requirements in each agent session. The governing guidance must defer worktree command ownership to [branch-worktree.instructions.md](.agents/instructions/commit/branch-worktree.instructions.md).

### Explicit Non-goals and Known Defect

- Fixing `session.exe query` is out of scope. The current command aborts an entire listing on one unreadable record with `session error: session data was not found at .\\.session\\sessions\\6a51a1af-6812-4dfc-80d7-0e4f56b4af4f\\session.json`.
- Adding a `handoff --read` subcommand is out of scope.
- Changing worktree-provisioning-hook behavior is out of scope.

### Traceability

- Related ticket: [7be23bd8 Agent session identity, worktree traceability, and prior-session inspection protocol](.ticket/tickets/7be23bd8-9793-4f86-a96d-403824f8af94/ticket.toml).
- Related guidance: [branch-worktree.instructions.md](.agents/instructions/commit/branch-worktree.instructions.md), [worktree-provisioning.instructions.md](.agents/instructions/session/worktree-provisioning.instructions.md), [session-artifacts.instructions.md](.agents/instructions/orchestration/session-artifacts.instructions.md), and [write-and-die.instructions.md](.agents/instructions/orchestration/write-and-die.instructions.md).
- Related specifications: `memory-api/session-api/durable-session-workflow`, `memory-api/session-api/handoff-provenance-and-tracks`, and `agent-workflow/handoff-package-schema` define adjacent runtime workflow, handoff-provenance, and package-schema behavior.
