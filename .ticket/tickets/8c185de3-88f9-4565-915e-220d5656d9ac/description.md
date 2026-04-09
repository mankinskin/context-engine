# Impl: Execution Provider Contracts + Copilot API Auth Client

## Purpose

Provide the typed HTTP client that all agent sessions use to communicate with GitHub Copilot's API. Per ADR-5, Copilot is the **only** LLM provider in v1 — there is no provider abstraction layer. This ticket delivers a thin, opinionated `CopilotClient` over `reqwest` with authentication, request/response mapping, retry logic, and redacted logging.

This is the lowest-level dependency in the execution stack: the sandbox manager, assignment runner, and review coordinator all depend on a working provider client before they can drive agent sessions.

## Component Boundaries

### In scope
- `CopilotClient` struct wrapping `reqwest::Client`
- API-key authentication (token injection per request)
- Token refresh/rotation support for short-lived tokens
- Typed request/response structs for Copilot chat completions
- Retry with exponential backoff and jitter on transient errors (429, 5xx)
- Configurable timeout per request
- Redacted logging: tokens and response bodies never appear in logs at INFO or below
- Error types distinguishing auth failure, rate limit, transient, and permanent errors

### Out of scope
- Multi-provider abstraction (no `Provider` trait in v1 — ADR-5)
- Streaming/SSE response handling (defer unless required by Copilot API)
- Token accounting/budget enforcement (owned by the cost watchdog in `orchestrator-core`)

## Key Data Types

```rust
/// Configuration for the Copilot API client.
struct CopilotConfig {
    endpoint: Url,
    api_key: SecretString,      // from secrecy crate or equivalent
    timeout: Duration,
    max_retries: u32,
    backoff_base: Duration,
}

/// A single chat completion request.
struct CompletionRequest {
    model: String,
    messages: Vec<Message>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}

/// A chat completion response.
struct CompletionResponse {
    id: String,
    choices: Vec<Choice>,
    usage: Usage,
}

/// Usage metrics returned by the API (feeds cost watchdog).
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

/// Typed error categories for caller-side handling.
enum ProviderError {
    Auth(String),
    RateLimit { retry_after: Option<Duration> },
    Transient(reqwest::Error),
    Permanent(String),
    Timeout,
}
```

## Design Decisions Mapped from ADRs

| ADR | Implication |
|---|---|
| ADR-5 (Copilot only) | No `Provider` trait; `CopilotClient` is the concrete type used everywhere |
| ADR-10 (Budget controls) | `Usage` struct returned on every response so the cost watchdog can accumulate token counts |
| ADR-7 (Per-session MCP) | Each session instantiates its own `CopilotClient` with session-scoped config |
| `db784443` (Trust boundaries) | API key handled as `SecretString`; never logged; injected via orchestrator secret-delivery path |

## Acceptance Criteria

- [ ] `CopilotClient` sends authenticated requests to the Copilot chat completion endpoint
- [ ] API key is stored as `SecretString` and never appears in log output (verified by test)
- [ ] Request/response types are fully typed with serde `Serialize`/`Deserialize`
- [ ] Retry logic handles 429 (with `Retry-After` header) and 5xx with exponential backoff
- [ ] Timeout is configurable and enforced per request
- [ ] `Usage` data is returned on every successful response for budget tracking
- [ ] Error types distinguish auth, rate-limit, transient, permanent, and timeout failures
- [ ] Unit tests cover: successful completion, auth failure, rate-limit retry, timeout, and redacted logging
