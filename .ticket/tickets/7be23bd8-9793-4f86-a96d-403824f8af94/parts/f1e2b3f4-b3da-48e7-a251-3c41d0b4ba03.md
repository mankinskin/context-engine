# Problem

Agents work in isolated Git worktrees and hand work between sessions, but repository guidance does not make session lineage legible. The implementation scope is guidance only.

## Verified Gaps

1. No rule requires an agent to discover and state its own session identity. `session_check_in` accepts `session_id`, but no instruction tells an agent to announce the value.
2. `.agents/instructions/commit/branch-worktree.instructions.md` documents claim-and-rename ordering in "1b. Name the topic (rename the worktree)": rename before `session_check_in`. No first-turn checklist requires agents to follow the documented sequence before the first edit.
3. No rule requires a final response to identify the worktree, branch, and session id. `board_check_out` may carry the lineage in `handoff_reason`, but chat transcripts cannot reliably surface the structured field.
4. `.agents/instructions/orchestration/session-artifacts.instructions.md` correctly prefers durable artifacts and forbids raw transcript dumping, but offers no concrete verified commands for extracting learning from a prior session.

## Verified Technical Facts

- Two distinct identifiers exist. The workspace session id uses slug-plus-hex form, and is stored under `workspace_session_id` in `.session/local/active_workspace_session.json`; `updated_at` is the only other key. Obtain the value with `jq -r .workspace_session_id .session/local/active_workspace_session.json`.
- A per-session UUID, for example `16263c13-7f29-4780-ba09-bf94190cb87f`, keys on-disk session records under `.session/sessions/<uuid>/`. Several commands accept only the UUID: `session.exe subagent-rollups --workspace-session-id epic-kickoff-8fdfe135` failed with `session data was not found`.
- Worktrees follow `.worktrees/<short-id>-<slug>`, with `<short-id>` equal to the first eight hexadecimal characters of the session UUID. `tools/worktree/worktree.sh new <short-id> <slug>` rejects non-plain-identifier short ids.
- The verified session CLI is `./target/debug/session.exe`. Verified read commands are: `init --workspace . --toon` (returns `context.session_id`, `context.workspace_session_id`, `active_run_id`, and `runs[]`); `lookup --session-id <uuid> --workspace . --toon` (returns `session_id`, `owner_id`, `ticket_id`, `worktree_path`, `branch`, `allocation_mode`, and `status`); `peek-skeleton --session-id <uuid> --preview-chars N --toon` (returns `total_turns` and `entries[]{sequence,role,preview,content_len}`); `peek-range --session-id <uuid> --start N --end M --toon` (returns `turns[]{sequence,role,content}`); `subagent-rollups --workspace-session-id <uuid> --toon` (returns per-run turn/tool/token counts); and `sessions-for-ticket <ticket-id> --workspace . --toon` (returns `count` and `sessions[]`).
- Known defect, out of scope: `session.exe query --workspace . --limit 5 --toon` aborts with `session error: session data was not found at .\.session\sessions\6a51a1af-6812-4dfc-80d7-0e4f56b4af4f\session.json`; one corrupt or missing record prevents the entire listing rather than being skipped. Record the observation as a follow-up; do not fix it in this ticket.
- Handoffs live at `.session/sessions/<uuid>/handoffs/<handoff-id>/handoff.json` and `handoff.md`. `session.exe handoff` is write-only; it requires `--objective`, `--higher-level-objective`, and at least one `--upward-context` JSON entry. Prior handoffs must be read from the durable files.

## Scope

Create `.agents/instructions/session/session-identity-and-handoff.instructions.md` as the repository guidance owner for session identity, first-turn declaration, final-response traceability, and durable-first prior-session inspection. Link to, rather than duplicate, the claim-and-rename sequence in `.agents/instructions/commit/branch-worktree.instructions.md`. Add discoverability pointers from `AGENTS.md` and `.agents/instructions/commit/branch-worktree.instructions.md`.

The working context for this ticket is branch `agent/8fdfe135-session-traceability-guidance` in worktree `.worktrees/8fdfe135-session-traceability-guidance`.

## Acceptance Criteria

1. AC1: A single instruction file exists at `.agents/instructions/session/session-identity-and-handoff.instructions.md` with `applyTo: "**"`, defining the session-identity protocol.
2. AC2: The new instruction distinguishes `workspace_session_id` from the per-session UUID and gives the exact verified command to obtain each identifier.
3. AC3: The new instruction mandates a first-turn opening declaration containing session id, worktree, and branch, plus a final-response traceability footer using an exact template.
4. AC4: The new instruction includes at least five copy-pasteable, verified prior-session inspection recipes consistent with the durable-artifact-first and never-dump-raw-transcripts rule in `.agents/instructions/orchestration/session-artifacts.instructions.md`.
5. AC5: The new instruction links to, but does not duplicate, the rename/claim command sequence owned by `.agents/instructions/commit/branch-worktree.instructions.md`.
6. AC6: `AGENTS.md` and `.agents/instructions/commit/branch-worktree.instructions.md` each contain a discoverability pointer to the new instruction.

## Prior-Session Recipe Baseline

The implementation must present at least five of these verified recipes: identify active ids with `./target/debug/session.exe init --workspace . --toon`; resolve a UUID to worktree and branch with `./target/debug/session.exe lookup --session-id <uuid> --workspace . --toon`; inspect a compact turn index with `./target/debug/session.exe peek-skeleton --session-id <uuid> --preview-chars 240 --toon`; read a bounded selected range with `./target/debug/session.exe peek-range --session-id <uuid> --start <n> --end <m> --toon`; summarize subagent activity with `./target/debug/session.exe subagent-rollups --workspace-session-id <uuid> --toon`; find records associated with a ticket via `./target/debug/session.exe sessions-for-ticket <ticket-id> --workspace . --toon`; and inspect durable handoff files at `.session/sessions/<uuid>/handoffs/<handoff-id>/handoff.json` and `handoff.md`.

## Related, Non-Duplicate Work

- Ticket `490f1cbc-8ae9-434a-9eef-d09433b25798` only injects a session id into rendered instructions at runtime.
- Ticket `9577b114-ec11-431b-8740-c488bef05fc9` is completed and scopes durable identity to the generated `/handoff` prompt.
- Ticket `68a49ca7-a6f6-42a8-b820-0a86e6a4de2e` is completed planning for worktree-backed sessions.

## Out of Scope

Do not modify session-api behavior, the session CLI, transcript storage, or the known `session.exe query` defect.
Merged as commit 944fe311 into local main; new instruction file .agents/instructions/session/session-identity-and-handoff.instructions.md; validation run: git diff --check (pass) and bash tools/agent-hooks/validate-docs.sh (pass); known follow-up: session.exe query listing defect (out of scope).