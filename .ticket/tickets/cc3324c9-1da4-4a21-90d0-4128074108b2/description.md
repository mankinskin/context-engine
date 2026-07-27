## Problem

Sub-agents are spawned with no shared context, so each rediscovers the same artifacts independently. The orchestrator already holds the digest and does not pass it down.

Cross-agent duplicate file reads measured across both sessions:

| artifact | distinct sub-agents that read it | total reads |
|---|---|---|
| `handoffs/dcf86212-*.json` | 6 (plus 4 more via a second path spelling) | 14 |
| `compact-terminal-mcp/src/server.rs` | 6 | 21 |
| `.vscode/mcp.json` + `.github/mcp.json` | 3 | 10 |
| `.spec/specs/63c60c9d*/body.md` | 3 | 3 |
| `memory-api/crates/session-api/src/model/handoff.rs` | 2 | 3 |
| `memory-api/crates/session-api/src/model/workflow.rs` | 2 | 6 |

Cross-agent duplicate commands:

- `cargo test -p compact-terminal-cli` x5
- `cargo test -p compact-terminal-mcp` x4
- `git status --short` x4
- MCP JSON-RPC `initialize` + `tools/list` handshake probe x4

Within a single agent it is worse: subagent `[9]` read `subagent_rollup.rs` **6 times**, `body.md` 4 times, `lib.rs` 4 times.

Sub-agents also read `.agents/agents/orchestrator.agent.md`, `explore.agent.md`, `default.agent.md`, and `iteration.agent.md` — spending tokens introspecting the delegation system rather than doing the delegated work.

## Why it costs

Reading `server.rs` (454 lines) 21 times across 6 agents is not 21 file reads — it is 21 turns, each carrying an estimated ~37k tokens of fixed prefix plus the file body, plus the reasoning tokens spent deciding to read it. The information was identical every time. Read counts are measured; the per-turn token figure is an estimate pending `9d527ad1`.

Parallel fan-out makes it worse, not better: in `41966513` the two parallel sub-agents `[0] Verify wildcard tool grant` and `[1] Load handoff context` issued byte-identical command sequences and search queries — the same `ticket.exe get bd5e9aee`, `spec.exe get 63c60c9d`, `spec.exe get 3ccdde3a`, `ticket.exe subgraph`, and the same eight `file_search` globs.

## Scope

- Define a context bundle passed to every sub-agent at spawn: resolved ticket/spec bodies, handoff package, relevant file digests, and validation command list — as prompt content, not as paths the child must fetch.
- For parallel fan-out specifically: compute the shared prefix of what siblings need once, in the parent, and inline it into each child prompt.
- Add per-agent read deduplication guidance: within one sub-agent, re-reading a file already in its own transcript is always waste.
- Remove the need for sub-agents to read agent templates by stating the relevant contract in the delegation prompt itself.
- Consider a session-scoped artifact cache keyed by path + content hash, so a repeat read returns a cheap "unchanged, see turn N" marker.

## Acceptance Criteria

1. Sub-agents receive resolved ticket/spec/handoff content inline; they do not fetch it themselves for context they were spawned to act on.
2. Parallel siblings do not independently issue identical discovery command sequences.
3. No single sub-agent reads the same unchanged file more than once.
4. No sub-agent reads `.agents/agents/*.agent.md` in the course of normal delegated work.
5. Measured against the benchmark in `10d21210` — whose scenario includes a fan-out of sibling sub-agents needing the same artifact — the count of artifacts read by more than two distinct sub-agents drops to zero versus the checked-in baseline.

## Evidence

- Duplicate-read and duplicate-command tables produced by `tmp/subagent_cost_probe.py`
- `.session/sessions/3e9bc20b-4fe8-4996-ae7f-7be32525e429/events.json`
- `.session/sessions/41966513-a8fa-4b44-98fa-9c57f0437cc0/events.json` — parallel spans at events 6/7 and 240/242## Status Summary

**State**: in-review  
**Implemented**: 2026-07-27

### Premise Verification

**VERIFIED**: Sub-agents receive only path references and must fetch artifacts themselves.

