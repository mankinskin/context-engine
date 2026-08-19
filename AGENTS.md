# Agent Rules

Global working rules for this repository. Keep this file small and stable.

## Operating Principles

- Gather context before coding. Do not guess.
- Read existing tests to infer expected behavior.
- For implementation work, create or update the relevant ticket(s) before editing code.
- For new or changed requirements and goals, create or update the relevant spec before implementation proceeds.
- Keep the ticket, spec, validation, and documentation trail current so review and status summaries stay accurate.
- Commit every completed change on its owning feature branch; uncommitted completed work is not a valid handoff.
- Prefer bash commands over PowerShell/cmd.
- Use Unix-style paths (`/`) in commands and docs.
- Read test logs in `target/test-logs/` for debugging instead of relying on truncated test stdout.
- Keep scope tight: do not add extra features or broad refactors unless requested.
- Declare the session identity and assigned worktree at the start of each session, then repeat both in the final response; follow [session-identity-and-handoff.instructions.md](.agents/instructions/session/session-identity-and-handoff.instructions.md).

## Discovery Protocol (Before Editing)

Use live sources first:

1. Documentation: use doc-viewer MCP tools to locate relevant module docs.
2. Known issues/plans: use ticket-mcp tools before duplicating work.
3. Board state: check active WIP, stale entries, and file ownership before touching
   implementation files — `mcp_ticket-mcp_board_show` with `{"workspace": "default"}` or:
   ```bash
   ./target/debug/ticket.exe board show --toon
   ```
4. Test failures: use log-viewer MCP tools (`get_log`, `search_all_logs`, `query_logs`).
5. Graph/workspace behavior: use context-mcp tools for context-engine operations.

Use static references as support:

1. Relevant crate `README.md` and `HIGH_LEVEL_GUIDE.md` for design context and
  API patterns.
2. Existing tests for usage examples, assertions, and type-level behavior.

## Task Routing

