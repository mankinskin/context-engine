
# Summary

Define a flattened two-tier orchestration architecture — **Planner/Architect** (frontier model, plans once) and **Worker** (fast/cheap model, executes exactly one isolated step) — as an alternative to the current multi-hop delegation chain (T0 orchestrator → T1/T2 sequencer → T3 executor) described in [model-routing.instructions.md](../../.agents/instructions/orchestration/model-routing.instructions.md) and [orchestrator-delegation.instructions.md](../../.agents/instructions/orchestration/orchestrator-delegation.instructions.md). This spec defines the plan schema, the Worker capability boundary, and reconciles the new architecture explicitly with the existing tier ladder, delegation contract, pre-dispatch gates, and shared context bundle protocol. It does not implement the architecture — behavior changes are tracked in separate tickets (1a240fdc, 7563ce30, 44c6cc5c) under epic f13c9836.

# Problem

The current orchestration model (per [orchestrator-delegation.instructions.md](../../.agents/instructions/orchestration/orchestrator-delegation.instructions.md) "Required Workflow" and the Reasoner/Sequencer/Executor capability-role bands) permits a multi-tier chain: a large orchestrator model (T0) plans and delegates to a mid-tier Sequencer (T1/T2), which may itself further decompose and delegate to a cheap Executor (T3). Each additional hop:

- **Dilutes instructions.** Every intermediate agent re-reads (or is re-handed) orchestration contract text — delegation rules, context-bundle composition, pre-dispatch gate mechanics — instead of receiving a single terminal instruction it can execute directly.
- **Adds structural token overhead.** Each hop is a full agent spawn (its own turns, its own tool calls, its own context-bundle inflation), and [shared-context-bundle.instructions.md](../../.agents/instructions/orchestration/shared-context-bundle.instructions.md) documents measured cross-agent duplicate-read costs from exactly this pattern (e.g. 10 distinct sub-agents each independently re-reading the same handoff package).
- **Offsets the cheap tier's per-token savings with iteration cost.** A Sequencer that re-plans or mis-scopes a step forces a retry hop; per [model-routing.instructions.md](../../.agents/instructions/orchestration/model-routing.instructions.md) "Failure Path", a failed cheap-tier attempt escalates exactly one band, which is itself another full spawn. Multi-hop chains multiply the number of places a plan can drift before it reaches an executor.

Source: `AGENT_WORKFLOW_OPTIMIZATIONS.md`, "Step 1: Tooling Restructure" and "The Two-Tier Architecture".

# Scope

- Define the Planner/Architect role: reads the ticket and repo schema/context, and outputs one immutable, rigid, structured (JSON) execution plan intended for **direct execution**, not further re-planning by an intermediate agent.
- Define the Worker role: executes exactly one isolated plan step against one declared target file/scope, then stops — no re-planning, no scope expansion, no chaining to a next step on its own initiative.
- Define a concrete plan schema (this document) that a Planner emits and a Worker consumes.
- Reconcile the two-tier model with the existing tier ladder ([model-routing.instructions.md](../../.agents/instructions/orchestration/model-routing.instructions.md) lines 20-90), the delegation contract ([orchestrator-delegation.instructions.md](../../.agents/instructions/orchestration/orchestrator-delegation.instructions.md) lines 55-91), the mandated pre-dispatch gate mechanism ([pre-dispatch-gates.instructions.md](../../.agents/instructions/orchestration/pre-dispatch-gates.instructions.md)), and the shared context bundle protocol ([shared-context-bundle.instructions.md](../../.agents/instructions/orchestration/shared-context-bundle.instructions.md)).
- Record explicit open questions this spec does not resolve (plan validation, mid-execution failure handling, immutability enforcement, gate interaction).

# Non-Goals

