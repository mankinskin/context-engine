# Rule Doc Duplication Inventory

## Scope

This inventory captures the current migration state for the phase-one rule-doc work across:

- `context-engine`
- `memory-viewers`
- `memory-viewers/memory-api`
- `memory-viewers/viewer-api`

The goal is to identify which user-facing markdown surfaces are already owned by canonical rule content and which shared files still remain byte-identical static copies.

## Current Canonical Owners

| Repo | Output file(s) | Current owner | Notes |
| --- | --- | --- | --- |
| `context-engine` | `AGENTS.md` | root `rule-targets.yaml` target `context-engine-agents` | Generated from canonical root `shared/agent-rules/*` entries. |
| `memory-viewers` | `AGENTS.md` | root `rule-targets.yaml` target `memory-viewers-agents` | Generated from the same canonical root `shared/agent-rules/*` entries as the root repo output. |
| `memory-viewers/memory-api` | `AGENTS.md` | root `rule-targets.yaml` target `memory-api-agents` | Generated from the same canonical root `shared/agent-rules/*` entries as the other AGENTS outputs. |
| `memory-viewers/viewer-api` | `AGENTS.md` | root `rule-targets.yaml` target `viewer-api-agents` | Generated from the same canonical root `shared/agent-rules/*` entries as the other AGENTS outputs. |
| `context-engine` | `.github/README.md` | `rule-targets.yaml` target `context-engine-github-readme` | Generated from canonical `github-copilot-configuration/*` sections. |
| `context-engine` | `.github/copilot-instructions.md` | `rule-targets.yaml` target `context-engine-copilot-instructions` | Generated from canonical `github-copilot-instructions/*` sections. |
| `context-engine` | `.agents/instructions/*.instructions.md` | `rule-targets.yaml` targets `context-engine-instruction-*` | Path-scoped instruction bodies are generated from canonical rule entries and preserve repo-specific `applyTo` metadata at render time. |
| `memory-viewers` | `README.md` | `memory-viewers/rule-targets.yaml` target `memory-viewers-readme` | Generated from canonical `memory-viewers/*` sections in the nested `.rule` store. |
| `memory-viewers/memory-api` | `README.md` | `memory-viewers/memory-api/rule-targets.yaml` target `memory-api-readme` | Generated from canonical `memory-api/*` sections. |
| `memory-viewers/memory-api` | `tools/cli/*/README.md`, `tools/mcp/*/README.md`, `tools/http/*/README.md` | `memory-viewers/memory-api/rule-targets.yaml` targets `*-readme` under `tools/*/readme` sections | Generated onboarding READMEs now exist for all CLI, MCP, and HTTP tool surfaces in the repo. |
| `memory-viewers/viewer-api` | `README.md` | `memory-viewers/viewer-api/rule-targets.yaml` target `viewer-api-readme` | Generated from canonical `viewer-api/*` sections. |
| `memory-viewers/viewer-api` | `viewer-ctl/README.md` | `memory-viewers/viewer-api/rule-targets.yaml` target `viewer-ctl-readme` | Generated onboarding README for the viewer lifecycle manager, owned by canonical `tools/viewer-ctl/readme` content. |

## Shared AGENTS Migration

The former byte-identical nested `AGENTS.md` group now renders from one canonical source in the root `.rule` store:

- canonical sections: `agent-rules/*`
- canonical slugs: `shared/agent-rules/*`
- generated outputs:
	- `AGENTS.md`
	- `memory-viewers/AGENTS.md`
	- `memory-viewers/memory-api/AGENTS.md`
	- `memory-viewers/viewer-api/AGENTS.md`

This removes copy-paste ownership of the shared `Agent Rules` body while preserving per-file provenance comments in each generated output.

## Path-Scoped Instruction Status

The path-scoped instruction migration is already represented in generated form at the root repo:

- `.agents/instructions/audit.instructions.md`
- `.agents/instructions/context-http.instructions.md`
- `.agents/instructions/core-crates.instructions.md`
- `.agents/instructions/frontend.instructions.md`
- `.agents/instructions/mcp-tools.instructions.md`
- `.agents/instructions/tests.instructions.md`
- `.agents/instructions/ticket-system.instructions.md`
- `.agents/instructions/viewer-api-tools.instructions.md`

Each file is rendered from canonical rule entries and keeps repo-local `applyTo` behavior in the file-level metadata instead of duplicating that metadata per paragraph.

## Regeneration and Validation Evidence

- Root generated docs are checked by `.githooks/pre-commit` through `rule sync-targets --check` for `rule-targets.yaml`.
- `memory-viewers/README.md` is checked by `rule sync-targets --check` in `memory-viewers`.
- `memory-viewers/memory-api/README.md` and the tool-local onboarding READMEs are checked by `rule sync-targets --check` in `memory-viewers/memory-api`.
- `memory-viewers/viewer-api/README.md` and `viewer-ctl/README.md` are checked by `rule sync-targets --check` in `memory-viewers/viewer-api`.

## Remaining Migration Gap

No remaining byte-identical markdown group remains in the current phase-one migration inventory. Further work, if any, is follow-up refinement rather than unresolved ownership migration.