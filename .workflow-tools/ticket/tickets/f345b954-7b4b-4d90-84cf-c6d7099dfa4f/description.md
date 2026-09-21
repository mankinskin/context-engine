# [AOH][Interview] Requirements Discovery — Full Q&A Record

## Status: COMPLETE — All OQs resolved 2026-04-09

---

## Original 10 Questions and Answers

### Q1: Sandbox execution environment
**Q**: What execution environment should agent sessions run in?

**A (round 1)**: cloud-hypervisor (Rust microVM) as primary, Docker as fallback. Local VMs only. Browser support required. Fast boot preferred.

**A (OQ1 final 2026-04-09)**: cloud-hypervisor / Firecracker ruled OUT — no `virtio-gpu` means browsers inside microVMs are limited to SwiftShader software rendering, unacceptable performance. **Adopted: Container-based Browser-as-a-Service** (Docker/Podman, `bollard` Rust crate, GPU passthrough, per-session network namespace isolation). Chromium uses `--use-gl=angle` (Windows/WSL2) or `--use-gl=egl` (Linux).

**ADR-1 locked**: Container BaaS.

---

### Q2: Messenger service
**Q**: Which messaging service(s) should the notifier support?

**A (round 1)**: WhatsApp as primary. Also Telegram, Slack, Discord.

**A (OQ2 final 2026-04-09)**: WhatsApp dropped — requires paid Meta Business API account. **Adopted: Telegram (primary/MVP), Discord, Slack** as secondary adapters. WhatsApp research ticket (`dd5872f4`) cancelled.

**ADR-2 locked**: Telegram → Discord → Slack, in that order.

---

### Q3: Git hosting and PR strategy
**Q**: GitHub or GitLab? How are PRs managed?

**A**: GitHub. **Local-first PR management**: TOML PR records on dedicated `aoh-meta` branch; no push to remote during implementation. Push only on explicit user trigger (merge or share).

**ADR-3 locked**: Local TOML + GitHub on push.

---

### Q4: Orchestrator UI surface
**Q**: How should users monitor and control sessions?

**A**: Terminal-first TUI using `ratatui`. VS Code extension deferred to Phase 2.

**ADR-4 locked**: ratatui TUI v1.

---

### Q5: Agent API provider
**Q**: Which LLM API providers should be supported?

**A (round 1)**: GitHub Copilot only for v1.

**A (OQ5 partial)**: Configurable token and time limits with sane defaults; fine-tune post-MVP. See ADR-10 for default thresholds.

**ADR-5 locked**: Copilot only v1 (thin `CopilotClient` over `reqwest`).

---

### Q6: Session concurrency
**Q**: How many parallel agent sessions should the orchestrator support?

**A**: 5–20 concurrent sessions.

---

### Q7: Ticket store location and crate home
**Q**: Where does the AOH system live relative to existing crates?

**A**: `ticket-api` as durable coordination store. New AOH crates live **inside the existing `context-engine` Cargo workspace**.

---

### Q8: Agent identity and persona assignment
**Q**: Should personas be unique across sessions or reusable?

**A**: Generated personas using nature/plant vocabulary (Petal, Cedar, Fern, Alder…). Reusable — same persona re-assigned when reviving a ticket. LRU rotation across available pool.

**ADR-8 locked**: Reusable nature-persona store with LRU assignment.

---

### Q9: Session archive and revival strategy
**Q**: How should interrupted or paused sessions be revived?

**A**: Summary-injected revival. `session-archive.toml` (result, summary, modified files, test results, open questions) injected into kickoff prompt. Same git worktree reused where possible.

**ADR-9 locked**: Summary-injected revival with archived context TOML.

---

### Q10: Budget and cost control
**Q**: How should token and compute costs be capped?

**A (round 1)**: Tiered escalation — soft limit → agent self-assessment → user notification via messenger → hard terminate.

**A (OQ5/Q10 final 2026-04-09)**: Confirmed tiered approach with **configurable thresholds and sane defaults** (to be calibrated post-MVP). Initial defaults:
- Soft token: 80,000 → warning
- Self-assessment window: 2,000 tokens
- User notify wait: 5 minutes
- Hard token: 200,000 → terminate
- Time soft: 30 min; hard: 90 min
- All values configurable in `orchestrator.toml`

**ADR-10 locked**: Tiered + configurable.

---

## Open Questions Resolution Log

| OQ | Question | Resolution Date | Final Answer |
|---|---|---|---|
| OQ1 | GPU/browser support in microVM | 2026-04-09 | Dead end — no virtio-gpu. Container BaaS adopted. |
| OQ2 | WhatsApp paid account feasibility | 2026-04-09 | Dropped. Telegram primary. |
| OQ5 | Budget limit configuration story | 2026-04-09 | Configurable with sane defaults. |

All OQs resolved. No open questions remain.

---

## Interview Outcome Summary

- **Sandbox**: Container BaaS (bollard/Docker) — not microVM
- **Messenger**: Telegram primary, Discord + Slack secondary — not WhatsApp
- **Git**: Local-first TOML records — no remote push during implementation
- **UI**: ratatui TUI — VS Code extension Phase 2
- **LLM**: GitHub Copilot only v1
- **Concurrency**: 5–20 sessions
- **Crate home**: Inside `context-engine` workspace
- **Identity**: Reusable nature-persona pool, LRU
- **Revival**: Summary-injected with `session-archive.toml`
- **Budget**: Tiered escalation with configurable defaults