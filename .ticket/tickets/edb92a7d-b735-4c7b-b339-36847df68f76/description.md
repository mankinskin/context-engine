# Unify bootstrap into one script + wire up Copilot CLI agent surfaces

## Problem

Bootstrapping `context-engine` today needs 5 separate manual commands
(`setup_git.sh`, `install-deps.sh`, `install-tools.sh --mcp`, `init.sh`,
`tools/verify-bootstrap.sh`), and even after that succeeds, a Copilot CLI
agent session in this repo has **no working MCP tools, no custom agents, no
prompt templates, and misses path-scoped instructions** — because the
repo-local config lives in paths Copilot CLI does not auto-discover.

## Confirmed root causes (verified live in this session)

1. **`.github/mcp.json` uses the wrong top-level key.** Copilot CLI's MCP
   loader requires `{"mcpServers": {...}}`. The repo file uses
   `{"servers": {...}}` (correct for VS Code's `.vscode/mcp.json`, wrong for
   Copilot CLI). Verified with:
   ```
   $ copilot mcp list
   No MCP servers configured.
   Warning: skipping workspace MCP config ".github\mcp.json" because it is
   malformed: mcpServers: Required
   ```
   Result: none of the 12 configured MCP servers (context-mcp, ticket-mcp,
   spec-mcp, ...) are available to a Copilot CLI agent, even after full
   bootstrap.

2. **Custom agents are not discoverable.** Copilot CLI auto-loads custom
   agent profiles only from `.github/agents/*.agent.md` (repo/user/org/
   enterprise precedence). This repo's 30 agent templates live in
   `.agents/agents/*.agent.md`, a path Copilot CLI never scans.

3. **Prompt templates are not discoverable.** Copilot CLI/VS Code auto-load
   reusable prompt files only from `.github/prompts/*.prompt.md`. This
   repo's 22 prompt files live in `.agents/prompts/*.prompt.md`.

4. **Path-scoped instructions are not discoverable.** Copilot CLI auto-loads
   `.github/instructions/**/*.instructions.md` (plus root `AGENTS.md` and
   `.github/copilot-instructions.md`, which already work). This repo's
   path-scoped instructions live in `.agents/instructions/*.instructions.md`.

5. **Stale doc reference.** `context-stack/tools/mcp/context-mcp/README.md`
   tells users to run
   `copilot --additional-mcp-config @.github/context-mcp-config.json`, but
   that file does not exist in the repo.

6. **Bootstrap requires 5 manual commands** with no single entry point, and
   partial failures (e.g. a submodule pinned to a temporarily-unreachable
   commit, see ticket `08b3c22c`) can leave submodules with silently empty/
   corrupted working trees that are hard for a newcomer to diagnose or
   recover from (see ticket `08b3c22c` resolution notes for the exact
   symptom: `git status` shows every tracked file as staged-deleted).

7. `install-tools.sh --mcp --dry-run` mis-reports success even when
   underlying cargo manifest loads fail (see ticket `08b3c22c`).

## Goal

A single script (e.g. `./bootstrap.sh`) that a brand-new contributor (human
or agent) runs once from a fresh clone and ends up with:

- All 5 submodules cloned, checked out, and on their tracked branch.
- All developer dependencies + all repo tools/MCP binaries installed.
- Ticket/spec/rule/doc stores initialized.
- Copilot CLI (and, ideally, VS Code) able to see: all MCP servers, all
  custom agents, all prompt templates, and all path-scoped instructions,
  with no additional manual step.
- A clear, itemized pass/fail report at the end (reusing/extending
  `tools/verify-bootstrap.sh`), where each failed step names the exact
  remediation command.
- Where safe, automatic detection + repair of known-recoverable failure
  modes (e.g. a submodule with a corrupted/empty working tree gets
  `git reset --hard` automatically instead of just failing).

## Proposed approach

1. Fix `.github/mcp.json` to use `"mcpServers"` as the top-level key
   (matching Copilot CLI's schema), keeping `.vscode/mcp.json` as-is (VS
   Code's own `"servers"` schema is separate and already correct).
2. Make `.github/agents/`, `.github/prompts/`, and `.github/instructions/`
   discoverable without duplicating content — prefer generating them (at
   bootstrap time, from `init.sh` or the new unifying script) as symlinks
   (or, on platforms/filesystems without symlink support, generated
   passthrough copies) pointing at the existing canonical
   `.agents/agents/`, `.agents/prompts/`, `.agents/instructions/` sources,
   so `.agents/` remains the single source of truth per
   `.github/copilot-instructions.md`.
3. Add a top-level `bootstrap.sh` that runs, in order, with per-step status
   reporting: `setup_git.sh` → `install-deps.sh` → `install-tools.sh --mcp`
   → `init.sh` (incl. the new agent/prompt/instruction wiring) →
   `tools/verify-bootstrap.sh`. On failure of a step, print which step
   failed, why, and the exact command to re-run just that step; do not
   silently continue past a hard failure.
4. Harden `setup_git.sh`/submodule handling: after
   `git submodule update --init --recursive`, verify each submodule's
   working tree is non-empty/matches its tree (cheap `git status
   --porcelain` check); auto-recover with `git reset --hard HEAD` when the
   whole tree is staged-deleted, and only then fail loudly if recovery
   doesn't fix it.
5. Fix the `install-tools.sh` dry-run success/failure accounting bug from
   ticket `08b3c22c` so failures are never masked.
6. Update `README.md` Getting Started to document the single
   `./bootstrap.sh` entry point (keep the itemized steps as the documented
   "what it does internally" / advanced/partial-run reference).
7. Fix the stale `context-mcp` README reference to the nonexistent
   `.github/context-mcp-config.json`.

## Acceptance

- Fresh clone + `./bootstrap.sh` (or `bash bootstrap.sh`) completes with
  zero unhandled errors, ending in "Bootstrap verification passed."
- `copilot mcp list` in the repo shows all configured MCP servers.
- `/agent` in Copilot CLI lists the repo's custom agent templates.
- Prompt files under `.agents/prompts/` are invocable as `/<name>` slash
  commands in Copilot CLI/VS Code.
- Path-scoped instructions under `.agents/instructions/` are picked up
  (verify via `copilot /instructions` or equivalent listing).
- A submodule bootstrap failure (simulated: force a submodule into a
  staged-deleted working tree) is auto-recovered by `bootstrap.sh` without
  manual `git reset --hard`.
- README.md documents the single-script flow.

## Related

- Follows up on ticket `08b3c22c` (fresh-clone bootstrap blocker /
  submodule pin, resolved).
