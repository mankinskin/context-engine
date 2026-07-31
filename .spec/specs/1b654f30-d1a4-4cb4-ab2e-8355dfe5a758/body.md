
# Summary

Flattened two-tier orchestration: **Planner/Architect** (frontier model, plans once) and **Worker** (fast/cheap model, executes exactly one isolated step), replacing the multi-hop chain (T0 orchestrator → T1/T2 sequencer → T3 executor) in [model-routing.instructions.md](../../.agents/instructions/orchestration/model-routing.instructions.md) and [orchestrator-delegation.instructions.md](../../.agents/instructions/orchestration/orchestrator-delegation.instructions.md). Defines the plan schema and Worker capability boundary; does not implement dispatch (tracked under tickets 1a240fdc, 7563ce30, 44c6cc5c, epic f13c9836 — all shipped, see Traceability).

# Problem

Each extra delegation hop (T0→T1/T2→T3) re-hands orchestration contract text to an intermediate agent, spawns a full sub-agent context (measured duplicate-read cost: [shared-context-bundle.instructions.md](../../.agents/instructions/orchestration/shared-context-bundle.instructions.md)), and multiplies the points where a plan can drift before reaching an executor (per [model-routing.instructions.md](../../.agents/instructions/orchestration/model-routing.instructions.md) "Failure Path", a failed cheap-tier attempt costs a full retry spawn). Source: `AGENT_WORKFLOW_OPTIMIZATIONS.md`, "Step 1: Tooling Restructure" and "The Two-Tier Architecture".

# Scope

- Define the Planner/Architect role: reads the ticket and repo schema/context, and outputs one immutable, rigid, structured (JSON) execution plan intended for **direct execution**, not further re-planning by an intermediate agent.
- Define the Worker role: executes exactly one isolated plan step against one declared target file/scope, then stops — no re-planning, no scope expansion, no chaining to a next step on its own initiative.
- Define a concrete plan schema (this document) that a Planner emits and a Worker consumes.
- Reconcile the two-tier model with the existing tier ladder ([model-routing.instructions.md](../../.agents/instructions/orchestration/model-routing.instructions.md) lines 20-90), the delegation contract ([orchestrator-delegation.instructions.md](../../.agents/instructions/orchestration/orchestrator-delegation.instructions.md) lines 55-91), the mandated pre-dispatch gate mechanism ([pre-dispatch-gates.instructions.md](../../.agents/instructions/orchestration/pre-dispatch-gates.instructions.md)), and the shared context bundle protocol ([shared-context-bundle.instructions.md](../../.agents/instructions/orchestration/shared-context-bundle.instructions.md)).
- Record explicit open questions this spec does not resolve (plan validation, mid-execution failure handling, immutability enforcement, gate interaction).

# Non-Goals

- Implementing the Planner/Worker dispatch mechanism, plan schema validator, or any change to `.agents/agents/orchestrator.agent.md`, `model-routing.instructions.md`, or `orchestrator-delegation.instructions.md` — tracked separately under tickets 1a240fdc, 7563ce30, 44c6cc5c (epic f13c9836).
- Changing the tier ladder's model list, prices, `mcp-cost-gate` budget formula, or the `model:` per-template frontmatter contract, or defining a multi-hop-vs-two-tier cost benchmark — owned elsewhere; see Related Specs.

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
| Run the step's declared validation command(s), and pre-authored test files for the ticket | Invent new validation commands not in the plan, or edit/weaken/skip pre-authored test files (test files are authored by the Planner tier before Worker dispatch — see [split-responsibility-testing.instructions.md](../../.agents/instructions/testing/split-responsibility-testing.instructions.md)) |
| Return exactly the step's `return_contract` shape | Return a free-form transcript, or silently change the contract shape |
| Report a blocker via `{pass: false, blocker: "..."}` when `done_criteria` cannot be met, or when a pre-authored test appears wrong | Attempt to re-plan around the blocker itself (e.g. skip to a different file, redefine the objective, or edit the test to make it pass) |
| Make one self-fix retry attempt inside its own session after a failing test (see [retry-limit.instructions.md](../../.agents/instructions/orchestration/retry-limit.instructions.md)) | Attempt a third fix on the same step after a second test failure |
| Stop after completing (or blocking on) its one step, per [write-and-die.instructions.md](../../.agents/instructions/orchestration/write-and-die.instructions.md) | Chain to the next `step_id` on its own initiative, or remain resident awaiting further instructions |
| Flag that a dependency (`depends_on`) looks unmet | Decide unilaterally to proceed anyway or reorder steps |

