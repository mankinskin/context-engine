Optimize orchestrator to compute and inline the shared context bundle prefix for parallel fan-out.

Acceptance criteria:
- Orchestrator computes shared artifact set once and inlines it into each sibling's context bundle.
- Bundle size targets 2k-5k tokens and uses skeletons/bounded windows for large files.
- Measured reduction in duplicate reads in a sample parallel fan-out run (document before/after numbers).

Traceability:
- References spec 63c60c9d-adbe-4ddb-8c1d-6156610d0753 and benchmark 10d21210 for delegation cost reductions.
- Links to ticket fb14754e-2be8-40a5-a995-488842ba6367 (carry verified repo paths in handoff) for path canonicalization.

Notes:
- Workspace: C:/Users/linus/git/context-engine/.ticket