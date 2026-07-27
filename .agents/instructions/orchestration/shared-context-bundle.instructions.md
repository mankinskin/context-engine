---
description: "Shared context bundle protocol for sub-agent delegation. Use when spawning sub-agents to avoid redundant artifact discovery. Covers bundle composition, inline content passing, parallel fan-out optimization, and read deduplication."
applyTo: ".agents/agents/orchestrator.agent.md"
---

## Purpose

Sub-agents spawn with zero inherited context and must rediscover artifacts independently. When multiple sub-agents (especially parallel siblings) need the same artifact, each pays for the same discovery. This instruction defines a shared context bundle protocol that eliminates redundant reads.

## Measured Cost

Cross-agent duplicate reads from analyzed sessions (`tmp/subagent_cost_probe.py`):

| artifact | distinct sub-agents reading | total reads |
|---|---|---|
| handoff package JSON | 10 sub-agents | 14 reads |
| `compact-terminal-mcp/src/server.rs` | 6 sub-agents | 21 reads |
| MCP config files | 3 sub-agents | 10 reads |
| spec body.md | 3 sub-agents | 3 reads |

Within-agent redundancy is worse: one sub-agent read `subagent_rollup.rs` 6 times, `body.md` 4 times, `lib.rs` 4 times.

Parallel fan-out siblings issued byte-identical command sequences: same `ticket.exe get`, `spec.exe get`, `spec.exe search`, and file globs.

## Context Bundle Composition

A context bundle is a **structured inline payload** passed to every sub-agent at spawn. It contains resolved artifacts the sub-agent is likely to need, eliminating the need to fetch them.

**Standard bundle fields**:

1. **Resolved tickets**: Full ticket TOML content (not just id) for the ticket(s) the sub-agent acts on
   ```
   tickets:
     <id>:
       title: "..."
       state: "..."
       component: "..."
       description: |
         <full markdown body>
   ```

2. **Resolved specs**: Full spec body content (not just id/slug) for specs covering the work scope
   ```
   specs:
     <id>:
       title: "..."
       slug: "..."
       component: "..."
       body: |
         <full markdown body>
       sections:
         <section-name>: |
           <section content>
   ```

3. **Handoff package**: Complete handoff JSON or structured content if the sub-agent is acting on a handoff
   ```
   handoff:
     objective: "..."
     context_anchors: [...]
     decisions: [...]
     target_files: [...]
     validation_gates: [...]
   ```

4. **Relevant file digests**: For files the sub-agent will likely read, include a bounded window or skeleton
   ```
   file_digests:
     <workspace-relative-path>:
       lines: <total-line-count>
       skeleton: |
         <interface-level view — exported symbols, type signatures, no bodies>
   ```

5. **Validation commands**: Pre-parsed list of validation commands the sub-agent should run
   ```
   validation_commands:
     - "cargo test -p compact-terminal-cli"
     - "cargo test -p compact-terminal-mcp"
   ```

**Bundle size guidance**: Target 2k-5k tokens for the bundle. Do not inline 20k of full file bodies — use bounded windows or skeletons. The bundle should make the sub-agent "context-warm", not "context-saturated".

## Parallel Fan-Out Optimization

When spawning parallel siblings (independent READ-ONLY probes dispatched concurrently), compute the **shared prefix** of what siblings need ONCE in the orchestrator, and inline it into EACH child prompt.

**Pattern**:

1. Identify shared artifact set (e.g., all siblings need the same ticket, spec, handoff package)
2. Resolve the shared artifacts ONCE via a single pre-fetch batch
3. Inline the identical shared bundle into every sibling's prompt
4. Each sibling still gets a unique objective and return contract, but the context bundle is duplicated text — no child fetches it

**Cost model**: Duplicating 3k tokens of inline context across 4 siblings = 12k input tokens. The alternative — 4 siblings each fetching the same 3 artifacts via separate tool calls — costs 4 turns × estimated 37k prefix = ~148k tokens. Input duplication is VASTLY cheaper than per-sibling discovery.

## Read Deduplication Within a Sub-Agent

Sub-agents must not re-read files they already have in their own transcript.

**Rule for sub-agent templates**: Before issuing a file read, check if the file was already read in this session. If the file content is unchanged (same path, no intervening edit), reference the prior turn instead of re-reading.

**Implementation**: Add to sub-agent guidance (Implement, Research, Testing, etc.):

