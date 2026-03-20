# Host Executor Service — Auth and Provider Adapter Spec

## Objective

Define how the host executor service is started, how provider credentials are loaded,
and how worker agents authenticate without receiving long-lived online API keys.

## Current Status

- `ticket serve --stdio` is planned in Phase 1.5.
- A dedicated host executor service process is not yet implemented.
- This document defines the target operational contract for implementation.

## Service Start Modes

### Mode A: Local foreground (development)

```bash
ticket host-executor \
  --index-root /absolute/path/to/ticket-index \
  --listen stdio \
  --provider github-models
```

### Mode B: Local daemon (production host)

```bash
ticket host-executor \
  --index-root /absolute/path/to/ticket-index \
  --listen http \
  --bind 127.0.0.1:8787 \
  --provider github-models
```

### Mode C: Hybrid

Serve ticket protocol over stdio and provider calls over internal HTTP adapter.

## Credential Model

Principle: provider secrets never leave trusted host runtime.

- Coordinator/host process loads provider credentials from env vars or secret manager.
- Worker agents receive only short-lived scoped executor tokens.
- Assignment packets never contain raw provider API keys.

## Environment Variables

### Core

- `TICKET_INDEX_ROOT` absolute index path
- `TICKET_EXECUTOR_LISTEN` `stdio|http`
- `TICKET_EXECUTOR_BIND` bind address for HTTP mode
- `TICKET_EXECUTOR_TOKEN_SIGNING_KEY` HMAC key for ephemeral worker tokens

### Provider selection

- `TICKET_LLM_PROVIDER` `github-models|azure-openai|openai|local`

### Provider credentials

- GitHub Models:
  - `GITHUB_TOKEN` (or org-scoped token from secret manager)
- Azure OpenAI:
  - `AZURE_OPENAI_ENDPOINT`
  - `AZURE_OPENAI_API_KEY`
  - `AZURE_OPENAI_DEPLOYMENT`
- OpenAI:
  - `OPENAI_API_KEY`
- Local backend:
  - provider-specific local endpoint vars

## Worker Authentication

Workers authenticate to host executor using ephemeral executor tokens minted by coordinator.

Token claims (minimum):

- `sub` worker identity
- `assignment_id`
- `ticket_id`
- `scope` allowed actions (`claim`, `update`, `unclaim`, `inference`)
- `exp` short expiry (recommended: 15 min)
- `nonce` replay protection

Rules:

- token is single-assignment scoped
- refresh requires live coordinator approval
- token invalid after assignment closure or reassignment

## Request Authorization Policy

For every worker request:

1. verify token signature and expiry
2. verify `assignment_id` is active and mapped to worker
3. verify requested action is in token scope
4. enforce ticket match (`ticket_id` claim must equal target ticket)
5. attach `assignment_id` to audit log

Deny reasons (structured):

- `auth.invalid_signature`
- `auth.expired_token`
- `auth.assignment_not_active`
- `auth.scope_denied`
- `auth.ticket_mismatch`

## Provider Adapter Contract

Host executor uses a provider adapter trait:

```rust
trait InferenceProvider {
    fn name(&self) -> &'static str;
    fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse, ProviderError>;
}
```

Adapters:

- `GithubModelsProvider`
- `AzureOpenAiProvider`
- `OpenAiProvider`
- `LocalProvider`

Selection:

- selected by `TICKET_LLM_PROVIDER`
- fallback chain optional but explicit (no silent fallback)

## GitHub Copilot Subscription Clarification

- A GitHub Copilot user subscription is not a generic API credential for custom
  host executor services.
- Do not assume a Rust "Copilot SDK" for server-side inference integration.
- Implement provider calls via supported public APIs and standard Rust HTTP client stack.

## Audit and Observability

Every external inference call must log:

- timestamp
- provider name
- assignment id
- worker id
- ticket id
- model/deployment id
- token scope
- latency + status

Never log raw provider secrets or full prompts containing sensitive data.

## Security Baseline

- secrets from env/secret manager only
- no secrets in tickets, assignment packets, or repo
- TLS required for non-local HTTP mode
- token signing key rotation policy (recommended: 30 days)
- executor token TTL <= 15 minutes

