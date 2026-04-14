# Impl: Notifier Adapters (Desktop + Messenger) for Assignment/Review Events

## Purpose

Deliver actionable notifications to the human operator when agent sessions produce events that require attention: review requests, budget warnings, failures, and merge completions. Per ADR-2, v1 supports **Telegram** (primary MVP), **Discord**, and **Slack** behind a `Notifier` trait with a `MultiNotifier` routing policy.

The notifier layer is a consumer of the progress events emitted by the assignment runner (`a8632357`) and review coordinator (`d0cc3c8b`). It does not make decisions — it formats and delivers messages.

## Component Boundaries

### In scope
- `Notifier` trait with `send(notification: Notification) -> Result<()>`
- `TelegramNotifier` — primary adapter using Telegram Bot API
- `DiscordNotifier` — webhook-based adapter
- `SlackNotifier` — webhook-based adapter
- `DesktopNotifier` — OS-level notifications via `notify-rust` or equivalent
- `MultiNotifier` — configurable routing policy dispatching to multiple adapters
- Rate limiting: suppress duplicate/noisy events (e.g., repeated progress ticks)
- Notification types: review-ready, budget-warning, session-failed, session-completed, merge-complete, change-requested
- Actionable prompts: messages include enough context for the operator to act (ticket ID, agent ID, summary, and action hints)
- Redaction: sensitive fields (tokens, secrets, raw error internals) are stripped before delivery (per `db784443`)
- Configurable per-channel routing: e.g., failures → desktop + Telegram, reviews → Telegram only
- `CommandListener` interface: inbound Telegram bot commands routed to the orchestrator operator-command channel (ADR-12)
- Operator command parsing: `/approve`, `/reject`, `/retry`, `/stop`, `/extend <minutes>`, `/status` commands from Telegram, Discord, and Slack
- Allow-list enforcement: verify sender's messenger user ID against the operator allow-list in `orchestrator.toml` before dispatching any command
- Command acknowledgment: reply to the operator with confirmation, current state, or error for each received command
- Inbound update polling (Telegram long-polling) or webhook receiver (Discord, Slack Events API)

### Out of scope
- Notification persistence/history (notifications are fire-and-forget in v1)
- Email adapter
- Per-action identity grants beyond allow-list membership (planned for future; v1 model is flat allow-list per ADR-12)

## Key Data Types

```rust
/// The notifier contract.
#[async_trait]
trait Notifier: Send + Sync {
    async fn send(&self, notification: &Notification) -> Result<(), NotifierError>;
    fn supports(&self, kind: NotificationKind) -> bool;
}

/// A notification to deliver.
struct Notification {
    kind: NotificationKind,
    session_id: SessionId,
    ticket_id: TicketId,
    agent_id: AgentId,
    title: String,
    body: String,               // markdown-formatted summary
    urgency: Urgency,
    timestamp: DateTime<Utc>,
}

enum NotificationKind {
    ReviewReady,
    BudgetWarning,
    SessionFailed,
    SessionCompleted,
    MergeComplete,
    ChangeRequested,
}

enum Urgency {
    Low,        // informational
    Medium,     // action suggested
    High,       // action required
    Critical,   // immediate attention
}

/// Multi-adapter router with routing policy.
struct MultiNotifier {
    adapters: Vec<Box<dyn Notifier>>,
    routing: RoutingPolicy,
    rate_limiter: RateLimiter,
}

/// Per-kind routing configuration.
struct RoutingPolicy {
    rules: Vec<RoutingRule>,
}

struct RoutingRule {
    kinds: Vec<NotificationKind>,
    adapters: Vec<String>,      // adapter names to dispatch to
}

/// Trait for adapters that also receive inbound operator commands.
#[async_trait]
trait CommandListener: Send + Sync {
    /// Start listening for inbound commands; emit authenticated commands to the operator channel.
    async fn listen(&self, tx: mpsc::Sender<AuthenticatedCommand>) -> Result<()>;
}

/// An inbound operator command with verified sender identity.
struct AuthenticatedCommand {
    sender_id: MessengerUserId,
    messenger: MessengerKind,
    command: OperatorCommand,
    raw_text: String,
    message_id: u64,    // for sending acknowledgment reply
}

/// Messenger-specific user identity.
enum MessengerUserId {
    Telegram(i64),
    Discord(u64),
    Slack(String),
}

enum MessengerKind {
    Telegram,
    Discord,
    Slack,
}
```

## Design Decisions Mapped from ADRs

| ADR | Implication |
|---|---|
| ADR-2 (Messaging service) | Telegram primary, Discord + Slack supported; WhatsApp dropped; `MultiNotifier` routing |
| ADR-4 (Rust daemon + TUI) | Desktop notifications complement TUI; both fed from mpsc channels |
| ADR-6 (Coordination protocol) | Notifiers receive progress events via dedicated `tokio::mpsc` channel (intra-process) |
| ADR-12 / `db784443` (Trust boundaries) | Full messenger control with operator allow-list (ADR-12); inbound commands traverse same transition guards as TUI; all outgoing notifications are redacted |

## Acceptance Criteria

- [ ] `Notifier` trait is implemented with `TelegramNotifier`, `DiscordNotifier`, `SlackNotifier`, and `DesktopNotifier` adapters
- [ ] `MultiNotifier` routes notifications to configured adapters based on `NotificationKind`
- [ ] Rate limiter suppresses duplicate/noisy events within a configurable window
- [ ] Notifications include actionable context: ticket ID, agent ID, summary, and next-step hints
- [ ] Sensitive fields are redacted before delivery (verified by test)
- [ ] Notification formatting is correct for each platform (Telegram markdown, Discord embeds, Slack blocks)
- [ ] Adapter errors are logged but do not crash the orchestrator (fire-and-forget with retry)
- [ ] Unit tests cover: routing policy dispatch, rate limiting, redaction, and adapter error isolation
- [ ] `TelegramNotifier` implements `CommandListener` and routes parsed commands to the operator-command channel
- [ ] Allow-list enforcement rejects commands from unlisted messenger user IDs (verified by test)
- [ ] Command acknowledgment is sent on every received command (success, rejection, or parse error)
- [ ] Invalid or unrecognized commands return a helpful error reply rather than silently dropping
