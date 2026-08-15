# Dossier Completion Checklist

## Traceability Matrix

| Transcript requirement | Dossier location | Status |
| --- | --- | --- |
| Verification is planned before solutions | `02-verification-operating-model.md`: Principle and criterion record | Pass |
| Deterministic, human, and LLM verification are distinguished | `02-verification-operating-model.md`: Verifier hierarchy | Pass |
| Criteria can be defined, refined, and corrected | `02-verification-operating-model.md`: criterion record and evidence chain | Pass |
| Flakiness and changing results are addressed | `02-verification-operating-model.md`: Flakiness policy | Pass |
| Product and customer-project artifacts are separated | `02-verification-operating-model.md`: Artifact ownership boundary | Pass |
| Test, Log, and Audit APIs are used as starting points | `01-research-inventory.md` | Pass |
| Cheap repeated checks, local linting, and architecture checking are considered | `REVIEW.md` and `03-improved-one-session-plan.md`: Roadmap item 2 | Pass |
| Ticket evidence and test-review concerns are preserved | `REVIEW.md` and `03-improved-one-session-plan.md`: Roadmap items 1 and 2 | Pass |
| MCP ergonomics and high cost are addressed distinctly | `01-research-inventory.md` and `03-improved-one-session-plan.md`: Roadmap item 4 | Pass |
| The current session stays one-shot and avoids product implementation | `03-improved-one-session-plan.md`: Completion contract and stop rule | Pass |
| The plan defines how its own completion is verified | This checklist and `03-improved-one-session-plan.md`: Completion contract | Pass |

## Deterministic Artifact Checks

| Check | Expected result | Status |
| --- | --- | --- |
| Raw source remains unchanged | `input.md` exists beside the clean transcript | Pass |
| Clean transcript remains available | `input.clean.md` exists | Pass |
| Review exists | `REVIEW.md` contains a verdict and required improvements | Pass |
| Index exists | `README.md` links all dossier artifacts | Pass |
| Research is bounded | Inventory names exactly four inspected repository surfaces | Pass |
| Plan is bounded | Plan explicitly excludes product implementation and state changes | Pass |
| Roadmap tasks are independently actionable | Every roadmap item states outcome, non-goal, and validation | Pass |

## Open Questions for a Follow-Up Planning Session

1. Which ticket lifecycle transition code currently enforces validation evidence,
   and which proposed rules are genuinely absent?
2. Which existing audit trial covers each desired deterministic check?
3. What representative task set can measure MCP interface cost without being
   distorted by unrelated model or repository changes?
4. What governance policy, if any, grants LLM verification authority beyond
   advisory evidence?
