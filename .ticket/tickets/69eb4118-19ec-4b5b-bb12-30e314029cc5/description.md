Tracking epic for the repository restructuring that turns this monorepo into an instantiated example of a general, self-improving agent framework, and extracts the workflow tooling into standalone, independently-versioned repositories.

## Goal
The whole repository should act as an environment in which an AI agent can operate autonomously: discover its tools, find next tasks, build and refine its own workflow, spawn sub-agents, gather user feedback (interviews, reviews, ratings), and self-improve the tools/workflows/environment. To generalize beyond this repo, we separate:
- the **target environment / application** (the context-stack: context-api/insert/read/search/trace + context-engine's own generated artifacts), from
- the **workflow tooling** (the memory-api domain stores, their transports, the viewer framework, and the guidance/skill), which becomes an installable dependency that any target project can consume.

## Locked decisions (interview 2026-07-25)
- GitHub owner: `mankinskin` (same as existing submodules).
- Granularity: **full per-tool split immediately** — each domain tool becomes its own repo bundling api + cli + mcp + http + viewer + vscode (where they exist) + its tool-scoped artifacts.
- Per-tool repo naming: **bare domain name** (ticket, spec, rule, doc, test, log, feedback, session, audit, peek, interview).
- Viewers bundle into their owning tool repo (memory-viewers is dissolved).
- Shared storage kernel is extracted and **renamed `memory-kernel`** (frees the `memory-api` name).
- Umbrella repo: **workflow-tools** aggregates the tool + shared repos as dependencies.
- Skill: dedicated **workflow-skill** repo published to skills.sh; installable at root and at each nested tool; single active install ignores nested installs and can self-uninstall.
- context-engine becomes a consuming example: keeps only context-stack + its own generated artifacts (.ticket/.spec/.rule/...); workflow tooling is an installed dependency.
- Artifacts: workflow-tools gets repo-level self-referential artifact stores now; per-tool artifact stores follow with the split.
- interview repo is pre-created as a placeholder even though the crate is not built yet.

## Repositories to create on GitHub (owner: mankinskin)
Shared libraries:
- `memory-kernel` — shared storage/index/search kernel (from memory-api/crates/memory-api)
- `memory-fixtures` — shared test support (memory-fixtures + memory-matrix)
- (`viewer-api` already exists — shared viewer framework + viewer-ctl; keep)

Per-tool repos (each bundles api + cli + mcp + http + viewer + vscode + tool artifacts as applicable):
- `ticket`, `spec`, `rule`, `doc`, `test`, `log`, `feedback`, `session`, `audit`, `peek`, `interview`

Umbrella + packaging:
- `workflow-tools` — aggregates all tool + shared repos as dependencies
- `workflow-skill` — skills.sh-installable entry point / guidance

Existing repos: `viewer-api` and `context-stack` are kept; `memory-api` name is freed (content split out); `memory-viewers` is dissolved into per-tool repos.

## Migration phases
- Phase 0 — Provisioning: user creates the GitHub repos above; agent waits for confirmation/links.
- Phase A — Shared foundations: extract `memory-kernel`; stabilize `viewer-api` + `memory-fixtures` as standalone deps.
- Phase B — Per-tool extraction: split each of the 11 domain tools into its own repo (api+transports+viewer+vscode+artifacts) using the cross-workspace move tooling for artifact integrity.
- Phase C — Umbrella + artifacts: create `workflow-tools`; add repo-level self-referential artifact stores; migrate tool-scoped artifacts into each tool repo.
- Phase D — Skill packaging: author `workflow-skill`; define scope/precedence for root + nested installs, single active install, self-uninstall.
- Phase E — context-engine reframing: reduce to context-stack + own artifacts; consume workflow-tools as installed dependency; update entry points across all three install sites.
- Phase F — Validation & cutover: end-to-end build/test/MCP/CLI/viewer + browser verification across split repos; migration guide.

## Cross-store prerequisite (recorded textually; graph edges cannot cross ticket stores)
Artifact moves must preserve reference integrity. The safe cross-workspace move tooling lives in the **memory-api** ticket store:
- memory-api `505b2cd4` "Deliver safe cross-workspace ticket move for git-backed stores" (+ children) — journaled, ref-relinking moves.
- This is a hard prerequisite for Phase B/C artifact migration and must be green before per-tool artifact moves begin.

## Related existing work to link (default store)
- `671d4e47` [architecture][multi-store] Tracker: cross-store interaction model and migration — internal contract/neutralization; complementary, must stay coordinated (linked).
- `13912e44` [architecture][memory-api] Neutral naming migration map — informs the memory-kernel rename (linked).
- `2b1279bd` [architecture][memory-api] Neutral storage kernel and API migration — kernel extraction dependency (linked).
- `b13c5d89` Epic: Agent Skill Foundation — skill contract feeds the workflow-skill packaging (linked).

## Acceptance criteria
- Complete dependency graph with every child ticket implementation-ready and validated.
- All new repos exist and build/test independently; workflow-tools aggregates them.
- context-engine builds using workflow-tools as an installed dependency, retaining only context-stack + its own artifacts.
- workflow-skill installs from skills.sh, wires the entry point at root and nested tools, and handles scope de-duplication + self-uninstall.
- Artifact moves preserve cross-reference integrity (validated), with no dangling references.
- Migration guide + install docs published; browser verification recorded for viewer-facing changes.