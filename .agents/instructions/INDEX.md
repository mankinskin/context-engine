# Instruction Index (agent-harness catalog)

Path- and description-scoped guidance lives in [.agents/instructions/](.agents/instructions/).
None of these files are auto-attached: every one is **description-gated**, so load
it with the Read tool only when its trigger matches the task at hand. Do not
preemptively read the whole set. Authoring conventions are in
[.agents/instructions/README.md](.agents/instructions/README.md).

When a file is loaded, treat its content as mandatory instructions for that task,
outranking generic defaults but yielding to [AGENTS.md](AGENTS.md) global rules per
the precedence table there.

## Catalog

| File | Use when |
| --- | --- |
| [audit/audit](.agents/instructions/audit/audit.instructions.md) | Editing or operating the audit tool: CLI/MCP usage, repo config, reading audit output. |
| [commit/generated-files](.agents/instructions/commit/generated-files.instructions.md) | Updating or verifying rule-system or repo-map generated artifacts. |
| [commit/message-conventions](.agents/instructions/commit/message-conventions.instructions.md) | Drafting or reviewing commit messages and multi-commit batches. |
| [commit/pre-commit](.agents/instructions/commit/pre-commit.instructions.md) | Troubleshooting or configuring the pre-commit hook. |
| [commit/submodule](.agents/instructions/commit/submodule.instructions.md) | Committing changes involving Git submodules or nested repos. |
| [commit/workflow](.agents/instructions/commit/workflow.instructions.md) | Running the repository commit workflow end to end. |
| [commit/worktree-workflow](.agents/instructions/commit/worktree-workflow.instructions.md) | Deciding whether a task needs worktree isolation; entry point/loop overview, naming, escalation triggers. |
| [commit/worktree-bootstrap](.agents/instructions/commit/worktree-bootstrap.instructions.md) | Creating or renaming a session worktree (worktree-ctl bootstrap/new, submodule population). |
| [commit/worktree-submodule-branch-check](.agents/instructions/commit/worktree-submodule-branch-check.instructions.md) | Verifying every submodule (recursively) is on the expected feature branch before editing. |
| [commit/worktree-claim](.agents/instructions/commit/worktree-claim.instructions.md) | Claiming a worktree-backed task with session_check_in/board_check_in before the first edit. |
| [commit/worktree-work](.agents/instructions/commit/worktree-work.instructions.md) | Working-directory discipline and entity-store targeting inside a worktree-backed task. |
| [commit/worktree-commit](.agents/instructions/commit/worktree-commit.instructions.md) | Committing changes inside a worktree-backed task. |
| [commit/worktree-rebase](.agents/instructions/commit/worktree-rebase.instructions.md) | Rebasing a worktree-backed feature branch onto updated main. |
| [commit/worktree-merge](.agents/instructions/commit/worktree-merge.instructions.md) | Marking a worktree-backed branch ready and merging it into main (bottom-up integration sequence). |
| [commit/worktree-submodules](.agents/instructions/commit/worktree-submodules.instructions.md) | A worktree-backed task touches a Git submodule. |
| [engine/context-http](.agents/instructions/engine/context-http.instructions.md) | Editing context-http: RPC dispatch, trace capture, state access, HTTP error mapping. |
| [engine/core-crates](.agents/instructions/engine/core-crates.instructions.md) | Editing context-engine core crates (trace/search/insert/read/api). |
| [frontend/frontend](.agents/instructions/frontend/frontend.instructions.md) | Editing frontend packages or generated TypeScript types. |
| [frontend/viewer-api-tools](.agents/instructions/frontend/viewer-api-tools.instructions.md) | Editing viewer-api-driven tools (viewer-api, log/doc/ticket viewers). |
| [orchestration/compact-output](.agents/instructions/orchestration/compact-output.instructions.md) | Choosing CLI output formats (TOON vs JSON) or applying the rtk proxy. |
| [orchestration/differential-patching](.agents/instructions/orchestration/differential-patching.instructions.md) | Editing files: surgical replacement over full-file rewrites. |
| [orchestration/dossier-merge](.agents/instructions/orchestration/dossier-merge.instructions.md) | Merging two or more completed prompt-ingestion dossiers into one. |
| [orchestration/duplication-consolidation](.agents/instructions/orchestration/duplication-consolidation.instructions.md) | Consolidating a completed duplication review's findings into authoritative snippets and reference-only replacements. |
| [orchestration/duplication-review](.agents/instructions/orchestration/duplication-review.instructions.md) | Running or resuming a structured duplication review of the instruction corpus. |
| [orchestration/escalation-gate](.agents/instructions/orchestration/escalation-gate.instructions.md) | A handoff package is incomplete or requirements are ambiguous. |
| [orchestration/fallback-escalation](.agents/instructions/orchestration/fallback-escalation.instructions.md) | Compact tooling is unavailable or insufficient. |
| [orchestration/file-inspection](.agents/instructions/orchestration/file-inspection.instructions.md) | Reading workspace files: bounded reads, peek CLI, repo_map.toon orientation. |
| [orchestration/loop-closure](.agents/instructions/orchestration/loop-closure.instructions.md) | Implementing, reviewing, or closing iteration work (Review/Interview/Commit/Handoff). |
| [orchestration/model-routing](.agents/instructions/orchestration/model-routing.instructions.md) | Deciding whether to delegate to a cheaper-model subagent. |
| [orchestration/orchestrator-delegation](.agents/instructions/orchestration/orchestrator-delegation.instructions.md) | Session start and throughout: cost gating and delegation contract. |
| [orchestration/phase-separation](.agents/instructions/orchestration/phase-separation.instructions.md) | Deciding whether to search, clarify, or implement. |
| [orchestration/preflight-validation](.agents/instructions/orchestration/preflight-validation.instructions.md) | Preflight write-hook failures or configuring write-time syntax validation. |
| [orchestration/roadmap-execution](.agents/instructions/orchestration/roadmap-execution.instructions.md) | Methodically executing a compiled ROADMAP.md from a prompt-ingestion dossier. |
| [orchestration/routine-actions](.agents/instructions/orchestration/routine-actions.instructions.md) | Deciding whether to narrate or just execute a routine operation. |
| [orchestration/session-artifacts](.agents/instructions/orchestration/session-artifacts.instructions.md) | Reading prior transcripts, handoff documents, or chat artifacts. |
| [orchestration/tool-output](.agents/instructions/orchestration/tool-output.instructions.md) | Handling tool output, command spills, or compact-terminal MCP. |
| [session/session-bootstrap](.agents/instructions/session/session-bootstrap.instructions.md) | Session start: discovering and pinning task-relevant guidance. |
| [session/session-optimization](.agents/instructions/session/session-optimization.instructions.md) | Optimizing context usage, tool-result compression, bootstrap quality. |
| [session/session-workflow](.agents/instructions/session/session-workflow.instructions.md) | Authoring or implementing durable session workflow graphs. |
| [spec/spec-system](.agents/instructions/spec/spec-system.instructions.md) | Creating or updating specs and their traceability links. |
| [testing/assertions](.agents/instructions/testing/assertions.instructions.md) | Writing test assertions or reviewing test quality. |
| [testing/benchmarks](.agents/instructions/testing/benchmarks.instructions.md) | Running Criterion benchmarks or adding performance measurements. |
| [testing/benchmarks-timeout](.agents/instructions/testing/benchmarks-timeout.instructions.md) | Before starting or while waiting on any cargo bench invocation. |
| [testing/benchmarks-criterion-calibration](.agents/instructions/testing/benchmarks-criterion-calibration.instructions.md) | A Criterion benchmark group covers a heterogeneous scenario matrix. |
| [testing/http-stress](.agents/instructions/testing/http-stress.instructions.md) | Running HTTP stress tests or concurrency sweeps. |
| [testing/test-debugging](.agents/instructions/testing/test-debugging.instructions.md) | Test failures, tracing setup, and test-log debugging. |
| [testing/test-execution](.agents/instructions/testing/test-execution.instructions.md) | Running tests, choosing scope, executing validation commands. |
| [testing/validation-evidence](.agents/instructions/testing/validation-evidence.instructions.md) | Recording validation specs/executions in the test-api store. |
| [ticket/board](.agents/instructions/ticket/board.instructions.md) | Coordinating multi-agent work on the draftboard: check-in/out, WIP, file ownership. |
| [ticket/engine](.agents/instructions/ticket/engine.instructions.md) | Editing ticket-system crates (API, storage/index, transport, viewer). |
| [ticket/lifecycle](.agents/instructions/ticket/lifecycle.instructions.md) | Moving a ticket through its states, undo/revert, review gate. |
| [ticket/workflow](.agents/instructions/ticket/workflow.instructions.md) | Cross-session ticket operations: orientation, discovery, picking next work. |
| [transcripts/audio-transcript](.agents/instructions/transcripts/audio-transcript.instructions.md) | Turning a raw audio transcript into a clean markdown prompt or document. |

Regenerate this table when instruction files are added, removed, or re-described.
