Tracking epic for the repository restructuring that turns this monorepo into an instantiated example of a general, self-improving agent framework, and extracts the workflow tooling into standalone, independently-versioned repositories.

## Goal
The whole repository should act as an environment in which an AI agent can operate autonomously: discover its tools, find next tasks, build and refine its own workflow, spawn sub-agents, gather user feedback, and self-improve. To generalize beyond this repo, we separate:
- the **target environment / application** (the context-stack: context-api/insert/read/search/trace + context-engine's own generated artifacts), from
- the **workflow tooling** (the memory-api domain stores, their transports, the viewer framework, and the guidance/skill), which becomes an installable dependency that any target project can consume.

## Locked decisions (interview 2026-07-25, refined after review)
- GitHub owner: `mankinskin`.
- Granularity: **full per-tool split immediately**.
- **Crate structure:** each domain tool is a **single domain crate** `{domain}` whose **lib is the primary build target/handle**. `{domain}-api` is kept as an **internal crate** the domain crate depends on and **re-exports**. Each transport (cli, mcp, http) is a **feature-gated binary target (`[[bin]]`)** of the domain crate, built on the shared **`transport-harness`** crate (`dbe0e955`), with current binary names preserved. Frontends (viewer, vscode) stay separate crates. See contract `0da6894c`.
- Per-tool repo naming: **bare domain name**.
- Viewers bundle into their owning tool repo as a separate frontend crate (memory-viewers dissolved).
- Shared storage kernel extracted and **renamed `memory-kernel`** (frees `memory-api`).
- Umbrella repo **workflow-tools** aggregates tool + shared repos; the dependency unit per tool is its domain crate (with feature-selected transport bins passed through).
- Skill: dedicated **workflow-skill** repo on skills.sh; installable at root + nested tools; single active install ignores nested installs and can self-uninstall.
- **context-engine consumption:** consumes each tool as a **domain crate lib dependency AND invokes the installed transport bins** (mcp/cli/http) as tools.
- context-engine keeps only context-stack + its own generated artifacts; workflow tooling is an installed dependency.
- workflow-tools gets repo-level self-referential artifact stores now; per-tool artifact stores follow.
- interview repo pre-created as a placeholder.

## Repositories to create on GitHub (owner: mankinskin)
Shared libraries: `memory-kernel`, `memory-fixtures` (viewer-api already exists). The shared `transport-harness` crate defaults to living inside the `memory-kernel` repo (final placement is a minor open sub-decision in `dbe0e955`).
Per-tool repos (single domain crate + feature-gated transport bins + separate frontend crates): `ticket`, `spec`, `rule`, `doc`, `test`, `log`, `feedback`, `session`, `audit`, `peek`, `interview`.
Umbrella + packaging: `workflow-tools`, `workflow-skill`.
Existing: `viewer-api`, `context-stack` kept; `memory-api` name freed; `memory-viewers` dissolved.

## Migration phases
- Phase 0 — Provisioning: user creates the repos; agent waits.
- Phase A — Foundations + crate contract: finalize domain-crate contract (`0da6894c`) + shared `transport-harness` (`dbe0e955`); extract `memory-kernel`; stabilize `viewer-api` + `memory-fixtures`.
- Phase B — Per-tool extraction: split each of the 11 tools into its own repo built around the single domain crate (lib + feature-gated transport bins over the harness) + separate frontend crates; migrate artifacts via the move tooling.
- Phase C — Umbrella + artifacts: create `workflow-tools`; add repo-level artifact stores; migrate tool-scoped artifacts.
- Phase D — Skill packaging: author `workflow-skill`; scope/precedence for root + nested installs, single active install, self-uninstall.
- Phase E — context-engine reframing: reduce to context-stack + own artifacts; consume domain crate libs + installed transport bins; update entry points across all three install sites.
- Phase F — Validation & cutover: end-to-end build/test/MCP/CLI/viewer + browser verification; migration guide.

## Cross-store prerequisite (recorded textually; edges cannot cross ticket stores)
Artifact moves depend on the memory-api store's move tooling `505b2cd4` "Deliver safe cross-workspace ticket move for git-backed stores" (+ children); must be green before Phase B/C artifact migration.

## Related existing work to link (default store)
- `671d4e47` cross-store interaction tracker — complementary (linked).
- `13912e44` neutral naming map / `2b1279bd` neutral storage kernel — inform memory-kernel (linked).
- `b13c5d89` Agent Skill Foundation — feeds workflow-skill (linked).

## Acceptance criteria
- Complete dependency graph; every child implementation-ready and validated.
- Domain-crate contract + transport-harness finalized; every tool repo built as a single domain crate (lib primary target) with feature-gated transport bins over the shared harness, internal api crate re-exported, and separate frontend crates.
- All new repos build/test independently; workflow-tools aggregates them.
- context-engine builds using workflow-tools as an installed dependency (domain crate libs + transport bins), retaining only context-stack + its own artifacts.
- workflow-skill installs from skills.sh; entry point wired at root + nested tools; scope de-dup + self-uninstall.
- Artifact moves preserve cross-reference integrity; no dangling references.
- Migration guide + install docs published; browser verification recorded for viewer changes.