# Agent Skill Infrastructure

Governing spec for the agent skill base that unblocks the critical path to context-graph ingestion:
ingestion → debug/inspect tooling → context-graph + log UI → UI dev/test/review loop → skills.

## Problem

- `.agents/skills/` contains only a vendored `find-skills/` and an orphaned `token-optimized-agentic-engineering.SKILL.md`; zero craft skills exist.
- All 12 files in `.agents/instructions/` are `rule-api:file generated=true`, coupling agent guidance to the rule-store generator we want to leave.
- No spec governs agent-skill infrastructure; fixture specs (`fixture/root`, `fixture/submodule-a`, `fixture/submodule-b`) and ultra-granular http/error/arg stubs pollute the store.
- The only "Skill System" ticket is a shader/spell feature, not agent skills.

## Decisions (interview batch 1 — user-confirmed)

1. Buy-first: adopt proven skills.sh skills; hand-author only where the ecosystem is thin (Dioxus).
2. Full migration now: convert all 12 `.agents/instructions/*.md` files to hand-owned by-description files and retire the rule-targets generator for agent guidance (instructions + skills). Rule-store may remain for non-agent-guidance surfaces if any.
3. Bold spec prune: delete test-fixture specs and contentless stubs, AND consolidate ultra-granular doorknob specs (http/error/arg-level) into their parents. Executed via a tracked, reviewable ticket — never during a planning pass.
4. Single governing spec (this one) defines the skill directory contract, the by-description loading model, and the migration path.
5. First materialized slice = skill foundation (this spec + adopt proven skills + author Dioxus).

## Skill Directory Contract

- One folder per skill: `.agents/skills/<skill-name>/SKILL.md` (retire the root `NAME.SKILL.md` scheme).
- `SKILL.md` frontmatter: `name`, `description` (by-description loading trigger), optional `applyTo`.
- Skills are hand-owned, not `rule-api:file generated=true`. Generated artifacts must not live in the skills tree.
- A master index lists skills with their description triggers.

## By-Description Loading Model

- Agent guidance is loaded on demand by matching a task to a skill/instruction `description`, not by rendering monolithic rule-store files into fixed instruction files.
- All instruction files migrate from generated blobs to fine-grained hand-owned `*.instructions.md` with accurate `applyTo`/`description`; the generator stops overwriting them.

## Domain Coverage Plan

| Domain | Source | Action |
|---|---|---|
| Rust async/best-practices | skills.sh (16K / 14K installs) | adopt |
| Browser/Playwright automation | skills.sh (97K / 64K installs) | adopt |
| WebGPU / 3D / TSL | skills.sh (51K installs) | adopt |
| User interviewing/review | skills.sh (~1.8K) | adopt |
| Skill authoring | skills.sh (70K) | adopt |
| Dioxus | thin (~71 installs) | hand-author in-repo |

## Acceptance Criteria

- AC1: This spec exists and is linked from the skill-foundation epic and its child tickets.
- AC2: Skill directory contract implemented; orphaned root skill re-homed into a folder; index present.
- AC3: Proven skills.sh skills adopted and normalized into the contract; presence verifiable by folder + index entry.
- AC4: Dioxus skill hand-authored with description trigger and at least one worked example.
- AC5: ALL 12 instruction files migrated off `rule-api:file generated=true` to hand-owned by-description files; rule-targets no longer regenerates agent guidance.
- AC6: Fixture/empty-stub specs pruned and ultra-granular specs consolidated via a reviewed ticket; real system specs untouched.
- AC7: Prompt-replay/validation confirms an agent can locate and load the right skill by description for each target domain.

## Traceability

- Tickets: skill-foundation epic + children (created alongside this spec).
- Related specs: to be linked as migration and prune tickets resolve.
- Validation: prompt-replay matrix for skill discovery by description.