## Phase Mapping

- Phase 1.5:
  - implement host executor auth skeleton + stdio mode token checks
  - integrate with `ticket serve --stdio` session identity
- Phase 5:
  - HTTP/MCP adapter hardening
  - provider adapter expansion and policy controls

## Open Decisions

- exact token format (`JWT` vs compact HMAC envelope)
- whether inference scope is split from ticket-mutation scope
- default provider for local dev in this repository

## Agent Lifecycle (End-to-End)

### 1) Agent start and assignment

1. Coordinator creates assignment packet (worker or validator).
2. Worker receives packet with:
  - assignment id
  - ticket id
  - protocol mode and index root
  - branch/workdir context
  - acceptance and validation requirements
3. Coordinator mints short-lived executor token scoped to assignment.
4. Worker starts session against host executor and acknowledges assignment.

### 2) Branch and working directory policy

Default policy:

- Working directory: repository root of current workspace checkout.
- Branch source: assignment packet execution.branch.feature.
- Merge target: assignment packet execution.branch.merge_target.

Rules:

- Worker must report current branch + cwd at session start.
- If branch does not exist, worker creates it from merge target head.
- Worker must not switch to unrelated branches during assignment.
- Any branch mismatch is a recoverable assignment error and must be reported.

### 3) Validation flow

Worker phase:

1. claim ticket
2. implement + run required checks
3. attach evidence refs
4. transition to review/validating

Validator phase:

1. coordinator dispatches validator assignment
2. validator claims ticket under validating state
3. runs required validation profile checks
4. marks validation passed or failed with evidence

Separation-of-duties is mandatory: validator identity must differ from worker identity.

### 4) Completion and merge flow

Ticket completion path:

1. validation passed
2. ticket moves to release-candidate
3. release gates checked (bugs, smoke, rollback)
4. branch merged to merge target
5. ticket updated to released/monitoring/done according to governance

Merge responsibility can be coordinator or release agent, but merge event must include:

- assignment id chain (worker + validator)
- merge commit
- release target/version

### 5) Session close protocol

Normal close:

1. worker sends final handoff payload
2. worker unclaims ticket
3. host executor marks session closed
4. coordinator marks assignment closed

Session close event must include reason:

- completed
- handed_off
- blocked
- superseded
- aborted

## Early Stop and Error Handling

### Early-stop detection

Detected via one or more:

- stdio disconnect
- heartbeat/session-liveness timeout
- repeated auth failures
- worker explicit abort

### Required handling steps

1. mark session closed with failure reason code
2. stop token refresh and invalidate active token
3. release or expire lease according to policy
4. emit structured incident event with assignment id + ticket id + worker id
5. move ticket state to blocked or back to review with blocker metadata
6. coordinator decides requeue vs reassignment

### Error classes

- `session.disconnect`
- `session.timeout`
- `session.auth_failure`
- `execution.branch_mismatch`
- `execution.cwd_mismatch`
- `execution.validation_failed`
- `execution.unexpected_exit`

## Minimal Early Integration Test Plan

These tests should run in Phase 1.5 before broad rollout.

### T1 Startup and auth bootstrap

- start host executor in stdio mode with test provider adapter
- mint scoped token and authenticate worker
- verify unauthorized request is rejected with structured auth error

### T2 Assignment start context

- dispatch assignment with explicit branch and cwd constraints
- verify worker reports matching branch + cwd before claim
- verify mismatch yields recoverable structured error

### T3 Ticket lifecycle happy path

- claim -> update -> evidence attach -> unclaim
- verify assignment id appears on all emitted events

### T4 Validation handoff

- worker completes implementation and moves ticket to validating
- validator (different identity) claims and validates
- verify same-identity validator assignment is rejected

### T5 Early-stop recovery

- terminate worker session mid-assignment
- verify token invalidation, lease expiry/release behavior, incident event emission
- verify ticket transitions to blocked/review with blocker metadata

### T6 Merge and completion linkage

- simulate validation passed -> release-candidate -> merge
- verify merge record includes assignment chain and release target

Required CI signal for Topic B exit: T1-T6 all green.