This table is the full Worker boundary. The instruction files linked in each row are the shipped, verified implementation of the corresponding row; where an instruction file states a rule, this table states it too — neither is authoritative over the other for a row present in both.

# Plan Schema

Illustrative plan shape, not a runnable schema — no JSON Schema, serde struct, or validator exists for this yet (see Validation). Field names below are normative for any future validator/dispatcher implementation; a validator may add non-normative metadata but must not omit these fields.

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
        "shape": "diff_summary | validation_result | blocker",
        "required_fields": ["string; see closed enum below for the required set per shape"]
      },
      "done_criteria": "string (verifiable condition, e.g. \"cargo test -p session-api passes\" or \"file X contains section Y\")",
      "depends_on": ["string (step_id of a prerequisite step; must reference only step_ids appearing earlier in this plan's steps array — no forward references, no cycles)"]
    }
  ],
  "validation_commands": ["string (plan-level validation run after all steps complete)"]
}
```

### `return_contract.shape` — closed enum (contract)

| `shape` | `required_fields` |
|---|---|
| `diff_summary` | `files_changed` (list of paths), `summary` (string) |
| `validation_result` | `validation_command` (string), `validation_output` (string), `pass` (bool) |
| `blocker` | `pass: false`, `blocker` (string reason) |

A Worker returns exactly one of these three shapes; a dispatcher switches on `shape` and requires the matching `required_fields` set. No other `shape` value is valid. (No dispatcher currently implements this switch — see Validation.)

### Invariants

- **`depends_on` is acyclic and forward-only**: a step may only depend on `step_id`s that appear earlier in the same plan's `steps` array. A plan with a cycle or a forward reference is invalid.
- **`immutable: true` is a declared, not mechanically enforced, property.** No structural control (e.g. a diff-scope check against `target_path`) currently exists to prevent a Worker from editing outside its step. This is unresolved — see Open Questions Q3 and Validation Gaps; it has no owning ticket at time of writing.
- **`done_criteria` is example-only, not schema-enforced as executable.** The schema requires the field be present and non-empty; it does not (yet) require the string be a literally runnable command vs. a natural-language condition — that enforcement is unowned, same as Q3.
- **Plan amendment is partially resolved.** [retry-limit.instructions.md](../../.agents/instructions/orchestration/retry-limit.instructions.md) (ticket 1a240fdc, shipped) answers the two-step-failure case: the dispatcher either patches the Step in place or invalidates the Step back to the Planner. It does not define whether a patched/invalidated step keeps the same `plan_id` or requires a new one — that narrower versioning question remains open (Q6).

Notes:

- `context_bundle_ref` is deliberately a pointer, not inline content, at the plan level; the Planner resolves and inlines the actual bundle content into each Worker's dispatch prompt at dispatch time, per [shared-context-bundle.instructions.md](../../.agents/instructions/orchestration/shared-context-bundle.instructions.md).
- `allowed_tools` and `forbidden_actions` together define the enforceable half of the Worker capability boundary table above; the rest (e.g. "does not re-plan") is a behavioral contract stated in the dispatch prompt, not a mechanically checked property.

# What Changes / What Is Preserved

| Rule | Old (multi-hop) | New (two-tier) | Status |
|---|---|---|---|
| Step decomposition | Sequencer (T1/T2) may further decompose a delegated unit (`orchestrator-delegation.instructions.md` "Case → Capability-Role → Cost-Class Mapping", lines 106+) | Decomposition happens exactly once, in the Planner; a Worker never re-decomposes or re-sequences | Superseded (two-tier path only; multi-hop path not deleted — see Non-Goals) |
| "Plan" step of Required Workflow | Ongoing orchestrator activity refined across the session (`orchestrator-delegation.instructions.md` "Required Workflow", lines 92+, step 1) | Single terminal Plan document, produced once | Superseded (two-tier path only) |
| Tier ladder | `model-routing.instructions.md` lines 20-90 | Unchanged; Planner uses T0, Worker per-step tier selection resolves through the same ladder | Preserved |
| `X = 15` orchestrator-mode threshold | `orchestrator-delegation.instructions.md` "When to Activate" | Unchanged; decides who runs as Planner, not which model a Worker step gets | Preserved |
| Pre-dispatch gate mechanism | [pre-dispatch-gates.instructions.md](../../.agents/instructions/orchestration/pre-dispatch-gates.instructions.md), gates the delegation | Same gate mechanism and `{pass, bundle}`/`{pass: false, blocker}` contract, unit of application shifts to the plan step | Preserved (unit of application changed, mechanism unchanged) |
| Shared context bundle protocol | [shared-context-bundle.instructions.md](../../.agents/instructions/orchestration/shared-context-bundle.instructions.md) | Unchanged; a Step's `context_bundle_ref` resolves to the same bundle shape | Preserved |
| Context isolation, verify-before-acting, one-retry-then-step-up-one-band, parallel fan-out | `model-routing.instructions.md` | Unchanged; apply per-step rather than per whole-task delegation | Preserved |
| `mcp-cost-gate` MCP boundary enforcement, `caller_model` requirements | `orchestrator-delegation.instructions.md` "Cost Gating" | Unaffected; this spec changes dispatch shape, not MCP tool-call cost enforcement | Preserved |

# Validation

**No executable validator, test suite, or CI job exists for this spec's plan schema or Worker capability boundary.** This is an architecture/contract spec; the artifacts it governs today are the three shipped instruction files below, not runnable code.

What exists and is verifiable today (documentation-conformance checks, not behavioral tests):

- `rg -n "one self-fix retry" .agents/instructions/orchestration/retry-limit.instructions.md` — confirms the shipped resolution of Open Question 2 (ticket 1a240fdc, state `done`).
- `rg -n "write-and-die" .agents/instructions/orchestration/write-and-die.instructions.md` — confirms the shipped "stop after one step" boundary row (ticket 7563ce30, state `done`).
- `rg -n "may not edit test files" .agents/instructions/testing/split-responsibility-testing.instructions.md` — confirms the shipped test-authorship boundary row added above (ticket 44c6cc5c, state `done`).

These are grep-level existence checks against shipped instruction text, not automated assertions that a live Worker session actually obeys the rules.

**Explicitly missing, not silently omitted:**

- No JSON Schema, serde struct, or other machine-readable schema for the Plan Schema above exists in the repo; nothing can validate a Plan document programmatically.
- No dispatcher implements the `return_contract.shape` closed-enum switch defined above.
- No structural check verifies a Worker's returned diff stays inside its step's `target_path` (Open Question 3) — this is a genuine, unowned validation gap, not a deferred nicety.
- No test-execution record exists against ticket 1a240fdc, 7563ce30, or 44c6cc5c's validation evidence at time of this spec update; `test-mcp` was not available to this authoring session to confirm current execution counts — treat as unverified rather than assumed zero.

# Traceability / Evidence

- Ticket (spec-authoring): [feb5784c Spec: two-tier Planner/Worker model routing architecture](../../.ticket/tickets/feb5784c-ece7-40d3-9617-3faee2f6a753/ticket.toml) — state `done`.
- Epic: [f13c9836 [epic] Worker-tier dispatch contract](../../.ticket/tickets/f13c9836-5b74-433b-b3d2-e1475d080ad0/ticket.toml).
- Shipped implementation tickets, all state `done`, each depends on feb5784c:
  - [1a240fdc Retry-limit escalation policy for worker-tier test failures](../../.ticket/tickets/1a240fdc-7de2-4494-8714-b2c81de09158/ticket.toml) → [retry-limit.instructions.md](../../.agents/instructions/orchestration/retry-limit.instructions.md).
  - [7563ce30 Write-and-die pattern for worker sub-agent dispatch](../../.ticket/tickets/7563ce30-bf9e-43fe-bca5-68473b1d9d79/ticket.toml) → [write-and-die.instructions.md](../../.agents/instructions/orchestration/write-and-die.instructions.md).
  - [44c6cc5c Split-responsibility testing: frontier-authored tests, worker-only implementation](../../.ticket/tickets/44c6cc5c-35c0-4cf0-b607-186914c21e5d/ticket.toml) → [split-responsibility-testing.instructions.md](../../.agents/instructions/testing/split-responsibility-testing.instructions.md).
- Instructions referenced, not edited, by this spec: [model-routing.instructions.md](../../.agents/instructions/orchestration/model-routing.instructions.md), [orchestrator-delegation.instructions.md](../../.agents/instructions/orchestration/orchestrator-delegation.instructions.md).
- Source: `AGENT_WORKFLOW_OPTIMIZATIONS.md`, "Step 1: Tooling Restructure" and "The Two-Tier Architecture".

# Related Specs

- [a4d61b8c Model cost awareness and tiered model routing for agent sessions](../a4d61b8c-df1c-454d-ab56-4bce5706eb15/spec.toml) — owns the tier ladder and `model:` per-template contract this spec's Planner/Worker tiers resolve through; not duplicated here.
- [39983ddf Model price awareness: orchestrator-mode enforcement](../39983ddf-1f7e-4081-a060-6b8258eb4c41/spec.toml) — owns the `X = 15` orchestrator-mode threshold and cost-gate enforcement this spec's "Preserved" rows defer to.
- [ec3b13f1 Per-template MCP tool grants](../ec3b13f1-ae9f-4f11-b3f9-e8fa3877afbd/spec.toml) — adjacent per-template tool-grant scoping; the Worker's `allowed_tools` field in the Plan Schema is a plan-level analog, not a replacement, of that per-template grant mechanism.

# Acceptance Criteria

1. **Boundary table is complete and traceable, not merely present.** Every row in the Worker capability boundary table either states a mechanically enforceable field-level constraint (`target_path`, `allowed_tools`, `return_contract.shape`) or links the shipped instruction file that implements the behavioral half (retry-limit, write-and-die, split-responsibility-testing). Checkable by: for each table row, either the referenced field exists in the Plan Schema below, or the linked instruction file exists at the given path and contains the quoted rule (see Validation greps).
2. **Plan Schema field completeness is checkable by structural diff, not prose review.** Every field named in this AC must appear in the JSON shape above: plan (`plan_id`, `ticket_id`, `goal`, `steps`, `validation_commands`) and step (`step_id`, `target_path`, `objective`, `allowed_tools`, `return_contract`, `done_criteria`, `depends_on`). Checkable today by manual inspection of the Plan Schema section; automatable once the missing JSON Schema (Validation, "Explicitly missing") is written.
3. **`return_contract.shape` is a closed enum with per-shape required fields**, not a free-text example — checkable by confirming the shape enum table above has no shape without a `required_fields` row, and no field appears in `required_fields` for more than one incompatible meaning.
4. **Reconciliation is a status per named rule, not a narrative claim.** The "What Changes / What Is Preserved" table above must assign exactly one status (Superseded / Preserved) to each named rule from `model-routing.instructions.md` and `orchestrator-delegation.instructions.md` cited elsewhere in this spec — checkable by confirming no rule is cited in Problem/Scope/Roles without a corresponding table row.
5. **Every Open Question below is tagged Open, Resolved (with citation), or Deferred (with explicit no-owner note)** — not left as an unstatused rhetorical question. Checkable by scanning the Open Questions section for a status tag on each numbered item.

# Open Questions

1. **Open — Plan validation before dispatch.** Who or what validates a Plan against this schema before any Worker is dispatched against it — a dedicated validator tool, the existing pre-dispatch gate mechanism repurposed per-step, or a new gate class? No validator exists (see Validation); no ticket currently owns building one.
2. **Resolved — Mid-execution failure handling.** [retry-limit.instructions.md](../../.agents/instructions/orchestration/retry-limit.instructions.md) (ticket 1a240fdc, shipped) answers this: a Worker gets exactly one self-fix retry inside its own session; on a second failure it escalates via `{pass: false, blocker: "..."}`, and the dispatcher either patches the Step in place or invalidates it back to the Planner. This composes with `model-routing.instructions.md` "Failure Path" (one retry, step up one band) as the test-failure-specific form of that same rule.
3. **Deferred, no owner — Enforceability of plan immutability.** `immutable: true` is declared in the schema; nothing described here mechanically prevents a Worker from improvising beyond its step. No structural control (e.g. a diff-scope check against `target_path`) exists, and no ticket is currently scoped to build one. This is a genuine gap, stated explicitly rather than left as a rhetorical question — the Worker capability boundary table's enforcement is behavioral (dispatch-prompt contract) only until such a control exists.
4. **Open — Interaction with the existing pre-dispatch gate contract.** `pre-dispatch-gates.instructions.md` defines gates per delegation *class* (Implement/Review/Testing/Commit), not per plan step. Whether a Plan step reuses its class's existing gate set unmodified, or needs a new per-step gate class, is unresolved and unowned.
5. **Open — Sequencer role fate.** Whether the T1/T2 Sequencer role fully disappears under the two-tier model, or survives as a three-tier fallback for a Planner handing off a cross-cutting, high-risk sub-plan to a mid-tier re-planner before Worker dispatch, is unresolved and unowned.
6. **Partially resolved — Plan amendment mechanics.** retry-limit.instructions.md (Q2) establishes that a dispatcher may patch a Step in place or invalidate it back to the Planner on repeated failure. It does not state whether a patched/invalidated Step keeps the same `plan_id` (in-place amendment) or requires a new one (versioned replacement) — that narrower question is unresolved and unowned.

