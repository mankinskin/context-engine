# Objective

Implement the agent world-model roadmap in the canonical repository surfaces. Add a named narrative entry point, repair stale root-guidance references, make lifecycle ownership explicit through cross-references, and preserve the distinction between tool use and tool improvement.

# Scope

- Add the repository-level narrative and reading-order entry point under the canonical guidance tree.
- Update `context-engine/AGENTS.md` so retained-root references resolve to `workflow-tools/.agents` and the root file keeps only repository-wide operating principles.
- Apply the ownership resolutions from `transcripts/04-09-2026_agent-code-world-model/03-guidance-ownership-and-rewrite.md` without rewriting unrelated guidance.
- Add focused validation for the narrative, links, and spec/ticket traceability.

# Acceptance Criteria

1. `workflow-tools/.agents` remains the only canonical guidance root; no guidance tree is moved.
2. A single narrative entry point indexes chapters for arrival, map, exploration, clarification, decision, execution, validation/return, tool improvement, and testing.
3. Every lifecycle transition has one canonical owner and duplicate rules are replaced by links or narrowly scoped pointers.
4. The four stale relative references in `context-engine/AGENTS.md` are repaired or retired.
5. Tool use and tool improvement are explicitly distinguished with repository-backed examples.
6. Relevant documentation, spec health, and link checks pass, with historical baseline debt reported separately from regressions.

# Validation

- `git diff --check`
- `workflow-tools/target/debug/spec.exe health workflow/agent-world-model --workspace context-engine/.spec --json`
- `workflow-tools/target/debug/ticket.exe health <ticket-id> --workspace context-engine/.ticket --json`
- `rg -n "\.agents/instructions/(session/session-identity-and-handoff|orchestration/(loop-closure|core-cycle|escalation-gate))" context-engine/AGENTS.md`
- `bash workflow-tools/tools/validate-instruction-links.sh --manifest transcripts/30-08-2026_spec-system-improvement-planning/instruction-distribution.md --baseline transcripts/30-08-2026_spec-system-improvement-planning/instruction-link-baseline.txt`
