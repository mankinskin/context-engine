# [AOH] Epic: Agent Orchestration Harness — Complete Agentic Workflow System

## Objective

Design and implement a full-stack agent orchestration harness that supports the complete development lifecycle from user-driven ticket research through autonomous parallel implementation to review, merge, and archival — with social messenger interaction and sandboxed agent isolation throughout.

## Complete Lifecycle

```
User → Orchestrator: research, ticket creation, user interview
     → Ticket Store: promoted to "ready" by user
     → Orchestrator: detects ready tickets, plans parallel sessions
     → Session Manager: provisions sandboxes, unique agent IDs, git branches
     → Agent Sessions: implement, commit, validate, report
     → PR Manager: creates PR, notifies user via messenger
     → User: reviews PR, requests changes or approves
     → Orchestrator: revives session for changes OR merges branch and archives session
```

## System Components

### 1. Orchestrator Core (Rust binary / VS Code extension)
- Autonomous ticket research and creation from user prompts
- Structured user interview sessions (meeting-style, tracked in ticket assets)
- Ready-ticket detection loop with parallelism planning
- Kick-off prompt generation per ticket assignment
- Agent session supervision and results ingestion

### 2. Agent Session Manager
- Unique agent author identity per session (persistent agent ID with name/persona)
- Git branch/worktree provisioning with cwd enforcement
- Optional VM/container sandbox (Firecracker, Docker, E2B, or process isolation)
- Session state persistence for revival after crashes or change requests
- Session archival policy after completion/merge

### 3. Sandbox Infrastructure
- Process-level: git worktree + restricted shell (baseline)
- Container-level: Docker or Podman per session (medium isolation)
- VM-level: Firecracker microVMs or E2B cloud sandboxes (high isolation)
- Cost tracking per session (token count, compute time)

### 4. Cross-Agent Communication
- Shared ticket store as coordination primitive
- MCP tool sharing across agent sessions
- Agent-to-orchestrator result reporting format (structured JSON report)
- Conflict detection when parallel agents touch overlapping code paths

### 5. PR Lifecycle Manager
- Automatic PR creation from agent branch when session reports completion
- PR metadata (ticket ID, agent ID, validation results, test logs)
- Review routing to user via messaging service
- Change-request propagation back to agent session

### 6. Messaging Service Integration
- User interaction via messenger (Slack / Teams / Discord — TBD by interview)
- Notifications: session start, PR ready, failures, conflicts
- Inline commands: approve, reject, request-changes, priority-boost
- Rate limiting and digest mode to prevent notification spam

### 7. VS Code Integration
- Orchestrator controls visible inside VS Code (extension or task panel)
- Ticket viewer integration for session-state visibility
- Agent session panels with live log streaming
- Meeting/interview UI for user-facing clarification sessions

## Technology Constraints

- Full Rust stack where feasible (tokio, gitoxide/git2, axum, MCP SDKs)
- GitHub Copilot Pro+ as primary agent API (with provider abstraction for future extensibility)
- Existing `ticket-api` as the coordination store
- Existing `context-engine` as the knowledge graph / context layer

## Existing Foundations

- **d5ced7e2** — Phase 2: Copilot API execution layer (already planned, `ready`)
- **T1–T6** (a8d6c1d2..f5d7e9a2) — Bootstrap: host executor auth, branch enforcement, lifecycle, early-stop, merge linkage

## Out of Scope (Initial Release)

- Full multi-cloud agent provider marketplace
- Web UI (TUI / VS Code panel is sufficient for v1)
- Billing subscription management

## Risks

| Risk | Severity | Mitigation |
|---|---|---|
| Credential leakage in agent logs | Critical | Structured redaction, secret scanning |
| Branch collision between parallel agents | High | Orchestrator assigns disjoint file scopes |
| Runaway agent sessions (cost overrun) | High | Token budget enforced per session |
| Orphaned sandboxes after session crash | High | Reconciliation on orchestrator startup |
| Messenger notification spam | Medium | Rate limiting + digest mode |

## Acceptance Criteria

- [ ] Orchestrator can autonomously research a topic and create a ticket tree with zero user code interaction
- [ ] User interview structured Q&A is captured as ticket assets and drives ticket field updates
- [ ] Orchestrator detects ready tickets and launches at least 2 parallel agent sessions in isolated git branches
- [ ] Each agent session commits changes, runs tests, and reports structured results back to orchestrator
- [ ] PR is created automatically with full ticket/agent metadata
- [ ] User receives PR notification via chosen messenger and can approve/reject inline
- [ ] Approved PR is merged; session is archived. Rejected PR revives session with change-request context
- [ ] All sessions run with unique author IDs that appear in git log
- [ ] No credential leakage in any log output (verified by test)
- [ ] Full end-to-end happy-path integration test passes