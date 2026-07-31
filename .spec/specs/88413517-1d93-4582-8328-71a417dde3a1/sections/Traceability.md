**Epic:** c608f5ac-cb7f-424f-ae99-22e75a9477d7 — [agents] Consolidate 17 agent templates + 24 prompts into 8 role-based templates with deterministic routing

**Child tickets (implementing C1-C6, DAG: C1 blocks C2/C3/C4; C2 blocks C3/C4/C5/C6; C3+C4 block C6):**
- 3c3b42f3-1412-4c73-a531-4567add92a33 — C1: Define 14-role taxonomy + deterministic routing table in AGENTS.md
- 1c850547-c76a-4d65-83c6-133289552661 — C2: Author 8 consolidated templates + new Telemetry template
- fb241a6c-165f-4a5e-bad7-9ac0ab63348b — C3: Delete/merge superseded templates and prompts
- 46d423d8-0a7e-4dc8-b701-b5c2768f34f7 — C4: Add R2 global state-overview mode to Explorer
- ce9edc5b-cb27-4cb8-8802-68a8714c686c — C5: Compress 5 orchestration instruction files into one
- ea80712b-3506-4b8f-bb36-fc2618aa7b82 — C6: Validation — prompt-replay/routing determinism check

**Related specs:**
- ec3b13f1-ae9f-4f11-b3f9-e8fa3877afbd — Per-template MCP tool grants (binding grant schema)
- 7c9757a7-739f-4dfe-a4de-26f187f3b5aa — Default agent tool suite
- a4d61b8c-df1c-454d-ab56-4bce5706eb15 — Model cost awareness and tiered model routing
- 39983ddf-1f7e-4081-a060-6b8258eb4c41 — Model price awareness: orchestrator-mode enforcement
- b71658f1-8de2-444a-9be1-64b1d8ecce70 — Iteration Loop Workflow