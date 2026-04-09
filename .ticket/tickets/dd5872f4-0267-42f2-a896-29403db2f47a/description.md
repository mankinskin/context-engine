# [AOH][Research] WhatsApp Business API and Multi-Messenger Adapter Design

## Context

**User decision (Q2):** Prefers WhatsApp as primary channel. Telegram, Slack, Discord also viable. System must support multiple simultaneously active messengers (user chooses preferred channel; orchestrator dispatches to all configured adapters or to user's preferred one).

## Research Questions

### WhatsApp
1. What are the options for programmatic WhatsApp messaging? (Official Business API, unofficial libraries, Twilio gateway)
2. Does the WhatsApp Business API require Meta business verification? What is the approval timeline?
3. Can a self-hosted server receive WhatsApp messages and send interactive messages (buttons)?
4. What Rust options exist for WhatsApp integration? (HTTP client to Cloud API, or a bridge)
5. Is there a self-hosted WhatsApp bridge compatible with Matrix/XMPP? (whatsapp-business-api, Baileys, mautrix-whatsapp)
6. What are the rate limits for WhatsApp Business API messages?

### General Multi-Messenger
7. How do we design the `Notifier` trait to support multiple simultaneous adapters?
8. How does command routing work when the user replies from WhatsApp vs Telegram?
9. What is the message threading model for each service (linking replies to original notifications)?
10. What is the self-hosting cost model for each service?

## WhatsApp API Options

### Option A: WhatsApp Cloud API (Meta official)
- Endpoint: `https://graph.facebook.com/v18.0/{phone-number-id}/messages`
- Auth: Meta Business Manager account + phone number verification
- Send: text, template messages, interactive messages (buttons/lists)
- Receive: webhook POST for incoming messages
- **Interactive buttons**: YES — up to 3 quick-reply buttons or list messages
- **Requirements**: Meta Business verification (~1-7 days), registered phone number, Facebook App
- **Rust integration**: raw `reqwest` against Graph API; no official Rust SDK
- **Rate limits**: 1000 messages/day on free tier; higher tiers with Meta verification
- **Self-hosted**: No — messages route through Meta servers; phone number must be registered
- **Key risk**: Phone number tied to the Business API cannot be used in WhatsApp consumer app simultaneously

### Option B: Twilio WhatsApp Gateway
- Twilio Messaging API wraps WhatsApp — same Meta backend but Twilio handles business verification
- Rust: `twilio-rs` crate (check status) or raw REST
- Faster onboarding via Twilio sandbox
- Cost: $0.005-0.015/message + Twilio fees
- Suitable for: quick MVP with Twilio sandbox number

### Option C: Self-Hosted Bridge (mautrix-whatsapp)
- https://github.com/mautrix/whatsapp — Matrix bridge to WhatsApp
- Pairs a personal WhatsApp account using multi-device protocol (unofficial but widely used)
- Orchestrator talks to Matrix homeserver; bridge relays to WhatsApp
- **No Meta approval needed** — uses personal account pairing
- **Risk**: unofficial API; WhatsApp may block; no SLA
- **Self-hosted**: YES — full control, no per-message cost
- Rust: Matrix client via `matrix-sdk` crate (async, maintained)

### Option D: Baileys (Node.js unofficial client)
- https://github.com/WhiskeySockets/Baileys — Node.js WhatsApp Web protocol implementation
- Orchestrator spawns Node.js sidecar process; communicates via stdio/HTTP
- **Risk**: same as mautrix — unofficial; WhatsApp can detect and block
- **Rust integration**: subprocess IPC (not idiomatic)

## Comparison Matrix

| Option | Official | Self-hosted | Buttons | Rust | Cost | Approval |
|---|---|---|---|---|---|---|
| WhatsApp Cloud API | Yes | No | Yes | reqwest | Free+/paid | ~1-7 days |
| Twilio gateway | Yes (via Twilio) | No | Yes | reqwest | $$/msg | Faster via sandbox |
| mautrix-whatsapp | No | Yes | No* | matrix-sdk | Free | None |
| Baileys sidecar | No | Yes | Yes | subprocess | Free | None |

*mautrix renders text with inline reactions; no native button UI in WhatsApp consumer

## Recommended Approach

**MVP**: mautrix-whatsapp (Matrix bridge) for fast onboarding, no Meta approval delay, self-hosted.  
**Production**: WhatsApp Cloud API once Meta verification is complete.  
Design the `Notifier` trait so both adapters are drop-in replaceable.

## Other Messengers — Rust Crate Assessment

| Service | Rust Crate | Maintained | Notes |
|---|---|---|---|
| Telegram | `teloxide` | ✓ Active | Excellent; async; inline keyboards |
| Discord | `serenity` | ✓ Active | Full bot API; slash commands; embeds |
| Discord | `twilight` | ✓ Active | Lower-level; more composable |
| Slack | `slack-morphism` | ✓ Active | Web API + Events API + Socket Mode |
| Matrix | `matrix-sdk` | ✓ Active | FOSS; E2E encryption; bridges to WA/TG |
| WhatsApp | (none official) | — | Use Cloud API via reqwest |

## Multi-Messenger Adapter Design

```rust
pub trait Notifier: Send + Sync + 'static {
    /// Send an orchestrator event as a formatted notification.
    async fn notify(&self, event: &OrchestratorEvent) -> Result<MessageHandle>;
    
    /// Poll for incoming user commands. Returns all received since last poll.
    async fn poll_commands(&self) -> Result<Vec<UserCommand>>;
    
    /// Edit or reply to a prior message (for status updates on an existing notification).
    async fn update(&self, handle: &MessageHandle, event: &OrchestratorEvent) -> Result<()>;
}

pub struct MultiNotifier {
    adapters: Vec<Box<dyn Notifier>>,
    routing: RoutingPolicy,  // Broadcast | PrimaryOnly | ByEventType
}
```

### RoutingPolicy
- **Broadcast**: all events go to all configured adapters
- **PrimaryOnly**: WhatsApp primary; others as fallback if primary fails
- **ByEventType**: PR reviews → primary; error alerts → all; digests → primary only

### Command Deduplication
When user sends same command (e.g. "approve 42") on both WhatsApp and Telegram within 30s — deduplicate by `(command_type, target_id, time_window)`.

### Message Thread Correlation
- Each `OrchestratorEvent` gets a `correlation_id`
- Adapters store `correlation_id → platform_message_id` map
- Follow-up events (status updates, completion) are sent as thread replies where platform supports it
- Telegram: reply to original message ID
- Discord: reply in thread
- WhatsApp: no native thread; use quote-reply

## WhatsApp Interactive Message Templates

WhatsApp Cloud API interactive message for PR review:
```json
{
  "type": "interactive",
  "interactive": {
    "type": "button",
    "body": {"text": "PR #42 ready for review\n[AOH] implement feature X\nAgent: Agent-Petal\n3 tests passing"},
    "action": {
      "buttons": [
        {"type": "reply", "reply": {"id": "approve-42", "title": "Approve ✓"}},
        {"type": "reply", "reply": {"id": "changes-42", "title": "Request Changes"}},
        {"type": "reply", "reply": {"id": "view-42", "title": "View Details"}}
      ]
    }
  }
}
```

## Self-Hosting Architecture (mautrix bridge path)

```
Orchestrator (Rust)
    ↕ matrix-sdk
Matrix Homeserver (Synapse / Conduit)
    ↕ mautrix-whatsapp bridge
WhatsApp (personal account, multi-device)
```

Conduit (Rust Matrix homeserver): https://github.com/conduwuit/conduwuit — actively maintained Rust port.

## Acceptance Criteria

- [ ] WhatsApp Cloud API call flow documented (send + receive webhook)
- [ ] mautrix-whatsapp + Conduit self-hosted path validated (docker-compose or native)
- [ ] `Notifier` trait defined with async signatures and `MessageHandle`
- [ ] `MultiNotifier` routing policy design documented
- [ ] Interactive button payload format for top 3 event types (PR ready, session failed, conflict)
- [ ] Telegram `teloxide` bot scaffold validated (compile + send test message)
- [ ] Command deduplication strategy documented
- [ ] Recommendation: MVP adapter + production path with migration plan