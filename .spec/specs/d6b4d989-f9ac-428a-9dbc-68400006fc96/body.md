<!-- aligned-structure:v2 -->

# Production Workflow: Specification

## Target Code Location

[.agents/instructions/spec/spec-system.instructions.md](.agents/instructions/spec/spec-system.instructions.md) owns specification structure; [workflow-tools/spec/crates/spec-api/](workflow-tools/spec/crates/spec-api/) owns persisted specs.

## Naming Conventions

Use a root `spec.toml` plus `body.md`, and `parent` children for independently
addressable components. This component owns `spec-goal`, `spec-owned-criteria`, and `spec-traceability`.

## Requester Input

> The spec is authored first, directly from a free-form request or dossier, and captures the goal/definition of success.

## Reading Order

1. [.agents/instructions/spec/spec-system.instructions.md](.agents/instructions/spec/spec-system.instructions.md) — structure and governing-rule owner.
2. [e8104080 Production Workflow: Request](.spec/specs/e8104080-df78-46cb-ac64-3bfeb51e583b/body.md) — request criteria provider.
3. [c522633d Production Workflow: Tickets](.spec/specs/c522633d-7ec8-462a-ae00-30370e37a2d7/body.md) — provider-criteria consumer.

## Responsibility

If implemented, Tickets can rely on a reviewable goal, exclusively owned
criteria, and traceability before it plans implementation work.

## Interfaces And Dependencies

Consumes request outcome and open questions. Persists draft specs in `.spec/specs/`
through `./target/debug/spec.exe --workspace . create` or `update`.

## Behavior

- `spec-goal` names the requested property before planning.
- `spec-owned-criteria` assigns every acceptance criterion to one provider.
- `spec-traceability` links related specs, returned ticket paths when they exist,
  and concrete review evidence.

## Boundaries And Failure Cases

Do not duplicate a matching spec or call a draft validated. A vague goal, missing
owner, or unclear success condition returns to Request/interview; independently
addressable components require a thin root and explicit children.

## Provider/Consumer Contract

Consumes `request-outcome` and `request-open-questions` from [e8104080 Production Workflow: Request](.spec/specs/e8104080-df78-46cb-ac64-3bfeb51e583b/body.md); provides all three `spec-*` criteria to [c522633d Production Workflow: Tickets](.spec/specs/c522633d-7ec8-462a-ae00-30370e37a2d7/body.md).

## Examples

`./target/debug/spec.exe --workspace . health --all` checks structural health;
`spec get <id> --json` exposes the draft root, children, and traceability.

## Evidence

Position: `implemented` CLI and authoring instruction. Guard executions are not
recorded for this draft; review checks provider ownership and link completeness.

## Scope

Owns goal and acceptance contract authoring, not ticket planning or implementation.
