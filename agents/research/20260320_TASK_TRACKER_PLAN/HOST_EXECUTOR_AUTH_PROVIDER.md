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