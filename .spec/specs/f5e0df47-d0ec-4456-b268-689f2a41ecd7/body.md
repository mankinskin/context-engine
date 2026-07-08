<!-- aligned-structure:v1 -->

# Summary

Define the contract for reducing model-bound Copilot context through upstream tool-result compression and workflow discipline, while using `session-api` captured artifacts only as diagnostic visibility into what prior sessions actually sent or accumulated.

## Behavior Story

Define the contract for reducing model-bound Copilot context through upstream tool-result compression and workflow discipline, while using `session-api` captured artifacts only as diagnostic visibility into what prior sessions actually sent or accumulated.

## Provided Surface Contracts

- Tool-result guards classify model-facing context into retain, summarize, reference-only, and drop-from-prompt before reuse.
- Bootstrap and handoff workflows prefer durable findings, validation outcomes, and ticket/spec references over raw transcript replay.
- Captured session artifacts remain diagnostic visibility only and are not treated as the mechanism that lowers the cost of the current request.

## Required Validation

- Executable validation: `./target/debug/rule.exe generate-target --config rule-targets.yaml --target context-engine-instruction-token-efficiency --check`
- Executable validation: `./target/debug/rule.exe generate-target --config rule-targets.yaml --target context-engine-instruction-session-optimization --check`
- Executable validation: `./target/debug/rule.exe sync-targets --config rule-targets.yaml --check`
- Contract clause validation: workflow guidance distinguishes upstream request shaping from post-hoc transcript capture.
- Contract clause validation: prompts and instruction surfaces discourage routine-action reasoning and oversized tool payload reuse.

## Related Implementation Tickets

- [.ticket/tickets/1c1ebfd1-4478-401f-a9ad-efcc2ff53b16/ticket.toml](.ticket/tickets/1c1ebfd1-4478-401f-a9ad-efcc2ff53b16/ticket.toml)
- [.ticket/tickets/47cc50db-8efa-4945-87fe-d30fe1f6bc61/ticket.toml](.ticket/tickets/47cc50db-8efa-4945-87fe-d30fe1f6bc61/ticket.toml)

## Background Knowledge References

- Prefer canonical rule-managed guidance over temporary design notes:
- [.agents/instructions/token-efficiency.instructions.md](.agents/instructions/token-efficiency.instructions.md)
- [.agents/instructions/session-optimization.instructions.md](.agents/instructions/session-optimization.instructions.md)
- [.agents/prompts/memory-setup.prompt.md](.agents/prompts/memory-setup.prompt.md)
- [.agents/prompts/handoff.prompt.md](.agents/prompts/handoff.prompt.md)
- [DESIGN_SESSION_BOOTSTRAPPING.md](DESIGN_SESSION_BOOTSTRAPPING.md) is exploratory background only, not an authoritative policy anchor.

## Legacy Content (Preserved)

# Goal
Define the contract for reducing model-bound Copilot context through upstream tool-result compression and workflow discipline, while using `session-api` captured artifacts only as diagnostic visibility into what prior sessions actually sent or accumulated.

# Problem
Captured Copilot transcripts show a large amount of low-value operational chatter: duplicated tool lifecycle events, repeated terminal orchestration, raw tool arguments, spill-file paths, empty or verbose `reasoningText`, routine-action narration, and repeated state re-checks. That chatter is expensive when it reaches high-cost models. Because `session-api` capture hooks run after token transmission, the real fix must happen upstream in tool wrappers, prompt guidance, and workflow policy.

# Scope
This spec covers upstream request-shaping policy and future prompt-facing compaction behavior for:
- tool-result compression and guarding before model reasoning
- routine-action discipline in workflow prompts and agent guidance
- bootstrap and handoff policies that prefer durable state over raw operational chatter
- diagnostic reading of:
- `.session/sessions/*/transcript.json`
- `.session/sessions/*/events.json`
- VS Code Copilot chat transcript JSONL files and associated chat-session resource artifacts
- bootstrap, handoff, and workflow prompts that consult those artifacts

# Non-goals
- assuming that transcript capture itself can reduce the cost of the current request
- changing the semantics of authoritative stored raw transcripts in this first phase
- deleting audit-grade raw event history
- building the full session pin/unpin runtime described in the broader bootstrapping design

# Acceptance Criteria
1. A repository policy explicitly distinguishes diagnostic transcript visibility from upstream request-shaping and states that cost reduction must happen before Copilot sends tokens.
2. Agent workflow guidance instructs agents to compress tool results, avoid routine-action reasoning, and avoid repeated unchanged state checks.
3. Bootstrap and handoff guidance instruct agents to prefer durable findings, ticket/spec references, validation outcomes, and artifact pointers over raw transcript replay.
4. The policy explicitly identifies high-confidence boilerplate that should not reach the LLM by default, including repeated tool lifecycle wrappers, raw `toolRequests`, empty or exploratory `reasoningText`, routine retry narration, repeated scope re-checks, and unbounded spill-file contents.
5. The follow-up implementation path is broken into coded facilities such as tool-result guards, prompt-facing compact state views, duplicate suppression, and artifact-pointer based context packing.
6. Traceability links the immediate guidance ticket and documents representative evidence from multiple captured sessions.

# Traceability

- Immediate guidance ticket: `.ticket/tickets/1c1ebfd1-4478-401f-a9ad-efcc2ff53b16`
- Follow-up implementation ticket: `.ticket/tickets/47cc50db-8efa-4945-87fe-d30fe1f6bc61`
- Canonical rule-managed guidance:
	- `.agents/instructions/token-efficiency.instructions.md`
	- `.agents/instructions/session-optimization.instructions.md`
	- `.rule/rules/4135e465-dc19-4966-892c-b232e062346b`
	- `.rule/rules/fe912923-78fc-4f59-b893-b4e6131d4937`
	- `.rule/rules/976d8f26-4664-479e-b1e4-6e198bba962d`
	- `.rule/rules/084fd4e6-660b-4227-a13e-514edf44e393`

# Representative Evidence

- Session `03a74288-df5d-4be1-beb3-252420f4d189`: tool lifecycle chatter and assistant-side tool payloads dominated the useful engineering state.
- Session `0f3721db-cf5e-4ad3-a939-1fa797dd1b67`: `run_in_terminal` and `read_file` repetition consumed more transcript volume than user intent.
- Session `b4096169-3e47-4180-a502-d6bdd366aabd`: repeated board and terminal orchestration created large prompt overhead.
- Session `38095e95-c056-478a-8fe4-2b0a80f34573`: repeated reads, searches, and status re-checks inflated context with limited durable value.

# Follow-up Implementation Slices

1. Add tool-result guards that classify outputs into retain, summarize, reference-only, and drop-from-prompt before they are reused in model-facing context.
2. Add duplicate suppression for repeated tool lifecycle wrappers, repeated state lookups, and routine retry narration.
3. Add artifact-pointer packing so large spill files and transcript resources stay out of prompt context by default.
4. Add prompt-facing helpers that emit compact bootstrap and handoff state records instead of raw transcript event streams.
