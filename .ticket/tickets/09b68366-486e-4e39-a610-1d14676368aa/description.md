# [AOH][Research] Multi-Agent Coordination and Cross-Agent Communication Protocols

## Objective

Define how parallel agent sessions communicate with each other and with the orchestrator: result reporting, conflict detection, work handoff, and shared state access. Identify patterns from existing multi-agent systems and design a Rust-native protocol.

## Research Questions

1. What information do agents need to share to avoid work conflicts?
2. How does agent A hand off results to agent B (e.g., validator agents)?
3. What is the minimal message format for orchestrator ↔ agent communication?
4. How do we detect that two agents are modifying the same files?
5. Can the existing ticket-api serve as the coordination primitive, or do we need a separate messaging layer?
6. How do mature multi-agent systems (AutoGen, LangGraph, CrewAI) handle agent coordination?

## Coordination Primitives

### Option A: Ticket Store as Sole Coordination Layer
- Agents read and write ticket fields (state, evidence_refs, notes)
- Orchestrator polls ticket states to detect progression
- No separate messaging channel
- Pros: simple, auditable, already exists
- Cons: polling latency, not suitable for real-time events

### Option B: Event Bus (in-process or lightweight broker)
- Orchestrator runs an in-process event bus (tokio channels or broadcast)
- Agents connect via stdio MCP or HTTP to emit/subscribe to events
- Events: `session_started`, `file_modified`, `test_passed`, `result_ready`, `conflict_detected`
- Pros: real-time, decoupled
- Cons: in-memory only (lost on orchestrator restart); needs persistence layer

### Option C: Persistent Message Queue
- Use a lightweight queue: SQLite (via `sqlx`), redb, or redis-lite
- Agents push messages; orchestrator pulls
- Durable across restarts
- Pros: durable, replay-able
- Cons: adds dependency

### Option D: Direct MCP Tool Calls (Agent → Orchestrator)
- Orchestrator exposes its own MCP server
- Agents call orchestrator tools: `report_progress`, `signal_conflict`, `request_review`
- Orchestrator handles calls synchronously
- Pros: leverages existing MCP infrastructure; typed API
- Cons: tight coupling; orchestrator must handle concurrent tool calls

## File-Level Conflict Detection

### Problem
Two agents modifying the same file path in separate git worktrees will produce conflicting PRs.

### Detection Strategy
1. **At dispatch time**: Orchestrator builds a work-scope map per ticket (based on ticket description or prior LLM analysis)
2. **At commit time**: Agent reports modified file list in result payload; orchestrator checks for overlap
3. **At PR creation time**: GitHub/GitLab diff API reveals overlapping paths

### Conflict Resolution Options
- **Pause and notify**: freeze the later agent, notify user
- **Sequential merge**: complete agent A first, rebase agent B onto A's branch
- **Scope revision**: LLM-assisted scope partitioning to eliminate overlap

## Agent Result Report Format

```json
{
  "session_id": "...",
  "agent_id": "...",
  "ticket_id": "...",
  "branch": "agent/...",
  "status": "success | partial | failed",
  "modified_files": ["src/foo.rs", "tests/bar.rs"],
  "test_results": {
    "run": 42,
    "passed": 40,
    "failed": 2,
    "skipped": 0
  },
  "evidence_refs": ["target/test-logs/...", "cargo-check-output.txt"],
  "acceptance_criteria": [
    {"criterion": "...", "met": true, "evidence": "..."},
    {"criterion": "...", "met": false, "evidence": "..."}
  ],
  "summary": "Implemented X by doing Y. Fixed Z. Remaining: ...",
  "cost": {"tokens_in": 5000, "tokens_out": 2000, "duration_secs": 120}
}
```

## Orchestrator → Agent: Kickoff Prompt Template

```markdown
# Agent Session: {agent_id}

## Your Assignment
Ticket: {ticket_title} ({ticket_id})
Branch: {branch_name} (already created; `git worktree add` complete)
Working directory: {cwd}

## Your Identity
You are `{agent_name}` ({agent_email}). All commits must use this identity.
Do not commit as any other user.

## Available Tools
{mcp_tool_list}

## Ticket Description
{ticket_description}

## Acceptance Criteria
{acceptance_criteria_checklist}

## Done Condition
When all acceptance criteria are met:
1. Run the validation suite
2. Commit all changes with message: `{ticket_id}: {ticket_title}`
3. Call `report_results` tool with your structured result JSON
4. Do not merge or create the PR yourself — the orchestrator handles that

## Constraints
- Only modify files within your assigned scope: {file_scope}
- Token budget: {token_budget} tokens total
- Do not leak secrets or credentials in any output
```

## Existing Multi-Agent Patterns to Research

### AutoGen / AG2
- `GroupChat` with `GroupChatManager` as orchestrator
- `ConversableAgent.initiate_chat()` for bilateral sessions
- Tool registration per agent type (worker vs validator)
- **Extract**: group chat termination condition design

### LangGraph
- `StateGraph` with parallel node execution (`RunnableParallel`)
- `interrupt_before` / `interrupt_after` for human-in-the-loop
- `MemorySaver` checkpointer for session persistence
- **Extract**: interrupt node design, state checkpoint format

### CrewAI
- Agent `role`, `goal`, `backstory` fields → maps to our agent identity
- Task `expected_output` and `context` fields → maps to our ticket acceptance criteria
- Crew `process=Process.hierarchical` with manager → maps to orchestrator pattern
- **Extract**: hierarchical process design, result delegation chain

## Acceptance Criteria

- [ ] Coordination primitive options A–D evaluated with tradeoffs
- [ ] File-level conflict detection algorithm documented
- [ ] Agent result report JSON schema finalized
- [ ] Orchestrator kickoff prompt template drafted
- [ ] AutoGen/LangGraph/CrewAI patterns documented with extraction recommendations
- [ ] Protocol recommendation (which option A–D) made with rationale