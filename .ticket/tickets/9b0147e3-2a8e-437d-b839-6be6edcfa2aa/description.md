## Objective

Integrate the **session-coupled feedback signal** into the delegation loop: the ability to flag specific problem spots and draw attention to particular scenarios, which is mostly coupled to a session.

## Scope (from transcript)

- Feedback lets an agent/user mark specific problem spots and highlight scenarios worth attention.
- This capability is mostly coupled to a session, so it must attach to session/entity context.
- The transcript flags that this capability may be incomplete.

## First task: implementation-status audit

- Audit how far the session-coupled feedback capability is implemented today (feedback-api, feedback-mcp, session-api coupling).
- Report the gap between current state and the "flag problem spots per session" requirement before building further.

## Acceptance criteria

- Written audit of current feedback + session coupling state (what exists, what is missing).
- Defined mechanism for attaching feedback to a session/entity and surfacing it into the delegation evaluation.
- Feedback entries target canonical entity URNs (per AGENTS.md feedback workflow).

## Anchor

Feeds the delegation quality/cost metric ticket (sibling under this epic).

---

# Implementation-Status Audit (2026-07-27)

## Finding: PARTIAL — data model already couples feedback to sessions; queries + pipeline + delegation-consumption missing.

## Audit table (capability → status → evidence)
- feedback-api core — EXISTS — memory-api/crates/feedback-api/src/lib.rs
- Entity URN targeting — EXISTS — FeedbackEntry.target: EntityUrn
- Session coupling in data model — EXISTS — FeedbackProvenance.session_id: Option<String> (~L284)
- Session+author co-requirement validation — EXISTS — feedback-api/src/lib.rs L436-447
- Turn-level provenance — EXISTS — FeedbackProvenance.turn_sequence, tool_call_id (L296-309)
- MCP feedback_ingest with session_id — EXISTS — feedback-mcp/src/server.rs IngestInput.session_id (L47)
- feedback_inbox / feedback_query — PARTIAL (no session filtering) — server.rs L144-175
- feedback_summary — PARTIAL (no session filtering) — server.rs L177-189
- CLI feedback ingest --session-id — EXISTS — feedback-cli/src/main.rs L39
- CLI feedback inbox/summary — PARTIAL (no session filtering) — main.rs L129-161
- session-api coupling — PARTIAL — session-api/src/transcript_feedback.rs
- Transcript signal mining — EXISTS — mine_structured_feedback_signals, mine_failed_tool_call_signals, mine_explicit_ingestion_signals
- Failed-tool-call → entity mapping — EXISTS — map_failed_tool_call_to_entity (failed_tool_calls.rs L44-52)
- Feedback ingestion-from-sessions persistence — MISSING (mining exists, no persistence pipeline)
- Delegation-evaluation consumer — MISSING (no integration)
- Cost-gate feedback integration — MISSING — mcp-cost-gate/src/gate.rs does not read feedback
- Session-mcp feedback surfacing — MISSING
- Rule indexing uses feedback — EXISTS — rule-api/src/store_index.rs L319 is_low_rated

## Direct answers
- (a) Can feedback attach to a session today? YES at ingestion (session_id in provenance, ingest accepts it); query/filter BY session_id is MISSING.
- (b) feedback-api supports BOTH: target: EntityUrn (primary index) + provenance.session_id (optional metadata).
- (c) Delegation-evaluation consumer of feedback: NONE. Only operational consumer is rule-api is_low_rated.

## Proposed mechanism
- feedback-api (feedback_store.rs): add entries_for_session(session_id), entries_for_urn_in_session(urn, session_id), entities_with_feedback_in_session(session_id).
- session-api (transcript_feedback/ingestion.rs): add ingest_session_feedback(session_path, store) to persist mined signals.
- session-api (runtime.rs): add SessionRuntimeContext.entity_feedback: HashMap<EntityUrn, EntityFeedbackSummary> with refresh_entity_feedback(store) and entities_needing_attention().
- transports: feedback_for_session(session_id), feedback_entities_in_session(session_id) (feedback-mcp); session_runtime_feedback_context(workspace_session_id) (session-mcp).
- delegation (mcp-cost-gate/src/gate.rs): add optional feedback_store + evaluate_with_feedback(tool, session_id) to escalate when a tool/entity has open feedback.

## Remaining build gaps (post-audit implementation scope)
1. Session-scoped feedback queries (feedback-api + MCP/CLI filtering).
2. Session→feedback persistence pipeline (mined signals are currently ephemeral).
3. Delegation-evaluation integration (cost-gate/orchestrator consumption).
4. Session runtime feedback context (session-mcp surfacing of feedback for pinned entities).

## AC reconciliation
- AC "Written audit of current feedback + session coupling state" — SATISFIED by this audit.
- AC "Defined mechanism for attaching feedback to a session/entity and surfacing into delegation evaluation" — SATISFIED by Proposed mechanism above.
- AC "Feedback entries target canonical entity URNs" — already true today (target: EntityUrn).
Remaining ticket work is the 4-gap build above.