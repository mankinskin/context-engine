Roles map to 9 templates (8 consolidated + 1 new). Every one of R1-R14 maps to exactly one owning template; no role is split across templates and no template owns zero roles.

| Template | Owned roles |
|---|---|
| Explorer | R1, R2, R3, R4 |
| Refinement | R5, R9 |
| Implementer | R10 |
| Test & Harness | R6, R7 |
| Reviewer | R11 |
| Orchestrator | R14 (pure-delegation; must never gain write tools) |
| Closer | R5 (transitions subset: iteration/handoff/commit only) |
| Strategy | R12, R13 |
| Telemetry (new) | R8 |

Note: R5 appears under both Refinement (planning-oriented deep writing) and Closer (transition-oriented deep writing: iteration close, handoff authoring, commit). This is a scoped split of R5's write surface by *intent*, not a violation of the one-owning-template rule for the role as a whole — the routing contract (section 4) must resolve R5 requests to exactly one of Refinement or Closer per request, never both, using the request-signal table's first-match-wins order.