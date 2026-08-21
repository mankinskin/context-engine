# Duplicate Passages — anchor slice [1,1] (`AGENTS.md`)

One row per marked-section finding returned by a batch worker. `no overlap` rows have no excerpt.

| Pair | Anchor lines | Target file | Target lines | Classification | Excerpt |
|---|---|---|---|---|---|
| 1 | 7-15 | agents/audit.agent.md | 17-21 | thematic overlap | |
| 2 | — | agents/brainstorm.agent.md | — | no overlap | |
| 3 | — | agents/bug-report.agent.md | — | no overlap | |
| 4 | 88-96 | agents/cleanup.agent.md | 33-36 | thematic overlap | |
| 5 | 45-46 | agents/code-architect.agent.md | 32-34 | thematic overlap | |
| 6 | — | agents/commit.agent.md | — | no overlap | |
| 7 | 21-31 | agents/context-enrichment.agent.md | 21-24 | thematic overlap | |
| 8 | 5-17 | agents/deduplication-campaign.agent.md | 12-16 | thematic overlap | |
| 9 | 5-17 | agents/duplication-batch-worker.agent.md | 21-29 | thematic overlap | |
| 10 | 5-17 | agents/duplication-cleanup.agent.md | 17-17 | thematic overlap | |
| 11 | 49-56 | agents/duplication-consolidation.agent.md | 12-15 | thematic overlap | |
| 12 | 39-47 | agents/duplication-review.agent.md | 34-37 | thematic overlap | |
| 13 | 16-17 | agents/explainer.agent.md | 38-44 | thematic overlap | |
| 14 | 21-31 | agents/explore.agent.md | 13-21 | thematic overlap | |
| 15 | 5-15 | agents/framing.agent.md | 36-39 | thematic overlap | |
| 16 | 29-36 | agents/guidance-lifecycle.agent.md | 20-29 | thematic overlap | |
| 17 | 48-50 | agents/handoff.agent.md | 15-19 | thematic overlap | |
| 18 | 35-36 | agents/implement.agent.md | 70-90 | thematic overlap | |
| 19 | 52-56 | agents/installer.agent.md | 1-10 | thematic overlap | |
| 20 | 61-66 | agents/interview.agent.md | 1-20 | thematic overlap | |
| 21 | 48-48 | agents/iteration.agent.md | 1-10 | thematic overlap | |
| 22 | 51-77 | agents/live-validation.agent.md | 24-40 | thematic overlap | |
| 23 | 45-47 | agents/merge.agent.md | 31-35 | thematic overlap | |
| 24 | 88-92 | agents/mission-planning.agent.md | 24-27 | thematic overlap | |
| 25 | 19-25 | agents/online-research.agent.md | 8-12 | thematic overlap | |
| 26 | 120-127 | agents/orchestrator.agent.md | 154-158 | near-duplicate | "repo-root-relative, forward-slash, verified to exist" |
| 27 | 42-49 | agents/refactoring.agent.md | 24-34 | thematic overlap | |
| 28 | 19-23 | agents/research.agent.md | 15-20 | thematic overlap | |
| 29 | 15 | agents/review.agent.md | 48-61 | near-duplicate | "Declare the session identity and assigned worktree at the start of each session, then repeat both in the final response; follow session-identity-and-handoff.instructions.md." |
| 30 | 18-26 | agents/roast.agent.md | 31-36 | thematic overlap | |
| 31 | 31-38 | agents/scoping.agent.md | 27-35 | thematic overlap | |
| 32 | 15 | agents/session-bootstrap.agent.md | 28-35 | near-duplicate | "Declare the session identity and assigned worktree at the start of each session, then repeat both in the final response; follow session-identity-and-handoff.instructions.md." |
| 33 | 51-56 | agents/session-learning.agent.md | 33-34 | thematic overlap | |
| 34 | 5 | agents/simplify.agent.md | 55 | near-duplicate | "Gather context before coding. Do not guess." |
| 35 | 8-9 | agents/spec.agent.md | 23-24 | near-duplicate | "For new or changed requirements and goals, create or update the relevant spec before implementation proceeds." / "Prefer updating an existing matching spec over creating a near-duplicate." |
| 36 | 4-15 | agents/structured-research.agent.md | 16-18 | thematic overlap | |
| 37 | 40-46 | agents/surface-design.agent.md | 29-35 | thematic overlap | |
| 38 | 4-15 | agents/teacher.agent.md | 30-40 | thematic overlap | |
| 39 | 40-46 | agents/testing.agent.md | 11-25 | thematic overlap | |
| 40 | 31-38 | agents/ticket-refinement.agent.md | 34-48 | thematic overlap | |
| 41 | 48-51 | agents/transcription.agent.md | 83-90 | thematic overlap | |
| 42 | 14-15 | agents/writing.agent.md | 11-20 | thematic overlap | |
| 43 | — | instructions/audit/audit.instructions.md | — | no overlap | |
| 44 | 28-31 | instructions/commit/branch-worktree.instructions.md | 11-13 | near-duplicate | "Use this protocol for changes spanning multiple files or components, submodules, active concurrent work, or risky behavior changes. A small, self-contained change to one existing file or the addition of one new file may be made in the main checkout after checking that no active board entry owns the path." |
| 44 | 49 | instructions/commit/branch-worktree.instructions.md | 171-176 | near-duplicate | "Worktree-backed commits land on the feature branch inside the worktree." |
| 45 | — | instructions/commit/cross-repo-dependencies.instructions.md | — | no overlap | |
| 46 | 1-2 | instructions/commit/generated-files.instructions.md | 14 | thematic overlap | |
| 47 | 10 | instructions/commit/message-conventions.instructions.md | 5-9 | thematic overlap | |
| 48 | 10 | instructions/commit/pre-commit.instructions.md | 8-17 | thematic overlap | |
| 49 | 33-35 | instructions/commit/submodule.instructions.md | 1-3 | thematic overlap | |
| 50 | 29-34 | instructions/commit/workflow.instructions.md | 5 | near-duplicate | "A small, self-contained main-checkout change may commit on main after checking the board, staging only its changed path, and running focused validation." |
| 50 | 50 | instructions/commit/workflow.instructions.md | 23 | near-duplicate | "Rebase the feature branch onto updated main in every affected repository: each affected submodule first, then the superproject." |
| 51 | 37-45 | instructions/engine/context-http.instructions.md | 41-45 | thematic overlap | |
| 52 | 5-6 | instructions/engine/core-crates.instructions.md | 26 | near-duplicate | "Read existing tests for expected behavior." |
| 52 | 13 | instructions/engine/core-crates.instructions.md | 37 | near-duplicate | "Use target/test-logs/ for full debug output when tests fail." |
| 53 | — | instructions/engine/kernel-layering.instructions.md | — | no overlap | |
| 54 | 28-33 | instructions/engine/workflow-tool-extraction.instructions.md | 13 | near-duplicate | "Do not extract the test or log tool until the relevant ticket or spec records a remediation decision." |
| 55 | 40-43 | instructions/frontend/frontend.instructions.md | 36-39 | near-duplicate | "Browser end-to-end checks where available. For browser-hosted frontend code, first try the MCP Playwright/browser tools; if they are unavailable or insufficient for the scenario, fall back to repo-local Playwright flows." |
| 56 | 40-43 | instructions/frontend/viewer-api-tools.instructions.md | 84-86 | near-duplicate | "Browser checks: run at least one browser flow that exercises changed UX paths in an external fullscreen Chromium-family browser rather than VS Code's integrated browser." |
| 57 | 28-36 | instructions/orchestration/code-quality.instructions.md | 16-19 | thematic overlap | |
| 58 | 67-68 | instructions/orchestration/compact-output.instructions.md | 6-8 | thematic overlap | |
| 59 | 67-68 | instructions/orchestration/differential-patching.instructions.md | 6-13 | thematic overlap | |
| 60 | 70-76 | instructions/orchestration/duplication-consolidation.instructions.md | 28 | thematic overlap | |
| 61 | 70-76 | instructions/orchestration/duplication-review.instructions.md | 126 | thematic overlap | |
| 62 | 70-76 | instructions/orchestration/entity-disambiguation.instructions.md | 8-13 | thematic overlap | |
| 63 | 58-65 | instructions/orchestration/escalation-gate.instructions.md | 6-15 | near-duplicate | "If blocked by ambiguity after focused research (10-15 minutes), ask the user." |
| 64 | 17-26 | instructions/orchestration/fallback-escalation.instructions.md | 6-11 | thematic overlap | |
| 65 | 17-26 | instructions/orchestration/file-inspection.instructions.md | 23-31 | thematic overlap | |
| 65 | 67-68 | instructions/orchestration/file-inspection.instructions.md | 34-42 | thematic overlap | |
| 66 | 58-66 | instructions/orchestration/intent-refinement.instructions.md | 28-31 | near-duplicate | "If blocked by ambiguity after focused research (10-15 minutes), ask the user." |
| 66 | 47 | instructions/orchestration/intent-refinement.instructions.md | 16-26 | thematic overlap | |
| 67 | 47-49 | instructions/orchestration/loop-closure.instructions.md | 6-16 | near-duplicate | "Follow the closed-loop iteration workflow: Review→Interview→Commit→Handoff." |
| 68 | 70-76 | instructions/orchestration/mermaid-graph-rendering.instructions.md | 6-8 | thematic overlap | |
| 69 | 67-68 | instructions/orchestration/model-prices.instructions.md | 6-12 | thematic overlap | |
| 70 | 67-68 | instructions/orchestration/model-routing.instructions.md | 6-20 | thematic overlap | |
| 71 | 31-52 | instructions/orchestration/orchestrator-delegation.instructions.md | (unspecified) | thematic overlap | |
| 72 | 31-52 | instructions/orchestration/phase-separation.instructions.md | (unspecified) | thematic overlap | |
| 73 | 31-52 | instructions/orchestration/pre-dispatch-gates.instructions.md | (unspecified) | thematic overlap | |
| 74 | — | instructions/orchestration/preflight-validation.instructions.md | — | no overlap | |
| 75 | 31-52 | instructions/orchestration/prompt-ingestion.instructions.md | (unspecified) | thematic overlap | |
| 76 | 61-66 | instructions/orchestration/question-quality.instructions.md | (unspecified) | thematic overlap | |
| 77 | 61-66 | instructions/orchestration/retry-limit.instructions.md | (unspecified) | thematic overlap | |
| 78 | 4-15 | instructions/orchestration/routine-actions.instructions.md | 6-19 | thematic overlap | |
| 79 | 17-29 | instructions/orchestration/session-artifacts.instructions.md | 6-15 | thematic overlap | |
| 80 | 15 | instructions/orchestration/shared-context-bundle.instructions.md | 34-43 | thematic overlap | |
| 81 | 62-66 | instructions/orchestration/subagent-return-contract.instructions.md | 18-29 | thematic overlap | |
| 82 | 13 | instructions/orchestration/tool-output.instructions.md | 24-28 | thematic overlap | |
| 83 | 31-38 | instructions/orchestration/write-and-die.instructions.md | 6-12 | thematic overlap | |
| 84 | 15 | instructions/session/session-bootstrap.instructions.md | 1-6 | thematic overlap | |
| 85 | 15 | instructions/session/session-identity-and-handoff.instructions.md | 31-39 | near-duplicate | "Declare the session identity and assigned worktree at the start of each session, then repeat both in the final response; follow session-identity-and-handoff.instructions.md." |
| 85 | 37 | instructions/session/session-identity-and-handoff.instructions.md | 57-64 | thematic overlap | |
| 86 | 74-75 | instructions/session/session-optimization.instructions.md | 29-36 | near-duplicate | "See token-efficient workflow guidance in .agents/instructions/orchestration/ covering compact output, bounded file inspection, tool-result handling, differential patching, and model-cost-aware routing." |
| 87 | 31-38 | instructions/session/session-workflow.instructions.md | 10-19, 31-36 | thematic overlap | |
| 88 | 37 | instructions/session/worktree-provisioning.instructions.md | 1-9 | near-duplicate | "Worktree-backed work is required for changes spanning multiple files or components, submodules, active concurrent work, or risky behavior changes." |
| 88 | 54 | instructions/session/worktree-provisioning.instructions.md | 148-156 | thematic overlap | |
| 89 | 8 | instructions/spec/spec-system.instructions.md | 20-36 | thematic overlap | |
| 90 | 5-6 | instructions/testing/assertions.instructions.md | 3-8 | thematic overlap | |
| 91 | 40-42 | instructions/testing/benchmarks.instructions.md | 1-12 | thematic overlap | |
| 92 | 9 | instructions/testing/data-capture-verification.instructions.md | 24-27 | thematic overlap | |
| 93 | 11-12 | instructions/testing/http-stress.instructions.md | 20-27 | thematic overlap | |
| 94 | 40-44 | instructions/testing/split-responsibility-testing.instructions.md | 19-25 | thematic overlap | |
| 95 | 13 | instructions/testing/test-debugging.instructions.md | 19 | near-duplicate | "Read test logs in target/test-logs/ for debugging instead of relying on truncated test stdout." |
| 96 | 17-26 | instructions/testing/test-execution.instructions.md | 21-27 | thematic overlap | |
| 97 | 51-56 | instructions/testing/validation-evidence.instructions.md | 16-33 | thematic overlap | |
| 98 | 21-23 | instructions/ticket/board.instructions.md | 12-20 | thematic overlap | |
| 99 | 17-26 | instructions/ticket/engine.instructions.md | 20-42 | thematic overlap | |
| 100 | 37-49 | instructions/ticket/lifecycle.instructions.md | 128-141 | thematic overlap | |
| 101 | 70-76 | instructions/ticket/workflow.instructions.md | 242 | thematic overlap | |
| 102 | — | instructions/transcripts/audio-transcript.instructions.md | — | no overlap | |
| 103 | 37-49 | prompts/audit.prompt.md | 20 | thematic overlap | |
| 104 | 42-46 | prompts/build-validate-tools.prompt.md | 128-209 | thematic overlap | |
| 105 | 10 | prompts/commit.prompt.md | 26 | thematic overlap | |
| 106 | 13 | prompts/debug-test.prompt.md | 16 | near-duplicate | "Read the relevant file in target/test-logs/." |
| 106 | 6 | prompts/debug-test.prompt.md | 21 | near-duplicate | "Read crate docs and nearby tests before changing code." |
| 106 | 14 | prompts/debug-test.prompt.md | 27-28 | near-duplicate | "Change only what is needed for the failing contract." |
| 106 | 20 | prompts/debug-test.prompt.md | 20 | near-duplicate | "Check existing tickets for similar symptoms or known limitations." |
| 107 | 70-76 | prompts/deduplication-campaign.prompt.md | 16 | thematic overlap | |
| 107 | 10 | prompts/deduplication-campaign.prompt.md | 29 | near-duplicate | "reminder that committing is Commit Agent's job" |
| 108 | 10 | prompts/duplication-cleanup.prompt.md | 27 | near-duplicate | "reminder that committing is Commit Agent's job" |
| 109 | 10 | prompts/duplication-consolidation.prompt.md | 22 | near-duplicate | "Do not commit — that stays with Commit Agent." |
| 110 | 70-76 | prompts/duplication-review.prompt.md | 22 | thematic overlap | |
| 111 | 15 | prompts/epic-db6980d1.prompt.md | 218-231 | near-duplicate | ticket schema field list overlap (objective/acceptance_criteria/owner_paths/etc.) |
| 112 | 70-76 | prompts/handoff-tickets.prompt.md | 47 | near-duplicate | "render all entity references per the Clickable Reference Policy in AGENTS.md" |
| 113 | 76-96 | prompts/handoff.prompt.md | 52-59 | thematic overlap | |
| 114 | 10-11 | prompts/implement.prompt.md | 12-14 | thematic overlap | |
| 115 | 17-29 | prompts/interview.prompt.md | 16-21 | thematic overlap | |
| 116 | 51 | prompts/iteration.prompt.md | 10 | near-duplicate | "Review → Interview → Commit → Handoff" |
| 117 | 17-26 | prompts/memory-setup.prompt.md | 20 | thematic overlap | |
| 118 | 31-38 | prompts/next.prompt.md | 10-18 | thematic overlap | |
| 119 | — | prompts/refine-ingest.prompt.md | — | no overlap | |
| 120 | 24-26 | prompts/research.prompt.md | 15-17 | near-duplicate | "Read crate-level README.md and HIGH_LEVEL_GUIDE.md for known gotchas." |
| 120 | 24-26 | prompts/research.prompt.md | 22-24 | near-duplicate | "Read the implementation and adjacent tests to infer expected behavior." |
| 121 | 70-76 | prompts/reviews.prompt.md | 86 | thematic overlap | |
| 122 | 70-76 | prompts/spec.prompt.md | 23 | thematic overlap | |
| 122 | 59 | prompts/spec.prompt.md | 25 | near-duplicate | "If required details are still ambiguous after a focused search, ask one concise clarification rather than guessing." |
| 123 | — | prompts/sync-model-prices.prompt.md | — | no overlap | |
| 124 | 5-13 | prompts/tdd.prompt.md | 16-23 | thematic overlap | |
| 125 | 5 | prompts/ticket-next.prompt.md | 19 | thematic overlap | |
| 126 | 75 | prompts/ticket.prompt.md | 11 | near-duplicate | "Extract the exact canonical ticket folder path from ticket-api output. If the first create or match response does not include the folder path, run an immediate follow-up ticket-api command that returns the authoritative path." |
| 126 | 59 | prompts/ticket.prompt.md | 13 | near-duplicate | "Ask one concise clarification if the target store, scope, or ticket shape is still ambiguous after a focused search." |
| 127 | 8 | prompts/tickets.prompt.md | 30 | near-duplicate | "For new or changed requirements and goals, create or update the relevant spec before implementation proceeds." |
| 127 | 61-64 | prompts/tickets.prompt.md | 33 | near-duplicate | Clickable Reference Policy section duplicated |
| 128 | — | prompts/tool-grant-regression-probe.prompt.md | — | no overlap | |
| 129 | — | prompts/transform-transcript.prompt.md | — | no overlap | |
| 130 | 20 | prompts/user-training.prompt.md | 18 | near-duplicate | "Known issues/plans: use ticket-mcp tools before duplicating work." |
| 130 | 30 | prompts/user-training.prompt.md | 24 | near-duplicate | "Small, self-contained change (no ticket needed): may be made in the main checkout without a ticket or spec update." |
| 131 | — | skills/customer-interviews/SKILL.md | — | no overlap | |
| 132 | 40 | skills/dioxus/SKILL.md | 100 | near-duplicate | "Browser verification is mandatory for any change to a server interface or frontend feature: open the affected viewer in an external fullscreen Chromium-family browser and confirm the feature works visually before marking work done." |
| 133 | — | skills/doc-coauthoring/SKILL.md | — | no overlap | |
| 134 | — | skills/find-skills/SKILL.md | — | no overlap | |
| 135 | 40-41 | skills/playwright-best-practices/SKILL.md | 12 | thematic overlap | |
| 136 | 40-41 | skills/playwright-cli/SKILL.md | 3 | thematic overlap | |
| 137 | — | skills/rust-async-patterns/SKILL.md | — | no overlap | |
| 138 | — | skills/rust-best-practices/SKILL.md | — | no overlap | |
| 139 | 58-59 | skills/token-optimized-agentic-engineering/SKILL.md | 12-14 | thematic overlap | |
| 140 | — | skills/typegpu/SKILL.md | — | no overlap | |
| 141 | — | skills/webgpu-threejs-tsl/SKILL.md | — | no overlap | |