```markdown
## Read Deduplication

Before reading a file, check if you already read it in this session. If you did and the file has not been edited since, DO NOT re-read it — reference the prior turn number instead.

Reading the same unchanged file twice in one session is ALWAYS waste.
```

## Eliminating Agent Template Reads

Sub-agents currently read `.agents/agents/*.agent.md` files to understand their role or the delegation system. This is pure waste — the orchestrator already knows the contract and should state it inline.

**Fix**: When delegating, the orchestrator prompt MUST include the relevant contract excerpt from the target agent's template. Do not make the sub-agent read the template itself.

**Example** (in orchestrator delegation prompt):

```markdown
You are dispatched as an Implement Agent. Your contract:
- Consume a complete handoff package (provided inline below)
- Make the smallest correct change that satisfies the behavior
- Validate immediately after the first substantive edit
- Return: implementation target, edits made, validation run, remaining risk

Handoff package:
<inline the full handoff here>
```

## Session-Scoped Artifact Cache (Future)

**Scope for future work**: A session-scoped cache keyed by `(path, content-hash)` could return a cheap "unchanged, see turn N" marker for repeat reads. This is NOT in scope for this ticket — the immediate fix is to inline the bundle and avoid repeat reads via prompt discipline.

## Integration with Orchestrator Template

**Required change to `.agents/agents/orchestrator.agent.md`** (DOCUMENT ONLY — Lane B will apply):

Add after the "Delegation contract" section:

```markdown
## Shared Context Bundle

EVERY sub-agent receives a **context bundle** containing resolved artifacts inline. Do NOT pass only ids/paths — pass the FULL CONTENT the sub-agent needs.

**Bundle fields**: resolved tickets (full TOML + description), resolved specs (full body + sections), handoff package (complete JSON), relevant file skeletons, validation command list.

**Parallel fan-out**: For sibling sub-agents, compute the shared context prefix ONCE and duplicate it into each sibling's prompt. Input duplication is far cheaper than per-sibling discovery.

**Size target**: 2k-5k tokens per bundle. Use bounded windows or skeletons, not full 20k file dumps.

See `.agents/instructions/orchestration/shared-context-bundle.instructions.md` for complete bundle composition rules.
```

## Integration with Delegation Instructions

**Required change to `.agents/instructions/orchestration/orchestrator-delegation.instructions.md`** (DOCUMENT ONLY — Lane B will apply):

Replace the current "Minimum context" item in the delegation contract (item 4) with:

```markdown
4. **Shared context bundle** — pass resolved artifact CONTENT inline, not just ids/paths
   - Resolved tickets: full TOML + description markdown, not just ticket id
   - Resolved specs: full body + sections, not just spec id/slug
   - Handoff package: complete JSON, not just a reference
   - Relevant file skeletons: bounded interface-level view, not "read it yourself"
   - Validation commands: exact command list, not "figure out what to run"
   - For parallel siblings: compute shared prefix ONCE, duplicate into each prompt
   - Size target: 2k-5k tokens per bundle
   - See `.agents/instructions/orchestration/shared-context-bundle.instructions.md`
```

And update the "Context Isolation" section to clarify:

```markdown
**Pre-dispatch checklist** (every sub-agent prompt MUST be self-contained):
- Pass FULL CONTENT of artifacts via context bundle, not just ids/paths
- Name every file with full workspace-relative path (never "the file we discussed")
- Include the target agent's contract excerpt inline (do not make sub-agent read its own template)
- State repository root and any command/cwd assumptions
- Define every referent — no "this", "that fix", or "the earlier change"
- State exact return shape you want back
```

## Validation

This is prose-only guidance that cannot be mechanically tested. The acceptance check is:
1. Context bundle composition is defined with exact field structure
2. Parallel fan-out optimization pattern is documented
3. Read deduplication rule for sub-agents is stated
4. Integration points with orchestrator template and delegation instructions are documented
5. Bundle size target (2k-5k tokens) is specified

## Relation to Benchmark

Benchmark ticket `10d21210` includes a scenario with parallel siblings needing the same artifact. With shared context bundles applied:
- The artifact is fetched ONCE by the orchestrator
- Duplicated inline into each sibling prompt (cheap input cost)
- Zero siblings fetch it independently (eliminates cross-agent duplicate reads)
- The count of artifacts read by >2 distinct sub-agents drops to zero versus baseline
