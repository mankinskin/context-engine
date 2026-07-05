Found. I searched ticket content and expanded dependency subgraphs around feedback and quality loops. Here is the related ticket set for enabling curation of spec/rule/ticket entities for effba966 session-bootstrap epic.

1. Direct entity feedback and ratings (core)
1. 88800b2e Attach ratings and feedback to rule entries (done)
2. [29bf9628 [spec-api] Add direct feedback on spec entities with integration tests](memory-api/.ticket/tickets/29bf9628-1dc5-4bb4-ae00-b7410dd52db5/ticket.toml) (new)
3. dee7de7a Create rule-api storage model and stable rule IDs (done, foundational model for feedback fields)
4. f15d9e8b Build rule-api for generated agent instruction docs (in-implementation, includes feedback loop surfaces)

2. Feedback store expansion (broad feedback ingestion/search)
1. [b1e9e744 [feedback-api] Feedback inbox, metadata indexing, and deep search](memory-api/.ticket/tickets/b1e9e744-aeac-474a-91d9-07e3a362dc76/ticket.toml) (new)
2. [3a1ec9f8 [feedback-api] Search latency and index growth SLOs](memory-api/.ticket/tickets/3a1ec9f8-15ea-43f2-b6d3-89b88cbdcb17/ticket.toml) (new)
3. [4f86d3d2 [feedback-api] Privileged feedback governance and abuse-boundary enforcement](memory-api/.ticket/tickets/4f86d3d2-2b2a-4c9d-9d46-5f2a437f91b7/ticket.toml) (new)
4. [9c95c1e4 [feedback-api] Event ingestion, metadata normalization, and retention policy](memory-api/.ticket/tickets/9c95c1e4-3cdb-428e-b9de-800684651226/ticket.toml) (new)
5. [b7b84c10 [feedback-api] High-scale search, clustering, and reconciliation workflows](memory-api/.ticket/tickets/b7b84c10-8dc5-4087-87ad-6fe27ebbcd45/ticket.toml) (new)
6. [c2d6a14a [feedback-api] Retention, redaction, and privacy incident controls](memory-api/.ticket/tickets/c2d6a14a-98b7-4f98-9f62-90a5ccf06d9e/ticket.toml) (new)

3. Quality rating and curation loops (cross-store health scoring)
1. [bd1c7cc0 [audit-api] Continuous store health scoring and cleanup loops](.ticket/tickets/bd1c7cc0-2850-418d-b701-981b95c587ee/ticket.toml) (new)
2. [67b6117b [audit-api] Health metric taxonomy and scoring model for store entries](.ticket/tickets/67b6117b-5978-4c89-9cd4-4c8b043f4fba/ticket.toml) (new)
3. [11fb9bcf [audit-api] Cleanup loop UX and automated remediation suggestions](.ticket/tickets/11fb9bcf-fcd5-4eff-b380-64b80f4a5c9c/ticket.toml) (new)
4. [89f21dd2 [audit-api] Health score recompute and queue refresh SLOs](.ticket/tickets/89f21dd2-6307-4f30-b0f8-7b36b3cfce66/ticket.toml) (new)
5. [8f021514 [audit-api] Explainable remediation queues and reversible cleanup actions](.ticket/tickets/8f021514-6d53-45f3-a0cf-667fb3865a4d/ticket.toml) (new)
6. [f6ee97de [audit-api] Score-model versioning and anti-gaming safeguards](.ticket/tickets/f6ee97de-c7c9-46d3-878b-c6df5f4a4bc9/ticket.toml) (new)
7. [acefc2ae [ticket-api] Validation-aware dependency requirements and health model](.ticket/tickets/acefc2ae-e257-4bc8-a4c7-0ec3137e374d/ticket.toml) (new, ticket-store quality signal contributor)
8. [8a90a63c [program][multi-store] Store expansion and operational health program](.ticket/tickets/8a90a63c-0a07-439f-90e8-9124212b2dc8/ticket.toml) (new, umbrella)

Key gap
1. I did not find an explicit ticket for direct feedback/ratings on ticket entities analogous to the spec entity ticket above. Current direct entity coverage is rule + spec, with ticket quality represented via audit/health model work.

Relevance to effba966
1. This set gives you the feedback capture layer (rule/spec), feedback search/reconciliation layer (feedback-api), and scoring/cleanup layer (audit-api) needed for continuous curation.
2. Right now, effba966 does not yet appear graph-linked to these tickets. If you want, I can map and propose concrete depends_on edges from effba966 to the minimal subset that should gate session-bootstrap curation behavior.