# Unified bootstrap and Copilot CLI agent-surface wiring

## Goal

Reduce fresh-clone onboarding to a single script invocation, and make this
repository's existing agent-config surfaces (MCP servers, custom agent
templates, prompt templates, path-scoped instructions) automatically
discoverable by a Copilot CLI session opened in this repo, with no manual
setup step.

## Current state (as verified live, 2026-08-13)

- Bootstrap requires 5 sequential manual commands (`setup_git.sh`,
  `install-deps.sh`, `install-tools.sh --mcp`, `init.sh`,
  `tools/verify-bootstrap.sh`), documented in README.md.
- `.github/mcp.json` uses `{"servers": {...}}`; Copilot CLI's MCP config
  loader requires `{"mcpServers": {...}}` and silently skips the file
  ("malformed: mcpServers: Required"), so no MCP tool is available to a
  Copilot CLI agent even post-bootstrap.
- `.agents/agents/*.agent.md` (30 templates), `.agents/prompts/*.prompt.md`
  (22 templates), `.agents/instructions/**/*.instructions.md` (50 files) are
  the canonical, tracked sources, but Copilot CLI only auto-discovers
  `.github/agents/`, `.github/prompts/`, `.github/instructions/`
  respectively. None of those `.github/*` paths exist today.
- A submodule bootstrap failure partway through `setup_git.sh` can leave
  a submodule's working tree empty/fully-staged-deleted (observed for
  `memory-viewers`, `viewer-api`, `memory-kernel` after a first bootstrap
  attempt aborted on the `memory-api` step), with no automated detection or
  recovery.
- `install-tools.sh --mcp --dry-run` reports `succeeded=N/failed=0` even
  when underlying `cargo` manifest loads fail.

## Requirements

1. A single root-level `bootstrap.sh` runs the existing 5 steps in order,
   reporting per-step pass/fail with the exact remediation command on
   failure, and stopping at the first hard failure (no silent
   continuation).
2. `bootstrap.sh` (via `init.sh`) regenerates `.github/agents/`,
   `.github/prompts/`, and `.github/instructions/` as plain file copies
   (not symlinks — Windows without Developer Mode/admin cannot create
   symlinks, confirmed live) sourced from `.agents/agents/`,
   `.agents/prompts/`, `.agents/instructions/`. These generated directories
   are derived build output: gitignored, regenerated idempotently on every
   run, never hand-edited.
3. `.github/mcp.json` is fixed to the `{"mcpServers": {...}}` schema Copilot
   CLI requires. `.vscode/mcp.json` (VS Code's own `{"servers": {...}}`
   schema) is left unchanged — the two files serve different consumers and
   must not be unified into one schema.
4. Submodule checkout is hardened: after
   `git submodule update --init --recursive`, each submodule's working tree
   is checked for the "fully staged-deleted" corruption pattern and
   auto-repaired with `git reset --hard HEAD` before continuing; only a
   failure that persists after this repair is reported as fatal.
5. `install-tools.sh`'s dry-run/real success accounting reflects the actual
   exit status of the underlying cargo invocation; a step that errors is
   never counted as succeeded.
6. `README.md` documents `./bootstrap.sh` as the primary Getting Started
   entry point, keeping the 5 discrete commands as the documented internal
   sequence for partial/advanced runs.
7. The stale `context-mcp` README reference to the nonexistent
   `.github/context-mcp-config.json` is corrected to point at the real,
   now-fixed `.github/mcp.json`.

## Non-goals

- Do not change the underlying MCP binaries, agent template content, or
  prompt template content — this work only wires up discovery paths.
- Do not attempt symlink-based sharing on Windows; copy-based generation is
  the portable default for all platforms.
- Do not change `.vscode/mcp.json`'s schema.

## Acceptance / validation

- Fresh clone + `./bootstrap.sh` ends in "Bootstrap verification passed."
  with zero unhandled errors.
- `copilot mcp list` (run from repo root) lists the configured MCP servers
  instead of "No MCP servers configured."
- `.github/agents/`, `.github/prompts/`, `.github/instructions/` exist
  post-bootstrap with content mirroring `.agents/{agents,prompts,instructions}`.
- A simulated submodule working-tree corruption (all tracked files
  staged-deleted) is auto-repaired by `bootstrap.sh` without manual
  intervention.
- `install-tools.sh --dry-run` (or a forced-failure case) reports a
  non-zero failed count when a step's cargo command actually errors.

## Related

- Ticket [08b3c22c Fresh-clone bootstrap fails](../../.ticket/tickets/08b3c22c-eefc-48d4-b2ed-64a9a7b53c98/ticket.toml)
  (resolved; root cause of the original hard blocker).
- Ticket [edb92a7d Unify bootstrap into one script and wire up Copilot CLI agent surfaces](../../.ticket/tickets/edb92a7d-b735-4c7b-b339-36847df68f76/ticket.toml)
  (this spec's implementation ticket).