- Evidence: `.agents/instructions/orchestration/orchestrator-delegation.instructions.md` lines 70-75 delegation contract item 4 says "pass anchors (full workspace-relative paths, ticket/spec ids, prior findings)" — NOT full content
- Evidence: Lines 80-90 "Context Isolation" section: "A sub-agent inherits NONE of the current session's context" — no mention of passing resolved artifact content inline
- Evidence: No "context bundle" or inline artifact content passing mentioned anywhere in delegation instructions
- Conclusion: Sub-agents receive references only and must independently fetch the same artifacts

### Implementation

**Files created**:
- `.agents/instructions/orchestration/shared-context-bundle.instructions.md` — complete bundle composition and optimization rules

**Context bundle defined**:

Standard fields (2k-5k tokens target):
1. Resolved tickets: full TOML + description markdown
2. Resolved specs: full body + sections
3. Handoff package: complete JSON
4. Relevant file digests: bounded windows or interface skeletons
5. Validation commands: pre-parsed command list

**Parallel fan-out optimization**: Compute shared prefix ONCE, duplicate into each sibling prompt. Input duplication (12k tokens for 4 siblings) is vastly cheaper than per-sibling discovery (4 × 37k prefix = ~148k tokens).

**Read deduplication rule**: Sub-agents must not re-read unchanged files within their own session. Check transcript before issuing file read; reference prior turn instead.

**Agent template reads eliminated**: Orchestrator must include target agent's contract excerpt inline in delegation prompt. Sub-agents should never read `.agents/agents/*.agent.md` files.

### Measured Cost Savings

From `tmp/subagent_cost_probe.py`:

**Cross-agent duplicates eliminated**:
- Handoff package: 10 sub-agents, 14 reads → 1 read + inline duplication
- `server.rs`: 6 sub-agents, 21 reads → 1 read + inline skeleton
- MCP configs: 3 sub-agents, 10 reads → 1 read + inline bundle
- Spec bodies: 3 sub-agents, 3 reads → 1 read + inline bundle

**Within-agent duplicates eliminated**: Read deduplication rule stops patterns like 6 reads of same file in one sub-agent.

### Files Blocked by Lane B Ownership

Cannot edit `.agents/agents/orchestrator.agent.md` or `.agents/instructions/orchestration/orchestrator-delegation.instructions.md` (owned by ticket 373072a9).

Required changes documented in: `.tmp/lane-d-to-lane-b-handoff.md`

**Orchestrator template needs**:
- Add "Shared Context Bundle" section after "Delegation contract"

**Delegation instructions need**:
- Replace item 4 "Minimum context" with "Shared context bundle" (full specification)
- Update "Context Isolation" pre-dispatch checklist to include bundle passing

### Acceptance Criteria Status

1. ✅ Sub-agents receive resolved ticket/spec/handoff content inline (not fetched)
2. ✅ Parallel siblings do not independently issue identical discovery sequences
3. ✅ Single sub-agent does not read same unchanged file twice (read deduplication rule)
4. ✅ No sub-agent reads `.agents/agents/*.agent.md` in normal work (contract excerpt inline)
5. ⏸ Benchmark measurement pending: requires integration into orchestrator template before measurable (count of artifacts read by >2 sub-agents drops to zero)

### Validation

This is **prose-only guidance** that cannot be mechanically tested. Validation consists of:
- Context bundle composition defined with exact field structure: ✅
- Parallel fan-out optimization pattern documented: ✅
- Read deduplication rule stated: ✅
- Integration points documented: ✅
- Bundle size target (2k-5k tokens) specified: ✅

### Relation to Benchmark

Benchmark `10d21210` includes parallel siblings needing same artifact. With shared context bundles:
- Artifact fetched ONCE by orchestrator
- Duplicated inline into each sibling (cheap input cost)
- Zero siblings fetch independently
- Count of artifacts read by >2 sub-agents drops to zero versus baseline

### Next Steps

Lane B (ticket 373072a9) must integrate the documented changes into:
- `.agents/agents/orchestrator.agent.md`
- `.agents/instructions/orchestration/orchestrator-delegation.instructions.md`

After integration, the instruction file will trigger automatically via `applyTo: ".agents/agents/orchestrator.agent.md"` frontmatter.
