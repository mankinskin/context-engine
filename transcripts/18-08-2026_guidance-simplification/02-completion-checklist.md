# Completion Checklist — Traceability

## Requirement → dossier location

| Requirement (from [input.clean.md](./input.clean.md)) | Addressed in |
|---|---|
| Cut ≥50% of guidance text, correctness-first, no over-specification | [01-work-packages.md](./01-work-packages.md) WP-A |
| Top-down inventory of guidance files, keep/shrink decision | WP-A step 1–2 |
| Find and inline store links, make corpus self-contained | WP-A step 3, validated by the zero-store-link grep check |
| Agent templates: workflow-step structure, less prohibition, tool links | WP-B |
| Instruction files: narrow, single closed step, short frontmatter | WP-A (kept-file shrink criteria) |
| Delete unneeded specs, massively shorten kept ones | WP-E |
| New compact spec format: structured acceptance criteria, separated links, prose-only description | WP-C |
| Derived full-Markdown rendering from structured metadata | WP-C outcome (b) |
| Migration tooling + spec-tool adaptation | WP-D |
| Second condensation pass after migration | WP-F |

## Deterministic artifact checks

- [x] `01-work-packages.md` exists with six work packages, each with outcome, non-goals, and validation method.
- [x] At least one direct policy conflict is identified and resolved with a stated recommendation (Finding 2).
