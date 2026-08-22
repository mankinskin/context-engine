# 04 - Completion Checklist

## Traceability

| Raw-transcript requirement | Dossier coverage |
|---|---|
| Consider the recently created example. | `01-case-study-and-target.md` treats the reviewed Presentation System spec as the case study. |
| The specification still visibly has the problem despite significant work. | `REVIEW.md` and `01-case-study-and-target.md` identify the monolithic document shape and explain why it blocks architectural traversal. |
| Use the example to refine the issue. | `02-existing-capability-and-decision.md` compares verified repository primitives with the requested semantics; `03-migration-pilot-roadmap.md` provides the next bounded work. |
| Prior architecture guidance should inform the analysis. | `ARTIFACTS.md` records the clean guidance source; `01-case-study-and-target.md` translates it into the target artifact model. |

## Artifact Check

The dossier is complete only when every file below exists and is non-empty:

```text
input.md
input.clean.md
input-2.md
input-2.clean.md
merged.clean.md
REVIEW.md
ARTIFACTS.md
01-case-study-and-target.md
02-existing-capability-and-decision.md
03-migration-pilot-roadmap.md
04-completion-checklist.md
05-target-artifact-contract.md
README.md
```

Deterministic check:

```bash
for file in input.md input.clean.md input-2.md input-2.clean.md merged.clean.md REVIEW.md ARTIFACTS.md 01-case-study-and-target.md 02-existing-capability-and-decision.md 03-migration-pilot-roadmap.md 04-completion-checklist.md 05-target-artifact-contract.md README.md; do
  test -s "transcripts/20-08-2026_specification-architecture-guidelines/$file"
done
```

## Open Questions

- Should components record only consumed expectations, offered contracts, or both?
- If both declarations exist, which one is authoritative and how are conflicts detected?
- Which validation-spec identifiers and screenshot/output locations should be canonical evidence for the Presentation System criteria?
- Should a future migration be a generic tool or a manual, per-spec process beginning with this pilot?