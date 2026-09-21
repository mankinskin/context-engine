## Problem

The presentation epic (`0ee95228`) establishes Slidev as the rendering engine and a `.presentation/` deck store (`deck.toml` + `slides.md`), but its Phase 1 (`89b0c64a`) scope is a single synthetic sample deck in one location (`memory-viewers/presentation-viewer/frontend/slidev/`). It does not address **per-repository composable entry points**: every repository/submodule in the superproject tree (context-stack, memory-api, memory-viewers, viewer-api, workflow-tools and its nested domains audit/test/doc/log/feedback/peek/rule/session/spec/ticket/interview/memory-kernel/contract-reference) should each expose its own presentational entry point, independently buildable, that a super-repository's deck can compose into its own build.

## Decisions (from user interview)

- Composition mechanism: **hybrid**. Each repo's `.presentation/` produces a standalone, independently buildable static Slidev deck (own `package.json`/Vite toolchain). A super-repo's deck additionally imports the sub-repo's `slides.md` source directly (Slidev's per-slide `src:` frontmatter include) so a combined top-level build can inline sub-repo content without iframing, while the sub-repo deck still works standalone.
- Entry point location: `.presentation/` at the root of every repo (matches the epic's existing `deck.toml` + `slides.md` convention).
- Rendering engine: Slidev (confirmed).
- Scope for this pass: toolchain + one real, working composition example — not a full 15-repo rollout. Build the Slidev toolchain scaffold once (reusable per-repo pattern, stock theme per Phase 1 ticket `89b0c64a`), author `context-engine`'s root deck (an overview walking through every repository) and `workflow-tools`'s deck (walking through its nested domains) as the proof pair: `context-engine`'s deck composes `workflow-tools`'s `slides.md` via `src:` include, and `workflow-tools`'s deck builds standalone too.

## Acceptance Criteria

- AC1: A reusable `.presentation/` Slidev toolchain scaffold (package.json, vite config, stock theme, npm `dev`/`build` scripts) exists and is documented so it can be replicated into any other repo.
- AC2: `context-engine/.presentation/` builds a static deck introducing every repository in the tree (context-stack, memory-api, memory-kernel-via-workflow-tools, memory-viewers, viewer-api, workflow-tools).
- AC3: `workflow-tools/.presentation/` builds a static deck introducing its nested domain repos, and builds standalone independent of the root deck.
- AC4: `context-engine`'s deck imports at least the `workflow-tools` deck's `slides.md` via Slidev's `src:` include and the combined build renders those slides inline (proving the composition mechanism).
- AC5: Both decks pass a Playwright smoke check (loads, first-slide screenshot) per AGENTS.md browser-verification rules.
- AC6: Remaining repos are explicitly out of scope for this ticket; a follow-up ticket tracks the full rollout.