- Implementing the Planner/Worker dispatch mechanism, plan schema validator, or any change to `.agents/agents/orchestrator.agent.md`, `model-routing.instructions.md`, or `orchestrator-delegation.instructions.md` — tracked separately under tickets 1a240fdc, 7563ce30, 44c6cc5c (epic f13c9836).
- Changing the tier ladder's model list, prices, or the `mcp-cost-gate` budget formula — those remain owned by [a4d61b8c](../a4d61b8c-df1c-454d-ab56-4bce5706eb15/spec.toml) and `model-prices.instructions.md`.
- Changing the `model:` per-template frontmatter contract — owned by `model-routing.instructions.md` "Per-Template `model:` Declaration" (ticket 66acb737) and explicitly out of scope for [ec3b13f1](../ec3b13f1-ae9f-4f11-b3f9-e8fa3877afbd/spec.toml).
- Defining a benchmark to measure multi-hop vs two-tier cost (that is a candidate for a future ticket in the same family as 10d21210, not this spec).

# Roles

## Planner / Architect

- **Model tier**: T0 (orchestrator tier — same band as today's orchestrator, e.g. Claude Opus 5).
- **Input**: the ticket (full TOML + description) and enough repo schema/context (file skeletons, relevant instruction excerpts) to produce a complete plan in one pass.
- **Output**: exactly one Plan document (see Plan Schema below), emitted once per ticket/unit of work.
- **Behavior**: reasons once, at high level, over the whole task; decomposes it into a rigid, ordered list of Steps; does **not** expect any downstream agent to re-plan, re-scope, or re-sequence its output.

## Worker

- **Model tier**: T2 default (Claude Sonnet 5), droppable to T3 per-step when the step is bulk/mechanical/read-only — tier selection for an individual step still uses the existing ladder in [model-routing.instructions.md](../../.agents/instructions/orchestration/model-routing.instructions.md), applied per-step instead of per-task.
- **Input**: exactly one Step object from the Plan, plus the context bundle referenced by that step.
- **Output**: the step's declared return contract, then **stops** — it does not proceed to the next step, does not decide the plan needs a change, and does not widen its own scope.

### Worker capability boundary

| Worker MAY | Worker MAY NOT |
|---|---|
| Read/edit files inside the step's declared `target_path` | Edit files outside `target_path`, or expand scope to "while I'm here" fixes |
| Use tools listed in the step's `allowed_tools` | Use a tool not listed in `allowed_tools` |
| Run the step's declared validation command(s) | Invent new validation commands not in the plan |
| Return exactly the step's `return_contract` shape | Return a free-form transcript, or silently change the contract shape |
| Report a blocker via `{pass: false, blocker: "..."}` when `done_criteria` cannot be met | Attempt to re-plan around the blocker itself (e.g. skip to a different file, redefine the objective) |
| Stop after completing (or blocking on) its one step | Chain to the next `step_id` on its own initiative |
| Flag that a dependency (`depends_on`) looks unmet | Decide unilaterally to proceed anyway or reorder steps |

# Plan Schema

Concrete JSON schema sketch for a Plan and its Steps. Field names are normative; a validator or dispatcher may add non-normative metadata but must not omit these fields.

```json
{
  "plan_id": "string (uuid, unique per plan)",
  "ticket_id": "string (source ticket UUID)",
  "goal": "string (one-sentence restatement of the ticket objective)",
  "planner_model": "string (price-table model_id of the Planner that emitted this plan)",
  "created_at": "string (ISO-8601 timestamp)",
  "immutable": true,
  "steps": [
    {
      "step_id": "string (e.g. \"step-01\", unique within plan)",
      "target_path": "string (repo-root-relative path; single file or narrowly-scoped directory)",
      "objective": "string (one scoped, independently verifiable objective — not a restatement of the whole goal)",
      "allowed_tools": ["string (tool name, e.g. \"read_file\", \"replace_string_in_file\", \"run_in_terminal\")"],
      "forbidden_actions": ["string (explicit denial beyond the default boundary, e.g. \"no edits outside target_path\")"],
      "context_bundle_ref": "string (pointer to the pre-resolved context bundle this step should use; see shared-context-bundle.instructions.md)",
      "return_contract": {
        "shape": "string (e.g. \"diff_summary | validation_result | blocker\")",
        "required_fields": ["string (e.g. \"files_changed\", \"validation_output\", \"blocker_reason\")"]
      },
      "done_criteria": "string (verifiable condition, e.g. \"cargo test -p session-api passes\" or \"file X contains section Y\")",
      "depends_on": ["string (step_id of a prerequisite step, empty if none)"]
    }
  ],
  "validation_commands": ["string (plan-level validation run after all steps complete)"]
}
```

Notes:

- `immutable: true` is a declared property of the plan, not (yet) a mechanically enforced one — see Open Questions.
- `context_bundle_ref` is deliberately a pointer, not inline content, at the plan level; the Planner resolves and inlines the actual bundle content into each Worker's dispatch prompt at dispatch time, per [shared-context-bundle.instructions.md](../../.agents/instructions/orchestration/shared-context-bundle.instructions.md).
- `allowed_tools` and `forbidden_actions` together define the enforceable half of the Worker capability boundary table above; the rest (e.g. "does not re-plan") is a behavioral contract stated in the dispatch prompt, not a mechanically checked property.

# What Changes / What Is Preserved

## Superseded (for the two-tier path only; existing multi-hop path is not deleted by this spec — see Non-Goals)

- **The Sequencer capability-role band** for step *decomposition*, as described in `orchestrator-delegation.instructions.md` "Case → Capability-Role → Cost-Class Mapping" (lines 106+): under the two-tier model, decomposition happens exactly once, in the Planner. A Worker executing a plan step does not re-decompose or re-sequence — that removes the "Sequencer decomposes further" hop implied by the current three-role table for cases like "Multi-file feature implementation".
- **The iterative "Plan" step of the 5-step Required Workflow** (`orchestrator-delegation.instructions.md` "Required Workflow", lines 92+, step 1 "Plan"): today this is described as an ongoing orchestrator activity across the session. Under the two-tier model it becomes a single terminal artifact (the Plan document) produced once, not refined per-dispatch.

## Preserved unchanged

- **The tier ladder itself** (`model-routing.instructions.md` lines 20-90) — model names, prices, dominated-model notes, and the T3 cheap-tier selection metric are unchanged. Planner uses T0; Worker tier selection per-step still resolves through this same ladder.
- **The `X = 15` orchestrator-mode threshold** and its scope (`orchestrator-delegation.instructions.md` "When to Activate") — unchanged; it still decides who runs as Planner, not which model a Worker step gets.
- **The mandated pre-dispatch gate mechanism** ([pre-dispatch-gates.instructions.md](../../.agents/instructions/orchestration/pre-dispatch-gates.instructions.md)) — preserved, but its unit of application shifts from "the delegation" to "the plan step": each Step is gated before Worker dispatch using the same Explore-Agent-on-GPT-5-mini mechanism and the same `{pass: true, bundle}` / `{pass: false, blocker}` contract, not a new gate mechanism.
- **The shared context bundle protocol** ([shared-context-bundle.instructions.md](../../.agents/instructions/orchestration/shared-context-bundle.instructions.md)) — preserved; a Step's `context_bundle_ref` resolves to exactly this bundle shape (resolved tickets, resolved specs, file digests, validation commands).
- **Context isolation, verify-before-acting, one-retry-then-step-up-one-band, and parallel fan-out rules** (`model-routing.instructions.md`) — all preserved and apply per-step rather than per whole-task delegation.
- **`mcp-cost-gate` MCP boundary enforcement and `caller_model` requirements** (`orchestrator-delegation.instructions.md` "Cost Gating") — entirely unaffected; this spec changes dispatch shape, not MCP tool-call cost enforcement.

# Traceability / Evidence

- Ticket: [feb5784c Spec: two-tier Planner/Worker model routing architecture](../../.ticket/tickets/feb5784c-ece7-40d3-9617-3faee2f6a753/ticket.toml).
- Epic: [f13c9836 [epic] Worker-tier dispatch contract](../../.ticket/tickets/f13c9836-5b74-433b-b3d2-e1475d080ad0/ticket.toml).
- Follow-up implementation tickets (out of scope here, tracked under the epic): 1a240fdc, 7563ce30, 44c6cc5c.
- Instructions modified in scope (referenced, not edited, by this spec): [model-routing.instructions.md](../../.agents/instructions/orchestration/model-routing.instructions.md), [orchestrator-delegation.instructions.md](../../.agents/instructions/orchestration/orchestrator-delegation.instructions.md).
- Source: `AGENT_WORKFLOW_OPTIMIZATIONS.md`, "Step 1: Tooling Restructure" and "The Two-Tier Architecture".

# Related Specs

- [a4d61b8c Model cost awareness and tiered model routing for agent sessions](../a4d61b8c-df1c-454d-ab56-4bce5706eb15/spec.toml) — owns the tier ladder and `model:` per-template contract this spec's Planner/Worker tiers resolve through; not duplicated here.
- [39983ddf Model price awareness: orchestrator-mode enforcement](../39983ddf-1f7e-4081-a060-6b8258eb4c41/spec.toml) — owns the `X = 15` orchestrator-mode threshold and cost-gate enforcement this spec's "Preserved unchanged" section defers to.
- [ec3b13f1 Per-template MCP tool grants](../ec3b13f1-ae9f-4f11-b3f9-e8fa3877afbd/spec.toml) — adjacent per-template tool-grant scoping; the Worker's `allowed_tools` field in the Plan Schema is a plan-level analog, not a replacement, of that per-template grant mechanism.

# Acceptance Criteria

1. This spec defines the Planner/Architect and Worker roles with an explicit Worker capability boundary table (MAY / MAY NOT columns) distinguishing them from the current Reasoner/Sequencer/Executor three-role model.
2. This spec includes a concrete plan JSON schema with named fields for the plan (plan_id, ticket_id, goal, steps, validation_commands) and each step (step_id, target_path, objective, allowed_tools, return_contract, done_criteria, depends_on), sufficient to implement a validator or dispatcher against.
3. This spec explicitly reconciles with `model-routing.instructions.md` and `orchestrator-delegation.instructions.md`, naming which existing rules are superseded (Sequencer decomposition, iterative Plan step) and which are preserved (tier ladder, X=15 threshold, pre-dispatch gates, shared context bundle, cost gating), and records open questions rather than assuming their answers.

# Open Questions

1. **Plan validation before dispatch.** Who or what validates a Plan against this schema before any Worker is dispatched against it — a dedicated validator tool, the existing pre-dispatch gate mechanism repurposed per-step, or a new gate class? This spec defines the schema but not the validator.
2. **Mid-execution failure handling.** When a Worker's step fails `done_criteria`, or execution reveals the Plan itself was wrong (missing step, wrong `target_path`, unmet hidden dependency), does the whole Plan invalidate and return to the Planner, or can a single Step be patched in place? The current "one retry then step up one band" failure path (`model-routing.instructions.md` "Failure Path") assumes a single delegation, not a multi-step immutable plan — how it composes with Plan immutability is unresolved.
3. **Enforceability of plan immutability.** `immutable: true` is declared in the schema, but nothing described here mechanically prevents a Worker from improvising beyond its step (e.g. editing an adjacent file "while it's there"). Is immutability enforced only by the Worker capability-boundary contract stated in the dispatch prompt (behavioral, unenforced), or does it need a structural control (e.g. a diff-scope check against `target_path` before accepting a Worker's return)?
4. **Interaction with the existing pre-dispatch gate contract.** `pre-dispatch-gates.instructions.md` defines gates per delegation *class* (Implement/Review/Testing/Commit), not per plan step. Does a Plan step reuse its class's existing gate set unmodified, or does the shift from "gate the delegation" to "gate the step" require a new gate class definition (e.g. "does this step's `target_path` exist and match the ticket's declared scope")?
5. **Sequencer role fate.** Does the T1-T2 Sequencer capability role fully disappear under the two-tier model for all cases, or does it survive for a Planner that itself needs to hand off a genuinely cross-cutting, high-risk sub-plan to a mid-tier re-planner before Worker dispatch (i.e., a three-tier fallback for the hardest cases only)?
6. **Plan amendment mechanics.** If a Planner does need to amend a Step mid-execution (per Q2), is that a new Plan (`plan_id` incremented/replaced) or an in-place patch to the existing Plan document, and how is that distinguished from silently violating declared immutability?
