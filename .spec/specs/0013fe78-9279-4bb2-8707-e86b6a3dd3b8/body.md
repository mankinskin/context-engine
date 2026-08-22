<!-- aligned-structure:v2 -->

# Validated Response

## Responsibility And Interface

Give the user a concise, evidence-backed result after review. Consume
Implementation's three criteria; report relevant `.test/` executions and
`target/test-logs/` outcomes; pass the user's judgment to Next iteration.

## Behavior And Contract

- `response-evidence`: states changed contract and actual command/manual verdict.
- `response-user-judgment`: gives scope and traceability to accept or follow up.
- `response-review-gate`: appears only after review and validation evidence exist.

## Boundaries And Failure Cases

Do not call a draft, unreviewed, unvalidated, or blocked change complete. Never
hide a limitation or fabricate a result. An unmet review criterion returns work
to Implementation instead of yielding a completion response.

## Acceptance Evidence And Position

Compare the response with the ticket review and recorded validation: scope,
verdict, and limitation must agree. `loop-closure.instructions.md` requires
approved work before commit and a forward handoff after passing work. No
independent executable `validated_by` exists.