- Any requested implementation or behavior change: create or update the tracking ticket(s) first, then create or update the relevant spec before editing files.
- Worktree-backed work is required for changes spanning multiple files or components, submodules, active concurrent work, or risky behavior changes. Create a worktree branch, claim the session and board, and use `worktree-ctl` for bootstrap, rebase, merge, and teardown; see [branch-worktree.instructions.md](.agents/instructions/commit/branch-worktree.instructions.md#bottom-up-integration-sequence-canonical). Rebase and integrate affected submodules before the superproject.
- A small, self-contained change to one existing file or the addition of one new file may be made in the main checkout. Verify that no active board entry owns the path, stage only the changed path, and validate before committing. A small main-checkout change does not require worktree provisioning, `session_check_in`, or `board_check_in`.
- Simple fix (1-2 files): after the ticket/spec setup when requirements or behavior change, gather context, implement, validate, update docs, verify spec links, and move the ticket to `in-review`.
- Bug fix: after the ticket/spec setup, follow `.agents/prompts/debug-test.prompt.md` when available.
- Feature or refactor (>5 files, >100 LOC, or unclear scope): use `.agents/prompts/tickets.prompt.md` to establish the ticket set, then `.agents/prompts/spec.prompt.md` to update the spec before implementation.
- Unfamiliar module or unclear behavior: follow `.agents/prompts/research.prompt.md` when available before locking the spec or implementation plan.

## Quality Gates

- Relevant validation must pass before completion. If a required check repeatedly fails, stop expanding scope and record the failing command, log or manual result, and blocker clearly in the ticket/spec status summary.
- Before a ticket moves to `in-review`, ensure the relevant spec is updated for the changed requirements or goals and links the related tickets, updated docs, and test or validation results.
- **Browser verification is mandatory** for any change to a server interface or frontend feature:
  open the affected viewer in an external fullscreen Chromium-family browser, not VS Code's integrated browser, and confirm the feature works visually before marking work done.
- Record the browser window or display resolution used for manual visual validation whenever layout, rendering, or responsive behavior could affect the result.
- **Write Playwright end-to-end tests** for all browser-facing features and server interface changes.
  When executing browser-hosted frontend checks, first try the MCP Playwright/browser tools. Fall back to repo-local Playwright commands only when the MCP surface is unavailable or cannot cover the scenario.
  Capture screenshots during Playwright verification for UI-facing changes so the rendered state is visually confirmed, not inferred only from DOM assertions.
  For modals, overlays, drawers, popovers, menus, and similar transient surfaces, include at least one screenshot with the surface open and, when useful, a before/after pair.
  Shared managed-viewer suites live under `viewer-api/viewer-api/frontend/dioxus/e2e/shared/`.
  Spec-viewer release E2E lives under `memory-viewers/spec-viewer/frontend/dioxus/`; run it with `npm run test:e2e:release`.
  Ticket-viewer release E2E lives under `memory-viewers/ticket-viewer/frontend/dioxus/`; run it with `npm run test:e2e:release`.
  Doc-viewer and log-viewer keep local Playwright wrappers under `memory-viewers/doc-viewer/e2e/` and `memory-viewers/log-viewer/e2e/`, importing shared suites from `memory-viewers/viewer-api`.
- For tracing-based tests, use:

```rust
let _tracing = init_test_tracing!(&graph);
```

- If public behavior or docs changed, update the docs and run doc validation workflows.
- When dedicated test, doc, or cross-store-link tooling is missing or partial, use the strongest available command or manual check and call out the limitation explicitly in the status summary and spec traceability.
- Follow `.github/hooks/` reminders when they fire.
- Scratch notes belong in temporary files only; do not commit ephemeral notes.
- Follow the closed-loop iteration workflow: Review→Interview→Commit→Handoff. See [loop-closure.instructions.md](.agents/instructions/orchestration/loop-closure.instructions.md).
- When a handoff package is incomplete or requirements are ambiguous, escalate rather than clarifying inline during implementation. See [escalation-gate.instructions.md](.agents/instructions/orchestration/escalation-gate.instructions.md).
- Never commit directly to `main` for worktree-backed work — all such commits land on the feature branch. After the branch is rebased clean and validation passes, the session merges its own feature branch into `main` (bottom-up: rebase every affected submodule then the superproject onto updated `main`, resolve conflicts on the feature branch, then fast-forward each `main`). A validated small main-checkout change may commit its explicitly staged path directly to `main`. See [branch-worktree.instructions.md](.agents/instructions/commit/branch-worktree.instructions.md#bottom-up-integration-sequence-canonical).

## Feedback Workflow

- Record feedback in the entity feedback store today. Use the canonical entity URN for the target, for example `ce://default/spec/<spec-id>` or `ce://default/ticket/<ticket-id>`.
- When feedback came from a specific hand-maintained instruction or prompt file, target the entity URN for the spec or ticket that owns that guidance instead of the file path.
- Record or inspect feedback with the feedback-api transports:
  - CLI: `feedback ingest|inbox|summary --store-root <path-to-.feedback> --workspace-slug <slug> --target <ce://...>` with `--source`, optional `--rating`, `--note`, `--note-kind`, `--session-id`, and `--author` on `ingest`.
  - MCP: `feedback_ingest`, `feedback_inbox` or `feedback_query`, `feedback_summary`, and `feedback_mine`.
- Use `feedback_summary` or `feedback summary` when you need the current low-signal state for an entity; use `feedback_inbox` or `feedback inbox` when you need the raw stored entries that explain why follow-up is needed.
- If feedback implies a contract or workflow change, open or update the corresponding spec or ticket and link the exact entity that received the feedback instead of leaving the signal stranded in chat only.

## Escalation Rules

- If blocked by ambiguity after focused research (10-15 minutes), ask the user.
- If evidence conflicts or architecture tradeoffs are required, ask before committing to a direction.
- In multi-agent workspaces, treat unrelated workspace changes as expected background activity and continue.
- Before worktree-backed editing, claim ownership of the files you will touch; commit only your owned changes, and release ownership when done. Before a small main-checkout change, inspect the board and do not touch a path actively owned by another agent.
- Ignore unrelated changes by default; do not interrupt work solely because they exist.
- Escalate only when unrelated changes create a real conflict with your owned scope (for example merge conflicts, overlapping owned paths, or failures directly caused by those changes).
- Never revert, stage, or commit unrelated changes created by other agents.

## Token-Efficient Output

See token-efficient workflow guidance in [.agents/instructions/orchestration/](.agents/instructions/orchestration/) covering compact output, bounded file inspection, tool output handling, differential patching, and model-cost-aware routing. For ticket reads, default to the narrowest `--view` profile (`summary` to orient, `plan` to implement, `review` to verify) instead of pulling a whole ticket — see [ticket/workflow.instructions.md](.agents/instructions/ticket/workflow.instructions.md).

## Clickable Reference Policy

Render every reference to a workspace entity (ticket, spec, doc, log, or one of their files) as a clickable markdown link in **all** agent and prompt responses. This entry is the single canonical owner of reference formatting for the repository: the "Formatting conflict policy" note in the Instruction Precedence section defers here, and switching the reference format is done here once.

**Scope.** These rules govern the reference token you emit in a response — the markdown link and the path inside it. They do not govern ordinary prose that merely names a file, nor backticked shell commands. The anti-backtick rule below applies to the emitted reference, not to illustrative prose in this policy.

Emit a reference in exactly one of three forms, selected by the active reference mode (default: manifest):

1. viewer — a deep link to the domain viewer server that opens the entity. Routes that exist today:
   - ticket-viewer: http://localhost:3002/workspace/{workspace}/ticket/{id}
   - spec-viewer: http://localhost:4002/specs/{id}
   - log-viewer: http://localhost:3000/#/file/{url-encoded-log-name} (append /stats or /hypergraph for those tabs)
   - doc-viewer (port 3001) has no stable per-entity deep-link route yet — its artifacts are keyed by package::target, not a URL. Use manifest or description mode for docs until a route exists.
2. manifest — a relative link to the entity's manifest file: a ticket's ticket.toml, a spec's spec.toml, or the equivalent manifest.
3. description — a relative link to the entity's top description/body file: a spec's body.md, or a ticket's rendered description.md.

Link text is always "{short-id} {title}", where {short-id} is the first 8 characters of the authoritative entity id and {title} is the authoritative entity title.

Path normalization for every emitted reference:
- Use forward-slash (unix) paths only; convert any Windows backslashes.
- Use repo-root-relative paths; never emit a drive-letter absolute path (no C:/… form).
- Assume a mingw (Git Bash) or WSL shell, so a repo-root-relative unix path resolves for both file links and terminal use.
- Do not wrap the emitted reference — its link text or its path — in backticks.

Resolve manifest and description paths from the owning API (ticket-api, spec-api, and so on), not from a template. If the first response omits the folder path, run a follow-up call (for example ticket get {id} --json and read the payload's ticket path) before composing the reference.
