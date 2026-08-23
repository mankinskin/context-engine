to Implementation instead of yielding a completion response.
<!-- aligned-structure:v2 -->

# Production Workflow: Validated Response

## Target Code Location

[.agents/instructions/orchestration/loop-closure.instructions.md](.agents/instructions/orchestration/loop-closure.instructions.md) owns review-to-handoff closure; [AGENTS.md](AGENTS.md) owns feedback workflow.

## Naming Conventions

Use response fields `scope`, `evidence`, `verdict`, and `limitations`; feedback
targets use entity URNs such as `ce://default/spec/<id>`. This component owns `response-evidence`, `response-user-judgment`, and `response-review-gate`.

## Requester Input

> Compile an evidence-backed summary back to the user — validation-gate results, links to closed tickets/specs, the updated technical docs, the updated deck, and the updated guidance.

## Reading Order

1. [.agents/instructions/orchestration/loop-closure.instructions.md](.agents/instructions/orchestration/loop-closure.instructions.md) — closure owner.
2. [44b32c98 Production Workflow: Implementation](.spec/specs/44b32c98-a4cf-4d83-b114-6e5db65b6212/body.md) — review-evidence provider.
3. [4199e86c Production Workflow: Next Iteration](.spec/specs/4199e86c-05b0-48a0-bebd-c55efcfa20a5/body.md) — judgment consumer.

## Responsibility

If implemented, Next Iteration can rely on a concise user-facing result whose
scope, recorded evidence, verdict, and limitation agree with review artifacts.

## Interfaces And Dependencies

Consumes `implementation-*` criteria, `.test/` executions, and `target/test-logs/`.
feedback-api persists an explicit user judgment in `.feedback`; automatic capture
from a chat response is specified-but-not-built.

## Behavior

- `response-evidence` states changed contract and actual command/manual verdict.
- `response-user-judgment` asks for acceptance or follow-up with entity traceability.
- `response-review-gate` is emitted only after review and validation evidence exist.

## Boundaries And Failure Cases

Do not call draft, blocked, unreviewed, or unvalidated work complete. Never hide
a limitation or fabricate a verdict; an unmet review criterion returns to Implementation.

## Provider/Consumer Contract

Consumes all `implementation-*` criteria from [44b32c98 Production Workflow: Implementation](.spec/specs/44b32c98-a4cf-4d83-b114-6e5db65b6212/body.md); provides all three `response-*` criteria to [4199e86c Production Workflow: Next Iteration](.spec/specs/4199e86c-05b0-48a0-bebd-c55efcfa20a5/body.md).

## Examples

`Validation: cargo test -p spec-api --test schema_test (passed); limitation: no browser surface changed` is a response evidence record. A satisfaction judgment is persisted with feedback-api against `ce://default/spec/<id>`.

## Evidence

Position: `partial`; response and loop-closure guidance exist, and feedback-api
owns `.feedback` records. There is no automatic response-to-feedback bridge.

## Scope

Owns reviewed reporting and judgment solicitation, not code changes or feedback analysis.
