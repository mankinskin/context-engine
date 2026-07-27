## Problem

The `ticket` pilot crate (61ce77f9, done) **duplicated** rather than replaced the legacy ticket transports. Both surfaces exist and build today:

| Surface | Legacy crate | Binary | Pilot equivalent |
|---|---|---|---|
| CLI | memory-api/tools/cli/ticket-cli (52 files, ~12.4k LOC) | `ticket.exe` | `ticket-cli.exe` |
| MCP | memory-api/tools/mcp/ticket-mcp (20 files, ~5.2k LOC) | `ticket-mcp` | `ticket-mcp.exe` |
| HTTP | memory-api/tools/http/ticket-http (49 files, ~13.5k LOC) | — | `ticket-http.exe` |

AGENTS.md, `.agents/instructions/**`, hooks, `.vscode/mcp.json` and `.github/mcp.json` all target the legacy surfaces. The pilot's reference proof covers exactly one operation, so it is not evidence of parity with ~31k LOC of legacy tooling. The duplication must be resolved before `ticket` can be split into its own repository.

## Decisions (interviewed 2026-07-27)

- Canonical CLI binary name is **`ticket`**. Rename the pilot's `ticket-cli` bin target to `ticket` so AGENTS.md, instructions, hooks, and scripts keep working unchanged. Do NOT mass-update docs to `ticket-cli`.
- Parity bar: **move the existing legacy implementations under the pilot crate's bins. No rewrite.** The legacy crates' existing tests are the parity gate and must still pass after the move.
- MCP and HTTP binary names stay `ticket-mcp` and `ticket-http`.

## Scope

1. Re-home the legacy command surface / MCP tool+handler registration / HTTP router registration into `memory-api/crates/ticket/src/bin/*` (and supporting modules under `memory-api/crates/ticket/src/`), preserving behavior.
2. Move the legacy crates' test suites along with the code so they keep running as the parity gate.
3. Rename the pilot bin target `ticket-cli` -> `ticket`.
4. Delete `memory-api/tools/cli/ticket-cli`, `memory-api/tools/mcp/ticket-mcp`, `memory-api/tools/http/ticket-http` and drop them from the memory-api workspace members.
5. Rewire in-tree consumers off the legacy crates: `memory-api/crates/memory-matrix` (depends on ticket-api + ticket-http + ticket-cli + ticket-mcp), `memory-api/crates/audit-api`, `memory-api/crates/session-api`, `memory-viewers/ticket-viewer`, `memory-viewers/ticket-viewer/frontend/dioxus`. Consumers depend on the public `ticket` crate (lib for library use, features for transports).
6. Verify `.vscode/mcp.json` and `.github/mcp.json` still resolve `ticket-mcp` (they invoke by name via the cost-gate wrapper, not by path — no path edits expected, but confirm the installed binary still lands on PATH).

## Acceptance Criteria

- `cargo build -p ticket` (slim) still has no `transport-harness` in `cargo tree -p ticket -e normal`.
- `cargo build -p ticket --features cli,mcp,http` produces binaries named `ticket`, `ticket-mcp`, `ticket-http`.
- The legacy test suites, re-homed under the `ticket` crate, pass — no reduction in test count versus the pre-move baseline (record both numbers).
- `cargo test -p ticket --features cli,mcp,http --test reference_proof` still passes (6 tests).
- The three legacy crates no longer exist and are removed from the workspace; `cargo build --workspace` is green.
- `./target/debug/ticket.exe board show --toon` (or equivalent smoke) works exactly as before.
- `ticket-mcp` still resolves for the MCP configs.
- No doc/instruction churn required for the CLI name (validated by grepping `.agents/instructions/**` and AGENTS.md for `ticket` CLI invocations that would break).

## Non-goals

- No repository split (that is ba4aaa9c).
- No rewrite onto new transport-harness idioms.
- No `.ticket` artifact store migration.